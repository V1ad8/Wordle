#!/bin/bash

echo Words that appear more than once:
sort output.csv | uniq -d

echo Words that do not appear in the output:
while read word; do
    is=$(grep -o -i $word output.csv | wc -l)
    if [ $is -eq 0 ]; then
        echo "$word"
    fi
done < words.txt