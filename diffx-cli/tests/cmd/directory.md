# Directory Comparison

Compare all files in two directories recursively.

## Basic Directory Comparison

```console
$ diffx -r tests/fixtures/dir1 tests/fixtures/dir2
? 1
...
```

## Directory Without -r Flag (Error)

```console
$ diffx tests/fixtures/dir1 tests/fixtures/dir2
? 2
...
```
