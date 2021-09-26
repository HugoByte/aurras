#register event source and link with balance filter

# To run this command
# ./deploy.sh --openwhiskApiHost <openwhiskApiHost> --openwhiskApiKey <openwhiskApiKey> --openwhiskNamespace <openwhiskNamespace>

openwhiskApiHost=${openwhiskApiHost:-https://localhost:31001}
openwhiskApiKey=${openwhiskApiKey:-23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP}
openwhiskNamespace=${openwhiskNamespace:-guest}
eventRegistrationAction=${eventRegistrationAction:-event-registration}
name=${name:-polkadot-balance}
balanceFilterAction=${balanceFilterAction:-balance-filter}
SCRIPTS_DIR="$PWD/scripts"

source "$SCRIPTS_DIR/accept_params.sh"
source "$SCRIPTS_DIR/check_dependencies.sh"

check wsk
check jq

TRIGGER=$($WSK_CLI -i --apihost "$openwhiskApiHost" action invoke "/${openwhiskNamespace}/${eventRegistrationAction}" \
--auth "$openwhiskApiKey" --param name "$name" --blocking --result | $JSON_PARSER -r '.trigger')

$WSK_CLI -i --apihost "$openwhiskApiHost" rule update "$TRIGGER-balance-filter" $TRIGGER $balanceFilterAction --auth "$openwhiskApiKey"

echo "Add TOPICS=<section>=$TRIGGER as environment variable for the substrate based event feed service, where <section> can be balances, system etc"