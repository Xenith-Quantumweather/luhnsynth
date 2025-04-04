# LuhnSynth

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/luhnsynth.svg)](https://crates.io/crates/luhnsynth)
[![Rust](https://github.com/yourusername/luhnsynth/workflows/Rust/badge.svg)](https://github.com/yourusername/luhnsynth/actions)

**LuhnSynth** is a robust Rust tool designed to generate realistic synthetic credit card transaction data for testing the security of PCI DSS systems. The tool produces valid credit card numbers that pass the Luhn algorithm check and conform to the correct BIN prefixes and PAN lengths for all major card brands.


## Features

- **Valid Credit Card Generation**: Produces credit card numbers that pass the Luhn check
- **Multiple Card Brands**: Supports Visa, Mastercard, American Express, and Discover
- **Realistic Transaction Data**: Includes all standard payment processing fields
- **Configurable Output Size**: Generate datasets of various sizes (default: 100, 250, and 500 records)
- **Multiple Export Formats**: Outputs in both CSV and JSON formats
- **Randomized But Realistic**: Creates varied but plausible transaction patterns

## Installation

### From Crates.io

```bash
cargo install luhnsynth
```

### From Source

```bash
git clone https://github.com/Xenith-Quantumweather/luhnsyth.git
cd luhnsynth
cargo install --path .
```

## Usage

### Basic Usage

Simply run the tool without any arguments to generate six files in your current directory:

```bash
luhnsynth
```

This will create:
- `transactions_100.csv` - CSV file with 100 records
- `transactions_250.csv` - CSV file with 250 records  
- `transactions_500.csv` - CSV file with 500 records
- `transactions_100.json` - JSON file with 100 records
- `transactions_250.json` - JSON file with 250 records
- `transactions_500.json` - JSON file with 500 records

### Command Line Options

```
USAGE:
    luhnsynth [OPTIONS]

OPTIONS:
    -o, --output-dir <DIR>     Directory to save the generated files [default: current directory]
    -s, --sizes <SIZES>        Comma-separated list of dataset sizes to generate [default: 100,250,500]
    -f, --format <FORMAT>      Output format: csv, json, or both [default: both]
    -h, --help                 Print help information
    -V, --version              Print version information
```

### Examples

Generate only CSV files:
```bash
luhnsynth --format csv
```

Generate custom dataset sizes:
```bash
luhnsynth --sizes 50,1000,5000
```

Specify a custom output directory:
```bash
luhnsynth --output-dir ./test-data
```

## Data Format

Each transaction record includes the following fields:

| Field | Description |
|-------|-------------|
| transaction_id | Unique identifier for the transaction |
| transaction_date | ISO 8601 timestamp of when the transaction occurred |
| status | Transaction status (approved, declined, pending, refunded) |
| decline_reason | Reason for decline (if applicable) |
| cardholder_name | Synthetic first and last name of the cardholder |
| card_number | Valid credit card number |
| card_brand | Card brand (Visa, Mastercard, American Express, Discover) |
| card_expiry | Card expiry date in MM/YY format |
| cvv | Card verification value |
| amount | Transaction amount |
| currency | Currency code (USD, EUR, GBP, CAD, AUD, JPY) |
| merchant_name | Name of the merchant |
| merchant_id | Merchant identifier |
| merchant_category | Category of the merchant |
| payment_method | Method used for payment (always "credit_card") |
| ip_address | Random IP address |
| device_id | Device identifier |
| user_agent | Browser user agent string |

## Use Cases

- Testing payment processing systems
- Developing fraud detection algorithms
- QA testing for financial applications
- Data visualization prototyping
- PCI compliance training
- Performance testing with realistic data

## Security Notice

⚠️ **IMPORTANT**: While the card numbers generated are structurally valid (pass the Luhn check and match correct BIN patterns), they are synthetic and NOT connected to any real accounts or financial systems. 

Never attempt to use these numbers for actual financial transactions. This tool is designed exclusively for testing and development purposes.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Acknowledgments

- Inspired by the need for realistic but safe test data in payment systems
- Thanks to the Rust community for providing excellent crates that made this tool possible
