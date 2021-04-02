use crate::interface::ethernet;
use std::result::Result;
use std::io::Error;
use std::net::TcpStream;
pub mod client;
pub mod server;

pub struct Tcp{
    ethernet:       ethernet::Ethernet,
    tcp_conn:       Result<TcpStream, Error>,
}

