# Run Limbo on the IC

This project shows how to compile and run the Limbo database (a compatible Sqlite alternative written in Rust) in the IC canister.


## Prerequisites

It is assumed that you have [rust](https://doc.rust-lang.org/book/ch01-01-installation.html), [dfx](https://internetcomputer.org/docs/current/developer-docs/setup/install/).


## Preparation

Install wasi2ic:
```bash
  cargo install wasi2ic
```

## Deployment and testing

Start the `dfx` environment in a separate console:
```bash
  dfx start --clean
```

To build and deploy the canister, run the command:
```bash
  dfx deploy
```

You can now do the canister test calls, the test script will create some persons in the database and list the selected persons via SQL query:
```bash
  ./scripts/test.sh
```



