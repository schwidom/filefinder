#!/bin/bash

set -x
set -e
set -u

test -v BINARY

$BINARY -V

set +x

for TEST in ./test[0-9][0-9][0-9].sh; do
 echo "$TEST"
 $TEST
done

echo "done (don't forget to check)"
$BINARY -V
