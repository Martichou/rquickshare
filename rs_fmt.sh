#!/bin/bash

for dir in core_lib frontend/src-tauri; do
  find "$dir" -name '*.rs' -not -path "*/target/*" -exec rustfmt {} +
done