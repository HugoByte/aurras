#!/bin/bash

# To run this command
# ./deploy.sh --openwhiskApiHost <openwhiskApiHost> --openwhiskApiKey <openwhiskApiKey> --openwhiskNamespace <openwhiskNamespace>

openwhiskApiHost=${openwhiskApiHost:-https://localhost:31001}
openwhiskApiKey=${openwhiskApiKey:-23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP}
openwhiskNamespace=${openwhiskNamespace:-guest}
actionHome=${actionHome:-actions/event-producer}
WSK_CLI="wsk"
if ! command -v $WSK_CLI &> /dev/null
then
    echo "wsk cli not found in path. Please get the cli from https://github.com/apache/openwhisk-cli/releases"
    exit
fi
ACTION="event-producer"

while [ $# -gt 0 ]; do
    if [[ $1 == *"--"* ]]; then
        param="${1/--/}"
        declare $param="${2%/}"
    fi

    shift
done

set -e

cd "$PWD/$actionHome"


$WSK_CLI -i --apihost "$openwhiskApiHost" action update ${ACTION} "$PWD/event-producer.py" \
    --auth "$openwhiskApiKey"