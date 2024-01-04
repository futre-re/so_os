use alloc::{string::String, vec::Vec};
struct Process {
    id: u32,
    name: String,
    remaining_time: u32,
}

impl Process {
    fn new(id: u32, name: String, remaining_time: u32) -> Process {
        Process {
            id,
            name,
            remaining_time,
        }
    }

    fn execute(&mut self, time_slice: u32) {
        if self.remaining_time > time_slice {
            self.remaining_time -= time_slice;
        } else {
            self.remaining_time = 0;
        }
    }

    fn is_finished(&self) -> bool {
        self.remaining_time == 0
    }
}


fn time_slice_round_robin(processes: &mut Vec<Process>, time_slice: u32) {
    let mut queue: Vec<&mut Process> = processes.iter_mut().collect();
    let mut index = 0;

    while !queue.is_empty() {
        let current_process = queue.get_mut(index).unwrap();
        current_process.execute(time_slice);
        if current_process.is_finished() {
            queue.remove(index);
        } else {
            index = (index + 1) % queue.len();
        }
    }
}