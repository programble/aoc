alpha='a b c d e f g h i j k l m n o p q r s t u v w x y z'
twos() {
	for a in $alpha; do
		tr -c -d "$a\n" < $1 \
			| grep -n '^..$' \
			| cut -d ':' -f 1
	done | sort -u | wc -l
}
threes() {
	for a in $alpha; do
		tr -c -d "$a\n" < $1 \
			| grep -n '^...$' \
			| cut -d ':' -f 1
	done | sort -u | wc -l
}
input=$(mktemp)
cat > $input
echo $(( $(twos $input) * $(threes $input) ))
rm $input
