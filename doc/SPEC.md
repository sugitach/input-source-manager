# Specification for input-source-manager

## 1. Overview

This document specifies the `input-source-manager` Rust library and its accompanying command-line interface (CLI) tool, `macism-rust`. The project aims to provide a robust and reliable way to manage macOS input sources from Rust applications, leveraging native Swift APIs for core functionality.

## 2. CLI Specification (`macism-rust`)

The `macism-rust` command-line tool provides the following functionalities:

- **Command Name:** `macism-rust`

- **Default Behavior (no arguments): Get Current Input Source ID**
  ```bash
  macism-rust
  ```
  Outputs the ID of the currently active input source to standard output.

- **Version Information:**
  ```bash
  macism-rust --version
  # or
  macism-rust -v
  ```
  Outputs the version of the `macism-rust` tool.

- **List Available Input Source IDs:**
  ```bash
  macism-rust --list
  # or
  macism-rust -l
  ```
  Outputs a list of all available input source IDs on the system, one ID per line.

- **Explicitly Get Current Input Source ID:**
  ```bash
  macism-rust get
  ```
  Outputs the ID of the currently active input source to standard output. This is functionally identical to the default behavior but provides an explicit subcommand.

- **Set Input Source:**
  ```bash
  macism-rust set <INPUT_SOURCE_ID>
  ```
  Attempts to switch the active input source to the one specified by `<INPUT_SOURCE_ID>`. Outputs the ID of the newly active input source on success. If the specified ID is not found or the switch fails, an error message is printed to standard error.

## 3. Library API Specification (`input-source-manager`)

The `input-source-manager` Rust library provides the following public API:

### 3.1. Enums

#### `pub enum InputSourceError`
Represents errors that can occur when interacting with the input source manager.

- `SwiftError(i32)`: An error occurred within the underlying Swift code, with the given error code.
- `SourceNotFound`: The requested input source ID was not found among the available sources.
- `SwitchFailed`: The attempt to switch the input source failed.
- `InternalError`: An internal error occurred, such as a failure in string conversion.

#### `pub enum SwitchResult`
Represents the result of an input source switch operation.

- `Switched`: The input source was successfully switched.
- `NotSwitched`: The input source was not switched (e.g., it was already the target source).

### 3.2. Functions

#### `pub fn initialize()`
Initializes the underlying Swift InputSourceManager. This function must be called once before using any other functions in this library. It sets up the internal list of available input sources.

#### `pub fn get_current_input_source_id() -> Result<String, InputSourceError>`
Retrieves the ID of the currently active input source.

Returns `Ok(String)` containing the ID of the current input source on success, or an `InputSourceError` if the operation fails.

#### `pub fn switch_input_source(sources: &[String]) -> Result<(SwitchResult, String), InputSourceError>`
Switches the input source based on a provided list of source IDs.

The function determines the next input source in the list based on the current active source. If the current source is not in the list, it switches to the first source in the list. If the current source is the last in the list, it cycles back to the first.

Returns `Ok((SwitchResult, String))` containing whether a switch occurred and the new source ID, or an `InputSourceError` if the operation fails.

#### `pub fn get_available_ids() -> Result<Vec<String>, InputSourceError>`
Returns a list of all available input source IDs.

Returns `Ok(Vec<String>)` containing a list of input source IDs on success, or an `InputSourceError` if the operation fails.

#### `pub fn set_input_source(id: &str) -> Result<String, InputSourceError>`
Directly sets the input source to the specified ID.

This function attempts to switch the input source to the exact ID provided. It does not cycle through a list of sources.

Returns `Ok(String)` containing the new active source ID on success, or an `InputSourceError` if the operation fails (e.g., source not found).

## 4. Architecture

This library employs a hybrid architecture to interact with macOS input source services:

1.  **Swift Bridge:** The core logic for interacting with macOS's native Text Input Source Services (TIS) APIs is implemented in Swift. This includes functions for getting the current input source, selecting a new one, and listing available sources.
2.  **Rust FFI:** Rust communicates with the Swift code via a C-compatible Foreign Function Interface (FFI). The Swift source files (`InputSourceManager.swift`, `rust_bridge.swift`) are compiled into a static library (`.a` file).
3.  **`build.rs`:** A custom build script (`build.rs`) written in Rust orchestrates the compilation of the Swift files and their linking into the main Rust project. This script ensures that `swiftc` is invoked correctly, object files are archived, and the resulting static library is properly linked with the Rust executable, including necessary Swift runtime libraries and macOS frameworks (`ApplicationServices`, `CoreFoundation`, `AppKit`).

## 5. License

MIT License