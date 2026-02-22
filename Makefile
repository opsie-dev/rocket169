fix:
	cargo clippy --fix
fmt:
	cargo fmt

.esp-setup:
	source $${HOME}/export-esp.sh

build: .esp-setup
	cargo build

run-test01: .esp-setup
	cargo run --bin rocket169-test_01
