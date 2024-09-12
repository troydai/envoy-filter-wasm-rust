build: src/lib.rs
	@ cargo build --target wasm32-wasip1 --release

up: build
	@ docker-compose up