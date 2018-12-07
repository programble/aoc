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
}
