#include <stdio.h>
#include <stdlib.h>

typedef unsigned uint;

int main() {
	uint steps = 0;
	uint deps[26] = {0};
	while (!feof(stdin)) {
		char dep, step;
		scanf(
			"Step %c must be finished before step %c can begin.\n",
			&dep, &step
		);
		deps[step - 'A'] |= 1 << (dep - 'A');
	}
	while (steps != (1 << 26) - 1) {
		for (uint i = 0; i < 26; ++i) {
			if (steps & (1 << i)) continue;
			if ((deps[i] & steps) != deps[i]) continue;
			printf("%c", 'A' + i);
			steps |= (1 << i);
			break;
		}
	}
	printf("\n");

	uint time = 0;
	uint prog = 0;
	uint done = 0;
	struct {
		uint step;
		uint time;
	} work[5] = {0};
	while (done != (1 << 26) - 1) {
		for (uint i = 0; i < 26; ++i) {
			if (done & (1 << i)) continue;
			if (prog & (1 << i)) continue;
			if ((deps[i] & done) != deps[i]) continue;
			for (uint w = 0; w < 5; ++w) {
				if (work[w].time) continue;
				work[w].step = (1 << i);
				work[w].time = 61 + i;
				prog |= work[w].step;
				break;
			}
		}
		for (uint w = 0; w < 5; ++w) {
			if (work[w].time && --work[w].time) continue;
			done |= work[w].step;
			prog &= ~work[w].step;
		}
		time++;
	}
	printf("%u\n", time);
}
