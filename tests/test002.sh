#!/bin/bash

echo "$LINENO"

diff <( $BINARY -p space/ | sort ) \
     <( find space/ | sort)

echo "$LINENO"

diff <( $BINARY -p space/ -e 'true' | sort ) \
     <( find space/ | sort)

echo "$LINENO"

diff <( $BINARY -p space/ -e '(true)' | sort ) \
     <( find space/ | sort)

echo "$LINENO"

diff <( $BINARY -p space/ -e 'false' | sort ) \
     <( find space/ -false | sort)

echo "$LINENO"

diff <( $BINARY -p space/ -e '(false)' | sort ) \
     <( find space/ -false | sort)

# not implemented
# diff <( $BINARY -p space/ -e '(false ox)' | sort ) \
#      <( find space/ -false | sort)

echo "$LINENO"

diff <( $BINARY -p space/ -e '(ct ox)' | sort ) \
     <( find space/ -true | sort)

echo "$LINENO"

diff <( $BINARY -p space/ -e '(cf ox)' | sort ) \
     <( find space/ -false | sort)



# { $BINARY -p space/ -e '(and (basename e) inject) '; }| sort

# diff <( { $BINARY -p space/ -e '(basename e) '; }| sort ) \
#      <( find space/ | sort)

echo "$LINENO"

diff <( { $BINARY -p space/ -e '(progn (and (basename e) cut) true)'; }| sort | uniq ) \
     <( find space/ | grep -vF '/e/' | sort)

echo "$LINENO"

diff <( $BINARY -p space/ -e 'islink' | sort ) \
     <( find space/ -type l | sort)

echo "$LINENO"

diff <( $BINARY -p space/ -e '(islink)' | sort ) \
     <( find space/ -type l | sort)

echo "$LINENO"

# links which point to nowhere
diff <( $BINARY -p space/ -e '(not exists)' | sort ) \
     <( find space/ -xtype l | sort)

