#!/bin/bash

#check initial file system size
dfx canister call vector-memory-backend fs_size

#test executing the greet function
dfx canister call vector-memory-backend write_file '("test.txt", "some text")'

#check file system size after creating a few files
dfx canister call vector-memory-backend fs_size
