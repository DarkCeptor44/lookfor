# lookfor

A cross-platform command line utility to find and highlight files that match a pattern.

## Installation

- From [crates.io](https://crates.io/crates/lookfor): `cargo install lookfor`
- From [GitHub](https://github.com/DarkCeptor44/lookfor): `cargo install --git https://github.com/DarkCeptor44/lookfor`
- Manually (after cloning the repo locally): `cargo install --path .`

## Usage

![usage](usage.png)

```sh
$ lookfor -h
Cross-Platform file finder

Usage: lookfor [OPTIONS] <PATTERN>

Arguments:
  <PATTERN>  Pattern to search for

Options:
      --path <PATH>     Path to search in [default: .]
      --color <COLOR>   Color of the highlighted text (off for no color) [default: blue]
  -I, --case-sensitive  Case sensitive search
  -h, --help            Print help
  -V, --version         Print version
```

## Todo

- Add support for regular expressions.
- Benchmark on a Linux machine with more find-like commands.

## Benchmarks

The benchmarks are performed on a Windows 10 machine with 32GB of RAM and a Ryzen 7 3800X at the root of the repository (to find anything with `clap` in the target directory).

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `lookfor clap` | 43.0 ± 3.7 | 41.0 | 65.3 | 1.00 |
| `dir /s /b *clap*` | 67.5 ± 3.2 | 64.0 | 78.0 | 1.57 ± 0.15 |
| `findstr /s /m /c:clap *` | 600.9 ± 11.1 | 589.4 | 620.2 | 13.96 ± 1.23 |

## License

This project is licensed under the terms of the [GNU General Public License](LICENSE) v3.0.
