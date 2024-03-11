mod args;
mod command;
use clap::Parser;
mod session_handler;
use std::net::SocketAddr;

fn main() {
    let args = args::Args::parse();
    let addr = SocketAddr::new(args.target, 22);
    let mut session = session_handler::create_session(&addr, &args.user, &args.pass);
    command::run(&mut session, &args.cmd);
}
