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

enum Tool {
	Neither,
	Torch,
	Gear,
};

static enum Tool validTool(enum Tool tool, enum Type a, enum Type b) {
	if (a == Rocky && b == Wet) return Gear;
	if (a == Wet && b == Rocky) return Gear;
	if (a == Rocky && b == Narrow) return Torch;
	if (a == Narrow && b == Rocky) return Torch;
	if (a == Wet && b == Narrow) return Neither;
	if (a == Narrow && b == Wet) return Neither;
	return tool;
}

static uint costs[1000][1000][3];

static struct Point {
	uint x, y;
	enum Tool tool;
} point(uint x, uint y, enum Tool tool) {
	return (struct Point) { x, y, tool };
}

static struct Q {
	struct Point point;
	uint cost;
	struct Q *next;
} *qHead;

static void qPush(struct Point point, uint cost) {
	struct Q *q = malloc(sizeof(*q));
	q->point = point;
	q->cost = cost;
	if (!qHead || q->cost < qHead->cost) {
		q->next = qHead;
		qHead = q;
		return;
	}
	for (struct Q *u = qHead; u; u = u->next) {
		if (u->next && q->cost > u->cost) continue;
		q->next = u->next;
		u->next = q;
		break;
	}
}

static struct Point qPop(void) {
	struct Q *q = qHead;
	struct Point p = q->point;
	qHead = q->next;
	free(q);
	return p;
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

	qPush(point(0, 0, Torch), 0);
	while (qHead) {
		struct Point p = qPop();
		if (p.x == targetX && p.y == targetY) {
			if (p.tool == Torch) break;
			struct Point next = point(p.x, p.y, Torch);
			uint cost = costs[p.x][p.y][p.tool] + 7;
			costs[next.x][next.y][next.tool] = cost;
			qPush(next, cost);
		}

		uint len = 0;
		struct Point nexts[4];
		nexts[len++] = point(p.x + 1, p.y, p.tool);
		nexts[len++] = point(p.x, p.y + 1, p.tool);
		if (p.x) nexts[len++] = point(p.x - 1, p.y, p.tool);
		if (p.y) nexts[len++] = point(p.x, p.y - 1, p.tool);

		for (uint i = 0; i < len; ++i) {
			struct Point next = nexts[i];
			uint cost = costs[p.x][p.y][p.tool] + 1;
			enum Type prevType = type(p.x, p.y);
			enum Type nextType = type(next.x, next.y);
			enum Tool tool = validTool(next.tool, prevType, nextType);
			if (next.tool != tool) {
				next.tool = tool;
				cost += 7;
			}

			uint oldCost = costs[next.x][next.y][next.tool];
			if (next.x == 0 && next.y == 0) continue;
			if (oldCost && cost >= oldCost) continue;
			costs[next.x][next.y][next.tool] = cost;
			qPush(next, cost);
		}
	}
	printf("%u\n", costs[targetX][targetY][Torch]);
}
