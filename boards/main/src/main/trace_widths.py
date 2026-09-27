#!/usr/bin/env python3
"""Trace width calculations behind the net classes and SmartCompass.kicad_dru.

Two independent questions set a trace's width:

* Impedance, for the GNSS antenna feed (50 ohm single-ended) and the USB pair
  (90 ohm differential). This depends only on geometry and the stackup, so the
  widths must be recalculated if the stackup changes.
* Current, for the power nets. IPC-2221 relates current, temperature rise and
  copper cross-section.

Models:

* Single-ended microstrip: Hammerstad-Jensen with the Hammerstad thickness
  correction - the model KiCad's own calculator uses.
* Edge-coupled microstrip: Wadell's approximation,
  Zdiff = 2 Z0 (1 - 0.48 exp(-0.96 s / h)). Rougher than the single-ended
  model, which is acceptable for full-speed USB.
* Solder mask is not modelled; a fixed allowance is subtracted instead, since
  mask over an outer trace typically pulls Z down by 1-2 ohm.

For the final check before ordering, use the fab's own impedance calculator
against the stackup by name.
"""

import argparse
import math
from dataclasses import dataclass

ETA0 = 376.730313668
OZ_TO_MM = 0.0348
MIL_TO_MM = 0.0254

# Ohms that a solder mask layer over an outer trace typically removes.
MASK_DROP_OHM = 1.5


@dataclass(frozen=True)
class Stackup:
    description: str
    dielectric_mm: float  # outer copper to the first inner layer
    er: float
    outer_copper_mm: float
    inner_copper_mm: float


STACKUPS = {
    # JLCPCB 4-layer 1.6 mm, the default "no requirement" build. Figures from
    # https://jlcpcb.com/impedance
    "jlc-7628": Stackup("JLC04161H-7628", 0.2104, 4.4, 0.035, 0.0152),
    "jlc-3313": Stackup("JLC04161H-3313", 0.0994, 4.1, 0.035, 0.0152),
    "jlc-2116": Stackup("JLC04161H-2116", 0.1164, 4.16, 0.035, 0.0152),
    "jlc-1080": Stackup("JLC04161H-1080", 0.0764, 3.91, 0.035, 0.0152),
    # KiCad's placeholder stackup for a new 4-layer board.
    "kicad-default": Stackup("KiCad default 4-layer", 0.1, 4.5, 0.035, 0.035),
}


def _eeff_and_z_air(u: float, er: float) -> tuple[float, float]:
    a = (
        1
        + math.log((u**4 + (u / 52) ** 2) / (u**4 + 0.432)) / 49
        + math.log(1 + (u / 18.1) ** 3) / 18.7
    )
    b = 0.564 * ((er - 0.9) / (er + 3)) ** 0.053
    eeff = (er + 1) / 2 + (er - 1) / 2 * (1 + 10 / u) ** (-a * b)
    f = 6 + (2 * math.pi - 6) * math.exp(-((30.666 / u) ** 0.7528))
    z_air = ETA0 / (2 * math.pi) * math.log(f / u + math.sqrt(1 + (2 / u) ** 2))
    return eeff, z_air


def microstrip_z0(width_mm: float, s: Stackup) -> float:
    """Single-ended microstrip impedance, no solder mask."""
    h, t, er = s.dielectric_mm, s.outer_copper_mm, s.er
    u = width_mm / h
    t_n = t / h
    du1 = t_n / math.pi * math.log(
        1 + 4 * math.e / (t_n * (1 / math.tanh(math.sqrt(6.517 * u))) ** 2)
    )
    dur = 0.5 * (1 + 1 / math.cosh(math.sqrt(er - 1))) * du1
    _, z_air_1 = _eeff_and_z_air(u + du1, 1.0)
    eeff_r, z_air_r = _eeff_and_z_air(u + dur, er)
    eeff = eeff_r * (z_air_1 / z_air_r) ** 2
    return z_air_r / math.sqrt(eeff)


def coupled_zdiff(width_mm: float, gap_mm: float, s: Stackup) -> float:
    """Edge-coupled microstrip differential impedance, no solder mask."""
    z0 = microstrip_z0(width_mm, s)
    return 2 * z0 * (1 - 0.48 * math.exp(-0.96 * gap_mm / s.dielectric_mm))


def solve_width(target_ohm: float, impedance, lo: float = 0.02, hi: float = 5.0) -> float:
    """Width giving target_ohm; impedance falls monotonically with width."""
    for _ in range(100):
        mid = (lo + hi) / 2
        if impedance(mid) > target_ohm:
            lo = mid
        else:
            hi = mid
    return (lo + hi) / 2


def ipc2221_width_mm(current_a: float, rise_c: float, copper_mm: float, internal: bool) -> float:
    """IPC-2221 minimum width: I = k * dT^0.44 * A^0.725, A in mil^2."""
    k = 0.024 if internal else 0.048
    area_mil2 = (current_a / (k * rise_c**0.44)) ** (1 / 0.725)
    thickness_mil = copper_mm / MIL_TO_MM
    return area_mil2 / thickness_mil * MIL_TO_MM


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="Trace widths for impedance and current on a given stackup."
    )
    parser.add_argument(
        "--stackup",
        choices=sorted(STACKUPS),
        default="jlc-7628",
        help="Stackup to calculate against (default: jlc-7628)",
    )
    parser.add_argument("--z0", type=float, default=50.0, help="Single-ended target, ohm")
    parser.add_argument("--zdiff", type=float, default=90.0, help="Differential target, ohm")
    parser.add_argument(
        "--gap",
        type=float,
        default=0.2,
        help="Differential pair gap in mm (default 0.2, the Default netclass clearance)",
    )
    parser.add_argument("--rise", type=float, default=10.0, help="Allowed temperature rise, degC")
    return parser


def main() -> None:
    args = build_parser().parse_args()
    s = STACKUPS[args.stackup]
    mask_note = f"with {MASK_DROP_OHM} ohm solder-mask allowance"

    print(f"Stackup: {s.description}")
    print(
        f"  h = {s.dielectric_mm} mm, er = {s.er}, outer Cu = {s.outer_copper_mm * 1000:.0f} um, "
        f"inner Cu = {s.inner_copper_mm * 1000:.1f} um"
    )

    print("\nSingle-ended microstrip, no mask")
    w_bare = solve_width(args.z0, lambda w: microstrip_z0(w, s))
    for w in sorted({round(w_bare + d, 2) for d in (-0.06, -0.04, -0.02, 0, 0.02, 0.04)}):
        if w > 0:
            print(f"  w = {w:.2f} mm   Z0 = {microstrip_z0(w, s):5.1f} ohm")
    w_se = solve_width(args.z0 + MASK_DROP_OHM, lambda w: microstrip_z0(w, s))
    print(f"  -> {args.z0:.0f} ohm: {w_bare:.3f} mm bare, {w_se:.3f} mm {mask_note}")

    print(f"\nEdge-coupled pair at {args.gap} mm gap, no mask")
    w_pair = solve_width(args.zdiff + MASK_DROP_OHM, lambda w: coupled_zdiff(w, args.gap, s))
    for w in sorted({round(w_pair + d, 2) for d in (-0.04, -0.02, 0, 0.02, 0.04)}):
        if w > 0:
            print(f"  w = {w:.2f} mm   Zdiff ~ {coupled_zdiff(w, args.gap, s):5.1f} ohm")
    print(f"  -> {args.zdiff:.0f} ohm: {w_pair:.3f} mm {mask_note}")

    print(f"\nIPC-2221 minimum width for a {args.rise:.0f} degC rise")
    print("  current   outer      inner")
    for amps in (0.25, 0.5, 1.0, 1.5, 2.0):
        outer = ipc2221_width_mm(amps, args.rise, s.outer_copper_mm, internal=False)
        inner = ipc2221_width_mm(amps, args.rise, s.inner_copper_mm, internal=True)
        print(f"  {amps:4.2f} A   {outer:5.2f} mm   {inner:5.2f} mm")
    print(
        "  (IPC-2221's internal-layer figures are known to be conservative; "
        "IPC-2152 puts inner layers close to outer.)"
    )


if __name__ == "__main__":
    main()
