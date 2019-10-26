if [ ! -f "./db_patch_count" ]; then
  echo "-1" > ./db_patch_count
fi
COUNT=$(cat ./db_patch_count)
COUNT=$(expr ${COUNT} + 0)

rm merge.sql &> /dev/null
for filename in ./*.sql; do
  if [ ! -f "${filename}" ]; then
    continue
  fi

  FILE=${filename:2}
  VERSION=$(expr ${FILE:0:5} + 0)

  if (( ${VERSION} > ${COUNT} )); then
    cat ${filename} >> merge.sql
    echo "" >> merge.sql
    echo ${VERSION} > ./db_patch_count
  fi
done