use std::str::FromStr;
use std::fmt;


#[derive(Debug, Eq, PartialEq, Hash, Clone)]
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

    pub fn name(&self) -> &str {
        match self {
            Category::Lu => "Letter, Uppercase",
            Category::Ll => "Letter, Lowercase",
            Category::Lt => "Letter, Titlecase",
            Category::Mn => "Mark, Non-Spacing",
            Category::Mc => "Mark, Spacing Combining",
            Category::Me => "Mark, Enclosing",
            Category::Nd => "Number, Decimal Digit",
            Category::Nl => "Number, Letter",
            Category::No => "Number, Other",
            Category::Zs => "Separator, Space",
            Category::Zl => "Separator, Line",
            Category::Zp => "Separator, Paragraph",
            Category::Cc => "Other, Control",
            Category::Cf => "Other, Format",
            Category::Cs => "Other, Surrogate",
            Category::Co => "Other, Private Use",
            Category::Cn => "Other, Not Assigned (no characters in the file have this property)",
            Category::Lm => "Letter, Modifier",
            Category::Lo => "Letter, Other",
            Category::Pc => "Punctuation, Connector",
            Category::Pd => "Punctuation, Dash",
            Category::Ps => "Punctuation, Open",
            Category::Pe => "Punctuation, Close",
            Category::Pi => "Punctuation, Initial quote (may behave like Ps or Pe depending on usage)",
            Category::Pf => "Punctuation, Final quote (may behave like Ps or Pe depending on usage)",
            Category::Po => "Punctuation, Other",
            Category::Sm => "Symbol, Math",
            Category::Sc => "Symbol, Currency",
            Category::Sk => "Symbol, Modifier",
            Category::So => "Symbol, Other",
        }
    }
}

impl FromStr for Category {
    
    type Err = (String,);

    fn from_str(input: &str) -> Result<Category, Self::Err> {
        match input {
            "Lu" => Ok(Category::Lu),
            "Ll" => Ok(Category::Ll),
            "Lt" => Ok(Category::Lt),
            "Mn" => Ok(Category::Mn),
            "Mc" => Ok(Category::Mc),
            "Me" => Ok(Category::Me),
            "Nd" => Ok(Category::Nd),
            "Nl" => Ok(Category::Nl),
            "No" => Ok(Category::No),
            "Zs" => Ok(Category::Zs),
            "Zl" => Ok(Category::Zl),
            "Zp" => Ok(Category::Zp),
            "Cc" => Ok(Category::Cc),
            "Cf" => Ok(Category::Cf),
            "Cs" => Ok(Category::Cs),
            "Co" => Ok(Category::Co),
            "Cn" => Ok(Category::Cn),
            "Lm" => Ok(Category::Lm),
            "Lo" => Ok(Category::Lo),
            "Pc" => Ok(Category::Pc),
            "Pd" => Ok(Category::Pd),
            "Ps" => Ok(Category::Ps),
            "Pe" => Ok(Category::Pe),
            "Pi" => Ok(Category::Pi),
            "Pf" => Ok(Category::Pf),
            "Po" => Ok(Category::Po),
            "Sm" => Ok(Category::Sm),
            "Sc" => Ok(Category::Sc),
            "Sk" => Ok(Category::Sk),
            "So" => Ok(Category::So),
            _    => Err((String::from("Unknown category supplied!"),)),
        }
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct UnicodeDatum {
    pub cdpt: u32,          // unicode codepoint
    pub cat: Category,      // unicode category
    val: i32,               // references the value in the unicode_values array (-1 signifies absence)
    upper: i32,         // uppercase conversion (-1 signifies absence)
    lower: i32,         // lowercase conversion (-1 signifies absence)
}

impl UnicodeDatum {
    pub fn from_idx(idx: usize) -> &'static UnicodeDatum{
        return &UNICODE_DATA[idx];
    }
}

impl UnicodeDatum {
    pub fn as_char(&self) -> char {
        return char::from_u32(self.cdpt).unwrap();
    }

    pub fn upper_char(&self) -> char {
        match self.upper {
            -1 => '-',
            _  => char::from_u32(self.upper as u32).unwrap()
        }
    }

    pub fn lower_char(&self) -> char {
        match self.lower {
            -1 => '-',
            _  => char::from_u32(self.lower as u32).unwrap()
        }
    }

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

impl UnicodeDatumValue {
    pub fn value_to_str(&self) -> String {
        if self.den == 1 {
            return self.num.to_string();
        }
        return format!("{}/{}", self.num, self.den);
    }
}

