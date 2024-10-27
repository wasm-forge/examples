# Basic "Hello World" example project

This is a simple "hello world" example project that can be compiled to the `wasm32-wasip1` target and run in dfx.

It is assumed that you have installed [rust](https://doc.rust-lang.org/book/ch01-01-installation.html) and [dfx](https://internetcomputer.org/docs/current/developer-docs/setup/install/).


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

You can now do the canister call:
```bash
  dfx canister call hello-world-backend greet '("test")'
```

Adter calling the canister, you should see the both messages:
 *"Hello from IC debugger: test"* and *"Hello from WASI: test"* in the dfx console.

