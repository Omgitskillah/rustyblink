#![no_std]
#![no_main]

use core::ptr::read_volatile;
use core::ptr::write_volatile;
use cortex_m::asm::nop;
use cortex_m_rt::entry;
use panic_halt as _;
/* LED is on PA5 */

#[entry]
fn main() -> ! {
    const RCC_IOPENR: *mut u32 = 0x4002_1034 as *mut u32;
    const GPIO_PORTA: *mut u32 = 0x5000_0000 as *mut u32;
    const GPIO_PORTA_BRR: *mut u32 = 0x5000_0028 as *mut u32;
    const GPIO_PORTA_BSRR: *mut u32 = 0x5000_0018 as *mut u32;

    let mut toggle: bool = false;
    unsafe {
        // enable clock
        let mut tmp = read_volatile(RCC_IOPENR);
        tmp |= 0x01;
        write_volatile(RCC_IOPENR, tmp);
        tmp = read_volatile(RCC_IOPENR); // needed for clock enable delay

        // configure PA5
        tmp = read_volatile(GPIO_PORTA);
        tmp &= !((0x3 << 0) << (5 * 2));
        tmp |= (0x01 & 0x00000003) << (5 * 2);
        write_volatile(GPIO_PORTA, tmp);
    }

    loop {
        toggle = !toggle;

        unsafe {
            if toggle == true {
                write_volatile(GPIO_PORTA_BRR, 0x20);
            } else {
                write_volatile(GPIO_PORTA_BSRR, 0x20);
            }
        }
        for _ in 0..40_000 {
            nop();
        }
    }
}
