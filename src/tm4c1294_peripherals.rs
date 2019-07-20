#![allow(dead_code)]

use core::mem::replace;

use crate::system_control::SystemControl;
use crate::gpio::Gpio;
use crate::uart::Uart;

#[allow(improper_ctypes)]
extern "C" {
    pub static mut SYSTEM_CONTROL: SystemControl;
    pub static mut GPIOA: Gpio;
    pub static mut GPION: Gpio;
    pub static mut UART0: Uart;
}

pub struct TM4C129Peripherals {
    system_control: Option<&'static mut SystemControl>,
    gpioa:          Option<&'static mut Gpio>,
    gpion:          Option<&'static mut Gpio>,
    uart0:          Option<&'static mut Uart>,
}

impl TM4C129Peripherals {
    pub fn take_system_control(&mut self) -> Option<&'static mut SystemControl> {
        replace(&mut self.system_control, None)
    }

    pub fn take_gpion(&mut self) -> Option<&'static mut Gpio> {
        replace(&mut self.gpion, None)
    }

    pub fn take_gpioa(&mut self) -> Option<&'static mut Gpio> {
        replace(&mut self.gpioa, None)
    }

    pub fn take_uart0(&mut self) -> Option<&'static mut Uart> {
        replace(&mut self.uart0, None)
    }
}

pub fn get_peripherals() -> TM4C129Peripherals {
    unsafe {
        TM4C129Peripherals {
            system_control: Some(&mut SYSTEM_CONTROL),
            gpioa: Some(&mut GPIOA),
            gpion: Some(&mut GPION),
            uart0: Some(&mut UART0),
        }
    }
}
