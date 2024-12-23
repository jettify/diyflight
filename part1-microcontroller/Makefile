build:
	cargo objcopy --release -- -O ihex microcontroller.hex
	teensy_loader_cli --mcu=TEENSY41 -v -w microcontroller.hex
