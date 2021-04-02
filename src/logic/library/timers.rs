//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///                                CONTROLLER - IOT - LOGIC CORE - TIMERS                                      ///
///                                DESIGNED BY JACOB MUSSLER                                                   ///
///                                11-SEP-2018 PROOF OF PRINCIPLE FOR RUST LANGUAGE                            ///
///                                IMPLEMENTATION OF STANDARD PLC LIBRARY INTO RUST LANGUAGE                   ///
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::time;

#[allow(dead_code)]
struct Timer {
    enable_timer: bool,
    start_time: time::Instant,
    en: bool,
    dn: bool,
    pre: u32,
    acc: u32,
}

impl Timer {
    #[allow(dead_code)]
    pub fn init() -> Timer {
        let instance = Timer {
            enable_timer: false,
            start_time: time::Instant::now(),
            en: false,
            dn: false,
            pre: 0,
            acc: 0,
        };

        return instance;
    }

    #[allow(dead_code)]
    pub fn set_pre(&mut self, pre: u32) {
        self.pre = pre;
    }

    #[allow(dead_code)]
    pub fn set_acc(&mut self, acc: u32) {
        self.acc = acc;
    }

    #[allow(dead_code)]
    pub fn get_acc(&mut self) -> u32 {
        return self.acc;
    }

    /// Timer On Delay - Timer that will run for specified time only if the Enable is true, upon completion of the timer the RLO (DN)
    /// will become true
    #[allow(dead_code)]
    pub fn timer_on_delay(&mut self, en: bool) { // Start the timer or disable the timer based on "en" being true or false
        if en == true {
            if self.enable_timer == false {
                self.start_time = time::Instant::now();
                self.enable_timer = true;
            }
        } else {
            self.enable_timer = false;
        }

        if self.enable_timer == true { // Evaluate the time and output true upon completion
            let elapsed_time = self.start_time.elapsed();
            let elapsed_time_ms = elapsed_time.subsec_millis();

            self.acc = elapsed_time_ms;
            self.en = true;

            if self.pre <= self.acc {
                self.dn = true;
            } else {
                self.dn = false;
            }
        } else {
            self.acc = 0;
            self.en = false;
            self.dn = false;
        }
    }

    /// Timer Off Delay - Timer that will run for specified time only if the Enable is true, during the running of the timer the RLO (DN)
    /// will become true and upon completion of the timer the DN will become false
    #[allow(dead_code)]
    pub fn timer_off_delay(&mut self, en: bool) {
        if en == true { // Start the timer or disable the timer based on "en" being true or false
            if self.enable_timer == false {
                self.start_time = time::Instant::now();
                self.enable_timer = true;
            }
        } else {
            self.enable_timer = false;
        }

        if self.enable_timer == true { // Evaluate the time and output true upon completion
            let elapsed_time = self.start_time.elapsed();
            let elapsed_time_ms = elapsed_time.subsec_millis();

            self.acc = elapsed_time_ms;
            self.en = true;

            if self.pre >= self.acc {
                self.dn = false;
            } else {
                self.dn = true;
            }
        } else {
            self.acc = 0;
            self.en = false;
            self.dn = false;
        }
    }

    /// Timed Pulse - Timer that will run for specified time upon the positive detection of the EN bit, during the running of the timer the RLO (DN)
    /// will become true and upon completion of the timer the DN will become false
    #[allow(dead_code)]
    pub fn timed_pulse(&mut self, en: bool) {
        if en == true { // Start the timer or disable the timer based on "en" being true
            if self.enable_timer == false {
                self.start_time = time::Instant::now();
                self.enable_timer = true;
            }
        }

        if self.enable_timer == true { // Evaluate the time and output true upon completion
            let elapsed_time = self.start_time.elapsed();
            let elapsed_time_ms = elapsed_time.subsec_millis();

            self.acc = elapsed_time_ms;
            self.en = true;

            if self.pre <= self.acc {
                self.dn = false;

                if en == false {
                    self.enable_timer = false;
                }
            } else {
                self.dn = true;
            }
        } else {
            self.acc = 0;
            self.en = false;
            self.dn = false;
        }
    }                
}


#[allow(dead_code)]
struct Counter {
    en: bool,
    dn: bool,
    rst: bool,
    pre: i32,
    acc: i32,
}

impl Counter {
    #[allow(dead_code)]
    pub fn init() -> Counter {
        let instance = Counter {
            en: false,
            dn: false,
            rst: false,
            pre: 0,
            acc: 0,
        };

        return instance;
    }

    #[allow(dead_code)]
    pub fn set_pre(&mut self, pre: i32) {
        self.pre = pre;
    }

    #[allow(dead_code)]
    pub fn set_acc(&mut self, acc: i32) {
        self.acc = acc;
    }

    #[allow(dead_code)]
    pub fn get_acc(&mut self) -> i32 {
        return self.acc;
    }

    /// Count Up Counter - Counter will add one to the Accumulated value once when Enable is true 
    #[allow(dead_code)]
    pub fn count_up(&mut self, en: bool, rst: bool) -> bool { // count based on "en" being true
        if en == true {
            if self.en == false {
                self.acc = self.acc + 1;
            }

            self.en = true;   
        } else {
            self.en = false;
        }

        if self.acc >= self.pre { // Check to see if the preset is reached and set DN to true
            self.dn = true;
        } else {
            self.dn = false;
        }

        if rst == true { // Check to see if reset is true and set the self.rst to true to ensure reset
            self.rst = true;
        }

        if self.rst == true { // Set accumlated value to 0 and reset the internal reset bit
            self.acc = 0;
            self.rst = false;
        }

        return self.dn;
    }

    /// Count Up Counter - Counter will add one to the Accumulated value once when Enable is true 
    #[allow(dead_code)]
    pub fn count_down(&mut self, en: bool, rst: bool) -> bool {
        if en == true {         // count based on "en" being true
            if self.en == false {
                self.acc = self.acc - 1;
            }

            self.en = true;   
        } else {
            self.en = false;
        }
 
        if self.acc <= 0 {      // Check to see if the preset is reached and set DN to true
            self.dn = true;
        } else {
            self.dn = false;
        }
        
        if rst == true {        // Check to see if reset is true and set the self.rst to true to ensure reset
            self.rst = true;
        }

        if self.rst == true {   // Set accumlated value to 0 and reset the internal reset bit
            self.acc = self.pre;
            self.rst = false;
        }

        return self.dn;
    }
}
