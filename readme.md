# ![RealWorld Example App](logo.png)

> ###  Rust / Rocket codebase containing real world examples (CRUD, auth, advanced patterns, etc) that adheres to the [RealWorld](https://github.com/gothinkster/realworld) spec and API.

### [Demo](http://demo.realworld.io/)&nbsp;&nbsp;&nbsp;&nbsp;[RealWorld](https://github.com/gothinkster/realworld)

This codebase was created to demonstrate a fully fledged fullstack application built with [Rocket](http://rocket.rs/) including CRUD operations, authentication, routing, pagination, and more.

We've gone to great lengths to adhere to the [Rocket](http://rocket.rs/) community styleguides & best practices.

For more information on how to this works with other frontends/backends, head over to the [RealWorld](https://github.com/gothinkster/realworld) repo.

# Getting started

Install [nightly](https://www.rust-lang.org/en-US/install.html)
```sh
# install rustup
curl https://sh.rustup.rs -sSf | sh

rustup install nightly
rustup default nightly

cargo run
# from another term or use postman
cargo test -- --test-threads=1
```

# How it works

Checkout Rocket's amazing [guide](https://rocket.rs/guide/)



### Features
To use random suffixes for articles, compile with `random_suffix` feature.
```sh
cargo run --feature random_suffix

```
