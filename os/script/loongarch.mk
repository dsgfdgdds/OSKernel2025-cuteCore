TARGET := loongarch64-unknown-none
MODE := release
KERNEL_ELF := target/loongarch64-unknown-none/$(MODE)/os
KERNEL_BIN := $(KERNEL_ELF).bin
KERNEL_QEMU := ../bin/kernel-laqemu

BOARD := laqemu
SBI ?=
BOOTLOADER := ../bootloader/u-boot-with-spl.bin

OBJCOPY := loongarch64-linux-gnu-objcopy
OBJDUMP := loongarch64-linux-gnu-objdump
READELF := loongarch64-linux-gnu-readelf

build: kernel mv

mv:
#	@cp $(KERNEL_BIN) $(KERNEL_QEMU)
	@cp $(KERNEL_ELF) $(KERNEL_QEMU)

$(KERNEL_BIN): kernel
	@$(OBJCOPY) ${KERNEL_ELF} --strip-all -O binary $@


kernel: pre
	@echo Platform: $(BOARD), SBI: $(SBI)
	@cp src/hal/arch/loongarch/linker-$(BOARD).ld src/hal/arch/loongarch/linker.ld
	@LOG=${LOG} cargo build --${MODE} --target $(TARGET) --features "board_$(BOARD)"

pre:
	@rm .cargo/config.toml || true
	@cp cargo/la-config.toml .cargo/config.toml

clean:
	@rm -f ../kernel-qemu
	@rm -f src/hal/arch/loongarch/linker.ld


run:
	qemu-system-loongarch64 \
	-kernel $(KERNEL_QEMU) \
	-m 1G \
	-nographic \
	-smp 1 \
	-no-reboot \
	-rtc base=utc \
	-snapshot



#run:
#	qemu-system-loongarch64 \
#	-M ls2k \
#	-serial stdio \
#	-serial vc	\
# 	-drive if=pflash,file=/tmp/qemu/2k1000/u-boot-with-spl.bin \
#	-m 1024 \
# 	-device usb-kbd,bus=usb-bus.0 \
# 	-device usb-tablet,bus=usb-bus.0 \
# 	-device usb-storage,drive=udisk \
#    -drive if=none,id=udisk,file=/tmp/disk \
# 	-net nic \
#	-net user,net=10.0.2.0/24,tftp=/srv/tftp \
# 	-vnc :0 -D /tmp/qemu.log -s -hda /tmp/qemu/2k1000/2kfs.img