# Medallion Board

## Component Summary

This page reflects the current schematic at `Medallion_Board.kicad_sch` only.

| Function | Refs | Part / Value | Notes |
| -------- | ---- | ------------ | ----- |
| BLE / 802.15.4 MCU module | U1 | MDBT50Q-1MV2 | Main host MCU and USB 2.0 device interface |
| LoRa radio module | IC1 | Seeed Wio-E5-LE (`114993121`) | UART-connected sub-GHz LoRa / LoRaWAN module |
| GNSS receiver | U3 | LC86LICMD | UART-connected GNSS module |
| IMU | U6 | BNO085 | I2C-connected 9-axis motion sensor |
| Fuel gauge | U2 | MAX17048G+T10 | I2C battery fuel gauge with `LOW_BATT` output |
| Battery charger | U5 | MCP73831T-2ACI/OT | Single-cell Li-ion / LiPo charger, programmed for 500 mA with `R8 = 2 kOhm` |
| 3.3 V regulator | U4 | RT9080-33GJ5 | Regulates `VBAT` down to the 3.3 V logic rail |
| USB-C connector | X1 | USB4085-GF-A | USB 2.0 receptacle carrying `VBUS`, `D+`, and `D-` |
| RF connector | J2 | RECE.20369.001E.01 | Coax connector for the LoRa RF path |
| Backup / energy storage | C8 | 100 mF supercapacitor | Supercapacitor present in the schematic power section |
| Low-speed clock | Y1 | 32.768 kHz crystal | External crystal on the MCU clock network |
| Addressable LEDs | D3-D11 | WS2812B2020 | 17-pixel LED chain driven from `NEOPIX` |
| Discrete indicator LED | D2 | RED | Separate status LED |
| On-board buttons | SW1, SW2 | KMR2 | Two tactile switches on the main board |
| External button connectors | P3, P5, P4 | JST button connectors | `JST_BTN_1`, `JST_BTN_2`, and `JST_BTN_3` |
| Power / harness connectors | P1, P2 | JST 2-pin connectors | `P1 = JST_ONOFF`; `P2 = JST_PH` |

Additional support circuitry in the schematic includes 25 resistors, 32 capacitors, 2 inductors, and the passives required for USB-C configuration, power filtering, GNSS, IMU, charger, and LED support.

## Bus / Interface Connections

```mermaid
flowchart LR
    subgraph "USB"
        USBC[USB-C X1] -->|D+ / D-| MCU[MCU U1 MDBT50Q]
        USBC -->|VBUS| CHARGER[Charger U5 MCP73831]
    end

    subgraph "Power"
        CHARGER -->|VBAT| VBAT[VBAT Rail]
        VBAT -->|store / buffer| SUPERCAP[C8 100 mF]
        VBAT -->|regulate| LDO[U4 RT9080-33GJ5]
        VBAT -->|decouple| VLED[VLED Rail]
        LDO -->|+3.3V| VDD[3.3V Rail]
    end

    subgraph "HEADERS"
        MCU -->|SWD Programming header| SWD[SWD]
        MCU -->|Expansion| HEADER[GPIO and I2C breakout]
    end

    subgraph "UART"
        MCU -->|TXD_GPS / RXD_GPS| GNSS[GNSS U3 LC86LICMD]
    end

    subgraph "I2C Bus"
        MCU -->|SDA / SCL| FUEL[Fuel Gauge U2 MAX17048]
        MCU -->|SDA / SCL| IMU[IMU U6 BNO085]
        FUEL -->|LOW_BATT| MCU
        IMU -->|BNO_NINT / BNO_NRST| MCU
    end

    subgraph "SPI Bus"
        MCU -->|MOSI / MISO| LORA[LoRa IC1 Wio-E5]
        LORA -->|LORA_NRST, LORA_NSS| MCU
    end

    subgraph "User IO"
        MCU -->|NEOPIX| LEDS[WS2812 chain]
        MCU -->|JST1| BTN1[P3 JST_BTN_1]
        MCU -->|JST2| BTN2[P5 JST_BTN_2]
        MCU -->|JST3| BTN3[P4 JST_BTN_3]
        MCU -->|SMD button| SW[Reset]
    end
```
