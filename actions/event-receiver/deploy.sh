#!/bin/bash

# To run this command
# ./deploy.sh --openwhiskApiHost <openwhiskApiHost> --openwhiskApiKey <openwhiskApiKey> --openwhiskNamespace <openwhiskNamespace>

openwhiskApiHost=${openwhiskApiHost:-https://localhost:31001}
openwhiskApiKey=${openwhiskApiKey:-23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP}
openwhiskNamespace=${openwhiskNamespace:-guest}
actionHome=${actionHome:-actions/event-receiver}
if [[ "$OSTYPE" == "darwin"* ]]; then
    WSK_CLI="$PWD/binaries/wsk-darwin"
else
    WSK_CLI="$PWD/binaries/wsk"
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

chmod +x WSK_CLI

$WSK_CLI -i --apihost "$openwhiskApiHost" action update --kind nodejs:default event-receiver "$PACKAGE_HOME/event-receiver.zip" \
    --auth "$openwhiskApiKey"

if [ -e ./temp/event-receiver ]; then
    echo "Clearing temporary packed action file."
    rm -rf ./temp/event-receiver
fi