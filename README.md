# snoop

A tool suite for inspecting Solana programs

## Installation

### Cargo

```bash
cargo install snoop
```

## Usage

```bash
snoop --help
```

#### Disassemble

Disassemble a compiled program binary, and print to stdout, or optionally write to a .s file

```bash
snoop disassemble <path-to-program-binary> -o <path-to-output-file>
```

#### Cfg

Inspect the program's config

```bash
snoop cfg <path-to-program-binary>
```

#### Graph

Generate a graphviz of a given program binary

```bash
snoop graph <path-to-program-binary>
```

#### Functions

View all functions defined in the binary

```bash
snoop graph <path-to-program-binary>
```

## Credits

This project draws inspiration from the work of [Dean Little](https://github.com/deanmlittle) and makes heavy use of the[Solana Labs rBPF Toolkit](https://github.com/solana-labs/rbpf).

## Disclaimer

This project is a work in progress. Proceed with caution.
