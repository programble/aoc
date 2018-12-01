seq -f "$(cat)" ${1:-150} \
	| sed 's/^/last/' \
	| bc \
	| cat -n \
	| sort -s -n -k 2 \
	| uniq -d -f 1 \
	| sort -b -n \
	| head -n 1
