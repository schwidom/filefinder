
echo "$LINENO"

diff <( "$BINARY" -p space/ --exclude-from-file <( echo space/b/e) | sort ) \
     <( find space/ | grep -v '^space/b/e'| sort )

# diff <( "$BINARY" -p space/ -e 'isempty') \
#      <( find space/ -empty)

echo "$LINENO"

diff <( "$BINARY" -p space/ -e '(and0 isempty (not0 islink0))') \
     <( find space/ -empty)

echo "$LINENO"

diff <( "$BINARY" -p space/ -e '(and0 (isempty0) (not0 islink0))') \
     <( find space/ -empty)

echo "$LINENO"

diff <( "$BINARY" -p space/ -e '(injectonce1 "space/")' | sort ) \
     <( { find space/ ; find space/; } | sort )

echo "$LINENO"

diff <( "$BINARY" -p space/ -e '(injectonce1 "space/a")' | sort ) \
     <( { find space/ ; find space/a; } | sort )

