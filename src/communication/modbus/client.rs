//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///                                CONTROLLER - IOT - CLIENT MODBUS OS MODBUS LAYER                            ///
///                                DESIGNED BY JACOB MUSSLER                                                   ///
///                                22-DEC-2019 PROOF OF PRINCIPLE FOR RUST LANGUAGE                            ///
///                                BASED OFF OF MODBUS CLIENT IN C# DEVELOPED BY JACOB MUSSLER                 ///
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// Layer created for the ability to use the system for operating systems easily and effectively, 
// Interface is designed for portability to embedded systems using Modbus / Industrial Protocols
use crate::interface::protocol::modbus;
use crate::interface::ethernet;

pub struct Client {
    command:        Vec<modbus::client::ModbusCommand>,
    client:         modbus::client::ModbusClient,
    tcp:            ethernet::tcp::Tcp,
    index:          usize,
    close:          bool,
}

impl Client {
    pub fn init(instance_name: &str, ip_address: &str, port: &str, unit_identifer: u8, close: bool) -> Client {
        let instance = Client {
            command:    Vec::new(),
            client:     modbus::client::ModbusClient::init(instance_name.to_string(), unit_identifer, false),
            tcp:        ethernet::tcp::Tcp::init(instance_name.to_string(), ip_address.to_string(), port.to_string()),
            index:      0,
            close:      close
        };

        return instance;
    }
    
    // Always call before moving to a new thread
    pub fn get_registers(&self) -> modbus::ModbusRegisters  {
        return self.client.get_registers();
    }

    pub fn cycle(&mut self) -> bool { // Should be self healing, returns true if error
        let mut input_data:     [u8; 1048] = [0; 1048];
        let mut output_data:    [u8; 1048] = [0; 1048];
        let mut error = false;
        
        if self.index >= self.command.len() {
            self.index = 0;
        }

        let len = self.client.request(&mut output_data, &self.command[self.index]);
        let result = self.tcp.write(&output_data[0..len]);

        let len = self.tcp.read(&mut input_data);

        if result == 0 {
            error = true;
        }

        if len > 0 {
            self.client.response(&input_data[0.. len]); 
        } else {
            error = true;
        }

        if error == true || self.close == true {
            self.tcp.close();
            self.tcp.connect();
        }

        self.index += 1;

        return error;
    }

    #[allow(dead_code)]
    pub fn read_coils(&mut self, read_offset: u16, read_quantity : u16) {
        self.command.push(modbus::client::ModbusCommand::init(1, read_offset, read_quantity, 0, 0));
    }

    #[allow(dead_code)]
    pub fn read_discrete_inputs(&mut self, read_offset: u16, read_quantity : u16) {
        self.command.push(modbus::client::ModbusCommand::init(2, read_offset, read_quantity, 0, 0));
    }

    #[allow(dead_code)]
    pub fn read_holding_registers(&mut self, read_offset: u16, read_quantity : u16) {
        self.command.push(modbus::client::ModbusCommand::init(3, read_offset, read_quantity, 0, 0));
    }

    #[allow(dead_code)]
    pub fn read_input_registers(&mut self, read_offset: u16, read_quantity : u16) {
        self.command.push(modbus::client::ModbusCommand::init(4, read_offset, read_quantity, 0, 0));
    }

    #[allow(dead_code)]
    pub fn write_coil(&mut self, write_offset: u16) {
        self.command.push(modbus::client::ModbusCommand::init(5, 0, 0, write_offset, 0));
    }

    #[allow(dead_code)]
    pub fn write_holding_register(&mut self, write_offset: u16) {
        self.command.push(modbus::client::ModbusCommand::init(6, 0, 0, write_offset, 0));
    }

    #[allow(dead_code)]
    pub fn write_coils(&mut self, write_offset: u16, write_quantity: u16) {
        self.command.push(modbus::client::ModbusCommand::init(15, 0, 0, write_offset, write_quantity));
    }

    #[allow(dead_code)]
    pub fn write_holding_registers(&mut self, write_offset: u16, write_quantity: u16) {
        self.command.push(modbus::client::ModbusCommand::init(16, 0, 0, write_offset, write_quantity));
    }

    #[allow(dead_code)]
    pub fn read_write_holding_registers(&mut self, read_offset: u16, read_quantity : u16, write_offset: u16, write_quantity: u16) {
        self.command.push(modbus::client::ModbusCommand::init(23, read_offset, read_quantity, write_offset, write_quantity));
    }
}