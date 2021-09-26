function build {
    if [ $ACTION_TYPE = "rust" ]
    then
        source "$SCRIPTS_DIR/build_rust_action.sh"
    elif [ $ACTION_TYPE = "js" ]
    then
        source "$SCRIPTS_DIR/build_js_action.sh"
    else
        echo "Action type not supported"
        exit
    fi
}

function clear_temp {
    if [ -e $TEMP_DIR ]; then
        echo "Clearing temporary packed action file."
        rm -rf $TEMP_DIR
    fi
}