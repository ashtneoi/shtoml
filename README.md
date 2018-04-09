## how to use shtoml

### invoking shtoml

`shtoml FILE KEY`

- FILE: a TOML v0.4.0 file
- KEY: a dotted key, as specified in the [post-0.4.0 TOML spec](https://github.com/toml-lang/toml), whose value is a scalar (not an array or table)

### example

`config.toml:`

```toml
[foo]
name = "Foo"
number = 194

[bar]
name = "Bar"
number = 12
yellow = false
```

- `shtoml config.toml foo.name` -> `Foo`
- `shtoml config.toml bar.yellow` -> `false`
- `shtoml config.toml bar` -> `error: value is not a scalar`

## the name

"shtoml" stands for "SHell TOML".
