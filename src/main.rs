#![no_std]
#![no_main]

extern crate alloc;

use uefi::prelude::*;
use log::info;
use alloc::string::ToString;

#[entry]
fn efi_main(handle: uefi::Handle, mut system_table: SystemTable<Boot>) -> Status {
    // Initializes:
    // - logger for the "log" crate
    // - panic handler
    uefi_services::init(&mut system_table).expect("Should init environment");
    info!("This output comes from the Hello World UEFI-App written in Rust.");
    info!("Firmware Version: {:?}", system_table.firmware_revision());
    info!("Firmware Vendor: {}", system_table.firmware_vendor().to_string());
    loop {}
    #[allow(unreachable_code)]
    Status::SUCCESS
}
