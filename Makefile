.PHONY: default
default: uefi-app


.PHONY: uefi-app
uefi-app:
	cargo build --release --target x86_64-unknown-uefi


# Execute unit tests in lib.rs. As this is a project with mixed compilation
# targets (UEFI for the App, standard for the tests), this requires some more
# care. As a consequence, all tests must be in `lib.rs` or its submodules, but
# not in `main.rs`.
.PHONY: test
test:
	cargo test --lib


.PHONY: qemu-run
qemu-run: uefi-app
	mkdir -p .qemu/efi/boot
	cp target/x86_64-unknown-uefi/release/uefi_app.efi .qemu/efi/boot/bootx64.efi
	./.run_qemu.sh

.PHONY: clean
clean:
	cargo clean
