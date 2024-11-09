install-lib:
	pip install .

start-server:
	uvicorn server.server:app --reload

test:
	cargo test --manifest-path=library/Cargo.toml
