# Database Technology

This repository holds course assignments and notes for database technology course.

## Resources

  - [Docker](https://www.docker.com/) - OS-level virtualization software.
  - [PostgreSQL](https://www.postgresql.org/) - SQL compliance database.
  - [Rust](https://doc.rust-lang.org/std/index.html) - Project application language

## Docker

Setup postgresql with docker compose file.

```
docker compose up
```

## PostgresSQL with SSL

Generate certificate authority(CA), key and certificate, and the server key and csr(certificate signing request) and then sign the request.

```
--CA
openssl genrsa 2048 > ca.key
openssl req -new -x509 -nodes -days 365 -key ca.key -out ca.cert
--server
openssl req -newkey rsa:2048 -nodes -days 365 -keyout server.key -out server.csr
openssl x509 -req -days 365 -set_serial 01 -in server.csr -out server.cert -CA ca.cert -CAkey ca.key
```

Files that should been generated.

```
ca.cert
ca.key
server.cert
server.csr
server.key
```

Uncomment these settings in the docker-compose.yml file to enable ssl. The volumes mapping doesn't work on windows.

```
#     -c ssl=on
#     -c ssl_ca_file='/var/lib/postgresql/ssl/ca.cert'
#     -c ssl_cert_file=/var/lib/postgresql/ssl/server.cert
#     -c ssl_key_file=/var/lib/postgresql/ssl/server.key
# ...
#     - ./data/ssl/ca.cert:/var/lib/postgresql/ssl/ca.cert
#     - ./data/ssl/server.cert:/var/lib/postgresql/ssl/server.cert
#     - ./data/ssl/server.key:/var/lib/postgresql/ssl/server.key
```

Use command below to fix permission for docker.

```
chmod og-rwx server.key
```
