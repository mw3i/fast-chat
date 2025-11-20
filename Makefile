.PHONY: init desktop desktop-clean build build-mac build-windows build-linux build-all icons
# ^ Makefile requires this for some reason

# Initialize the development environment
init-mac:
	./.dev/init-mac.sh

init-linux:
	./.dev/init-linux.sh

# Run the desktop app
desktop:
	cd desktop && cargo tauri dev

# Rebuild then run the desktop app
desktop-clean:
	cd desktop && cargo clean && cargo tauri dev

# Build for current platform
build:
	cd desktop && cargo tauri build

# Build for all platforms
build-all:
	cd desktop 
	cargo tauri build --bundles dmg
	cargo tauri build --bundles msi
	cargo tauri build --bundles appimage

# Generate icons
icons:
	cd desktop/ && cargo tauri icon