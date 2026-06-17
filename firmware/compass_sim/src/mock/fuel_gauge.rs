use crate::mock::device::{MockDevice, MockError};

/// MAX17043 I2C address
const MAX17043_ADDR: u8 = 0x36;

// Register addresses (mirrors the real driver)
const REG_VCELL: u8 = 0x02;
const REG_SOC: u8 = 0x04;
const REG_MODE: u8 = 0x06;
const REG_VERSION: u8 = 0x08;
const REG_HIBERNATE: u8 = 0x0A;
const REG_CONFIG: u8 = 0x0C;
const REG_CRATE: u8 = 0x16;
const REG_VRESET: u8 = 0x18;
const REG_STATUS: u8 = 0x1A;
const REG_CMD: u8 = 0xFE;

/// Simulated MAX17043 fuel gauge IC.
pub struct MockFuelGauge {
    // ── Controllable state ──────────────────────────────────────────
    /// Simulated cell voltage in volts (e.g. 3.8)
    pub voltage: f32,
    /// Simulated state of charge in percent (0.0 - 100.0+)
    pub soc: f32,
    /// Simulated charge/discharge rate in %/hr
    pub charge_rate: f32,
    /// IC version register value
    pub ic_version: u16,

    // ── Register state (written by the driver) ──────────────────────
    pub config_rcomp: u8,
    pub config_flags: u8,
    pub hibernate: u16,
    pub vreset: u16,
    pub status: u16,

    // ── Fault injection ─────────────────────────────────────────────
    /// If set, the next I2C operation will return this error (one-shot).
    pub error_next: Option<MockError>,
}

impl MockFuelGauge {
    pub fn new() -> Self {
        MockFuelGauge {
            voltage: 3.7,
            soc: 75.0,
            charge_rate: 0.0,
            ic_version: 3,
            config_rcomp: 0x97,
            config_flags: 0x1C,
            hibernate: 0x0000,
            vreset: 0x0000,
            status: 0x0000,
            error_next: None,
        }
    }

    /// Encode voltage into the VCELL register format.
    /// VCELL is a 12-bit value in the upper bits: raw = (vcell_counts << 4)
    /// where vcell_counts = voltage * 1000.0 / 1.25
    fn encode_vcell(&self) -> [u8; 2] {
        let vcell_counts = (self.voltage * 1000.0 / 1.25) as u16;
        (vcell_counts << 4).to_be_bytes()
    }

    /// Encode SOC into the SOC register format.
    /// High byte = integer percent, low byte = fractional (1/256 %) per datasheet
    fn encode_soc(&self) -> [u8; 2] {
        let integer = self.soc as u8;
        let fraction = ((self.soc - integer as f32) * 256.0) as u8;
        [integer, fraction]
    }

    /// Encode charge rate into the CRATE register format.
    /// Signed i16, real value = raw * 0.208 %/hr per datasheet
    fn encode_crate(&self) -> [u8; 2] {
        let raw = (self.charge_rate / 0.208) as i16;
        raw.to_be_bytes()
    }
}

impl Default for MockFuelGauge {
    fn default() -> Self {
        Self::new()
    }
}

impl MockDevice for MockFuelGauge {
    fn address(&self) -> u8 {
        MAX17043_ADDR
    }

    fn write_read(&mut self, write: &[u8], read: &mut [u8]) -> Result<(), MockError> {
        if let Some(err) = self.error_next.take() {
            return Err(err);
        }

        if write.is_empty() {
            read.fill(0);
            return Ok(());
        }

        let reg = write[0];
        let response = match reg {
            REG_VCELL => self.encode_vcell(),
            REG_SOC => self.encode_soc(),
            REG_VERSION => self.ic_version.to_be_bytes(),
            REG_HIBERNATE => self.hibernate.to_be_bytes(),
            REG_CONFIG => [self.config_rcomp, self.config_flags],
            REG_CRATE => self.encode_crate(),
            REG_VRESET => self.vreset.to_be_bytes(),
            REG_STATUS => self.status.to_be_bytes(),
            REG_CMD => 0x0000u16.to_be_bytes(),
            _ => [0x00, 0x00],
        };

        let len = read.len().min(response.len());
        read[..len].copy_from_slice(&response[..len]);
        Ok(())
    }

    fn write(&mut self, data: &[u8]) -> Result<(), MockError> {
        if let Some(err) = self.error_next.take() {
            return Err(err);
        }

        if data.len() < 3 {
            return Ok(());
        }

        let reg = data[0];
        let value = u16::from_be_bytes([data[1], data[2]]);

        match reg {
            REG_MODE => {
                // QuickStart, EnSleep, Hibernate - we just acknowledge
                log::debug!("FuelGauge: MODE write 0x{:04X}", value);
            }
            REG_HIBERNATE => {
                self.hibernate = value;
            }
            REG_CONFIG => {
                self.config_rcomp = data[1];
                self.config_flags = data[2];
            }
            REG_VRESET => {
                self.vreset = value;
            }
            REG_STATUS => {
                self.status = value;
            }
            REG_CMD => {
                if value == 0x5400 {
                    log::debug!("FuelGauge: POR command received, resetting state");
                    *self = Self::new();
                }
            }
            _ => {
                log::debug!("FuelGauge: unknown register write 0x{:02X} = 0x{:04X}", reg, value);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_vcell_known_voltage() {
        let fg = MockFuelGauge {
            voltage: 3.8,
            ..Default::default()
        };
        let raw = fg.encode_vcell();
        // Decode the same way the driver does: u16 >> 4 * 1.25 / 1000
        let vcell = u16::from_be_bytes(raw) >> 4;
        let decoded = vcell as f32 * 1.25 / 1000.0;
        assert!((decoded - 3.8).abs() < 0.01, "Expected ~3.8V, got {decoded}");
    }

    #[test]
    fn encode_soc_with_fraction() {
        let fg = MockFuelGauge {
            soc: 50.5,
            ..Default::default()
        };
        let raw = fg.encode_soc();
        let decoded = raw[0] as f32 + raw[1] as f32 / 256.0;
        assert!((decoded - 50.5).abs() < 0.01, "Expected ~50.5%, got {decoded}");
    }

    #[test]
    fn write_read_returns_voltage() {
        let mut fg = MockFuelGauge {
            voltage: 4.0,
            ..Default::default()
        };
        let mut buf = [0u8; 2];
        fg.write_read(&[REG_VCELL], &mut buf).unwrap();

        let vcell = u16::from_be_bytes(buf) >> 4;
        let decoded = vcell as f32 * 1.25 / 1000.0;
        assert!((decoded - 4.0).abs() < 0.01);
    }

    #[test]
    fn error_injection_is_one_shot() {
        let mut fg = MockFuelGauge::new();
        fg.error_next = Some(MockError::Nack);

        let mut buf = [0u8; 2];
        assert!(fg.write_read(&[REG_VCELL], &mut buf).is_err());
        // Second call should succeed
        assert!(fg.write_read(&[REG_VCELL], &mut buf).is_ok());
    }

    #[test]
    fn write_config_updates_state() {
        let mut fg = MockFuelGauge::new();
        fg.write(&[REG_CONFIG, 0xAB, 0xCD]).unwrap();
        assert_eq!(fg.config_rcomp, 0xAB);
        assert_eq!(fg.config_flags, 0xCD);
    }

    #[test]
    fn por_resets_state() {
        let mut fg = MockFuelGauge::new();
        fg.voltage = 2.0;
        fg.soc = 10.0;
        fg.write(&[REG_CMD, 0x54, 0x00]).unwrap();
        // Should be back to defaults
        assert!((fg.voltage - 3.7).abs() < 0.01);
        assert!((fg.soc - 75.0).abs() < 0.01);
    }
}
