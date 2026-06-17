use compass_sim::mock::bus::MockI2cBus;
use compass_sim::mock::controller::create_sim;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::mutex::Mutex;
use smart_compass_rs::compass::Compass;

type SimCompass = Compass<MockI2cBus>;

fn main() {
    env_logger::init();

    let (bus, ctrl) = create_sim();

    // Set up initial battery state
    ctrl.set_battery(3.85, 62.5);
    ctrl.set_charge_rate(-2.1);

    println!("Compass Simulator");
    println!("=================");
    println!(
        "Initial state: {:.2}V, {:.1}% SoC, {:.1}%/hr charge rate",
        ctrl.battery_voltage(),
        ctrl.battery_soc(),
        ctrl.fuel_gauge.lock().unwrap().charge_rate,
    );

    // Leak the bus into a static Mutex, matching the pattern Compass expects
    let i2c_bus: &'static Mutex<NoopRawMutex, _> = Box::leak(Box::new(Mutex::new(bus)));

    let mut compass = SimCompass::new(i2c_bus);
    compass.set_fuel_gauge_polling_interval(1);

    // Run a short simulation using tokio
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            // Scenario task: adjust simulated state, read results, then stop
            let sim_scenario = async {
                // Let the compass poll a few times
                embassy_time::Timer::after(embassy_time::Duration::from_secs(3)).await;

                if let Some(report) = SimCompass::read_fuel_gauge() {
                    println!(
                        "  Read: {:.2}V, {:.1}% SoC",
                        report.voltage, report.state_of_charge
                    );
                } else {
                    println!("  (no data yet)");
                }

                // Simulate battery draining
                ctrl.set_battery(3.5, 30.0);
                println!("\n--- Battery draining: 3.50V, 30.0% ---");

                embassy_time::Timer::after(embassy_time::Duration::from_secs(3)).await;

                if let Some(report) = SimCompass::read_fuel_gauge() {
                    println!(
                        "  Read: {:.2}V, {:.1}% SoC",
                        report.voltage, report.state_of_charge
                    );
                }

                // Simulate critical battery
                ctrl.set_battery(2.9, 5.0);
                println!("\n--- Critical battery: 2.90V, 5.0% ---");

                embassy_time::Timer::after(embassy_time::Duration::from_secs(3)).await;

                if let Some(report) = SimCompass::read_fuel_gauge() {
                    println!(
                        "  Read: {:.2}V, {:.1}% SoC",
                        report.voltage, report.state_of_charge
                    );
                }

                println!("\nStopping compass...");
                SimCompass::stop();
            };

            // start() runs the polling loop; sim_scenario eventually calls stop()
            embassy_futures::select::select(compass.start(), sim_scenario).await;

            println!("Simulation complete.");
        });
}
