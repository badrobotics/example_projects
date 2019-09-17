#![no_std]
#![no_main]

extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
extern crate tm4c129x_hal as hal;
extern crate atomic_queue;

use core::fmt::Write;
use hal::prelude::*;
use atomic_queue::AtomicQueue;

#[no_mangle]
pub fn main() -> ! {
    let p = hal::Peripherals::take().unwrap();

    let mut sc = p.SYSCTL.constrain();
    sc.clock_setup.oscillator = hal::sysctl::Oscillator::Main(
        hal::sysctl::CrystalFrequency::_16mhz,
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

    // Create the queue
    let mut storage: [u8; 2] = [0; 2];
    let ref queue: AtomicQueue<u8> = {
        let m = AtomicQueue::new(&mut storage);
        m
    };

    // Fill the first two slots in the queue with dummy variables
    match queue.push(0) {
        Err(_) => panic!("No room to push?"),
        Ok(_) => {},
    }
    match queue.push(0) {
        Err(_) => panic!("No room to push?"),
        Ok(_) => {},
    }

    let mut counter = 0u8;
    loop {
        writeln!(uart, "Hello, world! counter={} two_values_ago={}", counter, queue.pop().unwrap()).unwrap();
        match queue.push(counter) {
            Err(_) => panic!("No room to push?"),
            Ok(_) => {},
        }
        counter = counter.wrapping_add(1);
    }
}
