#include <stdio.h>
#include <stdlib.h>

typedef unsigned char byte;
typedef unsigned uint;

enum { Len = 1000 };
static byte fabric[Len][Len];

static struct {
	uint x, y, w, h;
} claims[2 * Len];

int main() {
	uint n = 0;
	while (!feof(stdin)) {
		scanf(
			"#%*u @ %u,%u: %ux%u\n",
			&claims[n].x, &claims[n].y, &claims[n].w, &claims[n].h
		);
		for (uint x = 0; x < claims[n].w; ++x) {
			for (uint y = 0; y < claims[n].h; ++y) {
				fabric[claims[n].x + x][claims[n].y + y]++;
			}
		}
		n++;
	}
	uint count = 0;
	for (uint x = 0; x < Len; ++x) {
		for (uint y = 0; y < Len; ++y) {
			if (fabric[x][y] > 1) count++;
		}
	}
	printf("%u\n", count);
	uint i;
	for (i = 0; i < n; ++i) {
		uint overlap = 0;
		for (uint x = 0; x < claims[i].w; ++x) {
			for (uint y = 0; y < claims[i].h; ++y) {
				if (fabric[claims[i].x + x][claims[i].y + y] > 1) overlap = 1;
			}
		}
		if (!overlap) break;
	}
	printf("%u\n", 1 + i);
}
