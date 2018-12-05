#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef unsigned uint;

enum { Len = 4096 };
uint guards[Len][60];

int main() {
	uint id, sleep;
	while (!feof(stdin)) {
		uint year, month, day, hour, minute;
		char word[256];
		scanf(
			"[%u-%u-%u %u:%u] %255s #%u",
			&year, &month, &day, &hour, &minute, word, &id
		);
		scanf("%*[^\n]\n");

		if (!strcmp(word, "falls")) sleep = minute;
		if (!strcmp(word, "wakes")) {
			for (uint m = sleep; m < minute; ++m) {
				guards[id][m]++;
			}
		}
	}
	uint maxID = 0, maxSum = 0;
	for (uint i = 0; i < Len; ++i) {
		uint sum = 0;
		for (uint m = 0; m < 60; ++m) {
			sum += guards[i][m];
		}
		if (sum < maxSum) continue;
		maxSum = sum;
		maxID = i;
	}
	uint maxMin = 0, maxSleep = 0;
	for (uint m = 0; m < 60; ++m) {
		if (guards[maxID][m] < maxSleep) continue;
		maxSleep = guards[maxID][m];
		maxMin = m;
	}
	printf("%u\n", maxID * maxMin);
	maxID = maxMin = maxSleep = 0;
	for (uint i = 0; i < Len; ++i) {
		for (uint m = 0; m < 60; ++m) {
			if (guards[i][m] < maxSleep) continue;
			maxSleep = guards[i][m];
			maxMin = m;
			maxID = i;
		}
	}
	printf("%u\n", maxID * maxMin);
}
