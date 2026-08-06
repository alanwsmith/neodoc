#!/bin/bash

# NOTES:
# -o in find is for "or"
# -E in sed is for extended (modern regex)
# -i in sed sets up to edit the files in place

find "/Users/alan/workshop/neodoc/src/" \
  -type f \( -iname "*.rs" \) -print0 | xargs -0 sed -E -i "" \
  's|new_extra\((.*) "(.*)"\)|new_extra(\1 vec!["\2"])|g'


