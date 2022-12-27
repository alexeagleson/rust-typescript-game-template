# Game Template

A complete template for building a 2D grid based game with a Rust backend and Typescript frontend.

Serialization (serde)
Shared types (typeshare)
Data persistence (sqlx)
Web server (warp)

Frontend 

Tooling & bundling (Vite)
UI (React)
Canvas rendering (Pixi.js)


## Usage

Local development:

A couple of Rust CLI tools are required to build the project:

For sharing Rust types with client:

```
cargo install typeshare-cli
```

For initializing the SQLite database:

```
cargo install sqlx-cli
```

Before running for the first time you must run these commands to build the initial types file and initialize the database:

```
sqlx database create

sqlx migrate run

typeshare ./ --lang=typescript --output-file=client/src/utility/types.ts
```


# License

AE Position is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).