use crate::interface::ethernet;
use std::result::Result;
use std::io::Error;
use std::net::UdpSocket;
pub mod comms;

pub struct Udp{
    local:          ethernet::Ethernet,
    target:         ethernet::Ethernet,
    udp_conn:       Result<UdpSocket, Error>,
}