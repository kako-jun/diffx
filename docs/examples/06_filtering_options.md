# Filtering Options

diffx provides options to filter or ignore certain differences.

## Ignore Keys by Regex

Use `--ignore-keys-regex` to skip certain keys from comparison:

```console
$ diffx tests/fixtures/file1.json tests/fixtures/file2.json --ignore-keys-regex "^age$"
? failed
~ city: "New York" -> "Boston"
  + items[2]: "orange"

```

The `age` field is now ignored and not shown in the output.

## Quiet Mode

Use `--quiet` or `-q` to suppress output and only return exit code:

```console
$ diffx tests/fixtures/file1.json tests/fixtures/file2.json --quiet
? failed

```

Exit codes:
- `0`: No differences
- `1`: Differences found
- `2`: Error occurred

## Files with No Differences

When comparing identical files:

```console
$ diffx tests/fixtures/file1.json tests/fixtures/file1.json

```

Exit code is `0` when files are identical.
