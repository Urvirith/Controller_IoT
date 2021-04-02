/// ALL FLOATS IN THIS PID SHOULD BE OF THE SAME SCALING OF 0-100%
/// THIS WILL REDUCE CONFUSION WHEN WORKING WITH THE PID LOOP

#[allow(dead_code)]
struct PID
{
    error: f32,                 // Error is SP - PV
    action: bool,               // false is + (reverse acting), true is - (direct acting)
    pid_type: bool,             // Determines the formula by addition of the KNL for Standard Form (true) or series form no KNL (false)
    extern_reset: f32,          // External reset input which is ethier BKCAL_IN or OUT
    gain: f32,                  // Gain
    rate: f32,                  // Rate
    reset: f32,                 // Reset
    dt: f32,                    // Delta Time is the scan time applied to the PID
    feedforward: f32,           // Feedforward Contribution
    setpoint: f32,              // Setpoint
    process_variable: f32,      // Process variable
    output: f32,                // Output

    //OPERATOR CONTROLS
    mode: bool,                 // Mode - true for auto, false for manual
    operator_setpoint: f32,     // Operator Setpoint

    // Structure
    // 0 (PID action on Error, Beta = 1, Gamma = 1)
    // 1 (PI action on Error, D action on PV, Beta = 1, Gamma = 0)
    // 2 (I action on Error, PD action on PV, Beta = 0, Gamma = 0)
    // 3 (PD action on Error, Beta = 1, Gamma = 1)
    // 4 (P action on Error, D action on PV, Beta = 1, Gamma = 0)
    // Else (set default to PID - 0)
    structure: u32,
    beta: f32,                  // Beta
    gamma: f32,                 // Gamma
}

impl PID
{
    pub fn init(action: bool, pid_type: bool, gain: f32, rate: f32, reset: f32, dt_ms: f32, structure: u32) -> PID {
        let instance = PID {
            error: 0.0,
            action: action,
            pid_type: pid_type,
            extern_reset: 0.0,
            gain: gain,
            rate: rate,
            reset: reset,
            dt: (dt_ms / 1000.0),    //Evaluate as MS
            feedforward: 0.0,
            setpoint: 0.0,
            process_variable: 0.0,
            output: 0.0,
            mode: false,
            operator_setpoint: 0.0,
            structure: structure,
            beta: 0.0,
            gamma: 0.0,
        };
        
        return instance;
    }

    pub fn set_gain(&mut self, gain: f32) {
        self.gain = gain;
    }

    pub fn set_rate(&mut self, rate: f32) {
        self.rate = rate;
    }

    pub fn set_reset(&mut self, reset: f32) {
        self.reset = reset;
    }

    pub fn set_action(&mut self, action: bool) {
        self.action = action;
    }

    pub fn set_pid_type(&mut self, pid_type: bool) {
        self.pid_type = pid_type;
    }

    pub fn pid(&mut self, en: bool, bk_cal_in: f32, sp: f32, pv: f32, op_sp: f32, ff: f32, mode: bool, st: u32) -> f32 {
        if en == true {
            self.pid_bkcal_in(bk_cal_in);
            self.setpoint = sp;
            self.process_variable = pv;
            self.operator_setpoint = op_sp;
            self.feedforward = ff;
            self.mode = mode;
            self.structure = st;

            // Ensure if the PID has been set to automatic mode and evaluate
            // Beta and Gamma based on the structure provided
            if self.mode == true {
                let gain = self.pid_action();
                self.pid_error();
                self.pid_structure();
                self.pid_type(gain);
            } else {
                self.output = self.operator_setpoint;
            }

        } else {
            self.output = 0.0;
        }

        let output = self.output;

        return output;
    }

    fn pid_bkcal_in(&mut self, bk_cal_in: f32) {
        if bk_cal_in > 0.0 { // Setup evaluation variables
            self.extern_reset = bk_cal_in;
        } else {
            self.extern_reset = self.output;
        }
    }

    // Determine the error of the PID
    fn pid_error(&mut self) {
        self.error = self.setpoint - self.process_variable;
    }

    fn pid_structure(&mut self) {
        if self.structure == 0 {
            self.beta = 1.0;
            self.gamma = 1.0;
        } else if self.structure == 1 {
            self.beta = 1.0;
            self.gamma = 0.0;
        } else if self.structure == 2 {
            self.beta = 0.0;
            self.gamma = 0.0;
        } else if self.structure == 3 {
            self.beta = 1.0;
            self.gamma = 1.0;
        } else if self.structure == 4 {
            self.beta = 1.0;
            self.gamma = 0.0;
        } else {
            self.beta = 1.0;
            self.gamma = 1.0;
        }
    }

    fn pid_action(&mut self) -> f32 { // Apply action to the gain
        if self.action == true {
            return self.gain * -1.0;
        } else {
            return self.gain;
        }
    }

    fn pid_type(&mut self, gain: f32) { // Evaluate the request for the equation as series or parallel
        if self.pid_type == true {
            self.output = gain * (((self.beta * self.reset) / (self.reset + 1.0)) + (self.error /  (self.reset + 1.0)) + ((self.gamma * self.reset * self.rate) / ((self.reset + 1.0) * (self.rate + 1.0)))) + ((self.extern_reset - self.feedforward) / (self.reset + 1.0)) + self.feedforward;
        } else {
            self.output = gain * (((self.beta * self.reset) / (self.reset + 1.0)) + (self.error /  (self.reset + 1.0)) + ((self.gamma * self.reset) / ((self.reset + 1.0) * (self.rate + 1.0)))) + ((self.extern_reset - self.feedforward) / (self.reset + 1.0)) + self.feedforward;
        }
    }