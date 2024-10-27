# The "Hello World" canister by using the C++ to WASI compiler

This "hello world" demo project shows how to write a Wasm canister written in C++ and deploy it using `dfx`. 

## Prerequisites

It is assumed that you have [rust](https://doc.rust-lang.org/book/ch01-01-installation.html), [dfx](https://internetcomputer.org/docs/current/developer-docs/setup/install/), 

You will also need the Wasm-oriented [clang](https://github.com/WebAssembly/wasi-sdk/releases/) installation. In this tutorial we use the `.deb` package [installation](https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-23/wasi-sdk-23.0-x86_64-linux.deb). Once installed the clang compiler is available from the path `/opt/wasi-sdk/bin/`.



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

You can now check that the canister works by calling the greet method:
```bash
dfx canister call demo2_backend greet --output raw --type raw `echo "world" | xxd -p` | xxd -p -r
```

If there are no errors, you should be able to see the text message in the dfx console window. 
To make the example less trivial, the return value is read from the disk.

