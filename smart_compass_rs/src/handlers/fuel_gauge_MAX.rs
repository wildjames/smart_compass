use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;
use embassy_sync::watch::Watch;
use embassy_futures::select::{select, Either};
use embassy_time::{Duration, Timer};
use embedded_hal_async::i2c::I2c;

const MAX17043_ADDR: u8 = 0x36;

// Register addresses
const REG_VCELL: u8 = 0x02;
const REG_SOC: u8 = 0x04;
const REG_MODE: u8 = 0x06;
const REG_VERSION: u8 = 0x08;
const REG_HIBERNATE: u8 = 0x0A;
const REG_CONFIG: u8 = 0x0C;
const REG_VALRT: u8 = 0x14;
const REG_CRATE: u8 = 0x16;
const REG_VRESET: u8 = 0x18;
const REG_STATUS: u8 = 0x1A;
const REG_TABLE: u8 = 0x40;
const REG_CMD: u8 = 0xFE;

#[derive(Clone, Copy, Default)]
pub struct FuelGaugeReport {
    pub voltage: f32,
    pub state_of_charge: f32,
}

static FUEL_GAUGE_DATA: Watch<CriticalSectionRawMutex, FuelGaugeReport, 2> = Watch::new();
static FUEL_GAUGE_STOP: Signal<CriticalSectionRawMutex, ()> = Signal::new();

pub struct FuelGaugeMAX<I: I2c> {
    i2c: I,
    polling_interval: Duration,
}

pub enum FuelGaugeMode {
    QuickStart,
    EnSleep,
    HibernateState,
}

pub enum FuelGaugeHibernateMode {
    ActiveThreshold,
    HibernateThreshold,
}

pub enum FuelGaugeStatus {
    RESET,
    VoltageHigh,
    VoltageLot,
    VoltageReset,
    SocLow,
    SocOnePcntChange,
    EnableVoltageResetAlert,
}

impl<I: I2c> FuelGaugeMAX<I> {
    pub fn new(i2c: I) -> Self {
        FuelGaugeMAX {
            i2c,
            polling_interval: Duration::from_secs(10),
        }
    }

    /// Returns a receiver that always holds the latest fuel gauge reading.
    pub fn receiver() -> embassy_sync::watch::Receiver<'static, CriticalSectionRawMutex, FuelGaugeReport, 2>
    {
        FUEL_GAUGE_DATA.receiver().unwrap()
    }

    pub async fn start(&mut self) {
        let sender = FUEL_GAUGE_DATA.sender();

        loop {
            let timer = Timer::after(self.polling_interval);

            match select(timer, FUEL_GAUGE_STOP.wait()).await {
                Either::First(_) => {
                    let report = self.poll().await;
                    sender.send(report);
                }
                Either::Second(_) => {
                    break;
                }
            }
        }
    }

    pub fn stop() {
        FUEL_GAUGE_STOP.signal(());
    }

    async fn read_register(&mut self, reg: u8) -> [u8; 2] {
        let mut buf = [0u8; 2];
        let _ = self.i2c.write_read(MAX17043_ADDR, &[reg], &mut buf).await;
        buf
    }

    async fn write_register(&mut self, reg: u8, value: u16) {
        let bytes = value.to_be_bytes();
        let _ = self.i2c.write(MAX17043_ADDR, &[reg, bytes[0], bytes[1]]).await;
    }

    async fn poll(&mut self) -> FuelGaugeReport {
        let voltage = self.get_voltage().await;
        let state_of_charge = self.get_state_of_charge().await;
        FuelGaugeReport { voltage, state_of_charge }
    }

    //--//--//--//--//--//--//--//--//--//--//--//--//--//--//--//--//--//--//
    // Methods for interacting with the MAX17043 fuel gauge IC via I2C      //
    //--//--//--//--//--//--//--//--//--//--//--//--//--//--//--//--//--//--//

    async fn get_voltage(&mut self) -> f32 {
        let raw = self.read_register(REG_VCELL).await;
        let vcell = u16::from_be_bytes(raw) >> 4;
        vcell as f32 * 1.25 / 1000.0
    }

    async fn get_state_of_charge(&mut self) -> f32 {
        let raw = self.read_register(REG_SOC).await;
        raw[0] as f32 + raw[1] as f32 / 256.0
    }

    pub async fn set_mode(&mut self, mode: FuelGaugeMode) {
        let value = match mode {
            FuelGaugeMode::QuickStart => 0x4000,
            FuelGaugeMode::EnSleep => 0x2000,
            FuelGaugeMode::HibernateState => 0x1000,
        };
        self.write_register(REG_MODE, value).await;
    }

    pub async fn get_ic_version(&mut self) -> u16 {
        let raw = self.read_register(REG_VERSION).await;
        u16::from_be_bytes(raw)
    }

    pub async fn set_hibernate(&mut self, hibernate_mode: FuelGaugeHibernateMode) {
        let value = match hibernate_mode {
            FuelGaugeHibernateMode::ActiveThreshold => 0x0000,
            FuelGaugeHibernateMode::HibernateThreshold => 0xFFFF,
        };
        self.write_register(REG_HIBERNATE, value).await;
    }

    pub async fn get_hibernate(&mut self) -> u16 {
        let raw = self.read_register(REG_HIBERNATE).await;
        u16::from_be_bytes(raw)
    }

    pub async fn set_config(&mut self, rcomp: u8, threshold: u8) {
        let value = (rcomp as u16) << 8 | threshold as u16;
        self.write_register(REG_CONFIG, value).await;
    }

    pub async fn get_config(&mut self) -> (u8, u8) {
        let raw = self.read_register(REG_CONFIG).await;
        (raw[0], raw[1])
    }

    pub async fn set_alert_threshold(&mut self, threshold: u8) {
        let (rcomp, flags) = self.get_config().await;
        let new_flags = (flags & 0xE0) | (32u8.saturating_sub(threshold) & 0x1F);
        self.set_config(rcomp, new_flags).await;
    }

    pub async fn get_alert_threshold(&mut self) -> u8 {
        let (_, flags) = self.get_config().await;
        32 - (flags & 0x1F)
    }

    pub async fn get_charge_rate(&mut self) -> f32 {
        let raw = self.read_register(REG_CRATE).await;
        let crate_val = i16::from_be_bytes(raw);
        crate_val as f32 * 0.208
    }

    pub async fn set_reset_voltage(&mut self, voltage_threshold: u8) {
        let raw = self.read_register(REG_VRESET).await;
        let value = (voltage_threshold as u16) << 9 | (raw[1] as u16 & 0x01);
        self.write_register(REG_VRESET, value).await;
    }

    pub async fn get_reset_voltage(&mut self) -> u8 {
        let raw = self.read_register(REG_VRESET).await;
        raw[0] >> 1
    }

    pub async fn set_status(&mut self, value: u16) {
        self.write_register(REG_STATUS, value).await;
    }

    pub async fn get_status(&mut self) -> u16 {
        let raw = self.read_register(REG_STATUS).await;
        u16::from_be_bytes(raw)
    }

    pub async fn set_table(&mut self, table: &[u8]) {
        for (i, chunk) in table.chunks(2).enumerate() {
            if chunk.len() == 2 {
                let value = u16::from_be_bytes([chunk[0], chunk[1]]);
                self.write_register(REG_TABLE + (i as u8 * 2), value).await;
            }
        }
    }

    pub async fn get_cmd(&mut self) -> u16 {
        let raw = self.read_register(REG_CMD).await;
        u16::from_be_bytes(raw)
    }

    pub async fn send_por(&mut self) {
        self.write_register(REG_CMD, 0x5400).await;
    }
}
