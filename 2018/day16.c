#include <stdio.h>
#include <stdlib.h>

typedef unsigned uint;
typedef unsigned short Set;

struct CPU {
	uint r[4];
};

static int cpuEq(struct CPU a, struct CPU b) {
	return a.r[0] == b.r[0]
		&& a.r[1] == b.r[1]
		&& a.r[2] == b.r[2]
		&& a.r[3] == b.r[3];
}

typedef struct CPU Op(struct CPU, uint, uint, uint);

static struct CPU addr(struct CPU cpu, uint a, uint b, uint c) {
	cpu.r[c] = cpu.r[a] + cpu.r[b]; return cpu;
}
static struct CPU addi(struct CPU cpu, uint a, uint b, uint c) {
	cpu.r[c] = cpu.r[a] + b; return cpu;
}
static struct CPU mulr(struct CPU cpu, uint a, uint b, uint c) {
	cpu.r[c] = cpu.r[a] * cpu.r[b]; return cpu;
}
static struct CPU muli(struct CPU cpu, uint a, uint b, uint c) {
	cpu.r[c] = cpu.r[a] * b; return cpu;
}
static struct CPU banr(struct CPU cpu, uint a, uint b, uint c) {
	cpu.r[c] = cpu.r[a] & cpu.r[b]; return cpu;
}
static struct CPU bani(struct CPU cpu, uint a, uint b, uint c) {
	cpu.r[c] = cpu.r[a] & b; return cpu;
}
static struct CPU borr(struct CPU cpu, uint a, uint b, uint c) {
	cpu.r[c] = cpu.r[a] | cpu.r[b]; return cpu;
}
static struct CPU bori(struct CPU cpu, uint a, uint b, uint c) {
	cpu.r[c] = cpu.r[a] | b; return cpu;
}
static struct CPU setr(struct CPU cpu, uint a, uint b, uint c) {
	cpu.r[c] = cpu.r[a]; return cpu;
}
static struct CPU seti(struct CPU cpu, uint a, uint b, uint c) {
	cpu.r[c] = a; return cpu;
}
static struct CPU gtir(struct CPU cpu, uint a, uint b, uint c) {
	cpu.r[c] = a > cpu.r[b]; return cpu;
}
static struct CPU gtri(struct CPU cpu, uint a, uint b, uint c) {
	cpu.r[c] = cpu.r[a] > b; return cpu;
}
static struct CPU gtrr(struct CPU cpu, uint a, uint b, uint c) {
	cpu.r[c] = cpu.r[a] > cpu.r[b]; return cpu;
}
static struct CPU eqir(struct CPU cpu, uint a, uint b, uint c) {
	cpu.r[c] = a == cpu.r[b]; return cpu;
}
static struct CPU eqri(struct CPU cpu, uint a, uint b, uint c) {
	cpu.r[c] = cpu.r[a] == b; return cpu;
}
static struct CPU eqrr(struct CPU cpu, uint a, uint b, uint c) {
	cpu.r[c] = cpu.r[a] == cpu.r[b]; return cpu;
}

static const struct {
	const char *name;
	Op *fn;
} Ops[16] = {
	{ "addr", addr },
	{ "addi", addi },
	{ "mulr", mulr },
	{ "muli", muli },
	{ "banr", banr },
	{ "bani", bani },
	{ "borr", borr },
	{ "bori", bori },
	{ "setr", setr },
	{ "seti", seti },
	{ "gtir", gtir },
	{ "gtri", gtri },
	{ "gtrr", gtrr },
	{ "eqir", eqir },
	{ "eqri", eqri },
	{ "eqrr", eqrr },
};

int main(void) {
	Set codes[16];
	for (uint i = 0; i < 16; ++i) {
		codes[i] = 0xFFFF;
	}

	uint samples = 0;
	for (;;) {
		struct CPU before, after;
		uint code, a, b, c;
		int match = scanf(
			"Before: [%u, %u, %u, %u]\n"
			"%u %u %u %u\n"
			"After: [%u, %u, %u, %u]\n",
			&before.r[0], &before.r[1], &before.r[2], &before.r[3],
			&code, &a, &b, &c,
			&after.r[0], &after.r[1], &after.r[2], &after.r[3]
		);
		if (match < 12) break;

		uint count = 0;
		for (uint i = 0; i < 16; ++i) {
			struct CPU result = Ops[i].fn(before, a, b, c);
			if (cpuEq(result, after)) {
				count++;
			} else {
				codes[code] &= ~(1 << i);
			}
		}
		if (count >= 3) samples++;
	}
	printf("%u\n", samples);

	Set known = 0;
	while (known != 0xFFFF) {
		for (uint i = 0; i < 16; ++i) {
			if (__builtin_popcount(codes[i]) == 1) {
				known |= codes[i];
			} else {
				codes[i] &= ~known;
			}
		}
	}

	struct CPU cpu = { .r = { 0, 0, 0, 0 } };
	while (!feof(stdin)) {
		uint code, a, b, c;
		scanf("%u %u %u %u\n", &code, &a, &b, &c);
		cpu = Ops[__builtin_ctz(codes[code])].fn(cpu, a, b, c);
	}
	printf("%u\n", cpu.r[0]);
}
