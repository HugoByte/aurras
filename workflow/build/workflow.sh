#!/bin/sh

workflow=$(mktemp).yaml
cat > "$workflow"

cd /usr/src/composer
go run main.go generate -c $workflow

cat /usr/src/composer/workflow.wasm