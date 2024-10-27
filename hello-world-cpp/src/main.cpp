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
