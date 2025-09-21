import Foundation
import Carbon

// Define constants to match Rust's InputSourceCategory bitflags
let CATEGORY_KEYBOARD_BIT: Int32 = 1 << 0 // 1
let CATEGORY_PALETTE_BIT: Int32 = 1 << 1  // 2

// This function needs to be called once before using the others.
@_cdecl("initialize_input_source_manager")
public func initialize_input_source_manager() {
    InputSourceManager.initialize()
}

// Gets the current input source ID.
// The returned string must be freed by the caller using `free_string`.
@_cdecl("get_current_input_source_id_swift")
public func getCurrentInputSourceID() -> UnsafeMutablePointer<CChar>? {
    let currentSourceID = InputSourceManager.getCurrentSource().id
    return strdup(currentSourceID) // Use strdup to create a C-string on the heap
}

// Selects an input source by its ID.
// Returns 0 on success, -1 if source not found, -2 if switch failed.
@_cdecl("select_input_source_by_id")
public func selectInputSourceByID(targetID: UnsafePointer<CChar>) -> Int32 {
    let targetIDString = String(cString: targetID)
    if let source = InputSourceManager.getInputSource(name: targetIDString) {
        source.select()
        // Verify the switch for robustness
        let newSourceID = InputSourceManager.getCurrentSource().id
        return newSourceID == targetIDString ? 0 : -2 // 0 for success, -2 for switch failed
    } else {
        return -1 // Source not found
    }
}

// A function to free the string returned by `get_current_input_source_id_swift`.
@_cdecl("free_string")
public func free_string(ptr: UnsafeMutablePointer<CChar>?) {
    free(ptr)
}

@_cdecl("get_available_input_source_ids")
public func getAvailableInputSourceIDs(category_type: Int32) -> UnsafeMutablePointer<CChar>? {
    // InputSourceManager.initialize() // Removed: Assumes manager is initialized by Rust

    var targetIDs: [String] = []
    if (category_type & CATEGORY_KEYBOARD_BIT) != 0 {
        targetIDs.append(contentsOf: InputSourceManager.keyboardInputSources.map { $0.id })
    }
    if (category_type & CATEGORY_PALETTE_BIT) != 0 {
        targetIDs.append(contentsOf: InputSourceManager.paletteInputSources.map { $0.id })
    }

    let allIDs = targetIDs.joined(separator: ",")
    return strdup(allIDs)
}
