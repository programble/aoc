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
		uint op, a, b, c;
	} prog[50];
	while (!feof(stdin)) {
		char code[4];
		scanf(
			"%4c %u %u %u\n",
			code, &prog[len].a, &prog[len].b, &prog[len].c
		);
		for (uint i = 0; i < 16; ++i) {
			if (strncmp(code, Ops[i].name, 4)) continue;
			prog[len].op = i;
			break;
		}
		len++;
	}

	while (cpu.r[cpu.ip] < len) {
		struct Ins ins = prog[cpu.r[cpu.ip]];
		cpu = Ops[ins.op].fn(cpu, ins.a, ins.b, ins.c);
		cpu.r[cpu.ip]++;
	}
	printf("%u\n", cpu.r[0]);

	/*
	00:	addi	ip	16	ip	goto 17
	01:	seti	1	_	r5	r5 = 1
	02:	seti	1	_	r3	r3 = 1
	03:	mulr	r5	r3	r2	r2 = r5 * r3
	04:	eqrr	r2	r4	r2	r2 = (r2 == r4)
	05:	addr	r2	ip	ip	if r2 goto 7 else goto 6
	06:	addi	ip	1	ip	goto 8
	07:	addr	r5	r0	r0	r0 += r5
	08:	addi	r3	1	r3	r3 += 1
	09:	gtrr	r3	r4	r2	r2 = (r3 > r4)
	10:	addr	ip	r2	ip	if r2 goto 12 else goto 11
	11:	seti	2	_	ip	goto 3
	12:	addi	r5	1	r5	r5 += 1
	13:	gtrr	r5	r4	r2	r2 = (r5 > r4)
	14:	addr	r2	ip	ip	if r2 goto 16 else goto 15
	15:	seti	1	_	ip	goto 2
	16:	mulr	ip	ip	ip	halt
	17:	addi	r4	2	r4	r4 += 2
	18:	mulr	r4	r4	r4	r4 *= r4
	19:	mulr	ip	r4	r4	r4 *= 19
	20:	muli	r4	11	r4	r4 *= 11
	21:	addi	r2	5	r2	r2 += 5
	22:	mulr	r2	ip	r2	r2 *= 22
	23:	addi	r2	12	r2	r2 += 12
	24:	addr	r4	r2	r4	r4 += r2
	25:	addr	ip	r0	ip	if r0 goto 27 else goto 26
	26:	seti	0	_	ip	goto 1
	27:	setr	ip	_	r2	r2 = 27
	28:	mulr	r2	ip	r2	r2 *= 28
	29:	addr	ip	r2	r2	r2 += 29
	30:	mulr	ip	r2	r2	r2 *= 30
	31:	muli	r2	14	r2	r2 *= 14
	32:	mulr	r2	ip	r2	r2 *= 32
	33:	addr	r4	r2	r4	r4 += r2
	34:	seti	0	_	r0	r0 = 0
	35:	seti	0	_	ip	goto 1
	*/

#if 0
	uint sum = 0;
	uint big = 10551358;
	for (uint a = 1; a <= big; a++) {
		for (uint b = 1; b <= big; b++) {
			if (a * b == big) sum += a;
		}
	}
	printf("%u\n", r0);
#endif

	uint big = 10551358;
	uint sum = 1 + big;
	for (uint f = 2; big > 1; ++f) {
		if (big % f) continue;
		sum += f;
		big /= f;
	}
	printf("%u\n", sum);
}
