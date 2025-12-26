use super::Loader;

impl Loader {
    pub fn finish(mut self, finish_message: &str) {
        self.finish_message = finish_message.to_string();
        self.execute();
    }
}