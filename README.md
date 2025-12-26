# Seno

A simple, lightweight progress bar library for Rust.

## Demo

![Seno Loader Demo](assets/loader.gif)

## Features

- **Easy to use**:  Simple API with method chaining
- **Customizable**: Change message, loading characters, speed, and rounds
- **Clean output**: Properly clears terminal lines after completion

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
seno = "0.1.0"
```

## Usage

```rust
use seno::Loader;

fn main() {
    let loader = Loader::new();
    loader.run()
        .msg("Loading...")
        .assets(["|", "/", "-", "\\"])
        .rounds(5)
        .wait(0.1)
        .finish("Done!");
}
```

## License

Apache-2.0