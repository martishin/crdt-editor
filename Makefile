build-lib:
	pip install .

build-container:
	docker build -t crdt_server .

run-container:
	docker run -p 8000:8000 crdt_server

run-server:
	uvicorn server.server:app --reload

run-client:
	cd client && npm run dev

test:
	cargo test --manifest-path=crdt/Cargo.toml
