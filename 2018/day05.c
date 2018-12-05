#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main() {
	char buf[50000];
	size_t len = fread(buf, 1, sizeof(buf), stdin);
	for (size_t i = 0; i < len - 1; ++i) {
		if (tolower(buf[i]) != tolower(buf[i + 1])) continue;
		if (islower(buf[i]) == islower(buf[i + 1])) continue;
		len -= 2;
		memmove(&buf[i], &buf[i + 2], len - i);
		i = (size_t)-1;
	}
	printf("%zu\n", len);
}
