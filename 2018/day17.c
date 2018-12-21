#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef unsigned uint;

static char map[2048][2048];

static void draw(void) {
	for (uint y = 0; y < 14; ++y) {
		for (uint x = 494; x < 508; ++x) {
			printf("%c", map[y][x]);
		}
		printf("\n");
	}
	printf("\n");
}

int main(void) {
	memset(map, '.', sizeof(map));
	uint minY = 2048, maxY = 0;
	while (!feof(stdin)) {
		char a, b;
		uint fixed, start, end;
		scanf("%c=%u, %c=%u..%u\n", &a, &fixed, &b, &start, &end);
		for (uint i = start; i <= end; ++i) {
			if (a == 'y') {
				map[fixed][i] = '#';
			} else {
				map[i][fixed] = '#';
			}
		}
		if (a == 'y') {
			if (fixed < minY) minY = fixed;
			if (fixed > maxY) maxY = fixed;
		} else {
			if (start < minY) minY = start;
			if (end > maxY) maxY = end;
		}
	}
	map[0][500] = '|';

	bool hot;
	do {
		hot = false;
		for (uint y = 0; y < 2047; ++y) {
			for (uint x = 0; x < 2048; ++x) {
				char *self = &map[y][x];
				char *below = &map[y + 1][x];
				char *left = &map[y][x - 1];
				char *right = &map[y][x + 1];
				if (*self != '|' && *self != '*') continue;
				if (*below == '.') {
					*below = '|';
					hot = true;
				}
				if (*below != '#' && *below != '~') continue;
				if (*left == '.') {
					*left = '|';
					hot = true;
				}
				if (*right == '.') {
					*right = '|';
					hot = true;
				}
				if (*self == '|' && (*left == '#' || *left == '*')) {
					*self = '*';
					hot = true;
				}
				if (*self == '*' && (*right == '#' || *right == '~')) {
					*self = '~';
					hot = true;
				}
			}
		}
	} while (hot);
	draw();

	uint count = 0;
	uint settled = 0;
	for (uint y = minY; y <= maxY; ++y) {
		for (uint x = 0; x < 2048; ++x) {
			if (map[y][x] != '.' && map[y][x] != '#') count++;
			if (map[y][x] == '~') settled++;
		}
	}
	printf("%u\n", count);
	printf("%u\n", settled);
}
