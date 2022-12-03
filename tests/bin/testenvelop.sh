#!/bin/bash

TESTBIN="$1"

case "$TESTBIN" in
 ( bin/run_all_tests_debug.sh ) ;;
 ( bin/run_all_tests_release.sh ) ;;
 (*) echo "wrong testbinary ''$TESTBIN'', only bin/run_all_tests_debug.sh or bin/run_all_tests_release.sh allowed"; exit 1 ;;
esac

mkdir -p difflog

"$TESTBIN" >difflog/out.txt 2>&1

{ diff difflog/expected.txt difflog/out.txt && echo all tests passed || echo something failed; } | less


