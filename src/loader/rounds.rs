use super::Loader;

impl Loader {
    pub fn rounds(mut self, rounds: usize) -> Self {
        self.rounds = rounds;
        self
    }
}