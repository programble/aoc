#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

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

static uint value(struct Map map) {
	uint wood = 0, lumber = 0;
	for (uint y = 0; y < 50; ++y) {
		for (uint x = 0; x < 50; ++x) {
			if (map.acres[y][x] == '|') wood++;
			if (map.acres[y][x] == '#') lumber++;
		}
	}
	return wood * lumber;
}

enum { RingLen = 64 };
static_assert(!(RingLen & (RingLen - 1)), "power of two");

static struct {
	struct Map buf[RingLen];
	uint end;
} ring;

static struct Map *ringMap(uint i) {
	return &ring.buf[(ring.end + i) & (RingLen - 1)];
}

static void ringPush(struct Map map) {
	*ringMap(0) = map;
	ring.end++;
}

static uint ringFind(struct Map map) {
	uint i;
	for (i = 0; i < RingLen; ++i) {
		if (!memcmp(ringMap(i), &map, sizeof(map))) break;
	}
	return i;
}

int main(void) {
	struct Map map;
	for (uint y = 0; y < 50; ++y) {
		scanf("%50c\n", map.acres[y]);
	}

	for (uint i = 0; i < 10; ++i) {
		map = step(map);
	}
	printf("%u\n", value(map));

	uint i;
	for (i = 11; i < 1000000000; ++i) {
		map = step(map);
		if (ringFind(map) < RingLen) break;
		ringPush(map);
	}
	uint skip = ringFind(map);
	uint period = RingLen - skip;
	uint final = skip + (1000000000 - i) % period;
	printf("%u\n", value(*ringMap(final)));
}
