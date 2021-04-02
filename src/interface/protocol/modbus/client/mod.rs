//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///                                CONTROLLER - IOT - INTERFACE MODBUS TCP CLIENT                              ///
///                                DESIGNED BY JACOB MUSSLER                                                   ///
///                                10-NOV-2019 PROOF OF PRINCIPLE FOR RUST LANGUAGE                            ///
///                                BASED OFF OF MODBUS CLIENT IN C# DEVELOPED BY JACOB MUSSLER                 ///
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// Declarations of the children modules required for communication to the field devices using the Modbus Protocol.
mod reply;
mod request;
use super::ModbusRegisters;
use super::ModbusMessage;
use crate::interface::common::common_library;

pub struct ModbusClient {
    registers:      ModbusRegisters,
    message:        ModbusMessage,
    library:        common_library::CommonLibrary,
    rtu:            bool
}

impl ModbusClient { // Needs to be expanded
    pub fn init(instance_name: String, unit_identifer: u8, rtu: bool) -> ModbusClient {
        let instance = ModbusClient {
            registers:      ModbusRegisters::init(&instance_name.clone(), unit_identifer),
            message:        ModbusMessage::init(),
            library:        common_library::CommonLibrary::init(),
            rtu:            rtu
        };
    
        return instance;
    }

    pub fn get_registers(&self) -> ModbusRegisters {
        return self.registers.clone();
    }
}

#[derive(Clone)]
pub struct ModbusCommand {
    function_code:  u8,
    read_offset:    u16,
    read_quantity:  u16,
    write_offset:   u16,
    write_quantity: u16,
}

impl ModbusCommand
{
    // Initalize the required parameters for the Modbus Command
    pub fn init(function_code: u8, read_offset: u16, read_quantity : u16, write_offset: u16, write_quantity: u16) -> ModbusCommand {
        let parameters = ModbusCommand {
            function_code:  function_code,
            read_offset:    read_offset,
            read_quantity:  read_quantity,
            write_offset:   write_offset,
            write_quantity: write_quantity,
        };

        return parameters;
    }

    pub fn get_function_code(&self) -> u8 {
        return self.function_code;
    }

    pub fn get_read_offset(&self) -> u16 {
        return self.read_offset;
    }

    pub fn get_read_quantity(&self) -> u16 {
        return self.read_quantity;
    }

    pub fn get_write_offset(&self) -> u16 {
        return self.write_offset;
    }

    pub fn get_write_quantity(&self) -> u16 {
        return self.write_quantity;
    }
}

