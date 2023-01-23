#![feature(asm_experimental_arch)]
#![feature(lang_items)]
#![no_std]
#![no_main]

use core::arch::asm;

struct Pin {
    addr: *const u8,
    ddr: *const u8,
    mask: u8,
}

impl Pin {
    fn new(addr: u16, ddr: u16, mask: u8) -> Pin {
        Pin {
            addr: addr as *const u8,
            ddr: ddr as *const u8,
            mask: mask,
        }
    }

    unsafe fn pin_on(&mut self) {
        core::ptr::write_volatile(
            self.addr as *mut u8, core::ptr::read_volatile(self.addr) | self.mask);
    }

    unsafe fn pin_off(&mut self) {
        core::ptr::write_volatile(
            self.addr as *mut u8, core::ptr::read_volatile(self.addr) & !self.mask);
    }

    unsafe fn set_out(&mut self) {
        core::ptr::write_volatile(
            self.ddr as *mut u8, core::ptr::read_volatile(self.ddr) | self.mask);
    }

    unsafe fn set_in(&mut self) {
        core::ptr::write_volatile(
            self.ddr as *mut u8, core::ptr::read_volatile(self.ddr) & !self.mask);
    }
}

struct Atmega32p8 {
    /// Port B
    PB0: Pin,
    PB1: Pin,
    PB2: Pin,
    PB3: Pin,
    PB4: Pin,
    PB5: Pin,
    PB6: Pin,
    PB7: Pin,

    /// Port C
    PC6: Pin,
    PC7: Pin,

    /// Port D
    PD0: Pin,
    PD1: Pin,
    PD2: Pin,
    PD3: Pin,
    PD4: Pin,
    PD5: Pin,
    PD6: Pin,
    PD7: Pin,

    /// Port E
    PE2: Pin,
    PE6: Pin,

    /// Port F
    PF0: Pin,
    PF1: Pin,
    PF4: Pin,
    PF5: Pin,
    PF6: Pin,
    PF7: Pin,
}

impl Atmega32p8 {
    fn new() -> Atmega32p8 {
        Atmega32p8 {
            /// Port B
            PB0: Pin::new(0x25, 0x24, 1 << 0),
            PB1: Pin::new(0x25, 0x24, 1 << 1),
            PB2: Pin::new(0x25, 0x24, 1 << 2),
            PB3: Pin::new(0x25, 0x24, 1 << 3),
            PB4: Pin::new(0x25, 0x24, 1 << 4),
            PB5: Pin::new(0x25, 0x24, 1 << 5),
            PB6: Pin::new(0x25, 0x24, 1 << 6),
            PB7: Pin::new(0x25, 0x24, 1 << 7 ),

            /// Port En
            PC6: Pin::new(0x28, 0x27, 1 << 6),
            PC7: Pin::new(0x28, 0x27, 1 << 7),

            /// Port Dn
            PD0: Pin::new(0x2B, 0x2A, 1 << 0),
            PD1: Pin::new(0x2B, 0x2A, 1 << 1),
            PD2: Pin::new(0x2B, 0x2A, 1 << 2),
            PD3: Pin::new(0x2B, 0x2A, 1 << 3),
            PD4: Pin::new(0x2B, 0x2A, 1 << 4),
            PD5: Pin::new(0x2B, 0x2A, 1 << 5),
            PD6: Pin::new(0x2B, 0x2A, 1 << 6),
            PD7: Pin::new(0x2B, 0x2A, 1 << 7),

            /// Port En
            PE2: Pin::new(0x2E, 0x2D, 1 << 2),
            PE6: Pin::new(0x2E, 0x2D, 1 << 6),

            /// Port Fn
            PF0: Pin::new(0x31, 0x30, 1 << 0),
            PF1: Pin::new(0x31, 0x30, 1 << 1),
            PF4: Pin::new(0x31, 0x30, 1 << 4),
            PF5: Pin::new(0x31, 0x30, 1 << 5),
            PF6: Pin::new(0x31, 0x30, 1 << 6),
            PF7: Pin::new(0x31, 0x30, 1 << 7),
        }
    }

    /// Busy-wait for `n` instruction
    #[inline(always)]
    fn busywait(&self, n: u16) {
        unsafe {
            asm!("1: sbiw {i}, 1",
                 "brne 1b",
                 i = inout(reg_iw) n => _,
                 )
        }
    }
}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {
}


#[no_mangle]
pub unsafe extern fn main() {

    let mut atm = Atmega32p8::new();

    atm.PB5.set_out();
    atm.PB0.set_out();
    atm.PD7.set_out();

    loop {
        atm.PB5.pin_on();
        atm.PB0.pin_off();
        atm.PD7.pin_on();

        for _ in 0..10  {
            atm.busywait(65535);
        }

        atm.PB5.pin_off();
        atm.PB0.pin_on();
        atm.PD7.pin_off();

        for _ in 0..10  {
            atm.busywait(65535);
        }
    }
}
