//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///                                CONTROLLER - IOT - LOGIC CORE - LIBRARY                                     ///
///                                DESIGNED BY JACOB MUSSLER                                                   ///
///                                04-SEP-2018 PROOF OF PRINCIPLE FOR RUST LANGUAGE                            ///
///                                IMPLEMENTATION OF STANDARD PLC LIBRARY INTO RUST LANGUAGE                   ///
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub mod advanced_control;
pub mod discrete_control;
pub mod timers;
use std::thread;
use std::time;

// Thread Sleep function
pub fn thread_sleep(delay_us: u64) {
    let delay_time = time::Duration::from_millis(delay_us);

    thread::sleep(delay_time)
}