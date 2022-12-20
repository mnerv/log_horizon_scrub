# Hope store

We sell hopes and dreams.

## Requirements

  - [Rust](https://www.rust-lang.org/)

## Development

Create `.env` file in the project root directory and use the template below to fill in the required secrets

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

## Queries

Use query command below to list schemas in `psql`.

```sql
SELECT schema_name FROM information_schema.schemata;
```

