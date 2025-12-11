# Supported Formats

diffx supports multiple structured data formats.

## YAML

```console
$ diffx tests/fixtures/file1.yaml tests/fixtures/file2.yaml
? 1
~ age: 30 -> 31
~ city: "New York" -> "Boston"
  + items[2]: "orange"

```

## TOML

```console
$ diffx tests/fixtures/file1.toml tests/fixtures/file2.toml
? 1
~ age: 30 -> 31
~ city: "New York" -> "Boston"
  + items[2]: "orange"

```

## XML

```console
$ diffx tests/fixtures/file1.xml tests/fixtures/file2.xml
? 1
...
```

## INI

```console
$ diffx tests/fixtures/file1.ini tests/fixtures/file2.ini
? 1
...
```

## CSV

```console
$ diffx tests/fixtures/file1.csv tests/fixtures/file2.csv
? 1
...
```
