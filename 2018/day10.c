#include <limits.h>
#include <stdio.h>
#include <stdlib.h>

struct Point {
	int x, y;
	int dx, dy;
};

int main() {
	size_t len = 0;
	struct Point points[500];
	while (!feof(stdin)) {
		scanf(
			"position=<%d, %d> velocity=<%d, %d>\n",
			&points[len].x, &points[len].y, &points[len].dx, &points[len].dy
		);
		len++;
	}
	int minX, minY, maxX, maxY;
	do {
		minX = minY = INT_MAX;
		maxX = maxY = INT_MIN;
		for (size_t i = 0; i < len; ++i) {
			points[i].x += points[i].dx;
			points[i].y += points[i].dy;
			if (points[i].x < minX) minX = points[i].x;
			if (points[i].y < minY) minY = points[i].y;
			if (points[i].x > maxX) maxX = points[i].x;
			if (points[i].y > maxY) maxY = points[i].y;
		}
	} while (maxY - minY > 9);
	for (int y = minY; y <= maxY; ++y) {
		for (int x = minX; x <= maxX; ++x) {
			size_t i;
			for (i = 0; i < len; ++i) {
				if (points[i].x == x && points[i].y == y) break;
			}
			printf("%c", (i == len ? '.' : '#'));
		}
		printf("\n");
	}
}
