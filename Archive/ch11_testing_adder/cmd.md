> To run tests

```bash
$ cargo t 
$ cargo test
```

> To run tests in Single thread **\<SerialExecution>**

```bash
$ cargo t -- --test-threads=1
$ cargo test -- --test-threads=1
```

> To show std-output
- Usually Output of PASSING tests are not printed
```bash
$ cargo test -- --show-output
```

> To run only 1 test OR set of test having common char
```bash
$ cargo test add
```
```
running 2 tests
test tests::it_adds_two ... ok
test tests::it_dont_add_three ... ok
```

```bash
$ cargo test greeting_contains_name
```
```
running 1 test
test tests::greeting_contains_name ... ok
```


> Run test based on `modules`
```bash
$ cargo t MODULE::
```

> Run ignored tests
```bash
$ cargo t -- --ignored
```

```
running 1 test
test tests::greeting_contains_name ... ok
```

> To only run `integration` test

```bash
$ cargo t --test integration_test
```