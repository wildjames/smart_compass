use embassy_embedded_hal::shared_bus::asynch::i2c::I2cDevice;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::mutex::Mutex;

use crate::handlers::fuel_gauge_max::{FuelGaugeMAX, FuelGaugeReport};

pub type SharedI2cBus<BUS> = &'static Mutex<NoopRawMutex, BUS>;

pub struct Compass<BUS: embedded_hal_async::i2c::I2c + 'static> {
    // External ICs
    fuel_gauge: FuelGaugeMAX<I2cDevice<'static, NoopRawMutex, BUS>>,
}

impl<BUS: embedded_hal_async::i2c::I2c + 'static> Compass<BUS> {
    pub fn new(i2c_bus: SharedI2cBus<BUS>) -> Self {
        Compass {
            fuel_gauge: FuelGaugeMAX::new(I2cDevice::new(i2c_bus)),
        }
    }

    pub fn set_fuel_gauge_polling_interval(&mut self, interval_secs: u64) {
        self.fuel_gauge.set_polling_interval(interval_secs);
    }

    /// Read the latest fuel gauge data (non-blocking, returns last published value).
    /// This is an associated function — it reads from a static channel, not instance state.
    pub fn read_fuel_gauge() -> Option<FuelGaugeReport> {
        let mut receiver = FuelGaugeMAX::<I2cDevice<'static, NoopRawMutex, BUS>>::receiver();
        receiver.try_get()
    }

    pub async fn start(&mut self) {
        self.fuel_gauge.start().await;
    }

    /// Signal the polling loop to stop.
    /// This is an associated function — it signals a static channel, not instance state.
    pub fn stop() {
        FuelGaugeMAX::<I2cDevice<'static, NoopRawMutex, BUS>>::stop();
    }
}
