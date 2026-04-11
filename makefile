# ====== CONFIG ======
ASM=nasm
CC=gcc
LD=ld

ASMFLAGS=-f elf32
CFLAGS=-m32 -c
LDFLAGS=-m elf_i386 -T src/link.ld

KERNEL=build/kernel
ISO=build/jos.iso

# ====== FILES ======
ASM_SRC=src/kernel.asm
C_SRC=src/kernel.c

ASM_OBJ=build/kasm.o
C_OBJ=build/kc.o

ISO_DIR=iso

# ====== DEFAULT ======
all: iso

# ====== ASM ======
$(ASM_OBJ): $(ASM_SRC)
	$(ASM) $(ASMFLAGS) $(ASM_SRC) -o $(ASM_OBJ)

# ====== C ======
$(C_OBJ): $(C_SRC)
	$(CC) $(CFLAGS) $(C_SRC) -o $(C_OBJ)

# ====== LINK ======
$(KERNEL): $(ASM_OBJ) $(C_OBJ)
	$(LD) $(LDFLAGS) -o $(KERNEL) $(ASM_OBJ) $(C_OBJ)

# ====== ISO STRUCTURE ======
iso: $(KERNEL)
	rm -rf $(ISO_DIR)
	mkdir -p $(ISO_DIR)/boot/grub

	cp $(KERNEL) $(ISO_DIR)/boot/kernel

	@echo "set timeout=0" > $(ISO_DIR)/boot/grub/grub.cfg
	@echo "set default=0" >> $(ISO_DIR)/boot/grub/grub.cfg
	@echo "" >> $(ISO_DIR)/boot/grub/grub.cfg
	@echo "menuentry \"My Kernel\" {" >> $(ISO_DIR)/boot/grub/grub.cfg
	@echo "    multiboot /boot/kernel" >> $(ISO_DIR)/boot/grub/grub.cfg
	@echo "    boot" >> $(ISO_DIR)/boot/grub/grub.cfg
	@echo "}" >> $(ISO_DIR)/boot/grub/grub.cfg

	grub-mkrescue -o $(ISO) $(ISO_DIR)

# ====== CLEAN ======
clean:
	rm -f $(ASM_OBJ) $(C_OBJ) $(KERNEL)
	rm -rf $(ISO_DIR)

fclean: clean
	rm -f $(ISO)

re: fclean all