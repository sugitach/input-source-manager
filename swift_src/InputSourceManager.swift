import Cocoa
import Foundation
import Carbon

class InputSource: Equatable {
    static func == (
        lhs: InputSource,
        rhs: InputSource
    ) -> Bool {
        return lhs.id == rhs.id
    }

    let tisInputSource: TISInputSource

    var id: String {
        return tisInputSource.id
    }

    var isCJKV: Bool {
        if let lang = tisInputSource.sourceLanguages.first {
            return lang == "ko" ||
                   lang == "ja" ||
                   lang == "vi" ||
                   lang.hasPrefix("zh")
        }
        return false
    }

    init(tisInputSource: TISInputSource) {
        self.tisInputSource = tisInputSource
    }

    func select() {
        let currentSource = InputSourceManager.getCurrentSource()
        if currentSource.id == self.id {
            return
        }


}

class InputSourceManager {
    static var inputSources: [InputSource] = [] // Keep this as the combined list
    static var keyboardInputSources: [InputSource] = []

    static var isInitialized = false // Add this flag
    static var waitTimeMs: Int = -1  // less than 0 means using default
    static var level: Int = 1

    static func initialize() {
        if isInitialized { return }

        let inputSourceList = TISCreateInputSourceList(
            nil, false
        ).takeRetainedValue() as! [TISInputSource]

        keyboardInputSources = inputSourceList
            .filter {
                $0.isSelectable && $0.category == TISInputSource.Category.keyboardInputSource
            }
            .map { InputSource(tisInputSource: $0) }

        inputSources = keyboardInputSources // Combine them
        isInitialized = true // Set flag to true after initialization
    }

    static func getCurrentSource() -> InputSource {
        return InputSource(
            tisInputSource:
                TISCopyCurrentKeyboardInputSource()
                .takeRetainedValue()
        )
    }

    static func getInputSource(name: String) -> InputSource? {
        return inputSources.first { $0.id == name }
    }
}

extension TISInputSource {
    enum Category {
        static var keyboardInputSource: String {
            return kTISCategoryKeyboardInputSource as String
        }

    }

    private func getProperty(_ key: CFString) -> AnyObject? {
        if let cfType = TISGetInputSourceProperty(self, key) {
            return Unmanaged<AnyObject>
                .fromOpaque(cfType)
                .takeUnretainedValue()
        }
        return nil
    }

    var id: String {
        return getProperty(kTISPropertyInputSourceID) as! String
    }

    var category: String {
        return getProperty(kTISPropertyInputSourceCategory) as! String
    }

    var isSelectable: Bool {
        return getProperty(
            kTISPropertyInputSourceIsSelectCapable
        ) as! Bool
    }

    var sourceLanguages: [String] {
        return getProperty(kTISPropertyInputSourceLanguages) as! [String]
    }


}
