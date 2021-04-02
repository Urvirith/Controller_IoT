//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///                                CONTROLLER - IOT                                                            ///
///                                DESIGNED BY JACOB MUSSLER                                                   ///
///                                07-JUN-2018 PROOF OF PRINCIPLE FOR RUST LANGUAGE                            ///
///                                BASED OFF OF MODBUS CLIENT IN C# DEVELOPED BY JACOB MUSSLER                 ///
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// Declaration of required dependancies
mod interface;
mod communication;
mod logic;

// Main function, use later for initialization of clients then calling of the loops
// Recieve back an array of information to allowed for called data and useful information
fn main() {
    logic::main_call();
}


