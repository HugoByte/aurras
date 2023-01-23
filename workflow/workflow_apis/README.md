# Web API for Workflow
This package provide API for creating action, trigger and rule also deleting these.


## Setup

### Prerequisites

- Latest verion of rust to be installed and configured.
- Diesel ORM should be installed
- Postgres database

## Configuration

The server host address and port should be mentioned in .env file, also the database url , jwt token key and hashin secret key should be specified. set rust log also in .env file

Example for configuration.

```
HOST=127.0.0.1
PORT=8080
DATABASE_URL=postgres://username:password@localhost/database
SECRET_KEY = hashing-key
JWT_SECRET=login-details
RUST_LOG="debug,actix_web=debug,sqlx=info"
```

## Working
- Clone this repository
- Run the postgres database and create a datbase.
- Add the database url username, password and database in the .env file
- After that go too inside this project directory and run
    
    ``` 
    diesel migration run 
    ```
- Then run the project 
    ```
    cargo run
    ```

## Usage

### Web APIs 

This project will expose APIs for openwhisk interaction for creating and deleting actio, trigger and rules. all APIs are post methods.

If you need to interact with the openwhisk through this APIs, before that you need to register. after register , with your credentials you can login and get the authentication token, using authentication token you can access these APIs.

### Examples


- `Deploy an Action`

    Endpoint : ``` {Host}:{Port}/action ```

    Parameters you needed for

    ```
    {
        name: "String",
        kind: "String",
        file: File,
        url: "String",
        namespace: "String",
        auth: "String",
    }
    ```
    if you are using postman change the content-type to "application/json".

- ` Create a Trigger and rule`

    Endpoint : ``` {Host}:{Port}/trigger ```

    ```
    {
        name: "String",
        param_json: "String",
        url: "String",
        namespace: "String",
        auth: "String",
        rule: "String",
        action: "String",
    }
    ```
    param_json is for mentioning the parameter for the action need to run.

- ` Delete `

    Endpoint : ``` {Host}:{Port}/delete ```

    ```
    {
        name: "String",
        url: "String",
        namespace: "String",
        auth: "String",
        deleting_type: "String",
    }
    ```
    delete_type shuld be `action`, `trigger` , or `rule`.

- ` List `

    Endpoint : ``` {Host}:{Port}/get_list ```

    ```
    {
        url: "String",
        namespace: "String",
        auth: "String",
        list_type: "String",
    }
    ```
    list_type shuld be `actions`, `triggers` , or `rules`.

## Contributions

Contributions welcome particularly for enhancement of this library and also adding new functionality which helps in seamless interaction with Openwhisk Apis in rust applications.

### Follow these steps for contributing

- Fork the repository
- Clone your fork
- Create a new branch
- Make changes in local repo
- commit and push your changes to forked repo
- Begin and create a pull request
- Once pull request is review and accepted code changes will be merged to main branch
- You will get a notification email once the changes have been merged

## References

- [Actix](https://actix.rs/docs/)
- [Diesel](https://diesel.rs/guides/getting-started)
- [PostgreSQL](https://postgresql.org)


## License

Licensed under [Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0)