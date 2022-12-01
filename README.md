# filefinder

Is currently not a replacement for the linux find tool.

But this tool allows me to find all rust repositories in my filesystem by calling:

filefinder --exclude-from-file filefinder-exclusions.txt \
 -e '(and (in1 "target" isdir0) (in1 "Cargo.toml" isfile0) (cut0))' -p projects/
 
(
 The command 'cut' means not to search deeper in the found directory
  so the directory is found and the aim is reached.

 And I introduced the arity for all commands with limited arity because I allow 
  a subsequential call after all commands after the last argument, so the
  arity makes clear where the last argument is.

 Who wants to see more of its functionality can currently see a lot in the tests.
)

It is not the fastest one but will be optimized in the future and will be extended by 
tests, functions and documentation.

So stay tuned.

Frank Schwidom
