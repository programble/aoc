#include <stdio.h>
#include <stdlib.h>

typedef unsigned uint;

static uint sum;

static uint meta(void) {
	uint meta;
	scanf("%u", &meta);
	sum += meta;
	return meta;
}

static uint node(void) {
	uint nchild, nmeta;
	scanf("%u %u", &nchild, &nmeta);
	uint children[nchild];
	uint metas[nmeta];
	for (uint i = 0; i < nchild; ++i) {
		children[i] = node();
	}
	for (uint i = 0; i < nmeta; ++i) {
		metas[i] = meta();
	}
	uint val = 0;
	if (nchild) {
		for (uint i = 0; i < nmeta; ++i) {
			if (!metas[i] || metas[i] > nchild) continue;
			val += children[metas[i] - 1];
		}
	} else {
		for (uint i = 0; i < nmeta; ++i) {
			val += metas[i];
		}
	}
	return val;
}

int main() {
	uint val = node();
	printf("%u\n", sum);
	printf("%u\n", val);
}
