#![allow(dead_code)]

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
