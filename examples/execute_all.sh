#! /usr/bin/sh

for f in $(ls *.mila)
do
    echo "executing $f"
    if [ $f = "stdinput.mila" ]; then
       echo "skip $f"
    else
        mila $f
    fi
done