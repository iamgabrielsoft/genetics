use std::{
    net::{IpAddr, TcpListener},
    path::{Path, PathBuf},
    sync::mpsc::channel,
    thread,
    time::Duration,
};
use errors::{ anyhow, Context, Result};
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Response, Server, StatusCode,
};
use notify_debouncer_full::{new_debouncer, notify::RecursiveMode};
use ws::{Message, Sender, WebSocket};
use crate::fs::{ build_output_dir_with_broadcaster, generate_site, create_directory };



#[derive(Debug, PartialEq)]
pub enum WatchStatus {
    Required, 
    Optional, 
    Conditional(bool),
}

// #[derive(Debug, Clone, PartialEq)]
// pub enum RecursiveMode {
//     // watch sub-directories
//     Recursive, 

//     // watch provided directory
//     NonRecursive,
// }


// impl RecursiveMode {
//     pub(crate) fn is_recursive(&self) -> bool {
//         match *self {
//             RecursiveMode::Recursive => true, 
//             RecursiveMode::NonRecursive => false,  
//         }
//     }
// }


pub fn serve_site(
    root_dir: &Path,
    interface: IpAddr,
    interface_port: u16,
    output_dir: Option<&Path>,
    force: bool,
    base_url: Option<&str>,
    config_file: &Path,
    open: bool,
    no_port_append: bool,
) ->  Result<()> {
   // let start = Instant::now();

    let (site, address, constructed_base_url) = generate_site(
        root_dir,
        interface,
        interface_port,
        output_dir,
        force,
        base_url,
        config_file,
        no_port_append,
    )?;

    // let base_path = match constructed_base_url.splitn(4, "/").nth(3) {
    //     Some(xm) => format!("/{}", xm), 
    //     None => "/".to_string(),
    // };

    
    if(TcpListener::bind(address)).is_err() {
        return Err(anyhow!("Cannot start server on address {}.", address));
    }

    // let config_buf = PathBuf::from(config_file); 
    let root_dir_str = root_dir.to_str().expect("Invalid root directory");

    let watch_vector = vec![
        (root_dir_str, WatchStatus::Required, RecursiveMode::NonRecursive),
        ("content", WatchStatus::Required, RecursiveMode::NonRecursive),
        ("static", WatchStatus::Optional, RecursiveMode::Recursive),
        ("templates", WatchStatus::Optional, RecursiveMode::Recursive),
       // ("themes", WatchStatus::Conditional(site.config.themes.is_some()), RecursiveMode::Recursive),
    ];

    //let (tx, rx) = channel();
    let (tx, _) = channel();
    let mut debouncer = new_debouncer(Duration::from_secs(1), None, tx).unwrap();


    let mut watchers = Vec::new();
    for (entry, mode, recursive_mode) in watch_vector {
        let watch_path = root_dir.join(entry);
        let watch_state = match mode {
            WatchStatus::Required => true,
            WatchStatus::Optional => watch_path.exists(),
            WatchStatus::Conditional(x) => x && watch_path.exists(),
        };

        if watch_state {
            debouncer.watch(
                &root_dir.join(entry),
                recursive_mode,
            )
            .with_context(|| format!("Unable to watch directory {}", entry))?;
        }
        watchers.push(entry.to_string());
    }


    //watch the directories 
    // websocket 
    const DEFAULT_WS_PORT: u16 = 8080;
    let ws_port = site.config.live_reload.unwrap_or(DEFAULT_WS_PORT);
    let ws_address = format!("{}:{}", interface, ws_port);
    let output_path = site.output_path.clone();

  //  let static_root_path = std::fs::canonicalize(&output_path).unwrap(); //the output directory can be changed

    let broadcaster = {
        thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Could not tokio builder");

            rt.block_on(async {
                let servelet = make_service_fn(move |_| {
                   // let static_root = static_root_path.clone();
                   // let base_path = base_path.clone(); 

                    async {
                        Ok::<_, hyper::Error>(service_fn(move |_| async {
                            let response = Response::builder()
                                .status(StatusCode::OK)
                                .body(Body::empty())
                                .unwrap();
                            Ok::<_, hyper::Error>(response)
                        }))
                    }
                });


                let server = Server::bind(&address).serve(servelet); 

                println!("Listening on {}, {}", constructed_base_url, address);

                if open {
                    if let Err(err) = open::that(&constructed_base_url) {
                        println!("Unable to open URL: {}", err);
                    }
                }


                server.await.expect("Unable to start server");
            })
        }); 


        let ws_server = WebSocket::new(|output: Sender| {
            move |msg: Message| {
                if msg.into_text().unwrap().contains("hello") {
                    return output.send(Message::text(
                        r#"
                        {
                            "command": "hello",
                            "protocols": [ "http://livereload.com/protocols/official-7" ],
                            "serverName": "Genetics"
                        }
                    "#
                    ))
                }

                Ok(())
            }
        })
        .unwrap();

        let broadcaster = ws_server.broadcaster();

        let ws_server = match ws_server.bind(&*ws_address) {
            Ok(server) => server,
            Err(err) => {
                eprintln!("Unable to bind to address: {}", err);
                return Err(anyhow!("Failed to bind WebSocket server: {}", err));
            }
        };


        thread::spawn(move || {
            ws_server.run().unwrap();
        });

        broadcaster
    };

    //we can watch for changes in the config file
    let config_path = PathBuf::from(config_file);
    let config_name = config_path.file_name().unwrap().to_str().expect("Invalid config file");
    let watch_list = watchers
        .iter()
        .map(|entry| if *entry == root_dir_str { config_name } else { entry })
        .collect::<Vec<&str>>()
        .join(",");

    println!("\nWatching directories: {}", watch_list);
    create_directory(&output_path)?;

    // let watch_list = watchers
    //     .iter()
    //     .map(|w| if w == root_dir_str { config_buf } else { w })
    //     .collect::<Vec<_>>()
    //     .join(",");

    // println!("\nWatching directories: {}", watch_list);

    println!("Use Ctrl+C to stop\n");

    ctrlc::set_handler(move || {
        println!("\nShutting down...");
        ::std::process::exit(0);
    })
    .expect("Unable to set Ctrl+C handler");

    // let templates = |site: &mut Site| {
    //     build_output_dir_with_broadcaster(
    //         &broadcaster, 
    //         site.reload_templates, 
    //         &site.templates_path.to_string_lossy()
    //     );
    // };

    
    let create_site = move || -> Result<()> {
        let _ = match generate_site(
            root_dir, 
            interface, 
            interface_port, 
            output_dir, 
            force, 
            base_url, 
            config_file, 
            no_port_append
    ) {
            Ok((_, _, _)) => {
                //clean up serve error if there's
                // perform rebuilding of site
                build_output_dir_with_broadcaster(
                    &broadcaster, 
                    Ok(()), 
                    "/x.js"
                );
                Ok(())
            },
            Err(err) => {
                println!("Unable to serve site: {}", err);
                Err(err)
            },
        };
        Ok(())
    };

    create_site() //run the site
}



/// Gets an available port
pub fn get_available_port(interface:IpAddr, prevent: u16) -> Option<u16> {
    (1024..9000).find(|port| *port != prevent && available_port_checker(interface, *port))
}

/// Checks if a port is available
pub fn available_port_checker(interface:IpAddr, port: u16) -> bool {
    TcpListener::bind((interface, port)).is_ok()
}



/// Checks if a link is external
pub fn is_external_link(link: &str) -> bool {
    link.starts_with("http://") || link.starts_with("https://")
}

#[cfg(test)]
mod test {
    
}