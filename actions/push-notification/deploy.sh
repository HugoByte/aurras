#!/bin/bash

# To run this command
# ./deploy.sh --openwhiskApiHost <openwhiskApiHost> --openwhiskApiKey <openwhiskApiKey> --openwhiskNamespace <openwhiskNamespace>

openwhiskApiHost=${openwhiskApiHost:-https://localhost:31001}
openwhiskApiKey=${openwhiskApiKey:-23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP}
openwhiskNamespace=${openwhiskNamespace:-guest}
actionHome=${actionHome:-actions/push-notification}

ACTION="push-notification"
ACTION_TYPE="rust"
SCRIPTS_DIR="$PWD/scripts"
SRC_DIR="$PWD/${actionHome}"
TEMP_DIR="$PWD/${actionHome}/temp"

source "$SCRIPTS_DIR/accept_params.sh"
source "$SCRIPTS_DIR/check_dependencies.sh"
source "$SCRIPTS_DIR/build_action.sh"

check wsk
check firebase_api_key

build

$WSK_CLI -i --apihost "$openwhiskApiHost" action update ${ACTION} "$TEMP_DIR/main.zip" --docker "$DOCKER_IMAGE" \
--auth "$openwhiskApiKey" --param api_key "$FIREBASE_API_KEY"

$WSK_CLI -i --apihost "$openwhiskApiHost" action update "push_notification" "$TEMP_DIR/main.zip" --docker "$DOCKER_IMAGE" \
--auth "$openwhiskApiKey" --param api_key "$FIREBASE_API_KEY"

$WSK_CLI -i --apihost "$openwhiskApiHost" trigger update "send-${ACTION}" --auth "$openwhiskApiKey"
$WSK_CLI -i --apihost "$openwhiskApiHost" rule update "${ACTION}-rule" "send-${ACTION}" ${ACTION} --auth "$openwhiskApiKey"
