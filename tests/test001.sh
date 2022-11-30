#!/bin/bash

echo $0

diff <( $BINARY -p space/ -e '(or (basename e) (basename f))' -e '(or (basename e) (basename d))') \
     <( $BINARY -p space/ -e '(and (or (basename e) (basename f)) (or (basename e) (basename d)))')

diff <( $BINARY -p space/ -e '(or (basename e) (basename f))' -e '(or (basename e) (basename d))') \
     <( $BINARY -p space/ -e '(basename e)')


