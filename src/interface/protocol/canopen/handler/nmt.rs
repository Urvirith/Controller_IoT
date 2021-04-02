//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///                                CONTROLLER - IOT - CANOPEN - NMO                                            ///
///                                DESIGNED BY JACOB MUSSLER                                                   ///
///                                17-MAR-2020 PROOF OF PRINCIPLE FOR RUST LANGUAGE                            ///
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::interface::common::bit_mask;

const NMT:      u8 = 0x00;
const HB:       u8 = 0x0E;
const TOGGLE:   i32 = 7;

pub struct CANOpenNMT {
    node:           u8,
    cs:             u8,
    node_id:        u8,
    state:          u8,
    toggle:         u8,
}

impl CANOpenNMT {
    pub fn init(node: u8, cs: u8, node_id: u8, state: u8) -> CANOpenNMT { 
        let instance = CANOpenNMT {
            node:           node,
            cs:             cs,
            node_id:        node_id,
            state:          state,
            toggle:         0
        };

        return instance;
    }

    pub fn module_control(&self, data: &mut [u8; 2]) {      // MASTER ONLY
        data[0] = match self.cs {
            1 => self.cs,       // Start Remote Node
            2 => self.cs,       // Stop Remote Node
            128 => self.cs,     // Enter Pre-operational State
            129 => self.cs,     // Reset Node
            130 => self.cs,     // Reset Communication
            _ => 0
        };

        if self.node_id <= 127 { // Limit the node id to the maximum number of CANOpen, if > then address all nodes on the network
            data[1] = self.node_id;
        } else {
            data[1] = 0;
        }
    }
    
    // SLAVE/ SERVER LOGIC
    pub fn node_guarding(&mut self) -> u8 {                 // Used to report when node guarding request is used
        let mut data: u8 = 0;
        let state = match self.state {
            0 => self.state,    // Initalizing
            1 => self.state,    // Disconnected
            2 => self.state,    // Connecting
            3 => self.state,    // Preparing
            4 => self.state,    // Stopped
            5 => self.state,    // Operational
            127 => self.state,  // Pre-operational
            _ => 0
        };

        data |= self.toggle << TOGGLE;
        data |= state & bit_mask::get_seven_bit_mask() as u8;

        if self.toggle == 0 { // Invert on each node guarding response
            self.toggle = 1;
        } else {
            self.toggle = 0;
        }

        return data;
    }

    pub fn heartbeat(&mut self) -> u8 {                     // Timed heartbeat, release with a 0 upon startup to a master
        let data = match self.state {
            0 => self.state,    // Boot-up 
            4 => self.state,    // Stopped
            5 => self.state,    // Operational
            127 => self.state,  // Pre-operational
            _ => 0
        };

        return data;
    }

    // Obtain the main parameters
    pub fn get_node(&self) -> u8 {
        return self.node;
    }

    pub fn get_cs(&self) -> u8 {
        return self.cs;
    }

    pub fn get_node_id(&self) -> u8 {
        return self.node_id;
    }

    pub fn get_state(&self) -> u8 {
        return self.state;
    }

}