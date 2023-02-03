#!/bin/bash

TMPDIR="$(mktemp -d /dev/shm/filefinder_test_XXXXXXXXXX)"

echo 123 > "$TMPDIR"/timefile.txt 

echo "$LINENO"

diff <(
"$BINARY" -p space/ -e '(and0 (isfile0) (path1 (exec1 "false")))'
) <( cat <<EOF
EOF
)

echo "$LINENO"

diff <(
"$BINARY" -p "$TMPDIR" -e '(and0 isfile (path1 (exec1 "grep -q 123")))'
) <( cat <<EOF
$TMPDIR/timefile.txt
EOF
)

