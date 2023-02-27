# Simple rust program with some unit tests

## Run the program

### Run program with a valid number

```bash
$ cargo run -- 123
```

### Run program with some invalid input

```bash
$ cargo run -- foo
$ cargo run -- ' '
$ cargo run -- 256
```

## Run the tests

### Run tests in default terse mode

```bash
$ cargo test
```

### Run tests in verbose mode

```bash
$ DEBUG=1 cargo test -vv -- --nocapture 
```

## See also

- [Kata Containers unit testing presentation](https://github.com/kata-containers/kata-containers/blob/main/docs/presentations/unit-testing/kata-containers-unit-testing.md)
- [Kata Containers unit test advice document](https://github.com/kata-containers/kata-containers/blob/main/docs/Unit-Test-Advice.md)
