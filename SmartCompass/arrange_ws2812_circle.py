#!/usr/bin/env python3

from __future__ import annotations

import argparse
import csv
import math
import sys


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description=(
            "Generate KiCad-friendly positions and rotations for WS2812B2020 LEDs "
            "arranged on a circle."
        )
    )
    parser.add_argument("center_x", type=float, help="Circle center X coordinate in mm")
    parser.add_argument("center_y", type=float, help="Circle center Y coordinate in mm")
    parser.add_argument("led_count", type=int, help="Number of LEDs to place on the circle")
    parser.add_argument(
        "--radius",
        type=float,
        required=False,
        help="Circle radius in mm",
    )
    parser.add_argument(
        "--diameter",
        type=float,
        required=False,
        help="Circle diameter in mm",
    )
    parser.add_argument(
        "--start-angle",
        type=float,
        default=0.0,
        help="Starting angle in degrees for the first LED, measured from +X (default: 0)",
    )
    parser.add_argument(
        "--placement-direction",
        choices=("ccw", "cw"),
        default="cw",
        help="Direction used to step around the circle (default: cw)",
    )
    parser.add_argument(
        "--tangent-direction",
        choices=("ccw", "cw"),
        default="ccw",
        help="Which tangent direction each LED should face along the circle (default: cw)",
    )
    parser.add_argument(
        "--reference-prefix",
        default="D",
        help="Reference designator prefix for output labels (default: D)",
    )
    parser.add_argument(
        "--start-index",
        type=int,
        default=0,
        help="Reference index for the first LED (default: 0)",
    )
    parser.add_argument(
        "--format",
        choices=("csv", "kicad"),
        default="csv",
        help="Output format: CSV table or KiCad at-snippet (default: csv)",
    )
    return parser


def normalize_angle(angle_degrees: float) -> float:
    normalized = angle_degrees % 360.0
    if math.isclose(normalized, 360.0, abs_tol=1e-9):
        return 0.0
    return normalized


def generate_led_rows(args: argparse.Namespace):
    if args.led_count <= 0:
        raise ValueError("led_count must be greater than zero")

    radius = None
    if args.radius:
        radius = args.radius
    if args.diameter:
        radius = args.diameter / 2.0

    if not radius or radius <= 0:
        raise ValueError("radius must be greater than zero")

    step = 360.0 / args.led_count
    if args.placement_direction == "cw":
        step *= -1.0

    tangent_offset = 90.0 if args.tangent_direction == "ccw" else -90.0

    for offset in range(args.led_count):
        reference = f"{args.reference_prefix}{args.start_index + offset}"
        angle = args.start_angle + (offset * step)
        angle_radians = math.radians(angle)
        x_pos = args.center_x + (radius * math.cos(angle_radians))
        y_pos = args.center_y + (radius * math.sin(angle_radians))
        rotation = normalize_angle(tangent_offset - angle)
        yield {
            "reference": reference,
            "x_mm": round(x_pos, 3),
            "y_mm": round(y_pos, 3),
            "rotation_deg": round(rotation, 3),
        }


def write_csv(rows) -> None:
    writer = csv.DictWriter(sys.stdout, fieldnames=["reference", "x_mm", "y_mm", "rotation_deg"])
    writer.writeheader()
    writer.writerows(rows)


def write_kicad(rows) -> None:
    for row in rows:
        print(
            f'{row["reference"]}: (at {row["x_mm"]:.3f} {row["y_mm"]:.3f} {row["rotation_deg"]:.3f})'
        )


def main() -> int:
    parser = build_parser()
    args = parser.parse_args()

    try:
        rows = list(generate_led_rows(args))
    except ValueError as error:
        parser.error(str(error))

    if args.format == "csv":
        write_csv(rows)
    else:
        write_kicad(rows)

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
