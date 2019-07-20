#![no_std]
#![no_main]

mod cortex_m4_init;
mod tm4c1294_peripherals;
mod system_control;
mod gpio;
mod uart;

use crate::tm4c1294_peripherals::get_peripherals;

const XTAL_FREQ: u32 = 25_000_000;
const CPU_FREQ: u32 = 120_000_000;

#[no_mangle]
pub fn main() -> ! {
    let mut p = get_peripherals();
    let sysctl = p.take_system_control().unwrap();
    let gpion = p.take_gpion().unwrap();

    let _cpu_freq = sysctl.tm4c129_config_sysclk(CPU_FREQ, XTAL_FREQ);

    sysctl.enable_gpio_clock(system_control::GpioPort::GpioN);
    gpion.configure_as_output(gpio::Pin::Pin0);
    gpion.configure_as_output(gpio::Pin::Pin1);
    gpion.configure_as_output(gpio::Pin::Pin2);

    loop {
        gpion.set_low(gpio::Pin::Pin0);
        gpion.set_low(gpio::Pin::Pin1);
        gpion.set_low(gpio::Pin::Pin2);
        let mut i = 5_000_000;
        while i > 0 {
            i = i - 1;
        }

        gpion.set_high(gpio::Pin::Pin0);
        gpion.set_high(gpio::Pin::Pin1);
        gpion.set_high(gpio::Pin::Pin2);
        let mut i = 5_000_000;
        while i > 0 {
            i = i - 1;
        }
    }
}
