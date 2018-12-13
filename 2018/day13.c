#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef unsigned uint;

#define T(x, y) ((uint)(x) << 8 | (uint)(y))

struct Cell {
	char track;
	char cart;
	char turn;
	char done;
};

int main() {
	struct Cell map[200][200];
	memset(&map, 0, sizeof(map));
	uint y = 0, x = 0;
	char ch;
	while (EOF != (ch = getchar())) {
		if (ch == '\n') {
			y++;
			x = 0;
		} else if (ch == '^' || ch == 'v') {
			map[y][x++] = (struct Cell) { '|', ch, '[', 0 };
		} else if (ch == '<' || ch == '>') {
			map[y][x++] = (struct Cell) { '-', ch, '[', 0 };
		} else {
			map[y][x++].track = ch;
		}
	}

	for (;;) {
		for (y = 0; y < 200; ++y) {
			for (x = 0; x < 200; ++x) {
				if (!map[y][x].cart || map[y][x].done) continue;
				uint ny = y, nx = x;
				switch (map[y][x].cart) {
					break; case '^': ny--;
					break; case 'v': ny++;
					break; case '<': nx--;
					break; case '>': nx++;
				}
				if (map[ny][nx].cart) {
					printf("%u,%u\n", nx, ny);
					map[y][x].cart = 0;
					map[ny][nx].cart = 0;
					continue;
				}
				map[ny][nx].cart = map[y][x].cart;
				map[ny][nx].turn = map[y][x].turn;
				map[ny][nx].done = 1;
				map[y][x].cart = 0;
				struct Cell *cell = &map[ny][nx];
				switch (T(cell->cart, cell->track)) {
					break; case T('^', '/'):  cell->cart = '>';
					break; case T('^', '\\'): cell->cart = '<';
					break; case T('v', '/'):  cell->cart = '<';
					break; case T('v', '\\'): cell->cart = '>';
					break; case T('<', '/'):  cell->cart = 'v';
					break; case T('<', '\\'): cell->cart = '^';
					break; case T('>', '/'):  cell->cart = '^';
					break; case T('>', '\\'): cell->cart = 'v';
					break; case T('^', '+'): switch (cell->turn) {
						break; case '[': cell->turn = '|'; cell->cart = '<';
						break; case '|': cell->turn = ']';
						break; case ']': cell->turn = '['; cell->cart = '>';
					}
					break; case T('v', '+'): switch (cell->turn) {
						break; case '[': cell->turn = '|'; cell->cart = '>';
						break; case '|': cell->turn = ']';
						break; case ']': cell->turn = '['; cell->cart = '<';
					}
					break; case T('<', '+'): switch (cell->turn) {
						break; case '[': cell->turn = '|'; cell->cart = 'v';
						break; case '|': cell->turn = ']';
						break; case ']': cell->turn = '['; cell->cart = '^';
					}
					break; case T('>', '+'): switch (cell->turn) {
						break; case '[': cell->turn = '|'; cell->cart = '^';
						break; case '|': cell->turn = ']';
						break; case ']': cell->turn = '['; cell->cart = 'v';
					}
				}
			}
		}
		uint carts = 0;
		for (y = 0; y < 200; ++y) {
			for (x = 0; x < 200; ++x) {
				if (map[y][x].cart) carts++;
				map[y][x].done = 0;
			}
		}
		if (carts == 1) break;
	}

	for (y = 0; y < 200; ++y) {
		for (x = 0; x < 200; ++x) {
			if (map[y][x].cart) printf("%u,%u\n", x, y);
		}
	}
}
