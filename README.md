# Setting up database

## PostgreSQL

### Install
```
brew install postgres
pg_ctl -D /usr/local/var/postgres start && brew services start postgresql
```

* FATAL:  database files are incompatible with server
rm -rf /usr/local/var/postgres && initdb /usr/local/var/postgres -E utf8

### Create postgres user, allow for e creation

```
createuser username --createdb
```

or manually:
``` sql
psql postgres
postgres=# CREATE ROLE username WITH LOGIN PASSWORD 'password' [OPTIONS]
postgres=# ALTER ROLE patrick CREATEDB;
```

## .env file

Required variables:
- ROCKET_ADDRESS
- ROCKET_PORT
- DATABASE_URL

Example:
```
ROCKET_ADDRESS=localhost
ROCKET_PORT=8080
DATABASE_URL=postgres://username:password@localhost/cybernetics
```

## Diesel

[Diesel](http://diesel.rs/guides/getting-started/)
```
cargo install diesel_cli

diesel setup
```
