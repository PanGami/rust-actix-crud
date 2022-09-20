### Development Branch

### Short Description
This is a CRUD Project with Actix to learn API with Rust better. In this project we build a simple api that create, read, update and delete with given JSON data, id at endpoint/path

### Available Routes

- [GET - Show all tweets](http://localhost:8080/api/tweets) // localhost:{port}/api/tweets
- [POST - Create new tweets](http://localhost:8080/api/tweets) // localhost:{port}/api/tweets
- [GET - Show by id](http://localhost:8080/api/tweets/1) // localhost:{port}/api/tweets/{id}
- [PUT - Edit tweets](http://localhost:8080/api/tweets/1) // localhost:{port}/api/tweets/{id}
- [DELETE - Delete tweets](http://localhost:8080/api/tweets/1) // localhost:{port}/api/tweets/{id}

### Crates Used

- [actix-web](https://crates.io/crates/actix-web) // Web framework for Rust.
- [chrono](https://crates.io/crates/chrono) // Date and time library for Rust.
- [diesel](https://crates.io/crates/diesel) // ORM and Query Builder
- [dotenv](https://crates.io/crates/dotenv) // Load environment variables from .env file
- [env_logger](https://crates.io/crates/env_logger) // Implements a logger that con be configured via env.
- [r2d2](https://crates.io/crates/r2d2) // A generic connection pool.
- [serde](https://crates.io/crates/serde) // A generic serialization/deserialization framework.
- [serde_json](https://crates.io/crates/serde_json) // A JSON serialization file format - to read a raw JSON. 