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
    let gpioa = p.take_gpioa().unwrap();
    let mut uart0 = p.take_uart0().unwrap();
    let pwm0 = p.take_pwm0().unwrap();

    // Configure the CPU for the maximum operating frequency
    let cpu_freq = sysctl.tm4c129_config_sysclk(CPU_FREQ, XTAL_FREQ);

    // Set up the PWM module
    sysctl.enable_pwm_clock(system_control::Pwm::Pwm0);
    pwm0.

    // Set up LEDs
    sysctl.enable_gpio_clock(system_control::GpioPort::GpioN);
    gpion.configure_as_output(gpio::Pin::Pin0);
    gpion.configure_as_output(gpio::Pin::Pin1);

    // Set up the debug UART
    sysctl.enable_gpio_clock(system_control::GpioPort::GpioA);
    sysctl.enable_uart_clock(system_control::Uart::Uart0);
    gpioa.select_alternate_function(gpio::Pin::Pin0, 1);
    gpioa.select_alternate_function(gpio::Pin::Pin1, 1);

    let _baud = uart0.configure(cpu_freq, 115200, uart::Parity::None, uart::StopBits::One, uart::WordLength::Eight).unwrap();
    let mut uart_driver = uart::drivers::UartBlockingDriver::new(&mut uart0);

    loop {
        gpion.set_low(gpio::Pin::Pin0);
        gpion.set_high(gpio::Pin::Pin1);
        uart_driver.putchar(b'a');
        let mut i = 200_000;
        while i > 0 {
            i = i - 1;
        }

        gpion.set_high(gpio::Pin::Pin0);
        gpion.set_low(gpio::Pin::Pin1);
        uart_driver.putchar(b'b');
        let mut i = 200_000;
        while i > 0 {
            i = i - 1;
        }
    }
}
