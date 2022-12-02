#!/bin/bash


echo "$LINENO"

diff <( $BINARY -p space/ -e '(in1 e isdir0)' | sed 's+.*+&/e+1' | sort ) \
     <( find space/ -type d -name e | sort )

echo "$LINENO"

diff <( $BINARY -p space/ -e '(and0 (basename1 e) isdir)' | sort ) \
     <( find space/ -type d -name e | sort )

echo "$LINENO"

diff <( $BINARY -p space/ -e '(in1 e and0 isdir cut)' | sed 's+.*+&/e+1' | sort ) \
     <( find space/ -maxdepth 2 -type d -name e | sort )

echo "$LINENO"

diff <( $BINARY -p space/ -e '(in1 e in1 g.txt exists0)' | sed 's+.*+&/e/g.txt+1' | sort ) \
     <( find space/ -ipath "*e/g.txt" | sort)

echo "$LINENO"

diff <( $BINARY -p space/ -e '(in1 e in1 g.txt exists0)' ) \
     <( $BINARY -p space/ -e '(in1 e do0 in1 g.txt exists0)' )

echo "$LINENO"

diff <( $BINARY -p space/ -e '(in1 e in1 g.txt exists0)' ) \
     <( $BINARY -p space/ -e '(in1 e and0 t (in1 g.txt exists0))' )

echo "$LINENO"

diff <( $BINARY -p space/ -e '(in1 e in1 g.txt exists0)' ) \
     <( $BINARY -p space/ -e '(in1 e progn0 (in1 g.txt exists0))' )


