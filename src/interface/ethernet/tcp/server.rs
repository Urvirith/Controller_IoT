//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///                                CONTROLLER - IOT - TCP SERVER                                               ///
///                                DESIGNED BY JACOB MUSSLER                                                   ///
///                                08-DEC-2019 PROOF OF PRINCIPLE FOR RUST LANGUAGE                            ///
///                                BASED OFF OF MODBUS CLIENT IN C# DEVELOPED BY JACOB MUSSLER                 ///
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////

//DEVELOP A COMMON INTERFACE FOR THIS SERVER // PASS THE COMMON CONNECTOR AND IT CAN BE DECOUPLED
use crate::interface::protocol::modbus::server::Connector; // FIGURE OUT THE GENERIC

use crate::interface::ethernet;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Write;
use std::io::Read;
use std::io::Error;

pub struct Server{
    ethernet:       ethernet::Ethernet,
    listener:       Result<TcpListener, Error>,
}

impl Server{
    pub fn init(instance_name: String, ip: String, port: String) -> Server {
        let ethernet = ethernet::Ethernet::init(instance_name, ip, port);
        let socket = ethernet.get_socket();
        let instance = Server {
            ethernet:   ethernet,
            listener:   TcpListener::bind(socket),     
        };

        return instance;
    }

    pub fn open(&mut self) {
        self.listener = TcpListener::bind(self.ethernet.get_socket());
    }

    pub fn accept(&self, connector: &dyn Connector) -> bool { // Set up the connection parameters to allow for connection to the host machine return false if good
        println!("{}", "Server Created".to_string());

        match &self.listener {
            Ok(server) => {
                for connection in server.incoming() { // Will need to be a parallel thread
                    println!("Connection Made");
                    match connection {
                        Ok(stream) => {
                            self.client(stream, connector);
                        }
                        Err(err) => {
                            println!("{} : {}", self.ethernet.get_instance_name(), err);
                            return true;
                        }
                    }
                }
            } Err(err) => {
                println!("{} : {}", self.ethernet.get_instance_name(), err);
                return true;
            }
        }
        return false;
    }

    fn client(&self, mut stream: TcpStream, connector: &dyn Connector) -> bool {
        loop { // Looks like this is nessicary for ignition to work, investigate for 
            let mut input_data: [u8; 1024] = [0; 1024];

            if let Ok(size) = stream.read(&mut input_data) {
                if size > 0 {
                    let mut output_data: [u8; 1024] = [0; 1024];
    
                    let len = connector.connector(&input_data[0.. size], &mut output_data);
                    if len > 0 {
                        if let Err(err) = stream.write(&output_data[0.. len]) {
                            println!("{}", err);
                            return true;
                        }
                        stream.flush().unwrap();
                    }
                } else {
                    println!("Read Size");
                    return true;
                }
            } else {
                println!("Read Error");
                return true;
            }
        }
    }

    pub fn close(self) {
        // Consume the instance
    }
}

