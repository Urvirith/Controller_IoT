//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///                                CONTROLLER - IOT - LOGIC CORE - ADVANCED CONTROL                            ///
///                                DESIGNED BY JACOB MUSSLER                                                   ///
///                                11-SEP-2018 PROOF OF PRINCIPLE FOR RUST LANGUAGE                            ///
///                                IMPLEMENTATION OF STANDARD PLC LIBRARY INTO RUST LANGUAGE                   ///
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////


#[allow(dead_code)]
pub fn scaling (input_value: f32, input_scale_low: f32, input_scale_high: f32, output_scale_low: f32, output_scale_high: f32) -> f32 {
    let input_scale = input_scale_high - input_scale_low;                                                   // Find the input scale
    let output_scale = output_scale_high - output_scale_low;                                                // Find the out scale
    let output_value = (((input_value - input_scale_low) * output_scale) / input_scale) + output_scale_low; // Subtract the input scale low, cross multiply and divide, and then add the output_scale_low

    return output_value;                                                                                    // Return the scaled value
}

#[allow(dead_code)]
// Limit the output based on selection, thus preventing wind up or over travel on PID loop outputs, or limiting analogue outputs
pub fn value_limiting (input_value: f32, limit_high: f32, limit_low: f32) -> f32 {
    let output_value = if input_value >= limit_high {
        limit_high        
    } else if input_value <= limit_low {
        limit_low
    } else {
        input_value
    };

    return output_value;
}

/// Analogue filter for processing over time - this is to reduce choppy signals
/// Will take the signal and divide over the filter number, do this in a timed scan
#[allow(dead_code)]
struct AnalogueFilter {
    signals: Vec<f32>,
}

impl AnalogueFilter {
    #[allow(dead_code)]
    pub fn filter(&mut self, input_value: f32, number_of_signals: usize) -> f32 {
        let output_value;
        let mut raw_total = 0.0;
        self.signals.push(input_value);

        if self.signals.len() >= number_of_signals {
            self.signals.pop();
        } 

        for value in self.signals.iter() {
            raw_total = raw_total + value;
        }

        output_value = raw_total / self.signals.len() as f32;

        return output_value;
    }
}

// Software PID controller
//
// This crate implements a PID controller. It has seen some amount of
// real-world usage driving 100+ kW electrical motors, but has not been tested
// to death. Use with caution (but do use it and file bug reports!).
//
// Any change in behaviour that may make calculations behave differently will
// result in a major version upgrade; your tunings are safe as long as you
// stay on the same major version.
//
// Owes a great debt to:
//
// * https://en.wikipedia.org/wiki/PID_controller
// * http://www.embedded.com/design/prototyping-and-development/4211211/PID-without-a-PhD
// * http://brettbeauregard.com/blog/2011/04/improving-the-beginners-pid-introduction/

// FIXME: it may be worth to explore http://de.mathworks.com/help/simulink/slref/pidcontroller.html
//        for additional features/inspiration
/// A generic controller interface.
///
/// A controller is fed timestamped values and calculates an adjusted value
/// based on previous readings.
///
/// Many controllers possess a set of adjustable parameters as well as a set
/// of input-value dependant state variables.
/// PID controller derivative modes.
///
/// Two different ways of calculating the derivative can be used with the PID
/// controller, allowing to avoid "derivative kick" if needed (see
/// http://brettbeauregard.com/blog/2011/04/improving-the-beginner%E2%80%99s-pid-derivative-kick/
/// for details information on the implementation that inspired this one).
///
/// Choosing `OnMeasurement` will avoid large bumps in the controller output
/// when changing the setpoint using `set_target()`.
#[derive(Debug, Clone, Copy)]
pub enum DerivativeMode {
    OnError,            // Calculate derivative of error (classic PID-Controller)
    OnMeasurement,      // Calculate derivative of actual changes in value.
}

/// PID Controller.
///
/// A PID controller, supporting the `Controller` interface. Any public values
/// are safe to modify while in operation.
///
/// `p_gain`, `i_gain` and `d_gain` are the respective gain values. The
/// controlller internally stores an already adjusted integral, making it safe
/// to alter the `i_gain` - it will *not* result in an immediate large jump in
/// controller output.
///
/// `i_min` and `i_max` are the limits for the internal integral storage.
/// Similarly, `out_min` and `out_max` clip the output value to an acceptable
/// range of values. By default, all limits are set to +/- infinity.
///
/// `d_mode` The `DerivativeMode`, the default is `OnMeasurement`.
pub struct PIDController {
    p_gain: f32,    // Proportional gain
    i_gain: f32,    // Integral gain
    d_gain: f32,    // Differential gain,

    sp: f32,
    delta_t: f32,   // Time in seconds of scan

    // Integral range limits
    i_min: f32,
    i_max: f32,

    // Output range limits
    out_min: f32,
    out_max: f32,

    d_mode: DerivativeMode,
    active: bool,
    action: bool,

    // The PIDs internal state. All other attributes are configuration values
    err_sum: f32,
    prev_input: f32,
    prev_error: f32,
    cv: f32
}

impl PIDController {
    /// Creates a new PID Controller.
    pub fn create(p_gain: f32, i_gain: f32, d_gain: f32, delta_t: f32) -> PIDController {
        PIDController{
            p_gain: p_gain,
            i_gain: i_gain,
            d_gain: d_gain,

            sp: 0.0,
            delta_t: delta_t,

            err_sum: 0.0,
            prev_input: 0.0,
            prev_error: 0.0,
            cv: 0.0,

            i_min: 0.0,
            i_max: 100.0,

            out_min: 0.0,
            out_max: 100.0,

            d_mode: DerivativeMode::OnMeasurement,
            active: false,
            action: false
        }
    }

    /// Convenience function to set `i_min`/`i_max` and `out_min`/`out_max`
    /// to the same values simultaneously.
    pub fn set_limits(&mut self, min: f32, max: f32) {
        self.i_min = min;
        self.i_max = max;
        self.out_min = min;
        self.out_max = max;
    }

    pub fn set_sp(&mut self, sp: f32) {
        self.sp = sp;
    }

    pub fn get_sp(&self) -> f32 {
        return self.sp;
    }

    pub fn set_auto(&mut self) {
        self.active = true;
    }

    pub fn set_manual(&mut self) {
        self.active = false;
    }

    pub fn set_normal(&mut self) {
        self.action = false;
    }

    pub fn set_inverse(&mut self) {
        self.action = true;
    }

    pub fn update(&mut self, input: f32) -> f32 {
        if self.active == false {
            self.err_sum = input;                                           // PREVENT WIND UP
            return self.sp;
        } else {
            let error = if self.action == false {
                self.sp - input
            } else {
                input - self.sp
            };

            let p_term = self.p_gain * error;                               // PROPORTIONAL
    
            self.err_sum = value_limiting(self.err_sum + self.i_gain * error * self.delta_t, self.out_max, self.out_min);
            let i_term = self.err_sum;                                      // INTEGRAL
    
            // DIFFERENTIAL
            let d_term = if self.prev_input == 0.0 || self.prev_error == 0.0 {
                0.0     // we have no previous values, so skip the derivative calculation
            } else {
                match self.d_mode {
                    DerivativeMode::OnMeasurement => {
                        // we use -delta_v instead of delta_error to reduce "derivative kick",
                        // see http://brettbeauregard.com/blog/2011/04/improving-the-beginner%E2%80%99s-pid-derivative-kick/
                        self.d_gain * (self.prev_input - input) / self.delta_t
                    },
                    DerivativeMode::OnError => {
                        self.d_gain * (error - self.prev_error) / self.delta_t
                    }
                }
            };
    
            // store previous values
            self.prev_input = input;
            self.prev_error = error;
    
            println!("{}, {}, {}", p_term, d_term, i_term);
            self.cv = value_limiting(p_term + d_term + i_term, self.out_max, self.out_min);
            return self.cv;
        }
    }

    pub fn reset(&mut self) {
        self.prev_input = 0.0;
        self.prev_error = 0.0;

        // FIXME: http://brettbeauregard.com/blog/2011/04/improving-the-beginner
        //               %E2%80%99s-pid-initialization/
        //        suggests that this should not be there. however, it may miss
        //        the fact that input and output can be two completely
        //        different domains
        self.err_sum = 0.0;
    }
}