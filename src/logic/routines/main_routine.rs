//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///                                CONTROLLER - IOT - TCP CLIENT - Logic Core                                  ///
///                                DESIGNED BY JACOB MUSSLER                                                   ///
///                                07-JUN-2018 PROOF OF PRINCIPLE FOR RUST LANGUAGE                            ///
///                                BASED OFF OF MODBUS CLIENT IN C# DEVELOPED BY JACOB MUSSLER                 ///
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::interface::protocol::modbus;
use crate::communication::symbol_scanner;
use crate::logic::routines;

pub struct MainRoutine {
    alarm: u16,
    state: u16,
    sequence: u16,
    index: u16,
    idle_param: routines::idle::IdleState,
}

impl MainRoutine{
    pub fn create() -> MainRoutine {
        let instance = MainRoutine {
            alarm: 0,
            state: 0,
            sequence: 0,
            index: 0,
            idle_param: routines::idle::IdleState::create(),
        };

        return instance;
    }

    /*pub fn main_call(&self, client_registers: &modbus::ModbusRegisters, /*wago_registers: &modbus::ModbusRegisters,*/ server_registers: &modbus::ModbusRegisters) {   
        let mut input_boolean: [bool; 8] = [false; 8];
        let mut input_registers: [u16; 12]= [0; 12];
        let mut output_boolean: [bool; 8] = [false; 8];
        let mut output_registers: [u16; 12]= [0; 12];
        
        server_registers.get_holding_registers(0, &mut input_registers);
        client_registers.get_discrete_inputs(0, &mut input_boolean);
        //println!("{:?}", client_discretes);
    
        output_boolean[0] = input_boolean[0];
        output_boolean[1] = input_boolean[1];
    
        if input_registers[9] == 1 {
            output_boolean[2] = true;
        } else {
            output_boolean[2] = false;
        }
    
        if input_registers[10] == 1 {
            output_boolean[3] = true;
        } else {
            output_boolean[3] = false;
        }
    
        if input_boolean[0] == true {
            output_registers[0] = 1;
            output_boolean[2] = true
        } else {
            output_registers[0]= 0;
            output_boolean[2] = false;
        }
    
        if input_boolean[1] == true {
            output_registers[1] = 1;
        } else {
            output_registers[1] = 0;
        }
    
        if input_registers[11] == 1 {
            output_registers[2] = 1;
        } else {
            output_registers[2] = 0;
        }    
    
        client_registers.set_coils(0, &output_boolean);
        //wago_registers.set_coils(0, output_boolean);
        server_registers.set_holding_registers(12, &output_registers);
        
    }*/

    pub fn main_call(&mut self, ur_registers: &modbus::ModbusRegisters, server_registers: &modbus::ModbusRegisters, scanner: &mut symbol_scanner::BarcodeScanner) {
        let mut load_registers: [u16; 5] = [0; 5]; 
        let mut read_registers: [u16; 5] = [0; 5]; 

        load_registers[0] = self.index;
        load_registers[1] = scanner.get_batch_size();
        load_registers[2] = scanner.get_size();
        load_registers[3] = scanner.get_side();

        //println!("{}, {}, {}", scanner.get_batch_size(), scanner.get_size(), scanner.get_side());
        ur_registers.get_holding_registers(140, &mut read_registers);
        //println!("{:?}", read_registers);
        if read_registers[1] == load_registers[1] && read_registers[2] == load_registers[2] && read_registers[3] == load_registers[3] {
            if read_registers[1] > 0 && read_registers[2] > 0 && read_registers[3] > 0 { 
                load_registers[4] = 1; 
            } else {
                load_registers[4] = 0;
            }
        } else {
            load_registers[4] = 0;
        }

        if read_registers[4] == 1 {
            scanner.clear();
        }



        ur_registers.set_holding_registers(128, &load_registers);
    }
}
