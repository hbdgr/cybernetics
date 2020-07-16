## _(VERY EARLY STAGE) (WORK IN PROGRESS)_

# Cybernetics
Simple graph database with HASH keys and http server.

## Setting up database (OSX)
```
brew install postgres
pg_ctl -D /usr/local/var/postgres start && brew services start postgresql
```

### Create postgres user, allow for e creation
```
createuser username --createdb
```

or manually:
``` sql
psql postgres
postgres=# CREATE ROLE username WITH LOGIN PASSWORD 'password';
postgres=# ALTER ROLE username CREATEDB;
```

### .env file
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

### Diesel
[Diesel](http://diesel.rs/guides/getting-started/)
```
cargo install diesel_cli

diesel setup
```

### Tests
Tests require setting the `DATABASE_PASSWORD` environment variable
```
./script/test.sh setup
./script/test.sh run
```
