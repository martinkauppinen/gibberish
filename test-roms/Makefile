AS=rgbasm
LD=rgblink

ALL_ROMS = $(patsubst %.asm, %.gb, $(wildcard *.asm))

default: $(ALL_ROMS)

%.gb: %.o
	$(LD) -o $@ $^

%.o: %.asm
	$(AS) -o $@ $^

.PHONY: clean
clean:
	rm -f *.gb *.o
