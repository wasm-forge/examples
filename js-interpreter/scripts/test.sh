#!/bin/bash

#test executing the eval function
dfx canister call js-interpreter-backend eval '("
    function factorialize(num) {
        if (num < 0) 
                return -1;
        else if (num == 0) 
            return 1;
        else {
            return (num * factorialize(num - 1));
        }
    }
    factorialize(5);
")'

