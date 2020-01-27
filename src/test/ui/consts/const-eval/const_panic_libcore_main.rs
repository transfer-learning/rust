#![crate_type = "bin"]
#![feature(lang_items, core_intrinsics)]
#![feature(const_panic)]
#![no_main]
#![no_std]

use core::panic::PanicInfo;

const Z: () = panic!("cheese");
//~^ ERROR any use of this value will cause an error

const Y: () = unreachable!();
//~^ ERROR any use of this value will cause an error

const X: () = unimplemented!();
//~^ ERROR any use of this value will cause an error

#[lang = "eh_personality"]
fn eh() {}

unsafe extern "C" fn eh_unwind_resume(_: *mut u8) -> ! {
    core::intrinsics::abort();
}
#[lang = "eh_unwind_resume"]
static _RESUME: unsafe extern "C" fn(*mut u8) -> ! = eh_unwind_resume;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
