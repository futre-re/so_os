use alloc::string::String;

//实现了进程的id，状态
struct PCB {
    process_id: u32,       //进程ID
    process_state: String, //进程的状态
    program_counter: u32,  //程序计数器的值
    registers: [u32; 8],   //存储进程的寄存器的值
                           // 其他进程相关的信息
}

impl PCB {
    fn new(process_id: u32) -> PCB {
        PCB {
            process_id,
            process_state: String::from("READY"),
            program_counter: 0,
            registers: [0; 8],
        }
    }

    fn set_state(&mut self, state: &str) {
        self.process_state = String::from(state);
    }

    fn get_state(&self) -> &str {
        &self.process_state
    }

    // 其他操作和方法
}
