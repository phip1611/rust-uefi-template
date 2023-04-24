//! Entry-point into all unit-testable code of the project.
//! This module should be the owner of all Rust modules and re-export them
//! so that they can be consumed in main.rs.
//!
//! For various Rust-runtime related reasons (panic handler, global allocator),
//! this approach is the simplest to allow unit-tests and a productive
//! project setup.

#![no_std]

extern crate alloc;

// Prevent that the global allocator from uefi is used during test execution.
// This will change with uefi v0.21.0, because of https://github.com/rust-osdev/uefi-rs/pull/705
#[cfg(not(test))]
use uefi::prelude::*;
// Prevent that the panic-handler from uefi-services is used during test execution.
#[cfg(not(test))]
use uefi_services::*;

#[cfg(test)]
mod tests {
    use alloc::vec;

    #[test]
    fn test_tests_compile() {
        // test allocation compiles (there is a global allocator available)
        assert_eq!([1, 2, 3].as_slice(), vec![1, 2, 3].as_slice());
        assert_eq!(1 + 1, 2)
    }
}
