# Filesystem inside a vector memory with memory consumption estimation

This is a simple filesystem example project. It demonstrates how to use a vector of bytes as a memory for the file system and reports its current size after doing file reads and writes.

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

You can now do the canister calls:
```bash
#report current file system size
dfx canister call vector-memory-backend fs_size

#write some content to a new file
dfx canister call vector-memory-backend write_file '("test1.txt", "some text")'

#report current file system size
dfx canister call vector-memory-backend fs_size

#write some content to another file
dfx canister call vector-memory-backend write_file '("a/b/c/test2.txt", "some text 2")'

#report contents of the first file created
dfx canister call vector-memory-backend read_file '("test1.txt")'

#report current file system size
dfx canister call vector-memory-backend fs_size

```

After calling the canister, you should see the message:

`("Hello from test.\nThis is a new line of text, should be there in a file.\n")`
 
