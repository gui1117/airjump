check:
	cargo check --target asmjs-unknown-emscripten --release --features include_all

build:
	cargo build --target asmjs-unknown-emscripten --release --features include_all
	cp target/asmjs-unknown-emscripten/release/airjump.js target/publication/html/
	cp src/emscripten_audio.js target/publication/html/emscripten_audio.js
	cp release.html target/publication/html/index.html
	cp sounds/jump.ogg target/publication/html/
	cp sounds/wall.ogg target/publication/html/

run: build
	firefox target/publication/html/index.html

doc:
	cargo doc --open &
	rustup doc
