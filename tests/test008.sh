#!/bin/bash

# TMPDIR="$(mktemp -d /dev/shm/filefinder_test_XXXXXXXXXX)"
# 
# echo 123 > "$TMPDIR"/timefile.txt 

echo "$LINENO"

diff <(
"$BINARY" -p space/ -e '(and0 (isfile0) (path1 (exec1 "false")))'
) <( cat <<EOF
EOF
)

echo "$LINENO"

diff <(
"$BINARY" -p space -e '(and0 isfile (path1 (exec1 "grep -q 123")))'
) <( cat <<EOF
space/filledfilewith123
EOF
)

echo "$LINENO"

diff <(
"$BINARY" -p space -e '(filecontents1 "123
")'
) <( cat <<EOF
space/filledfilewith123
EOF
)

echo "$LINENO"

diff <(
"$BINARY" -p space -e '(filecontents1 (regex1 "123"))'
) <( cat <<EOF
space/filledfilewith123
EOF
)

echo "$LINENO"

diff <(
"$BINARY" -p space -e '(and0 isfile (not0 or0 (filecontents1 (<1 "123
")) (filecontents1 (>1 "123
"))))'
) <( cat <<EOF
space/filledfilewith123
EOF
)

echo "$LINENO"

diff <(
"$BINARY" -p space -e '(and0 isfile (|0 not0 filecontents1 (<1 "123
") filecontents1 (>1 "123
")))'
) <( cat <<EOF
space/filledfilewith123
EOF
)

