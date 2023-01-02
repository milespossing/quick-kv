use clap::{Command, Parser, Subcommand};
use thin_kv::{Vault, encoding::basic::BasicEncoder};

#[derive(Debug, Parser)]
#[command(name = "quick kv")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(long)]
    path: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Get {
        key: String,
    },
    Set {
        key: String,
        value: String,
    }
}

fn main() {
    let args = Cli::parse();
    let path: String = match &args.path {
        Some(p) => p.to_owned(),
        None => {
            let home = std::env::var("HOME").expect("HOME environment variable not set");
            let home_path = std::path::Path::new(&home);
            let kv_path = std::path::Path::join(&home_path, ".kv/kv.txt");
            kv_path.into_os_string().into_string().unwrap()
        }
    };
    match args.command {
        Commands::Get { key } => {
            let vault: Vault = Vault::new::<BasicEncoder>(path);
            match vault.get(key) {
                Some(s) => println!("{}", s),
                None => println!("key not found"),
            }
        },
        Commands::Set { key, value } => {
            let mut vault: Vault = Vault::new::<BasicEncoder>(path);
            vault.set(key, value);
            vault.commit::<BasicEncoder>();
        },
    }
}
