#!/bin/sh

workflow=$(mktemp).yaml
cat > "$workflow"

cd /usr/src/composer

case $@ in
    generate)
    go run main.go generate -c $workflow
    cat /usr/src/composer/workflow.wasm
    ;;

    test)
    go run main.go test -c $workflow
    ;;
esac


