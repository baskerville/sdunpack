# Installation

```sh
cargo install --path .
```

# Usage

```sh
sdunpack file.dict < file.idx > file.txt
```

# Convert StarDict to dictd

```sh
dictzip -d file.dict.dz
sdunpack file.dict < file.idx > file.txt
dictfmt --utf8 --index-keep-orig --headword-separator '|' -s "ShortName" -u "URL" -t file2 < file.txt
dictzip file2.dict
```

