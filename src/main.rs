mod ucd;
mod string_search;


fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 || args.len() > 2 {
        println!("usage: whonicode \"<search srting>\"");
        std::process::exit(1);
    }

    let query = &args[1];

    let results = string_search::search_ucds(query.to_lowercase().as_bytes());

    if results.len() == 0 {
        println!("No results for \"{}\"!", query);
        std::process::exit(-1);
    }

    println!("matches for \"{}\":", query);

    //      ""
    let mut hidden = 0;
    for result in results {
        let res_info = &ucd::UNICODE_DATA[result.idx];
        if !res_info.cat.is_printable() {
            hidden += 1;
            continue;
        }
        let name = String::from_utf8(result.name).unwrap();

        let symb = ucd::UnicodeDatum::as_char(res_info);

        println!("{}\t{}", symb, name);
    }

    if hidden > 0 {
        println!("not showing {} unprintable characters", hidden);
    }
}
