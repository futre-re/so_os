// 通用寄存器（如 rax; rbx; rcx; rdx 等）
// 段寄存器（如 cs; ds; es; fs; gs）
// 指令指针寄存器（rip）
// 栈指针寄存器（rsp）
// 基址指针寄存器（rbp）
// 状态寄存器（rflags）
// 控制寄存器（如 cr0; cr3; cr4 等）
pub use x86_64::registers::control;
pub use x86_64::registers::rflags;
pub use x86_64::registers::segmentation;

pub trait RegisterReadWrite {
    fn read(&self);
    fn write(&self);
}

#[cfg(feature(asm))]

pub struct RAX; //累加器寄存器，常用于整数运算和函数返回值。
pub struct RBX; //基址寄存器，常用于数据存储。
pub struct RCX; //计数寄存器，常用于字符串和循环操作。
pub struct RDX; //数据寄存器，常用于整数运算和I/O操作。
pub struct RSI; //源索引寄存器，常用于字符串和数组操作。
pub struct RDI; //目的索引寄存器，同样用于字符串和数组操作。
pub struct RBP; //基指针寄存器，常用于栈帧指针。
pub struct RSP; //栈指针寄存器，指向栈顶。
                //非指定意义寄存器
pub struct R8;
pub struct R9;
pub struct R10;
pub struct R11;
pub struct R12;
pub struct R13;
pub struct R14;
pub struct R15;
pub struct Rip; //指令指针寄存器（rip）
pub struct Rsp; // 栈指针寄存器（rsp）
pub struct Rbp; // 基址指针寄存器（rbp

impl RegisterReadWrite for RAX {
    fn read(&self) -> usize {
        let mut rax: usize;
        unsafe {
            asm!("mov {}, rax", out(reg) rax);
        }
        rax
    }

    fn write(&self, val: usize) {
        unsafe {
            asm!("mov rax, {}", in(reg) val);
        }
    }
}

impl RegisterReadWrite for RBX {
    fn read(&self) -> usize {
        let mut rax: usize;
        unsafe {
            asm!("mov {}, rbp", out(reg) rax);
        }
        rax
    }

    fn write(&self, val: usize) {
        unsafe {
            asm!("mov rbp, {}", in(reg) val);
        }
    }
}

impl RegisterReadWrite for RCX {
    fn read(&self) -> usize {
        let mut rax: usize;
        unsafe {
            asm!("mov {}, rbp", out(reg) rax);
        }
        rax
    }

    fn write(&self, val: usize) {
        unsafe {
            asm!("mov rbp, {}", in(reg) val);
        }
    }
}

impl RegisterReadWrite for RDX {
    fn read(&self) -> usize {
        let mut rax: usize;
        unsafe {
            asm!("mov {}, rbp", out(reg) rax);
        }
        rax
    }

    fn write(&self, val: usize) {
        unsafe {
            asm!("mov rbp, {}", in(reg) val);
        }
    }
}

impl RegisterReadWrite for RSI {
    fn read(&self) -> usize {
        let mut rax: usize;
        unsafe {
            asm!("mov {}, rbp", out(reg) rax);
        }
        rax
    }

    fn write(&self, val: usize) {
        unsafe {
            asm!("mov rbp, {}", in(reg) val);
        }
    }
}

impl RegisterReadWrite for RDI {
    fn read(&self) -> usize {
        let mut rax: usize;
        unsafe {
            asm!("mov {}, rbp", out(reg) rax);
        }
        rax
    }

    fn write(&self, val: usize) {
        unsafe {
            asm!("mov rbp, {}", in(reg) val);
        }
    }
}

impl RegisterReadWrite for RBP {
    fn read(&self) -> usize {
        let mut rax: usize;
        unsafe {
            asm!("mov {}, rbp", out(reg) rax);
        }
        rax
    }

    fn write(&self, val: usize) {
        unsafe {
            asm!("mov rbp, {}", in(reg) val);
        }
    }
}

impl RegisterReadWrite for RSP {
    fn read(&self) -> usize {
        let mut rax: usize;
        unsafe {
            asm!("mov {}, rbp", out(reg) rax);
        }
        rax
    }

    fn write(&self, val: usize) {
        unsafe {
            asm!("mov rbp, {}", in(reg) val);
        }
    }
}

impl RegisterReadWrite for R8 {
    fn read(&self) -> usize {
        let mut rax: usize;
        unsafe {
            asm!("mov {}, rbp", out(reg) rax);
        }
        rax
    }

    fn write(&self, val: usize) {
        unsafe {
            asm!("mov rbp, {}", in(reg) val);
        }
    }
}

impl RegisterReadWrite for R9 {
    fn read(&self) -> usize {
        let mut rax: usize;
        unsafe {
            asm!("mov {}, rbp", out(reg) rax);
        }
        rax
    }

    fn write(&self, val: usize) {
        unsafe {
            asm!("mov rbp, {}", in(reg) val);
        }
    }
}

impl RegisterReadWrite for R10 {
    fn read(&self) -> usize {
        let mut rax: usize;
        unsafe {
            asm!("mov {}, rbp", out(reg) rax);
        }
        rax
    }

    fn write(&self, val: usize) {
        unsafe {
            asm!("mov rbp, {}", in(reg) val);
        }
    }
}

impl RegisterReadWrite for R11 {
    fn read(&self) -> usize {
        let mut rax: usize;
        unsafe {
            asm!("mov {}, rbp", out(reg) rax);
        }
        rax
    }

    fn write(&self, val: usize) {
        unsafe {
            asm!("mov rbp, {}", in(reg) val);
        }
    }
}

impl RegisterReadWrite for R12 {
    fn read(&self) -> usize {
        let mut rax: usize;
        unsafe {
            asm!("mov {}, rbp", out(reg) rax);
        }
        rax
    }

    fn write(&self, val: usize) {
        unsafe {
            asm!("mov rbp, {}", in(reg) val);
        }
    }
    fn write(&self) {
        todo!()
    }
}

impl RegisterReadWrite for R13 {
    fn read(&self) -> usize {
        let mut rax: usize;
        unsafe {
            asm!("mov {}, rbp", out(reg) rax);
        }
        rax
    }

    fn write(&self, val: usize) {
        unsafe {
            asm!("mov rbp, {}", in(reg) val);
        }
    }
}

impl RegisterReadWrite for R14 {
    fn read(&self) -> usize {
        let mut rax: usize;
        unsafe {
            asm!("mov {}, rbp", out(reg) rax);
        }
        rax
    }

    fn write(&self, val: usize) {
        unsafe {
            asm!("mov rbp, {}", in(reg) val);
        }
    }
}

impl RegisterReadWrite for R15 {
    fn read(&self) -> usize {
        let mut rax: usize;
        unsafe {
            asm!("mov {}, rbp", out(reg) rax);
        }
        rax
    }

    fn write(&self, val: usize) {
        unsafe {
            asm!("mov rbp, {}", in(reg) val);
        }
    }
}

impl RegisterReadWrite for Rip {
    fn read(&self) -> usize {
        let mut rax: usize;
        unsafe {
            asm!("mov {}, rbp", out(reg) rax);
        }
        rax
    }

    fn write(&self, val: usize) {
        unsafe {
            asm!("mov rbp, {}", in(reg) val);
        }
    }
}

impl RegisterReadWrite for Rsp {
    fn read(&self) -> usize {
        let mut rax: usize;
        unsafe {
            asm!("mov {}, rbp", out(reg) rax);
        }
        rax
    }

    fn write(&self, val: usize) {
        unsafe {
            asm!("mov rbp, {}", in(reg) val);
        }
    }
}

impl RegisterReadWrite for Rbp {
    fn read(&self) -> usize {
        let mut rax: usize;
        unsafe {
            asm!("mov {}, rbp", out(reg) rax);
        }
        rax
    }

    fn write(&self, val: usize) {
        unsafe {
            asm!("mov rbp, {}", in(reg) val);
        }
    }
}
