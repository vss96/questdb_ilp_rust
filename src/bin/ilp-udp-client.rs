use questdb_ilp_rust::{LineSenderOperations, LineUdpSender, Result};
use std::net::SocketAddr;
fn main() -> Result<()> {
    let buffer = b"Hello world";
    let ip_address = "127.0.0.1:6379";
    let addr: SocketAddr = ip_address.parse().expect("Couldn't parse ip addresss");
    let ilp_client = LineUdpSender::create_socket(100, addr)?;
    ilp_client.send_to_socket(buffer)?;
    Ok(())
}
