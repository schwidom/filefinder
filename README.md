# filefinder

Is currently not a replacement for the linux find tool.

But this tool allows me to find all rust repositories in my filesystem by calling:

filefinder --exclude-from-file filefinder-exclusions.txt \
 -e '(and (in "target" isdir) (in "Cargo.toml" isfile) (cut))' -p projects/
 
('cut' means not to search in the found directory)

It is not the fastest one but will be optimized in the future and will be extended by 
tests, functions and documentation.

So stay tuned.

Frank Schwidom
