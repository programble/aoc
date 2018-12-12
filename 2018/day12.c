#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef unsigned uint;

enum { Zero = 128, Len = Zero * 2 };
struct State {
	char pots[Len];
};

struct Rule {
	char pots[5];
	char next;
};

int main() {
	struct State state;
	memset(state.pots, '.', Len);
	scanf("initial state: %s\n\n", &state.pots[Zero]);
	for (uint i = 0; i < Len; ++i) {
		if (!state.pots[i]) state.pots[i] = '.';
	}

	uint rulen = 0;
	struct Rule rules[32];
	while (!feof(stdin)) {
		scanf("%5c => %c\n", rules[rulen].pots, &rules[rulen].next);
		rulen++;
	}

	for (uint g = 0; g < 20; ++g) {
		struct State next;
		memset(next.pots, '.', Len);
		for (uint i = 2; i < Len - 2; ++i) {
			for (uint r = 0; r < rulen; ++r) {
				if (memcmp(&state.pots[i - 2], rules[r].pots, 5)) continue;
				next.pots[i] = rules[r].next;
				break;
			}
		}
		state = next;
	}

	int sum = 0;
	for (int i = 0; i < Len; ++i) {
		if (state.pots[i] == '#') sum += i - Zero;
	}
	printf("%d\n", sum);
}
