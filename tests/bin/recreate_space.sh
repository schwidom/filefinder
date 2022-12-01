#!/bin/bash

set -x
set -e
set -u

test -d space/

rm -rf space

./bin/touchpath.sh space/{a,b,c}/{d,e,f}/{g,h,i}.txt
./bin/touchpath.sh space/a/f/space/{a,b,c}/{d,e,f}/{g,h,i}.txt

(cd space
 ln -s nowhere tonowhere 
 ln -s c/f/i.txt to-c-f-i.txt 
 ln -s b/e/ to-b-e 
 ln -s a/d to-a-d
)

echo
echo "done creating space"
