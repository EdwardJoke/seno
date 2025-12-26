use super::Loader;

impl Loader {
    pub fn wait(mut self, wait_time: f64) -> Self {
        self.wait_time = wait_time;
        self
    }
}