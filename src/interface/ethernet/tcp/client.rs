use super::Tcp;
use crate::interface::ethernet;
use std::net::TcpStream;
use std::net::Shutdown;
use std::io::Write;
use std::io::Read;


impl Tcp {
    pub fn init(instance_name: String, ip: String, port: String) -> Tcp {
        let ethernet = ethernet::Ethernet::init(instance_name, ip, port);
        let instance = Tcp {
            ethernet:   ethernet.clone(),
            tcp_conn:   TcpStream::connect(ethernet.get_socket()),
        };

        return instance;
    }

    pub fn connect(&mut self) {
        self.tcp_conn = TcpStream::connect(self.ethernet.get_socket());
    }

    pub fn read(&mut self, data: &mut[u8]) -> usize {
        match self.tcp_conn.as_ref() {      // Match the connection state of the system
            Ok(mut stream) => {
                match stream.read(data) {
                    Ok(size) => {
                        return size;
                    } Err(e) => {
                        println!("{}", e);  // IMPLEMENT A RETURN PATH FOR THIS
                        return 0;
                    }
                } 
            } Err(e) => {
                println!("{}", e);          // IMPLEMENT A RETURN PATH FOR THIS
                return 0;
            }
        }
    }

    pub fn write(&mut self, data: &[u8]) -> usize {
        match self.tcp_conn.as_ref() {
            Ok(mut stream) => {
                match stream.write(data) {
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

    pub fn close(&self) {
        match self.tcp_conn.as_ref() {
            Ok(stream) => {
                match stream.shutdown(Shutdown::Both) {
                    Ok(_) => {
                        println!("{}: shutdown", self.ethernet.instance_name);
                    } Err(e) => {
                        println!("{}", e); // IMPLEMENT A RETURN PATH FOR THIS
                    }
                } 
            }
            Err(e) => {
                println!("{}", e); // IMPLEMENT A RETURN PATH FOR THIS
            }
        }
    }

}