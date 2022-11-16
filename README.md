# Database Technology

## PostgreSQL Docker

### Usage

Run `psql`

```
docker exec -it {container_name} psql -U {user} {dbname}
```

Run sql files with docker.

```
docker cp ./localfile.sql containername:/container/path/file.sql
docker exec -u postgresuser containername psql dbname postgresuser -f /container/path/file.sql
```

### Configurations

#### Environment Variables

```
POSTGRES_PORT=5432
POSTGRES_DATABASE=database
POSTGRES_USER=user
POSTGRES_PASSWORD=password
PGADMIN_PORT=80
PDADMIN_EMAIL=user@domain.dev
PGADMIN_PASSWORD=password
```

#### Persistance data in pgAdmin

Uncomment

```yml
    # volumes:
    #   - ./data/pgadmin:/var/lib/pgadmin
```
Fix permission

```
mkdir data && cd data
mkdir pgadmin
```

Fix the permission

```
sudo chown -R 5050:5050 pgadmin
```

#### Use with SSL

Uncomment

```yml
#   command: >
#     -c ssl=on
#     -c ssl_cert_file=/var/lib/postgresql/server.crt
#     -c ssl_key_file=/var/lib/postgresql/server.key
```

and

```yml
#     - ./data/ssl/server.crt:/var/lib/postgresql/server.crt
#     - ./data/ssl/server.key:/var/lib/postgresql/server.key
```

Create `data/ssl` directory.

```
mkdir data && cd data && mkdir ssl && cd ssl
```

Generate `Certificate Signing Request (CSR) file`

```
openssl req -new -text -out server.req
```

Removed passphrase

```
openssl rsa -in privkey.pem -out server.key
rm privkey.pem
```

Turn into self-signed certificate

```
openssl req -x509 -in server.req -text -key server.key -out server.crt
```

Fix permission

```
chmod og-rwx server.key
```
