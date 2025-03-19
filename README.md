# lookfor

[![madewith](https://forthebadge.com/images/badges/made-with-rust.svg)](https://forthebadge.com)

A cross-platform CLI tool to find and highlight files that match a pattern.

## Features

- Cross-platform
- Concurrent searching
- Case-sensitive and insensitive search (insensitive by default)
- Customizable colored output for errors and highlighting (can be disabled by setting a `NO_COLOR` environment variable to any value)

## Installation

- From [crates.io](https://crates.io/crates/lookfor): `cargo install lookfor`
- From [GitHub](https://github.com/DarkCeptor44/lookfor): `cargo install --git https://github.com/DarkCeptor44/lookfor`
- From [releases](https://github.com/DarkCeptor44/lookfor/releases/latest).
- Manually:

```sh
git clone https://github.com/DarkCeptor44/lookfor
cd lookfor
cargo install --path .
```

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
  -c, --color <COLOR>   Color of the highlighted text (off for no color) [default: blue] [possible values: red, black, green, yellow, blue, 
magenta, cyan, white, bright-black, bright-red, bright-green, bright-yellow, bright-blue, bright-magenta, bright-cyan, bright-white]        
  -I, --case-sensitive  Case sensitive search
  -h, --help            Print help
  -V, --version         Print version
```

```sh
$ lookfor clap
.\target\debug\.fingerprint\clap-df93454be42887a2\lib-clap.json
.\target\debug\.fingerprint\clap-df93454be42887a2
.\target\debug\.fingerprint\clap-df93454be42887a2\dep-lib-clap
.\target\debug\.fingerprint\clap-df93454be42887a2\invoked.timestamp
.\target\debug\.fingerprint\clap-df93454be42887a2\lib-clap
.\target\debug\.fingerprint\clap_builder-7a64f8b7e0ac51c1
.\target\debug\.fingerprint\clap_builder-7a64f8b7e0ac51c1\invoked.timestamp
.\target\debug\.fingerprint\clap_builder-7a64f8b7e0ac51c1\dep-lib-clap_builder
.\target\debug\.fingerprint\clap_builder-7a64f8b7e0ac51c1\lib-clap_builder
.\target\debug\.fingerprint\clap_builder-7a64f8b7e0ac51c1\lib-clap_builder.json
.\target\debug\.fingerprint\clap_derive-19d5354e6d08a992
.\target\debug\.fingerprint\clap_derive-19d5354e6d08a992\dep-lib-clap_derive
.\target\debug\.fingerprint\clap_derive-19d5354e6d08a992\invoked.timestamp
.\target\debug\.fingerprint\clap_derive-19d5354e6d08a992\lib-clap_derive
.\target\debug\.fingerprint\clap_derive-19d5354e6d08a992\lib-clap_derive.json
.\target\debug\.fingerprint\clap_lex-1195a16252b95268
.\target\debug\.fingerprint\clap_lex-1195a16252b95268\dep-lib-clap_lex
.\target\debug\.fingerprint\clap_lex-1195a16252b95268\invoked.timestamp
.\target\debug\.fingerprint\clap_lex-1195a16252b95268\lib-clap_lex
.\target\debug\.fingerprint\clap_lex-1195a16252b95268\lib-clap_lex.json
.\target\debug\deps\clap-df93454be42887a2.d

...
```

## Todo

- [ ] Add support for regular expressions.

## Tests

```sh
cargo build
cargo test
```

## Benchmarks

The benchmarks were performed using [Hyperfine](https://github.com/sharkdp/hyperfine) at the root of the repository after running both `cargo build` and `cargo build -r`, to find anything with `clap` in the `target` directory.

### Windows

- AMD64, 32GB RAM, Ryzen 7 3800X, Windows 10.

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `dir /s /b *clap*` | 39.4 ± 1.1 | 37.7 | 45.0 | 1.06 ± 0.05 |
| `findstr /s /m /c:clap *` | 327.0 ± 1.9 | 325.6 | 331.6 | 8.78 ± 0.31 |
| `target\release\lookfor.exe clap` | 37.3 ± 1.3 | 35.6 | 42.0 | 1.00 |

### Linux

- ARM64, 1GB RAM, Orange Pi Zero2, Debian 12.

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `target/release/lookfor clap` | 37.5 ± 0.8 | 36.4 | 41.6 | 1.02 ± 0.03 |
| `find . -iname "*clap*"` | 37.0 ± 0.5 | 35.9 | 38.2 | 1.00 |

## License

This project is licensed under the terms of the [GNU General Public License](LICENSE) version 3.
