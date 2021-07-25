DN=$(dirname $(readlink -f $0))
cd $DN
cd ..

echo "Running in directory: $(pwd)"

DATAS=$(find ./data/task* | grep .in$)
for data in $DATAS; do
  ans="${data%.*}.ans"
  ./calc.sh < $data | diff - $ans
  if [[ -n $? ]]; then
    echo "Failed on test case $data"
    exit 1
  fi
done
