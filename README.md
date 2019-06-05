# ![RealWorld Example App](logo.png)

[![Build Status](https://travis-ci.org/TatriX/realworld-rust-rocket.svg?branch=master)](https://travis-ci.org/TatriX/realworld-rust-rocket)

> ###  Rust / Rocket codebase containing real world examples (CRUD, auth, advanced patterns, etc) that adheres to the [RealWorld](https://github.com/gothinkster/realworld) spec and API.

### [RealWorld](https://github.com/gothinkster/realworld)

This codebase was created to demonstrate a fully fledged fullstack application built with [Rocket](http://rocket.rs/) including CRUD operations, authentication, routing, pagination, and more.

We've gone to great lengths to adhere to the [Rocket](http://rocket.rs/) community styleguides & best practices.

For more information on how to this works with other frontends/backends, head over to the [RealWorld](https://github.com/gothinkster/realworld) repo.

# Getting started

Install [nightly](https://www.rust-lang.org/en-US/install.html)
```sh
# install rustup
curl https://sh.rustup.rs -sSf | sh

rustup install nightly

# start postgresql and seed the database
psql -f init.sql
cargo install diesel_cli --no-default-features --features "postgres"
diesel migration run

cargo run
```

## Testing
Simply run:
```sh
cargo test
```
You can also check postman/newman. See `/tests` directory.

# How it works
`diesel` cli uses `.env` file.
Rocket reads database configuration from `Rocket.toml` file.
Checkout Rocket's amazing [guide](https://rocket.rs/guide/)

### Features
By default random suffixes feature is enabled, so one could easily create multiple articles with the same title. To disable it:
```sh
cargo run --no-default-features

```

### TODO
1. Use insert into table (...) select
2. Error handling: either `snafu`, `failure` or `error_chain`
3. Consider using rockets custom config and reading db url from `.env`
