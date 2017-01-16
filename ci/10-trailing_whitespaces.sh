#!/bin/bash

FOLDER="."
FILES='.+\.\(md\|rs\|\sh\|toml\|txt\|yml\)'

# Exit script on first error
set -o errexit -o nounset

# Explain what we do
echo -n ">>> Seaching for lines with trailing whitespaces..."

# Search for trailing whitespaces
FOUND=0
for FILE in $(find $FOLDER -regex $FILES); do
    if egrep -q " +$" $FILE ; then
        echo -e "\nFound:\t$FILE"
        FOUND=1
    fi
done
test $FOUND == 0
echo -e "\tDone."