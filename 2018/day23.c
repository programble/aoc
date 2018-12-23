#include <stdio.h>
#include <stdlib.h>

typedef unsigned uint;

struct Point {
	int x, y, z;
};

static int distance(struct Point a, struct Point b) {
	int x = (a.x > b.x ? a.x - b.x : b.x - a.x);
	int y = (a.y > b.y ? a.y - b.y : b.y - a.y);
	int z = (a.z > b.z ? a.z - b.z : b.z - a.z);
	return x + y + z;
}

struct Bot {
	struct Point pos;
	int radius;
};

int main(void) {
	uint len = 0;
	struct Bot bots[1000];
	while (!feof(stdin)) {
		scanf(
			"pos=<%d,%d,%d>, r=%d\n",
			&bots[len].pos.x, &bots[len].pos.y, &bots[len].pos.z,
			&bots[len].radius
		);
		len++;
	}

	uint max, maxRadius = 0;
	for (uint i = 0; i < len; ++i) {
		if (bots[i].radius < maxRadius) continue;
		max = i;
		maxRadius = bots[i].radius;
	}

	uint inRange = 0;
	for (uint i = 0; i < len; ++i) {
		if (distance(bots[i].pos, bots[max].pos) > bots[max].radius) continue;
		inRange++;
	}
	printf("%u\n", inRange);
}
