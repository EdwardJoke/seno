use seno::Loader;

fn main() {
    let loader = Loader::new();
    loader.run()                       // Start a loader
        .msg("Runing...")              // Message to display when loading
        .assets(["|", "/", "-", "\\"]) // Assets to display
        .rounds(9)                     // Number of rounds to display each assets
        .wait(0.1)                     // Wait time between each asset
        .finish("Ok!");                // Message to display when finish
}