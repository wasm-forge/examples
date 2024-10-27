#!/bin/bash

#test executing the greet function
dfx canister call hello-world-cpp-backend greet --output raw --type raw `echo "world" | xxd -p` | xxd -p -r