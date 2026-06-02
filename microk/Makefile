.PHONY: all clean

all: kernel.x86_64

CC = x86_64-elf-gcc
LD = x86_64-elf-ld

CFLAGS = -Wall -Wextra -Werror --pedantic -O2 -ffreestanding -mno-red-zone -mcmodel=kernel -nostdlib

C_SRC = $(wildcard *.c)
ASM_SRC = $(wildcard *.S)

OBJ = $(patsubst %.c, %.o, $(C_SRC)) \
      $(patsubst %.S, %.o, $(ASM_SRC)) \

%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

%.o: %.S
	$(CC) $(CFLAGS) -c $< -o $@

kernel.x86_64: $(OBJ)
	$(LD) -T linker-x86_64.ld -o kernel.x86_64 $(OBJ)


