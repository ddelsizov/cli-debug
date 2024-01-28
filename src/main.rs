mod args;
mod session_handler;
mod command;
use std::net::SocketAddr;
use clap::Parser;

fn main() {
    let args = args::Args::parse();
    let addr = SocketAddr::new(args.target, 22);
    let mut session = session_handler::create_session(&addr, &args.user, &args.pass);
    command::run(&mut session, &args.cmd);
}
