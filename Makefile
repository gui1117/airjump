check:
	cargo check --target asmjs-unknown-emscripten --release --features include_all

build:
	rm -rf target/publication/html/*
	cargo build --target asmjs-unknown-emscripten --release --features include_all
	cp target/asmjs-unknown-emscripten/release/airjump.js target/publication/html/
	cp src/emscripten_audio.js target/publication/html/emscripten_audio.js
	cp release.html target/publication/html/index.html
	cp sounds/jump.mp3 target/publication/html/
	cp sounds/wall.mp3 target/publication/html/

run: build
	firefox target/publication/html/index.html

publish: build
	butler push target/publication/html/ rope/airjump:html

doc:
	cargo doc --open &
	rustup doc

android_log:
	~/android-sdk-linux/platform-tools/adb logcat | grep -ie AndroidGLue

android_build:
	sudo docker run --rm -v `pwd`:/root/src -w /root/src tomaka/android-rs-glue cargo apk --features include_all

android_install:
	~/android-sdk-linux/platform-tools/adb install -r target/android-artifacts/build/bin/airjump-debug.apk

# TODO https://developer.android.com/studio/publish/app-signing.html
# TODO zipalign -v -p 4 my-app-unsigned.apk my-app-unsigned-aligned.apk
# TODO apksigner sign --ks my-release-key.jks --out my-app-release.apk my-app-unsigned-aligned.apk
