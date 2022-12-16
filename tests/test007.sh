#!/bin/bash

TMPDIR="$(mktemp -d /dev/shm/filefinder_test_XXXXXXXXXX)"

touch -a -d "2022-01-17 20:23:45" "$TMPDIR"/timefile.txt # beware of a filesystem that is mounted with "noatime"
touch -m -d "2022-01-16 20:23:45" "$TMPDIR"/timefile.txt
# touch -d "2022-01-15 20:23:45" "$TMPDIR"/timefile.txt # creation time cannot be changed, this changes the mtime and atime

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

echo "$LINENO"

diff <(
"$BINARY" -p space -e '(basename1 (and0 (startswith1 s) (endswith1 e)))'
) <( cat <<EOF
space
space/a/f/space
EOF
)

echo "$LINENO"

diff <(
"$BINARY" -p space -e '(basename1 (startswith1 s endswith1 e))'
) <( cat <<EOF
space
space/a/f/space
EOF
)

echo "$LINENO"

diff <(
"$BINARY" -p "$TMPDIR" -e '(basename1 timefile.txt)' --format '{path} {size}'
) <( cat <<EOF
$TMPDIR/timefile.txt 0
EOF
)

echo "$LINENO"

diff <(
"$BINARY" -p "$TMPDIR" -e '(size_string1 "0")' --format '{path} {size}'
) <( cat <<EOF
$TMPDIR/timefile.txt 0
EOF
)

echo "$LINENO"

diff <(
"$BINARY" -p "$TMPDIR" -e '(size1 0)' --format '{path} {size}'
) <( cat <<EOF
$TMPDIR/timefile.txt 0
EOF
)


echo "$LINENO"

diff <(
"$BINARY" -p "$TMPDIR" -e '(size1 (<1 1))' --format '{path} {size}'
) <( cat <<EOF
$TMPDIR/timefile.txt 0
EOF
)


echo "$LINENO"

diff <(
"$BINARY" -p "$TMPDIR" -e '(size1 (not0 not0 <1 1))' --format '{path} {size}'
) <( cat <<EOF
$TMPDIR/timefile.txt 0
EOF
)

echo "$LINENO"

diff <(
"$BINARY" -p "$TMPDIR" -e '(size1 (<1 1e0))' --format '{path} {size}'
) <( cat <<EOF
$TMPDIR/timefile.txt 0
EOF
)


