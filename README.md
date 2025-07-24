# Itch

> InTerCHanges one data format into another (get it?)

A very simple cli to convert between some of the most common plain-text data formats.
It can't perform every conversion that might be theoretically possible, but it tries its best

# Installation

```bash
cargo install itch --locked
```

# Overview

```
USAGE:
    itch [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --from <from-type>    Format of the input, will be derived if possible
    -i, --input <input>       Path to the input file, leave empty for stdin
    -o, --output <output>     Path to the output file, leave empty for stdout
    -t, --to <to-type>        Format of the output, will be derived if possible
```

`itch` can take input from a file or std in, and output to a file or std out. If given a file as input or output it will try to detect the format automatically.

`itch` doesn't do any manipulation to data to satisfy the different constructs that different formats can express (eg: in `toml`, object key ordering is important); so there's no guarantee that a conversion will work.

## Formats

### First Class

Can all be pretty reliably used as sources and targets

- cbor
- json
- toml
- yaml

### Second Class

Somewhat unreliable, but can be used for basic transformations

- url query strings
- xml

## Behaviour

**input**

```shell
echo '<element key="value"><child/></element>' | itch -f xml -t json
```

**output**

```json
{ "key": "value", "child": {} }
```

`itch` will not necessarily produce output that can automatically be reversed:

**input**

```shell
echo '<element key="value"><child/></element>' | itch -f xml -t json | itch -f json -t xml
```

**output**

```xml
<key>value</key><child></child>
```

# Implementation

Uses [serde][serde] and it's own internal data representation format to act as an intermediary between the different data formats:

```rust
enum Itch {
    Obj(IndexMap<String, Itch>),
    Array(Vec<Itch>),
    Bool(bool),
    Int(i64),
    Float(f64),
    Text(String),
}
```

Each deserialization step converts to this type, and each serialisation step converts from it.

[clap]: https://github.com/clap-rs/clap
[crates]: https://crates.io/
[rustup]: https://rustup.rs/
[serde]: https://serde.rs/
