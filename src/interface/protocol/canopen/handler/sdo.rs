//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///                                CONTROLLER - IOT - CANOPEN - SDO                                            ///
///                                DESIGNED BY JACOB MUSSLER                                                   ///
///                                17-MAR-2020 PROOF OF PRINCIPLE FOR RUST LANGUAGE                            ///
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::interface::common::bit_mask;
use crate::interface::common::common_library;

// Client Server Model Only will work well with the break down
// const TX_SDO: u8 = 0x0B;
// const RX_SDO: u8 = 0x0C;
const S:        i32 =   0;
const E:        i32 =   1;
const N:        i32 =   2;
const CCS:      i32 =   5;
const TOGGLE:   i32 =   4;
const MIN_LEN:  usize = 4;

// css = bit 5 - 7      ccs client command specifier) is the transfer type (e.g. 1: Download, 2: Upload)
// n = bit 2-3          n is the #bytes in data bytes 4-7 that do not contain data (valid if e & s are set)
// e = bit 1            If set, e indicates an “expedited transfer” (all data is in a single CAN frame)
// s = bit 0            If set, s indicates that data size is shown in n
// Structure is Little endian

pub struct CANOpenSDO {
    node:           u8,
    ccs:            u8,
    n:              u8,
    e:              u8,
    s:              u8,
    toggle:         u8,
    od_index:       u16,
    od_subindex:    u8,
}

impl CANOpenSDO {
    pub fn init_request(node: u8, ccs: u8, n: u8, e: u8, s: u8, od_index: u16, od_subindex: u8) -> CANOpenSDO { // Initialize for the class based polling
        let instance = CANOpenSDO {
            node:           node,
            ccs:            ccs,
            n:              n,
            e:              e,
            s:              s,
            toggle:         0,
            od_index:       od_index,
            od_subindex:    od_subindex,
        };

        return instance;
    } 

    pub fn request(&mut self, raw_data: &[u8], data: &mut [u8; 8]) -> usize {               // Data will be a [u8; 8] array, raw data will be the data to write to canbus device if downloading)
        if data.len() >= 8 {                                                                // ensure the new array length is >= Canbus Frame
            let library = common_library::CommonLibrary::init();                            // Create a new common library instance
            let od_index = library.u16_to_u8_array(self.od_index, true);
            let mut index = MIN_LEN;                                                        // Minimum of index size

            if raw_data.len() <= MIN_LEN {
                self.n = 4 - raw_data.len() as u8;
            } else {
                self.n = 0
            }
    
            data[0] |= (self.ccs & bit_mask::get_three_bit_mask() as u8) << CCS;            // The CCS (client command specifier) is the transfer type (e.g. 1: Download, 2: Upload)
            data[0] |= (self.n & bit_mask::get_two_bit_mask() as u8) << N;                  // n is the #bytes in data bytes 4-7 that do not contain data (valid if e & s are set)
            data[0] |= (self.e & bit_mask::get_one_bit_mask() as u8) << E;                  // If set, e indicates an “expedited transfer” (all data is in a single CAN frame)
            data[0] |= (self.s & bit_mask::get_one_bit_mask() as u8) << S;                  // If set, s indicates that data size is shown in n
            data[1] = od_index[0];                                                          // Load the od_index into canbus data
            data[2] = od_index[1];                                                          // Load the od_index into canbus data
            data[3] = self.od_subindex;                                                     // Load the od_subindex into canbus data

            if raw_data.len() <= 0 {
                for i in 0.. raw_data.len() {
                    if i < MIN_LEN {                                                        // Limit the data to copy 
                        data[index] = raw_data[i];
                        index += 1
                    }
                }
            } 
    
            return index;                                                                   // Will return the size of the data
        } else {
            return 0;
        }
    }

    pub fn request_cont(&mut self, data: &mut [u8; 8], raw_data: &[u8], c: u8) -> usize {   // Data will be a [u8; 8] array, raw data will be the data to write to canbus device if downloading)
        if data.len() >= 8 {                                                                // ensure the new array length is >= Canbus Frame
            let mut index = 1;                                                              // Minimum of index size

            if raw_data.len() <= 7 {
                self.n = raw_data.len() as u8;
            } else {
                self.n = 0
            }
    
            data[0] |= (self.ccs & bit_mask::get_three_bit_mask() as u8) << CCS;            // The CCS (client command specifier) is the transfer type (e.g. 1: Download, 2: Upload)
            data[0] |= (self.toggle & bit_mask::get_one_bit_mask() as u8) << TOGGLE;        // n is the #bytes in data bytes 4-7 that do not contain data (valid if e & s are set)
            data[0] |= (self.n & bit_mask::get_two_bit_mask() as u8) << E;                  // If set, e indicates an “expedited transfer” (all data is in a single CAN frame)
            data[0] |= (c & bit_mask::get_one_bit_mask() as u8) << S;                       // If set, s indicates that data size is shown in n

            if self.toggle == 0 {                                                           // Invert on each node guarding response
                self.toggle = 1;
            } else {
                self.toggle = 0;
            }

            if raw_data.len() <= 0 {
                for i in 0.. raw_data.len() {
                    data[index] = raw_data[i];
                    index += 1
                }
            }     
            return index;                                                                   // Will return the size of the data
        } else {
            return 0;
        }
    }
    // END DOWNLOAD WILL REQUIRE THE GENERATION OF A CRC CHECK SUM THAT CONSUMES 16-BITS

    pub fn init_reply(node: u8) -> CANOpenSDO {                                             // Can be initialized before decoding the message
        let instance = CANOpenSDO {
            node:           node,
            ccs:            0,
            n:              0,
            e:              0,
            s:              0,
            toggle:         0,
            od_index:       0,
            od_subindex:    0
        };

        return instance;
    } 

    pub fn reply(&mut self, data: &mut [u8; 4], raw_data: &[u8]) -> usize {                 // Deconstruct the response // INITIAL RESPONSE, FOR BULK UPLOADS WILL HAVE TO REMOVE REST OF THE DATA
        if raw_data.len() >= MIN_LEN {
            let library = common_library::CommonLibrary::init();
            let mut index = 0;   

            self.ccs = (raw_data[0] >> CCS) & bit_mask::get_three_bit_mask() as u8 ;
            self.n = (raw_data[0] >> N) & bit_mask::get_two_bit_mask() as u8;
            self.e = (raw_data[0] >> E) & bit_mask::get_one_bit_mask() as u8;
            self.s = (raw_data[0] >> S) & bit_mask::get_one_bit_mask() as u8;
            self.od_index = library.u8_slice_to_u16(&raw_data[1..3], true);
            self.od_subindex = raw_data[3];

            if raw_data.len() > MIN_LEN {
                for i in 0.. raw_data.len() - MIN_LEN {
                    data[i] = raw_data[i + MIN_LEN];
                    index += 1;
                }  
                return index;
            }
            return index;                                                                   // Return the length of the data returned
        } else {                                                                            // Below the minimum length and will not be deconstructed
            return 0;
        }
    }

    pub fn reply_cont(&mut self, data: &mut [u8; 7], raw_data: &[u8]) -> usize {            // Deconstruct the response // INITIAL RESPONSE, FOR BULK UPLOADS WILL HAVE TO REMOVE REST OF THE DATA
        if raw_data.len() >= 1 {
            let mut index = 0;
            let len = 1;                                                                    // Minimum length  

            self.ccs = (raw_data[0] >> CCS) & bit_mask::get_three_bit_mask() as u8 ;
            self.n = (raw_data[0] >> N) & bit_mask::get_two_bit_mask() as u8;
            self.e = (raw_data[0] >> E) & bit_mask::get_one_bit_mask() as u8;
            self.s = (raw_data[0] >> S) & bit_mask::get_one_bit_mask() as u8;

            if raw_data.len() > len {
                for i in 0.. raw_data.len() - len {
                    data[i] = raw_data[i + len];
                    index += 1;
                }  
                return index;
            }
            return index;                                                                   // Return the length of the data returned
        } else {                                                                            // Below the minimum length and will not be deconstructed
            return 0;
        }
    }

    pub fn get_node(&self) -> u8 {
        return self.node
    }

    pub fn get_ccs(&self) -> u8 {
        return self.ccs
    }

    pub fn get_n(&self) -> u8 {
        return self.n
    }

    pub fn get_e(&self) -> u8 {
        return self.e
    }

    pub fn get_s(&self) -> u8 {
        return self.s
    }

    pub fn get_od_index(&self) -> u16 {
        return self.od_index
    }

    pub fn get_od_subindex(&self) -> u8 {
        return self.od_subindex
    }
}
