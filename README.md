# 👋🏼 Hello -x- QEMU

`Hello, World!` hardware emulated using `cross-rs` and `qemu`.

What is a Target Triple?

A target triple specifies the platform for which you’re compiling your Rust code. It generally consists of:

	1.	Architecture: CPU architecture (e.g., x86_64, arm, aarch64).
	2.	Vendor: The vendor of the system (e.g., unknown, apple, pc).
	3.	Operating System: The OS (e.g., linux, windows, macos).
	4.	Environment: Optional; specifies the ABI or other environmental details (e.g., gnu, msvc, musl).


----

	1.	Samsung Galaxy S3
	•	Architecture: ARMv7
	•	Target Triple: armv7-linux-androideabi
	2.	Raspberry Pi 3
	•	Architecture: ARMv8 (64-bit)
	•	Target Triple:
	•	For 32-bit: armv7-unknown-linux-gnueabihf
	•	For 64-bit: aarch64-unknown-linux-gnu
	3.	Raspberry Pi 4
	•	Architecture: ARMv8 (64-bit)
	•	Target Triple:
	•	For 32-bit: armv7-unknown-linux-gnueabihf
	•	For 64-bit: aarch64-unknown-linux-gnu
	4.	Android Devices (General)
	•	32-bit ARM: armv7-linux-androideabi
	•	64-bit ARM: aarch64-linux-android
	•	32-bit x86: i686-linux-android
	•	64-bit x86: x86_64-linux-android
	5.	iOS Devices (iPhone, iPad)
	•	64-bit ARM: aarch64-apple-ios
	6.	macOS Devices (Apple Silicon)
	•	Architecture: ARM64
	•	Target Triple: aarch64-apple-darwin
	1.	Samsung Galaxy S3
	•	Target Triple: armv7-linux-androideabi
	2.	Raspberry Pi 3
	•	32-bit: armv7-unknown-linux-gnueabihf
	•	64-bit: aarch64-unknown-linux-gnu
	3.	Raspberry Pi 4
	•	32-bit: armv7-unknown-linux-gnueabihf
	•	64-bit: aarch64-unknown-linux-gnu
	4.	NVIDIA Jetson Nano
	•	Target Triple: aarch64-unknown-linux-gnu
	5.	BeagleBone Black
	•	Target Triple: armv7-unknown-linux-gnueabihf
	6.	iPhone 12
	•	Target Triple: aarch64-apple-ios
	7.	Google Pixel 4
	•	Target Triple: aarch64-linux-android
	8.	Samsung Galaxy S10
	•	Target Triple: aarch64-linux-android
	9.	Amazon Fire TV Stick
	•	Target Triple: armv7-linux-androideabi
	10.	PlayStation 4
	•	Target Triple: x86_64-scei-ps4
	11.	Nintendo Switch
	•	Target Triple: aarch64-nintendo-switch-freestanding
	12.	ESP32 (Xtensa)
	•	Target Triple: xtensa-esp32-none-elf
	13.	Windows 10 (x64)
	•	Target Triple: x86_64-pc-windows-msvc
	14.	Ubuntu Desktop (x64)
	•	Target Triple: x86_64-unknown-linux-gnu
	15.	macOS (Intel)
	•	Target Triple: x86_64-apple-darwin
Phone Devices

	1.	Samsung Galaxy S9
	•	Target Triple: aarch64-linux-android
	2.	Google Pixel 5
	•	Target Triple: aarch64-linux-android
	3.	OnePlus 8
	•	Target Triple: aarch64-linux-android
	4.	Huawei P30
	•	Target Triple: aarch64-linux-android
	5.	Sony Xperia XZ2
	•	Target Triple: aarch64-linux-android
	6.	Moto G7
	•	Target Triple: armv7-linux-androideabi
	7.	Xiaomi Mi 10
	•	Target Triple: aarch64-linux-android
	8.	LG G7 ThinQ
	•	Target Triple: armv7-linux-androideabi

Embedded Devices

	1.	Raspberry Pi Zero
	•	Target Triple: armv6-unknown-linux-gnueabihf
	2.	Raspberry Pi Pico
	•	Target Triple: thumbv6m-none-eabi
	3.	Arduino Uno
	•	Target Triple: avr-unknown-unknown
	4.	STM32F4 Discovery
	•	Target Triple: thumbv7em-none-eabihf
	5.	BeagleBone AI
	•	Target Triple: aarch64-unknown-linux-gnu
	6.	ESP8266
	•	Target Triple: xtensa-esp8266-none-elf
	7.	ESP32-S2
	•	Target Triple: xtensa-esp32s2-none-elf
	8.	NXP i.MX RT1060
	•	Target Triple: thumbv7em-none-eabihf
	9.	Nordic nRF52840
	•	Target Triple: thumbv7em-none-eabihf
	10.	Atmel SAMD21
	•	Target Triple: thumbv6m-none-eabi


## Targets

| TARGET               | DEVICE        | RUNNER ADDED |
| --------             | ------------  | ---------    |
| aarch64-apple-darwin | * macbook pro |              |


aarch64-apple-ios
aarch64-apple-ios-macabi
aarch64-apple-ios-sim
aarch64-apple-tvos
aarch64-apple-tvos-sim
aarch64-apple-visionos
aarch64-apple-visionos-sim
aarch64-apple-watchos
aarch64-apple-watchos-sim
aarch64-fuchsia
aarch64-kmc-solid_asp3
aarch64-linux-android
aarch64-nintendo-switch-freestanding
aarch64-pc-windows-gnullvm
aarch64-pc-windows-msvc
aarch64-unknown-freebsd
aarch64-unknown-fuchsia
aarch64-unknown-hermit
aarch64-unknown-illumos
aarch64-unknown-linux-gnu
aarch64-unknown-linux-gnu_ilp32
aarch64-unknown-linux-musl
aarch64-unknown-linux-ohos
aarch64-unknown-netbsd
aarch64-unknown-none
aarch64-unknown-none-softfloat
aarch64-unknown-nto-qnx710
aarch64-unknown-openbsd
aarch64-unknown-redox
aarch64-unknown-teeos
aarch64-unknown-uefi
aarch64-uwp-windows-msvc
aarch64-wrs-vxworks
aarch64_be-unknown-linux-gnu
aarch64_be-unknown-linux-gnu_ilp32
aarch64_be-unknown-netbsd
arm-linux-androideabi
arm-unknown-linux-gnueabi
arm-unknown-linux-gnueabihf
arm-unknown-linux-musleabi
arm-unknown-linux-musleabihf
arm64_32-apple-watchos
arm64e-apple-darwin
arm64e-apple-ios
arm64ec-pc-windows-msvc
armeb-unknown-linux-gnueabi
armebv7r-none-eabi
armebv7r-none-eabihf
armv4t-none-eabi
armv4t-unknown-linux-gnueabi
armv5te-none-eabi
armv5te-unknown-linux-gnueabi
armv5te-unknown-linux-musleabi
armv5te-unknown-linux-uclibceabi
armv6-unknown-freebsd
armv6-unknown-netbsd-eabihf
armv6k-nintendo-3ds
armv7-linux-androideabi
armv7-sony-vita-newlibeabihf
armv7-unknown-freebsd
armv7-unknown-linux-gnueabi
armv7-unknown-linux-gnueabihf
armv7-unknown-linux-musleabi
armv7-unknown-linux-musleabihf
armv7-unknown-linux-ohos
armv7-unknown-linux-uclibceabi
armv7-unknown-linux-uclibceabihf
armv7-unknown-netbsd-eabihf
armv7-wrs-vxworks-eabihf
armv7a-kmc-solid_asp3-eabi
armv7a-kmc-solid_asp3-eabihf
armv7a-none-eabi
armv7a-none-eabihf
armv7k-apple-watchos
armv7r-none-eabi
armv7r-none-eabihf
armv7s-apple-ios
armv8r-none-eabihf
avr-unknown-gnu-atmega328
bpfeb-unknown-none
bpfel-unknown-none
csky-unknown-linux-gnuabiv2
csky-unknown-linux-gnuabiv2hf
hexagon-unknown-linux-musl
hexagon-unknown-none-elf
i386-apple-ios
i586-pc-nto-qnx700
i586-pc-windows-msvc
i586-unknown-linux-gnu
i586-unknown-linux-musl
i586-unknown-netbsd
i686-apple-darwin
i686-linux-android
i686-pc-windows-gnu
i686-pc-windows-gnullvm
i686-pc-windows-msvc
i686-unknown-freebsd
i686-unknown-haiku
i686-unknown-hurd-gnu
i686-unknown-linux-gnu
i686-unknown-linux-musl
i686-unknown-netbsd
i686-unknown-openbsd
i686-unknown-uefi
i686-uwp-windows-gnu
i686-uwp-windows-msvc
i686-win7-windows-msvc
i686-wrs-vxworks
loongarch64-unknown-linux-gnu
loongarch64-unknown-linux-musl
loongarch64-unknown-none
loongarch64-unknown-none-softfloat
m68k-unknown-linux-gnu
mips-unknown-linux-gnu
mips-unknown-linux-musl
mips-unknown-linux-uclibc
mips64-openwrt-linux-musl
mips64-unknown-linux-gnuabi64
mips64-unknown-linux-muslabi64
mips64el-unknown-linux-gnuabi64
mips64el-unknown-linux-muslabi64
mipsel-sony-psp
mipsel-sony-psx
mipsel-unknown-linux-gnu
mipsel-unknown-linux-musl
mipsel-unknown-linux-uclibc
mipsel-unknown-netbsd
mipsel-unknown-none
mipsisa32r6-unknown-linux-gnu
mipsisa32r6el-unknown-linux-gnu
mipsisa64r6-unknown-linux-gnuabi64
mipsisa64r6el-unknown-linux-gnuabi64
msp430-none-elf
nvptx64-nvidia-cuda
powerpc-unknown-freebsd
powerpc-unknown-linux-gnu
powerpc-unknown-linux-gnuspe
powerpc-unknown-linux-musl
powerpc-unknown-netbsd
powerpc-unknown-openbsd
powerpc-wrs-vxworks
powerpc-wrs-vxworks-spe
powerpc64-ibm-aix
powerpc64-unknown-freebsd
powerpc64-unknown-linux-gnu
powerpc64-unknown-linux-musl
powerpc64-unknown-openbsd
powerpc64-wrs-vxworks
powerpc64le-unknown-freebsd
powerpc64le-unknown-linux-gnu
powerpc64le-unknown-linux-musl
riscv32gc-unknown-linux-gnu
riscv32gc-unknown-linux-musl
riscv32i-unknown-none-elf
riscv32im-risc0-zkvm-elf
riscv32im-unknown-none-elf
riscv32ima-unknown-none-elf
riscv32imac-esp-espidf
riscv32imac-unknown-none-elf
riscv32imac-unknown-xous-elf
riscv32imafc-esp-espidf
riscv32imafc-unknown-none-elf
riscv32imc-esp-espidf
riscv32imc-unknown-none-elf
riscv64-linux-android
riscv64gc-unknown-freebsd
riscv64gc-unknown-fuchsia
riscv64gc-unknown-hermit
riscv64gc-unknown-linux-gnu
riscv64gc-unknown-linux-musl
riscv64gc-unknown-netbsd
riscv64gc-unknown-none-elf
riscv64gc-unknown-openbsd
riscv64imac-unknown-none-elf
s390x-unknown-linux-gnu
s390x-unknown-linux-musl
sparc-unknown-linux-gnu
sparc-unknown-none-elf
sparc64-unknown-linux-gnu
sparc64-unknown-netbsd
sparc64-unknown-openbsd
sparcv9-sun-solaris
thumbv4t-none-eabi
thumbv5te-none-eabi
thumbv6m-none-eabi
thumbv7a-pc-windows-msvc
thumbv7a-uwp-windows-msvc
thumbv7em-none-eabi
thumbv7em-none-eabihf
thumbv7m-none-eabi
thumbv7neon-linux-androideabi
thumbv7neon-unknown-linux-gnueabihf
thumbv7neon-unknown-linux-musleabihf
thumbv8m.base-none-eabi
thumbv8m.main-none-eabi
thumbv8m.main-none-eabihf
wasm32-unknown-emscripten
wasm32-unknown-unknown
wasm32-wasi
wasm32-wasip1
wasm32-wasip1-threads
wasm32-wasip2
wasm64-unknown-unknown
x86_64-apple-darwin
x86_64-apple-ios
x86_64-apple-ios-macabi
x86_64-apple-tvos
x86_64-apple-watchos-sim
x86_64-fortanix-unknown-sgx
x86_64-fuchsia
x86_64-linux-android
x86_64-pc-nto-qnx710
x86_64-pc-solaris
x86_64-pc-windows-gnu
x86_64-pc-windows-gnullvm
x86_64-pc-windows-msvc
x86_64-unikraft-linux-musl
x86_64-unknown-dragonfly
x86_64-unknown-freebsd
x86_64-unknown-fuchsia
x86_64-unknown-haiku
x86_64-unknown-hermit
x86_64-unknown-illumos
x86_64-unknown-l4re-uclibc
x86_64-unknown-linux-gnu
x86_64-unknown-linux-gnux32
x86_64-unknown-linux-musl
x86_64-unknown-linux-ohos
x86_64-unknown-netbsd
x86_64-unknown-none
x86_64-unknown-openbsd
x86_64-unknown-redox
x86_64-unknown-uefi
x86_64-uwp-windows-gnu
x86_64-uwp-windows-msvc
x86_64-win7-windows-msvc
x86_64-wrs-vxworks
x86_64h-apple-darwin
