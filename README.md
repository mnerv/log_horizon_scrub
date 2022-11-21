# Database Technology

## psql

## Docker

Setup postgresql with docker compose file.

## SSL

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

Use command below to fix permission for docker.

```
chmod og-rwx server.key
```
