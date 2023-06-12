// All code with UEFI-specific Rust language-items (allocator, panic handler).
// Hence, there is no unit-test support in this crate. All testable code should
// be put into the library crate.

#![no_std]
#![no_main]

// There will be ugly compiler errors anyway. Hence, lets output an explicit
// message.
#[cfg(test)]
compile_error!("Do not invoke 'cargo test' on the no_std binary as it won't work anyway.");

extern crate alloc;

// Once you remove uefi-services, add this:
// use uefi::allocator::Allocator as UefiAllocator;
// #[global_allocator]
// static ALLOCATOR: UefiAllocator = UefiAllocator;

use alloc::string::ToString;
use log::info;
use uefi::Status;

use uefi::table::{Boot, SystemTable};
use uefi::table::runtime::ResetType;
use uefi_lib::*;

#[uefi::entry]
fn efi_main(handle: uefi::Handle, mut system_table: SystemTable<Boot>) -> uefi::Status {
    // If you develop a medium-sized app or more, you should drop uefi-services
    // and instead provide your own panic handler, logger, global allocator,
    // that better fits your needs.
    uefi_services::init(&mut system_table).unwrap();

    info!("This output comes from the UEFI-App written in Rust.");
    info!("Firmware Version: {:?}", system_table.firmware_revision());
    info!(
        "Firmware Vendor: {}",
        system_table.firmware_vendor().to_string()
    );
    info!(
        "5 + 7 = {}",
        add(5, 7)
    );
    info!("Shutting down in 2 seconds ...");

    system_table.boot_services().stall(2_000_000);
    system_table.runtime_services().reset(ResetType::SHUTDOWN, Status::SUCCESS, None);

    // loop {}
    // uefi::Status::SUCCESS
}
