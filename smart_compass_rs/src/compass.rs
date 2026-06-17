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

    /// Read the latest fuel gauge data (non-blocking, returns last published value)
    pub fn read_fuel_gauge(&self) -> Option<FuelGaugeReport> {
        let mut receiver = FuelGaugeMAX::<I2cDevice<'static, NoopRawMutex, BUS>>::receiver();
        receiver.try_get()
    }

    pub async fn start(&mut self) {
        self.fuel_gauge.start().await;
    }

    pub fn stop(&self) {
        FuelGaugeMAX::<I2cDevice<'static, NoopRawMutex, BUS>>::stop();
    }
}
