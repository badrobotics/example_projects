#![allow(non_snake_case)]

use volatile_register::{RW, RO, WO};

// UARTCTL bits
const UARTEN_BIT: u32 = 1<<0;

// UARTLCRH bits
const UARTLCRH_ALL_BITS:  u32 = 0xff;

// UARTFR bits
const BUSY_BIT: u32 = 1<<3;

#[repr(C)]
pub struct Uart {
    pub UARTDR: RW<u32>,
    pub UARTRSR_UARTECR: RW<u32>,
    Reserved0: [RO<u32>; 4],
    pub UARTFR: RO<u32>,
    Reserved1: [RO<u32>; 1],
    pub UARTILPR: RW<u32>,
    pub UARTIBRD: RW<u32>,
    pub UARTFBRD: RW<u32>,
    pub UARTLCRH: RW<u32>,
    pub UARTCTL: RW<u32>,
    pub UARTIFLS: RW<u32>,
    pub UARTIM: RW<u32>,
    pub UARTRIS: RO<u32>,
    pub UARTMIS: RO<u32>,
    pub UARTICR: WO<u32>,
    pub UARTDMACTL: RW<u32>,
    Reserved2: [RO<u32>; 22],
    pub UART9BITADDR: RW<u32>,
    pub UART9BITAMASK: RW<u32>,
    Reserved3: [RO<u32>; 965],
    pub UARTPP: RO<u32>,
    Reserved4: [RO<u32>; 1],
    pub UARTCC: RW<u32>,
    Reserved5: [RO<u32>; 1],
    pub UARTPeriphID4: RO<u32>,
    pub UARTPeriphID5: RO<u32>,
    pub UARTPeriphID6: RO<u32>,
    pub UARTPeriphID7: RO<u32>,
    pub UARTPeriphID0: RO<u32>,
    pub UARTPeriphID1: RO<u32>,
    pub UARTPeriphID2: RO<u32>,
    pub UARTPeriphID3: RO<u32>,
    pub UARTPCellID0: RO<u32>,
    pub UARTPCellID1: RO<u32>,
    pub UARTPCellID2: RO<u32>,
    pub UARTPCellID3: RO<u32>,
}

#[allow(dead_code)]
pub enum Parity {
    Even,
    Odd,
    None
}

impl Parity {
    fn bits(&self) -> u32 {
        match self {
           Parity::Even => 0x6,
           Parity::Odd  => 0x2,
           Parity::None => 0x0,
        }
    }
}

#[allow(dead_code)]
pub enum StopBits {
    One,
    Two,
}

impl StopBits {
    fn bits(&self) -> u32 {
        match self {
            StopBits::One => 0x0,
            StopBits::Two => 0x8,
        }
    }
}

#[allow(dead_code)]
pub enum WordLength {
    Five,
    Six,
    Seven,
    Eight,
}

impl WordLength {
    fn bits(&self) -> u32 {
        match self {
            WordLength::Five  => 0<<5,
            WordLength::Six   => 1<<5,
            WordLength::Seven => 2<<5,
            WordLength::Eight => 3<<5,
        }
    }
}

impl Uart {
    ///
    /// Configures a UART with the baud rate, parity, number of stop bits, and data length. This
    /// function always programs ClkDiv to 16.
    ///
    /// Returns the actual programmed baud rate.
    ///
    /// # Arguments
    /// * `sysclk` - The frequency of system clock. This is needed for correctly calculating the baud rate.
    /// * `baud` - The desired baud rate.
    /// * `parity` - One of the valid parity options in the Parity enum.
    /// * `stop_bits` - One of the valid stop bit options in the StopBits enum.
    /// * `word_length` - One of the valid data lengths in the WordLength enum.
    ///
    pub fn configure(&mut self, sysclk: u32, baud: u32, parity: Parity, stop_bits: StopBits, word_length: WordLength) -> Option<u32> {
        let clkdiv: u32 = 16;   // Always assume ClkDiv to be 16
        let fbrd: u32 = 0;      // Always program the fractional part of the baud rate divisor (UARTFBRD) to 0
        let ibrd: u32 = sysclk / (clkdiv*baud); // Find the value of the ibrd register.

        // Verify ibrd is valid
        if ibrd & 0xffff0000 > 0 {
            return None;
        }

        // Find the actual baud rate to be programmed.
        let real_baud: u32 = sysclk / (clkdiv*ibrd);

        unsafe {
            // Disable the UART
            self.UARTCTL.modify(|x| x & !UARTEN_BIT);
            // Configure the baud rate
            self.UARTIBRD.write(ibrd);
            self.UARTFBRD.write(fbrd);
            // Set line control parameters
            self.UARTLCRH.modify(|x| (x & !UARTLCRH_ALL_BITS) | parity.bits() | word_length.bits() | stop_bits.bits() );
            // Enable the UART
            self.UARTCTL.modify(|x| x | UARTEN_BIT);
        }

        Some(real_baud)
    }

    pub fn busy(&self) -> bool {
        self.UARTFR.read() & BUSY_BIT > 0
    }
}

pub mod drivers {
    use crate::uart::Uart;

    pub struct UartBlockingDriver<'a> {
        uart: &'a mut Uart,
    }

    impl<'a> UartBlockingDriver<'a> {
        pub fn new(uart: &'a mut Uart) -> UartBlockingDriver {
            UartBlockingDriver {
                uart
            }
        }

        pub fn putchar(&mut self, c: u8) {
            while self.uart.busy() {};
            unsafe {
                self.uart.UARTDR.write(c.into());
            }
        }
    }
}
