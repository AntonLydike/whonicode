static STRINGS_FILE: &'static [u8] = include_bytes!("ucd_strings.txt");

pub struct SearchResult {
    pub idx: usize,
    pub name: Vec<u8>,
}

pub fn search_ucds(needle: &[u8]) -> Vec<SearchResult> {
    let mut pos = 0;
    let mut matches = Vec::new();
    let mut start = 0;

    for i in 0..STRINGS_FILE.len() {
        if STRINGS_FILE[i] == 0 {
            pos += 1;
            start = i+1;
            continue;
        }
        if STRINGS_FILE[i] == needle[0] {
            if i + needle.len() >= STRINGS_FILE.len() {
                break
            }
            
            let mut is_match = true;
            for scan in 1..needle.len() {
                if STRINGS_FILE[i+scan] != needle[scan] {
                    is_match = false;
                    break
                }
            }

            if !is_match {
                continue
            }

            let mut end = needle.len();
            while STRINGS_FILE[i + end] != 0 {
                end += 1;
            }

            matches.push(SearchResult {
                idx: pos,
                name: STRINGS_FILE[start .. i+end].to_vec()
            });
        }
    }

    return matches;

}