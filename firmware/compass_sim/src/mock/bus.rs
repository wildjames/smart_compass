use std::sync::{Arc, Mutex};

use crate::mock::device::{MockDevice, MockError};

/// A simulated I2C bus that routes transactions to registered `MockDevice`
/// implementations based on the target address.
///
/// Devices are held behind `Arc<Mutex<..>>` so the `SimController` can
/// mutate device state while the bus is in use by the compass.
pub struct MockI2cBus {
    devices: Vec<Arc<Mutex<dyn MockDevice>>>,
}

impl MockI2cBus {
    pub fn new() -> Self {
        MockI2cBus {
            devices: Vec::new(),
        }
    }

    /// Register a device on the bus. The same `Arc<Mutex<dyn MockDevice>>`
    /// should be held by the `SimController` for state manipulation.
    pub fn add_device(&mut self, device: Arc<Mutex<dyn MockDevice>>) {
        self.devices.push(device);
    }

    fn find_device(&self, address: u8) -> Option<&Arc<Mutex<dyn MockDevice>>> {
        self.devices
          .iter()
          .find(
            |d| d.lock().unwrap().address() == address)
    }
}

impl Default for MockI2cBus {
    fn default() -> Self {
        Self::new()
    }
}

impl embedded_hal_async::i2c::ErrorType for MockI2cBus {
    type Error = MockError;
}

impl embedded_hal_async::i2c::I2c for MockI2cBus {
    async fn transaction(
        &mut self,
        address: u8,
        operations: &mut [embedded_hal_async::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        let device_arc = self
            .find_device(address)
            .ok_or(MockError::Nack)?
            .clone();

        let mut device = device_arc.lock().unwrap();

        let mut op_iter = operations.iter_mut().peekable();
        while let Some(op) = op_iter.next() {
            match op {
                embedded_hal_async::i2c::Operation::Write(data) => {
                    // Check if this is part of a write_read pair
                    if let Some(embedded_hal_async::i2c::Operation::Read(read_buf)) =
                        op_iter.peek_mut()
                    {
                        // write_read pair
                        device.write_read(data, read_buf)?;
                        op_iter.next(); // consume the Read
                    } else {
                        // plain write
                        device.write(data)?;
                    }
                }
                embedded_hal_async::i2c::Operation::Read(read_buf) => {
                    device.read(read_buf)?;
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::fuel_gauge::MockFuelGauge;
    use embedded_hal_async::i2c::I2c;

    #[tokio::test]
    async fn routes_to_correct_device() {
        let fg = Arc::new(Mutex::new(MockFuelGauge::new()));
        let mut bus = MockI2cBus::new();
        bus.add_device(fg.clone());

        // Read voltage register from address 0x36
        let mut buf = [0u8; 2];
        bus.write_read(0x36, &[0x02], &mut buf).await.unwrap();

        // Should get a valid voltage encoding
        let vcell = u16::from_be_bytes(buf) >> 4;
        let voltage = vcell as f32 * 1.25 / 1000.0;
        assert!(voltage > 0.0, "Expected non-zero voltage, got {voltage}");
    }

    #[tokio::test]
    async fn nack_for_unknown_address() {
        let mut bus = MockI2cBus::new();
        let mut buf = [0u8; 2];
        let result = bus.write_read(0x99, &[0x00], &mut buf).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn controller_changes_visible_to_bus() {
        let fg = Arc::new(Mutex::new(MockFuelGauge::new()));
        let mut bus = MockI2cBus::new();
        bus.add_device(fg.clone());

        // Set voltage via the shared handle (simulating SimController)
        fg.lock().unwrap().voltage = 4.2;

        let mut buf = [0u8; 2];
        bus.write_read(0x36, &[0x02], &mut buf).await.unwrap();

        let vcell = u16::from_be_bytes(buf) >> 4;
        let voltage = vcell as f32 * 1.25 / 1000.0;
        assert!((voltage - 4.2).abs() < 0.01, "Expected ~4.2V, got {voltage}");
    }
}
