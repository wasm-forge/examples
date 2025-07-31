#!/bin/bash

# add some persons to the database
dfx canister call ic-rusqlite-direct-backend add '("Amy","test1", 25: nat32)'

dfx canister call ic-rusqlite-direct-backend add '("John","test2", 34: nat32)'

dfx canister call ic-rusqlite-direct-backend add '("Mark","test3", 19: nat32)'

# list persons
dfx canister call ic-rusqlite-direct-backend list

# use the query method
dfx canister call ic-rusqlite-direct-backend query '("select * from person")'
