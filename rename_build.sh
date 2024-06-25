#!/bin/bash

# Check if the base directory argument is provided
if [ -z "$1" ] || [ -z "$2" ]; then
  echo "Usage: $0 <base_directory> <build_tauri_ver>"
  exit 1
fi

base_dir="$1"
tauri_ver="$2"

# Find all relevant files in the base directory
files=$(find "$base_dir" -type f \( -name "*.deb" -o -name "*.rpm" -o -name "*.AppImage" \))

glib_ver=$(ldd --version | head -n1 | awk '{print $NF}')
echo "GLIBC version: ${glib_ver}"

# Loop through each file
for file in $files; do
    # Extract the directory, filename and extension
    dir=$(dirname "$file")
    filename=$(basename "$file")
    extension="${filename##*.}"

    # Extract the version and anything part
    if [[ "$filename" =~ ([Rr]-?[Qq]uick[Ss]hare)_?([0-9]+\.[0-9]+\.[0-9]+)_(.*)\.${extension} ]]; then
        version="v${BASH_REMATCH[2]}"
        anything="${BASH_REMATCH[3]}"
    elif [[ "$filename" =~ ([Rr]-?[Qq]uick[Ss]hare)-([0-9]+\.[0-9]+\.[0-9]+)-([0-9]+)\.(.*)\.${extension} ]]; then
        version="v${BASH_REMATCH[2]}"
        anything="${BASH_REMATCH[3]}-${BASH_REMATCH[4]}"
    elif [[ "$filename" =~ ([rR]([-_][qQ]uick[-_][sS]hare))_?([0-9]+\.[0-9]+\.[0-9]+)_?(.*)\.${extension} ]]; then
        version="v${BASH_REMATCH[3]}"
        anything="${BASH_REMATCH[4]}"
    else
        echo "Filename does not match the expected pattern: $filename"
        continue
    fi

    # Construct the new filename
    new_filename="r-quick-share_${version}_glibc-${glib_ver}_${anything}.${extension}"

    # Rename the file
    mv "$file" "$dir/$new_filename"

    echo "Renamed $filename to $dir/$new_filename"
done

echo "Renaming completed."