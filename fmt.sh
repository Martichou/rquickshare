#!/bin/bash

for dir in core_lib app/legacy/src-tauri; do
  find "$dir" -name '*.rs' -not -path "*/target/*" -exec rustfmt {} +
done

# Check if there are changes in the /app/legacy/src directory
if git diff --name-only HEAD | grep '^app/legacy/src/' >/dev/null; then
    echo "Changes detected in /app/legacy/src. Executing script..."

    # Navigate to the app/legacy/src-tauri directory
    cd app/legacy/src-tauri

    # Run your commands
    pnpm lint --fix
else
    echo "No changes detected in /app/legacy/src."
fi