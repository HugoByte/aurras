if [ "$build" = true ]; then
    if [ -e $TEMP_DIR ]; then
        echo "Clearing previously packed action files."
        rm -rf $TEMP_DIR
    fi
    
    mkdir -p $TEMP_DIR
    echo "Creating temporary directory"
    
    echo "Copying files to temporary directory"
    cp -r "$SRC_DIR/package.json" "$SRC_DIR/lib" "$SRC_DIR/main.js" $TEMP_DIR
    
    cd $TEMP_DIR
    
    yarn install --production=true
    
    zip -r "$TEMP_DIR/main.zip" *
fi
