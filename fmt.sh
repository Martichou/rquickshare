#!/bin/bash

for dir in core_lib frontend/src-tauri; do
  find "$dir" -name '*.rs' -not -path "*/target/*" -exec rustfmt {} +
done

# Check if there are changes in the /frontend/src directory
if git diff --name-only HEAD | grep '^frontend/src/' >/dev/null; then
    echo "Changes detected in /frontend/src. Executing script..."

    # Navigate to the frontend/src-tauri directory
    cd frontend/src-tauri

    # Run your commands
    pnpm lint --fix
else
    echo "No changes detected in /frontend/src."
fi
