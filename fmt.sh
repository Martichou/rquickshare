#!/bin/bash

for dir in core_lib app/legacy/src-tauri app/main/src-tauri; do
  find "$dir" -name '*.rs' -not -path "*/target/*" -exec rustfmt {} +
done

# Check if there are changes in the /app/legacy/src directory
if git diff --name-only HEAD | grep '^app/legacy/src/' >/dev/null; then
    echo "Changes detected in /app/legacy/src. Executing script..."

    # Navigate to the app/legacy/src-tauri directory
    cd app/legacy/src-tauri

    # Run your commands
    pnpm lint --fix

    cd ../../..
else
    echo "No changes detected in /app/legacy/src."
fi

# Check if there are changes in the /app/main/src directory
if git diff --name-only HEAD | grep '^app/main/src/' >/dev/null; then
    echo "Changes detected in /app/main/src. Executing script..."

    # Navigate to the app/main/src-tauri directory
    cd app/main/src-tauri

    # Run your commands
    pnpm lint --fix

    cd ../../..
else
    echo "No changes detected in /app/main/src."
fi