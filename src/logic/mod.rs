//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///                                CONTROLLER - IOT - TCP CLIENT - LOGIC CORE                                  ///
///                                DESIGNED BY JACOB MUSSLER                                                   ///
///                                07-JUL-2018 PROOF OF PRINCIPLE FOR RUST LANGUAGE                            ///
///                                BASED OFF OF MODBUS CLIENT IN C# DEVELOPED BY JACOB MUSSLER                 ///
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////

mod thread_handler;
mod routines;
mod library;

//Declare all interfaces at this level then push them to the main routine for distribution to the various areas in the logic for use
pub fn main_call() {
    thread_handler::interface_header();
}

