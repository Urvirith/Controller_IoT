//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///                                CONTROLLER - IOT - CANOPEN - CLIENT                                         ///
///                                DESIGNED BY JACOB MUSSLER                                                   ///
///                                17-MAR-2020 PROOF OF PRINCIPLE FOR RUST LANGUAGE                            ///
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
use crate::interface::common::bit_mask;
mod consumer;
mod producer;
mod nmt;
mod sdo;
mod sfo;

const S:        i32 =   0;
const E:        i32 =   1;
const N:        i32 =   2;
const CCS:      i32 =   5;
const TOGGLE:   i32 =   4;

pub struct CANOpen {
    function_code:  u8,
    node_id:        u8,
    rtr:            u8,
    data_length:    u8
}

impl CANOpen {
    pub fn init(function_code: u8, node_id: u8, rtr : u8, data_length: u8) -> CANOpen {
        let instance = CANOpen {
            function_code:  function_code,
            node_id:        node_id,
            rtr:            rtr,
            data_length:    data_length
        };

        return instance;
    }

    // Obtain the main parameters
    pub fn get_function_code(&self) -> u8 {
        return self.function_code;
    }

    pub fn get_node_id(&self) -> u8 {
        return self.node_id;
    }

    pub fn get_rtr(&self) -> u8 {
        return self.rtr;
    }

    pub fn get_data_length(&self) -> u8 {
        return self.data_length;
    }

    pub fn build_header(&self) -> [u8; 2] {
        raw_header: u16 = 0;

        raw_header |= (self.function_code & bit_mask::get_four_bit_mask() as u8) << CCS;            // The CCS (client command specifier) is the transfer type (e.g. 1: Download, 2: Upload)
        raw_header |= (self.n & bit_mask::get_two_bit_mask() as u8) << N;                  // n is the #bytes in data bytes 4-7 that do not contain data (valid if e & s are set)
        raw_header |= (self.e & bit_mask::get_one_bit_mask() as u8) << E;                  // If set, e indicates an “expedited transfer” (all data is in a single CAN frame)
        raw_header |= (self.s & bit_mask::get_one_bit_mask() as u8) << S;      

    }
}