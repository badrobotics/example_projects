#![no_std]
#![no_main]

extern crate alloc;
extern crate fe_osi;
use fe_rtos;
use rust_tm4c;
use tm4c123x_hal as hal;

use alloc::boxed::Box;
use core::fmt::Write;
use cortex_m::peripheral::scb::Exception;
use hal::prelude::*;

const CPU_FREQ: u32 = 80_000_000;
static mut COUNTER: u8 = 0;

fn counter_task(_: &mut u32) {
    unsafe {
        loop {
            COUNTER = COUNTER.wrapping_add(1);
            fe_osi::sleep(1000);
        }
    }
}

fn hello_world<T: Write>(writer: &mut T) {
    unsafe {
        loop {
            writeln!(writer, "Hello, world! seconds={}", COUNTER).unwrap();
            fe_osi::sleep(100);
        }
    }
}

#[no_mangle]
fn main() -> ! {
    let p = hal::Peripherals::take().unwrap();
    let cp = hal::CorePeripherals::take().unwrap();

    let mut sc = p.SYSCTL.constrain();
    let systick = cp.SYST;
    sc.clock_setup.oscillator = hal::sysctl::Oscillator::Main(
        hal::sysctl::CrystalFrequency::_16mhz,
        hal::sysctl::SystemClock::UsePll(hal::sysctl::PllOutputFrequency::_80_00mhz),
    );
    let clocks = sc.clock_setup.freeze();

    let mut porta = p.GPIO_PORTA.split(&sc.power_control);

    // Activate UART
    let uart0 = hal::serial::Serial::uart0(
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

    rust_tm4c::interrupt::int_register(
        (Exception::SysTick.irqn() + 16) as u32,
        fe_rtos::task::sys_tick,
    );
    rust_tm4c::interrupt::int_register(
        (Exception::PendSV.irqn() + 16) as u32,
        fe_rtos::task::context_switch,
    );
    rust_tm4c::interrupt::int_register(
        (Exception::SVCall.irqn() + 16) as u32,
        fe_rtos::syscall::svc_handler,
    );

    unsafe {
        fe_rtos::task::add_task(
            fe_rtos::task::DEFAULT_STACK_SIZE,
            hello_world,
            Some(Box::new(uart0)),
        );
        fe_rtos::task::add_task(fe_rtos::task::DEFAULT_STACK_SIZE, counter_task, None);
    }

    let reload_val: u32 = CPU_FREQ / 10000;

    // Start the FeRTOS scheduler
    fe_rtos::task::start_scheduler(cortex_m::peripheral::SCB::set_pendsv, systick, reload_val);

    loop {}
}
