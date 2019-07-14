#![allow(non_snake_case)]

use volatile_register::{RW, RO, WO};

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

