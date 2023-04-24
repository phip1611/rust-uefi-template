# Cargo Template: UEFI Application with Unit Tests and Runtime Test in QEMU

This project is meant as inspiration and template for Rust/Cargo projects that
create UEFI applications. The biggest challenge is to enable `cargo test` in a
Cargo project, that uses a compilation target that is not a standard Windows,
Linux, or MacOS target.

To work around these problems, I use a small Makefile-based wrapper that
ensures a proper handling of `cargo test` and `cargo build`.

To test and build everything in this template, I recommend the usage of
[Nix](https://nixos.org) so that you can run: `nix-shell --run "make uefi-app && make test && make qemu-run"`.

## Build and Run
You need `make`, `rustup`, `qemu`, and the environment variable `OVMF` must
point to `OVMF.fd`. If you have Nix on your machine, `$ nix-shell` will bring
the environment variable into the game.
