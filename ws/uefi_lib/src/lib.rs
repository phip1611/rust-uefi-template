// All generic and unit-testable code independent of language-items specific to
// UEFIs runtime.

#![no_std]

extern crate alloc;

/// Test that doctests work:
/// ```rust
/// use uefi_lib::add;
/// assert_eq!(add(1, 2), 3);
/// ```
pub fn add(a: u64, b: u64) -> u64 {
    a + b
}

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
