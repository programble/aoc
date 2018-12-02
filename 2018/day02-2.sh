input=$(mktemp)
cat > $input
for i in $(seq 26); do
	colrm $i $i < $input | sort | uniq -d
done
rm $input
