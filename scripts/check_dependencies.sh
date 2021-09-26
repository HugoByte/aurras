WSK_CLI="wsk"
JSON_PARSER="jq"

function check {
    DEPENDENCY="$1"
    if [ $DEPENDENCY = "wsk" ]
    then
        if ! command -v $WSK_CLI &> /dev/null
        then
            echo "wsk cli not found in path. Please get the cli from https://github.com/apache/openwhisk-cli/releases"
            exit
        fi
    elif [ $DEPENDENCY = "firebase_api_key" ]
    then
        if [ -z "$FIREBASE_API_KEY" ]
        then
            echo "FIREBASE_API_KEY env is not defined, Generate server token from https://console.firebase.google.com/project/<project-name>/settings/cloudmessaging and add as env FIREBASE_API_KEY"
            exit
        fi
    elif [ $DEPENDENCY = "jq"]
    then
        if ! command -v $JSON_PARSER &> /dev/null
        then
            echo "jq found in path. jq is needed to parse the json response kindly install it"
            exit
        fi
    else
        echo "dependency $DEPENDENCY not supported"
        exit
    fi
}