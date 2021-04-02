//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///                                CONTROLLER - IOT - TCP CLIENT - THREAD HANLDER                              ///
///                                DESIGNED BY JACOB MUSSLER                                                   ///
///                                23-JUL-2018 PROOF OF PRINCIPLE FOR RUST LANGUAGE                            ///
///                                BASED OFF OF MODBUS CLIENT IN C# DEVELOPED BY JACOB MUSSLER                 ///
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::logic::routines;
use crate::logic::library;
use crate::communication::modbus;
use crate::communication::symbol_scanner;
use crate::interface::protocol;
use std::thread;

//Declare all interfaces at this level then push them to the main routine for distribution to the various areas in the logic for use
pub fn interface_header() {
    //let mut e1212 = modbus::client::Client::init("E1212", "192.168.1.150", "502", 0, false);
    let mut ur = modbus::client::Client::init("UR Robot", "192.168.0.20", "502", 1, false);
    //let mut wago = modbus::client::Client::init("E1212", "192.168.1.50", "502", 0, false);
    let mut server = modbus::server::Server::init("Server", "192.168.0.252", "8502", 0);
    let mut scanner = symbol_scanner::BarcodeScanner::create();

    //e1212.read_discrete_inputs(0, 8);
    //e1212.write_coils(0, 8);
    //wago.write_coils(0, 8);
    ur.write_holding_registers(128, 12);
    ur.read_holding_registers(140, 12);

    let ur_registers = ur.get_registers();
    //let e1212_registers = e1212.get_registers();
    //let wago_registers = wago.get_registers();
    let server_registers = server.get_registers();
    let scanner_data = scanner.get_registers();

    thread::spawn(move || {
        logic_thread(ur_registers, /*wago_registers,*/ server_registers, scanner_data);
    });

    thread::spawn(move || {
        loop {
            scanner.read();
        }
    });


    thread::spawn(move || { // Spawn the server thread
        loop {
            server.cycle();
            library::thread_sleep(10000); // Rest in ms between rerunning
        }
    });

    loop {
        //wago.cycle();
        ur.cycle();
        library::thread_sleep(10);
    }
}

// Thread for all logic, if created in a struct will be cleaner
fn logic_thread(client_register: protocol::modbus::ModbusRegisters, /*wago_register: protocol::modbus::ModbusRegisters,*/ server_register: protocol::modbus::ModbusRegisters, mut scanner: symbol_scanner::BarcodeScanner) {
    // MAIN ROUTINE CALL ENABLE THE SYSTEM AND THEN GENERATE A SCAN RATE
    // THIS IS GENERATED FOR THE SYSTEM 
    let mut main_function = routines::main_routine::MainRoutine::create();

    loop {
        main_function.main_call(&client_register, /*&wago_register,*/ &server_register, &mut scanner);
        library::thread_sleep(1000);
    } 
}
