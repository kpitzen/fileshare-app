.PHONY: serve-dev serve

serve-dev:
	cargo watch -x run

serve:
	cargo run