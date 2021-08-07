DN="$(dirname $(readlink -f $0))"
cd "$DN"

echo "Running in directory: $(pwd)"

DATAS=$(find ./data/task* | grep .in$)
for data in $DATAS; do
  ans="${data%.*}.ans"
  echo "Running $data -> $ans"
  ./calc.sh < $data | diff - $ans
  if [[ $? != 0 ]]; then
    echo "Failed on test case $data"
    exit 1
  fi
done
