# Installation

```sh
cargo install --path .
```

# Usage

```sh
sdunpack file.txt file.idx file.dict [file.syn]
```

# Convert StarDict to dictd

This example relies on some of the command line utilities provided by [dictd](https://sourceforge.net/projects/dict/files/dictd/).

```sh
dictzip -d file.dict.dz
sdunpack file.txt file.idx file.dict file.syn
short_name=$(grep '^bookname=' file.ifo | cut -d '=' -f 2)
url=$(grep '^website=' file.ifo | cut -d '=' -f 2)
dictfmt --utf8 --index-keep-orig --headword-separator '|' -s "$short_name" -u "$url" -t file2 < file.txt
dictzip file2.dict
```
