#![no_std]
#![no_main]

mod compass;
mod handlers;

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_nrf::Peripherals;
use embassy_nrf::bind_interrupts;
use embassy_nrf::peripherals;
use embassy_nrf::twim::{self, Twim};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::{Duration, Timer};
use panic_probe as _;

bind_interrupts!(struct Irqs {
    // TWIM - Twin Wire Interface Master (I2C)
    TWISPI0 => twim::InterruptHandler<peripherals::TWISPI0>;
});

static I2C_BUS: static_cell::StaticCell<Mutex<NoopRawMutex, Twim<'static>>> =
    static_cell::StaticCell::new();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p: Peripherals = embassy_nrf::init(Default::default());

    // Set up the shared I2C bus
    let twim_config = twim::Config::default();
    // Empty tx_ram_buffer: all writes will be from RAM, not flash
    let i2c = Twim::new(p.TWISPI0, Irqs, p.P0_26, p.P0_27, twim_config, &mut []);
    let i2c_bus = I2C_BUS.init(Mutex::new(i2c));

    let mut compass_handler = compass::Compass::new(i2c_bus);

    compass_handler.start().await;

    for _ in 0..10 {
        if let Some(report) = compass_handler.read_fuel_gauge() {
            defmt::info!(
                "Voltage: {}, SoC: {}%",
                report.voltage,
                report.state_of_charge
            );
        }

        Timer::after(Duration::from_secs(5)).await;
    }
    compass_handler.stop();
}
