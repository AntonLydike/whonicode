# whonicode - unicode lookup tool

This is a unicode lookup tool. This is also my first rust project.
It may (will?) break on you.

## Building:

You need to run the following command before you can build the project:

```bash
./generate_index.py --strings --rs >> src/ucd.rs
```

This downloads the unicode database and builds the required datastructures.
If you run this command more often, download the `UnicodeData.txt` from the official website:

```bash
wget https://www.unicode.org/Public/UCD/latest/ucd/UnicodeData.txt
```

*Note:* Adding these ~35k lines to the project thoroughly f*cks rust-analyzer.