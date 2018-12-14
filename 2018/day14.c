#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct Vec {
	size_t cap, len;
	char *ptr;
};
static struct Vec new(size_t cap) {
	struct Vec vec = { cap };
	vec.ptr = malloc(sizeof(*vec.ptr) * cap);
	return vec;
}
static void push(struct Vec *vec, char val) {
	if (vec->len == vec->cap) {
		vec->cap *= 2;
		vec->ptr = realloc(vec->ptr, sizeof(*vec->ptr) * vec->cap);
	}
	vec->ptr[vec->len++] = val;
}

int main() {
	size_t count;
	scanf("%zu", &count);
	char *scores;
	asprintf(&scores, "%zu", count);

	struct Vec vec = new(256);
	push(&vec, '3');
	push(&vec, '7');

	size_t elf[2] = { 0, 1 };
	for (size_t i = 0; 1; ++i) {
		if (i == count + 10) {
			for (size_t j = count; j < count + 10; ++j) {
				printf("%c", vec.ptr[j]);
			}
			printf("\n");
		}
		char sum = (vec.ptr[elf[0]] - '0') + (vec.ptr[elf[1]] - '0');
		if (sum / 10) push(&vec, '0' + sum / 10);
		push(&vec, '0' + sum % 10);
		elf[0] = (elf[0] + 1 + vec.ptr[elf[0]] - '0') % vec.len;
		elf[1] = (elf[1] + 1 + vec.ptr[elf[1]] - '0') % vec.len;
		size_t len = strlen(scores);
		if (vec.len < len) continue;
		char *match = strnstr(&vec.ptr[vec.len - len - 1], scores, len + 1);
		if (!match) continue;
		printf("%zu\n", match - vec.ptr);
		break;
	}
}
