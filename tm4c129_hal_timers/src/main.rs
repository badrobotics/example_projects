#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting as sh;
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
extern crate atomic_queue;
extern crate embedded_hal;
#[macro_use]
extern crate lazy_static;

extern crate tm4c129x_hal as hal;
extern crate tm4c129x as device;

use core::fmt::Write;

use rt::ExceptionFrame;
use rt::exception;
use rt::entry;
use sh::hio;

use device::interrupt;

use hal::prelude::*;
use hal::timer::{TIMER0, TIMER1, TIMER2, TIMER3, TIMER4, TIMER5, TIMER6, TIMER7};
use hal::time::Hertz;
use hal::timer::Event;

use atomic_queue::AtomicQueue;

macro_rules! timer_interrupt_macro {
    ( $($int_name:ident,$timer_path:ident,$timer_num:tt;)+ ) => {
        $(
            #[interrupt]
            fn $int_name() {
                // clear the interrupt
                unsafe { (*device::$timer_path::ptr()).icr.write(|w| w.tatocint().bit(true)); }
                match MSG_QUEUE.push($timer_num) {
                    _ => {},
                }
            }
        )+
    }
}

macro_rules! configure_timers {
    ( $pc_ref:expr, $clk_ref:expr ; $( ($var_name:ident, $peripheral:expr, $hal_func:path, $interrupt:ident) ,)+ ) => {
        $(
            let mut $var_name = {
                let mut tim = $hal_func (
                    $peripheral,
                    Hertz(10),
                    $pc_ref,
                    $clk_ref,
                );
                unsafe { device::NVIC::unmask(device::Interrupt::$interrupt); }
                tim.listen(Event::TimeOut);
                tim
            };
        )+
    }
}

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

    configure_timers! { &sc.power_control, &clocks;
        (timer0, peripherals.TIMER0, hal::timer::Timer::<TIMER0>::timer0, TIMER0A),
        (timer1, peripherals.TIMER1, hal::timer::Timer::<TIMER1>::timer1, TIMER1A),
        (timer2, peripherals.TIMER2, hal::timer::Timer::<TIMER2>::timer2, TIMER2A),
        (timer3, peripherals.TIMER3, hal::timer::Timer::<TIMER3>::timer3, TIMER3A),
        (timer4, peripherals.TIMER4, hal::timer::Timer::<TIMER4>::timer4, TIMER4A),
        (timer5, peripherals.TIMER5, hal::timer::Timer::<TIMER5>::timer5, TIMER5A),
        (timer6, peripherals.TIMER6, hal::timer::Timer::<TIMER6>::timer6, TIMER6A),
        (timer7, peripherals.TIMER7, hal::timer::Timer::<TIMER7>::timer7, TIMER7A),
    };

    writeln!(stdout, "Starting timers. Listen on UART0 for timout messages.").unwrap();
    timer0.start(Hertz(10));
    timer1.start(Hertz(20));
    timer2.start(Hertz(30));
    timer3.start(Hertz(40));
    timer4.start(Hertz(50));
    timer5.start(Hertz(60));
    timer6.start(Hertz(60));
    timer7.start(Hertz(70));

    loop {
        match MSG_QUEUE.pop() {
            Some(0) => {
                writeln!(uart, "Timer 0").unwrap();
                timer0.start(Hertz(10));
            },
            Some(1) => {
                writeln!(uart, "Timer 1").unwrap();
                timer1.start(Hertz(20));
            },
            Some(2) => {
                writeln!(uart, "Timer 2").unwrap();
                timer2.start(Hertz(30));
            },
            Some(3) => {
                writeln!(uart, "Timer 3").unwrap();
                timer3.start(Hertz(40));
            },
            Some(4) => {
                writeln!(uart, "Timer 4").unwrap();
                timer4.start(Hertz(50));
            },
            Some(5) => {
                writeln!(uart, "Timer 5").unwrap();
                timer5.start(Hertz(60));
            },
            Some(6) => {
                writeln!(uart, "Timer 6").unwrap();
                timer6.start(Hertz(70));
            },
            Some(7) => {
                writeln!(uart, "Timer 7").unwrap();
                timer7.start(Hertz(80));
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

timer_interrupt_macro! {
    TIMER0A, TIMER0, 0;
    TIMER1A, TIMER1, 1;
    TIMER2A, TIMER2, 2;
    TIMER3A, TIMER3, 3;
    TIMER4A, TIMER4, 4;
    TIMER5A, TIMER5, 5;
    TIMER6A, TIMER6, 6;
    TIMER7A, TIMER7, 7;
}
