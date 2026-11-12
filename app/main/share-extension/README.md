# macOS Finder share extension

This is the macOS Share extension that adds RQuickShare to Finder's right-click Share menu, the Share button in Quick
Look and any other app that uses the system share sheet.

The extension is a small AppKit bundle (`RQuickShareShare.appex`) embedded in the app under
`rquickshare.app/Contents/PlugIns/`. It collects the selected file URLs and opens them with the containing app, the
same open-documents event a drop on the Dock icon sends. The app queues the files from that event
(`RunEvent::Opened`) until the front end takes them, then stages them like a drop and opens the device list. If the
app is already running, the files go to that copy.

A sandboxed extension can't pass command-line arguments, since macOS drops `NSWorkspace.OpenConfiguration.arguments`
for it, which is why it opens the files instead. For macOS to deliver them, the app declares that it can open any file
(`public.data`) with the `Alternate` rank. It is never the default app for a file type, but it does show up under
"Open With".

## Building

The extension is built automatically as part of a macOS bundle build:

```sh
cd app/main
pnpm build
```

`pnpm build` runs `share-extension/build.sh` before Tauri packages the app. The script compiles the Swift source with
`swiftc`, assembles the `.appex`, and signs it with `APPLE_SIGNING_IDENTITY` when set, falling back to ad-hoc signing.
Tauri's `bundle.macOS.files` setting then copies the result to `Contents/PlugIns` (see
`src-tauri/tauri.macos.conf.json`).

`pnpm dev` does not build or install the extension because there is no app bundle to embed it in. Use a bundled build
to test Finder integration.

## Installing and troubleshooting

Install the built `rquickshare.app` in `/Applications` and launch it once. macOS should pick up the extension
automatically. To check that the system registered it:

```sh
pluginkit -m -p com.apple.share-services | grep rquickshare
```

If it is missing, register it manually:

```sh
pluginkit -a /Applications/rquickshare.app/Contents/PlugIns/RQuickShareShare.appex
```

If it is registered but does not show up in the Share menu, enable it in System Settings > General > Login Items &
Extensions > Sharing, or from the shell:

```sh
pluginkit -e use -i dev.mandre.rquickshare.share
```

Ad-hoc signed builds get a new signature on every build, so macOS may keep an old registration around. If the Finder
menu does not update, run `killall Finder` or remove the stale entry with `pluginkit -r <path-to-old.appex>` before
registering the new one.

## Files

- `ShareViewController.swift` collects file URLs and opens them with the app
- `Info.plist` contains the extension metadata and activation rule (`com.apple.share-services`)
- `ShareExtension.entitlements` contains the App Sandbox entitlement required for app extensions
- `build.sh` builds, stamps the version and signs the `.appex`
