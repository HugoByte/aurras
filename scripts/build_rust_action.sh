DOCKER_IMAGE="hugobyte/openwhisk-runtime-rust:v0.2"

if [ -e $TEMP_DIR ]; then
    echo "Clearing previously packed action files."
    rm -rf $TEMP_DIR
fi

mkdir -p $TEMP_DIR
echo "Creating temporary directory"

echo "Building Source to $TEMP_DIR/main.zip"

cd $SRC_DIR

zip -r - Cargo.toml src | docker run -e RELEASE=true -i --rm $DOCKER_IMAGE -compile main > "$TEMP_DIR/main.zip"

cd $TEMP_DIR
