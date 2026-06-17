use std::sync::{Arc, Mutex};

use crate::mock::bus::MockI2cBus;
use crate::mock::device::MockError;
use crate::mock::fuel_gauge::MockFuelGauge;

/// High-level handle for controlling all simulated devices.
///
/// Shares ownership of each mock device with the `MockI2cBus`, so
/// mutations here are immediately visible to the compass driver.
pub struct SimController {
    pub fuel_gauge: Arc<Mutex<MockFuelGauge>>,
}

impl SimController {
    // ── Battery / fuel gauge helpers ────────────────────────────────

    /// Set the simulated battery voltage and state of charge.
    pub fn set_battery(&self, voltage: f32, soc: f32) {
        let mut fg = self.fuel_gauge.lock().unwrap();
        fg.voltage = voltage;
        fg.soc = soc;
    }

    /// Set the simulated charge/discharge rate (%/hr).
    /// Positive = charging, negative = discharging.
    pub fn set_charge_rate(&self, rate: f32) {
        self.fuel_gauge.lock().unwrap().charge_rate = rate;
    }

    /// Inject a one-shot I2C error on the fuel gauge.
    pub fn inject_fuel_gauge_error(&self, error: MockError) {
        self.fuel_gauge.lock().unwrap().error_next = Some(error);
    }

    /// Read the current simulated voltage (useful for logging/assertions).
    pub fn battery_voltage(&self) -> f32 {
        self.fuel_gauge.lock().unwrap().voltage
    }

    /// Read the current simulated SOC (useful for logging/assertions).
    pub fn battery_soc(&self) -> f32 {
        self.fuel_gauge.lock().unwrap().soc
    }
}

/// Create a `MockI2cBus` with all simulated devices wired up,
/// and a `SimController` that can manipulate them.
pub fn create_sim() -> (MockI2cBus, SimController) {
    let fuel_gauge = Arc::new(Mutex::new(MockFuelGauge::new()));

    let mut bus = MockI2cBus::new();
    bus.add_device(fuel_gauge.clone());

    let controller = SimController { fuel_gauge };

    (bus, controller)
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_hal_async::i2c::I2c;

    #[tokio::test]
    async fn controller_drives_bus_reads() {
        let (mut bus, ctrl) = create_sim();

        ctrl.set_battery(4.1, 90.0);

        // Read voltage
        let mut buf = [0u8; 2];
        bus.write_read(0x36, &[0x02], &mut buf).await.unwrap();
        let vcell = u16::from_be_bytes(buf) >> 4;
        let voltage = vcell as f32 * 1.25 / 1000.0;
        assert!((voltage - 4.1).abs() < 0.01);

        // Read SOC
        bus.write_read(0x36, &[0x04], &mut buf).await.unwrap();
        let soc = buf[0] as f32 + buf[1] as f32 / 256.0;
        assert!((soc - 90.0).abs() < 0.5);
    }

    #[tokio::test]
    async fn error_injection() {
        let (mut bus, ctrl) = create_sim();
        ctrl.inject_fuel_gauge_error(MockError::BusError);

        let mut buf = [0u8; 2];
        let result = bus.write_read(0x36, &[0x02], &mut buf).await;
        assert!(result.is_err());

        // Next read should succeed
        let result = bus.write_read(0x36, &[0x02], &mut buf).await;
        assert!(result.is_ok());
    }
}
