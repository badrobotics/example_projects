#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting as sh;
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
//extern crate tm4c123x_hal as hal;
extern crate tm4c129x_hal as hal;
extern crate tm4c129x;
extern crate atomic_queue;
#[macro_use]
extern crate lazy_static;

use core::fmt::Write;

use rt::ExceptionFrame;
use rt::exception;
use rt::entry;
use sh::hio;

use tm4c129x::interrupt;

use hal::prelude::*;
use hal::timer::TIMER0;
use hal::time::Hertz;
use hal::timer::Event;

use atomic_queue::AtomicQueue;

static mut MSG_QUEUE_STORAGE: [u8; 100] = [0; 100];
lazy_static! {
    static ref MSG_QUEUE: AtomicQueue<'static, u8> = {
        let m = unsafe { AtomicQueue::new(&mut MSG_QUEUE_STORAGE) };
        m
    };
}

#[entry]
fn main() -> ! {
    // Set up semihosting
    let mut stdout = hio::hstdout().unwrap();
    writeln!(stdout, "Semihosting initialized").unwrap();

    let peripherals = hal::Peripherals::take().unwrap();
    let _core_peripherals = hal::CorePeripherals::take().unwrap();

    writeln!(stdout, "Setting up system clock").unwrap();
    let mut sc = peripherals.SYSCTL.constrain();
    sc.clock_setup.oscillator = hal::sysctl::Oscillator::Main(
        hal::sysctl::CrystalFrequency::_16mhz,
        hal::sysctl::SystemClock::UsePll(hal::sysctl::PllOutputFrequency::_120mhz),
    );
    let clocks = sc.clock_setup.freeze();

    writeln!(stdout, "Setting up interrupts").unwrap();
    unsafe {
        tm4c129x::NVIC::unmask(tm4c129x::Interrupt::TIMER0A);
    }

    writeln!(stdout, "Enabling GPIO PORTA").unwrap();
    let mut porta = peripherals.GPIO_PORTA_AHB.split(&sc.power_control);

    writeln!(stdout, "Activating UART0").unwrap();
    let mut uart = hal::serial::Serial::uart0(
        peripherals.UART0,
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

    // Activate Timer
    let mut timer0 = hal::timer::Timer::<TIMER0>::timer0(
        peripherals.TIMER0,
        Hertz(2),
        &sc.power_control,
        &clocks,
    );
    timer0.listen(Event::TimeOut);

    writeln!(stdout, "Starting timers. Listen on UART0 for timout messages.").unwrap();
    timer0.start(Hertz(20));

    loop {
        match MSG_QUEUE.pop() {
            Some(0) => {
                writeln!(uart, "Timer 0").unwrap();
                timer0.start(Hertz(20));
            },
            _ => {},
        }
    }
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

#[exception]
fn DefaultHandler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}

#[interrupt]
fn TIMER0A() {
    // clear the interrupt
    unsafe { (*tm4c129x::TIMER0::ptr()).icr.write(|w| w.tatocint().bit(true)); }
    match MSG_QUEUE.push(0) {
        _ => {},
    }
}
