```bash
$ cargo r frog poem.txt
```
<details>
<summary>result</summary>

```
Searching for 'frog'
In file 'poem.txt'

Matching Contents:
How public, like a frog
```

</details>

```bash
$ export CASE_INSENSITIVE=false
```

```bash
$ cargo r frog poem.txt
```

<details>
<summary>result</summary>

```
Searching for 'frog'
In file 'poem.txt'

Matching Contents:
How public, like a frog
```

</details>

```bash
cargo r frOg poem.txt
```

<details>
<summary>result</summary>

```
Searching for 'frOg'
In file 'poem.txt'

Matching Contents:
```

</details>

```bash
$ unset CASE_INSENSITIVE
```

> Redirect std-output to a file
```bash
$ cargo r To poem.txt > output.txt
```