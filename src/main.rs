#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

// use cortex_m::asm;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    // asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

    let pioc_per: *mut i32  = 0x400e1200 as *mut i32;
    let pioc_oer: *mut i32  = 0x400e1210 as *mut i32;
    let pioc_odsr: *mut i32  = 0x400e1238 as *mut i32;
    let pioc_ower: *mut i32  = 0x400e12a0 as *mut i32;
    let pioc_owdr: *mut i32  = 0x400e12a4 as *mut i32;
    let wdt_mr: *mut i32  = 0x400e1854 as *mut i32;

    unsafe {
        *wdt_mr = 0x000080000; // Disable watchdog timer.

        *pioc_per = 0x100;
        *pioc_oer = 0x100;
        *pioc_owdr = 0;
        *pioc_ower = 0x100;
    }

    loop {
        unsafe {
            *pioc_odsr  ^= 0x100;
        }

        for _wait in 0..10000 {}
    }
}
