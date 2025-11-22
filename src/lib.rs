#![no_std]
#![no_main]

#[unsafe(no_mangle)]
/// Halt function for esp-backtrace feature <custom-halt>
pub extern "Rust" fn custom_halt() {
    esp_hal::system::software_reset()
}

#[unsafe(no_mangle)]
/// Timestamp function for esp-println feature <timestamp>
pub extern "Rust" fn _esp_println_timestamp() -> u64 {
    esp_hal::time::Instant::now()
        .duration_since_epoch()
        .as_millis()
}
