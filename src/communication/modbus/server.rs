//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///                                CONTROLLER - IOT - SERVER MODBUS OS MODBUS LAYER                            ///
///                                DESIGNED BY JACOB MUSSLER                                                   ///
///                                24-DEC-2019 PROOF OF PRINCIPLE FOR RUST LANGUAGE                            ///
///                                BASED OFF OF MODBUS CLIENT IN C# DEVELOPED BY JACOB MUSSLER                 ///
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// Layer created for the ability to use the system for operating systems easily and effectively, 
// Interface is designed for portability to embedded systems using Modbus / Industrial Protocols
use crate::interface::protocol::modbus;
use crate::interface::ethernet;


pub struct Server {
    server:         modbus::server::Server,
    tcp:            ethernet::tcp::server::Server,
}

impl Server {
    pub fn init(instance_name: &str, ip_address: &str, port: &str, unit_identifer: u8) -> Server {
        let server = modbus::server::Server::init(instance_name.to_string(), unit_identifer);
        let instance = Server {
            server:     server,
            tcp:        ethernet::tcp::server::Server::init(instance_name.to_string(), ip_address.to_string(), port.to_string()),
        };

        return instance;
    }

    // Always call before moving to a new thread
    pub fn get_registers(&self) -> modbus::ModbusRegisters  {
        return self.server.get_registers();
    }

    pub fn cycle(&mut self) -> bool {   //Return the result of what is happening in the server
        let result = self.tcp.accept(&self.server.get_connector());

        if result == true {
            self.tcp.open();
        }

        return result;
    }
}

