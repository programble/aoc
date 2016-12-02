BINS = day01

all: $(BINS)

%: %.o
	ld -o $@ $<

%.o: %.asm sys.asm lib.asm
	nasm -f elf64 -o $@ $<

clean:
	rm -f $(BINS)

.PHONY: clean
