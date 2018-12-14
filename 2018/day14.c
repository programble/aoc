#include <stdio.h>
#include <stdlib.h>

typedef unsigned uint;

struct Vec {
	size_t cap, len;
	uint *ptr;
};
static struct Vec new(size_t cap) {
	struct Vec vec = { cap };
	vec.ptr = malloc(sizeof(*vec.ptr) * cap);
	return vec;
}
static void push(struct Vec *vec, uint val) {
	if (vec->len == vec->cap) {
		vec->cap *= 2;
		vec->ptr = realloc(vec->ptr, sizeof(*vec->ptr) * vec->cap);
	}
	vec->ptr[vec->len++] = val;
}

int main() {
	uint count;
	scanf("%u", &count);

	struct Vec vec = new(256);
	push(&vec, 3);
	push(&vec, 7);

	size_t elf[2] = { 0, 1 };
	for (uint i = 0; i < count + 10; ++i) {
		uint sum = vec.ptr[elf[0]] + vec.ptr[elf[1]];
		if (sum / 10) push(&vec, sum / 10);
		push(&vec, sum % 10);
		elf[0] = (elf[0] + 1 + vec.ptr[elf[0]]) % vec.len;
		elf[1] = (elf[1] + 1 + vec.ptr[elf[1]]) % vec.len;
	}

	for (uint i = count; i < count + 10; ++i) {
		printf("%u", vec.ptr[i]);
	}
	printf("\n");
}
