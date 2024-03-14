use ssh::LocalSession;
use std::net::TcpStream;

pub fn run(session: &mut LocalSession<TcpStream>, command: &String) {
    info!("Running {}", command);
    let exec = session.open_exec().expect("Could not establish an SSH session with remote host");
    let vec: Vec<u8> = exec.send_command(command).expect("Error during command execution");
    info!("Result:\n");
    println!("{}", String::from_utf8(vec).expect("Could not retrieve output from remote command"));
}
