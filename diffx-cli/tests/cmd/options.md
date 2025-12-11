# Comparison Options

## Ignore Keys by Regex

Skip keys matching a pattern (applied recursively).

```console
$ diffx --ignore-keys-regex "^timestamp$" tests/fixtures/context_test1.json tests/fixtures/context_test2.json
? 1
...
```

## Numeric Tolerance (epsilon)

Treat numbers as equal if difference is within tolerance.

```console
$ diffx --epsilon 0.01 tests/fixtures/file1.json tests/fixtures/file1.json

```

## Case Insensitive

Ignore case differences in string values.

```console
$ diffx --ignore-case tests/fixtures/case_test1.json tests/fixtures/case_test2.json
...
```

## Ignore Whitespace

Ignore whitespace differences in strings.

```console
$ diffx --ignore-whitespace tests/fixtures/whitespace_test1.json tests/fixtures/whitespace_test2.json
...
```

## Array ID Key

Match array elements by ID instead of index.

```console
$ diffx --array-id-key id tests/fixtures/users_v1.json tests/fixtures/users_v2.json
? 1
...
```
