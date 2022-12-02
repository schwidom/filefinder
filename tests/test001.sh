#!/bin/bash

echo "$LINENO"

diff <( $BINARY -p space/ -e '(or0 (basename1 e) (basename1 f))' -e '(or0 (basename1 e) (basename1 d))') \
     <( $BINARY -p space/ -e '(and0 (or0 (basename1 e) (basename1 f)) (or0 (basename1 e) (basename1 d)))')

echo "$LINENO"

diff <( $BINARY -p space/ -e '(or0 (basename1 e) (basename1 f))' -e '(or0 (basename1 e) (basename1 d))') \
     <( $BINARY -p space/ -e '(and0 (or0 (basename1 e) (basename1 f)) (or0 (basename1 e) (basename1 d)))')

echo "$LINENO"

diff <( $BINARY -p space/ -e '(or0 (basename1 e) (basename1 f))' -e '(or0 (basename1 e) (basename1 d))') \
     <( $BINARY -p space/ -e '(basename1 e)')

echo "$LINENO"

diff <( $BINARY -p space/ -e '(t0)') \
     <( $BINARY -p space/ -e 't')

echo "$LINENO"

diff <( $BINARY -p space/ -e '(t0 t0)') \
     <( $BINARY -p space/ -e 't')

echo "$LINENO"

diff <( $BINARY -p space/ -e '(t0 f0)') \
     <( $BINARY -p space/ -e 't')

echo "$LINENO"

diff <( $BINARY -p space/ -e '(ct0 f0)') \
     <( $BINARY -p space/ -e 't')

echo "$LINENO"

diff <( $BINARY -p space/ -e '(basename1 e)') \
     <( $BINARY -p space/ -e '(basename1 e t0)')


