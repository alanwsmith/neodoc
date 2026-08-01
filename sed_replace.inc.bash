#!/bin/bash

# NOTES:
# -o in find is for "or"
# -E in sed is for extended (modern regex)
# -i in sed sets up to edit the files in place

find "/Users/alan/workshop/neodoc/content/" \
  -type f \( -iname "*.js" -o -iname "*.html" -o -iname "*.txt" -o -iname "*.md" -o -iname "*.neo" \) -print0 | xargs -0 sed -E -i "" \
  's|file_list|fileList|g'


