# Medallion Board

### External components

For the user button leads, use this connector: [GHR-02V-S](https://www.digikey.co.uk/en/products/detail/jst-sales-america-inc/GHR-02V-S/807814?s=N4IgjCBcoBw1oDGUBmBDANgZwKYBoQB7KAbXACYwBmcgNhAF0CAHAFyhAGVWAnASwB2AcxABfUQXKkQAKU4AVAAQBxABKLGooA). I've not settled on buttons yet. There are two options - either side-mounted tactile smd buttons like [these](https://www.amazon.co.uk/ECSiNG-Momentary-5x7-8x3-5mm-Controls-Dashboards/dp/B0GFPX8CMW/ref=sr_1_3?sr=8-3), or through hole button like [this](https://www.amazon.co.uk/10PCS-Momentary-Push-Button-Switch/dp/B0FHVRM49T/ref=sr_1_3?sr=8-3).

There will also be a power switch on the case - it can interface with [this](https://www.amazon.co.uk/sourcingmap%C2%AE-Position-Locking-Switch-7x2x1mm/dp/B01N25FBWD) SMD switch, or possibly the [chunkier cousin](https://www.amazon.co.uk/Miniature-Model-Railway-Switch-2-Position/dp/B00TXNXFZC/) that has through-hole legs

I should be able to fit a 504050 or 505050 LiPo behind the PCB. That gives approx 1000mAh capacity, e.g. [here](https://www.ebay.co.uk/itm/375695071227) and [here](https://www.ebay.co.uk/itm/195457885965)

For the haptic engine, use the Vybronics `VLV101040A` - a 10 x 10 x 4 mm linear resonant actuator giving 2.75 G at 170 Hz with a 10 ms rise time ([datasheet](../../datasheets/VLV101040A_Vybronics.pdf), and it is on the [extra parts BOM](../EXTRA_PARTS_BOM.md)). It was picked over a plain coin motor for the rise time, which is what makes the tick feel sharp rather than buzzy. This can, in a pinch, have a hole for it punched through the PCB if that helps place it, but it needs to be adhered to the rear of the case to work properly. Note that its contacts are pressure pads meant for pogo pins, not solder tabs - Vybronics permits thin UL3302 AWG 32/34 flying leads instead, which is what the pigtail should be made from.

For the LoRa antenna, I plan on using a PCB antenna, e.g. [here](https://www.amazon.co.uk/915MHz-Antenna-Meshtastic-Development-Boards-Black/dp/B0FLVF19CQ). I think these are flexible PCB antennas, so can bend to fit the contour of the case. This should not affect the performance too much.

## System Block Diagrams

### Power

```mermaid
flowchart LR
    USBC[USB-C X1] -->|VBUS| CHARGER[MCP73831]
    CHARGER --> VBAT[VBAT Rail]
    VBAT --> SUPERCAP[C8 100mF]
    VBAT --> LDO[RT9080-33GJ5]
    VBAT --> VLED[VLED Rail]
    LDO --> VDD[3.3V Rail]

    MCU[nRF52840] <-->|I²C| FUEL[MAX17048 Fuel Gauge]
    FUEL -->|GPIO: LOW_BATT| MCU
    USBC -->|USB D+/D-| MCU
```

### Navigation & Positioning

```mermaid
flowchart LR
    MCU[nRF52840] <-->|UART| GNSS[LC86LICMD GNSS]
    MCU -->|GPIO: GNSS_RESET| GNSS

    MCU <-->|I²C| IMU[BNO086 IMU]
    MCU -->|GPIO: BNO_NINT BNO_NRST| IMU
```

### Communication

```mermaid
flowchart LR
    MCU[nRF52840] <-->|SPI| LORA[LoRa Wio-E5]
    MCU -->|GPIO: LORA_NSS LORA_NRST| LORA
    LORA -->|GPIO: LORA_DIO LORA_BUSY LORA_RF_SW| MCU
```

### Storage

```mermaid
flowchart LR
    MCU[nRF52840] <-->|SPI| FLASH[Flash Memory]
    MCU -->|GPIO: FLASH_CS RST WP| FLASH
```

### Display & Feedback

```mermaid
flowchart LR
    MCU[nRF52840] -->|SPI| J7[J7 FFC]
    MCU -->|GPIO: DISP_CS DC RES BUSY| J7
    J7 --> EINK[Display board]

    MCU -->|1-Wire: NEOPIX| R13[R13 500R]
    R13 --> J6[J6 FFC]
    J6 --> LEDS[LedRing board]
    J6 -->|NEOPIX_RET| TP1[TP1 test pad]

    MCU <-->|I²C| HAPTIC[Haptic Engine]
    MCU -->|GPIO: DRV_EN| HAPTIC
```

### User Input & Debug

```mermaid
flowchart LR
    MCU[nRF52840] -->|GPIO: JST1| BTN1[P3 JST BTN 1]
    MCU -->|GPIO: JST2| BTN2[P5 JST BTN 2]
    MCU -->|GPIO: JST3| BTN3[P4 JST BTN 3]
    MCU --- SW[Reset Button]

    MCU -->|SWD| SWD[SWD Header]
    MCU --> HEADER[GPIO / I²C Breakout]
```
