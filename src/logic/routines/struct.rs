pub struct SteamGenerator{
    active: bool,
    enabled: bool,
    running: bool,
    enable: bool,
    call: bool,
}

impl SteamGenerator {
    pub fn create() -> SteamGenerator {
        let instance = SteamGenerator {
            active: false,
            enabled: false,
            running: false,
            enable: false,
            call: false,
        };

        return Instance;
    }

    pub fn get_active(&self) -> bool {
        return self.active;
    }

    pub fn get_enabled(&self) -> bool {
        return self.enabled;
    }

    pub fn get_running(&self) -> bool {
        return self.running;
    }

    pub fn get_enable(&self) -> bool {
        return self.enable;
    }

    pub fn get_call(&self) -> bool {
        return self.call;
    }

    pub fn set_active(&self, data: bool) {
        self.active = data;
    }

    pub fn set_enabled(&self, data: bool) {
        self.enabled = data;
    }

    pub fn set_running(&self, data: bool) {
        self.running = data;
    }

    pub fn set_enable(&self) {
        if self.active == true {
            self.enable = true;
        }
    }

    pub fn reset_enable(&self) {
        self.enable = false;
    }

    pub fn set_call(&self) {
        if (self.active == true && self.enabled == true) {
            self.call = true; 
        } else {
            self.call = false; 
        }
    }

    pub fn reset_call(&self) {
        self.call = false;
    }
}