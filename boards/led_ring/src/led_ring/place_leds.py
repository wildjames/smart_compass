import math

from kipy import KiCad
from kipy.board_types import Field
from kipy.geometry import Angle, Vector2

LED_MPN = "IN-PI15TAT5R5G5B"
CX, CY = 100, 100
LED_RADIUS = 32.5
CAP_MPN = "CL05B104KO5VPNC"
CAP_RADIUS = 30.5

def get_location_in_circle(cx, cy, radius, index, total):
    arc = 2 * math.pi / total
    x = cx + (radius * math.cos(arc * index))
    y = cy + (radius * math.sin(arc * index))
    return Vector2.from_xy_mm(x, y)


def get_orientation_in_circle(index, total, offset = 0):
    arc = 360.0 / total
    angle = arc * -index + offset
    return Angle.from_degrees(angle)


def field_value(fp, name):
    return next((t.text.value for t in fp.texts_and_fields
                 if isinstance(t, Field) and t.name == name), None)


def find_matching_mpn_footprints(footprints, target_mpn):
    matches = [
        f for f in footprints
        if field_value(f, "MPN") == target_mpn
    ]
    return sorted(matches, key=lambda f: int(f.reference_field.text.value[1:]))


def main():
    board = KiCad().get_board()
    footprints = board.get_footprints()

    led_footprints = find_matching_mpn_footprints(footprints, LED_MPN)
    print(f"Found {len(led_footprints)} matching footprints for {LED_MPN}")

    # C1 is different, so don't include it in the setting
    cap_footprints = find_matching_mpn_footprints(footprints, CAP_MPN)[1:]
    print(f"Found {len(cap_footprints)} matching footprints for {CAP_MPN}")

    for i, f in enumerate(led_footprints):
        new_position = get_location_in_circle(CX, CY, LED_RADIUS, i, len(led_footprints))
        new_rotation = get_orientation_in_circle(i, len(led_footprints))
        f.position = new_position
        f.orientation = new_rotation

    for i, f in enumerate(cap_footprints):
        new_position = get_location_in_circle(CX, CY, CAP_RADIUS, i, len(cap_footprints))
        new_rotation = get_orientation_in_circle(i, len(cap_footprints), 90.0)
        f.position = new_position
        f.orientation = new_rotation

    board.update_items(led_footprints)
    board.update_items(cap_footprints)
