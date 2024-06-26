# hoymiles-rs

A CLI to interact with your HoyMiles solar installation

**WIP: This crate is under heavy development!**

## Pre-requirements

Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Setting up development

- `git clone git@github.com:gruberb/hoymiles-rs.git`
- `cd hoymiles-rs`
- `cargo build --release`

## Easy Installation

```bash
> cargo install hoymiles-rs
```

## Usage

```bash
❯ hoymiles-rs
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
❯ hoymiles-rs login --user-name USERNAME --password PASSWORD
Successfully saved auth token to config file in /Users/username/.config/hoymiles-rs/config.toml
```

Afterwards, we can start reading data from a specific installation for a specific day and the resolution we want (day | week | month | year).

```bash
❯ hoymiles-rs power --help
Fetch your daily power data from a specific solar installation

Usage: hoymiles-rs power [OPTIONS] --sid <SID> --date <DATE> --resolution <RESOLUTION>

Options:
      --sid <SID>                Pass the SID of your solar installation [env: SOLAR_SID=]
  -v, --verbose...               Defines the verbosity level
      --date <DATE>              Pass the day you want to fetch data for [env: SOLAR_DATE=]
      --resolution <RESOLUTION>  Pass the day you want to fetch data for [env: SOLAR_DATE=] [possible values: day, week, month, year]
      --save <SAVE>              Save the response in a CSV or JSON file [env: SOLAR_SAVE_AS=] [possible values: csv, json]
  -h, --help                     Print help
```

```bash
❯ hoymiles-rs power --ssid 123456789 --date 2024-01-01 --resolution day
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
```

We can also save it to a `JSON` or `CSV` file.

```bash
❯ hoymiles-rs power --ssid 123456789 --date 2024-01-01 --resolution day --save csv
Data saved in grid_power.csv
```
