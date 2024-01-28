use std::net::TcpStream;
use ssh::LocalSession;

pub fn run(session: &mut LocalSession<TcpStream>, command: &String) {
    let exec = session.open_exec().unwrap();
    let vec: Vec<u8> = exec.send_command(command).unwrap();
    println!("{}", String::from_utf8(vec).unwrap());
}
