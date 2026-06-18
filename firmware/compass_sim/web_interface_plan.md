# Compass Simulator Web Interface Plan

## Goal

Add a lightweight web UI that allows direct control of mocked ICs while the compass simulator runs indefinitely.

## Architecture

```
┌─────────────┐       ┌──────────────────┐       ┌─────────────┐
│  Browser UI │◄─────►│ Axum HTTP Server │◄─────►│SimController│
│  (HTML/JS)  │  REST │ (localhost:3000) │  Arc  │Arc<Mutex<…>>│
└─────────────┘       └──────────────────┘       └──────┬──────┘
                                                        │
                                                        ▼
                                                 ┌──────────────┐
                                                 │ MockI2cBus   │
                                                 │  ┌─────────┐ │
                                                 │  │FuelGauge│ │
                                                 │  └─────────┘ │
                                                 │  (future ICs)│
                                                 └──────┬───────┘
                                                        │
                                                        ▼
                                                 ┌─────────────┐
                                                 │   Compass   │
                                                 │  (polling)  │
                                                 └─────────────┘
```

The existing `SimController` already uses `Arc<Mutex<...>>` for device state, making it trivially sharable between the compass polling loop and HTTP handlers.

## Dependencies to Add

```toml
axum = "0.8"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tower-http = { version = "0.6", features = ["cors"] }
```

## REST API Endpoints

| Method | Path | Body | Description |
|--------|------|------|-------------|
| `GET` | `/api/state` | — | Returns full simulator state (voltage, SOC, charge rate, last reading) |
| `POST` | `/api/battery` | `{ "voltage": 3.8, "soc": 62.5 }` | Set battery voltage and SOC |
| `POST` | `/api/charge_rate` | `{ "rate": -2.1 }` | Set charge/discharge rate (%/hr) |
| `POST` | `/api/error` | `{ "kind": "nack" \| "bus_error" }` | Inject a one-shot I2C error |
| `GET` | `/api/fuel_gauge` | — | Latest `Compass::read_fuel_gauge()` result |
| `GET` | `/` | — | Serve the HTML/JS UI |

## Frontend (Single-Page HTML)

A single `index.html` served as an embedded string or from a `static/` directory:

- **Status panel**: Displays current simulated state + last compass reading (auto-refreshes via polling or SSE)
- **Controls**:
  - Voltage slider (2.5V – 4.2V)
  - SOC slider (0% – 100%)
  - Charge rate input (-10 to +10 %/hr)
  - "Inject Error" buttons (NACK, Bus Error)
- **Log**: Scrolling list of compass readings over time

No build tooling — vanilla HTML + JS + fetch API.

## Changes to `main.rs`

Replace the scripted scenario with:

```rust
tokio::select! {
    _ = compass.start() => {},
    _ = web::serve(controller) => {},
}
```

Both tasks run concurrently:
- `compass.start()` polls the mock bus indefinitely
- `web::serve()` runs the axum server on `0.0.0.0:3000`

## File Structure

```
compass_sim/
├── Cargo.toml              (add deps)
├── src/
│   ├── main.rs             (simplified: create sim, start both tasks)
│   ├── lib.rs              (add `pub mod web;`)
│   ├── web/
│   │   ├── mod.rs          (router setup, serve function)
│   │   ├── handlers.rs     (endpoint handler functions)
│   │   └── state.rs        (shared AppState struct)
│   ├── mock/               (unchanged)
│   └── static/
│       └── index.html      (embedded or served from disk)
```

## Implementation Steps

1. Add dependencies to `Cargo.toml`
2. Create `src/web/state.rs` — `AppState` struct wrapping `Arc<SimController>`
3. Create `src/web/handlers.rs` — handler functions for each endpoint
4. Create `src/web/mod.rs` — router construction and `serve()` async fn
5. Create `src/static/index.html` — single-page UI
6. Update `src/lib.rs` to export `pub mod web`
7. Rewrite `src/main.rs` to run indefinitely with both tasks
8. Test: `cargo run`, open `http://localhost:3000`
