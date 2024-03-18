use crate::registers;
use registers::general_register;

// enum ProcessState {
//     New,
//     Ready,
//     Running,
//     Waiting,
//     Terminated,
// }

// struct ProcessControlBlock {
//     process_id: u32,
//     state: ProcessState,
//     program_counter: usize,
//     // 其他寄存器状态
//     // 内存管理信息
//     // I/O状态信息
// }

// impl ProcessControlBlock {
//     fn new(pid: u32, pc: usize) -> ProcessControlBlock {
//         ProcessControlBlock {
//             process_id: pid,
//             state: ProcessState::New,
//             program_counter: pc,
//             // 初始化其他寄存器和状态
//         }
//     }
// }

// fn context_switch(current: &mut ProcessControlBlock, next: &ProcessControlBlock) {
//     // 保存当前进程的状态
//     save_state(current);
//     // 恢复下一个进程的状态
//     restore_state(next);
//     // 更新进程状态
//     current.state = ProcessState::Waiting;
//     // 假设next是一个引用并且可变的
//     // next.state = ProcessState::Running;
// }
