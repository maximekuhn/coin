#!/usr/bin/env python3

import argparse
import xml.etree.ElementTree as ET
import sys

# XLIFF2 namespace
NS = {"x": "urn:oasis:names:tc:xliff:document:2.0"}

# Supported locales
LOCALES = ["fr"]


def die(message):
    print(f"ERROR: {message}", file=sys.stderr)
    sys.exit(1)


def read_stdin():
    data = sys.stdin.read().strip()
    if not data:
        die("No input received on stdin.")
    return data


def parse_llm_units_from_stdin():
    xml_text = read_stdin()
    try:
        wrapped = f"<root>{xml_text}</root>"
        root = ET.fromstring(wrapped)
    except ET.ParseError as e:
        die(f"Failed to parse XML from stdin: {e}")

    units = []
    for elem in root:
        if elem.tag != "unit":
            die(f"Unexpected top-level element <{elem.tag}>; only <unit> allowed.")
        units.append(elem)

    if not units:
        die("No <unit> elements found in LLM output.")

    return units


def validate_unit_has_id_attribute(unit):
    if not unit.get("id"):
        die("<unit> missing required id attribute.")


def validate_unit_has_segment(unit):
    if not unit.find("segment"):
        die(f"<unit> '{unit.get('id')}' has no <segment>")


def validate_unit_has_target(unit):
    target = unit.find("segment/target")
    if target is None:
        die(f"<unit> '{unit.get('id')}' has no <target>")
    if target.text.strip() == "__TRANSLATE__":
        die(f"<unit> '{unit.get('id')}' has placeholder __TRANSLATE__ <target>")


def validate_units(units):
    for unit in units:
        validate_unit_has_id_attribute(unit)
        validate_unit_has_segment(unit)
        validate_unit_has_target(unit)


def get_trusted_units(root):
    units = []
    for unit in root.findall(".//x:unit", NS):
        units.append(unit)
    return units


def remove_target(unit):
    segment = unit.find(".//x:segment", NS)
    for child in segment:
        if child.tag.endswith("target"):
            segment.remove(child)
            return True
    return False


def add_target(unit, target):
    segment = unit.find(".//x:segment", NS)
    new_target = ET.Element(f"{{{NS['x']}}}target")

    new_target.text = target.text
    for child in target:
        new_target.append(child)
    if target.tail:
        new_target.tail = target.tail

    segment.append(new_target)


def get_file_element(target_root):
    file_elem = target_root.find(".//x:file", NS)
    if file_elem is None:
        die("No <file> element found in target XLIFF document")
    return file_elem


def apply_xliff_namespace(elem, ns):
    elem.tag = f"{{{ns}}}{elem.tag}"
    for child in list(elem):
        apply_xliff_namespace(child, ns)


def main(locale_code, dry_run, verbose):
    ET.register_namespace("", "urn:oasis:names:tc:xliff:document:2.0")
    target_tree = ET.parse("src/locale/messages.{}.xlf".format(locale_code))
    target_root = target_tree.getroot()

    llm_units = parse_llm_units_from_stdin()
    validate_units(llm_units)

    target_units = get_trusted_units(target_root)
    target_units_map = {unit.get("id"): unit for unit in target_units}
    target_file_elem = get_file_element(target_root)

    actions = {}

    for llm_unit in llm_units:
        llm_unit_id = llm_unit.get("id")

        if llm_unit_id not in target_units_map:
            # Create new <unit> in target file and add <target> from LLM.
            new_unit = ET.fromstring(ET.tostring(llm_unit))
            apply_xliff_namespace(new_unit, NS["x"])
            target_file_elem.append(new_unit)
            actions[llm_unit_id] = "NEW"
            continue

        # <unit> already exists in target file, update or add <target> if needed.
        target_unit = target_units_map[llm_unit_id]
        had_target = remove_target(target_unit)
        add_target(target_unit, llm_unit.find("segment/target"))

        actions[llm_unit_id] = "UPDATE" if had_target else "ADD"

    if verbose:
        print(f"{len(llm_units)} translation(s)")
        for key, value in actions.items():
            print(f"* {key}: {value}")

    if dry_run:
        print(
            "--dry-run: Execution successful, run the program again with no dry run (--no-dry-run) to save changes."
        )
        return

    ET.indent(target_tree, space="  ", level=0)
    target_tree.write(
        f"src/locale/messages.{locale_code}.xlf",
        encoding="utf-8",
        xml_declaration=True,
    )


def parse_args():
    parser = argparse.ArgumentParser(description="Import translated XLIFF units")
    parser.add_argument(
        "--target",
        required=True,
        choices=LOCALES,
        help="Target locale code (e.g. fr)",
    )
    parser.add_argument(
        "--dry-run",
        action=argparse.BooleanOptionalAction,
        help="Validate translations without modifying files",
    )
    parser.add_argument(
        "--verbose",
        action=argparse.BooleanOptionalAction,
        help="Print changes to apply",
    )
    return parser.parse_args()


if __name__ == "__main__":
    args = parse_args()
    main(args.target, args.dry_run, args.verbose)
