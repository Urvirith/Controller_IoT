pub mod tcp;
pub mod udp;


#[derive(Clone)]
pub struct Ethernet {
    instance_name:  String,
    socket:         String,
}

impl Ethernet {
    pub fn init(instance_name: String, ip: String, port: String) -> Ethernet {
        let instance = Ethernet {
            instance_name: instance_name,
            socket: format!("{}{}{}", ip, ":", port),
        };

        return instance;
    }

    pub fn get_instance_name(&self) -> String {
        return self.instance_name.clone();
    }

    pub fn get_socket(&self) -> String {
        return self.socket.clone();
    }
}