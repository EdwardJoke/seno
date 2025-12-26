mod assets;
mod finish;
mod rounds;
mod run;
mod wait;

pub struct Loader {
    pub(crate) message: String,
    pub(crate) assets: Vec<&'static str>,
    pub(crate) rounds: usize,
    pub(crate) wait_time: f64,
    pub(crate) finish_message: String,
}

impl Loader {
    pub fn new() -> Self {
        Self {
            message: String::from("Loading..."),
            assets: vec!["|", "/", "-", "\\"],
            rounds: 1,
            wait_time: 0.1,
            finish_message: String::from("Done!"),
        }
    }
    
    pub fn msg(mut self, message: &str) -> Self {
        self.message = message.to_string();
        self
    }
}

impl Default for Loader {
    fn default() -> Self {
        Self::new()
    }
}