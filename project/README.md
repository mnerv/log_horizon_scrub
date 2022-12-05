# Hope store

We sell hopes and dreams.

## Requirements

  - [Rust](https://www.rust-lang.org/)

## Development

Create `.env` file and use the template below to fill in the required secrets

```.env
PG_HOST=host               # Defaults to localhost
PG_USER=user               # Defaults to postgres
PG_PASSWORD=password       # Defaults to postgres
PG_DB=database_name        # Defaults to postgres
PG_SCHEMA=database_schema  # Defaults to public
```

Use `cargo` to build and run the application.

```
cargo run
```

