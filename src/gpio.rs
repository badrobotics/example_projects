#![allow(non_snake_case)]

use volatile_register::{RW, RO, WO};

#[allow(dead_code)]
pub enum Pin {
    Pin0,
    Pin1,
    Pin2,
    Pin3,
    Pin4,
    Pin5,
    Pin6,
    Pin7,
}

#[repr(C)]
pub struct Gpio {
    pub GPIODATA: RW<u32>,
    Reserved0: [RO<u32>; 255],
    pub GPIODIR: RW<u32>,
    pub GPIOIS: RW<u32>,
    pub GPIOIBE: RW<u32>,
    pub GPIOIEV: RW<u32>,
    pub GPIOIM: RW<u32>,
    pub GPIORIS: RO<u32>,
    pub GPIOMIS: RO<u32>,
    pub GPIOICR: WO<u32>,
    pub GPIOAFSEL: RW<u32>,
    Reserved1: [RO<u32>; 55],
    pub GPIODR2R: RW<u32>,
    pub GPIODR4R: RW<u32>,
    pub GPIODR8R: RW<u32>,
    pub GPIOODR: RW<u32>,
    pub GPIOPUR: RW<u32>,
    pub GPIOPDR: RW<u32>,
    pub GPIOSLR: RW<u32>,
    pub GPIODEN: RW<u32>,
    pub GPIOLOCK: RW<u32>,
    pub GPIOCR: WO<u32>,
    pub GPIOAMSEL: RW<u32>,
    pub GPIOPCTL: RW<u32>,
    pub GPIOADCCTL: RW<u32>,
    pub GPIODMACTL: RW<u32>,
    pub GPIOSI: RW<u32>,
    pub GPIODR12R: RW<u32>,
    pub GPIOWAKEPEN: RW<u32>,
    pub GPIOWAKELVL: RW<u32>,
    pub GPIOWAKESTAT: RO<u32>,
    Reserved2: [RO<u32>; 669],
    pub GPIOPP: RO<u32>,
    pub GPIOPC: RW<u32>,
    Reserved3: [RO<u32>; 2],
    pub GPIOPeriphID4: RO<u32>,
    pub GPIOPeriphID5: RO<u32>,
    pub GPIOPeriphID6: RO<u32>,
    pub GPIOPeriphID7: RO<u32>,
    pub GPIOPeriphID0: RO<u32>,
    pub GPIOPeriphID1: RO<u32>,
    pub GPIOPeriphID2: RO<u32>,
    pub GPIOPeriphID3: RO<u32>,
    pub GPIOPCellID0: RO<u32>,
    pub GPIOPCellID1: RO<u32>,
    pub GPIOPCellID2: RO<u32>,
    pub GPIOPCellID3: RO<u32>,
}

impl Gpio {
    fn get_pin_bitmask(pin: Pin) -> u32 {
        match pin {
            Pin::Pin0 => 1<<0,
            Pin::Pin1 => 1<<1,
            Pin::Pin2 => 1<<2,
            Pin::Pin3 => 1<<3,
            Pin::Pin4 => 1<<4,
            Pin::Pin5 => 1<<5,
            Pin::Pin6 => 1<<6,
            Pin::Pin7 => 1<<7,
        }
    }

    pub fn configure_as_output(&mut self, pin: Pin) {
        unsafe {
            self.GPIODIR.modify(|x| x | Gpio::get_pin_bitmask(pin));
        }
    }

    pub fn set_low(&mut self, pin: Pin) {
        unsafe {
            self.GPIODATA.modify(|x| x | Gpio::get_pin_bitmask(pin));
        }
    }

    pub fn set_high(&mut self, pin: Pin) {
        unsafe {
            self.GPIODATA.modify(|x| x & !Gpio::get_pin_bitmask(pin));
        }
    }
}
