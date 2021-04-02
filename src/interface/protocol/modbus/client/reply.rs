//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///                                CONTROLLER - IOT - MODBUS CLIENT REPLY                                      ///
///                                DESIGNED BY JACOB MUSSLER                                                   ///
///                                10-NOV-2019 PROOF OF PRINCIPLE FOR RUST LANGUAGE                            ///
///                                BASED OFF OF MODBUS CLIENT IN C# DEVELOPED BY JACOB MUSSLER                 ///
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use super::ModbusClient;

impl ModbusClient {
    // Client Response Logic
    // This is received from the TCP Stream
    pub fn response(&self, data: &[u8]) { 
        if data.len() > 3 {             // Take the modbus_packet from the modbus header and the data
            let packet;                 // Take the modbus_packet from the modbus header and the data
            let trans_ident;            // Extract the transaction identifer from modbus response

            if self.rtu == false {
                packet = &data[7 .. data.len()];
                trans_ident = &data[0 .. 2];
            } else {
                packet = &data[1 .. data.len() - 2];  //SERIAL IMPLEMENTATION change the dummy array to the pointer
                trans_ident =  &data[0 .. 1];                 // THIS DOES NOT LOOK RIGHT INVESTIGATE!!!!
            }

            let function_code = packet[0];   // Extract the function code from the modbus packet

            if function_code == 1 {                 // Read Coils            
                self.response_read(packet, trans_ident);
            } else if function_code == 2 {          // Read Discrete Inputs
                self.response_read(packet, trans_ident);
            } else if function_code == 3 {          // Read Holding Registers
                self.response_read(packet, trans_ident);
            } else if function_code == 4 {          // Read Input Registers
                self.response_read(packet, trans_ident);
            } else if function_code == 5 {          // Write Single Coil
                self.response_write(function_code);  
            } else if function_code == 6 {          // Write Single Register
                self.response_write(function_code);              
            } else if function_code == 15 {         // Write Multiple Coils
                self.response_write(function_code);              
            } else if function_code == 16 {         // Write Multiple Registers
                self.response_write(function_code);            
            } else if function_code == 23 {         //Read Write Multiple Registers
                self.response_read(packet, trans_ident);            
            } else {                                // Modbus Error
                let error_code = packet[1];

                println!("Function Code : {}, Error Code : {}", function_code, error_code);
            }     
        } else {
            println!("Not A Valid Modbus Packet");
        }
    }

    fn response_read(&self, packet: &[u8], trans_ident: &[u8]) {                            // Determine the Endianess of the processor
        let function_code = packet[0];                                                      // Extract the function code from the byte stream
        let message_identifer = self.library.u8_slice_to_u16(trans_ident, false) as usize;  // Obtain the message identifer for the offset
        let offset = self.message.get_message_id(message_identifer) as usize;
        let data = &packet[2 .. packet.len()];

        if function_code == 1 || function_code == 2 {
            let mut offset_pointer = offset;

            for byte in data {
                if function_code == 1 {             // Read Coils
                    self.registers.set_coils(offset_pointer, &mut self.library.u8_to_bool(*byte));
                } else if function_code == 2 {      // Read Discrete
                    self.registers.set_discrete_inputs(offset_pointer, &mut self.library.u8_to_bool(*byte));
                }

                offset_pointer += 8;
            }

        } else if function_code == 3 || function_code == 4 || function_code == 23 {
            let mut index = 0;
            let array_length = data.len() / 2;

            while index < array_length {
                let register_place = index * 2;
                let register_slice = &data[register_place .. (register_place + 2)];
                let register = self.library.u8_slice_to_u16(register_slice, false);

                if function_code == 3 || function_code == 23 {  // Read Holding Registers
                    self.registers.set_holding_register(offset + index, register);
                } else if function_code == 4 {                  // Read Input Registers
                    self.registers.set_input_register(offset + index, register);
                }

                index += 1;
            }
        }
    }

    fn response_write(&self, _function_code: u8) {
        // DUMMY AREA THAT IS JUST TO ABSORB THE FUNCTION CODE, MIGHT HAVE FURTHER USE LATER
    }
}