dfu:
	cargo build --release
	arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/release/spbee_blinky target/thumbv7em-none-eabihf/release/spbee_blinky.bin
	dfu-suffix -v 0483 -p df11 -a target/thumbv7em-none-eabihf/release/spbee_blinky.bin
	dfu-util -l
	dfu-util -a 0 -s 0x08000000:leave -D target/thumbv7em-none-eabihf/release/spbee_blinky.bin

bf:
	dfu-util -l
	dfu-util -a 0 -s 0x08000000:leave -D betaflight-speedybee-firmware.bin
