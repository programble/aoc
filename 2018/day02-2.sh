input=$(mktemp)
cat > $input
for i in $(seq $(head -n 1 $input | wc -c)); do
	colrm $i $i < $input | sort | uniq -d
done
rm $input
