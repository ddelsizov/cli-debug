use ssh::LocalSession;
use std::net::{SocketAddr, TcpStream};

pub fn create_session(addr: &SocketAddr, usr: &String, pass: &String ) -> LocalSession<TcpStream> {
    info!("Connecting to {} via SSH.", addr);
    let session = ssh::create_session()
        .username(usr)
        .password(pass)
        .connect(addr)
        .unwrap()
        .run_local();
    session
}