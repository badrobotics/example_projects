//! Prints "Hello, world!" on the OpenOCD console using semihosting
//!
//! ---

#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting as sh;
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
//extern crate tm4c123x_hal as hal;
extern crate tm4c129x_hal as hal;

use core::fmt::Write;

use rt::ExceptionFrame;
use rt::exception;
use rt::entry;
use sh::hio;

use hal::prelude::*;

#[entry]
fn main() -> ! {
    // Set up semihosting
    let mut stdout = hio::hstdout().unwrap();
    writeln!(stdout, "Semihosting initialized").unwrap();

    let p = hal::Peripherals::take().unwrap();

    writeln!(stdout, "Setting up system clock").unwrap();
    let mut sc = p.SYSCTL.constrain();
    sc.clock_setup.oscillator = hal::sysctl::Oscillator::Main(
        hal::sysctl::CrystalFrequency::_16mhz,
        hal::sysctl::SystemClock::UsePll(hal::sysctl::PllOutputFrequency::_120mhz),
    );
    let clocks = sc.clock_setup.freeze();

    writeln!(stdout, "Enabling GPIO PORTA").unwrap();
    let mut porta = p.GPIO_PORTA_AHB.split(&sc.power_control);

    // Activate UART
    writeln!(stdout, "Activating UART0").unwrap();
    let mut _uart = hal::serial::Serial::uart0(
        p.UART0,
        porta
            .pa1
            .into_af_push_pull::<hal::gpio::AF1>(&mut porta.control),
        porta
            .pa0
            .into_af_push_pull::<hal::gpio::AF1>(&mut porta.control),
        (),
        (),
        115200_u32.bps(),
        hal::serial::NewlineMode::SwapLFtoCRLF,
        &clocks,
        &sc.power_control,
    );

    loop {}
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

#[exception]
fn DefaultHandler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
