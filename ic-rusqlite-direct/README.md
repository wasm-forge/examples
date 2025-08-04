# Run Rusqlite on the IC

This project shows how to compile the Rusqlite dependency in order to build the IC canister with the sqlite database.


## Prerequisites

It is assumed that you have [rust](https://doc.rust-lang.org/book/ch01-01-installation.html), [dfx](https://internetcomputer.org/docs/current/developer-docs/setup/install/).

You also need the Wasm-oriented [clang](https://github.com/WebAssembly/wasi-sdk/releases/) installation. 
Once installed the `clang` compiler should be available from the path `/opt/wasi-sdk/bin/`. 

Finally, install the `wasi2ic` tool:
```bash
cargo install wasi2ic
```


## Deployment and testing

Start the `dfx` environment in a separate console:
```bash
  dfx start --clean --background
```

To build and deploy the canister, run the command:
```bash
  dfx deploy
```

You can now do the canister test calls, the test script will create some persons in the database and list the selected persons via SQL query:
```bash
  ./scripts/test.sh
```



