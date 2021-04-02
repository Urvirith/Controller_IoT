use super::Udp;

use crate::interface::ethernet;
use std::net::UdpSocket;

impl Udp {
    pub fn init(instance_name: String, local_ip: String, local_port: String, target_ip: String, target_port: String) -> Udp {
        let instance = instance_name;
        let ethernet = ethernet::Ethernet::init(instance.clone(), local_ip, local_port);
        let instance = Udp {
            local:      ethernet.clone(),
            target:     ethernet::Ethernet::init(instance, target_ip, target_port),
            udp_conn:   UdpSocket::bind(ethernet.get_socket()),
        };

        return instance;
    }

    pub fn connect(&mut self) -> bool {
        match self.udp_conn.as_ref() {  // Match the connection state of the system
            Ok(socket) => {
                match socket.connect(self.target.get_socket()) {
                    Ok(_) => {
                        return true;
                    } Err(e) => {
                        println!("{}", e); // IMPLEMENT A RETURN PATH FOR THIS
                        return false;
                    }
                } 
            } Err(e) => {
                println!("{}", e); // IMPLEMENT A RETURN PATH FOR THIS
                return false;
            }
        }
    }

    pub fn read(&mut self, data: &mut[u8]) -> usize {
        match self.udp_conn.as_ref() {  // Match the connection state of the system
            Ok(socket) => {
                match socket.recv(data) {
                    Ok(size) => {
                        return size;
                    } Err(e) => {
                        println!("{}", e); // IMPLEMENT A RETURN PATH FOR THIS
                        return 0;
                    }
                } 
            } Err(e) => {
                println!("{}", e); // IMPLEMENT A RETURN PATH FOR THIS
                return 0;
            }
        }
    }

    pub fn write(&mut self, data: &[u8]) -> usize {
        match self.udp_conn.as_ref() {
            Ok(socket) => {
                match socket.send(data) {
                    Ok(size) => {
                        return size;
                    } Err(e) => {
                        println!("{}", e); // IMPLEMENT A RETURN PATH FOR THIS
                        return 0;
                    }
                } 
            }
            Err(e) => {
                println!("{}", e); // IMPLEMENT A RETURN PATH FOR THIS
                return 0;
            }
        }
    }

    pub fn close(self) {
       // Deallocated the UDP session, will have to recreated if wanting a new connection
    }

}