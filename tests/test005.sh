
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

# "$BINARY" -p space/ -e '(basename1 "space")'

echo "$LINENO"

diff <( "$BINARY" -p space/ -e '(in1 (basename1 "space"))' | sort ) \
     <( echo 'space/a/f' )

# TODO : use mktemp
## echo "$LINENO"
## 
## diff <( "$BINARY" --debug-log-cuts-file /tmp/001.txt -p space/ -e '(progn0 (in1 (basename1 "space") cut0) t)' | sort ) \
##      <( find space/ | grep -vF 'space/a/f/' | sort )
## 
## diff /tmp/001.txt <( echo 'cut: Some("space/a/f")')

echo "$LINENO"

diff <( "$BINARY" -p space/ -e '(progn0 (in1 (basename1 "space") cut0) t)' | sort ) \
     <( find space/ | grep -vF 'space/a/f/' | sort )

echo "$LINENO"

diff <( "$BINARY" -p space/ -e '(progn0 (in1 (basename1 "space" cut0)) t)' | sort ) \
     <( find space/ | grep -vF 'space/a/f/' | sort )

# TODO : use mktemp
## echo "$LINENO"
## 
## diff <( "$BINARY" --debug-log-cuts-file /tmp/002.txt -p space/ -e '(progn0 (in1 (basename1 "space" cut0)) t)' | sort ) \
##      <( find space/ | grep -vF 'space/a/f/' | sort )
## 
## diff /tmp/002.txt <( echo 'cut: Some("space/a/f")')

