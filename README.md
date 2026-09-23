# csv2parquet

A fast, standalone CLI to convert CSV files to Parquet. Fills the gap left by Python Pandas/Polars scripts that require a heavy environment setup by providing a single native binary.

## Status

**built, untested**: the CSV schema inference and Parquet writing logic using Apache Arrow is built. It has not been run against real data files in this sandbox environment.

## Installation

```sh
cargo install --path .
```

## Usage

```sh
csv2parquet input.csv output.parquet
```
