#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static size_t react(char *buf, size_t len) {
	for (size_t i = 0; i < len - 1; ++i) {
		if (tolower(buf[i]) != tolower(buf[i + 1])) continue;
		if (islower(buf[i]) == islower(buf[i + 1])) continue;
		len -= 2;
		memmove(&buf[i], &buf[i + 2], len - i);
		i = (size_t)-1;
	}
	return len;
}

int main() {
	char buf[50000];
	size_t len = fread(buf, 1, sizeof(buf), stdin);

	char buf1[50000];
	memcpy(buf1, buf, len);
	printf("%zu\n", react(buf1, len));

	size_t min = len;
	for (char x = 'a'; x <= 'z'; ++x) {
		char buf2[50000];
		size_t len2 = 0;
		for (size_t i = 0; i < len; ++i) {
			if (tolower(buf[i]) == x) continue;
			buf2[len2++] = buf[i];
		}
		len2 = react(buf2, len2);
		if (len2 < min) min = len2;
	}
	printf("%zu\n", min);
}
