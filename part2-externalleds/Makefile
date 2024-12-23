build:
	cargo objcopy --release -- -O ihex target/externalleds.hex
	teensy_loader_cli --mcu=TEENSY41 -v -w target/externalleds.hex
