#!/bin/bash

TESTBIN="$1"

case "$TESTBIN" in
 ( bin/run_all_tests_debug.sh ) ;;
 ( bin/run_all_tests_release.sh ) ;;
 (*) echo "wrong testbinary ''$TESTBIN'', only bin/run_all_tests_debug.sh or bin/run_all_tests_release.sh allowed"; exit 1 ;;
esac

mkdir -p difflog


{
 unset USE_COMPILER

 bash -c "time \"$TESTBIN\"" >difflog/out_interpreter.txt 2>&1

 diff difflog/expected.txt difflog/out_interpreter.txt 

 export USE_COMPILER=1

 bash -c "time \"$TESTBIN\"" >difflog/out_compiler.txt 2>&1

 diff difflog/expected.txt difflog/out_compiler.txt 

} | less

