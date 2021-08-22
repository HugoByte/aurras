#!/bin/bash

# To run this command
# ./deploy.sh --openwhiskApiHost <openwhiskApiHost> --openwhiskApiKey <openwhiskApiKey> --openwhiskNamespace <openwhiskNamespace>

openwhiskApiHost=${openwhiskApiHost:-https://localhost:31001}
openwhiskApiKey=${openwhiskApiKey:-23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP}
openwhiskNamespace=${openwhiskNamespace:-guest}
actionHome=${actionHome:-actions/event-receiver}
WSK_CLI="wsk"

if ! command -v $WSK_CLI &> /dev/null
then
    echo "wsk cli not found in path. Please get the cli from https://github.com/apache/openwhisk-cli/releases"
    exit
fi
    
PACKAGE_HOME="$PWD/${actionHome}/temp/event-receiver"

while [ $# -gt 0 ]; do
    if [[ $1 == *"--"* ]]; then
        param="${1/--/}"
        declare $param="${2%/}"
    fi

    shift
done

set -e

cd "$PWD/$actionHome"

echo "Installing Dependencies"
yarn install

echo "Building Source"
yarn build

if [ -e ./temp/event-receiver ]; then
    echo "Clearing previously packed action file."
    rm -rf ./temp/event-receiver
fi

mkdir -p ./temp/event-receiver
echo "Creating temporary directory"

cp -r ./package.json ./dist ./temp/event-receiver
echo "Copying files to temporary directory"

cd ./temp/event-receiver

yarn install --production=true

zip -r event-receiver.zip *

$WSK_CLI -i --apihost "$openwhiskApiHost" action update --kind nodejs:default event-receiver "$PACKAGE_HOME/event-receiver.zip" \
    --auth "$openwhiskApiKey" --param event_producer "event-producer"

if [ -e ./temp/event-receiver ]; then
    echo "Clearing temporary packed action file."
    rm -rf ./temp/event-receiver
fi