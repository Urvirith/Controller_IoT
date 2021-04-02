//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///                                CONTROLLER - IOT - TCP CLIENT - MODBUS_SERVER                               ///
///                                DESIGNED BY JACOB MUSSLER                                                   ///
///                                17-NOV-2019 PROOF OF PRINCIPLE FOR RUST LANGUAGE                            ///
///                                BASED OFF OF MODBUS CLIENT IN C# DEVELOPED BY JACOB MUSSLER                 ///
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use super::Server;

impl Server{
    // Client Response Logic
    // This is received from the TCP Stream
    pub fn request(&self, data_req: &[u8], data_res: &mut [u8]) -> usize {
        let header = &data_req[0 .. 7];                                 // Take the modbus header from the modbus request
        let packet = &data_req[7 .. data_req.len()];                    // Take the modbus_packet from the modbus header and the data
        let function_code = packet[0];                                  // Extract the function code from the modbus packet
        let mut data: [u8; 512] = [0; 512];
        let mut len = 0;
        let header_len = self.library.index_array(data_res, header, 0);    // Index back the header

        if function_code == 1 {             // Read Coils
            len = self.request_read(packet, &mut data);
        } else if function_code == 2 {      // Read Discrete Inputs
            len = self.request_read(packet, &mut data);
        } else if function_code == 3 {      // Read Holding Registers
            len = self.request_read(packet, &mut data);
        } else if function_code == 4 {      // Read Input Registers
            len = self.request_read(packet, &mut data);
        } else if function_code == 5 {      // Write Single Coil
            len = self.request_write(packet, &mut data);  
        } else if function_code == 6 {      // Write Single Register
            len = self.request_write(packet, &mut data);              
        } else if function_code == 15 {     // Write Multiple Coils 
            len = self.request_write(packet, &mut data);              
        } else if function_code == 16 {     // Write Multiple Registers
            len = self.request_write(packet, &mut data);            
        } else if function_code == 23 {     //Read Write Multiple Registers
            len = self.request_read(packet, &mut data);            
        } else {                            // Modbus Error
            println!("Malformed Modbus Packet From Client");

            len = self.library.index_u8(&mut data, 80 + function_code, len);
            len = self.library.index_u8(&mut data, 1, len);
        }

        let set_len = len + 1;
        len = self.library.index_array(data_res, &data[0 .. len], header_len);

        data_res[5] = set_len as u8;

        return len;     
    }

    fn request_read(&self, packet: &[u8], data: &mut [u8]) -> usize {
        let function_code = packet[0];   // Extract the function code from the byte stream
        let mut len = 0;

        // ERROR CHECKING AREA
        if packet.len() < 4 {    // Build the error response and return;
            len = self.library.index_u8(data, 80 + function_code, len);
            len = self.library.index_u8(data, 3, len);

            return len;
        }

        let starting_address = &packet[1 .. 3];                                                     // Obtain the starting address
        let offset = self.library.u8_slice_to_u16(starting_address, false);             // Obtain the offset from the the packet
        let raw_quantity = &packet[3 .. 5];                                                         // Obtain the quantity from the packet
        let quantity = self.library.u8_slice_to_u16(raw_quantity, false);               // Obtain the quantity from the packet
        let quantity_u8 = quantity as u8;
        let mut byte_count = 0;                                                                     // Build the encapsulated register data

        if function_code == 1 || function_code == 2 {
            if quantity % 8 == 0 {
                byte_count = quantity_u8 / 8;
            } else {
                byte_count = (quantity_u8 / 8) + 1;
            }       
        }
        else if function_code == 3 || function_code == 4 || function_code == 23 {
            byte_count = quantity_u8 * 2;
        }

        // ERROR CHECKING AREA
        if byte_count > 250 {   // Check to ensure the number of bytes allowed which is 250
            len = self.library.index_u8(data, 80 + function_code, len);
            len = self.library.index_u8(data, 3, len);

            return len;
        }

         // ERROR CHECKING AREA
        let mut length = 0 as u16;

        if function_code == 1 { // Obtain the length of the array being used
            length = self.registers.get_coils_len() as u16;
        } else if function_code == 2 {
            length = self.registers.get_discrete_inputs_len() as u16;
        } else if function_code == 3 {
            length = self.registers.get_holding_registers_len() as u16;
        } else if function_code == 4 {
            length = self.registers.get_input_registers_len() as u16;
        } else if function_code == 23 {
            length = self.registers.get_holding_registers_len() as u16;
        }

        // Check if offset is greater than the length or the offset plus the byte count
        if offset > length || (offset + quantity) > length {
            len = self.library.index_u8(data, 80 + function_code, len);
            len = self.library.index_u8(data, 2, len);

            return len;
        }

        len = self.library.index_u8(data, function_code, len);
        len = self.library.index_u8(data, byte_count, len);
                
        if function_code == 1 || function_code == 2 {
            if function_code == 1 || function_code == 2 { // Read Coils or Discrete
                let mut address_bool = offset as usize; // Assign the offset to the bool array to allow for indexing of the array pointer

                for _index in 0.. byte_count {
                    let mut byte = 0 as u8;   // Obtain a slice from the modbus registers
                    let mut bool_data: [bool; 8] = [false; 8];

                    if function_code == 1 {
                        self.registers.get_coils(address_bool, &mut bool_data);
                        byte = self.library.bool_to_u8(bool_data);
                    } else if function_code == 2 {
                        self.registers.get_discrete_inputs(address_bool, &mut bool_data);
                        byte = self.library.bool_to_u8(bool_data);
                    }

                    len = self.library.index_u8(data, byte, len);
                    address_bool = address_bool + 8;    // increment the address pointer by 8 for the number of bits in a byte
                }
            }
        } else if function_code == 3 || function_code == 4 || function_code == 23 {
            let address_usize = offset as usize;
            let quantity_usize = quantity as usize;

            if function_code == 3 || function_code == 23 {
                for index in 0.. quantity_usize {
                    len = self.library.index_array(data, &(self.library.u16_to_u8_array(self.registers.get_holding_register(address_usize + index), false)), len);
                }
            } else if function_code == 4 {
                for index in 0.. quantity_usize {
                    len = self.library.index_array(data, &(self.library.u16_to_u8_array(self.registers.get_input_register(address_usize + index), false)), len);
                }
            }
  
            if function_code == 23 { // Write of data
                self.request_write(packet, data);
            }
        }
        
        return len;
    }

    fn request_write(&self, packet: &[u8], data: &mut [u8]) -> usize {
        let function_code = packet[0];
        let packet_conversion;
        let mut len = 0;

        // ERROR CHECKING AREA
        if packet.len() < 4 {                                                                       // Build the error response and return;
            len = self.library.index_u8(data, 80 + function_code, len);
            len = self.library.index_u8(data, 3, len);

            return len;
        }

        if function_code == 23 {                                                                    // Retrieve the write data from the packet
            packet_conversion = &packet[5 .. packet.len()];
        } else {
            packet_conversion = &packet[1 .. packet.len()];
        }

        let offset =        self.library.u8_slice_to_u16(&packet_conversion[0 .. 2], false);    // Obtain the offset from the the packet                                          // Obtain the quantity from the packet
        let quantity =      self.library.u8_slice_to_u16(&packet_conversion[2 .. 4], false);    // Obtain the quantity from the packet
        let byte_count:     u8;

        if function_code == 15 || function_code == 16 || function_code == 23 {                              // Obtain byte count from the packet
            byte_count = packet_conversion[4];
        } else {
            byte_count = 0;
        }

        // ERROR CHECKING AREA
        if byte_count > 250 { // Check to ensure the number of bytes allowed which is 250
            len = self.library.index_u8(data, 80 + function_code, len);
            len = self.library.index_u8(data, 3, len);

            return len;
        }

        // ERROR CHECKING AREA
        let mut length = 0 as u16;

        if function_code == 5 || function_code == 15 {                  // Obtain the length of the array being used
            length = self.registers.get_coils_len() as u16;
        } else if function_code == 6 || function_code == 16 || function_code == 23 {
            length = self.registers.get_holding_registers_len() as u16;
        }

        if function_code == 5 || function_code == 6 {                   // Check if offset is greater than the length or the offset plus the byte count            
            if offset > length {
                len = self.library.index_u8(data, 80 + function_code, len);
                len = self.library.index_u8(data, 2, len);
    
                return len;
            }
        } else {                                                        // Check if offset is greater than the length or the offset plus the byte count            
            if offset > length || (offset + quantity) > length {
                len = self.library.index_u8(data, 80 + function_code, len);
                len = self.library.index_u8(data, 2, len);
    
                return len;
            }
        }

        if function_code == 5 || function_code == 6 {                   // Write a single bit or register
            let register_value = quantity;
            let offset_usize = offset as usize;

            if function_code == 5 {
                if register_value == 65280 {                            // Retrieve the bit from the assigned coil array
                    self.registers.set_coil(offset_usize, true);
                } else {                                                // Declare the modbus response
                    self.registers.set_coil(offset_usize, false);
                }
            } else if function_code == 6 {                              // Set the register value from information passed back in
                self.registers.set_holding_register(offset_usize, register_value);
            }
        } else if function_code == 15 || function_code == 16 || function_code == 23 {   // Write multiple coils or registers
            let encapsulated_data = &packet_conversion[5 .. packet_conversion.len()];   // Retrieve the data from the converted packet

            if function_code == 15 {                                    // Obtain coil specific data
                let mut offset_pointer = offset as usize;

                for byte in encapsulated_data {
                    let bool_array = self.library.u8_to_bool(*byte);

                    self.registers.set_coils(offset_pointer, &bool_array);
                    offset_pointer = offset_pointer + 8;
                }
            } else if function_code == 16 || function_code == 23 {      // Obtain holding register specific data
                let array_length = encapsulated_data.len() / 2;

                for index in 0.. array_length {
                    let register_place = index * 2;
                    let register_slice = &encapsulated_data[register_place .. (register_place + 2)];

                    self.registers.set_holding_register(offset as usize, self.library.u8_slice_to_u16(register_slice, false));
                }
            }
        }

        if function_code != 23 {
            len = self.library.index_u8(data, function_code, len);
            len = self.library.index_array(data, &packet_conversion[0 .. packet_conversion.len()], len);

            return len;
        } else {
            len = self.library.index_array(data, &packet_conversion[0 .. packet_conversion.len()], len);

            return len;
        }
    }

    pub fn close(self) {
        // Destroy Instance
    }
}