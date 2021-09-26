#!/bin/bash

# To run this command
# ./deploy.sh --openwhiskApiHost <openwhiskApiHost> --openwhiskApiKey <openwhiskApiKey> --openwhiskNamespace <openwhiskNamespace>

openwhiskApiHost=${openwhiskApiHost:-https://localhost:31001}
openwhiskApiKey=${openwhiskApiKey:-23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP}
openwhiskNamespace=${openwhiskNamespace:-guest}

actions=("actions/push-notification" "actions/balance-filter" "actions/event-receiver" "actions/event-registration" "actions/balance-notification-registration" "actions/event-producer" "actions/kafka-provider-feed" "actions/kafka-provider-web" "actions/substrate-event-processor")

source ./scripts/accept_params.sh

for index in ${!actions[@]};
do  
    chmod +x "$PWD/${actions[$index]}/deploy.sh"
    "$PWD/${actions[$index]}/deploy.sh" --openwhiskApiHost "$openwhiskApiHost" --openwhiskApiKey "$openwhiskApiKey" --openwhiskNamespace "$openwhiskNamespace"
done