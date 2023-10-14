// The values in this field are abbreviations for the following. Some of the values are normative, and some are informative. For more information, see the Unicode Standard.

// Note: the standard does not assign information to control characters (except for certain cases in the Bidirectional Algorithm). Implementations will generally also assign categories to certain control characters, notably CR and LF, according to platform conventions.

#[derive(Debug)]
pub enum Category {
    // Normative Categories
    Lu,     // Letter, Uppercase
    Ll,     // Letter, Lowercase
    Lt,     // Letter, Titlecase
    Mn,     // Mark, Non-Spacing
    Mc,     // Mark, Spacing Combining
    Me,     // Mark, Enclosing
    Nd,     // Number, Decimal Digit
    Nl,     // Number, Letter
    No,     // Number, Other
    Zs,     // Separator, Space
    Zl,     // Separator, Line
    Zp,     // Separator, Paragraph
    Cc,     // Other, Control
    Cf,     // Other, Format
    Cs,     // Other, Surrogate
    Co,     // Other, Private Use
    Cn,     // Other, Not Assigned (no characters in the file have this property)
    // Informative Categories
    Lm,     // Letter, Modifier
    Lo,     // Letter, Other
    Pc,     // Punctuation, Connector
    Pd,     // Punctuation, Dash
    Ps,     // Punctuation, Open
    Pe,     // Punctuation, Close
    Pi,     // Punctuation, Initial quote (may behave like Ps or Pe depending on usage)
    Pf,     // Punctuation, Final quote (may behave like Ps or Pe depending on usage)
    Po,     // Punctuation, Other
    Sm,     // Symbol, Math
    Sc,     // Symbol, Currency
    Sk,     // Symbol, Modifier
    So,     // Symbol, Other
}

impl Category {
    pub fn is_printable(&self) -> bool {
        match self {
            Category::Cc | Category::Cf | Category::Cs | Category::Co | Category::Cn => false,    // control characters are out
            Category::Lm => false,                        // letter modifiers are not printed
            _ => true,
        }
    }
}

#[derive(Debug)]
pub struct UnicodeDatum {
    pub cdpt: u32,          // unicode codepoint
    pub cat: Category,      // unicode category
    val: i32,               // references the value in the unicode_values array
    upper: i32,         // uppercase conversion
    lower: i32,         // lowercase conversion
}

impl UnicodeDatum {
    pub fn as_char(&self) -> char {
        return char::from_u32(self.cdpt).unwrap();
    }
/*
    pub fn upper_char(&self) -> Option<char> {
        if self.upper == -1 {
            return None
        }
        return Some(char::from_u32(self.upper.into().unwrap()).unwrap());
    }
    pub fn lower_char(&self) -> Option<char> {
        if self.lower == -1 {
            return None
        }
        return Some(char::from_u32(self.lower.into().unwrap()).unwrap());
    }
 */
    pub fn value(&self) -> Option<&UnicodeDatumValue> {
        if self.val == -1 {
            return None;
        }
        let idx: usize = self.val.try_into().unwrap();
        return Some(&UNICODE_VALUES[idx])
    }
}

#[derive(Debug)]
pub struct UnicodeDatumValue {
    pub digit: i32,         // digit value
    pub num: i64,           // numerator of value
    pub den: i32,           // denomintor of value
}
