#!/bin/bash

# To run this command
# ./deploy.sh --openwhiskApiHost <openwhiskApiHost> --openwhiskApiKey <openwhiskApiKey> --openwhiskNamespace <openwhiskNamespace>

openwhiskApiHost=${openwhiskApiHost:-https://localhost:31001}
openwhiskApiKey=${openwhiskApiKey:-23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP}
openwhiskNamespace=${openwhiskNamespace:-guest}
actionHome=${actionHome:-actions/kafka-provider-feed}
WSK_CLI="wsk"
if ! command -v $WSK_CLI &> /dev/null
then
    echo "wsk cli not found in path. Please get the cli from https://github.com/apache/openwhisk-cli/releases"
    exit
fi
ACTION="kafka-provider-feed"
PACKAGE_HOME="$PWD/${actionHome}/temp/$ACTION"

while [ $# -gt 0 ]; do
    if [[ $1 == *"--"* ]]; then
        param="${1/--/}"
        declare $param="${2%/}"
    fi

    shift
done

set -e

cd "$PWD/$actionHome"

if [ -e ./temp/${ACTION} ]; then
    echo "Clearing previously packed action file."
    rm -rf ./temp/${ACTION}
fi

mkdir -p ./temp/${ACTION}
echo "Creating temporary directory"


cp -r ./package.json ./lib ./main.js ./temp/${ACTION}
echo "Copying files to temporary directory"

cd ./temp/${ACTION}

yarn install --production=true

zip -r main.zip *

$WSK_CLI -i --apihost "$openwhiskApiHost" action update ${ACTION} "$PACKAGE_HOME/main.zip" --kind nodejs:default \
    --auth "$openwhiskApiKey" -a feed true --param endpoint "http://172.17.0.1:8888" --param web_action "kafka-provider-web" -a provide-api-key true

if [ -e ./temp/${ACTION} ]; then
    echo "Clearing temporary packed action file."
    rm -rf ./temp/${ACTION}
fi