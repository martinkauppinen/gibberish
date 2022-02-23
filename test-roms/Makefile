AS=rgbasm
LD=rgblink
FIX=rgbfix

ALL_ROMS = $(patsubst %.asm, %.gb, $(wildcard *.asm))

default: $(ALL_ROMS)

%.gb: %.o
	$(LD) -o $@ $^
	$(FIX) -v -p 0xFF $@

%.o: %.asm
	$(AS) --halt-without-nop -o $@ $^

.PHONY: clean
clean:
	rm -f *.gb *.o
