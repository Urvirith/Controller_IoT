//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///                                CONTROLLER - IOT - LOGIC CORE - DISCRETE CONTROL                            ///
///                                DESIGNED BY JACOB MUSSLER                                                   ///
///                                11-SEP-2018 PROOF OF PRINCIPLE FOR RUST LANGUAGE                            ///
///                                IMPLEMENTATION OF STANDARD PLC LIBRARY INTO RUST LANGUAGE                   ///
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[allow(dead_code)]
struct ONS {
    bool_lock: bool,
}

impl ONS {
    #[allow(dead_code)]
    pub fn init() -> ONS {
        let instance = ONS {
            bool_lock: false,
        };

        return instance;
    }

    #[allow(dead_code)]
    pub fn one_shot_rising(&mut self, input: bool) -> bool {
        let output: bool;

        if self.bool_lock == false {
            output = input;   
        } else {
            output = false;
        }

        if input == true {
            self.bool_lock = true;
        } else {
            self.bool_lock = false;
        }

        return output;
    }

    #[allow(dead_code)]
    pub fn one_shot_falling(&mut self, input: bool) -> bool {
        let output: bool;

        if (input == false) && (self.bool_lock == true) {
            output = true
        } else {
            output = false
        }

        if input == true {
            self.bool_lock = true;
        } else {
            self.bool_lock = false;
        }

        return output;
    }
}

