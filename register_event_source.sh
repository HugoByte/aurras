#register event source and link with balance filter

# To run this command
# ./deploy.sh --openwhiskApiHost <openwhiskApiHost> --openwhiskApiKey <openwhiskApiKey> --openwhiskNamespace <openwhiskNamespace>

openwhiskApiHost=${openwhiskApiHost:-https://localhost:31001}
openwhiskApiKey=${openwhiskApiKey:-23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP}
openwhiskNamespace=${openwhiskNamespace:-guest}
eventRegistrationAction=${eventRegistrationAction:-event-registration}
name=${name:-polkadot}
balanceFilterAction=${balanceFilterAction:-balance-filter}
WSK_CLI="wsk"
JSON_PARSER="jq"

if ! command -v $WSK_CLI &> /dev/null
then
    echo "wsk cli not found in path. Please get the cli from https://github.com/apache/openwhisk-cli/releases"
    exit
fi

if ! command -v $JSON_PARSER &> /dev/null
then
    echo "jq found in path. jq is needed to parse the json response kindly install it"
    exit
fi

while [ $# -gt 0 ]; do
    if [[ $1 == *"--"* ]]; then
        param="${1/--/}"
        declare $param="${2%/}"
    fi

    shift
done


TRIGGER=$($WSK_CLI -i --apihost "$openwhiskApiHost" action invoke "/${openwhiskNamespace}/${eventRegistrationAction}" \
    --auth "$openwhiskApiKey" --param name $name --blocking --result | $JSON_PARSER -r '.trigger')

$WSK_CLI -i --apihost "$openwhiskApiHost" rule update "$TRIGGER-balance-filter" $TRIGGER $balanceFilterAction --auth "$openwhiskApiKey"

echo "Add KAFKA_TOPIC=$TRIGGER as environment variable for the substrate based event feed service"