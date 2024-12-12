# Synthetic Data Generator

A versatile command-line tool written in Rust for generating synthetic data for testing and development purposes. The tool generates realistic-looking data for users, addresses, payment providers, and transactions in either JSON or CSV format.

## Features
- Generate synthetic data for:
  - Users (with names, emails, and phone numbers)
  - Addresses (linked to users)
  - Payment Providers
  - Transactions
- Support for both JSON and CSV output formats
- Auto-incrementing IDs with customizable starting point for each entity
- Progress bars for visual feedback
- Skewed data distribution option for realistic transaction patterns
- Realistic fake data generation for names, addresses, and contact information
- Configurable number of records for each entity type

## Installation

1. Make sure you have Rust and Cargo installed. If not, install from [rustup.rs](https://rustup.rs/)
2. Clone the repository:
```bash
git clone [repository-url]
cd synthetic-data-generator
```

3. Build the project:
```bash
cargo build --release
```

## Usage

The project provides two binary targets:
- `transaction-generator-id`: Generates data with auto-incrementing integer IDs
- `transaction-generator-uuid`: Generates data with UUID identifiers

### Command Line Options

| Option | Short | Long | Default | Description |
|--------|-------|------|---------|-------------|
| Users | -u | --users | 100 | Number of users to generate |
| Transactions | -t | --transactions | 1000 | Number of transactions to generate |
| Providers | -p | --providers | 10 | Number of payment providers to generate |
| Output Directory | -o | --output-dir | "output" | Directory where files will be saved |
| Format | -f | --format | "json" | Output format ("json" or "csv") |
| Skewed | -s | --skewed | false | Enable skewed transaction distribution |
| User Start ID | | --user-start-id | 1 | Starting ID for users |
| Address Start ID | | --address-start-id | 1 | Starting ID for addresses |
| Provider Start ID | | --provider-start-id | 1 | Starting ID for providers |
| Transaction Start ID | | --transaction-start-id | 1 | Starting ID for transactions |

### Examples

#### Basic Usage with Integer IDs

1. Generate data with default settings:
```bash
cargo run --bin transaction-generator-id
```

2. Generate 1000 transactions with IDs starting from 1000:
```bash
cargo run --bin transaction-generator-id -- --transaction-start-id 1000 -t 1000
```

3. Generate data with custom start IDs for all entities:
```bash
cargo run --bin transaction-generator-id -- \
    --user-start-id 100 \
    --address-start-id 100 \
    --provider-start-id 100 \
    --transaction-start-id 1000 \
    -t 1000 \
    -u 50 \
    -p 5
```

4. Generate CSV output with custom ID ranges:
```bash
cargo run --bin transaction-generator-id -- \
    --user-start-id 1000 \
    --transaction-start-id 5000 \
    -f csv \
    -t 2000
```

5. Generate skewed transaction data:
```bash
cargo run --bin transaction-generator-id -- \
    -u 1000 \
    -t 10000 \
    -s
```

#### Using UUID Generation

1. Generate data with UUIDs:
```bash
cargo run --bin transaction-generator-uuid
```

2. Generate large UUID dataset in CSV format:
```bash
cargo run --bin transaction-generator-uuid -- \
    -u 1000 \
    -t 10000 \
    -p 15 \
    -f csv
```

### Output Examples

#### Users (users.json/users.csv)
```json
{
    "id": 1,
    "name": "John Doe",
    "email": "john.doe@example.com",
    "phone": "+1-555-123-4567"
}
```

#### Addresses (addresses.json/addresses.csv)
```json
{
    "id": 1,
    "user_id": 1,
    "street": "123 Main St",
    "city": "Springfield",
    "state": "IL",
    "country": "United States",
    "postal_code": "62701"
}
```

#### Payment Providers (providers.json/providers.csv)
```json
{
    "id": 1,
    "name": "Visa"
}
```

#### Transactions (transactions.json/transactions.csv)
```json
{
    "id": 1,
    "user_id": 1,
    "provider_id": 1,
    "amount": 123.45,
    "timestamp": "2024-12-12T10:30:00Z"
}
```

## Additional Features

### Skewed Distribution
When using the `-s` or `--skewed` flag, the transaction generation will follow a more realistic pattern where some users generate more transactions than others. This is useful for testing scenarios that require more realistic data patterns.

### Format Options
- JSON output: Records are written as newline-delimited JSON
- CSV output: Data is properly escaped and formatted according to CSV standards

### Progress Tracking
The tool provides real-time progress bars showing:
- Generation progress for each entity type
- Elapsed time
- Completion percentage

