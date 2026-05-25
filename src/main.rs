mod store;
mod cli;

use tokio;
use clap::Parser;
use store::ServiceList;
use cli::{Commands, Cli};

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let mut services = ServiceList::load();

    match args.command {
        Commands::Add { ip, port, name } => {
            match services.add(&ip, &port, &name) {
                Ok(_) => println!("\nAdded service {name} as {ip}"),
                Err(e) => println!("\n{e}")
            };
        },
        Commands::Delete { name } => {
            match services.delete(&name) {
                Ok(_) => println!("\nDeleted service {name}"),
                Err(e) => println!("\n{e}")
            };
        },
        Commands::List => {
            println!("Your services:\n");
            services.list();
        },
        Commands::GetIp { name } => {
            match services.get_ip(&name) {
                Ok(pth) => println!("{pth}"),
                Err(e) => println!("{e}")
            }
        },
        Commands::TestAll => {
            services.test_all().await
        },
        Commands::Test { name } => {
            match services.test(&name).await {
                Ok(state) => println!("name: {state}"),
                Err(e) => println!("{e}")
            }
        },
        Commands::Update { action, name, new_value } => {
            match services.update(&action, &name, &new_value) {
                Ok(_) => println!("{name}'s {} now is {new_value}", action.to_lowercase()),
                Err(e) => println!("{e}")
            } 
        }
    }
}
