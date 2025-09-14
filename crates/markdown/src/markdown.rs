//! Markdown processing functionality

use pulldown_cmark::CowStr;
use pulldown_cmark as cmark;
use pulldown_cmark_escape::escape_html;
use anyhow::Result;
use std::fmt::Write;
use std::vec;

use pulldown_cmark::{Event, Options, Parser, Tag};
use crate::context::RenderContext;
use utils::content::Heading;


const CONTINUE_READING: &str = "<span id=\"continue-reading\"></span>";

#[derive(Debug)]
pub struct Rendered {
    pub body: String,
    pub summary: Option<String>,
   // pub toc: Vec<HeadingStruct>,
    /// Links to site-local pages: relative path plus optional anchor target.
    pub internal_links: Vec<(String, Option<String>)>,
    /// Outgoing links to external webpages (i.e. HTTP(S) targets).
    pub external_links: Vec<String>,
}

#[derive(Debug)]
pub struct HeadingStruct {
    start_idx: usize,
    end_idx: usize,
    level: u32,
    id: Option<String>,
    classes: Vec<String>,   
}


impl HeadingStruct {
    pub fn new(start: usize, level: u32, anchor: Option<String>, classes: &[String]) -> HeadingStruct {
        HeadingStruct {
            start_idx: start,
            end_idx: 0,
            level,
            id: anchor,
            classes: classes.to_vec(),
        }
    }

    pub fn format_to_html(&self, id: &str) -> String {
        let mut buffer = String::with_capacity(100);

        buffer.write_str("<h").unwrap();
        buffer.write_str(&format!("{}", self.level)).unwrap();

        buffer.write_str(" id=\"").unwrap();
        escape_html(&mut buffer, id).unwrap();
        buffer.write_str("\"").unwrap();

        if !self.classes.is_empty() {
            buffer.write_str(" class=\"").unwrap();
            for (i, class) in self.classes.iter().enumerate() {
                escape_html(&mut buffer, class).unwrap();
                if i < self.classes.len() - 1 {
                    buffer.write_str(" ").unwrap();
                }
            }
            buffer.write_str("\"").unwrap();
        }
        buffer.write_str(">").unwrap();
        buffer
    }
}


/// Extracts text from a slice of markdown events
fn get_text(parser_slice: &[Event]) -> String {
    let mut title = String::new(); 

    for event in parser_slice.iter() {
        match  event {
            Event::Text(text) | Event::Code(text)=> title += text,
            _ => continue,
        }
    }

    title
}


/// Returns a unique anchor for a heading
fn get_anchor(anchors: &[String], name: String, level: u16) -> String {
    if level == 0 && !anchors.contains(&name) {
        return name
    }

    let new_anchor = format!("{}-{}", name, level + 1);
    if !anchors.contains(&new_anchor) {
        return new_anchor;
    }

    // if the anchor is not unique, try a different one

    get_anchor(anchors, new_anchor, level + 1)
}

fn get_heading_refs(events: &[Event]) -> Vec<HeadingStruct> {
    let mut heading_refs = vec![]; 

    for (i, event) in events.iter().enumerate() {
        match event {
            Event::Start(Tag::Heading(level, _, _)) => {
                heading_refs.push(
                    HeadingStruct::new(
                        i, 
                        *level as u32, 
                        None, // The ID is not directly available in the current API
                        &[], // Classes are not directly available in the current API
                    ));
            },
            Event::End(Tag::Heading(_, _, _)) => {
                heading_refs.last_mut().expect("Heading end before start?").end_idx = i;
            },
            _ => {}
        }
    }

    heading_refs
}

/// Converts markdown text to HTML
pub fn markdown_to_html(content: &str, context: &RenderContext) -> Result<Rendered> {
    let path = context.tera_context
        .get("page")
        .or_else(|| context.tera_context.get("section"))
        .map(|x| x.as_object().unwrap().get("relative_path").unwrap().as_str().unwrap());
    let mut html = String::with_capacity(content.len());
    let summary = None;
    // Set while parsing
    let error = None;

    let mut opts = Options::empty();
    let internal_links = Vec::new(); 
    let external_links = Vec::new();

    let mut headings: Vec<Heading> = Vec::new();


    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TASKLISTS);
    opts.insert(Options::ENABLE_HEADING_ATTRIBUTES);


    {
        let mut events = Vec::new();
        for (event, mut range) in Parser::new_ext(content, opts).into_offset_iter() {
            match event {
                Event::Start(tag) => {

                },
                Event::End(tag) => {
                    
                },
                Event::Text(cow_str) => {
                    // if let Some(ref mut code_block) = code_block {
                    //     let stack_start = events.len(); 


                    //     events.truncate(stack_start);
                    // }
                    // else {

                    // }
                },
                Event::Code(cow_str) => {
                    
                },
                Event::Html(cow_str) => {
                    
                },
                Event::FootnoteReference(cow_str) => {
                    
                },
                Event::SoftBreak => {
                    
                },
                Event::HardBreak => {
                    
                },
                Event::Rule => {
                    
                },
                Event::TaskListMarker(_) => {
                    
                },
                _ => events.push(event)
            }
        }

        events.retain(|e | match e {
            Event::Text(text) | Event::Html(text) => !text.is_empty(),
            _ => true,
        });

        let heading_refs = get_heading_refs(&events); 

        let mut anchors_to_insert: Vec<(usize, Event<'_>)> = vec![];
        let mut inserted_anchors = vec![];
        for heading in &heading_refs {
            if let Some(e) = &heading.id {
                 // This line of code creates a new owned copy of the string `e` and pushes it into the 
                 //`inserted_anchors` vector. This is done to ensure that we have a complete list of all the anchors that were inserted. 
                inserted_anchors.push(e.to_owned());
               
            }
        }


        for mut heading_ref in heading_refs {
            let  start_idx = heading_ref.start_idx; 
            let end_idx = heading_ref.end_idx; 
            let title = get_text(&events[start_idx + 1..end_idx]);

            if heading_ref.id.is_none() {
                heading_ref.id = Some(get_anchor(&inserted_anchors, title.clone(), 0));
            }


            inserted_anchors.push(heading_ref.id.clone().unwrap());
            let id = inserted_anchors.last().unwrap();

            let html = heading_ref.format_to_html(id);
            events[start_idx] = Event::Html(html.into()); 

            let permalink = format!("{}#{}", context.current_page_permalink, id); 
            let h = Heading  {
                level: heading_ref.level, 
                id: id.to_owned(), 
                title,
                permalink, 
                children: Vec::new(),
            };

            //headings.
            headings.push(h);
        }

        let continue_reading = events
            .iter()
            .position(|e| matches!(e, Event::Html(CowStr::Borrowed(CONTINUE_READING))))
            .unwrap_or(events.len());


        // This line creates a new empty vector to track HTML tags
        let mut tags: Vec<Tag> = Vec::new();
        for event in &events[..continue_reading] {
            match event {
                Event::Html(_) => {},
                Event::Start(tag) => tags.push(tag.clone()),
                Event::End(end_tag) => {
                  tags.truncate(tags.iter().rposition(|x|*x == *end_tag).unwrap_or(0));
                }, 
                _ => {}
            }
        }

        //let parser = Parser::new_ext(content, opts);
        //events.extend(parser);
        // The `into_iter()` method consumes the `events` vector and returns an iterator over its elements. The `mut` keyword indicates that the iterator can mutate the elements.
        // By assigning the iterator to the `events` variable, we can use the iterator to iterate over the elements of the vector one by one.
        // The `mut` keyword is necessary because the `events` vector is being consumed and we want to be able to modify its elements if needed.
        let mut events = events.into_iter();

        cmark::html::push_html(&mut html, events.by_ref().take(continue_reading));
       // events.for_each(drop);
    }

    if let Some(e) = error {
        Err(e)
    }
    else {
        Ok(Rendered {
            summary, 
            body: html, 
            // toc, 
            internal_links, 
            external_links
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markdown_to_html() -> Result<()> {
        let markdown = "# Hello, World!";
        let html = to_html(markdown)?;
        assert_eq!(html, "<h1>Hello, World!</h1>\n");
        Ok(())
    }
}