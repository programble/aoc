#include <stdio.h>
#include <stdlib.h>

typedef unsigned uint;

static uint sum;

static void meta(void) {
	uint meta;
	scanf("%u", &meta);
	sum += meta;
}

static void node(void) {
	uint nchild, nmeta;
	scanf("%u %u", &nchild, &nmeta);
	for (uint i = 0; i < nchild; ++i) {
		node();
	}
	for (uint i = 0; i < nmeta; ++i) {
		meta();
	}
}

int main() {
	node();
	printf("%u\n", sum);
}
