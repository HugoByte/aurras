# Workflow Builder

### Usage
- Build the docker image
    ``` 
    docker build -t <image_name> .
    ```
- Create the Config YAML and run the command
    ```
    docker run -it --rm -v $(pwd)/<Yaml_file_path>:/input.yaml -v $PWD:/host <image_name> 
    ```
- The workflow wasm will be generated in the current working directory(ie in the $(pwd)).


