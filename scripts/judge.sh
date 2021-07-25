#!/bin/bash

INPUT=$(mktemp)
ANS=$(mktemp)
TASK=1

function gen() {
  cd ./data/gen
  if [[ $TASK = "1" ]]; then
    cargo run -q --release -- -i $INPUT -s $ANS -o 20
  elif [[ $TASK = "2" ]]; then
    cargo run -q --release -- -i $INPUT -s $ANS -o 20 -n 0.1
  else
    cargo run -q --release -- -i $INPUT -s $ANS -o 20 -n 0.1 -g 10
  fi
  cd ../..
}

function task() {
  echo "Testing task $TASK..."
  for i in $(seq 1 20); do
    gen

    ./calc.sh < $INPUT | diff - $ANS

    if [[ -n $? ]]; then
      echo "Failed on case $i"
      mkdir -p ./failed
      cp $INPUT ./failed/input
      cp $ANS ./failed/ans

      echo "$TASK" > ./judge

      exit 1
    fi
    echo -n "."
  done

  echo "PASS"
}

task
TASK=2
task
TASK=3
task

echo "0" > ./judge
