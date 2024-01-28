use std::net::TcpStream;
use ssh::LocalSession;

pub fn run(session: &mut LocalSession<TcpStream>, command: &String) {
    let exec = session.open_exec().expect("Could not establish an SSH session with remote host");
    let vec: Vec<u8> = exec.send_command(command).expect("Error during command execution");
    println!("{}", String::from_utf8(vec).expect("Could not retrieve output from remote command"));
}
