# lookfor

A cross-platform command line utility to find and highlight files that match a pattern.

## Installation

- From [crates.io](https://crates.io/crates/lookfor): `cargo install lookfor`
- From [GitHub](https://github.com/DarkCeptor44/lookfor): `cargo install --git https://github.com/DarkCeptor44/lookfor`
- Manually (after cloning the repo locally): `cargo install --path .`
- From [releases](https://github.com/DarkCeptor44/lookfor/releases/latest).

## Usage

![usage](usage.png)

```sh
$ lookfor -h
Cross-Platform file finder

Usage: lookfor [OPTIONS] <PATTERN>

Arguments:
  <PATTERN>  Pattern to search for

Options:
      --in <PATH>       Path to search in [default: .]
      --color <COLOR>   Color of the highlighted text [default: blue] [possible values: red, black, green, yellow, blue, magenta, cyan, white, bright-black, bright-red, bright-green, bright-yellow, bright-blue, bright-magenta, bright-cyan, bright-white]
  -I, --case-sensitive  Case sensitive search
  -h, --help            Print help
  -V, --version         Print version
```

## Todo

- Add support for regular expressions.

## Benchmarks

The benchmarks were performed on different machines at the root of the repository after running both `cargo build` and `cargo build -r`, to find anything with `clap` in the target directory. Using [Hyperfine](https://github.com/sharkdp/hyperfine).

### Machine A

- AMD64, 32GB RAM, Ryzen 7 3800X, Windows 10.

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `dir /s /b *clap*` | 74.9 ± 1.8 | 72.2 | 81.8 | 1.23 ± 0.04 |
| `findstr /s /m /c:clap *` | 805.9 ± 10.1 | 798.8 | 825.9 | 13.22 ± 0.31 |
| `lookfor clap` | 61.0 ± 1.2 | 59.6 | 64.9 | 1.00 |

### Machine B

- ARM64, 1GB RAM, Orange Pi Zero2, Debian 12.

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `find . -iname "*clap*"` | 29.5 ± 4.4 | 27.8 | 54.6 | 1.20 ± 0.18 |
| `lookfor clap` | 24.6 ± 0.6 | 23.4 | 27.8 | 1.00 |

## License

This project is licensed under the terms of the [GNU General Public License](LICENSE) v3.0.
