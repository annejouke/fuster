#!/bin/bash

git clone https://github.com/github/gitignore.git tmp
find tmp -name '*.gitignore' -exec cp {} ../assets/gitignore \;
rm -rf tmp
