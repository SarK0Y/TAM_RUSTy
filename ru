#!/bin/bash
out=""
for i in "$@"
do
echo $i
out="$out $i"
done
#out=${out// /\\ }
echo "ru=$out"
bash -c  "LC_ALL=ru_RU.utf8 $out"
