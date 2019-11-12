test:
	cargo test

vrt:
	node vrt/index.js && reg-cli vrt/screenshot/actual vrt/screenshot/expected vrt/screenshot/diff -R vrt/report.html

vrt-update:
	node vrt/index.js && reg-cli vrt/screenshot/actual vrt/screenshot/expected vrt/screenshot/diff -R vrt/report.html -U