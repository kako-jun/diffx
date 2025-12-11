# Help and Version

## Show Version

```console
$ diffx --version
diffx 0.6.0

```

## Show Help

```console
$ diffx --help
A diff tool for structured data

Usage: diffx [OPTIONS] [FILE1] [FILE2]

Arguments:
  [FILE1]  The first input file
  [FILE2]  The second input file

Options:
      --completions <SHELL>
          Generate shell completions for the specified shell [possible values: bash, elvish, fish, powershell, zsh]
  -f, --format <FORMAT>
          Input file format (auto-detected if not specified) [possible values: json, yaml, csv, toml, ini, xml]
  -o, --output <OUTPUT>
          Output format
  -r, --recursive
          Compare directories recursively
      --path <PATH>
          Filter by path (only show differences in paths containing this string)
      --ignore-keys-regex <IGNORE_KEYS_REGEX>
          Ignore keys matching this regex pattern
      --epsilon <EPSILON>
          Numerical comparison tolerance (for floating point numbers)
      --array-id-key <ARRAY_ID_KEY>
          Array comparison by ID key (compare arrays by this field instead of index)
      --ignore-whitespace
          Ignore whitespace differences
      --ignore-case
          Ignore case differences
  -q, --quiet
          Suppress normal output; return only exit status
      --brief
          Report only whether files differ, not the differences
  -v, --verbose
          Show verbose processing information
      --no-color
          Disable colored output
  -h, --help
          Print help
  -V, --version
          Print version

```
