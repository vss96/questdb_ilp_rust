use crate::Result;
use std::net::SocketAddr;

pub trait LineSenderOperations {
    type LineSender;

    fn create_socket(ttl: u32, addr: SocketAddr) -> Result<Self::LineSender>;
    fn send_to_socket(self, buf: &[u8]) -> Result<()>;
    fn flush(self) -> Result<()>;
}
