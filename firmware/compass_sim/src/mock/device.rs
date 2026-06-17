/// Errors that mock devices can produce, mapping to real I2C failure modes.
#[derive(Debug, Clone)]
pub enum MockError {
    /// Device did not acknowledge (e.g. wrong address, device not present)
    Nack,
    /// Bus arbitration lost
    BusError,
    /// Generic / custom error message
    Other(&'static str),
}

impl embedded_hal::i2c::Error for MockError {
    fn kind(&self) -> embedded_hal::i2c::ErrorKind {
        match self {
            MockError::Nack => embedded_hal::i2c::ErrorKind::NoAcknowledge(
                embedded_hal::i2c::NoAcknowledgeSource::Address,
            ),
            MockError::BusError => embedded_hal::i2c::ErrorKind::Bus,
            MockError::Other(_) => embedded_hal::i2c::ErrorKind::Other,
        }
    }
}

impl core::fmt::Display for MockError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            MockError::Nack => write!(f, "I2C NACK"),
            MockError::BusError => write!(f, "I2C bus error"),
            MockError::Other(msg) => write!(f, "I2C error: {msg}"),
        }
    }
}

/// Trait for simulated I2C devices. Each implementation models a specific IC's
/// register map and encodes controllable internal state into raw I2C responses.
pub trait MockDevice: Send {
    /// The 7-bit I2C address this device responds to.
    fn address(&self) -> u8;

    /// Handle a write-then-read transaction
    /// `write` contains the bytes written by the master (e.g. a register address).
    /// `read` is the buffer to fill with the response.
    fn write_read(&mut self, write: &[u8], read: &mut [u8]) -> Result<(), MockError>;

    /// Handle a plain write transaction
    /// `data` contains all bytes written by the master (register address + value).
    fn write(&mut self, data: &[u8]) -> Result<(), MockError>;

    /// Handle a plain read transaction (read without preceding write).
    /// Default implementation returns zeros.
    fn read(&mut self, read: &mut [u8]) -> Result<(), MockError> {
        read.fill(0);
        Ok(())
    }
}
