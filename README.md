filefinder
==========

Is currently not a replacement for the linux find tool.

But this tool allows me to find all rust repositories in my filesystem by calling:

> ``` filefinder --exclude-from-file filefinder-exclusions.txt -e '(and0 (in1 "target" isdir0) (in1 "Cargo.toml" isfile0) cut)' -p projects/ ```
 
It is also possible to process an already available list of files via pipe:

> ```find projects/ | filefinder --exclude-from-file filefinder-exclusions.txt --files-from-stdin -e '(and0 (in1 "target" isdir0) (in1 "Cargo.toml" isfile0) cut)'```


( The command 'cut' means not to search deeper in the found directory so the directory is found and the aim is reached. And I introduced a number for the minimum arity for all commands which stays at the end of each command. Who wants to see more of its functionality can currently see a lot in the tests. )

A practical example: which directory in includes contains the files def.hpp and str.hpp ?

> ``` filefinder -p /usr/include/ -e '(and0 (in1 def.hpp isfile0) (in1 str.hpp isfile0))'```

> ```/usr/include/boost/python```

Another Example:

> ```filefinder -p /usr/src/linux/ -e '(dirname1 (regex1 "Doc.*/bridge$"))'```

> ```/usr/src/linux/Documentation/devicetree/bindings/drm/bridge/ptn3460.txt```

It is not the fastest one but will be optimized in the future and will be extended by 
tests, functions and documentation.

So stay tuned.

Frank Schwidom
