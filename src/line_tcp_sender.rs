use crate::{LineSenderOperations, Result};
use log::{debug, error};
use std::io::{BufReader, BufWriter, Write};
use std::net::{SocketAddr, TcpStream};

#[derive(Debug)]
pub struct LineTcpSender {
    tcp_reader: BufReader<TcpStream>,
    tcp_writer: BufWriter<TcpStream>,
}

impl LineSenderOperations for LineTcpSender {
    type LineSender = LineTcpSender;

    fn create_socket(ttl: u32, addr: SocketAddr) -> Result<LineTcpSender> {
        let tcp_reader = TcpStream::connect(addr)?;
        tcp_reader.set_ttl(ttl).expect("set_ttl for reader failed");
        let tcp_writer = tcp_reader.try_clone()?;

        Ok(LineTcpSender {
            tcp_reader: BufReader::new(tcp_reader),
            tcp_writer: BufWriter::new(tcp_writer),
        })
    }

    fn send_to_socket(mut self, buf: &[u8]) -> Result<()> {
        self.tcp_writer.write_all(buf)?;
        Ok(())
    }

    fn flush(mut self) -> Result<()> {
        self.tcp_writer.flush()?;
        Ok(())
    }
}
