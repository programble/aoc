#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef unsigned uint;

struct CPU {
	uint ip;
	uint r[6];
};

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
	struct CPU cpu = { .r = { 0, 0, 0, 0, 0, 0 } };
	scanf("#ip %u\n", &cpu.ip);

	uint len = 0;
	struct Ins {
		char op[4];
		uint a, b, c;
	} prog[50];
	while (!feof(stdin)) {
		scanf(
			"%4c %u %u %u\n",
			prog[len].op, &prog[len].a, &prog[len].b, &prog[len].c
		);
		len++;
	}

	while (cpu.r[cpu.ip] < len) {
		struct Ins ins = prog[cpu.r[cpu.ip]];
		for (uint i = 0; i < 16; ++i) {
			if (strncmp(ins.op, Ops[i].name, 4)) continue;
			cpu = Ops[i].fn(cpu, ins.a, ins.b, ins.c);
			cpu.r[cpu.ip]++;
			break;
		}
	}
	printf("%u\n", cpu.r[0]);
}
