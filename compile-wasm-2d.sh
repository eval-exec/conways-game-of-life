#!/usr/bin/env bash
set -Eevuo pipefail
cd wasm
rm -v pkg/*
wasm-pack build  --dev --target web --out-dir pkg
rm -v ./pkg/.gitignore

