#![no_std]
#![no_main]

mod cortex_m4_init;
mod tm4c1294_peripherals;
mod system_control;
mod gpio;
mod uart;

use crate::tm4c1294_peripherals::SYSTEM_CONTROL;
use crate::tm4c1294_peripherals::GPION;

const XTAL_FREQ: u32 = 25_000_000;
const CPU_FREQ: u32 = 120_000_000;

#[no_mangle]
pub fn main() -> ! {
    let _cpu_freq = unsafe { SYSTEM_CONTROL.tm4c129_config_sysclk(CPU_FREQ, XTAL_FREQ) };
    unsafe { SYSTEM_CONTROL.enable_gpio_clock(system_control::GpioPort::GpioN) };
    unsafe { GPION.configure_as_output(gpio::Pin::Pin0) };
    loop {
        unsafe {
            GPION.set_low(gpio::Pin::Pin0);
            let mut i = 50_000_000;
            while i > 0 {
                i = i - 1;
            }

            GPION.set_high(gpio::Pin::Pin0);
            let mut i = 50_000_000;
            while i > 0 {
                i = i - 1;
            }
        }
    }
}
