build:
	cargo web deploy --release
	cp release.html target/deploy/index.html

publish_itch: build
	butler push target/deploy/ rope/airjump:html

# android_log:
# 	~/android-sdk-linux/platform-tools/adb logcat | grep -ie AndroidGLue

# android_build:
# 	sudo docker run --rm -v `pwd`:/root/src -w /root/src tomaka/android-rs-glue cargo apk

# android_install:
# 	~/android-sdk-linux/platform-tools/adb install -r target/android-artifacts/build/bin/airjump-debug.apk

# TODO https://developer.android.com/studio/publish/app-signing.html
# TODO zipalign -v -p 4 my-app-unsigned.apk my-app-unsigned-aligned.apk
# TODO apksigner sign --ks my-release-key.jks --out my-app-release.apk my-app-unsigned-aligned.apk
