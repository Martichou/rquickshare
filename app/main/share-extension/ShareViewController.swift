import AppKit
import UniformTypeIdentifiers

final class ShareViewController: NSViewController {
    private let lock = NSLock()
    private var remaining = 0
    private var urls: [URL] = []
    private var finished = false
    private var context: NSExtensionContext?

    override func loadView() {
        view = NSView(frame: NSRect(x: 0, y: 0, width: 1, height: 1))
    }

    override func beginRequest(with context: NSExtensionContext) {
        self.context = context

        let attachments = (context.inputItems as? [NSExtensionItem])?
            .flatMap { $0.attachments ?? [] } ?? []

        guard !attachments.isEmpty else {
            context.completeRequest(returningItems: [], completionHandler: nil)
            return
        }

        remaining = attachments.count

        for provider in attachments {
            let type = UTType.fileURL.identifier

            guard provider.hasItemConformingToTypeIdentifier(type) else {
                itemFinished(nil)
                continue
            }

            provider.loadItem(forTypeIdentifier: type, options: nil) { [weak self] item, error in
                if let error {
                    NSLog("RQuickShare share extension: could not load item: \(error)")
                }
                self?.itemFinished(Self.url(from: item))
            }
        }
    }

    private static func url(from item: NSSecureCoding?) -> URL? {
        if let url = item as? URL {
            return url
        }

        if let url = item as? NSURL {
            return url as URL
        }

        if let data = item as? Data {
            if let url = URL(dataRepresentation: data, relativeTo: nil) {
                return url
            }
            if let string = String(data: data, encoding: .utf8) {
                return url(fromString: string)
            }
        }

        if let string = item as? String {
            return url(fromString: string)
        }

        return nil
    }

    private static func url(fromString string: String) -> URL? {
        if string.hasPrefix("/") {
            return URL(fileURLWithPath: string)
        }
        return URL(string: string)
    }

    private func itemFinished(_ url: URL?) {
        lock.lock()
        if let url, url.isFileURL, !urls.contains(where: { $0.path == url.path }) {
            urls.append(url)
        }
        remaining -= 1
        let done = remaining == 0
        lock.unlock()

        if done {
            finish()
        }
    }

    private func finish() {
        lock.lock()
        guard !finished else {
            lock.unlock()
            return
        }
        finished = true
        let items = urls
        lock.unlock()

        DispatchQueue.main.async { [weak self] in
            guard let self else {
                return
            }
            if !items.isEmpty {
                self.openInApp(items)
            }
            self.context?.completeRequest(returningItems: [], completionHandler: nil)
        }
    }

    private func openInApp(_ files: [URL]) {
        let appURL = Bundle.main.bundleURL
            .deletingLastPathComponent()
            .deletingLastPathComponent()
            .deletingLastPathComponent()
        let configuration = NSWorkspace.OpenConfiguration()
        configuration.activates = true

        // The sandbox drops OpenConfiguration.arguments, so hand the files over
        // as an open-documents event instead, like a drop on the Dock icon.
        NSWorkspace.shared.open(files, withApplicationAt: appURL, configuration: configuration) { _, error in
            if let error {
                NSLog("RQuickShare share extension: could not open the app: \(error)")
            }
        }
    }
}
