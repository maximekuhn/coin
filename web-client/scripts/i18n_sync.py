#!/usr/bin/env python3

import argparse
import copy
import xml.etree.ElementTree as ET

# XLIFF2 namespace
NS = {"x": "urn:oasis:names:tc:xliff:document:2.0"}

# Supported locales
LOCALES = ["fr"]


def get_units(root):
    units = []
    for unit in root.findall(".//x:unit", NS):
        units.append(unit)
    return units


def get_unit_parent(root, unit):
    for parent in root.findall(".//x:file", NS):
        for child in parent.findall("x:unit", NS):
            if child is unit:
                return parent
    return None


def remove_unit(root, unit):
    parent = get_unit_parent(root, unit)
    if parent is not None:
        parent.remove(unit)


def serialize_element(el):
    return b"".join(ET.tostring(el, encoding="utf-8").split())


def source_changed(src_unit, target_unit):
    src_source = src_unit.find(".//x:source", NS)
    target_source = target_unit.find(".//x:source", NS)
    if src_source is None or target_source is None:
        return True
    return serialize_element(src_source) != serialize_element(target_source)


def update_source(src_unit, target_unit):
    src_source = src_unit.find(".//x:source", NS)
    target_segment = target_unit.find(".//x:segment", NS)
    target_source = target_segment.find("x:source", NS)

    target_segment.remove(target_source)
    target_segment.insert(0, copy.deepcopy(src_source))


def sync(src_root, target_root):
    actions = {}

    src_units = get_units(src_root)
    src_units_map = {unit.get("id"): unit for unit in src_units}

    target_units = get_units(target_root)

    units_to_remove = []

    for unit in target_units:
        unit_id = unit.get("id")
        if unit_id not in src_units_map:
            units_to_remove.append(unit)
            actions[unit_id] = "REMOVE"
            continue

        src_unit = src_units_map[unit_id]
        if source_changed(src_unit, unit):
            update_source(src_unit, unit)
            actions[unit_id] = "UPDATE SOURCE"

    for unit in units_to_remove:
        remove_unit(target_root, unit)

    return actions


def main(dry_run, verbose):
    ET.register_namespace("", "urn:oasis:names:tc:xliff:document:2.0")
    src_tree = ET.parse("src/locale/messages.xlf")
    src_root = src_tree.getroot()

    total_changes = 0

    for locale in LOCALES:
        target_tree = ET.parse(f"src/locale/messages.{locale}.xlf")
        target_root = target_tree.getroot()
        actions = sync(src_root, target_root)

        total_changes += len(actions)

        if verbose:
            print(f"Found {len(actions)} change(s) for language '{locale}'")
            for key, value in actions.items():
                print(f"* {key}: {value}")

        if dry_run:
            continue

        ET.indent(target_tree, space="  ", level=0)
        target_tree.write(
            f"src/locale/messages.{locale}.xlf",
            encoding="utf-8",
            xml_declaration=True,
        )

    if dry_run:
        print(
            f"--dry-run: Execution successful, run the program again with no dry run (--no-dry-run) to apply {total_changes} change(s)."
        )


def parse_args():
    parser = argparse.ArgumentParser(description="Sync translations with source file")
    parser.add_argument(
        "--dry-run",
        action=argparse.BooleanOptionalAction,
        help="Check for changes without applying them",
    )
    parser.add_argument(
        "--verbose",
        action=argparse.BooleanOptionalAction,
        help="Print changes to apply",
    )
    return parser.parse_args()


if __name__ == "__main__":
    args = parse_args()
    main(args.dry_run, args.verbose)
