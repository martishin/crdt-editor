install-lib:
	pip install .

run-server:
	uvicorn server.server:app --reload

run-client:
	cd client && npm run dev

test:
	cargo test --manifest-path=library/Cargo.toml
