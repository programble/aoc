#include <limits.h>
#include <stdio.h>
#include <stdlib.h>

static int len;
static struct Point {
	int x, y;
} points[50];
static int area[50];

static int dist(struct Point a, struct Point b) {
	int x = a.x - b.x;
	int y = a.y - b.y;
	return (x < 0 ? -x : x) + (y < 0 ? -y : y);
}

static int close(struct Point p) {
	int c = 0;
	for (int i = 1; i < len; ++i) {
		if (dist(p, points[i]) < dist(p, points[c])) c = i;
	}
	for (int i = 0; i < len; ++i) {
		if (dist(p, points[i]) == dist(p, points[c]) && i != c) return -1;
	}
	return c;
}

int main() {
	while (!feof(stdin)) {
		scanf("%d, %d\n", &points[len].x, &points[len].y);
		len++;
	}
	int maxX = 0, maxY = 0;
	for (int i = 0; i < len; ++i) {
		if (points[i].x > maxX) maxX = points[i].x;
		if (points[i].y > maxY) maxY = points[i].y;
	}
	for (int x = 0; x <= maxX; ++x) {
		for (int y = 0; y <= maxY; ++y) {
			int c = close((struct Point) { x, y });
			if (c < 0) continue;
			if (area[c] < 0) continue;
			area[c]++;
			if (x == 0 || x == maxX || y == 0 || y == maxY) area[c] = -1;
		}
	}
	int max = 0;
	for (int i = 0; i < len; ++i) {
		if (area[i] > max) max = area[i];
	}
	printf("%d\n", max);

	int size = 0;
	for (int x = 0; x <= maxX; ++x) {
		for (int y = 0; y <= maxY; ++y) {
			int sum = 0;
			for (int i = 0; i < len; ++i) {
				sum += dist((struct Point) { x, y }, points[i]);
			}
			if (sum < 10000) size++;
		}
	}
	printf("%d\n", size);
}
