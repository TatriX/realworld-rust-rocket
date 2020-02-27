# ![RealWorld Example App](logo.png)

[![Build Status](https://travis-ci.org/TatriX/realworld-rust-rocket.svg?branch=master)](https://travis-ci.org/TatriX/realworld-rust-rocket)
[![Crates.io](https://img.shields.io/crates/v/realworld.svg)](https://crates.io/crates/realworld)
[![codecov](https://codecov.io/gh/TatriX/realworld-rust-rocket/branch/master/graph/badge.svg)](https://codecov.io/gh/TatriX/realworld-rust-rocket)

### RealWorld

This codebase was created to demonstrate a fully fledged fullstack
application built with [Rocket](http://rocket.rs/) including CRUD operations,
authentication, routing, pagination, and more.

We've gone to great lengths to adhere to the [Rocket](http://rocket.rs/) community
styleguides & best practices.

For more information on how to this works with other
frontends/backends, head over to the [RealWorld](https://github.com/gothinkster/realworld) repo.

### CHANGELOG

Please see the [CHANGELOG](CHANGELOG.md) for a release history.

### Getting started

Install [nightly](https://www.rust-lang.org/en-US/install.html)
```sh
# install rustup
curl https://sh.rustup.rs -sSf | sh

rustup install nightly

# start postgresql and seed the database
psql -f init.sql
# or with database connecting infomation:
# psql \
#   -U <database username has permission to create databases and database users> \
#   -h <database host address> \
#   -p <database port> \
#   -f init.sql
psql -U postgres -h localhost -p 5432 -f init.sql

cargo install diesel_cli --no-default-features --features "postgres"

diesel migration run
# or with database connecting url(these information are defined in init.sql):
# diesel migration run --database-url postgresql://<username>:<password>@<host>:<port>/<db name>
diesel migration run --database-url postgresql://realworld:realworld@localhost:5432/realworld

# run build and server with same database url:
DATABASE_URL=postgresql://realworld:realworld@localhost:5432/realworld cargo run
```

### Testing
Simply run:
```sh
cargo test
```
You can also check postman/newman. See `/tests` directory.

### How it works
`diesel` cli uses `.env` file.
Rocket sets database configuration from `.env` file.
Checkout Rocket's amazing [guide](https://rocket.rs/guide/)

### Features
By default random suffixes feature is enabled, so one could easily
create multiple articles with the same title. To disable it:

```sh
cargo run --no-default-features

```

### TODO
1. Bettter error handling
