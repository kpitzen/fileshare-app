.PHONY: serve-dev serve format

serve-dev:
	cargo watch -x run

serve:
	cargo run

format:
	cargo fmt

lint:
	cargo fmt -- --check
