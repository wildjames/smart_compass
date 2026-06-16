pub struct MAXFuelGauge {
  state_of_charge: f32,
  voltage: f32,
  // TODO: Probably should cache the other stuff with some lifetime maybe? Future work
}

enum FuelGaugeMode {
  QUICKSTART,
  EN_SLEEP,
  HIBERNATE_STATE,
}

enum FuelGaugeHibernateMode {
  ACTIVE_THRESHOLD,
  HIBERNATE_THRESHOLD,
}

enum FuelGaugeStatus {
  RESET,
  VOLTAGE_HIGH,
  VOLTAGE_LOT,
  VOLTAGE_RESET,
  SOC_LOW,
  SOC_ONE_PCNT_CHANGE,
  ENABLE_VOLTAGE_RESET_ALERT,
}

impl MAXFuelGauge {
  pub fn new() -> Self {
    MAXFuelGauge {}
  }

  pub async fn get_voltage(&self) -> f32 {
    // Placeholder implementation
    let voltage = 3.7;
    self.voltage = voltage;
    voltage
  }

  pub async fn get_state_of_charge(&self) -> f32 {
    // Placeholder implementation
    let soc = 75.0;
    self.state_of_charge = soc;
    soc
  }

  pub async fn set_mode(&self, mode: FuelGaugeMode) {
    // Placeholder implementation
    println!("Setting mode to {}", mode);
  }

  pub async fn get_ic_version(&self) -> String {
    // Placeholder implementation
    "1.0.0".to_string()
  }

  pub async fn set_hibernate(&self, hibernate_mode: FuelGaugeHibernateMode) {
    // Placeholder implementation
    println!("Configuring hibernate to mode {}", hibernate_mode);
  }

  pub async fn get_hibernate(&self) -> FuelGaugeHibernateMode {
    // Placeholder implementation
    FuelGaugeHibernateMode::ACTIVE_THRESHOLD
  }

  pub async fn set_config(&self, rcomp: u8, address: u8) {
    // Placeholder implementation
    println!("Setting configuration to rcomp {}, address {}", rcomp, address);
  }

  pub async fn get_config(&self) -> (u8, u8) {
    // Placeholder implementation
    (0, 0)
  }

  pub async fn set_alert_threshold(&self, threshold: f32) {
    // Placeholder implementation
    println!("Setting alert threshold to {}%", threshold);
  }

  pub async fn get_alert_threshold(&self) -> f32 {
    // Placeholder implementation
    20.0
  }

  pub async fn get_charge_rate(&self) -> f32 {
    // Placeholder implementation
    0.5
  }

  pub async fn set_reset_voltage(&self, voltage: f32) {
    // Placeholder implementation
    println!("Setting reset voltage to {}V", voltage);
  }

  pub async fn get_reset_voltage(&self) -> f32 {
    // Placeholder implementation
    3.0
  }

  pub async fn set_status(&self, status: FuelGaugeStatus) {
    // Placeholder implementation
    println!("Setting status to {}", status);
  }

  pub async fn get_status(&self) -> FuelGaugeStatus {
    // Placeholder implementation
    FuelGaugeStatus::RESET
  }

  pub async fn set_table(self, table: &[u8]) {
    // Placeholder implementation
    println!("Setting table with length {}", table.len());
  }

  pub async fn get_cmd(&self) -> u8 {
    // Placeholder implementation
    0
  }

  pub async fn set_cmd(&self) {
    // Placeholder implementation
    println!("Sending CMF POR command");
  }
}
