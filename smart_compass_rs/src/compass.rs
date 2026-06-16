use handlers::*;

use embassy_time::Duration;


pub struct Compass {
    pub loop_sleep_duration: Duration,

    // External ICs
    fuel_gauge: MAXFuelGauge,
}


pub impl Compass {
    pub fn new() -> Self {
        Compass {
            loop_sleep_duration: Duration::from_secs(1),
            fuel_gauge: MAXFuelGauge::new(),
        }
    }

    pub fn get_fuel_gauge(&self) -> &MAXFuelGauge {
        &self.fuel_gauge
    }

    pub async fn timestep(&self) {
        // Await all these tasks concurrently, since they're calling out to external ICs
        embassy_futures::join::join(
            self.fuel_gauge.get_state_of_charge(),
        ).await;
    }
}
