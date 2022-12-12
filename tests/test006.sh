#!/bin/bash

echo "$LINENO"

diff <(
cat <<EOF | "$BINARY" --files-from-stdin -e 't' --format="- {extension} - {filestem} - {basename} - {dirname} - {path} - {isfile}  - {islink} - {isdir} - {isempty} - {isreadonly} - {exists} -"
/
//
///
/..
/../
//../
.
..
./.
./..
../.
EOF
) <(
cat<<EOF
-  -  -  -  - / - false  - false - true - false - false - true -
-  -  -  -  - // - false  - false - true - false - false - true -
-  -  -  -  - /// - false  - false - true - false - false - true -
-  -  -  - / - /.. - false  - false - true - false - false - true -
-  -  -  - / - /../ - false  - false - true - false - false - true -
-  -  -  - / - //../ - false  - false - true - false - false - true -
-  -  -  -  - . - false  - false - true - false - false - true -
-  -  -  -  - .. - false  - false - true - false - false - true -
-  -  -  -  - ./. - false  - false - true - false - false - true -
-  -  -  - . - ./.. - false  - false - true - false - false - true -
-  -  -  -  - ../. - false  - false - true - false - false - true -
EOF
)

echo "$LINENO"

diff <(
"$BINARY" -p space -e 't' --format="- {extension} - {filestem} - {basename} - {dirname} - {path} - {isfile}  - {islink} - {isdir} - {isempty} - {isreadonly} - {exists} -"
) <(
cat <<EOF
-  - space - space -  - space - false  - false - true - false - false - true -
-  - tonowhere - tonowhere - space - space/tonowhere - false  - true - false - false - false - false -
-  - c - c - space - space/c - false  - false - true - false - false - true -
-  - emptydir - emptydir - space - space/emptydir - false  - false - true - true - false - true -
-  - b - b - space - space/b - false  - false - true - false - false - true -
-  - emptyfile - emptyfile - space - space/emptyfile - true  - false - false - true - false - true -
-  - a - a - space - space/a - false  - false - true - false - false - true -
- txt - to-c-f-i - to-c-f-i.txt - space - space/to-c-f-i.txt - true  - true - false - false - false - true -
-  - to-emptyfile - to-emptyfile - space - space/to-emptyfile - true  - true - false - true - false - true -
-  - to-b-e - to-b-e - space - space/to-b-e - false  - true - true - false - false - true -
-  - to-a-d - to-a-d - space - space/to-a-d - false  - true - true - false - false - true -
-  - to-emptydir - to-emptydir - space - space/to-emptydir - false  - true - true - true - false - true -
-  - e - e - space/c - space/c/e - false  - false - true - false - false - true -
-  - d - d - space/c - space/c/d - false  - false - true - false - false - true -
-  - f - f - space/c - space/c/f - false  - false - true - false - false - true -
-  - e - e - space/b - space/b/e - false  - false - true - false - false - true -
-  - d - d - space/b - space/b/d - false  - false - true - false - false - true -
-  - f - f - space/b - space/b/f - false  - false - true - false - false - true -
-  - e - e - space/a - space/a/e - false  - false - true - false - false - true -
-  - d - d - space/a - space/a/d - false  - false - true - false - false - true -
-  - f - f - space/a - space/a/f - false  - false - true - false - false - true -
- txt - i - i.txt - space/c/e - space/c/e/i.txt - true  - false - false - false - false - true -
- txt - h - h.txt - space/c/e - space/c/e/h.txt - true  - false - false - false - false - true -
- txt - g - g.txt - space/c/e - space/c/e/g.txt - true  - false - false - false - false - true -
- txt - i - i.txt - space/c/d - space/c/d/i.txt - true  - false - false - false - false - true -
- txt - h - h.txt - space/c/d - space/c/d/h.txt - true  - false - false - false - false - true -
- txt - g - g.txt - space/c/d - space/c/d/g.txt - true  - false - false - false - false - true -
- txt - i - i.txt - space/c/f - space/c/f/i.txt - true  - false - false - false - false - true -
- txt - h - h.txt - space/c/f - space/c/f/h.txt - true  - false - false - false - false - true -
- txt - g - g.txt - space/c/f - space/c/f/g.txt - true  - false - false - false - false - true -
- txt - i - i.txt - space/b/e - space/b/e/i.txt - true  - false - false - false - false - true -
- txt - h - h.txt - space/b/e - space/b/e/h.txt - true  - false - false - false - false - true -
- txt - g - g.txt - space/b/e - space/b/e/g.txt - true  - false - false - false - false - true -
- txt - i - i.txt - space/b/d - space/b/d/i.txt - true  - false - false - false - false - true -
- txt - h - h.txt - space/b/d - space/b/d/h.txt - true  - false - false - false - false - true -
- txt - g - g.txt - space/b/d - space/b/d/g.txt - true  - false - false - false - false - true -
- txt - i - i.txt - space/b/f - space/b/f/i.txt - true  - false - false - false - false - true -
- txt - h - h.txt - space/b/f - space/b/f/h.txt - true  - false - false - false - false - true -
- txt - g - g.txt - space/b/f - space/b/f/g.txt - true  - false - false - false - false - true -
- txt - i - i.txt - space/a/e - space/a/e/i.txt - true  - false - false - false - false - true -
- txt - h - h.txt - space/a/e - space/a/e/h.txt - true  - false - false - false - false - true -
- txt - g - g.txt - space/a/e - space/a/e/g.txt - true  - false - false - false - false - true -
- txt - i - i.txt - space/a/d - space/a/d/i.txt - true  - false - false - false - false - true -
- txt - h - h.txt - space/a/d - space/a/d/h.txt - true  - false - false - false - false - true -
- txt - g - g.txt - space/a/d - space/a/d/g.txt - true  - false - false - false - false - true -
- txt - i - i.txt - space/a/f - space/a/f/i.txt - true  - false - false - false - false - true -
- txt - h - h.txt - space/a/f - space/a/f/h.txt - true  - false - false - false - false - true -
- txt - g - g.txt - space/a/f - space/a/f/g.txt - true  - false - false - false - false - true -
-  - space - space - space/a/f - space/a/f/space - false  - false - true - false - false - true -
-  - c - c - space/a/f/space - space/a/f/space/c - false  - false - true - false - false - true -
-  - b - b - space/a/f/space - space/a/f/space/b - false  - false - true - false - false - true -
-  - a - a - space/a/f/space - space/a/f/space/a - false  - false - true - false - false - true -
-  - e - e - space/a/f/space/c - space/a/f/space/c/e - false  - false - true - false - false - true -
-  - d - d - space/a/f/space/c - space/a/f/space/c/d - false  - false - true - false - false - true -
-  - f - f - space/a/f/space/c - space/a/f/space/c/f - false  - false - true - false - false - true -
-  - e - e - space/a/f/space/b - space/a/f/space/b/e - false  - false - true - false - false - true -
-  - d - d - space/a/f/space/b - space/a/f/space/b/d - false  - false - true - false - false - true -
-  - f - f - space/a/f/space/b - space/a/f/space/b/f - false  - false - true - false - false - true -
-  - e - e - space/a/f/space/a - space/a/f/space/a/e - false  - false - true - false - false - true -
-  - d - d - space/a/f/space/a - space/a/f/space/a/d - false  - false - true - false - false - true -
-  - f - f - space/a/f/space/a - space/a/f/space/a/f - false  - false - true - false - false - true -
- txt - i - i.txt - space/a/f/space/c/e - space/a/f/space/c/e/i.txt - true  - false - false - false - false - true -
- txt - h - h.txt - space/a/f/space/c/e - space/a/f/space/c/e/h.txt - true  - false - false - false - false - true -
- txt - g - g.txt - space/a/f/space/c/e - space/a/f/space/c/e/g.txt - true  - false - false - false - false - true -
- txt - i - i.txt - space/a/f/space/c/d - space/a/f/space/c/d/i.txt - true  - false - false - false - false - true -
- txt - h - h.txt - space/a/f/space/c/d - space/a/f/space/c/d/h.txt - true  - false - false - false - false - true -
- txt - g - g.txt - space/a/f/space/c/d - space/a/f/space/c/d/g.txt - true  - false - false - false - false - true -
- txt - i - i.txt - space/a/f/space/c/f - space/a/f/space/c/f/i.txt - true  - false - false - false - false - true -
- txt - h - h.txt - space/a/f/space/c/f - space/a/f/space/c/f/h.txt - true  - false - false - false - false - true -
- txt - g - g.txt - space/a/f/space/c/f - space/a/f/space/c/f/g.txt - true  - false - false - false - false - true -
- txt - i - i.txt - space/a/f/space/b/e - space/a/f/space/b/e/i.txt - true  - false - false - false - false - true -
- txt - h - h.txt - space/a/f/space/b/e - space/a/f/space/b/e/h.txt - true  - false - false - false - false - true -
- txt - g - g.txt - space/a/f/space/b/e - space/a/f/space/b/e/g.txt - true  - false - false - false - false - true -
- txt - i - i.txt - space/a/f/space/b/d - space/a/f/space/b/d/i.txt - true  - false - false - false - false - true -
- txt - h - h.txt - space/a/f/space/b/d - space/a/f/space/b/d/h.txt - true  - false - false - false - false - true -
- txt - g - g.txt - space/a/f/space/b/d - space/a/f/space/b/d/g.txt - true  - false - false - false - false - true -
- txt - i - i.txt - space/a/f/space/b/f - space/a/f/space/b/f/i.txt - true  - false - false - false - false - true -
- txt - h - h.txt - space/a/f/space/b/f - space/a/f/space/b/f/h.txt - true  - false - false - false - false - true -
- txt - g - g.txt - space/a/f/space/b/f - space/a/f/space/b/f/g.txt - true  - false - false - false - false - true -
- txt - i - i.txt - space/a/f/space/a/e - space/a/f/space/a/e/i.txt - true  - false - false - false - false - true -
- txt - h - h.txt - space/a/f/space/a/e - space/a/f/space/a/e/h.txt - true  - false - false - false - false - true -
- txt - g - g.txt - space/a/f/space/a/e - space/a/f/space/a/e/g.txt - true  - false - false - false - false - true -
- txt - i - i.txt - space/a/f/space/a/d - space/a/f/space/a/d/i.txt - true  - false - false - false - false - true -
- txt - h - h.txt - space/a/f/space/a/d - space/a/f/space/a/d/h.txt - true  - false - false - false - false - true -
- txt - g - g.txt - space/a/f/space/a/d - space/a/f/space/a/d/g.txt - true  - false - false - false - false - true -
- txt - i - i.txt - space/a/f/space/a/f - space/a/f/space/a/f/i.txt - true  - false - false - false - false - true -
- txt - h - h.txt - space/a/f/space/a/f - space/a/f/space/a/f/h.txt - true  - false - false - false - false - true -
- txt - g - g.txt - space/a/f/space/a/f - space/a/f/space/a/f/g.txt - true  - false - false - false - false - true -
EOF
)

echo "$LINENO"

diff <(
"$BINARY" -p space -c -e 'isfile' --format="- {extension} - {filestem} - {basename} - {dirname} - {path} - {isfile}  - {islink} - {isdir} - {isempty} - {isreadonly} - {exists} -"
"$BINARY" -p space -c -e 'isdir' --format="- {extension} - {filestem} - {basename} - {dirname} - {path} - {isfile}  - {islink} - {isdir} - {isempty} - {isreadonly} - {exists} -"
) <(
cat <<EOF
false -  - space - space -  - space - false  - false - true - false - false - true -
true -  - space - space -  - space - false  - false - true - false - false - true -
EOF
)

