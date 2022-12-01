#!/bin/bash

set -x
set -e
set -u

test -v BINARY

set +x

for TEST in ./test[0-9][0-9][0-9].sh; do
 echo "$TEST"
 $TEST
done

