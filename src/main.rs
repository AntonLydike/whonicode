use std::{collections::HashSet, cmp::max, str::FromStr};
mod ucd;
mod string_search;
use clap::Parser;
use string_search::SearchResult;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    query: String,

    /// restrict to category
    #[arg(short, long)]
    category: Option<String>,

    #[arg(short, long, default_value_t = false)]
    print_categories: bool,
}

fn main() {
    let args = Args::parse();

    let query = &args.query;

    let selected_cat: Option<ucd::Category>  = match args.category {
        None => None,
        Some(c) => Some(ucd::Category::from_str(c.as_str()).unwrap())
    };

    let results = match selected_cat {
        // excuse my terrible way of adding this filter
        Some(cat) => string_search::search_ucds(query.to_lowercase().as_bytes())
        .into_iter().filter( // this is just hideous, I agree
            |res| ucd::UnicodeDatum::from_idx(res.idx).cat == cat
        ).collect::<Vec<SearchResult>>(),
        // I just haven't quite figured out how to do this
        None => string_search::search_ucds(query.to_lowercase().as_bytes())
    };

    if results.len() == 0 {
        println!("No results for \"{}\"!", query);
        std::process::exit(-1);
    }

    //      ""
    let mut hidden = 0;
    let mut categories: HashSet<&ucd::Category> = HashSet::new();

    let mut max_name_len = 0;
    for result in &results {
        max_name_len = max(result.name.len(), max_name_len);
    }

    // print header
    let _name = "Name";
    println!("Symbol\t{_name:<max_name_len$}\tCat\tValue");

    for result in results {
        let res_info = &ucd::UNICODE_DATA[result.idx];

        if !res_info.cat.is_printable() {
            hidden += 1;
            continue;
        }

        categories.insert(&res_info.cat);

        let name = String::from_utf8(result.name).unwrap();

        let symb = ucd::UnicodeDatum::as_char(res_info);

        let value = res_info.value();

        let value_str : String = match value {
            Some(val) => val.value_to_str(),
            None => "-".to_string()
        };

        println!("{symb}\t{name:<max_name_len$}\t{}\t{value_str}", res_info.cat);
    }

    if (args.print_categories && categories.len() > 0) || hidden > 0{
        println!("---")
    }

    if args.print_categories {
        for cat in categories {
            println!("{:?} = {}", cat, cat.name())
        }
    }

    if hidden > 0 {
        println!("not showing {} unprintable characters", hidden);
    }
}
