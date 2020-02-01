// Test that we don't ICE when we are missing the owned_box lang item.

// error-pattern: requires `owned_box` lang_item

#![feature(lang_items, box_syntax, core_intrinsics)]
#![no_std]

use core::panic::PanicInfo;

fn main() {
    let x = box 1i32;
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
unsafe extern "C" fn eh_unwind_resume(_: *mut u8) -> ! {
    core::intrinsics::abort();
}
#[lang = "eh_unwind_resume"]
static _RESUME: unsafe extern "C" fn(*mut u8) -> ! = eh_unwind_resume;
#[lang = "panic_impl"]
fn panic_impl(panic: &PanicInfo) -> ! {
    loop {}
}
