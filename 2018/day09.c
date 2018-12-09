#include <stdio.h>
#include <stdlib.h>

typedef unsigned uint;

static struct Marble {
	uint num;
	struct Marble *prev;
	struct Marble *next;
} *current;

static struct Marble *ins(struct Marble *prev, uint num) {
	struct Marble *next = malloc(sizeof(*next));
	next->num = num;
	next->prev = prev;
	next->next = prev->next;
	next->prev->next = next;
	next->next->prev = next;
	return next;
}

static struct Marble *rem(struct Marble *marb) {
	struct Marble *next = marb->next;
	marb->prev->next = marb->next;
	marb->next->prev = marb->prev;
	free(marb);
	return next;
}

int main() {
	uint players, marbles;
	scanf("%u players; last marble is worth %u points", &players, &marbles);
	marbles += 1;

	uint scores[players];
	for (uint i = 0; i < players; ++i) {
		scores[i] = 0;
	}

	current = malloc(sizeof(*current));
	current->num = 0;
	current->prev = current;
	current->next = current;

	uint marble = 1;
	while (marble < marbles) {
		for (uint i = 0; i < players; ++i) {
			if (marble % 23) {
				current = ins(current->next, marble);
			} else {
				scores[i] += marble;
				struct Marble *marb = current;
				for (uint j = 0; j < 7; ++j) {
					marb = marb->prev;
				}
				scores[i] += marb->num;
				current = rem(marb);
			}
			if (++marble == marbles) break;
		}
	}

	uint max = 0;
	for (uint i = 0; i < players; ++i) {
		if (scores[i] > max) max = scores[i];
	}
	printf("%u\n", max);
}
