# input-source-manager

A Rust library and command-line tool (`macism-rust`) for managing macOS input sources. This project leverages Swift to interact with the macOS Text Input Source Services (TIS) APIs, providing a robust solution for getting, setting, and listing input sources from Rust.

## Features

- **Get Current Input Source:** Retrieve the ID of the currently active input source.
- **Set Input Source:** Directly switch to a specific input source by its ID.
- **Cycle Input Sources:** Cycle through a predefined list of input sources.
- **List Available Input Sources:** Get a list of all input source IDs available on the system.
- **`macism-rust` CLI:** A command-line interface tool that mimics the functionality of the original `macism` tool, with added features.

## `macism-rust` CLI Usage

The `macism-rust` command-line tool provides the following functionalities:

- **Get current input source ID:**
  ```bash
  macism-rust
  # or
  macism-rust get
  ```

- **List keyboard input source IDs:**
  ```bash
  macism-rust -l
  # or
  macism-rust --list
  ```

- **List palette input source IDs:**
  ```bash
  macism-rust -p
  # or
  macism-rust --palette
  ```

- **List all input source IDs:**
  ```bash
  macism-rust -l -p
  ```

- **Set input source to a specific ID:**
  ```bash
  macism-rust set jp.sourceforge.inputmethod.aquaskk.Hiragana
  ```

- **Print version information:**
  ```bash
  macism-rust --version
  ```

## Library Usage

Add `input-source-manager` to your `Cargo.toml`:

```toml
[dependencies]
input-source-manager = "0.1.0"
```

Example:

```rust
use input_source_manager::{self, get_current_input_source_id, set_input_source, get_available_ids, InputSourceCategory};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    self::initialize(); // Must be called once

    let current_id = get_current_input_source_id()?;
    println!("Current input source: {}", current_id);

    let available_ids = get_available_ids(InputSourceCategory::Keyboard)?;
    println!("Available keyboard input sources: {:?}", available_ids);

    // Example: Set to a specific input source
    if let Some(target_id) = available_ids.first() {
        println!("Setting input source to: {}", target_id);
        set_input_source(target_id)?; // Note: This will only work if target_id is a valid input source on your system
        println!("New current input source: {}", get_current_input_source_id()?);
    }

    Ok(())
}
```

## Building

This project requires `swiftc` to be available in your PATH, as it compiles Swift source files during the build process. Ensure you have Xcode or the Command Line Tools for Xcode installed.

To build the project:

```bash
cargo build --release
```

The `macism-rust` executable will be found in `target/release/`.

## How it Works

This library uses a hybrid approach:
1.  **Swift Bridge:** Core input source management logic is implemented in Swift, leveraging macOS's native Text Input Source Services (TIS) APIs.
2.  **Rust FFI:** Rust communicates with the Swift code via a C-compatible Foreign Function Interface (FFI). The Swift code is compiled into a static library (`.a` file), which is then linked into the Rust project.
3.  **`build.rs`:** A custom build script (`build.rs`) handles the compilation of Swift files and their linking into the Rust project, ensuring a seamless build experience.

## License

MIT License

## Acknowledgments

This project is heavily inspired by and builds upon the work of [laishulu](https://github.com/laishulu)'s `macism` project. We extend our sincere gratitude for their foundational work on macOS input source management.