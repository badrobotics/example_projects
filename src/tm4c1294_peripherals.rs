#![no_std]

use core::mem::replace;

use crate::system_control::SystemControl;
use crate::gpio::Gpio;
use crate::uart::Uart;

struct TM4C129Peripherals {
    system_control: Option<&'static mut SystemControl>,
    gpioa: Option<&'static mut Gpio>,
    uart0: Option<&'static mut Uart>,
}

impl TM4C129Peripherals {
    pub fn take_sysctl(&mut self) -> &'static mut SystemControl {
        let p = replace(&mut self.system_control, None);
        let sysctl = p.unwrap();
        sysctl
    }
    pub fn take_gpioa(&mut self) -> &'static mut Gpio {
        let p = replace(&mut self.gpioa, None);
        let gpio = p.unwrap();
        gpio
    }
    pub fn take_uart0(&mut self) -> &'static mut Uart {
        let p = replace(&mut self.uart0, None);
        let uart = p.unwrap();
        uart
    }
}

extern "C" {
    static mut SYSTEM_CONTROL: SystemControl;
    static mut GPIOA: Gpio;
    static mut UART0: Uart;
}

pub static mut PERIPHERALS: TM4C129Peripherals = unsafe {
    TM4C129Peripherals {
        system_control: Some(&mut SYSTEM_CONTROL),
        gpioa:          Some(&mut GPIOA),
        uart0:          Some(&mut UART0),
    }
};
