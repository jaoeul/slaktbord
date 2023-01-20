#![feature(asm_experimental_arch)]
#![feature(lang_items)]
#![no_std]
#![no_main]

use core::arch::asm;

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {
}

/// Busy-wait for `n` instruction
#[inline(always)]
fn busywait(n: u16) {
    unsafe {
        asm!("1: sbiw {i}, 1",
             "brne 1b",
             i = inout(reg_iw) n => _,
             )
    }
}

#[no_mangle]
pub extern fn main() {

    // Magic values from datasheet
    let port_b_adr = 0x25 as *const u8;
    let ddr_b_adr  = 0x24 as *const u8;
    let b5_mask: u8 = 1 << 5;

    unsafe {

        // Set pin B5 to output
        core::ptr::write_volatile(
            ddr_b_adr as *mut u8, core::ptr::read_volatile(ddr_b_adr) | b5_mask);

        loop {
            // Set pin B5 to 1
            core::ptr::write_volatile(
                port_b_adr as *mut u8, core::ptr::read_volatile(port_b_adr) | b5_mask);

            for _ in 0..10  {
                busywait(65535);
            }

            // Set pin B5 to 0
            core::ptr::write_volatile(
                port_b_adr as *mut u8,
                core::ptr::read_volatile(port_b_adr) & !b5_mask);

            for _ in 0..10  {
                busywait(65535);
            }
        }
    }
}
