#![no_main]
#![no_std]

mod cortex_m4_init;
mod tm4c1294_peripherals;
mod system_control;
mod gpio;
mod uart;

use crate::tm4c1294_peripherals::PERIPHERALS;

const XTAL_FREQ: u32 = 25_000_000;
const CPU_FREQ: u32 = 120_000_000;

#[no_mangle]
entry!(main);
pub fn main() -> ! {
    let sysctl = unsafe { PERIPHERALS.take_sysctl() };
    let _cpu_freq = sysctl.tm4c129_config_sysclk(CPU_FREQ, XTAL_FREQ);
    loop {}
}
