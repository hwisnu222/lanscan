.PHONY: run

run:
	cargo build
	sudo ./target/debug/lanscan
