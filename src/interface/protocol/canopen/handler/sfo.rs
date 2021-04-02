//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///                                CONTROLLER - IOT - CANOPEN - SFO                                            ///
///                                DESIGNED BY JACOB MUSSLER                                                   ///
///                                17-MAR-2020 PROOF OF PRINCIPLE FOR RUST LANGUAGE                            ///
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::interface::common::common_library;

const SYNC: u8 = 0x01;
const EMCY: u8 = 0x01;
const TIME: u8 = 0x02;

pub struct CANOpenSFO {
    node:           u8,
    error_code:     u16,
    error_register: u8
}

impl CANOpenSFO {
    pub fn init(node: u8, error_code: u16, error_register: u8) -> CANOpenSFO {
        let instance = CANOpenSFO {
            node:           node,
            error_code:     error_code,
            error_register: error_register
        };

        return instance;
    }

    pub fn emergency(&self) -> [u8; 3] {
        let mut data: [u8; 3] = [0; 3];
        let library = common_library::CommonLibrary::init();                    // Create a new common library instance
        let error_code = library.u16_to_u8_array(self.error_code, true);

        data[0] = error_code[0];
        data[1] = error_code[1];
        data[2] = self.error_register;

        return data;
    }
}