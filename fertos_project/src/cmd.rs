extern crate alloc;
use crate::uart_server::{INPUT_QUEUE, OUTPUT_QUEUE};
use crossbeam_queue::PopError;
use alloc::vec::Vec;

fn print_promt() {
    let prompt = "\r\n>> ";
    for c in prompt.chars() {
        OUTPUT_QUEUE.push(c as u8);
    }
}

fn parse(_buffer: &Vec<char>) {

}

pub fn cmd(_: &mut u8) {
    let mut buffer: Vec<char> = Vec::new();
    let mut newline: bool = false;

    print_promt();

    loop {
        match INPUT_QUEUE.pop() {
            Ok(c) => {
                buffer.push(c as char);
                if (c as char == '\n')  || (c as char == '\r') {
                    newline = true;
                } else {
                    OUTPUT_QUEUE.push(c);
                    newline = false;
                }
            },
            Err(PopError) => {
                fe_osi::sleep(10);
            }
        };

        if newline {
            parse(&buffer);
            buffer.clear();
            print_promt();
            newline = false;
        }
    }
}