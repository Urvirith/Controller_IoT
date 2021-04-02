pub struct IdleState {
    robot_idle: u16,
    home_position: u16,
    steam_state: u16,
    robot_state: u16,
    bath_state: u16,
    robot_program: u16,
}

impl IdleState {
    pub fn create() -> IdleState {
        let instance = IdleState {
            robot_idle: 0,
            home_position: 0,
            steam_state: 0,
            robot_state: 0,
            bath_state: 0,
            robot_program: 0,
        };

        return instance;
    }
}

