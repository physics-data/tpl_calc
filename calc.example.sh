#!/bin/bash

# Turn off globbing, so "*" is just "*" and won't be expanded into files in current directory
set -o noglob

# Basic calculation
echo "1 + 2 = $((1 + 2))"

# Dynamic operation
OP='*'
LHS='3'
RHS='5'

VAL="$(($LHS $OP $RHS))"
echo "$LHS $OP $RHS = $VAL"

# Read loop
echo -en "a\nb\nc\n" | while read var; do
  echo "Read line: $var"
done

# See `docs/ideas.md` if you run of of idea
