# Synthetic Data Generator

A versatile command-line tool written in Rust for generating synthetic data for testing and development purposes. The tool generates realistic-looking data for users, addresses, payment providers, and transactions in either JSON or CSV format.

## Features

- Generate synthetic data for:
    - Users (with names, emails, and phone numbers)
    - Addresses (linked to users)
    - Payment Providers
    - Transactions
- Support for both JSON and CSV output formats
- Auto-incrementing IDs with customizable starting point
- Progress bars for visual feedback
- Skewed data distribution option for realistic transaction patterns
- Realistic fake data generation for names, addresses, and contact information
- Configurable number of records for each entity type

## Installation

1. Make sure you have Rust and Cargo installed. If not, install from [rustup.rs](https://rustup.rs/)

2. Clone the repository:
```bash
git clone [repository-url]
cd dummy-data-rust
```

3. Add the following dependencies to your `Cargo.toml`:
```toml
[dependencies]
clap = { version = "4.0", features = ["derive"] }
fake = "2.5"
indicatif = "0.17"
rand = "0.8"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
csv = "1.2"
chrono = "0.4"
```

4. Build the project:
```bash
cargo build --release
```

## Usage

The basic command structure is:
```bash
cargo run -- [OPTIONS]
```

### Command Line Options

| Option | Short | Long | Default | Description |
|--------|-------|------|---------|-------------|
| Users | -u | --users | 100 | Number of users to generate |
| Transactions | -t | --transactions | 1000 | Number of transactions to generate |
| Providers | -p | --providers | 10 | Number of payment providers to generate |
| Output Directory | -o | --output-dir | "output" | Directory where files will be saved |
| Format | -f | --format | "json" | Output format ("json" or "csv") |
| Start ID | -i | --start-id | 1 | Starting ID for auto-increment |
| Skewed | -s | --skewed | false | Enable skewed transaction distribution |

### Examples

1. Generate default dataset in JSON format:
```bash
cargo run -- -u 100 -t 1000 -p 10
```

2. Generate large dataset in CSV format:
```bash
cargo run -- -u 1000 -t 10000 -p 15 -f csv
```

3. Generate data with custom starting ID:
```bash
cargo run -- -u 100 -t 1000 -p 10 -i 1000
```

4. Generate data with skewed transaction distribution:
```bash
cargo run -- -u 100 -t 1000 -p 10 -s
```

### Output Structure

The generator creates four files in your specified output directory:

#### Users File (users.json/users.csv)
```json
{
  "id": 1,
  "name": "John Doe",
