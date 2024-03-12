# Storage Module

Storing the entire runtime state refers to saving all the data and context associated with a program's execution at runtime.

This will provide the operations like set, get, delete, modify for the other modules. Where Storage interface lets other modules to interact with the Db.

The `CoreStorage` module implements the `Storage` trait to interact with a `RocksDB` database.

## Features

- CRUD operations (Create, Read, Update, Delete)
- Data persistence mechanisms (e.g., file system, database)
- Caching capabilities

## License

Licensed under [Apache-2.0](https://github.com/HugoByte/aurras-documentation/tree/f07f6727f0cb01cccf04f15ec446e2d310ca1cb9/components/event-feed/substrate-event-feed/LICENSE/README.md)
