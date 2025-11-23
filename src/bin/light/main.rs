#![no_std]
#![no_main]
#![forbid(unsafe_code)]

use esp_metadata_generated::*;
use log::*;
use rs_matter_embassy::*;

mod cluster;
use cluster::identify::identify::ClusterHandler as _;

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
    let rng = esp_hal::rng::Rng::new();
    rand::esp::esp_init_rand(rng);

    info!("Customizing commissioning...");
    const BASIC_INFORMATION: matter::dm::clusters::basic_info::BasicInfoConfig =
        matter::dm::clusters::basic_info::BasicInfoConfig {
            vendor_name: "OpenSource",
            product_name: "Light",
            device_name: "Light",
            ..matter::dm::devices::test::TEST_DEV_DET
        };
    let password = rng.random() & 0x7ffffff;
    let discriminator = (rng.random() & 0xfff) as u16;

    info!("Initializing Matter stack...");
    let stack = matter::utils::init::InitMaybeUninit::init_with(
        alloc::boxed::Box::leak(alloc::boxed::Box::new_uninit()),
        wireless::EmbassyWifiMatterStack::<BUMP_SIZE, ()>::init(
            &BASIC_INFORMATION,
            matter::BasicCommData {
                password,
                discriminator,
            },
            &matter::dm::devices::test::TEST_DEV_ATT,
            epoch::epoch,
            rand::esp::esp_rand,
        ),
    );

    info!("Setting up data model...");
    const NODE: matter::dm::Node = matter::dm::Node {
        id: 0,
        endpoints: &[
            wireless::EmbassyWifiMatterStack::<0, ()>::root_endpoint(),
            matter::dm::Endpoint {
                id: 1,
                device_types: matter::devices!(matter::dm::devices::DEV_TYPE_ON_OFF_LIGHT),
                clusters: matter::clusters!(cluster::identify::IdentifyHandler::CLUSTER),
            },
        ],
    };
    let handler = matter::dm::EmptyHandler
        // Endpoint 1: Identify Cluster
        .chain(
            matter::dm::EpClMatcher::new(
                Some(1),
                Some(cluster::identify::IdentifyHandler::CLUSTER.id),
            ),
            matter::dm::Async(
                cluster::identify::IdentifyHandler::new(matter::dm::Dataver::new_rand(
                    stack.matter().rand(),
                ))
                .adapt(),
            ),
        );

    info!("Retrieving flash storage block...");
    let mut flash = esp_storage::FlashStorage::new(peripherals.FLASH);
    let mut storage = [0; esp_bootloader_esp_idf::partitions::PARTITION_TABLE_MAX_LEN];
    let nvs = esp_bootloader_esp_idf::partitions::read_partition_table(&mut flash, &mut storage)
        .unwrap()
        .find_partition(esp_bootloader_esp_idf::partitions::PartitionType::Data(
            esp_bootloader_esp_idf::partitions::DataPartitionSubType::Nvs,
        ))
        .unwrap()
        .unwrap();

    info!("Configuring persistent sessions...");
    let persist = stack
        .create_persist_with_comm_window(persist::EmbassyKvBlobStore::new(
            embassy_embedded_hal::adapter::BlockingAsync::new(flash),
            nvs.offset()..(nvs.offset() + nvs.len()),
        ))
        .await
        .unwrap();

    info!("Starting Matter server...");
    core::pin::pin!(stack.run_coex(
        wireless::EmbassyWifi::new(
            wireless::esp::EspWifiDriver::new(
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
