# demo2 - Create the "Hello World" canister by using C++ to WASI compiler

This "hello world" demo project shows how to compile a Wasm canister written in C++ and deploy it using `dfx`. 

## Prerequisites

It is assumed that you have [rust](https://doc.rust-lang.org/book/ch01-01-installation.html), [dfx](https://internetcomputer.org/docs/current/developer-docs/setup/install/), and [wasi2ic](https://github.com/wasm-forge/wasi2ic) installed.

You will also need the Wasm-oriented [clang](https://github.com/WebAssembly/wasi-sdk/releases/) installation. In this tutorial we use the `.deb` package [installation](https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-23/wasi-sdk-23.0-x86_64-linux.deb). Once installed the clang compiler is available from the path `/opt/wasi-sdk/bin/`.

Make sure you have the `ic-wasi-polyfill` library source available in the neighbouring folder, you can download it from GitHub:
```bash
git clone https://github.com/wasm-forge/ic-wasi-polyfill.git
```

Enter the folder and compile it with the wasi32-wasm target:
```bash
cd ic-wasi-polyfill
cargo build --release --target wasm32-wasi
```

It should create the static library `libic_wasi_polyfill.a` under the 'target/wasm32-wasi/release' folder.

Now return to the parent folder:

```bash
cd ..
```

You can now either create this project from scratch or clone this repository and skip to the "Deployment and testing" section.

## Building project from scratch

Creare a new project using `dfx`:

```bash
dfx new --type=rust --no-frontend demo2
```

Enter the src folder of the project:
```bash
cd demo2/src
```

Create the `main.cpp` file with the following content:
```cpp
#include <iostream>
#include <string>
#include <fstream>

#define __IMPORT(module, name) __attribute__((__import_module__(#module), __import_name__(#name)))
#define __EXPORT(name) __attribute__((__export_name__(#name)))

// Initialize the WASI polyfill library first.
extern "C" void raw_init(char* p, std::size_t len) __IMPORT(polyfill, raw_init);
class WasiPolyfill{
  public:
    WasiPolyfill(){
        raw_init(nullptr, 0);
    }
} __wasi_polyfill;

extern "C" void ic0_debug_print(const char *str, std::size_t len) __IMPORT(ic0, debug_print);
extern "C" int  ic0_msg_arg_data_size() __IMPORT(ic0, msg_arg_data_size);
extern "C" void ic0_msg_arg_data_copy(char * buf, std::size_t offset, std::size_t length) __IMPORT(ic0, msg_arg_data_copy);
extern "C" void ic0_msg_reply() __IMPORT(ic0, msg_reply);
extern "C" void ic0_msg_reply_data_append(const char * buf, std::size_t length) __IMPORT(ic0, msg_reply_data_append);

// some static variable
const std::vector<std::string> s_msg ({"Hello, ", "world"});

extern "C" __EXPORT(canister_query greet) __attribute__((noinline)) void greet()  {

    int n = ic0_msg_arg_data_size();
    char buf[n];

    ic0_msg_arg_data_copy(buf, 0, n);
    
    // work with text
    std::string s(buf);
    
    std::string content = s_msg[0] + s;

    // do some file operations
    std::ofstream ofile("content.txt");
    ofile << "File content: " << content;
    ofile.close();

    std::string line;
    std::ifstream ifile("content.txt");
    getline (ifile,line);
    ifile.close();

    ic0_msg_reply_data_append(line.c_str(), line.length());
    ic0_msg_reply();
}


```


## Deployment and testing

In a separate terminal start the `dfx` environment:
```bash
dfx start
```

Go to the `demo2` project folder and create the canister:
```bash
dfx canister create demo2_backend
```
This will create and deploy the default "Hello world" canister named `demo2_backend`.

Enter the `src` folder of the `demo2` project, you should now be able to compile the `main.cpp` and and link the `ic_wasi_polyfill` library with it:

```bash
cd src
/opt/wasi-sdk/bin/clang++ -mexec-model=reactor -fno-exceptions main.cpp -L../../ic-wasi-polyfill/target/wasm32-wasi/release -lic_wasi_polyfill -o main.wasm
```

Clean out WASI dependencies with the `wasi2ic` tool:

```bash
wasi2ic main.wasm no_wasi.wasm
```

The `no_wasi.wasm` file should be clear from any WASI dependencies, you can check that with the `wasm2wat` tool that converts a `.wasm` file to its textual `.wat` representation. If everything works out, you will see there are no WASI imports left and there are only some `ic0` imports the `.wat` file.


Once you have the `no_wasi.wasm`, you can deploy it using the `dfx` command:
```bash
dfx canister install --mode reinstall --wasm no_wasi.wasm demo2_backend
```

You can now check that the canister works by calling the greet method:
```bash
dfx canister call demo2_backend greet --output raw --type raw `echo "world" | xxd -p` | xxd -p -r
```

If there are no errors, you should be able to see the "Hello from WASI" text in the dfx console window.
