use crate::{LineSenderOperations, Result};
use std::net::{SocketAddr, UdpSocket};

pub struct LineUdpSender {
    socket: UdpSocket,
}

impl LineSenderOperations for LineUdpSender {
    type LineSender = LineUdpSender;
    fn create_socket(ttl: u32, addr: SocketAddr) -> Result<LineUdpSender> {
        //automatically binds to an available port
        let socket = UdpSocket::bind("0.0.0.0:0").expect("Couldn't bind to address");
        socket.connect(addr).expect("connect function failed");
        socket.set_ttl(ttl).expect("set_ttl failed for udp socket");
        Ok(LineUdpSender { socket: socket })
    }

    fn send_to_socket(self, buffer: &[u8]) -> Result<()> {
        self.socket.send(&buffer)?;
        Ok(())
    }

    fn flush(self) -> Result<()> {
        unimplemented!()
    }
}
