test:
	cargo test

lint:
	cargo clippy --all-targets --all-features -- -D warnings

vrt:
	node vrt/index.js && reg-cli vrt/screenshot/actual vrt/screenshot/expected vrt/screenshot/diff -R vrt/report.html -I

vrt-update:
	node vrt/index.js && reg-cli vrt/screenshot/actual vrt/screenshot/expected vrt/screenshot/diff -R vrt/report.html -U