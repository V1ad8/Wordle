#!/bin/bash

# Define the input CSV file
input_file="combined.csv"

# Use awk to read the CSV, and sort based on the bits column
# Then check for duplicate bits values
awk -F, 'NR > 1 {print $2, $1}' "$input_file" | sort -n | awk '
{
  if ($1 == prev_bits) {
    print prev_word, "and", $2, "have the same bits value:", $1
  }
  prev_bits = $1
  prev_word = $2
}
'
