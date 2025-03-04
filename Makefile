jsbuild:
	cd pdf.js && npx gulp generic
	cp -rf pdf.js/build/generic/* src/
	rm src/LICENSE
	rm src/build/*.map
	rm src/build/*.sandbox.mjs
	rm -rf src/web/cmaps
	rm -rf src/web/standard_fonts
	rm -rf src/web/wasm
	rm -rf src/web/*.pdf
	rm -rf src/web/*.map

dev:
	pnpm tauri dev

build:
	pnpm tauri build -b deb

dist: jsbuild build

.PHONY: dev build