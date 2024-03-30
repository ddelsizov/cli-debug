use ssh::LocalSession;
use std::net::TcpStream;

pub fn exec_command(session: &mut LocalSession<TcpStream>, command: &String) {
    info!("Running '{}'", command);
    let exec = session.open_exec().expect("Could not open LocalExec channel to target.");
    let vec: Vec<u8> = exec.send_command(command).expect("Error during command execution");
    info!("Command output:\n");
    println!("{}", String::from_utf8(vec).expect("Could not retrieve output from remote command"));
}
