#!/bin/bash

for dir in core_lib app/nogui app/gui/src-tauri; do
  find "$dir" -name '*.rs' -not -path "*/target/*" -exec rustfmt {} +
done

# Check if there are changes in the app/gui/src directory
if git diff --name-only HEAD | grep '^app/gui/src/' >/dev/null; then
    echo "Changes detected in /app/gui/src. Executing script..."

    # Navigate to the app/gui/src-tauri directory
    cd app/gui/src-tauri

    # Run your commands
    pnpm lint --fix
else
    echo "No changes detected in /app/gui/src."
fi