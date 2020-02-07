# Itch

> Simple format InTerCHange (get it)

A simple CLI that attempts best effort translation between data formats

Useful for debugging and writing quick scripts, probably shouldn't be used as part of your production pipeline

```bash
# view help
itch --help

# convert input.json to output.toml
itch -i input.json -o output.toml

# convert input.xml to output.yml
cat input.xml | itch -f xml -t yaml | output.yml
```

## Supported Formats

### First Class

Can all be pretty reliably used as sources and targets

- json
- toml
- yaml

### Second Class

Somewhat unreliable, but can be used for basic transformations

- url query strings
- xml
