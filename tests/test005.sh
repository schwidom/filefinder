
echo "$LINENO"

diff <( "$BINARY" -p space/ --exclude-from-file <( echo space/b/e) | sort ) \
     <( find space/ | grep -v '^space/b/e'| sort )

# diff <( "$BINARY" -p space/ -e 'empty') \
#      <( find space/ -empty)

echo "$LINENO"

diff <( "$BINARY" -p space/ -e '(and0 empty (not0 islink0))') \
     <( find space/ -empty)

echo "$LINENO"

diff <( "$BINARY" -p space/ -e '(and0 (empty0) (not0 islink0))') \
     <( find space/ -empty)



