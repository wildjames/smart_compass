# Medallion Board

## Component Summary

## Bus / Interface Connections

```mermaid
flowchart LR
    subgraph USB
        USBC[USB-C X1] -->|D+ / D-| MCU[MDBT50Q U5]
        USBC -->|VBUS| PWR[Power Management]
    end

    subgraph "I2C_Bus"[I2C Bus - SDA/SCL]
        MCU -->|I2C| FUEL[MAX17048 U3]
        MCU -->|I2C| IMU[BNO085 U6]
    end

    subgraph Power
        PWR -->|VBUS| CHRG[MCP73831 U4]
        CHRG -->|VBAT| BATTERY[LiPo Battery P1]
        PWR -->|VBAT| VREG[RT9080 LDO U1]
        VREG -->|+3.3V| MCU
        VREG -->|+3.3V| GNSS
        VREG -->|+3.3V| IMU
    end
```

## Power Consumption Budget (Worst-Case)

| Component | Supply (V) | Max Current per Unit (mA) | Qty | Total Current (mA) | Total Power (mW) |
| --------- | :--------: | :-----------------------: | :-: | :----------------: | :--------------: |
| **TOTAL** | | | | **XXX** | **XXX** |
