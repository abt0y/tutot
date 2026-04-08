.PHONY: build test run-web docker-up docker-down clean

# Build all Rust crates
build:
	cargo build --workspace

# Build for WebAssembly (agents and ui crates)
build-wasm:
	wasm-pack build agents
	wasm-pack build ui

# Run all tests
test:
	cargo test --workspace

# Run a simple dev server for the frontend
run-web:
	npx serve .

# Start DeepTutor backend services
docker-up:
	cd DeepTutor && docker-compose up -d

# Stop DeepTutor backend services
docker-down:
	cd DeepTutor && docker-compose down

# Clean build artifacts
clean:
	cargo clean
	rm -rf target/
