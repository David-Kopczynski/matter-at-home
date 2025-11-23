#![no_std]
#![no_main]
#![forbid(unsafe_code)]

use esp_metadata_generated::*;
use log::*;

// Panic handler is required for [no_std]
use esp_backtrace as _;

// Import common utilities from lib.rs
use matter_at_home as _;

// Crate required for allocations
extern crate alloc;

// Application descriptor for esp-idf bootloader
esp_bootloader_esp_idf::esp_app_desc!();

// Allocate heap in reclaimed RAM from bootloader and regular RAM
const HEAP_SIZE: usize = 256 * 1024;
const BOOTLOADER_RECLAIMED_RAM: usize =
    memory_range!("DRAM2_UNINIT").end - memory_range!("DRAM2_UNINIT").start;

#[esp_rtos::main]
async fn main(spawner: embassy_executor::Spawner) {
    // Initialize logger from environment variables
    esp_println::logger::init_logger_from_env();

    // Allocate heap
    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: BOOTLOADER_RECLAIMED_RAM);
    esp_alloc::heap_allocator!(size: HEAP_SIZE - BOOTLOADER_RECLAIMED_RAM);

    // Initialize HAL
    let peripherals =
        esp_hal::init(esp_hal::Config::default().with_cpu_clock(esp_hal::clock::CpuClock::max()));

    // Initialize RTOS
    esp_rtos::start(
        esp_hal::timer::timg::TimerGroup::new(peripherals.TIMG0).timer0,
        esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT)
            .software_interrupt0,
    );

    info!("Embassy initialized!");
    info!("{}", esp_alloc::HEAP.stats());

    // ---------- Begin Example Application Code ----------

    // Spawn example task
    spawner.spawn(example_task()).unwrap();
}

#[embassy_executor::task]
async fn example_task() {
    warn!("Code will crash!");
    panic!("esp_backtrace should show this panic!");
}
