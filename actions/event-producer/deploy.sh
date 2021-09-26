#!/bin/bash

# To run this command
# ./deploy.sh --openwhiskApiHost <openwhiskApiHost> --openwhiskApiKey <openwhiskApiKey> --openwhiskNamespace <openwhiskNamespace>

openwhiskApiHost=${openwhiskApiHost:-https://localhost:31001}
openwhiskApiKey=${openwhiskApiKey:-23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP}
openwhiskNamespace=${openwhiskNamespace:-guest}
actionHome=${actionHome:-actions/event-producer}

ACTION="event-producer"
SCRIPTS_DIR="$PWD/scripts"
SRC_DIR="$PWD/${actionHome}"

source "$SCRIPTS_DIR/accept_params.sh"
source "$SCRIPTS_DIR/check_dependencies.sh"

$WSK_CLI -i --apihost "$openwhiskApiHost" action update ${ACTION} "$SRC_DIR/event-producer.py" \
--auth "$openwhiskApiKey"