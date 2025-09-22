use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// Declare the C-callable functions from our Swift library.
extern "C" {
    fn initialize_input_source_manager();
    fn get_current_input_source_id_swift() -> *mut c_char;
    fn select_input_source_by_id(targetID: *const c_char) -> i32;
    fn free_string(ptr: *mut c_char);
    fn get_available_input_source_ids() -> *mut c_char;
}





/// Represents errors that can occur when interacting with the input source manager.
#[derive(Debug)]
pub enum InputSourceError {
    /// An error occurred within the underlying Swift code, with the given error code.
    SwiftError(i32),
    /// The requested input source ID was not found among the available sources.
    SourceNotFound,
    /// The attempt to switch the input source failed.
    SwitchFailed,
    /// An internal error occurred, such as a failure in string conversion.
    InternalError,
}

/// Represents the result of an input source switch operation.
#[derive(Debug, PartialEq)]
pub enum SwitchResult {
    /// The input source was successfully switched.
    Switched,
    /// The input source was not switched (e.g., it was already the target source).
    NotSwitched,
}

/// Initializes the underlying Swift InputSourceManager.
/// This function must be called once before using any other functions in this library.
/// It sets up the internal list of available input sources.
pub fn initialize() {
    unsafe {
        initialize_input_source_manager();
    }
}

/// Retrieves the ID of the currently active input source.
///
/// Returns `Ok(String)` containing the ID of the current input source on success,
/// or an `InputSourceError` if the operation fails.
pub fn get_current_input_source_id() -> Result<String, InputSourceError> {
    unsafe {
        let c_str_ptr = get_current_input_source_id_swift();
        if c_str_ptr.is_null() {
            return Err(InputSourceError::InternalError);
        }
        let c_str = CStr::from_ptr(c_str_ptr);
        let rust_str = c_str.to_string_lossy().into_owned();
        free_string(c_str_ptr); // Free the memory allocated by Swift's strdup
        Ok(rust_str)
    }
}

/// Switches the input source based on a provided list of source IDs.
///
/// The function determines the next input source in the list based on the current active source.
/// If the current source is not in the list, it switches to the first source in the list.
/// If the current source is the last in the list, it cycles back to the first.
///
/// Returns `Ok((SwitchResult, String))` containing whether a switch occurred and the new source ID,
/// or an `InputSourceError` if the operation fails.
pub fn switch_input_source(sources: &[String]) -> Result<(SwitchResult, String), InputSourceError> {
    let current_source_id = get_current_input_source_id()?;

    if sources.is_empty() {
        return Ok((SwitchResult::NotSwitched, current_source_id));
    }

    let current_index = sources.iter().position(|s| *s == current_source_id);

    let target_id = match current_index {
        None => &sources[0],
        Some(index) => {
            let next_index = (index + 1) % sources.len();
            &sources[next_index]
        }
    };

    if *target_id == current_source_id {
        return Ok((SwitchResult::NotSwitched, current_source_id));
    }

    let c_target_id =
        CString::new(target_id.as_str()).map_err(|_| InputSourceError::InternalError)?;

    let result_code = unsafe { select_input_source_by_id(c_target_id.as_ptr()) };

    match result_code {
        0 => {
            let new_id = get_current_input_source_id()?;
            Ok((SwitchResult::Switched, new_id))
        }
        -1 => Err(InputSourceError::SourceNotFound),
        -2 => Err(InputSourceError::SwitchFailed),
        _ => Err(InputSourceError::SwiftError(result_code)),
    }
}

/// Returns a list of input source IDs for a specified category.
///
/// Returns `Ok(Vec<String>)` containing a list of input source IDs on success,
/// or an `InputSourceError` if the operation fails.
pub fn get_available_ids() -> Result<Vec<String>, InputSourceError> {
    unsafe {
        let c_str_ptr = get_available_input_source_ids();
        if c_str_ptr.is_null() {
            return Ok(Vec::new()); // Return empty vec if no IDs or error
        }
        let c_str = CStr::from_ptr(c_str_ptr);
        let rust_str = c_str.to_string_lossy().into_owned();
        free_string(c_str_ptr); // Free the memory allocated by Swift's strdup
        let ids = rust_str
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();
        Ok(ids)
    }
}

/// Directly sets the input source to the specified ID.
///
/// This function attempts to switch the input source to the exact ID provided.
/// It does not cycle through a list of sources.
///
/// Returns `Ok(String)` containing the new active source ID on success,
/// or an `InputSourceError` if the operation fails (e.g., source not found).
pub fn set_input_source(id: &str) -> Result<String, InputSourceError> {
    let c_target_id = CString::new(id).map_err(|_| InputSourceError::InternalError)?;

    let result_code = unsafe { select_input_source_by_id(c_target_id.as_ptr()) };

    match result_code {
        0 => {
            let new_id = get_current_input_source_id()?;
            Ok(new_id)
        }
        -1 => Err(InputSourceError::SourceNotFound),
        -2 => Err(InputSourceError::SwitchFailed),
        _ => Err(InputSourceError::SwiftError(result_code)),
    }
}
