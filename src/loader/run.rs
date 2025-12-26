use super::Loader;
use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

impl Loader {
    pub fn run(self) -> Self {
        self
    }
    
    pub fn execute(self) {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        
        // Calculate total iterations
        let total_iterations = self.assets.len() * self.rounds;
        
        for i in 0..total_iterations {
            // Get asset
            let asset_index = i % self.assets.len();
            let asset = self.assets[asset_index];
            
            // Clear current line and print new loader
            let _ = write!(handle, "\r{} {}", self.message, asset);
            let _ = handle.flush();
            
            // Wait for next iteration
            sleep(Duration::from_secs_f64(self.wait_time));
        }
        
        // Clear line and print finish message
        let _ = write!(handle, "\r\x1b[2K{}\n", self.finish_message);
        let _ = handle.flush();
    }
}