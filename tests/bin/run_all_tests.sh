#!/bin/bash


if ! test -v BINARY ; then echo "BINARY has to be set"; exit 1; fi

$BINARY -V

for TEST in ./test[0-9][0-9][0-9].sh; do
 echo "$TEST"
 $TEST
done

echo "done (don't forget to check)"
$BINARY -V
