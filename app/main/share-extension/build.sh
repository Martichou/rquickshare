#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")"

APPEX_NAME="RQuickShareShare.appex"
EXECUTABLE="RQuickShareShare"
MODULE_NAME="RQuickShareShare"
# Tauri exports MACOSX_DEPLOYMENT_TARGET for the Rust build (10.13 by default),
# so the extension uses its own variable to keep UTType available.
DEPLOYMENT_TARGET="${SHARE_EXTENSION_DEPLOYMENT_TARGET:-12.0}"
IDENTITY="${APPLE_SIGNING_IDENTITY:--}"
BUILD_DIR="build"
APPEX_DIR="$BUILD_DIR/$APPEX_NAME"

VERSION="$(sed -n 's/^version = "\(.*\)"/\1/p' ../src-tauri/Cargo.toml | head -n 1)"
VERSION="${VERSION:-0.0.0}"

if ! command -v xcrun >/dev/null 2>&1; then
    echo "error: xcrun not found. The share extension can only be built on macOS." >&2
    exit 1
fi

ARCH="$(uname -m)"
SDK_PATH="$(xcrun --sdk macosx --show-sdk-path)"

rm -rf "$APPEX_DIR"
mkdir -p "$APPEX_DIR/Contents/MacOS" "$APPEX_DIR/Contents/Resources"

xcrun --sdk macosx swiftc \
    -O \
    -module-name "$MODULE_NAME" \
    -target "${ARCH}-apple-macos${DEPLOYMENT_TARGET}" \
    -sdk "$SDK_PATH" \
    -Xlinker -e -Xlinker _NSExtensionMain \
    -o "$APPEX_DIR/Contents/MacOS/$EXECUTABLE" \
    ShareViewController.swift

cp Info.plist "$APPEX_DIR/Contents/Info.plist"
plutil -replace CFBundleShortVersionString -string "$VERSION" "$APPEX_DIR/Contents/Info.plist"
plutil -replace CFBundleVersion -string "$VERSION" "$APPEX_DIR/Contents/Info.plist"

if [[ "$IDENTITY" == "-" ]]; then
    codesign --force --sign - \
        --entitlements ShareExtension.entitlements \
        --timestamp=none \
        "$APPEX_DIR"
else
    codesign --force --sign "$IDENTITY" \
        --entitlements ShareExtension.entitlements \
        --options runtime \
        --timestamp \
        "$APPEX_DIR"
fi

codesign --verify --strict "$APPEX_DIR"

echo "Built $APPEX_DIR (version $VERSION, identity $IDENTITY)"
