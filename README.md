# calc

A simple command-line calculator.

## Installation

`clac` can be installed from a couple different places:

### Arch Linux/AUR

`clac` is available from the [AUR](https://aur.archlinux.org) as [clac-bin](https://aur.archlinux.org/packages/clac-bin).

```bash
# With yay or any other AUR helper:
yay -S clac-bin
```

### Cargo/Crates.io

`clac` is also available from [crates.io](https://crates.io).

```bash
cargo install clac
```

### Build from Source

To build from source (requires the Rust toolchain be installed:

```bash
# Clone the repo
git clone https://github.com/alex-laycalvert/clac

# Navigate to repo
cd clac

# Build
cargo build --release

# Binary available
./target/release/clac
```

## Usage

Start `clac` by running `clac` from a terminal. Once inside the prompt, you can evaluate normal mathematical expressions:

```bash
$ clac

> 1 + 2 + 3
= 6
4 * 6 - 7
= 17
```

The following operators are available:

- Addition `+`
- Subtraction `-`
- Multiplication `*`
- Division `/`
- Exponentiation `^`

You can also use `ans` in any expression to use the last evaluated result. Defaults to `0` on the first run and any failing expressions.
