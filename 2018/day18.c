#include <stdio.h>
#include <stdlib.h>

typedef unsigned uint;

struct Map {
	char acres[50][50];
};

static uint adjacent(struct Map map, uint y, uint x, char type) {
	uint count = 0;
	if (y > 0) {
		count += (type == map.acres[y - 1][x]);
		if (x > 0)  count += (type == map.acres[y - 1][x - 1]);
		if (x < 49) count += (type == map.acres[y - 1][x + 1]);
	}
	if (x > 0)  count += (type == map.acres[y][x - 1]);
	if (x < 49) count += (type == map.acres[y][x + 1]);
	if (y < 49) {
		count += (type == map.acres[y + 1][x]);
		if (x > 0)  count += (type == map.acres[y + 1][x - 1]);
		if (x < 49) count += (type == map.acres[y + 1][x + 1]);
	}
	return count;
}

static struct Map step(struct Map prev) {
	struct Map next;
	for (uint y = 0; y < 50; ++y) {
		for (uint x = 0; x < 50; ++x) {
			if (prev.acres[y][x] == '.') {
				if (adjacent(prev, y, x, '|') > 2) {
					next.acres[y][x] = '|';
				} else {
					next.acres[y][x] = '.';
				}
			} else if (prev.acres[y][x] == '|') {
				if (adjacent(prev, y, x, '#') > 2) {
					next.acres[y][x] = '#';
				} else {
					next.acres[y][x] = '|';
				}
			} else {
				if (adjacent(prev, y, x, '#') && adjacent(prev, y, x, '|')) {
					next.acres[y][x] = '#';
				} else {
					next.acres[y][x] = '.';
				}
			}
		}
	}
	return next;
}

int main(void) {
	struct Map map;
	for (uint y = 0; y < 50; ++y) {
		scanf("%50c\n", map.acres[y]);
	}

	for (uint i = 0; i < 10; ++i) {
		map = step(map);
	}
	uint wood = 0, lumber = 0;
	for (uint y = 0; y < 50; ++y) {
		for (uint x = 0; x < 50; ++x) {
			if (map.acres[y][x] == '|') wood++;
			if (map.acres[y][x] == '#') lumber++;
		}
	}
	printf("%u\n", wood * lumber);
}
