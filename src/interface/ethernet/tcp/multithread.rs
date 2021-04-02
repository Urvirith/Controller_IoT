//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///                                CONTROLLER - IOT - TCP SERVER                                               ///
///                                DESIGNED BY JACOB MUSSLER                                                   ///
///                                08-DEC-2019 PROOF OF PRINCIPLE FOR RUST LANGUAGE                            ///
///                                BASED OFF OF MODBUS CLIENT IN C# DEVELOPED BY JACOB MUSSLER                 ///
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////

//DEVELOP A COMMON INTERFACE FOR THIS SERVER

use crate::interface::protocol::modbus::server::Connector;
use crate::interface::ethernet;
use std::net::TcpListener;
use std::net::TcpStream;
use std::net::Shutdown;
use std::io::Write;
use std::io::Read;
use std::thread;
use std::time;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Clone)]
pub struct Server{
    ethernet:       ethernet::Ethernet,
    worker:         Arc<Mutex<usize>>,
    connect:        Arc<Mutex<bool>>,
    rpi:            time::Duration
}

impl Server{
    pub fn init(instance_name: String, ip: String, port: String, rpi: u64) -> Server {
        let ethernet = ethernet::Ethernet::init(instance_name, ip, port);
        let instance = Server {
            ethernet:   ethernet.clone(),
            worker:     Arc::new(Mutex::new(0)),
            connect:    Arc::new(Mutex::new(false)), 
            rpi:        time::Duration::from_millis(rpi)      
        };

        return instance;
    }

    pub fn create(&self, connector: Arc<Mutex<dyn Connector>>) { // Set up the connection parameters to allow for connection to the host machine
        let listener = TcpListener::bind(self.ethernet.get_socket());
        let mut end_connection = false;

        println!("{}", "Server Created".to_string());

        match listener {
            Ok(server) => {
                while end_connection == false {
                    for connection in server.incoming() {
                        let workers;
        
                        if self.worker.lock().is_ok() {
                            workers = *self.worker.lock().unwrap();
                        } else {
                            workers = 10;
                            println!("{}, : ,{}", self.ethernet.get_instance_name(), "Unable To Reach The Thread Worker Arc Mutex");
                        }
        
                        if workers < 3 {        
                            match connection {
                                Ok(stream) => {
                                    let worker = self.worker.clone();
                                    let clone = self.clone();
                                    
                                    thread::spawn(move|| {clone.client(stream, Box::new(connector.clone()))});
                                }
                                Err(_error) => {
                                    break
                                }
                            }
                        }

                        if self.connect.lock().is_ok() {
                            end_connection = *self.connect.lock().unwrap();
                        } else {
                            end_connection = true;
                            println!("{}, : ,{}", self.ethernet.get_instance_name(), "Mutex Poisoned Terminating Server");
                        }
                                           
                        thread::sleep(self.rpi);
                    }
                }
            } Err(err) => {
                println!("{}, : ,{}", self.ethernet.get_instance_name(), err);
            }
        }
    }

    fn client(&self, mut stream: TcpStream, connector: Box<Arc<Mutex<dyn Connector>>>) {
        let mut end_connection = false;
        let mut input_data: [u8; 1024] = [0; 1024];

        if self.worker.lock().is_ok() {
            let mut worker_count = *self.worker.lock().unwrap();

            worker_count = worker_count + 1;
            *self.worker.lock().unwrap() = worker_count;
        } else {
            println!("{}, {}", self.ethernet.instance_name, "Unable To Reach The Thread Worker Client Connection");
            return;
        }

        while end_connection == false {
            if let Ok(size) = stream.read(&mut input_data) {
                if size > 0 {
                    let mut output_data: [u8; 1024] = [0; 1024];

                    let len = self.connector(&input_data, &mut output_data);
                    if len > 0 {
                        if let Err(_) = stream.write(&output_data) {
                            break;
                        }
                    }
                } else {
                    break;
                }
            } else {
                break;
            }

            if self.connect.lock().is_ok() {
                end_connection = *self.connect.lock().unwrap();
            } else if self.connect.is_poisoned() {
                end_connection = true;
            }

            thread::sleep(self.rpi);
        }

        stream.shutdown(Shutdown::Both).expect("Server Failed To Shutdown Properly");

        if self.worker.lock().is_ok() {
            let mut worker_count = *self.worker.lock().unwrap();

            worker_count = worker_count - 1;
            *self.worker.lock().unwrap() = worker_count;
        } else {
            println!("{}, {}", self.ethernet.instance_name, "Unable To Reach The Thread Worker Client Connection");
        }

        println!("{}, {}", stream.peer_addr().unwrap().ip(), "disconnected");
    }

    pub fn connector(&self, data_req: &[u8], data_res: &mut [u8]) -> usize {
        println!("I am a dummy connector for trait implementation");
        return 0;
    }

    pub fn close(self) {

    }
}

