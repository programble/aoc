#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef unsigned uint;

#define T(x, y) ((uint)(x) << 8 | (uint)(y))

struct Cell {
	char track;
	char cart;
	char turn;
};

struct Map {
	struct Cell cells[200][200];
};

int main() {
	struct Map map;
	memset(&map, 0, sizeof(map));
	uint y = 0, x = 0;
	char ch;
	while (EOF != (ch = getchar())) {
		if (ch == '\n') {
			y++;
			x = 0;
		} else if (ch == '^' || ch == 'v') {
			map.cells[y][x++] = (struct Cell) { '|', ch, '[' };
		} else if (ch == '<' || ch == '>') {
			map.cells[y][x++] = (struct Cell) { '-', ch, '[' };
		} else {
			map.cells[y][x++].track = ch;
		}
	}

	for (;;) {
		struct Map next = map;
		for (y = 0; y < 200; ++y) {
			for (x = 0; x < 200; ++x) {
				if (!map.cells[y][x].cart) continue;
				next.cells[y][x].cart = 0;
				uint ny = y, nx = x;
				switch (map.cells[y][x].cart) {
					break; case '^': ny--;
					break; case 'v': ny++;
					break; case '<': nx--;
					break; case '>': nx++;
				}
				struct Cell *cell = &next.cells[ny][nx];
				if (cell->cart) {
					printf("%u,%u\n", nx, ny);
					exit(EXIT_SUCCESS);
				}
				cell->cart = map.cells[y][x].cart;
				cell->turn = map.cells[y][x].turn;
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
		map = next;
	}
}
