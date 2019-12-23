#![no_std]
#![no_main]

extern crate panic_halt;
extern crate tm4c129x_hal as hal;
extern crate alloc;
extern crate fe_rtos;
extern crate fe_osi;
extern crate cortex_m_rt as rt; // v0.5.x

use core::fmt::Write;
use cortex_m_rt::entry;
use hal::prelude::*;
use alloc::vec;

#[entry]
fn main() -> ! {
    let p = hal::Peripherals::take().unwrap();
    let mut sc = p.SYSCTL.constrain();
    sc.clock_setup.oscillator = hal::sysctl::Oscillator::Main(
        hal::sysctl::CrystalFrequency::_25mhz,
        hal::sysctl::SystemClock::UsePll(hal::sysctl::PllOutputFrequency::_120mhz),
    );
    let clocks = sc.clock_setup.freeze();

    let mut porta = p.GPIO_PORTA_AHB.split(&sc.power_control);

    // Activate UART
    let mut uart = hal::serial::Serial::uart0(
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

    let _test_vec = vec![0,1,2,3];

    let mut counter = 0u32;
    loop {
        writeln!(uart, "Hello, world! counter={}", counter).unwrap();
        counter = counter.wrapping_add(1);
    }
}
