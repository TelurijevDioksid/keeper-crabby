dev:
	KRAB_DIR=dev cargo run -p krab

test:
	mkdir -p /tmp/krab
	KRAB_TEMP_DIR=/tmp/krab cargo test

build:
	KRAB_DIR=release cargo build --release
