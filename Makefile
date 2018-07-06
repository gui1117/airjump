check:
	cargo check --target asmjs-unknown-emscripten

build:
	mkdir -p target/publication/html/
	rm -rf target/publication/html/*
	cargo build --target wasm32-unknown-emscripten --release
	cp target/wasm32-unknown-emscripten/release/airjump.wasm target/publication/html/
	cp target/wasm32-unknown-emscripten/release/airjump.js target/publication/html/
	cp release.html target/publication/html/index.html

run: build
	firefox target/publication/html/index.html

publish_itch: build
	butler push target/publication/html/ rope/airjump:html

publish_thiolliere: build
	scp target/publication/html/* root@thiolliere.org:/var/www/html/airjump/

# android_log:
# 	~/android-sdk-linux/platform-tools/adb logcat | grep -ie AndroidGLue

# android_build:
# 	sudo docker run --rm -v `pwd`:/root/src -w /root/src tomaka/android-rs-glue cargo apk

# android_install:
# 	~/android-sdk-linux/platform-tools/adb install -r target/android-artifacts/build/bin/airjump-debug.apk

# TODO https://developer.android.com/studio/publish/app-signing.html
# TODO zipalign -v -p 4 my-app-unsigned.apk my-app-unsigned-aligned.apk
# TODO apksigner sign --ks my-release-key.jks --out my-app-release.apk my-app-unsigned-aligned.apk
