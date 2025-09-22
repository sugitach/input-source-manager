import Foundation
import Carbon



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
        let currentSourceIDBeforeSelect = InputSourceManager.getCurrentSource().id // Capture before select
        source.select()



        // For non-palette sources, verify the switch
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
public func getAvailableInputSourceIDs() -> UnsafeMutablePointer<CChar>? {
    let targetIDs = InputSourceManager.keyboardInputSources.map { $0.id }

    let allIDs = targetIDs.joined(separator: ",")
    return strdup(allIDs)
}
