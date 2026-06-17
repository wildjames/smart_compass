run_sim:
	cargo run -p compass_sim

build_firmware:
	cargo build -p smart_compass_rs --features embedded --target thumbv7em-none-eabihf

test_firmware:
	cargo test -p smart_compass_rs --lib
	cargo test -p compass_sim

test_sim:
	cargo test -p compass_sim

test: test_firmware test_sim
