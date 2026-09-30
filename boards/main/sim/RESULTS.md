# Main board simulation results

Results of the transient scenario in [README.md](README.md), run on 2026-09-29
after three changes to the power path: the pull-down on the load-sharing FET's
gate went from 100k to 4.7k, the VBUS Schottky went from a 0.5 A MBR0540 to a
1 A SS1040-AU, and a second SS1040-AU went in across the load-sharing FET,
from the cell to VSYS.

## Cases

The scenario was run as it stands and with the `.param` values below changed.
Full white is every LED on the ring at full brightness. 3.4 V is a nearly flat
cell.

| Case | `VBAT` | `RING_W` |
| ---- | ------ | -------- |
| Nominal | 3.7 V | 0.45 W |
| Full white | 3.7 V | 1.4 W |
| Low cell | 3.4 V | 0.45 W |
| Low cell, full white | 3.4 V | 1.4 W |
| Full cell, full white | 4.2 V | 1.4 W |

The plug-in was also run on its own with a 20 ns step, because the 10 us step
of the main scenario is too coarse for the cable ringing. For that run the
cable goes in at 5 ms with the ring already on, and nothing else happens.

## Summary

| Area | Verdict |
| ---- | ------- |
| Plugging in | Fine |
| Running on battery | Fine |
| Running on USB | Fine, but draws more than 500 mA with the ring bright |
| Unplugging | Fine |

## Plugging in

| Case | VBUS peak | VSYS peak |
| ---- | --------- | --------- |
| Nominal | 5.20 V | 4.59 V |
| Low cell, full white | 5.00 V | 4.50 V |

The ringing on VBUS is gone within about 80 us. VBUS is well under the
nRF52840's 5.8 V limit on its VBUS pin, and everything behind the Schottky
sees less than 4.6 V. The 3.3 V rail does not move.

The ideal source and 0.2 ohm cable give an inrush peak of about 11.5 A for a
few microseconds, and up to 3.5 A through the ring's ferrite bead for about
20 us. A real host limits this, and both parts survive pulses like that.

## Running on battery

| Case | VSYS min | 3.3 V min | Cell current max |
| ---- | -------- | --------- | ---------------- |
| Nominal | 3.65 V | 3.30 V | 274 mA |
| Full white | 3.60 V | 3.30 V | 548 mA |
| Low cell | 3.35 V | 3.27 V | 286 mA |
| Low cell, full white | 3.29 V | 3.21 V | 587 mA |

With a low cell the LDO runs in dropout and the 3.3 V rail follows the cell,
but it stays well clear of the 2.7 V that the GNSS module and QSPI flash need.

## Running on USB

| Case | USB current, steady | USB current, peak | VBUS Schottky current, peak |
| ---- | ------------------- | ----------------- | ---------------------- |
| Nominal | 391 mA | 509 mA | 251 mA |
| Full white | 610 mA | 731 mA | 472 mA |
| Full cell, full white | 350 mA | 470 mA | 468 mA |

The peaks are during the LoRa transmission. A haptic buzz at the same time
adds about 60 mA, which keeps the VBUS Schottky at around half its 1 A rating.
The charger delivers 256 mA until the cell is full. VBUS at the connector stays above 4.84 V through the
0.2 ohm cable, and the 3.3 V rail holds at 3.30 V.

The 4.7k pull-down costs about 1 mA from USB and nothing on battery. On
battery, the only current through it is the VBUS Schottky's reverse leakage,
which does not depend on the resistor.

## Unplugging

| Case | VSYS min | 3.3 V min | Time under 3.2 V | Time under 2.7 V |
| ---- | -------- | --------- | ---------------- | ---------------- |
| Nominal | 3.29 V | 3.27 V | 0 | 0 |
| Full white | 3.19 V | 3.16 V | 11 ms | 0 |
| Low cell | 2.99 V | 2.97 V | 14 ms | 0 |
| Low cell, full white | 2.87 V | 2.85 V | 93 ms | 0 |
| Full cell, full white | 3.71 V | 3.30 V | 0 | 0 |

The load-sharing FET only turns on once VBUS has fallen about 0.55 V below
VSYS. The capacitance on VBUS drains quickly into the loads until the VBUS
Schottky stops conducting. After that only the gate pull-down drains it, which
takes a few milliseconds with 4.7k. Until the FET turns on, the cell feeds VSYS
through the Schottky across the FET.

Without that Schottky, the cell fed VSYS through the FET's body diode, which
drops about 0.65 V against the Schottky's 0.45 V. With the old 100k pull-down,
the nominal case sat at 2.93 V for 119 ms. With 4.7k and no parallel Schottky,
the low cell, full white case went under 2.7 V for 10 ms.

The low cell, full white case spends longer under 3.2 V than it did without the
parallel Schottky. VSYS sits higher during the dip, so VBUS has to drain
further before the FET turns on. The rail bottoms out at 2.85 V. The
parallel Schottky carries about 0.6 A for that time, within its 1 A rating.

## Outstanding

1. **USB current budget.** With the ring bright, the board draws more than the
   500 mA a USB 2.0 port has to supply. The CC resistors don't tell the
   firmware what the source can supply. Cap the ring's brightness on USB, or
   lower the charge current.
2. **Low priority: capacitance on VBUS.** 14.7 uF sits directly on VBUS,
   which is over the USB guideline of 10 uF. Modern USB-C chargers won't
   care, but an old port could trip its overcurrent protection.

## Caveats

The MLCCs' loss of capacitance with DC bias is not modelled. Less capacitance
on VBUS makes the unplug dip shorter. Less on VSYS and the 3.3 V rail makes it
deeper. The nRF52840's VBUS pin is assumed to draw 1 uA. See
[README.md](README.md) for the rest.
