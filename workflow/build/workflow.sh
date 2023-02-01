#!/bin/sh
cd /usr/src/composer
go run main.go generate  -c /input.yaml
mv /usr/src/composer/workflow.wasm /usr/src/workflow/workflow.wasm

exec "$@"