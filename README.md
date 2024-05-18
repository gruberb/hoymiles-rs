# hoymiles-rs

A CLI to interact with your HoyMiles solar installation

**WIP: This crate is under heavy development!**

## Pre-requirements

Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Install

- `git clone git@github.com:gruberb/hoymiles-rs.git`
- `cd hoymiles-rs`
- Create `.env` file with the key `HOYMILES_TOKEN=`.
- `cargo build`

## Usage

```bash
❯ ./target/release/hoymiles-rs
A CLI to interact with your HoyMiles solar installation

Usage: hoymiles-rs [OPTIONS] <COMMAND>

Commands:
  login  Login to your Hoymiles account
  power  Fetch your daily power data from a specific solar installation
  help   Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...  Defines the verbosity level
  -h, --help        Print help
```

We first need to get the token from HoyMiles.

```bash
❯ ./target/release/hoymiles-rs login --user-name USERNAME --password PASSWORD
Set HOYMILES_TOKEN env variable successfully.
```

This will overwrite the `HOYMILES_TOKEN` value in your `.env` file. Afterwards, we can start reading data from a specific installation for a specific day.

```bash
❯ ./target/release/hoymiles-rs power --ssid 123456789 --date 2024-01-01
Time     |      Power
---------+-----------
00:00    |       0.00
01:00    |       0.00
02:00    |       0.00
03:00    |       0.00
04:00    |       0.00
05:00    |       0.00
06:00    |     141.10
06:15    |     340.90
06:30    |     576.90
06:45    |     895.90
07:00    |    1269.50
07:15    |    1485.00
07:30    |    1472.70
07:45    |    1874.50
```
