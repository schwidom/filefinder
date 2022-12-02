#!/bin/bash

echo "$LINENO"

diff <( $BINARY -p space/ | sort ) \
     <( find space/ | sort)

echo "$LINENO"

diff <( $BINARY -p space/ -e 't' | sort ) \
     <( find space/ | sort)

echo "$LINENO"

diff <( $BINARY -p space/ -e '(t0)' | sort ) \
     <( find space/ | sort)

echo "$LINENO"

diff <( $BINARY -p space/ -e 'f' | sort ) \
     <( find space/ -false | sort)

echo "$LINENO"

diff <( $BINARY -p space/ -e '(f0)' | sort ) \
     <( find space/ -false | sort)

# not implemented
# diff <( $BINARY -p space/ -e '(f0 ox)' | sort ) \
#      <( find space/ -false | sort)

echo "$LINENO"

diff <( $BINARY -p space/ -e '(ct0 ox)' | sort ) \
     <( find space/ -true | sort)

echo "$LINENO"

diff <( $BINARY -p space/ -e '(cf0 ox)' | sort ) \
     <( find space/ -false | sort)



# { $BINARY -p space/ -e '(and0 (basename1 e) inject) '; }| sort

# diff <( { $BINARY -p space/ -e '(basename1 e) '; }| sort ) \
#      <( find space/ | sort)

echo "$LINENO"

diff <( { $BINARY -p space/ -e '(progn0 (and0 (basename1 e) cut) t)'; }| sort | uniq ) \
     <( find space/ | grep -vF '/e/' | sort)

echo "$LINENO"

diff <( { $BINARY -p space/ -e '(progn0 (basename1 e cut0) t)'; }| sort | uniq ) \
     <( find space/ | grep -vF '/e/' | sort)

echo "$LINENO"

diff <( $BINARY -p space/ -e 'islink' | sort ) \
     <( find space/ -type l | sort)

echo "$LINENO"

diff <( $BINARY -p space/ -e '(islink0)' | sort ) \
     <( find space/ -type l | sort)

echo "$LINENO"

# links which point to nowhere
diff <( $BINARY -p space/ -e '(not0 exists0)' | sort ) \
     <( find space/ -xtype l | sort)

