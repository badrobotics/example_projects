use embedded_hal::serial::{Read as SerialRead, Write as SerialWrite};
use crossbeam_queue::{PopError, SegQueue};
use core::fmt::Write;

lazy_static! {
    pub static ref INPUT_QUEUE: SegQueue<u8> = SegQueue::new();
    pub static ref OUTPUT_QUEUE: SegQueue<u8> = SegQueue::new();
}

pub fn uart_transmit_server<T: SerialWrite<u8> + Write>(serial: &mut T) {
    loop {
        match OUTPUT_QUEUE.pop() {
            Ok(c) => {
                write!(serial, "{}", c as char).unwrap();
            }
            Err(PopError) => {
                fe_osi::sleep(10);
            }
        }
    }
}

pub fn uart_receive_server<T: SerialRead<u8>>(serial: &mut T) {
    loop {
        match serial.read() {
            Ok(c) => {
                INPUT_QUEUE.push(c);
            }
            Err(_) => {
                fe_osi::sleep(10);
            }
        };
    }
}