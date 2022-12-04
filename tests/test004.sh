#!/bin/bash

echo "$LINENO"

diff <(echo true) <( $BINARY -p space/ -c -e 't')

echo "$LINENO"

diff <(echo false) <( $BINARY -p space/ -c -e 'f')


echo "$LINENO"

diff <(echo true) <( $BINARY -p space/ -c -e '(t0)')

echo "$LINENO"

diff <(echo false) <( $BINARY -p space/ -c -e '(f0)')


echo "$LINENO"

diff <(echo true) <( $BINARY -p space/ -c -e '(ct0)')

echo "$LINENO"

diff <(echo false) <( $BINARY -p space/ -c -e '(cf0)')


echo "$LINENO"

diff <(echo true) <( $BINARY -p space/ -c -e '(basename1 space)')

echo "$LINENO"

diff <(echo true) <( $BINARY -p space/ -c -e '(basename1 (regex1 space))')
diff <(echo true) <( $BINARY -p space/ -c -e '(basename1 (regex1 "s.*e"))')
diff <(echo true) <( $BINARY -p space/ -c -e '(basename1 (regex1 "s.*.*e"))')
diff <(echo false) <( $BINARY -p space/ -c -e '(basename1 (regex1 "s.*z.*e"))')

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

diff <(echo false) <( $BINARY -p space/ -c -e '(basename1 spac)')

echo "$LINENO"

diff <(echo false) <( $BINARY -p space/ -c -e '(basename1 space f0)')

echo "$LINENO"

diff <(echo true) <( $BINARY -p space/a/d/g.txt -c -e '(basename1 g.txt)')
diff <(echo true) <( $BINARY -p space/a/d/g.txt -c -e '(filestem1 g)')
diff <(echo true) <( $BINARY -p space/a/d/g.txt -c -e '(extension1 txt)')

diff <(echo true) <( $BINARY -p space/ -c -e '(in1 noexistent)')
diff <(echo false) <( $BINARY -p space/ -c -e '(in1 noexistent exists0)')
diff <(echo true) <( $BINARY -p space/ -c -e '(in1 noexistent not0 exists0)')

diff <(echo true) <( $BINARY -p space/ -c -e '(in1 a isdir0)')
diff <(echo true) <( $BINARY -p space/ -c -e '(in1 a in1 d isdir0)')
diff <(echo true) <( $BINARY -p space/ -c -e '(in1 a in1 d in1 g.txt isfile0)')
diff <(echo true) <( $BINARY -p space/ -c -e '(in1 a in1 d in1 g.txt basename1 g.txt)')
diff <(echo true) <( $BINARY -p space/ -c -e '(in1 a/d/g.txt isfile0)')
diff <(echo true) <( $BINARY -p space/ -c -e '(in1 a/d/g.txt basename1 g.txt)')
diff <(echo true) <( $BINARY -p space/ -c -e '(in1 a/d/g.txt progn0 (basename1 g.txt))')

diff <(echo true) <( $BINARY -p space/ -c -e '(in1 a/d/g.txt progn0 (path1 space/a/d/g.txt))')
diff <(echo true) <( $BINARY -p space/ -c -e '(in1 a/d/g.txt progn0 (dirname1 space/a/d))')
diff <(echo true) <( $BINARY -p space/ -c -e '(in1 a/d/g.txt inback0 basename1 d)')

diff <(echo true) <( $BINARY -p space/a/d/g.txt -c -e 'isfile')
diff <(echo true) <( $BINARY -p space/a/d/g.txt -c -e '(isfile0)')
diff <(echo true) <( $BINARY -p space/a/d/g.txt -c -e 'exists')
diff <(echo true) <( $BINARY -p space/a/d/g.txt -c -e '(exists0)')
diff <(echo false) <( $BINARY -p space/a/d/g.txt -c -e 'isdir')
diff <(echo false) <( $BINARY -p space/a/d/g.txt -c -e '(isdir0)')

echo "$LINENO"

diff <(echo true) <( $BINARY -p space/ -c -e 'isdir')
diff <(echo false) <( $BINARY -p space/ -c -e 'isfile')
diff <(echo true) <( $BINARY -p space/ -c -e 'exists')
diff <(echo false) <( $BINARY -p space/ -c -e 'islink')

diff <(echo true) <( $BINARY -p space/ -c -e '(isdir0)')
diff <(echo false) <( $BINARY -p space/ -c -e '(isfile0)')
diff <(echo true) <( $BINARY -p space/ -c -e '(exists0)')
diff <(echo false) <( $BINARY -p space/ -c -e '(islink0)')

echo "$LINENO"

diff <(echo false) <( $BINARY -p space/ -c -e '(or0)')
diff <(echo false) <( $BINARY -p space/ -c -e '(not0)')
diff <(echo true) <( $BINARY -p space/ -c -e '(and0)')

diff <(echo true) <( $BINARY -p space/ -c -e '(or0 t)')
diff <(echo false) <( $BINARY -p space/ -c -e '(or0 f)')
diff <(echo true) <( $BINARY -p space/ -c -e '(or0 f t)')
diff <(echo true) <( $BINARY -p space/ -c -e '(and0 t)')
diff <(echo false) <( $BINARY -p space/ -c -e '(and0 f)')
diff <(echo false) <( $BINARY -p space/ -c -e '(and0 f t)')

diff <(echo false) <( $BINARY -p space/ -c -e '(not0 t0)')
diff <(echo true) <( $BINARY -p space/ -c -e '(not0 f0)')

diff <(echo false) <( $BINARY -p space/ -c -e '(not0 and0 t)')
diff <(echo true) <( $BINARY -p space/ -c -e '(not0 or0 f)')

diff <(echo true) <( $BINARY -p space/ -c -e '(not0 and0 t f)')
diff <(echo false) <( $BINARY -p space/ -c -e '(not0 or0 f t)')

