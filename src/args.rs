use clap::Parser;
use std::net::IpAddr;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub target: IpAddr,
    #[arg(short, long)] // Optional: default_value = "blabla"
    pub user: String,
    #[arg(short, long)] // Same ^
    pub pass: String,
    #[arg(short, long)]
    pub cmd: String,
}
