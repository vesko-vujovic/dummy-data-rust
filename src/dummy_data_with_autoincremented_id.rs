use clap::Parser;
use fake::faker::address::{en::*, *};
use fake::faker::company::en::*;
use fake::faker::name::en::*;
use fake::faker::phone_number::en::*;
use fake::faker::internet::en::FreeEmail;
use fake::locales::EN;
use fake::Fake;
use indicatif::{ProgressBar, ProgressStyle};
use rand::seq::SliceRandom;
use rand::Rng;
use serde::Serialize;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::atomic::{AtomicI64, Ordering};
use chrono;
use csv;

// Separate counters for each entity
struct EntityCounters {
    user_id: AtomicI64,
    address_id: AtomicI64,
    provider_id: AtomicI64,
    transaction_id: AtomicI64,
}

impl EntityCounters {
    fn new(start_id: i64) -> Self {
        Self {
            user_id: AtomicI64::new(start_id),
            address_id: AtomicI64::new(start_id),
            provider_id: AtomicI64::new(start_id),
            transaction_id: AtomicI64::new(start_id),
        }
    }

    fn next_user_id(&self) -> i64 {
        self.user_id.fetch_add(1, Ordering::SeqCst)
    }

    fn next_address_id(&self) -> i64 {
        self.address_id.fetch_add(1, Ordering::SeqCst)
    }

    fn next_provider_id(&self) -> i64 {
        self.provider_id.fetch_add(1, Ordering::SeqCst)
    }

    fn next_transaction_id(&self) -> i64 {
        self.transaction_id.fetch_add(1, Ordering::SeqCst)
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value_t = 100)]
    users: usize,

    #[clap(short, long, default_value_t = 1000)]
    transactions: usize,

    #[clap(short, long, default_value_t = 10)]
    providers: usize,

    #[clap(short, long)]
    skewed: bool,

    #[clap(short, long, default_value = "output")]
    output_dir: String,

    #[clap(short, long, default_value = "json", value_parser = ["json", "csv"])]
    format: String,

    // Separate start IDs for each entity
    #[clap(long, default_value_t = 1)]
    user_start_id: i64,

    #[clap(long, default_value_t = 1)]
    address_start_id: i64,

    #[clap(long, default_value_t = 1)]
    provider_start_id: i64,

    #[clap(long, default_value_t = 1)]
    transaction_start_id: i64,
}

#[derive(Serialize)]
struct User {
    id: i64,
    name: String,
    email: String,
    phone: String,
}

#[derive(Serialize)]
struct Address {
    id: i64,
    user_id: i64,
    street: String,
    city: String,
    state: String,
    country: String,
    postal_code: String,
}

#[derive(Serialize)]
struct PaymentProvider {
    id: i64,
    name: String,
}

#[derive(Serialize)]
struct Transaction {
    id: i64,
    user_id: i64,
    provider_id: i64,
    amount: f64,
    timestamp: String,
}

enum FileWriter {
    Json(File),
    Csv(csv::Writer<File>),
}

impl FileWriter {
    fn write<T: Serialize>(&mut self, record: &T) -> std::io::Result<()> {
        match self {
            FileWriter::Json(file) => {
                writeln!(file, "{}", serde_json::to_string(record).unwrap())
            }
            FileWriter::Csv(writer) => {
                writer.serialize(record).unwrap();
                Ok(())
            }
        }
    }
}

fn create_writer(dir: &str, name: &str, format: &str) -> FileWriter {
    let extension = format;
    let filename = format!("{}.{}", name, extension);
    let path = Path::new(dir).join(filename);
    match format {
        "json" => FileWriter::Json(
            File::create(&path).expect(&format!("Unable to create file: {:?}", path))
        ),
        "csv" => FileWriter::Csv(
            csv::Writer::from_path(&path).expect(&format!("Unable to create CSV file: {:?}", path))
        ),
        _ => panic!("Unsupported format: {}", format),
    }
}

fn main() {
    let args = Args::parse();

    // Initialize counters with their respective start IDs
    let counters = EntityCounters::new(1);
    counters.user_id.store(args.user_start_id, Ordering::SeqCst);
    counters.address_id.store(args.address_start_id, Ordering::SeqCst);
    counters.provider_id.store(args.provider_start_id, Ordering::SeqCst);
    counters.transaction_id.store(args.transaction_start_id, Ordering::SeqCst);

    std::fs::create_dir_all(&args.output_dir).expect("Unable to create output directory");

    let mut rng = rand::thread_rng();
    let mut users_writer = create_writer(&args.output_dir, "users", &args.format);
    let mut addresses_writer = create_writer(&args.output_dir, "addresses", &args.format);
    let mut providers_writer = create_writer(&args.output_dir, "providers", &args.format);
    let mut transactions_writer = create_writer(&args.output_dir, "transactions", &args.format);

    let progress_style = ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .unwrap()
        .progress_chars("##-");

    // Generate Users
    let users_pb = ProgressBar::new(args.users as u64);
    users_pb.set_style(progress_style.clone());
    users_pb.set_message("Generating Users");

    let users: Vec<User> = (0..args.users)
        .map(|_| {
            let name: String = Name().fake();
            let user = User {
                id: counters.next_user_id(),
                name: name.clone(),
                email: format!("{}@{}.com", name.to_lowercase().replace(' ', "."), FreeEmail().fake::<String>()),
                phone: PhoneNumber().fake(),
            };
            users_writer.write(&user).unwrap();
            users_pb.inc(1);
            user
        })
        .collect();
    users_pb.finish_with_message("Users completed");

    // Generate Addresses
    let addresses_pb = ProgressBar::new(args.users as u64);
    addresses_pb.set_style(progress_style.clone());
    addresses_pb.set_message("Generating Addresses");

    for user in &users {
        let address = Address {
            id: counters.next_address_id(),
            user_id: user.id,
            street: StreetName().fake(),
            city: CityName().fake(),
            state: StateName().fake(),
            country: CountryName().fake(),
            postal_code: PostCode().fake(),
        };
        addresses_writer.write(&address).unwrap();
        addresses_pb.inc(1);
    }
    addresses_pb.finish_with_message("Addresses completed");

    // Generate Providers
    let providers_pb = ProgressBar::new(args.providers as u64);
    providers_pb.set_style(progress_style.clone());
    providers_pb.set_message("Generating Providers");

    let real_providers = vec![
        "Visa", "Mastercard", "American Express", "PayPal", "Stripe",
        "Square", "Alipay", "WeChat Pay", "Apple Pay", "Google Pay",
        "Venmo", "Zelle", "Klarna", "Affirm", "Adyen",
    ];

    let providers: Vec<PaymentProvider> = (0..args.providers)
        .map(|_| {
            let provider = PaymentProvider {
                id: counters.next_provider_id(),
                name: real_providers.choose(&mut rng).unwrap().to_string(),
            };
            providers_writer.write(&provider).unwrap();
            providers_pb.inc(1);
            provider
        })
        .collect();
    providers_pb.finish_with_message("Providers completed");

    // Generate Transactions
    let transactions_pb = ProgressBar::new(args.transactions as u64);
    transactions_pb.set_style(progress_style.clone());
    transactions_pb.set_message("Generating Transactions");

    let mut transactions_count = 0;
    while transactions_count < args.transactions {
        let user_index = if args.skewed {
            (rng.gen::<f64>().powi(3) * users.len() as f64) as usize
        } else {
            rng.gen_range(0..users.len())
        };

        let transaction = Transaction {
            id: counters.next_transaction_id(),
            user_id: users[user_index].id,
            provider_id: providers[rng.gen_range(0..providers.len())].id,
            amount: rng.gen_range(1.0..1000.0),
            timestamp: chrono::Utc::now().to_rfc3339(),
        };

        transactions_writer.write(&transaction).unwrap();
        transactions_count += 1;
        transactions_pb.inc(1);
    }
    transactions_pb.finish_with_message("Transactions completed");

    println!("Data generation complete. Output saved to {} directory in {} format", args.output_dir, args.format);
}