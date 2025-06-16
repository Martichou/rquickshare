#!/bin/bash

for dir in core_lib app/main/src-tauri; do
  find "$dir" -name '*.rs' -not -path "*/target/*" -exec rustfmt {} +
done

# Check if there are changes in the /app/main/src directory
if git diff --name-only HEAD | grep '^app/main/src/' >/dev/null; then
    echo "Changes detected in /app/main/src. Executing script..."

    # Navigate to the app/main directory
    cd app/main

    # Run your commands
    pnpm lint --fix

    cd ../..
else
    echo "No changes detected in /app/main/src."
fi