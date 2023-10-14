#!/usr/bin/env python3
"""
Used to generate the unicode codepoint index used by whonicode.
"""

from collections import defaultdict
import os
import sys
from urllib.request import urlopen
import re
from dataclasses import dataclass, field


int_re = re.compile("^[0-9]+$")

"""
Field
	

Name
	

Status
	

Explanation
0 	Code value 	normative 	Code value in 4-digit hexadecimal format.
1 	Character name 	normative 	These names match exactly the names published in Chapter 7 of the Unicode Standard, Version 2.0, except for the two additional characters.
2 	General category 	normative / informative
(see below) 	This is a useful breakdown into various "character types" which can be used as a default categorization in implementations. See below for a brief explanation.
3 	Canonical combining classes 	normative 	The classes used for the Canonical Ordering Algorithm in the Unicode Standard. These classes are also printed in Chapter 4 of the Unicode Standard.
4 	Bidirectional category 	normative 	See the list below for an explanation of the abbreviations used in this field. These are the categories required by the Bidirectional Behavior Algorithm in the Unicode Standard. These categories are summarized in Chapter 3 of the Unicode Standard.
5 	Character decomposition mapping 	normative 	In the Unicode Standard, not all of the mappings are full (maximal) decompositions. Recursive application of look-up for decompositions will, in all cases, lead to a maximal decomposition. The decomposition mappings match exactly the decomposition mappings published with the character names in the Unicode Standard.
6 	Decimal digit value 	normative 	This is a numeric field. If the character has the decimal digit property, as specified in Chapter 4 of the Unicode Standard, the value of that digit is represented with an integer value in this field
7 	Digit value 	normative 	This is a numeric field. If the character represents a digit, not necessarily a decimal digit, the value is here. This covers digits which do not form decimal radix forms, such as the compatibility superscript digits
8 	Numeric value 	normative 	This is a numeric field. If the character has the numeric property, as specified in Chapter 4 of the Unicode Standard, the value of that character is represented with an integer or rational number in this field. This includes fractions as, e.g., "1/5" for U+2155 VULGAR FRACTION ONE FIFTH Also included are numerical values for compatibility characters such as circled numbers.
8 	Mirrored 	normative 	If the character has been identified as a "mirrored" character in bidirectional text, this field has the value "Y"; otherwise "N". The list of mirrored characters is also printed in Chapter 4 of the Unicode Standard.
10 	Unicode 1.0 Name 	informative 	This is the old name as published in Unicode 1.0. This name is only provided when it is significantly different from the Unicode 3.0 name for the character.
11 	10646 comment field 	informative 	This is the ISO 10646 comment field. It is in parantheses in the 10646 names list.
12 	Uppercase mapping 	informative 	Upper case equivalent mapping. If a character is part of an alphabet with case distinctions, and has an upper case equivalent, then the upper case equivalent is in this field. See the explanation below on case distinctions. These mappings are always one-to-one, not one-to-many or many-to-one. This field is informative.
13 	Lowercase mapping 	informative 	Similar to Uppercase mapping
14 	Titlecase mapping

"""

"""
struct unicode_datum {
    codepoint: i32,             // unicode codepoint
    category: uc_category,      // unicode category
    uc_value_idx: i32,          // references the value in the unicode_values array
    uppercase_codepoint: i32,   // uppercase conversion
    lowercase_codepoint: i32,   // lowercase conversion
}
"""
@dataclass(unsafe_hash=True)
class UnicodeDatumVal:
    """
    struct unicode_datum_values {
        digit_value: i32,           // digit value
        numeric_value_num: i32,     // numerator of value
        numeric_value_den: i32,     // denomintor of value
    }
    """
    digit: int
    num: int
    den: int

def main(argv: list[str]):
    data = []
    values: dict[UnicodeDatumVal, int] = dict()
    strings = []

    for line in iter_unicode_str_lines():
        assert len(line) == 15
        num, name, cat, _, _, _, _, digit_val, val, _, _, _, upper, lower, _ = line        
        num = int(num, 16)

        val = get_val(digit_val, val)
        if val is not None and val not in values:
            values[val] = len(values)
        val_id = values.get(val, -1)
        
        strings.append(name.lower())

        data.append((num, cat, val_id, *upper_lower_cdpt(upper, lower)))

    print(f"// {len(data)} codepoints, carrying {len(values)} distinct values")

    str_vals = "\n    ".join(
        "UnicodeDatum {{cdpt: {}, cat: Category::{}, val: {}, upper: {}, lower: {}}},".format(*d)
        for d in data
    )

    val_vals = "\n    ".join(
        "UnicodeDatumValue {{digit: {}, num: {}, den: {}}},".format(v.digit, v.num, v.den)
        for v, _ in sorted(values.items(), key=lambda i: i[1])
    )

    if '--rs' in argv:

        print(f"""
// unicode data:
pub static UNICODE_DATA: [UnicodeDatum; {len(data)}] = [
    {str_vals}
];

// list of codepoint values (compressed)
pub static UNICODE_VALUES: [UnicodeDatumValue; {len(values)}] = [
    {val_vals}
];

""")
    
    if '--strings' in argv:

        with open("src/ucd_strings.txt", 'wb') as f:
            f.write(b'\x00'.join(
                f.encode('utf-8') for f in strings
            ) + b'\x00')

    return 0


def get_val(digit_val: str, val: str):
    """
    struct unicode_datum_values {
        digit_value: i32,           // digit value
        numeric_value_num: i32,     // numerator of value
        numeric_value_den: i32,     // denomintor of value
    }
    """
    if not digit_val and not val:
        return None
    if not digit_val:
        digit_val = "-1"
    if '/' in val:
        num, den = val.split('/')
    else:
        num = val
        den = "1"
    num = int(num)
    den = int(den)
    return UnicodeDatumVal(int(digit_val), num, den)


def upper_lower_cdpt(lower: str, upper: str):
    yield int(lower, 16) if lower else -1
    yield int(upper, 16) if upper else -1


def get_unicode_str():
    # use cached copy for dev
    if os.path.exists('UnicodeData.txt'):
        with open('UnicodeData.txt', 'r') as f:
            return f.read()
    # use web
    with urlopen("https://www.unicode.org/Public/UCD/latest/ucd/UnicodeData.txt") as ucd:
        return ucd.read().decode('utf-8')


def iter_unicode_str_lines():
    for line in get_unicode_str().split('\n'):
        if len(line) == 0:
            continue
        yield line.split(';')


if __name__ == '__main__':
    sys.exit(main(sys.argv))
print("what")