//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///                                CONTROLLER - IOT - PROTOCOL                                                 ///
///                                DESIGNED BY JACOB MUSSLER                                                   ///
///                                10-NOV-2019 PROOF OF PRINCIPLE FOR RUST LANGUAGE                            ///
///                                BASED OFF OF MODBUS CLIENT IN C# DEVELOPED BY JACOB MUSSLER                 ///
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub mod client;
pub mod server;

use std::sync::Arc;
use std::sync::Mutex;

pub struct ModbusMessage {
    // Message type dependant on the location genenated by the transation identifier
    message_id: [u16; 256],
    message_identifer: u16,
}

impl ModbusMessage
{
    pub fn init() -> ModbusMessage {
        let instance = ModbusMessage {
            message_id: [0; 256],
            message_identifer: 0,
        };

        return instance;
    }

    pub fn get_message_id(&self, offset: usize) -> u16 {
        return self.message_id[offset];
    }

    pub fn set_message_id(&mut self, offset: usize, message_id: u16) {
        self.message_id[offset] = message_id;
    }

    pub fn get_message_identifer(&self) -> u16 {
        return self.message_identifer;
    }

    pub fn set_message_identifer(&mut self, message_identifer: u16) {
        self.message_identifer = message_identifer;
    }
}

#[derive(Clone)]
pub struct ModbusRegisters {
    instance_name: String,                              // Instance Name
    coils: Arc<Mutex<Coils>>,                           // Generation of 4096 coils - defined at boolean values - unsigned (read write)
    discrete_inputs: Arc<Mutex<DiscreteInputs>>,        // Generation of 4096 input contacts - defined at boolean values - unsigned (read only)
    holding_registers: Arc<Mutex<HoldingRegisters>>,    // Generation of 2000 holding register - defined at 16 bit registers - unsigned (read write)
    input_registers: Arc<Mutex<InputRegisters>>,        // Generation of 2000 input register - defined at 16 bit registers - unsigned (read only)
    unit_identifer: Arc<Mutex<UnitIdentifer>>,          // Unit Identifer
}

impl ModbusRegisters {
    // Initalization of the registers Arc Mutex
    fn init(instance_name: &str, unit_identifer: u8) -> ModbusRegisters {
        let instance = ModbusRegisters {
            instance_name: instance_name.to_string(),
            coils: Arc::new(Mutex::new(Coils::init())),
            discrete_inputs: Arc::new(Mutex::new(DiscreteInputs::init())),
            holding_registers: Arc::new(Mutex::new(HoldingRegisters::init())),
            input_registers: Arc::new(Mutex::new(InputRegisters::init())),
            unit_identifer: Arc::new(Mutex::new(UnitIdentifer::init(unit_identifer))),
        };

        return instance;
    }

    pub fn get_instance_name(&self) -> String {
        return self.instance_name.clone();
    }

    pub fn get_coil(&self, offset: usize) -> bool {
        if self.coils.lock().is_ok() {
            return self.coils.lock().unwrap().coils[offset];
        } else {
            println!("{}, {}", self.instance_name, " Failed To Get Coil");
            return false;
        }
    }

    pub fn get_coils(&self, offset: usize, data: &mut [bool]) {      
        if self.coils.lock().is_ok() {
            for index in 0.. data.len() {
                data[index] = self.coils.lock().unwrap().coils[offset + index];
            }
        } else {
            println!("{}, {}", self.instance_name, " Failed To Get Coils");
        }
    }

    pub fn get_coils_len(&self) -> usize {
        if self.coils.lock().is_ok() {
            return self.coils.lock().unwrap().coils.len();
        } else {
            println!("{}, {}", self.instance_name, " Failed To Get Coils Length");
            return 0;
        }
    }

    pub fn set_coil(&self, offset: usize, bit: bool)
    {
        if self.coils.lock().is_ok() {
            self.coils.lock().unwrap().coils[offset] = bit;
        } else {
            println!("{}, {}", self.instance_name, " Failed To Set Coil");
        }
    }

    pub fn set_coils(&self, offset: usize, data: &[bool]) {
        if self.coils.lock().is_ok() {
            for index in 0.. data.len() {
                self.coils.lock().unwrap().coils[offset + index] = data[index];
            }
        } else {
            println!("{}, {}", self.instance_name, " Failed To Set Coils");
        }
    }

    pub fn get_discrete_inputs(&self, offset: usize, data: &mut [bool]) {
        if self.discrete_inputs.lock().is_ok() {
            for index in 0.. data.len() {
                data[index] = self.discrete_inputs.lock().unwrap().discrete_inputs[offset + index];
            }
        } else {
            println!("{}, {}", self.instance_name, " Failed To Get Input Contacts");
        }
    }

    pub fn get_discrete_inputs_len(&self) -> usize {
        if self.discrete_inputs.lock().is_ok() {
            return self.discrete_inputs.lock().unwrap().discrete_inputs.len();
        } else {
            println!("{}, {}", self.instance_name, " Failed To Get Input Contacts Length");
            return 0;
        }
    }

    pub fn set_discrete_inputs(&self, offset: usize, data: &[bool]) {
        if self.discrete_inputs.lock().is_ok() {
            for index in 0..data.len() {
                self.discrete_inputs.lock().unwrap().discrete_inputs[offset + index] = data[index];
            }
        } else {
            println!("{}, {}", self.instance_name, " Failed To Set Input Contacts");
        }
    }

    pub fn get_holding_register(&self, offset: usize) -> u16 {
        if self.holding_registers.lock().is_ok() {
            return self.holding_registers.lock().unwrap().holding_registers[offset];
        } else {
            println!("{}, {}", self.instance_name, " Failed To Get Holding Register");
            return 0;
        }
    }

    pub fn get_holding_registers(&self, offset: usize, data: &mut [u16]) {
        if self.holding_registers.lock().is_ok() {
            for index in 0.. data.len() {
                data[index] = self.holding_registers.lock().unwrap().holding_registers[offset + index];
            }
        } else {
            println!("{}, {}", self.instance_name, " Failed To Get Holding Registers");
        }
    }

    pub fn get_holding_registers_len(&self) -> usize {
        if self.holding_registers.lock().is_ok() {
            return self.holding_registers.lock().unwrap().holding_registers.len();
        } else {
            println!("{}, {}", self.instance_name, " Failed To Get Holding Registers Length");
            return 0;
        }        
    }

    pub fn set_holding_register(&self, offset: usize, register: u16)
    {
        if self.holding_registers.lock().is_ok() {
            self.holding_registers.lock().unwrap().holding_registers[offset] = register;
        } else {
            println!("{}, {}", self.instance_name, " Failed To Set Holding Register");
        }
    }

    pub fn set_holding_registers(&self, offset: usize, data: &[u16])
    {
        if self.holding_registers.lock().is_ok() {
            for index in 0.. data.len() {
                self.holding_registers.lock().unwrap().holding_registers[offset + index] = data[index];
            }
        } else {
            println!("{}, {}", self.instance_name, " Failed To Set Holding Registers");
        }
    }

    pub fn get_input_register(&self, offset: usize) -> u16 {  
        if self.input_registers.lock().is_ok() {
            return self.input_registers.lock().unwrap().input_registers[offset];
        } else {
            println!("{}, {}", self.instance_name, " Failed To Get Input Register");
            return 0;
        }
    }

    pub fn get_input_registers(&self, offset: usize, data: &mut [u16]) {
        if self.input_registers.lock().is_ok() {
            for index in 0.. data.len() {
                data[index] = self.input_registers.lock().unwrap().input_registers[offset + index];
            }
        }
        else {
            println!("{}, {}", self.instance_name, " Failed To Get Input Registers");
        }
    }

    pub fn get_input_registers_len(&self) -> usize {
        if self.input_registers.lock().is_ok() {
            return self.input_registers.lock().unwrap().input_registers.len();
        } else {
            println!("{}, {}", self.instance_name, "Failed To Get Input Registers Length");
            return 0;
        }
    }

    pub fn set_input_registers(&self, offset: usize, data: &[u16]) {
        if self.input_registers.lock().is_ok() {
            for index in 0.. data.len() {
                self.input_registers.lock().unwrap().input_registers[offset + index] = data[index];
            }
        } else {
            println!("{}, {}", self.instance_name, " Failed To Set Input Registers");
        }
    }

    pub fn set_input_register(&self, offset: usize, register: u16)
    {
        if self.input_registers.lock().is_ok() {
            self.input_registers.lock().unwrap().input_registers[offset] = register;
        } else {
            println!("{}, {}", self.instance_name, " Failed To Set Holding Register");
        }
    }

    pub fn get_unit_identifer(&self) -> u8 {
        if self.unit_identifer.lock().is_ok() {
            return self.unit_identifer.lock().unwrap().unit_identifer;
        } else {
            println!("{}, {}", self.instance_name, " Failed To Get Unit Identifer");
            return 0;
        }
    }
}

pub struct Coils {
    coils: [bool; 4096],            // Generation of 4096 coils - defined at boolean values - unsigned (read write)
}

impl Coils {
    fn init() -> Coils {
        let instance = Coils {
            coils: [false; 4096],
        };

    return instance;
    }
}

pub struct DiscreteInputs {
    discrete_inputs: [bool; 4096],   // Generation of 4096 input contacts - defined at boolean values - unsigned (read only)
}

impl DiscreteInputs {
    fn init() -> DiscreteInputs {
        let instance = DiscreteInputs {
            discrete_inputs: [false; 4096],
        };

        return instance;
    }
}

pub struct HoldingRegisters
{
    holding_registers: [u16; 2000], // Generation of 2000 holding registers - defined at 16 bit register - unsigned (read write)
}

impl HoldingRegisters {
    fn init() -> HoldingRegisters {
        let instance = HoldingRegisters {
            holding_registers: [0; 2000],
        };

        return instance;
    }
}

pub struct InputRegisters {
    input_registers: [u16; 2000],   // Generation of 2000 input registers - defined at 16 bit register - unsigned (read only)
}

impl InputRegisters {
    fn init() -> InputRegisters {
        let instance = InputRegisters {
            input_registers: [0; 2000],
        };

        return instance;
    }
}

pub struct UnitIdentifer {
    unit_identifer: u8      // Used for serial chains
}

impl UnitIdentifer {
    fn init(unit_identifer: u8) -> UnitIdentifer {
        let instance = UnitIdentifer {
            unit_identifer: unit_identifer,
        };

        return instance;
    }
}
