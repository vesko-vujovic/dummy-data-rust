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
use uuid::Uuid;
use chrono;

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
}

#[derive(Serialize)]
struct User {
    id: String,
    name: String,
    email: String,
    phone: String,
}

#[derive(Serialize)]
struct Address {
    user_id: String,
    street: String,
    city: String,
    state: String,
    country: String,
    postal_code: String,
}

#[derive(Serialize)]
struct PaymentProvider {
    id: String,
    name: String,
}

#[derive(Serialize)]
struct Transaction {
    id: String,
    user_id: String,
    provider_id: String,
    amount: f64,
    timestamp: String,
}

fn create_file(dir: &str, name: &str) -> File {
    let path = Path::new(dir).join(name);
    File::create(&path).expect(&format!("Unable to create file: {:?}", path))
}

fn main() {
    let args = Args::parse();

    std::fs::create_dir_all(&args.output_dir).expect("Unable to create output directory");

    let mut rng = rand::thread_rng();
    let mut users_file = create_file(&args.output_dir, "users.jsonl");
    let mut addresses_file = create_file(&args.output_dir, "addresses.jsonl");
    let mut providers_file = create_file(&args.output_dir, "providers.jsonl");
    let mut transactions_file = create_file(&args.output_dir, "transactions.jsonl");

    let progress_style = ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .unwrap()
        .progress_chars("##-");

    // Generate users
    let users_pb = ProgressBar::new(args.users as u64);
    users_pb.set_style(progress_style.clone());
    users_pb.set_message("Generating Users");

    let users: Vec<User> = (0..args.users)
        .map(|_| {
            let name: String = Name().fake();
            let user = User {
                id: uuid::Uuid::new_v4().to_string(),
                name: name.clone(),
                email: format!("{}@{}.com", name.to_lowercase().replace(' ', "."), FreeEmail().fake::<String>()),
                phone: PhoneNumber().fake(),
            };
            writeln!(users_file, "{}", serde_json::to_string(&user).unwrap()).unwrap();
            users_pb.inc(1);
            user
        })
        .collect();
    users_pb.finish_with_message("Users completed");

    // Generate addresses
    let addresses_pb = ProgressBar::new(args.users as u64);
    addresses_pb.set_style(progress_style.clone());
    addresses_pb.set_message("Generating Addresses");

    for user in &users {
        let address = Address {
            user_id: user.id.clone(),
            street: StreetName().fake(),
            city: CityName().fake(),
            state: StateName().fake(),
            country: CountryName().fake(),
            postal_code: PostCode().fake(),
        };
        writeln!(addresses_file, "{}", serde_json::to_string(&address).unwrap()).unwrap();
        addresses_pb.inc(1);
    }
    addresses_pb.finish_with_message("Addresses completed");

    // Generate payment providers
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
                id: uuid::Uuid::new_v4().to_string(),
                name: real_providers.choose(&mut rng).unwrap().to_string(),
            };
            writeln!(providers_file, "{}", serde_json::to_string(&provider).unwrap()).unwrap();
            providers_pb.inc(1);
            provider
        })
        .collect();
    providers_pb.finish_with_message("Providers completed");

    // Generate transactions
    let transactions_pb = ProgressBar::new(args.transactions as u64);
    transactions_pb.set_style(progress_style.clone());
    transactions_pb.set_message("Generating Transactions");

    let mut transactions_count = 0;
    while transactions_count < args.transactions {
        let user_index = if args.skewed {
            // Use a power-law distribution for skewed data
            (rng.gen::<f64>().powi(3) * users.len() as f64) as usize
        } else {
            rng.gen_range(0..users.len())
        };

        let transaction = Transaction {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: users[user_index].id.clone(),
            provider_id: providers[rng.gen_range(0..providers.len())].id.clone(),
            amount: rng.gen_range(1.0..1000.0),
            timestamp: chrono::Utc::now().to_rfc3339(),
        };

        writeln!(transactions_file, "{}", serde_json::to_string(&transaction).unwrap()).unwrap();
        transactions_count += 1;
        transactions_pb.inc(1);
    }
    transactions_pb.finish_with_message("Transactions completed");

    println!("Data generation complete. Output saved to {} directory", args.output_dir);
}