#!/bin/bash

# Files
bits_file="bits.csv"
frequency_file="frequency.csv"
output_file="combined.csv"

# Combine the CSVs by the 'word' column using join
join -t, -1 1 -2 1 -o 1.1,1.2,2.2 <(sort -t, -k1,1 $bits_file) <(sort -t, -k1,1 $frequency_file) > $output_file

echo "Combined CSV created: $output_file"
