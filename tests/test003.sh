#!/bin/bash


diff <( $BINARY -p space/ -e '(in e isdir)' | sed 's+.*+&/e+1' | sort ) \
     <( find space/ -type d -name e | sort )

diff <( $BINARY -p space/ -e '(and (basename e) isdir)' | sort ) \
     <( find space/ -type d -name e | sort )


diff <( $BINARY -p space/ -e '(in e and isdir cut)' | sed 's+.*+&/e+1' | sort ) \
     <( find space/ -maxdepth 2 -type d -name e | sort )


