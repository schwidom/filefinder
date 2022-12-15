#!/bin/bash

TMPDIR="$(mktemp -d /dev/shm/filefinder_test_XXXXXXXXXX)"

touch -a -d "2022-01-17 20:23:45" "$TMPDIR"/timefile.txt # beware of a filesystem that is mounted with "noatime"
touch -m -d "2022-01-16 20:23:45" "$TMPDIR"/timefile.txt
# touch -d "2022-01-15 20:23:45" "$TMPDIR"/timefile.txt # creation time cannot be changed, this changes the mtime

echo "$LINENO"

diff <(
"$BINARY" -p "$TMPDIR" -e '(basename1 timefile.txt)' --format '{atime} {mtime}'
) <( cat <<EOF
2022-01-17 20:23:45 +01:00 2022-01-16 20:23:45 +01:00
EOF
)

echo "$LINENO"

diff <(
"$BINARY" -p "$TMPDIR" -e '(atime1 (startswith1 "2022-01-17"))' 
) <( cat <<EOF
$TMPDIR/timefile.txt
EOF
)

echo "$LINENO"

diff <(
"$BINARY" -p "$TMPDIR" -e '(mtime1 (startswith1 "2022-01-16"))' 
) <( cat <<EOF
$TMPDIR/timefile.txt
EOF
)


