#include <stdio.h>
#include <stdlib.h>

typedef unsigned uint;

enum Type {
	Rocky,
	Wet,
	Narrow,
};

static uint depth;
static uint targetX, targetY;

static uint erosionLevel(uint x, uint y);

static uint geologicIndex(uint x, uint y) {
	if (x == 0 && y == 0) return 0;
	if (x == targetX && y == targetY) return 0;
	if (y == 0) return x * 16807;
	if (x == 0) return y * 48271;
	return erosionLevel(x - 1, y) * erosionLevel(x, y - 1);
}

static uint erosionLevels[1000][1000];

static uint erosionLevel(uint x, uint y) {
	if (!erosionLevels[x][y]) {
		erosionLevels[x][y] = (geologicIndex(x, y) + depth) % 20183;
	}
	return erosionLevels[x][y];
}

static enum Type type(uint x, uint y) {
	return erosionLevel(x, y) % 3;
}

int main(void) {
	scanf("depth: %u\n", &depth);
	scanf("target: %u,%u\n", &targetX, &targetY);

	uint riskLevel = 0;
	for (uint x = 0; x <= targetX; ++x) {
		for (uint y = 0; y <= targetY; ++y) {
			riskLevel += type(x, y);
		}
	}
	printf("%u\n", riskLevel);
}
