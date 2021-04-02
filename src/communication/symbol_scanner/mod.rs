use std::io;
use std::sync::Arc;
use std::sync::Mutex;

const TRACE_CODE_SIZE: usize = 5;
const CATA_NUM_SIZE: usize = 8;
const SHOP_ORDER_SIZE: usize = 6;

#[derive(Clone)]
pub struct BarcodeScanner {
    batch_size: Arc<Mutex<BatchSize>>,
    trace_code: Arc<Mutex<TraceCode>>,
    cata_num: Arc<Mutex<CataNum>>,
    so_num: Arc<Mutex<SoNum>>,
    size: Arc<Mutex<Size>>,
    side: Arc<Mutex<Side>>,
}

impl BarcodeScanner {
    pub fn create() -> BarcodeScanner {
        let instance = BarcodeScanner {
            batch_size: Arc::new(Mutex::new(BatchSize::init())),
            trace_code: Arc::new(Mutex::new(TraceCode::init())),
            cata_num: Arc::new(Mutex::new(CataNum::init())),
            so_num: Arc::new(Mutex::new(SoNum::init())),
            size: Arc::new(Mutex::new(Size::init())),
            side: Arc::new(Mutex::new(Side::init())),
        };
        
        return instance;
    }

    pub fn read(&mut self) {
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(len) => {
                if (len >= 2 + 4) && (&input[0.. 2] == "$4") {                              //Batch Size Number
                    match input[2.. (len - 2)].parse::<u16>() {
                        Ok(size) => {
                            if self.batch_size.lock().is_ok(){
                                self.batch_size.lock().unwrap().batch_size = size;
                            } else {
                                println!("Batch Size Lock unaccessable");
                            }

                        } Err (e) => {
                            println!("Convert to number failure: {}", e);
                        }
                    }
                } else if (len >= CATA_NUM_SIZE + 4) && (&input[0.. 2] == "$5") {     //Catalog Number
                    let data = input[2.. (len - 2)].as_bytes();
                    let part_type = &input[2.. (len - 2)];
                    if self.cata_num.lock().is_ok() { 
                        for index in 0.. CATA_NUM_SIZE {
                            self.cata_num.lock().unwrap().cata_num[index] = data[index];
                        }
                    } else {
                        println!("Catalogue Number Lock unaccessable");
                    }

                    match part_type[part_type.len() - 3.. part_type.len()].parse::<u16>() {
                        Ok(value) => {
                            if self.side.lock().is_ok(){ 
                                if (value & 1) == 0 {
                                    self.side.lock().unwrap().side = 2;
                                } else {
                                    self.side.lock().unwrap().side = 1;
                                }
                            } else {
                                println!("Side Lock unaccessable");
                            }
                            
                            if value >= 100 {   // 
                                if self.size.lock().is_ok(){
                                    self.size.lock().unwrap().size = value / 100;
                                } else {
                                    println!("Size Lock unaccessable");
                                }
                            } else {
                                println!("Converted number to small: {}", value);
                            }
                        } Err(e) => {
                            println!("Convert to number failure: {}", e);
                        }
                    }
                } else if (len >= SHOP_ORDER_SIZE + 4) && (&input[0.. 2] == "$6") {       // Shop Order Number
                    let data = input[2.. (len - 2)].as_bytes();
                    if self.so_num.lock().is_ok() { 
                        for index in 0.. SHOP_ORDER_SIZE {
                            self.so_num.lock().unwrap().so_num[index] = data[index];
                        }
                    } else {
                        println!("Shop Order Lock unaccessable");
                    }
                } else if (len >= TRACE_CODE_SIZE + 4) && (&input[0.. 2] == "$7") {   // Trace Code Number
                    let data = input[2.. (len - 2)].as_bytes();
                    if self.trace_code.lock().is_ok() { 
                        for index in 0.. TRACE_CODE_SIZE {
                            self.trace_code.lock().unwrap().trace_code[index] = data[index];
                        }
                    } else {
                        println!("Trace Code Lock unaccessable");
                    }
                } else {
                    println!("Incorrect Format: {}", input);
                }
            } Err(e) => {
                println!("{}", e);
            }
        }
    }

    pub fn print_data(&self) {
        if self.batch_size.lock().is_ok(){
            println!("Catalog Number: {:?}, Shop Order: {:?}, Trace Code: {:?}, , Batch Size: {:?}, Side: {:?}, Size: {:?}", String::from_utf8_lossy(&self.cata_num.lock().unwrap().cata_num), String::from_utf8_lossy(&self.so_num.lock().unwrap().so_num), String::from_utf8_lossy(&self.trace_code.lock().unwrap().trace_code), self.batch_size.lock().unwrap().batch_size, self.side.lock().unwrap().side, self.size.lock().unwrap().size);
        } else {
            println!("Write Lock Failed");
        }
    }

    pub fn get_registers(&self) -> BarcodeScanner {
        return self.clone();
    } 

    pub fn get_batch_size(&self) -> u16 {
        if self.batch_size.lock().is_ok(){
           return self.batch_size.lock().unwrap().batch_size;
        } else {
            println!("Get Batch Size Failed");
            return 0;
        }
    }


    pub fn get_size(&self) -> u16 {
        if self.size.lock().is_ok(){
           return self.size.lock().unwrap().size;
        } else {
            println!("Get Size Failed");
            return 0;
        }
    }


    pub fn get_side(&self) -> u16 {
        if self.side.lock().is_ok(){
           return self.side.lock().unwrap().side;
        } else {
            println!("Get Side Failed");
            return 0;
        }
    }

    pub fn clear(&mut self) {
        if self.batch_size.lock().is_ok(){
            self.batch_size.lock().unwrap().batch_size = 0;
        } else {
            println!("Batch Size Clear unaccessable");
        }
        if self.trace_code.lock().is_ok(){
            self.trace_code.lock().unwrap().trace_code = [0; TRACE_CODE_SIZE];
        } else {
            println!("Trace Code Clear unaccessable");
        }
        if self.cata_num.lock().is_ok(){
            self.cata_num.lock().unwrap().cata_num = [0; CATA_NUM_SIZE];
        } else {
            println!("Catalogue Number Clear unaccessable");
        }
        if self.so_num.lock().is_ok(){
            self.so_num.lock().unwrap().so_num = [0; SHOP_ORDER_SIZE];
        } else {
            println!("Shop Order Clear unaccessable");
        }
        if self.size.lock().is_ok(){
            self.size.lock().unwrap().size = 0;
        } else {
            println!("Size Clear unaccessable");
        }
        if self.side.lock().is_ok(){
            self.side.lock().unwrap().side = 0;
        } else {
            println!("Side Clear unaccessable");
        }
    }
}

pub struct BatchSize {
    batch_size: u16,   
}

impl BatchSize {
    fn init() -> BatchSize {
        let instance = BatchSize {
            batch_size: 0,
        };

        return instance;
    }
}

pub struct TraceCode {
    trace_code: [u8; TRACE_CODE_SIZE],   
}

impl TraceCode {
    fn init() -> TraceCode {
        let instance = TraceCode {
            trace_code: [0; TRACE_CODE_SIZE],
        };

        return instance;
    }
}

pub struct CataNum {
    cata_num: [u8; CATA_NUM_SIZE],   
}

impl CataNum {
    fn init() -> CataNum {
        let instance = CataNum {
            cata_num: [0; CATA_NUM_SIZE],
        };

        return instance;
    }
}

pub struct SoNum {
    so_num: [u8; SHOP_ORDER_SIZE],   
}

impl SoNum {
    fn init() -> SoNum {
        let instance = SoNum {
            so_num: [0; SHOP_ORDER_SIZE],
        };

        return instance;
    }
}

pub struct Size {
    size: u16,   
}

impl Size {
    fn init() -> Size {
        let instance = Size {
            size: 0,
        };

        return instance;
    }
}

pub struct Side {
    side: u16,   
}

impl Side {
    fn init() -> Side {
        let instance = Side {
            side: 0,
        };

        return instance;
    }
}