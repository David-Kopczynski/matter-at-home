#![no_std]
#![no_main]
#![forbid(unsafe_code)]

use esp_metadata_generated::*;
use log::*;
use rs_matter_embassy::*;

// Panic handler is required for [no_std]
use esp_backtrace as _;

// Import common utilities from lib.rs
use matter_at_home as _;

// Crate required for allocations
extern crate alloc;

// Application descriptor for esp-idf bootloader
esp_bootloader_esp_idf::esp_app_desc!();

// Allocate heap in reclaimed RAM from bootloader and regular RAM
const HEAP_SIZE: usize = 150 * 1024;
const BUMP_SIZE: usize = 25 * 1024;
const BOOTLOADER_RECLAIMED_RAM: usize =
    memory_range!("DRAM2_UNINIT").end - memory_range!("DRAM2_UNINIT").start;

#[esp_rtos::main]
async fn main(_spawner: embassy_executor::Spawner) {
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

    // ---------- Begin Matter Application Code ----------

    info!("Configuring rs-matter-embassy...");
    rs_matter_embassy::rand::esp::esp_init_rand(esp_hal::rng::Rng::new());

    info!("Initializing Matter stack...");
    let stack = rs_matter_embassy::matter::utils::init::InitMaybeUninit::init_with(
        alloc::boxed::Box::leak(alloc::boxed::Box::new_uninit()),
        rs_matter_embassy::wireless::EmbassyWifiMatterStack::<BUMP_SIZE, ()>::init(
            &rs_matter_embassy::matter::dm::devices::test::TEST_DEV_DET,
            rs_matter_embassy::matter::dm::devices::test::TEST_DEV_COMM,
            &rs_matter_embassy::matter::dm::devices::test::TEST_DEV_ATT,
            rs_matter_embassy::epoch::epoch,
            rs_matter_embassy::rand::esp::esp_rand,
        ),
    );

    info!("Setting up data model...");
    const NODE: matter::dm::Node = rs_matter_embassy::matter::dm::Node {
        id: 0,
        endpoints: &[rs_matter_embassy::wireless::EmbassyWifiMatterStack::<0, ()>::root_endpoint()],
    };
    let handler = rs_matter_embassy::matter::dm::EmptyHandler;

    info!("Configuring persistent sessions...");
    let persist = stack
        .create_persist_with_comm_window(rs_matter_embassy::stack::persist::DummyKvBlobStore)
        .await
        .unwrap();

    info!("Starting Matter server...");
    core::pin::pin!(stack.run_coex(
        rs_matter_embassy::wireless::EmbassyWifi::new(
            rs_matter_embassy::wireless::esp::EspWifiDriver::new(
                &esp_radio::init().unwrap(),
                peripherals.WIFI,
                peripherals.BT
            ),
            stack
        ),
        &persist,
        (NODE, handler),
        (),
    ))
    .await
    .unwrap();
}
