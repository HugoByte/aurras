# Workflow Builder

### Usage
- Pull the docker file
    ``` 
    docker pull kkshanith/workflow
    ```
- Create the Config YAML and run the command
    ```
    docker run -it --rm -v $(pwd)/<Yaml_file_path>:/input.yaml -v $PWD:/host kkshanith/workflow
    ```
- The workflow wasm will be generated in the current working directory(ie in the $(pwd)).


