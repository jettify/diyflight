run:
	cargo objcopy --release -- -O ihex firmware.hex
	teensy_loader_cli --mcu=TEENSY41 -v -w firmware.hex
