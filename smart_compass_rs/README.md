# Smart compass firmware

There are a lot of things that need to be interfaced with. I'm going to go with Rust here, for the concurrency benefits and the seemingly mature ecosystem, though I'll have to evaluate that as I go...

## List of core dependencies

| Crate | Responsibility | Docs |
|-------|---------------|------|
| `cortex-m` | Low-level access to Cortex-M peripherals (NVIC, SCB), critical section implementation for single-core nRF52840 | [docs.rs/cortex-m](https://docs.rs/cortex-m/0.7.7/cortex_m/) |
| `cortex-m-rt` | Startup runtime — defines the reset vector, memory layout, and `#[entry]` macro | [docs.rs/cortex-m-rt](https://docs.rs/cortex-m-rt/0.7.5/cortex_m_rt/) |
| `defmt` | Lightweight, deferred formatting logging framework for embedded (sends minimal data over RTT) | [docs.rs/defmt](https://docs.rs/defmt/1.1.0/defmt/) |
| `defmt-rtt` | Transport layer that streams `defmt` log frames over RTT (via SWD probe) | [docs.rs/defmt-rtt](https://docs.rs/defmt-rtt/1.2.0/defmt_rtt/) |
| `embassy-executor` | Async task executor for Cortex-M — runs Embassy tasks cooperatively | [docs.rs/embassy-executor](https://docs.rs/embassy-executor/0.10.0/embassy_executor/) |
| `embassy-nrf` | Hardware Abstraction Layer for nRF52840 — provides async drivers for GPIO, I2C, SPI, UART, PWM, etc. | [docs.rs/embassy-nrf](https://docs.rs/embassy-nrf/0.10.0/embassy_nrf/) |
| `embassy-time` | Async timers, delays, and time-keeping backed by the RTC peripheral | [docs.rs/embassy-time](https://docs.rs/embassy-time/0.5.1/embassy_time/) |
| `embedded-hal` | Standard hardware abstraction traits (I2C, SPI, GPIO) for portable device drivers | [docs.rs/embedded-hal](https://docs.rs/embedded-hal/1.0.0/embedded_hal/) |
| `nrf-sdc` | Rust bindings to Nordic's SoftDevice Controller — provides the BLE link layer | [docs.rs/nrf-sdc](https://docs.rs/nrf-sdc/0.3.0/nrf_sdc/) |
| `trouble-host` | BLE host stack (GATT, GAP) that runs on top of `nrf-sdc` — handles connections, services, characteristics | [docs.rs/trouble-host](https://docs.rs/trouble-host/0.6.0/trouble_host/) |

