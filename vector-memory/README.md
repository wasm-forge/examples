# Filesystem memory consumption estimation

This is a simple filesystem example project that can be compiled to the `wasm32-wasip1` target and run in dfx.

It demonstrates how to use a simple vector of bytes as a memory.

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
  #check initial file system size
  dfx canister call vector-memory-backend fs_size

  #test executing the greet function
  dfx canister call vector-memory-backend greet '("test")'

  #check file system size after creating a few files
  dfx canister call vector-memory-backend fs_size
```

After calling the canister, you should see the message:

`("Hello from test.\nThis is a new line of text, should be there in a file.\n")`
 
