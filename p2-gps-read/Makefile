dfu:
	cargo build --release
	arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/release/ublox_gps target/thumbv7em-none-eabihf/release/ublox_gps.bin
	dfu-suffix -v 0483 -p df11 -a target/thumbv7em-none-eabihf/release/ublox_gps.bin
	# Device must be in dfu mode, first: hold boot -> hold rst ->release rst -> release boot
	dfu-util -a 0 -s 0x08000000:leave -D target/thumbv7em-none-eabihf/release/ublox_gps.bin
