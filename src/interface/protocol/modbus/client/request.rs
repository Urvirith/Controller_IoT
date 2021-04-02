//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///                                CONTROLLER - IOT - MODBUS CLIENT REQUEST                                    ///
///                                DESIGNED BY JACOB MUSSLER                                                   ///
///                                10-NOV-2019 PROOF OF PRINCIPLE FOR RUST LANGUAGE                            ///
///                                BASED OFF OF MODBUS CLIENT IN C# DEVELOPED BY JACOB MUSSLER                 ///
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use super::ModbusCommand;
use super::ModbusClient;

impl ModbusClient{
    // Client Response Logic
    // This is received from the TCP Stream
    pub fn request(&mut self, data: &mut [u8], command: &ModbusCommand) -> usize {
        // Declare the modbus request
        let mut len = 0;
        let mut header_len = 0;
        let mut modbus_data: [u8; 256] = [0;256]; 
        let function_code = command.get_function_code();

        if function_code == 1 {                 // Read Coils            
            len = self.request_read(&mut modbus_data, command);
        } else if function_code == 2 {          // Read Discrete Inputs
            len = self.request_read(&mut modbus_data, command);
        } else if function_code == 3 {          // Read Holding Registers
            len = self.request_read(&mut modbus_data, command);
        } else if function_code == 4 {          // Read Input Registers
            len = self.request_read(&mut modbus_data, command);
        } else if function_code == 5 {          // Write Single Coil
            len = self.request_write(&mut modbus_data, command);  
        } else if function_code == 6 {          // Write Single Register
            len = self.request_write(&mut modbus_data, command);            
        } else if function_code == 15 {         // Write Multiple Coils
            len = self.request_write(&mut modbus_data, command);            
        } else if function_code == 16 {         // Write Multiple Registers
            len = self.request_write(&mut modbus_data, command);            
        } else if function_code == 23 {         //Read Write Multiple Registers
            len = self.request_read_write(&mut modbus_data, command);             
        } else {                                // Modbus Error
            println!("{}", "Unidentifed Function Code");
            return len;
        }     

        let message_identifer = self.message.get_message_identifer() as usize;

        // Log the message offset for the read command on return
        // as it is not in the return message     
        self.message.set_message_id(message_identifer, command.get_read_offset());

        if self.rtu == false {
            header_len = self.build_tcp_header(data, len as u16, self.registers.get_unit_identifer());
            self.library.index_array(data, &modbus_data[0..len], header_len);
        } else {
            // Implementation Of The Serial Layer
        }

        let message_identifer = self.message.get_message_identifer();     // Obtain the offset from the the global storage array

        self.message.set_message_identifer(message_identifer + 1);

        if self.message.get_message_identifer() >= 255 {
            self.message.set_message_identifer(0);
        }

        return len + header_len;
    }

    fn request_read(&self, data: &mut [u8], command: &ModbusCommand) -> usize {
        let mut len = 0;

        len = self.library.index_u8(data, command.get_function_code(), len);
        len = self.library.index_array(data, &(self.library.u16_to_u8_array(command.get_read_offset(), false)), len);    // Load the modbus read location
        len = self.library.index_array(data, &(self.library.u16_to_u8_array(command.get_read_quantity(), false)), len);  // Load the modbus read quantity

        return len;
    }

    fn request_write(&self, data: &mut [u8], command: &ModbusCommand) -> usize {
        let mut len = 0;
        let function_code = command.get_function_code();
        let offset = command.get_write_offset() as usize;

        len = self.library.index_u8(data, command.get_function_code(), len);
        len = self.library.index_array(data, &(self.library.u16_to_u8_array(command.get_write_offset(), false)), len); // Load the modbus write location

        // Write a single bit or register
        if function_code == 5 || function_code == 6 {   // Write a single bit or register
            let mut register_value = 0 as u16;

            if function_code == 5 { // Retrieve the bit from the assigned coil array
                if self.registers.get_coil(offset) == true {
                    register_value = 65280;
                } else {
                    register_value = 0;
                }
            } else if function_code == 6 {
                register_value = self.registers.get_holding_register(offset);
            }

            len = self.library.index_array(data, &(self.library.u16_to_u8_array(register_value, false)), len);  // Load the modbus write value
        } else if function_code == 15 || function_code == 16 { // Write multiple coils or registers
            let mut byte_count = 0 as u16; // Define the byte count for sending

            if function_code == 15 {
                if command.get_write_quantity() % 8 == 0 {
                    byte_count = command.get_write_quantity() / 8;
                } else {
                    byte_count = (command.get_write_quantity() / 8) + 1;
                }                
            } else if function_code == 16 {
                byte_count = command.get_write_quantity() * 2;   // Multiply by 2 to gather the number of bytes from 
            }

            len = self.library.index_array(data, &(self.library.u16_to_u8_array(command.get_write_quantity(), false)), len); // Load the modbus quantity to write
            len = self.library.index_u8(data, byte_count as u8, len); // Convert the byte_count to a u8 and push down to the vector array

            if function_code == 15 { // Obtain coil specific data
                let mut address_bool = command.get_write_offset() as usize;  // Assign the offset to the bool array to allow for indexing of the array pointer

                for _index in 0 .. byte_count {
                    let mut bool_data: [bool; 8] = [false; 8];
                    self.registers.get_coils(address_bool, &mut bool_data);
                    len = self.library.index_u8(data, self.library.bool_to_u8(bool_data), len);
                    address_bool = address_bool + 8;    // increment the address pointer by 8 for the number of bits in a byte    
                }
            } else if function_code == 16 { // Obtain holding register specific data
                let address = command.get_write_offset() as usize;
                let quantity = command.get_write_quantity() as usize;

                for index in 0 .. quantity {
                    len = self.library.index_array(data, &(self.library.u16_to_u8_array(self.registers.get_holding_register(address + index), false)), len);
                }
            }
        }
        return len;
    }

    fn request_read_write(&self, data: &mut [u8], command: &ModbusCommand) -> usize{
        let mut len = 0;
        let address = command.get_write_offset() as usize;
        let quantity = command.get_write_quantity() as usize;
        let byte_count = (quantity * 2) as u8;

        len = self.library.index_u8(data, command.get_function_code(), len);
        len = self.library.index_array(data, &(self.library.u16_to_u8_array(command.get_read_offset(), false)), len);    // Load the modbus read location
        len = self.library.index_array(data, &(self.library.u16_to_u8_array(command.get_read_quantity(), false)), len);  // Load the modbus read quantity
        len = self.library.index_array(data, &(self.library.u16_to_u8_array(command.get_write_offset(), false)), len);   // Load the modbus write location
        len = self.library.index_array(data, &(self.library.u16_to_u8_array(command.get_write_quantity(), false)), len); // Load the modbus write quantity
        len = self.library.index_u8(data, byte_count, len);

        for index in 0 .. quantity {
            len = self.library.index_array(data, &(self.library.u16_to_u8_array(self.registers.get_holding_register(address + index), false)), len);
        }

        return len;
    }

    fn build_tcp_header(&mut self, data: &mut [u8], request_size: u16, unit_ident: u8) -> usize { 
        let mut len = 0;
        let protocol_identifer: u16 = 0;    // Standard Modbus TCP Header Requirements
        let mut unit_identifer = unit_ident;
        let modbus_byte_length = request_size + 1;

        if unit_identifer == 0 {
            unit_identifer = 255;
        }

        len = self.library.index_array(data, &(self.library.u16_to_u8_array(self.message.get_message_identifer(), false)), len);    // Load the message id
        len = self.library.index_array(data, &(self.library.u16_to_u8_array(protocol_identifer, false)), len);    // Load the protocol id
        len = self.library.index_array(data, &(self.library.u16_to_u8_array(modbus_byte_length, false)), len);    // Load the length
        len = self.library.index_u8(data, unit_identifer, len);

        return len;
    }  
}