# Medallion Board

## Component Summary

This page reflects the current schematic at `Medallion_Board.kicad_sch` only.

| Function | Part / Value | Notes |
| -------- | ------------ | ----- |

## Bus / Interface Connections

```mermaid
flowchart LR
    subgraph "USB"
        USBC[USB-C X1] -->|D+ / D-| MCU[MCU MDBT50Q]
        USBC -->|VBUS| CHARGER[Charger MCP73831]
    end

    subgraph "Power"
        CHARGER -->|VBAT| VBAT[VBAT Rail]
        VBAT -->|store / buffer| SUPERCAP[C8 100mF]
        VBAT -->|regulate| LDO[U4 RT9080-33GJ5]
        VBAT -->|decouple| VLED[VLED Rail]
        LDO -->|+3.3V| VDD[3.3V Rail]
    end

    subgraph "HEADERS"
        MCU -->|SWD Programming header| SWD[SWD]
        MCU -->|Expansion| HEADER[GPIO and I2C breakout]
    end

    subgraph "GPIO PINS"
        LORA[LoRa Wio-E5] -->|LORA_NRST / LORA_NSS| MCU
        LORA[LoRa Wio-E5] -->|LORA_RF_SW / LORA_DIO / LORA_BUSY| MCU
        IMU[IMU BNO085] -->|BNO_NINT / BNO_NRST| MCU
        FUEL[Fuel Gauge MAX17048] -->|LOW_BATT| MCU
        GNSS -->|GNSS_RESET / GNSS_WAKEUP / GNSS_ANT_ACTIVE| GNSS[GNSS LC86LICMD]
        MCU -->|DRV_EN| HAPTIC[Haptic Engine]
    end

    subgraph "UART"
        MCU -->|TXD_GPS / RXD_GPS| GNSS
    end

    subgraph "I2C Bus"
        MCU --> FUEL
        MCU --> IMU
        MCU --> HAPTIC
    end

    subgraph "SPI Bus"
        MCU -->|MOSI / MISO| LORA
    end

    subgraph "User IO"
        MCU -->|NEOPIX| LEDS[WS2812 chain]
        MCU -->|JST1| BTN1[P3 JST_BTN_1]
        MCU -->|JST2| BTN2[P5 JST_BTN_2]
        MCU -->|JST3| BTN3[P4 JST_BTN_3]
        MCU -->|SMD button| SW[Reset]
    end
```
