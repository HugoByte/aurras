#!/bin/bash

# To run this command
# ./deploy.sh --openwhiskApiHost <openwhiskApiHost> --openwhiskApiKey <openwhiskApiKey> --openwhiskNamespace <openwhiskNamespace>

openwhiskApiHost=${openwhiskApiHost:-https://localhost:31001}
openwhiskApiKey=${openwhiskApiKey:-23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP}
openwhiskNamespace=${openwhiskNamespace:-guest}
build=${build:-push-notification,balance-filter,event-receiver,event-registration,balance-notification-registration,event-producer,kafka-provider-feed,kafka-provider-web,substrate-event-processor,user_registration,user_login,workflow-invoker,workflow_registration,workflow_management}
skip=${skip:-}

actions=("actions/push-notification" "actions/balance-filter" "actions/event-receiver" "actions/event-registration" "actions/balance-notification-registration" "actions/event-producer" "actions/kafka-provider-feed" "actions/kafka-provider-web" "actions/substrate-event-processor" "actions/user_registration" "actions/user_login" "actions/workflow-invoker" "actions/workflow_registration" "actions/workflow_management")

source ./scripts/accept_params.sh
source ./scripts/util.sh

build_array=(${build//,/ })
skip_array=(${skip//,/ })

for index in ${!actions[@]};
do  
    array_contains "${actions[$index]//actions\//}" "${skip_array[@]}" && true || {
        array_contains "${actions[$index]//actions\//}" "${build_array[@]}" && should_build=true || should_build=false
        chmod +x "$PWD/${actions[$index]}/deploy.sh"
        "$PWD/${actions[$index]}/deploy.sh" --openwhiskApiHost "$openwhiskApiHost" --openwhiskApiKey "$openwhiskApiKey" --openwhiskNamespace "$openwhiskNamespace" --build $should_build
    }
done
