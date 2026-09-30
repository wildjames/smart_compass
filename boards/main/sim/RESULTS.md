# Main board simulation results

Results of the transient scenario in [README.md](README.md), run on 2026-09-30.
Since the previous run, the 10 uF capacitor by the MCU's VBUS pin has come off
the board, and the LED ring's load now pulses at the LEDs' 1 kHz PWM instead of
drawing steady power.

The run before that, on 2026-09-29, followed three changes to the power path:
the pull-down on the load-sharing FET's gate went from 100k to 4.7k, the VBUS
Schottky went from a 0.5 A MBR0540 to a 1 A SS1040-AU, and a second SS1040-AU
went in across the load-sharing FET, from the cell to VSYS.

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

`RING_W` is the ring's average draw. At 0.45 W the ring pulses between
0.05 W and the full-white 1.4 W, on for 30% of each millisecond. Full white
is steady.

The plug-in was also run on its own with a 20 ns step, because the 10 us step
of the main scenario is too coarse for the cable ringing. For that run the
cable goes in at 5 ms with the ring already on, and nothing else happens.

## Summary

| Area | Verdict |
| ---- | ------- |
| Plugging in | Fine |
| Running on battery | Fine |
| Running on USB | Fine, but the ring's pulses take it over 500 mA at any brightness |
| Unplugging | Fine |

## Plugging in

| Case | VBUS peak | VSYS peak |
| ---- | --------- | --------- |
| Nominal | 5.20 V | 4.64 V |
| Low cell, full white | 4.95 V | 4.50 V |

The ringing on VBUS is gone within about 55 us. After that VBUS steps by about
60 mV with each ring pulse, from the cable's resistance. VBUS is well under
the nRF52840's 5.8 V limit on its VBUS pin, and everything behind the Schottky
sees less than 4.7 V. The 3.3 V rail does not move.

The ideal source and 0.2 ohm cable give an inrush peak of about 7.9 A for a
few microseconds, down from 11.5 A with the 10 uF capacitor on VBUS. Up to
3.4 A goes through the ring's ferrite bead, over its 1 A rating for about
30 us. A real host limits this, and both parts survive pulses like that.

## Running on battery

| Case | VSYS min | 3.3 V min | Cell current max |
| ---- | -------- | --------- | ---------------- |
| Nominal | 3.60 V | 3.30 V | 548 mA |
| Full white | 3.60 V | 3.30 V | 548 mA |
| Low cell | 3.29 V | 3.21 V | 587 mA |
| Low cell, full white | 3.29 V | 3.21 V | 587 mA |

The nominal cases now reach the same minimums and peaks as full white,
because each ring pulse is at full-white power. On average they draw what
they did before.

With a low cell the LDO runs in dropout and the 3.3 V rail follows the cell,
but it stays well clear of the 2.7 V that the GNSS module and QSPI flash need.

The ring's pulses put a 1 kHz ripple on the supply: 110 mV peak to peak on
`Vdrive` (120 mV with a low cell) and 70 to 80 mV on VSYS. There is no
ringing, and the ferrite bead peaks at 430 mA, within its 1 A rating. In the
simulation the 3.3 V rail only sees it with a low cell, where the LDO is in
dropout, and then only 4 mV. The LDO model has no supply rejection. The
RT9080's 75 dB at 1 kHz would take 80 mV on VSYS down to about 15 uV on the
3.3 V rail.

## Running on USB

| Case | USB current, average | USB current, peak | VBUS Schottky current, peak |
| ---- | -------------------- | ----------------- | --------------------------- |
| Nominal | 393 mA | 731 mA | 472 mA |
| Full white | 610 mA | 731 mA | 472 mA |
| Full cell, full white | 350 mA | 470 mA | 468 mA |

In the nominal case each ring pulse takes the USB current to 610 mA. The
731 mA peaks are a ring pulse during the LoRa transmission. A haptic buzz at
the same time adds about 60 mA, which keeps the VBUS Schottky at around half
its 1 A rating. The charger delivers 256 mA until the cell is full. VBUS at
the connector stays above 4.84 V through the 0.2 ohm cable, and the 3.3 V
rail holds at 3.30 V.

The 4.7k pull-down costs about 1 mA from USB and nothing on battery. On
battery, the only current through it is the VBUS Schottky's reverse leakage,
which does not depend on the resistor.

## Unplugging

| Case | VSYS min | 3.3 V min | Time under 3.2 V | Time under 2.7 V |
| ---- | -------- | --------- | ---------------- | ---------------- |
| Nominal | 3.19 V | 3.17 V | 0.6 ms | 0 |
| Full white | 3.19 V | 3.16 V | 3.6 ms | 0 |
| Low cell | 2.87 V | 2.85 V | 6.2 ms | 0 |
| Low cell, full white | 2.87 V | 2.85 V | 16.5 ms | 0 |
| Full cell, full white | 3.71 V | 3.30 V | 0 | 0 |

The load-sharing FET only turns on once VBUS has fallen about 0.55 V below
VSYS. The capacitance on VBUS drains quickly into the loads until the VBUS
Schottky stops conducting. After that only the gate pull-down drains it, which
takes a few milliseconds with 4.7k. Until the FET turns on, the cell feeds VSYS
through the Schottky across the FET.

The dips are much shorter than in the previous run (16.5 ms under 3.2 V for
the low cell, full white case, against 93 ms), because there is less
capacitance on VBUS to drain. The nominal and low cell cases now bottom out as
low as full white, because the ring's pulses are at full-white power. They
spend less time there because the ring is dark for most of each millisecond.

Without the parallel Schottky, the cell fed VSYS through the FET's body diode,
which drops about 0.65 V against the Schottky's 0.45 V. With the old 100k
pull-down, the nominal case sat at 2.93 V for 119 ms. With 4.7k and no
parallel Schottky, the low cell, full white case went under 2.7 V for 10 ms.

The rail bottoms out at 2.85 V. The parallel Schottky carries about 0.6 A for
that time, within its 1 A rating.

## Outstanding

1. **USB current budget.** On USB the board draws more than the 500 mA a
   USB 2.0 port has to supply, in pulses at any ring brightness and on
   average with the ring bright. The CC resistors don't tell the firmware
   what the source can supply. The height of a pulse depends on how many
   colour channels are lit, not on their brightness, so dimming the ring in
   firmware lowers the average but not the peaks. Lighting fewer channels at
   once, or lowering the charge current, lowers both.

## Caveats

The ring's LEDs are assumed to switch in step. Real ones drift in and out of
step, so the full-height pulses come and go rather than arriving every
millisecond.

The MLCCs' loss of capacitance with DC bias is not modelled. Less capacitance
on VBUS makes the unplug dip shorter. Less on VSYS and the 3.3 V rail makes it
deeper. The nRF52840's VBUS pin is assumed to draw 1 uA. See
[README.md](README.md) for the rest.
