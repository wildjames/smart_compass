# Outstanding component and sourcing work

Every part on all three boards has been checked against the Digi-Key catalogue,
and each one against its manufacturer reference circuit where there is one. The
results — manufacturer part number, Digi-Key part number, key parameters, and a
`Sourcing_Note` on anything with a problem — are written into the schematics as
BOM properties, so they travel with a BOM export and reach whoever places the
order.

## 2. Footprints and library work

### boost inductor needs a footprint

| Where | Part | Land currently used |
| --- | --- | --- |
| Display board, 10 µH | Bourns `SRN3015BTA-100M` | `L_Taiyo-Yuden_NR-30xx` — KiCad has no `SRN3015` footprint |

---

## 7. Documentation and missing files

### The FFC conductor order belongs on the assembly drawing

Both board-to-board links use the same 10-way `FH12-10S-0.5SH(55)` at both ends,
so one cable part serves both — but **neither pinout is a palindrome**, and a
cable fitted the wrong way round puts `Vdrive` onto ground pins. The ring link's
pin 4 is `RING_EN`, which makes getting the orientation right more consequential
than it was when that way was a spare supply conductor. Both pinouts are in
[README.md](README.md).
