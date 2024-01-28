use std::net::{SocketAddr, TcpStream};
use ssh::LocalSession;

pub fn create_session(addr: &SocketAddr, usr: &String, pass: &String ) -> LocalSession<TcpStream> {
    let session = ssh::create_session()
        .username(usr)
        .password(pass)
        .connect(addr)
        .unwrap()
        .run_local();
    session
}