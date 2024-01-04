#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(so_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use so_os::os_start;
use so_os::print;
use so_os::println;
use so_os::shell;
use so_os::shell::shell;
use so_os::task::executor::Executor;
use so_os::task::keyboard;
use so_os::task::Task;
extern crate alloc;
//extern crate compiler_builtins;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use so_os::allocator;
    use so_os::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    so_os::init();
    os_start::start_show();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
    //print!(">>");
    let mut executor = Executor::new();
    executor.spawn(Task::new(shell()));
    //executor.spawn(Task::new(shell::shell_start()));
    executor.run();
    #[cfg(test)]
    test_main();
    println!("It did not crash!");
    so_os::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    so_os::hlt_loop(); // new
}
