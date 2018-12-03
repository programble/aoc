#include <stdio.h>
#include <stdlib.h>

typedef unsigned char byte;
typedef unsigned uint;

enum { Len = 1000 };
static byte fabric[Len][Len];

int main() {
	while (!feof(stdin)) {
		uint d, x, y, w, h;
		scanf("#%u @ %u,%u: %ux%u\n", &d, &x, &y, &w, &h);
		for (uint i = 0; i < w; ++i) {
			for (uint j = 0; j < h; ++j) {
				fabric[x + i][y + j]++;
			}
		}
	}
	uint count = 0;
	for (uint x = 0; x < Len; ++x) {
		for (uint y = 0; y < Len; ++y) {
			if (fabric[x][y] > 1) count++;
		}
	}
	printf("%u\n", count);
}
