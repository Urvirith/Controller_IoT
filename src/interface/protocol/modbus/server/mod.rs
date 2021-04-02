//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///                                CONTROLLER - IOT - TCP CLIENT - MODBUS_SERVER_INTERFACE                     ///
///                                DESIGNED BY JACOB MUSSLER                                                   ///
///                                12-JUL-2018 PROOF OF PRINCIPLE FOR RUST LANGUAGE                            ///
///                                BASED OFF OF MODBUS CLIENT IN C# DEVELOPED BY JACOB MUSSLER                 ///
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////

mod server;
use super::ModbusRegisters;
use crate::interface::common::common_library;

#[derive(Clone)]
pub struct Server {
    registers:      ModbusRegisters,
    library:        common_library::CommonLibrary
}

impl Server {
    pub fn init(instance_name: String, unit_identifer: u8) -> Server {
        let instance = Server {
            registers:      ModbusRegisters::init(&instance_name, unit_identifer),
            library:        common_library::CommonLibrary::init()
        };

        return instance;
    }

    pub fn get_registers(&self) -> ModbusRegisters {
        return self.registers.clone();
    }

    pub fn get_connector(&self) -> impl Connector {
        return self.clone();
    }
}

pub trait Connector {
    fn connector(&self, data_req: &[u8], data_res: &mut [u8]) -> usize;
}

impl Connector for Server {
    fn connector(&self, data_req: &[u8], data_res: &mut [u8]) -> usize {
        return self.request(data_req, data_res);
    }
}

