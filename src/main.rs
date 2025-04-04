use chrono::{DateTime, Duration, Datelike, Utc};
use rand::{
    distributions::{Distribution, Standard},
    prelude::SliceRandom,
    Rng,
};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{self, Write},
};

// Struct to store card brand information
#[derive(Debug, Clone)]
struct CardBrand {
    name: String,
    prefix: Vec<String>,
    lengths: Vec<usize>,
    cvv_length: usize,
}

// Struct to store merchant information
#[derive(Debug, Clone)]
struct Merchant {
    name: String,
    id: String,
    category: String,
}

// Transaction status enum
#[derive(Debug, Clone, Serialize, Deserialize)]
enum TransactionStatus {
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "declined")]
    Declined,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "refunded")]
    Refunded,
}

// Helper function to implement random distribution for TransactionStatus
impl Distribution<TransactionStatus> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TransactionStatus {
        match rng.gen_range(0..4) {
            0 => TransactionStatus::Approved,
            1 => TransactionStatus::Declined,
            2 => TransactionStatus::Pending,
            _ => TransactionStatus::Refunded,
        }
    }
}

// Decline reason enum (Option to handle null cases)
#[derive(Debug, Clone, Serialize, Deserialize)]
enum DeclineReason {
    #[serde(rename = "insufficient_funds")]
    InsufficientFunds,
    #[serde(rename = "card_expired")]
    CardExpired,
    #[serde(rename = "invalid_card")]
    InvalidCard,
    #[serde(rename = "suspicious_activity")]
    SuspiciousActivity,
}

// Helper function to implement random distribution for DeclineReason
impl Distribution<DeclineReason> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DeclineReason {
        match rng.gen_range(0..4) {
            0 => DeclineReason::InsufficientFunds,
            1 => DeclineReason::CardExpired,
            2 => DeclineReason::InvalidCard,
            _ => DeclineReason::SuspiciousActivity,
        }
    }
}

// Card expiry struct
#[derive(Debug, Clone)]
struct CardExpiry {
    month: u8,
    year: u16,
}

impl CardExpiry {
    fn new(month: u8, year: u16) -> Self {
        Self { month, year }
    }

    fn to_string(&self) -> String {
        format!("{:02}/{}", self.month, self.year % 100)
    }
}

// Main transaction struct
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Transaction {
    transaction_id: String,
    transaction_date: String,
    status: TransactionStatus,
    decline_reason: Option<DeclineReason>,
    cardholder_name: String,
    card_number: String,
    card_brand: String,
    card_expiry: String,
    cvv: String,
    amount: f64,
    currency: String,
    merchant_name: String,
    merchant_id: String,
    merchant_category: String,
    payment_method: String,
    ip_address: String,
    device_id: String,
    user_agent: String,
}

// Helper function to generate random data
fn gen_random_element<T>(vec: &[T]) -> &T {
    let mut rng = rand::thread_rng();
    vec.choose(&mut rng).unwrap()
}

// Generate a random date within the last 3 years
fn gen_random_date() -> DateTime<Utc> {
    let mut rng = rand::thread_rng();
    let now = Utc::now();
    let days_ago = rng.gen_range(0..365 * 3);
    now - Duration::days(days_ago)
}

// Generate a random future expiry date (1-5 years in the future)
fn gen_random_expiry_date() -> CardExpiry {
    let mut rng = rand::thread_rng();
    let now = Utc::now();
    let future_years = rng.gen_range(1..=5);
    let future_month = rng.gen_range(1..=12);
    CardExpiry::new(future_month, (now.year() + future_years) as u16)
}

// Generate a random transaction ID
fn gen_transaction_id() -> String {
    let mut rng = rand::thread_rng();
    let mut id = String::from("TXN");
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    for _ in 0..9 {
        let idx = rng.gen_range(0..CHARSET.len());
        id.push(CHARSET[idx] as char);
    }
    id
}

// Generate a random IPv4 address
fn gen_ip_address() -> String {
    let mut rng = rand::thread_rng();
    format!(
        "{}.{}.{}.{}",
        rng.gen_range(1..255),
        rng.gen_range(0..255),
        rng.gen_range(0..255),
        rng.gen_range(0..255)
    )
}

// Generate a random device ID
fn gen_device_id() -> String {
    let mut rng = rand::thread_rng();
    format!("DEV{}", rng.gen_range(10000..99999))
}

// Apply Luhn algorithm to generate valid credit card numbers
fn apply_luhn_algorithm(partial: &str) -> String {
    let mut rng = rand::thread_rng();
    let mut number = partial.to_string();
    
    // Complete the number with random digits if needed
    while number.len() < 15 {
        number.push_str(&rng.gen_range(0..=9).to_string());
    }
    
    // Remove the last digit if it exists to calculate the check digit
    let without_check_digit = if number.len() < 16 {
        number.clone()
    } else {
        number[0..number.len()-1].to_string()
    };
    
    // Calculate Luhn sum
    let mut sum = 0;
    let mut double = false;
    
    for c in without_check_digit.chars().rev() {
        if let Some(digit) = c.to_digit(10) {
            let mut value = digit;
            if double {
                value *= 2;
                if value > 9 {
                    value -= 9;
                }
            }
            sum += value;
            double = !double;
        }
    }
    
    // Calculate check digit
    let check_digit = (10 - (sum % 10)) % 10;
    
    format!("{}{}", without_check_digit, check_digit)
}

// Generate a valid credit card number for a specific brand
fn generate_card_number(brand: &CardBrand) -> String {
    // Choose a random prefix
    let prefix = gen_random_element(&brand.prefix);
    
    // Choose a random length
    let length = *gen_random_element(&brand.lengths);
    
    // Generate a partial number with the prefix
    let partial = prefix.clone();
    
    // Apply Luhn algorithm to generate a valid number
    let full_number = apply_luhn_algorithm(&partial);
    
    // Ensure the number has the correct length
    full_number[0..length].to_string()
}

// Generate a CVV code
fn generate_cvv(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let mut cvv = String::new();
    for _ in 0..length {
        cvv.push_str(&rng.gen_range(0..=9).to_string());
    }
    cvv
}

// Generate a single transaction
fn generate_transaction(
    card_brands: &[CardBrand],
    merchants: &[Merchant],
    first_names: &[String],
    last_names: &[String],
    currencies: &[String],
    user_agents: &[String],
) -> Transaction {
    let mut rng = rand::thread_rng();
    
    // Select random elements
    let brand = gen_random_element(card_brands);
    let merchant = gen_random_element(merchants);
    let status: TransactionStatus = rand::random();
    let first_name = gen_random_element(first_names);
    let last_name = gen_random_element(last_names);
    let currency = gen_random_element(currencies);
    let user_agent = gen_random_element(user_agents);
    
    // Generate card number and expiry
    let card_number = generate_card_number(brand);
    let expiry_date = gen_random_expiry_date();
    
    // Generate transaction date
    let transaction_date = gen_random_date();
    
    // Generate amount based on currency
    let amount = if currency == "JPY" {
        rng.gen_range(100..=50000) as f64
    } else {
        (rng.gen_range(1..=1000) as f64) + (rng.gen_range::<f64, _>(0.0..1.0) * 100.0).round() / 100.0
    };
    
    // Generate decline reason if status is declined
    let decline_reason = match status {
        TransactionStatus::Declined => Some(rand::random()),
        _ => None,
    };

    Transaction {
        transaction_id: gen_transaction_id(),
        transaction_date: transaction_date.to_rfc3339(),
        status,
        decline_reason,
        cardholder_name: format!("{} {}", first_name, last_name),
        card_number,
        card_brand: brand.name.clone(),
        card_expiry: expiry_date.to_string(),
        cvv: generate_cvv(brand.cvv_length),
        amount,
        currency: currency.clone(),
        merchant_name: merchant.name.clone(),
        merchant_id: merchant.id.clone(),
        merchant_category: merchant.category.clone(),
        payment_method: "credit_card".to_string(),
        ip_address: gen_ip_address(),
        device_id: gen_device_id(),
        user_agent: user_agent.clone(),
    }
}

// Generate multiple transactions
fn generate_transactions(
    count: usize,
    card_brands: &[CardBrand],
    merchants: &[Merchant],
    first_names: &[String],
    last_names: &[String],
    currencies: &[String],
    user_agents: &[String],
) -> Vec<Transaction> {
    (0..count)
        .map(|_| {
            generate_transaction(
                card_brands,
                merchants,
                first_names,
                last_names,
                currencies,
                user_agents,
            )
        })
        .collect()
}

// Write transactions to a CSV file
fn write_transactions_to_csv(transactions: &[Transaction], filename: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;
    
    // Write headers
    writeln!(
        file,
        "transaction_id,transaction_date,status,decline_reason,cardholder_name,card_number,card_brand,card_expiry,cvv,amount,currency,merchant_name,merchant_id,merchant_category,payment_method,ip_address,device_id,user_agent"
    )?;
    
    // Write data rows
    for tx in transactions {
        let decline_reason = match &tx.decline_reason {
            Some(reason) => match reason {
                DeclineReason::InsufficientFunds => "insufficient_funds",
                DeclineReason::CardExpired => "card_expired",
                DeclineReason::InvalidCard => "invalid_card",
                DeclineReason::SuspiciousActivity => "suspicious_activity",
            },
            None => "",
        };
        
        let status = match tx.status {
            TransactionStatus::Approved => "approved",
            TransactionStatus::Declined => "declined",
            TransactionStatus::Pending => "pending",
            TransactionStatus::Refunded => "refunded",
        };
        
        writeln!(
            file,
            "{},{},{},{},\"{}\",{},{},{},{},{:.2},{},{},{},{},{},{},{},\"{}\"",
            tx.transaction_id,
            tx.transaction_date,
            status,
            decline_reason,
            tx.cardholder_name,
            tx.card_number,
            tx.card_brand,
            tx.card_expiry,
            tx.cvv,
            tx.amount,
            tx.currency,
            tx.merchant_name,
            tx.merchant_id,
            tx.merchant_category,
            tx.payment_method,
            tx.ip_address,
            tx.device_id,
            tx.user_agent
        )?;
    }
    
    Ok(())
}

// Write transactions to a JSON file
fn write_transactions_to_json(transactions: &[Transaction], filename: &str) -> io::Result<()> {
    let json = serde_json::to_string_pretty(transactions)?;
    let mut file = File::create(filename)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

fn main() -> io::Result<()> {
    // Define card brands
    let card_brands = vec![
        CardBrand {
            name: "Visa".to_string(),
            prefix: vec!["4".to_string()],
            lengths: vec![16],
            cvv_length: 3,
        },
        CardBrand {
            name: "Mastercard".to_string(),
            prefix: vec![
                "51".to_string(),
                "52".to_string(),
                "53".to_string(),
                "54".to_string(),
                "55".to_string(),
            ],
            lengths: vec![16],
            cvv_length: 3,
        },
        CardBrand {
            name: "American Express".to_string(),
            prefix: vec!["34".to_string(), "37".to_string()],
            lengths: vec![15],
            cvv_length: 4,
        },
        CardBrand {
            name: "Discover".to_string(),
            prefix: vec![
                "6011".to_string(),
                "644".to_string(),
                "645".to_string(),
                "646".to_string(),
                "647".to_string(),
                "648".to_string(),
                "649".to_string(),
                "65".to_string(),
            ],
            lengths: vec![16],
            cvv_length: 3,
        },
    ];

    // Define merchants
    let merchants = vec![
        Merchant {
            name: "Acme Retail".to_string(),
            id: "MER12345".to_string(),
            category: "Retail".to_string(),
        },
        Merchant {
            name: "Sunshine Groceries".to_string(),
            id: "MER22468".to_string(),
            category: "Grocery".to_string(),
        },
        Merchant {
            name: "Tech Universe".to_string(),
            id: "MER39521".to_string(),
            category: "Electronics".to_string(),
        },
        Merchant {
            name: "Cozy Coffee Shop".to_string(),
            id: "MER41327".to_string(),
            category: "Food & Beverage".to_string(),
        },
        Merchant {
            name: "Fitness Plus".to_string(),
            id: "MER57845".to_string(),
            category: "Health & Fitness".to_string(),
        },
        Merchant {
            name: "BookWorld".to_string(),
            id: "MER61234".to_string(),
            category: "Books & Media".to_string(),
        },
        Merchant {
            name: "QuickMart".to_string(),
            id: "MER78523".to_string(),
            category: "Convenience Store".to_string(),
        },
        Merchant {
            name: "Urban Fashion".to_string(),
            id: "MER84751".to_string(),
            category: "Clothing".to_string(),
        },
        Merchant {
            name: "Travel Now".to_string(),
            id: "MER92456".to_string(),
            category: "Travel".to_string(),
        },
        Merchant {
            name: "Gourmet Dining".to_string(),
            id: "MER10387".to_string(),
            category: "Restaurant".to_string(),
        },
    ];

    // Define first names
    let first_names = vec![
        "John".to_string(),
        "Jane".to_string(),
        "Michael".to_string(),
        "Emily".to_string(),
        "David".to_string(),
        "Sarah".to_string(),
        "Robert".to_string(),
        "Lisa".to_string(),
        "William".to_string(),
        "Emma".to_string(),
        "James".to_string(),
        "Olivia".to_string(),
        "Daniel".to_string(),
        "Sophia".to_string(),
        "Matthew".to_string(),
        "Ava".to_string(),
        "Christopher".to_string(),
        "Mia".to_string(),
        "Andrew".to_string(),
        "Isabella".to_string(),
    ];

    // Define last names
    let last_names = vec![
        "Smith".to_string(),
        "Johnson".to_string(),
        "Williams".to_string(),
        "Brown".to_string(),
        "Jones".to_string(),
        "Garcia".to_string(),
        "Miller".to_string(),
        "Davis".to_string(),
        "Rodriguez".to_string(),
        "Martinez".to_string(),
        "Hernandez".to_string(),
        "Lopez".to_string(),
        "Gonzalez".to_string(),
        "Wilson".to_string(),
        "Anderson".to_string(),
        "Thomas".to_string(),
        "Taylor".to_string(),
        "Moore".to_string(),
        "Jackson".to_string(),
        "Martin".to_string(),
    ];

    // Define currencies
    let currencies = vec![
        "USD".to_string(),
        "EUR".to_string(),
        "GBP".to_string(),
        "CAD".to_string(),
        "AUD".to_string(),
        "JPY".to_string(),
    ];

    // Define user agents
    let user_agents = vec![
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36".to_string(),
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.1.1 Safari/605.1.15".to_string(),
        "Mozilla/5.0 (iPhone; CPU iPhone OS 14_6 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0 Mobile/15E148 Safari/604.1".to_string(),
    ];

    // Generate datasets with different sizes
    println!("Generating test datasets...");
    
    // Small dataset (100 records)
    let small_dataset = generate_transactions(
        100,
        &card_brands,
        &merchants,
        &first_names,
        &last_names,
        &currencies,
        &user_agents,
    );
    
    // Medium dataset (250 records)
    let medium_dataset = generate_transactions(
        250,
        &card_brands,
        &merchants,
        &first_names,
        &last_names,
        &currencies,
        &user_agents,
    );
    
    // Large dataset (500 records)
    let large_dataset = generate_transactions(
        500,
        &card_brands,
        &merchants,
        &first_names,
        &last_names,
        &currencies,
        &user_agents,
    );

    // Write the datasets to files
    println!("Writing datasets to files...");
    
    // CSV Files
    write_transactions_to_csv(&small_dataset, "transactions_100.csv")?;
    write_transactions_to_csv(&medium_dataset, "transactions_250.csv")?;
    write_transactions_to_csv(&large_dataset, "transactions_500.csv")?;
    
    // JSON Files
    write_transactions_to_json(&small_dataset, "transactions_100.json")?;
    write_transactions_to_json(&medium_dataset, "transactions_250.json")?;
    write_transactions_to_json(&large_dataset, "transactions_500.json")?;

    println!("Done! Generated 6 files:");
    println!("- CSV files: transactions_100.csv, transactions_250.csv, transactions_500.csv");
    println!("- JSON files: transactions_100.json, transactions_250.json, transactions_500.json");

    Ok(())
}
