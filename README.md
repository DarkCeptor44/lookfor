# lookfor

A cross-platform CLI tool to find and highlight files or folders that match a pattern.

## Features

- Cross-platform
- Asynchronous
- Case-sensitive and insensitive search (insensitive by default)
- Customizable colored output for highlighting (can be disabled by setting a `NO_COLOR` environment variable to any value)

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
- [ ] Add highlighting for every matching part of a line.
- [ ] Write better tests.

## Tests

```sh
cargo build
cargo test
```

## Benchmarks

The benchmarks were performed using [Hyperfine](https://github.com/sharkdp/hyperfine) at the root of the repository after running both `cargo build` and `cargo build -r`, to find anything with `clap` in the `target` directory.

### TLDR

The new results are different even for `dir`, `findstr` and `find` but essentially:

- On Windows the async version is **24% faster** than `dir`, where sync version was **19%**, and **1176% faster** than `findstr`, where sync version was **~900%**
- On Linux the async version is **9% slower** than `find`, where sync version was **2% faster**, this probably comes from the overhead of `tokio` and I don't think it's worth fixing

### Windows

- AMD64, 32GB RAM, Ryzen 7 3800X, Windows 10.

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `dir /s /b *clap*` | 56.9 ± 2.0 | 54.3 | 66.9 | 1.24 ± 0.18 |
| `findstr /s /m /c:clap *` | 538.1 ± 18.3 | 525.7 | 577.7 | 11.76 ± 1.69 |
| `target\release\lookfor.exe clap` | 45.8 ± 6.4 | 39.2 | 72.1 | 1.00 |

### Linux

- ARM64, 1GB RAM, Orange Pi Zero2, Debian 12.

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `find . -iname "*clap*"` | 20.5 ± 0.3 | 19.7 | 21.5 | 1.00 |
| `target/release/lookfor clap` | 73.3 ± 1.3 | 70.6 | 76.5 | 3.57 ± 0.09 |

## License

This project is licensed under the terms of the [GNU General Public License](LICENSE) version 3.
