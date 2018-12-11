#include <limits.h>
#include <stdio.h>
#include <stdlib.h>

typedef unsigned uint;

int main() {
	uint serial;
	scanf("%u", &serial);

	int cells[300][300];
	for (uint y = 0; y < 300; ++y) {
		for (uint x = 0; x < 300; ++x) {
			uint id = (x + 1) + 10;
			cells[y][x] = id * (y + 1);
			cells[y][x] += serial;
			cells[y][x] *= id;
			cells[y][x] /= 100;
			cells[y][x] %= 10;
			cells[y][x] -= 5;
		}
	}

	int max = INT_MIN;
	uint maxY = 0, maxX = 0;
	for (uint y = 0; y < 297; ++y) {
		for (uint x = 0; x < 297; ++x) {
			int power = cells[y][x] + cells[y][x + 1] + cells[y][x + 2]
				+ cells[y + 1][x] + cells[y + 1][x + 1] + cells[y + 1][x + 2]
				+ cells[y + 2][x] + cells[y + 2][x + 1] + cells[y + 2][x + 2];
			if (power < max) continue;
			max = power;
			maxY = y;
			maxX = x;
		}
	}
	printf("%u,%u\n", maxX + 1, maxY + 1);
}
