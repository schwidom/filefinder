#!/bin/bash

echo "$LINENO"

diff <(echo true) <( $BINARY -p space/ -c 't')

echo "$LINENO"

diff <(echo false) <( $BINARY -p space/ -c 'f')


echo "$LINENO"

diff <(echo true) <( $BINARY -p space/ -c '(t0)')

echo "$LINENO"

diff <(echo false) <( $BINARY -p space/ -c '(f0)')


echo "$LINENO"

diff <(echo true) <( $BINARY -p space/ -c '(ct0)')

echo "$LINENO"

diff <(echo false) <( $BINARY -p space/ -c '(cf0)')


echo "$LINENO"

diff <(echo true) <( $BINARY -p space/ -c '(basename1 space)')

echo "$LINENO"

diff <(echo true) <( $BINARY -p space/ -c '(basename1 (regex1 space))')
diff <(echo true) <( $BINARY -p space/ -c '(basename1 (regex1 "s.*e"))')
diff <(echo true) <( $BINARY -p space/ -c '(basename1 (regex1 "s.*.*e"))')
diff <(echo false) <( $BINARY -p space/ -c '(basename1 (regex1 "s.*z.*e"))')

diff <( $BINARY -p space/ -e '(path1 (regex1 "sp.*g.txt$"))') \
     <( find space/ -iregex '^sp.*g.txt$')

diff <( $BINARY -p space/ -e '(path1 (regex1 ".*g.txt$"))') \
     <( find space/ -iregex '^sp.*g.txt$')

diff <( $BINARY -p space/ -e '(path1 (regex1 "g.txt$"))') \
     <( find space/ -iregex '^sp.*g.txt$')

diff <( $BINARY -p space/ -e '(path1 (regex1 "^g.txt$"))') \
     <( find space/ -iregex '^sp.*g.txt$' -a -false)

diff <( $BINARY -p space/ -e '(path1 (regex1 "^g.txt$"))') \
     <( find space/ -iregex '^g.txt$')

diff <( $BINARY -p space/ -e '(path1 (regex1 "^sp.*g.txt$"))') \
     <( find space/ -iregex '^sp.*g.txt$')

diff <( $BINARY -p space/ -e '(dirname1 (regex1 "d"))') \
     <( find space/ -iregex '.*d.*' -a -not -name '*d*')

echo "$LINENO"

diff <(echo false) <( $BINARY -p space/ -c '(basename1 spac)')

echo "$LINENO"

diff <(echo false) <( $BINARY -p space/ -c '(basename1 space f0)')

echo "$LINENO"

diff <(echo true) <( $BINARY -p space/a/d/g.txt -c '(basename1 g.txt)')
diff <(echo true) <( $BINARY -p space/a/d/g.txt -c '(filestem1 g)')
diff <(echo true) <( $BINARY -p space/a/d/g.txt -c '(extension1 txt)')

diff <(echo true) <( $BINARY -p space/ -c '(in1 noexistent)')
diff <(echo false) <( $BINARY -p space/ -c '(in1 noexistent exists0)')
diff <(echo true) <( $BINARY -p space/ -c '(in1 noexistent not0 exists0)')

diff <(echo true) <( $BINARY -p space/ -c '(in1 a isdir0)')
diff <(echo true) <( $BINARY -p space/ -c '(in1 a in1 d isdir0)')
diff <(echo true) <( $BINARY -p space/ -c '(in1 a in1 d in1 g.txt isfile0)')
diff <(echo true) <( $BINARY -p space/ -c '(in1 a in1 d in1 g.txt basename1 g.txt)')
diff <(echo true) <( $BINARY -p space/ -c '(in1 a/d/g.txt isfile0)')
diff <(echo true) <( $BINARY -p space/ -c '(in1 a/d/g.txt basename1 g.txt)')
diff <(echo true) <( $BINARY -p space/ -c '(in1 a/d/g.txt progn0 (basename1 g.txt))')

diff <(echo true) <( $BINARY -p space/ -c '(in1 a/d/g.txt progn0 (path1 space/a/d/g.txt))')
diff <(echo true) <( $BINARY -p space/ -c '(in1 a/d/g.txt progn0 (dirname1 space/a/d))')
diff <(echo true) <( $BINARY -p space/ -c '(in1 a/d/g.txt inback0 basename1 d)')

diff <(echo true) <( $BINARY -p space/a/d/g.txt -c 'isfile')
diff <(echo true) <( $BINARY -p space/a/d/g.txt -c '(isfile0)')
diff <(echo true) <( $BINARY -p space/a/d/g.txt -c 'exists')
diff <(echo true) <( $BINARY -p space/a/d/g.txt -c '(exists0)')
diff <(echo false) <( $BINARY -p space/a/d/g.txt -c 'isdir')
diff <(echo false) <( $BINARY -p space/a/d/g.txt -c '(isdir0)')

echo "$LINENO"

diff <(echo true) <( $BINARY -p space/ -c 'isdir')
diff <(echo false) <( $BINARY -p space/ -c 'isfile')
diff <(echo true) <( $BINARY -p space/ -c 'exists')
diff <(echo false) <( $BINARY -p space/ -c 'islink')

diff <(echo true) <( $BINARY -p space/ -c '(isdir0)')
diff <(echo false) <( $BINARY -p space/ -c '(isfile0)')
diff <(echo true) <( $BINARY -p space/ -c '(exists0)')
diff <(echo false) <( $BINARY -p space/ -c '(islink0)')

echo "$LINENO"

diff <(echo false) <( $BINARY -p space/ -c '(or0)')
diff <(echo false) <( $BINARY -p space/ -c '(not0)')
diff <(echo true) <( $BINARY -p space/ -c '(and0)')

diff <(echo true) <( $BINARY -p space/ -c '(or0 t)')
diff <(echo false) <( $BINARY -p space/ -c '(or0 f)')
diff <(echo true) <( $BINARY -p space/ -c '(or0 f t)')
diff <(echo true) <( $BINARY -p space/ -c '(and0 t)')
diff <(echo false) <( $BINARY -p space/ -c '(and0 f)')
diff <(echo false) <( $BINARY -p space/ -c '(and0 f t)')

diff <(echo false) <( $BINARY -p space/ -c '(not0 t0)')
diff <(echo true) <( $BINARY -p space/ -c '(not0 f0)')

diff <(echo false) <( $BINARY -p space/ -c '(not0 and0 t)')
diff <(echo true) <( $BINARY -p space/ -c '(not0 or0 f)')

diff <(echo true) <( $BINARY -p space/ -c '(not0 and0 t f)')
diff <(echo false) <( $BINARY -p space/ -c '(not0 or0 f t)')

