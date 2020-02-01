// error-pattern: `#[panic_handler]` function required, but not found

// Regression test for #54505 - range borrowing suggestion had
// incorrect syntax (missing parentheses).

// This test doesn't use std
// (so all Ranges resolve to core::ops::Range...)

#![no_std]
#![feature(core_intrinsics, lang_items)]

use core::ops::RangeBounds;

#[cfg(any(not(target_arch = "wasm32"), target_os = "emscripten"))]
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[cfg(target_os = "windows")]
extern "C" fn eh_unwind_resume(_: *mut u8) -> ! {
    core::intrinsics::abort();
}
#[cfg(target_os = "windows")]
#[lang = "eh_unwind_resume"]
static _RESUME: fn(*mut u8) -> ! = eh_unwind_resume;

// take a reference to any built-in range
fn take_range(_r: &impl RangeBounds<i8>) {}

fn main() {
    take_range(0..1);
    //~^ ERROR mismatched types [E0308]
    //~| HELP consider borrowing here
    //~| SUGGESTION &(0..1)

    take_range(1..);
    //~^ ERROR mismatched types [E0308]
    //~| HELP consider borrowing here
    //~| SUGGESTION &(1..)

    take_range(..);
    //~^ ERROR mismatched types [E0308]
    //~| HELP consider borrowing here
    //~| SUGGESTION &(..)

    take_range(0..=1);
    //~^ ERROR mismatched types [E0308]
    //~| HELP consider borrowing here
    //~| SUGGESTION &(0..=1)

    take_range(..5);
    //~^ ERROR mismatched types [E0308]
    //~| HELP consider borrowing here
    //~| SUGGESTION &(..5)

    take_range(..=42);
    //~^ ERROR mismatched types [E0308]
    //~| HELP consider borrowing here
    //~| SUGGESTION &(..=42)
}
