#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(global_asm)]

#[macro_use]
mod console;
mod batch;
mod lang_items;
mod sbi;
mod trap;
use sbi::sbi_call;

global_asm!(include_str!("entry.asm"));

const SBI_SHUTDOWN: usize = 8;
pub fn shutdown() {
    sbi_call(SBI_SHUTDOWN, 0, 0, 0);
}

#[allow(dead_code)]
fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) })
}

#[no_mangle]
extern "C" fn rust_main() {
    warn!("hello wolrd");
    debug!("hello wolrd");
    info!("hello wolrd");
    trace!("hello wolrd");
    error!("hello wolrd");
    shutdown()
}
