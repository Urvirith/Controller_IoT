//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///                                CONTROLLER - IOT - DATA STRUCTURES                                          ///
///                                DESIGNED BY JACOB MUSSLER                                                   ///
///                                07-JUN-2018 PROOF OF PRINCIPLE FOR RUST LANGUAGE                            ///
///                                BASED OFF OF MODBUS CLIENT IN C# DEVELOPED BY JACOB MUSSLER                 ///
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct CommonLibrary {
    endianness: bool
}
    
impl CommonLibrary {
    pub fn init() -> CommonLibrary {
        let instance = CommonLibrary {
            endianness: CommonLibrary::test_endianness(),
        };

        return instance;
    } 

    fn test_endianness() -> bool { // Determine the endianess of a system by splitting the integer and Shifting bits and then checking the value
        let mut little_endian = false;
        let integer: u16 = 1;
        let byte_0: u8 = ((integer >> 8) & 0xFF) as u8;
        let byte_1: u8 = (integer & 0xFF) as u8;

        if byte_0 == 1 {
            little_endian = true;
        } else if byte_1 == 1 {
            little_endian = false;
        }

        return little_endian;
    }

    pub fn u16_to_u8_array(&self, integer : u16, little_endian : bool) -> [u8; 2] { // Shift the u16 to a u8 array
        let mut u8_array: [u8; 2] = [0; 2];
        let byte_0: u8 = ((integer >> 8) & 0xFF) as u8;
        let byte_1: u8 = (integer & 0xFF) as u8;

        if self.endianness == true {
            if little_endian == true {
                u8_array[0] = byte_0;
                u8_array[1] = byte_1;
            } else {
                u8_array[0] = byte_1;
                u8_array[1] = byte_0;
            }
        } else {
            if little_endian == true {
                u8_array[0] = byte_1;
                u8_array[1] = byte_0;
            } else {
                u8_array[0] = byte_0;
                u8_array[1] = byte_1;
            }
        }

        return u8_array;
    }

    pub fn u8_slice_to_u16(&self, u8_slice: &[u8], little_endian : bool) -> u16 { // Shift the u16 to a u8 vector
        let mut u8_array: Vec<u8> = vec![0; 2];
        let return_u16: u16;

        if little_endian == true {
            if self.endianness == true {
                u8_array[0] = u8_slice[0];
                u8_array[1] = u8_slice[1];
            } else {
                u8_array[0] = u8_slice[1];
                u8_array[1] = u8_slice[0];
            }
        } else {
            if self.endianness == true {
                u8_array[0] = u8_slice[1];
                u8_array[1] = u8_slice[0];
            } else {
                u8_array[0] = u8_slice[0];
                u8_array[1] = u8_slice[1];
            }
        }

        return_u16 = ((u8_array[0] as u16) << 8 )| ((u8_array[1] << 0) as u16);

        return return_u16;
    }

    pub fn bool_to_u8(&self, bool_array: [bool; 8]) -> u8 { // Convert bool array into a byte and return
        let mut return_byte: u8 = 0;
        let mut index = 0;

        while index < bool_array.len() {
            if bool_array[index] == true { // Shift the bit into byte if true
                return_byte |= 1 << (index);
            }

            index = index + 1;
        }

        return return_byte;
    }

    pub fn u8_to_bool(&self, byte: u8) -> [bool; 8] { // Convert bool array into a byte and return
        let mut return_bool_array: [bool; 8] = [false; 8];
        let mut index = 0;

        while index < 8 {
            if (byte & (1 << index)) == 0 {
                return_bool_array[index] = false;
            } else {
                return_bool_array[index] = true;
            }
            index = index + 1;
        }

        return return_bool_array;
    }

    // Index an array by sequencing the second array into the first array
    pub fn index_u8(&self, data: &mut[u8], data_to_copy: u8, len: usize) -> usize {
        let mut index = len;

        if (1 + index) <= data.len() {
            data[index] = data_to_copy;
            index += 1;

            return index;
        } else {
            return 0;
        }
    }

    // Index an array by sequencing the second array into the first array
    pub fn index_array(&self, data: &mut[u8], data_to_copy: &[u8], len: usize) -> usize {
        let mut index = len;

        if (data_to_copy.len() + index) <= data.len() {
            for point in data_to_copy {
                data[index] = *point;
                index += 1;
            }
            return index;
        } else {
            return 0;
        }
    }
}

