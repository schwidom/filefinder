#!/bin/bash

echo $0

diff <( $BINARY -p space/ | sort ) \
     <( find space/ | sort)

diff <( $BINARY -p space/ -e '(true)' | sort ) \
     <( find space/ | sort)


# { $BINARY -p space/ -e '(and (basename e) inject) '; }| sort

# diff <( { $BINARY -p space/ -e '(basename e) '; }| sort ) \
#      <( find space/ | sort)

diff <( { $BINARY -p space/ -e '(progn (and (basename e) cut) true)';
          find space/*/e; }| sort | uniq ) \
     <( find space/ | sort)


