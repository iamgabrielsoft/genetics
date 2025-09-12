
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Mode {
    Build, 
    Serve,
    Check,
}

#[derive(Serialize)]
pub struct SerializedConfig<'a> {
    base_url: &'a str,
    title: Option<&'a str>,
    description: Option<&'a str>,
    mode: Mode,

}

pub struct Config {
    /// base url of the site 
    pub base_ul: String,

    /// title of the site 
    pub title: Option<String>, 

    /// description of the site 
    pub description: Option<String>, 

    pub mode: Mode
}

impl Config {
    
    pub fn is_in_check_mode(&self) -> bool {
        self.mode == Mode::Check
    }
    pub fn enable_serve_mode(&mut self) {
        self.mode = Mode::Serve; 
    }

    pub fn enable_check_mode(&mut self) {
        self.mode = Mode::Check;
        
    } 
    /// Serializes the config
    pub fn serizlie(&self) -> SerializedConfig<'_> {
        SerializedConfig {
            base_url: &self.base_ul,
            title: self.title.as_deref(),
            description: self.description.as_deref(),
            mode: self.mode,
        }
    }
}