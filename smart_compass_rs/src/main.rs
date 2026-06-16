mod compass;
use handlers::*;

use embassy_nrf::{
    Peripherals,
};
use embassy_time::{Duration, Timer};
use embassy_executor::Spawner;


#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p: Peripherals = embassy_nrf::init(Default::default());


    let fuel_gauge = MAXFuelGauge::new();
    let compass = compass::Compass::new();

    // Start the main loop
    loop {
        let timer = Timer::after(compass.loop_sleep_duration);
        let compass_timestep_promise = compass.timestep();

        // Await both the timer and the compass timestep concurrently
        embassy_futures::join::join(timer, compass_timestep_promise).await;
    }
}
