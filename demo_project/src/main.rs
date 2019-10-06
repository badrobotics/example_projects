#![no_std]
#![no_main]

extern crate atomic_queue;

use core::fmt::Write;

use rust_tm4c::tm4c_peripherals::get_peripherals;
use rust_tm4c::gpio;
use rust_tm4c::system_control;
use rust_tm4c::uart;
use rust_tm4c::timer;
use rust_tm4c::interrupt;

use atomic_queue::AtomicQueue;

const CPU_FREQ: u32  = 120_000_000;
const XTAL_FREQ: u32 = 25_000_000;

#[no_mangle]
pub fn main() -> ! {
    let mut p = get_peripherals();
    let scb = p.take_scb().unwrap();
    let nvic = p.take_nvic().unwrap();
    let sysctl = p.take_system_control().unwrap();
    let gpion = p.take_gpion().unwrap();
    let gpioa = p.take_gpioa().unwrap();
    let mut uart0 = p.take_uart0().unwrap();
    let timer0 = p.take_timer0().unwrap();

    // Configure the CPU for the maximum operating frequency
    let cpu_freq = sysctl.tm4c129_config_sysclk(CPU_FREQ, XTAL_FREQ);

    // Set up LEDs
    sysctl.enable_gpio_clock(system_control::GpioPort::GpioN);
    gpion.configure_as_output(gpio::Pin::Pin0);
    gpion.configure_as_output(gpio::Pin::Pin1);
    unsafe { GPIO_BLOCK = Some(gpion); }

    // Set up the debug UART
    sysctl.enable_gpio_clock(system_control::GpioPort::GpioA);
    sysctl.enable_uart_clock(system_control::Uart::Uart0);
    gpioa.select_alternate_function(gpio::Pin::Pin0, 1);
    gpioa.select_alternate_function(gpio::Pin::Pin1, 1);
    let _baud = uart0
        .configure(
            CPU_FREQ,
            115200,
            uart::Parity::None,
            uart::StopBits::One,
            uart::WordLength::Eight,
        )
        .unwrap();
    let mut uart_driver = uart::drivers::UartBlockingDriver::new(&mut uart0, uart::drivers::NewlineMode::CRLF);

    // Set up the timer
    sysctl.enable_timer_clock(system_control::Timer::Timer0);
    scb.int_register(interrupt::IntType::Timer0A, timer0a_handler);
    match timer0.set_periodic_mode_32bit(cpu_freq / 2) {
        Err(_) => panic!("Can't call this func?"),
        _ => {},
    };
    match timer0.enable_timeout_interrupt_32bit() {
        Err(_) => panic!("Can't call this func?"),
        _ => {},
    };
    unsafe { TIMER_BLOCK = Some(timer0); }
    nvic.clear_pending(interrupt::IntType::Timer0A);
    nvic.set_priority(interrupt::IntType::Timer0A, 0);
    nvic.enable(interrupt::IntType::Timer0A);

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

    let mut counter = 0_u8;
    loop {
        writeln!(uart_driver, "Hello, world! counter={} two_values_ago={}", counter, queue.pop().unwrap()).unwrap();
        match queue.push(counter) {
            Err(_) => panic!("No room to push?"),
            Ok(_) => {},
        }
        counter = counter.wrapping_add(1);
    }
}

static mut GPIO_BLOCK: Option<&'static mut gpio::Gpio> = None;
static mut TIMER_BLOCK: Option<&'static mut timer::Timer> = None;
pub unsafe extern "C" fn timer0a_handler() {
    if let Some(t) = &mut TIMER_BLOCK {
        match t.clear_timeout_interrupt_32bit() {
            _ => {},
        }
    }

    if let Some(g) = &mut GPIO_BLOCK {
        if g.get(gpio::Pin::Pin0) == 0 {
            g.set_high(gpio::Pin::Pin0);
        } else {
            g.set_low(gpio::Pin::Pin0);
        }
    }
}
