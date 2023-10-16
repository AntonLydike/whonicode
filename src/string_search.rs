// single array of zero terminated strings
static STRINGS_FILE: &'static [u8] = include_bytes!("ucd_strings.txt");

pub struct SearchResult {
    pub idx: usize,
    pub name: Vec<u8>,
}

pub fn search_ucds(needle: &[u8]) -> Vec<SearchResult> {
    let mut matches = Vec::new();   // results
    let mut pos = 0;                // index of the string inside STRINGS_FILE
    let mut start = 0;              // start of the last seen string
    let mut skip_line = false;      // once a line is matched, skip the rest of it

    // loop over the file
    'outer: for i in 0..STRINGS_FILE.len() {
        // we hit the end of a string
        if STRINGS_FILE[i] == 0 {
            pos += 1;           // we're at the next string now
            start = i+1;        // not down the next starting point
            skip_line = false;  // reset line skipping
            continue;
        }

        // no match, or line already matched
        if STRINGS_FILE[i] != needle[0] || skip_line {
            continue;
        }

        // we have a possible match:
        if i + needle.len() >= STRINGS_FILE.len() {
            break
        }
        
        // check if we actually match
        for scan in 1..needle.len() {
            // if we don't match, continue the loop
            if STRINGS_FILE[i+scan] != needle[scan] {
                continue 'outer;
            }
        }

        // find the end of the string
        let mut end = needle.len();
        while STRINGS_FILE[i + end] != 0 {
            end += 1;
        }

        // insert search result
        matches.push(SearchResult {
            idx: pos,
            name: STRINGS_FILE[start .. i+end].to_vec()
        });

        // skip the remainder of this line
        skip_line = true;
    }

    return matches;

}