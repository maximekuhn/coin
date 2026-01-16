#!/usr/bin/env python3

import xml.etree.ElementTree as ET
import copy
import argparse

# XLIFF2 namespace
NS = {"x": "urn:oasis:names:tc:xliff:document:2.0"}

# Supported locales
LOCALES = {"fr": "French (France)"}


def generate_prompt(locale_code, existing_translations):
    existing_block = ""
    if len(existing_translations) > 0:
        existing_block += "For reference, here are the existing translations for the target locale (showing text only):\n"
        existing_block += "\n".join(f"- {t}" for t in existing_translations)

    return """
You are an excellent professional software translator, specialized in UI/UX and web applications.

Your task is to translate the provided XLIFF2 XML units into the following locale:

TARGET LOCALE: {}

You will receive a list of <unit> XML nodes. Each unit contains:
- contextual notes (location, description)
- a <segment> with a <source> text in English
- a <target> containing the placeholder "__TRANSLATE__"

IMPORTANT RULES (MANDATORY):
1. You must ONLY replace the text "__TRANSLATE__" inside each <segment><target>.
2. Do NOT modify anything else:
   - Do NOT change <unit>, <segment>, <source>, <notes>, attributes, or formatting
   - Do NOT reorder nodes
3. You MUST preserve all Angular / XLIFF elements exactly as-is:
   - <ph>, <pc>, interpolation markers, spacing, punctuation, and placeholders
   - Do NOT translate or alter interpolation content (e.g. {{ user()!.name }})
4. The translation must be natural, idiomatic, and appropriate for a modern expense-sharing web application.
5. Use the notes (description & location) to choose the most accurate translation.
6. Be consistent across similar terms (e.g. Login / Logout, Group, Expense, Balance).

OUTPUT FORMAT (CRITICAL):
- Output ONLY the resulting XML
- Do NOT add explanations, comments, or markdown outside the XML
- The output must be directly copy-pasteable
- Wrap the full output in a single code block

{}

Here are the XML units to translate:

    """.format(
        LOCALES[locale_code], existing_block
    )


def strip_namespaces(elem):
    if "}" in elem.tag:
        elem.tag = elem.tag.split("}", 1)[1]
    for child in elem:
        strip_namespaces(child)


def get_segment(unit):
    return unit.find("x:segment", NS)


def get_source(unit):
    return get_segment(unit).find("x:source", NS)


def get_target(unit):
    segment = get_segment(unit)

    target = segment.find("x:target", NS)
    if target is None:
        return None

    if (target.text or "").strip() == "":
        return None

    return target


def has_target(unit):
    return get_target(unit) is not None


def should_translate(target_unit, src_unit):
    if target_unit is None or not has_target(target_unit):
        return True

    target_source = get_source(target_unit).text
    src_source = get_source(src_unit).text
    return target_source != src_source


def add_target(unit):
    segment = unit.find("segment")
    target = ET.SubElement(segment, "target")
    target.text = "__TRANSLATE__"


def extract_target_text(target_elem):
    return "".join(target_elem.itertext()).strip()


def extract_existing_translations(target_root):
    translations = set()
    for unit in target_root.findall(".//x:unit", NS):
        segment = unit.find("x:segment", NS)
        target = segment.find("x:target", NS)
        if target is None:
            continue

        text = extract_target_text(target)
        if text:
            translations.add(text)

    return translations


def main(locale_code):
    src_tree = ET.parse("src/locale/messages.xlf")
    src_root = src_tree.getroot()

    target_tree = ET.parse("src/locale/messages.{}.xlf".format(locale_code))
    target_root = target_tree.getroot()

    print(generate_prompt(locale_code, extract_existing_translations(target_root)))

    target_units = {}
    for unit in target_root.findall(".//x:unit", NS):
        unit_id = unit.get("id")
        target_units[unit_id] = unit

    for src_unit in src_root.findall(".//x:unit", NS):
        unit_id = src_unit.get("id")
        target_unit = target_units.get(unit_id)

        if should_translate(target_unit, src_unit):
            unit_copy = copy.deepcopy(src_unit)
            strip_namespaces(unit_copy)
            add_target(unit_copy)
            ET.indent(unit_copy, space="  ")
            print(ET.tostring(unit_copy, encoding="unicode"))


def parse_args():
    parser = argparse.ArgumentParser(
        description="Generate an LLM prompt for missing XLIFF translations"
    )

    parser.add_argument(
        "--target",
        required=True,
        choices=LOCALES.keys(),
        help="Target locale code (e.g. fr)",
    )

    return parser.parse_args()


if __name__ == "__main__":
    args = parse_args()
    main(args.target)
