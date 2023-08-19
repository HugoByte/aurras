#!/bin/bash

# To run this command
# ./deploy.sh --openwhiskApiHost <openwhiskApiHost> --openwhiskApiKey <openwhiskApiKey> --openwhiskNamespace <openwhiskNamespace>

openwhiskApiHost=${openwhiskApiHost:-https://localhost:31001}
openwhiskApiKey=${openwhiskApiKey:-23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP}
openwhiskNamespace=${openwhiskNamespace:-guest}
actionHome=${actionHome:-actions/workflow-invoker}

ACTION="workflow-invoker"
ACTION_TYPE="rust"
SCRIPTS_DIR="$PWD/scripts"
SRC_DIR="$PWD/${actionHome}"
TEMP_DIR="$PWD/${actionHome}/temp"

source "$SCRIPTS_DIR/accept_params.sh"
source "$SCRIPTS_DIR/check_dependencies.sh"
source "$SCRIPTS_DIR/build_action.sh"

check wsk

build

$WSK_CLI -i --apihost "$openwhiskApiHost" action update ${ACTION} "$TEMP_DIR/main.zip" --docker "$DOCKER_IMAGE" \
--auth "$openwhiskApiKey" --param polkadot_payout_trigger "send-payout-notify" --param db_url "http://admin:p@ssw0rd@172.17.0.1:5984" --param db_name "workflow_management_db" -a provide-api-key true

$WSK_CLI -i --apihost "$openwhiskApiHost" trigger update "send-payout-notify" --auth "$openwhiskApiKey"
# $WSK_CLI -i --apihost "$openwhiskApiHost" rule update "payout_notify-rule" "send-payout-notify" "payout_notify" --auth "$openwhiskApiKey" // after deploying the action for polkadot payout
