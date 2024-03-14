mod args;
mod command;
use clap::Parser;
mod session_handler;
use std::net::SocketAddr;
#[macro_use] extern crate log;
use log::{LevelFilter, Metadata, Record}; 

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.target().starts_with("cli_debug::")
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

fn main() {
    log::set_logger(&SimpleLogger).unwrap();
    log::set_max_level(LevelFilter::Info);
    let args = args::Args::parse();
    let addr = SocketAddr::new(args.target, 22);
    let mut session = session_handler::create_session(&addr, &args.user, &args.pass);
    command::run(&mut session, &args.cmd);
}
