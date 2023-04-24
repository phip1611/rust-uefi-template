//! Entry-point into all unit-testable code of the project.
//! This module should be the owner of all Rust modules and re-export them
//! so that they can be exported in main.rs.
//!
//! For various Rust-runtime related reasons, this approach is the simplest
//! to allow unit-tests.

#![no_std]

extern crate alloc;

// Prevent that the global allocator from uefi is used during test execution.
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
