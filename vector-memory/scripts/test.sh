#!/bin/bash

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

#write some content to another file
dfx canister call vector-memory-backend write_file '("test3.txt", "some text 3")'
#write some content to another file
dfx canister call vector-memory-backend write_file '("test4.txt", "some text 4")'
#write some content to another file
dfx canister call vector-memory-backend write_file '("test5.txt", "some text 5")'
#write some content to another file
dfx canister call vector-memory-backend write_file '("test6.txt", "some text 6")'
#write some content to another file
dfx canister call vector-memory-backend write_file '("test7.txt", "some text 7")'
#write some content to another file
dfx canister call vector-memory-backend write_file '("test8.txt", "some text 8")'
#write some content to another file
dfx canister call vector-memory-backend write_file '("test9.txt", "some text 9")'
#write some content to another file
dfx canister call vector-memory-backend write_file '("test10.txt", "some text 10")'

#report current file system size
dfx canister call vector-memory-backend fs_size
