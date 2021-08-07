#!/bin/bash

SCRIPT_PATH=$(readlink -f "${BASH_SOURCE[0]}")
DN="$(dirname "$SCRIPT_PATH")"
cd "${DN}/../"

echo "Running in directory: $(pwd)"

DATAS=$(find ./data/task* | grep .in$)
for data in $DATAS; do
  ans="${data%.*}.ans"
  echo "Running $data -> $ans"
  bash ./calc.sh < $data | diff - $ans
  if [[ $? != 0 ]]; then
    echo "Failed on test case $data"
    exit 1
  fi
done
