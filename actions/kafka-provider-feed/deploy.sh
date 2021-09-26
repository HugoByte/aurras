#!/bin/bash

# To run this command
# ./deploy.sh --openwhiskApiHost <openwhiskApiHost> --openwhiskApiKey <openwhiskApiKey> --openwhiskNamespace <openwhiskNamespace>

openwhiskApiHost=${openwhiskApiHost:-https://localhost:31001}
openwhiskApiKey=${openwhiskApiKey:-23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP}
openwhiskNamespace=${openwhiskNamespace:-guest}
actionHome=${actionHome:-actions/kafka-provider-feed}

ACTION="kafka-provider-feed"
ACTION_TYPE="js"
SCRIPTS_DIR="$PWD/scripts"
SRC_DIR="$PWD/${actionHome}"
TEMP_DIR="$PWD/${actionHome}/temp"

source "$SCRIPTS_DIR/accept_params.sh"
source "$SCRIPTS_DIR/check_dependencies.sh"
source "$SCRIPTS_DIR/build_action.sh"

check wsk

build

$WSK_CLI -i --apihost "$openwhiskApiHost" action update ${ACTION} "$TEMP_DIR/main.zip" --kind nodejs:default \
--auth "$openwhiskApiKey" -a feed true --param endpoint "http://172.17.0.1:8888" --param web_action "kafka-provider-web" -a provide-api-key true

clear_temp