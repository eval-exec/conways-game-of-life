#!/usr/bin/env bash
set -Eevu pipefail
cd wasm
wasm-pack build --target web
rm -v ./pkg/.gitignore

