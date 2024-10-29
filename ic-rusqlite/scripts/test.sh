#!/bin/bash

# add some persons to the database
dfx canister call ic-rusqlite-backend add '("Amy","test1", 25: nat32)'

dfx canister call ic-rusqlite-backend add '("John","test2", 34: nat32)'

dfx canister call ic-rusqlite-backend add '("Mark","test3", 19: nat32)'

# list persons
dfx canister call ic-rusqlite-backend list
