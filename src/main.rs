#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(asm)]

#[no_mangle]
fn _start() {
    let buf = "Hello, World!\n";
    unsafe {
        // align stack (if we want to call real code)
        // asm!("xor rbp, rbp", "pop rdi", "mov rsi, rsp", "and rsp, -16");

        asm!(
            "syscall",
            in("rax") 1, // syscall number
            in("rdi") 1, // fd (stdout)
            in("rsi") buf.as_ptr(),
            in("rdx") buf.len(),
            out("rcx") _, // clobbered by syscalls
            out("r11") _, // clobbered by syscalls
        );

        // exit(0)
        asm!(
            "syscall",
            in("rax") 60,
            in("rdi") 0,
        )
    }
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
