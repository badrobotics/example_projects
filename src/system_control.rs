#![allow(non_snake_case)]
#![allow(dead_code)]

use volatile_register::{RW, RO};

// MOSCCTL bits
const OSCRNG_BIT: u32 = 1<<4;
const PWRDN_BIT:  u32 = 1<<3;
const NOXTAL_BIT: u32 = 1<<2;
const MOSCIM_BIT: u32 = 1<<1;
const CVAL_BIT:   u32 = 1<<0;

// RIS and MISC bits
const MOSCPUPRIS_BIT: u32 = 1<<8;

// RSCLKCFG bits
const OSCSRC_BITS:        u32 = 0xf<<20;
const OSCSRC_MOSC_SELECT: u32 = 0x3<<20;
const PLLSRC_BITS:        u32 = 0xf<<24;
const PLLSRC_MOSC_SELECT: u32 = 0x3<<24;
const PSYSDIV_BITS:       u32 = 0x3ff;
const MEMTIMU_BIT:        u32 = 1<<31;
const NEWFREQ_BIT:        u32 = 1<<30;
const USEPLL_BIT:         u32 = 1<<28;

// PLLFREQ0 bits
const MDIV_BITS:  u32 = 0x000fffff;
const PLLPWR_BIT: u32 = 1<<23;

// PLLFREQ1 bits
const Q_BITS: u32 = 0x00001f00;
const N_BITS: u32 = 0x0000001f;

// PLLSTAT bits
const PLL_LOCK: u32 = 0x1;

// MEMTIM0 bits
const EEPROM_TIMING_BITS: u32 = 0x03ef_0000;
const FLASH_TIMING_BITS:  u32 = 0x0000_03ef;

#[allow(dead_code)]
pub enum GpioPort {
    GpioA,
    GpioB,
    GpioC,
    GpioD,
    GpioE,
    GpioF,
    GpioG,
    GpioH,
    GpioJ,
    GpioK,
    GpioL,
    GpioM,
    GpioN,
    GpioP,
    GpioQ,
}

#[repr(C)]
pub struct SystemControl {
    pub DID0: RO<u32>,
    pub DID1: RO<u32>,
    Reserved0: [RO<u32>; 12],
    pub PTBOCTL: RW<u32>,
    Reserved1: [RO<u32>; 5],
    pub RIS: RO<u32>,
    pub IMC: RW<u32>,
    pub MISC: RW<u32>,
    pub RESC: RW<u32>,
    pub PWRTC: RW<u32>,
    pub NMIC: RW<u32>,
    Reserved2: [RO<u32>; 5],
    pub MOSCCTL: RW<u32>,
    Reserved3: [RO<u32>; 12],
    pub RSCLKCFG: RW<u32>,
    Reserved4: [RO<u32>; 3],
    pub MEMTIM0: RW<u32>,
    Reserved5: [RO<u32>; 29],
    pub ALTCLKCFG: RW<u32>,
    Reserved6: [RO<u32>; 2],
    pub DSCLKCFG: RW<u32>,
    pub DIVSCLK: RW<u32>,
    pub SYSPROP: RO<u32>,
    pub PIOSCCAL: RW<u32>,
    pub PIOSCSTAT: RO<u32>,
    Reserved7: [RO<u32>; 2],
    pub PLLFREQ0: RW<u32>,
    pub PLLFREQ1: RW<u32>,
    pub PLLSTAT: RO<u32>,
    Reserved8: [RO<u32>; 7],
    pub SLPPWRCFG: RW<u32>,
    pub DSLPPWRCFG: RW<u32>,
    Reserved9: [RO<u32>; 4],
    pub NVMSTAT: RO<u32>,
    Reserved10: [RO<u32>; 4],
    pub LDOSPCTL: RW<u32>,
    pub LDOSPCAL: RO<u32>,
    pub LDODPCTL: RW<u32>,
    pub LDODPCAL: RO<u32>,
    Reserved11: [RO<u32>; 2],
    pub SDPMST: RO<u32>,
    Reserved12: [RO<u32>; 2],
    pub RESBEHAVCTL: RW<u32>,
    Reserved13: [RO<u32>; 6],
    pub HSSR: RW<u32>,
    Reserved14: [RO<u32>; 34],
    pub USBPDS: RO<u32>,
    pub USBMPC: RW<u32>,
    pub EMACPDS: RO<u32>,
    pub EMACMPC: RW<u32>,
    Reserved15: [RO<u32>; 2],
    pub CAN0PDS: RO<u32>,
    pub CAN0MPC: RW<u32>,
    pub CAN1PDS: RO<u32>,
    pub CAN1MPC: RW<u32>,
    Reserved16: [RO<u32>; 22],
    pub PPWD: RO<u32>,
    pub PPTIMER: RO<u32>,
    pub PPGPIO: RO<u32>,
    pub PPDMA: RO<u32>,
    pub PPEPI: RO<u32>,
    pub PPHIB: RO<u32>,
    pub PPUART: RO<u32>,
    pub PPSSI: RO<u32>,
    pub PPI2C: RO<u32>,
    Reserved17: [RO<u32>; 1],
    pub PPUSB: RO<u32>,
    Reserved18: [RO<u32>; 1],
    pub PPEPHY: RO<u32>,
    pub PPCAN: RO<u32>,
    pub PPADC: RO<u32>,
    pub PPACMP: RO<u32>,
    pub PPPWM: RO<u32>,
    pub PPQEI: RO<u32>,
    pub PPLPC: RO<u32>,
    Reserved19: [RO<u32>; 1],
    pub PPPECI: RO<u32>,
    pub PPFAN: RO<u32>,
    pub PPEEPROM: RO<u32>,
    pub PPWTIMER: RO<u32>,
    Reserved20: [RO<u32>; 4],
    pub PPRTS: RO<u32>,
    pub PPCCM: RO<u32>,
    Reserved21: [RO<u32>; 6],
    pub PPLCD: RO<u32>,
    Reserved22: [RO<u32>; 1],
    pub PPOWIRE: RO<u32>,
    pub PPEMAC: RO<u32>,
    pub PPPRB: RO<u32>,
    pub PPHIM: RO<u32>,
    Reserved23: [RO<u32>; 86],
    pub SRWD: RW<u32>,
    pub SRTIMER: RW<u32>,
    pub SRGPIO: RW<u32>,
    pub SRDMA: RW<u32>,
    pub SREPI: RW<u32>,
    pub SRHIB: RW<u32>,
    pub SRUART: RW<u32>,
    pub SRSSI: RW<u32>,
    pub SRI2C: RW<u32>,
    Reserved24: [RO<u32>; 1],
    pub SRUSB: RW<u32>,
    Reserved25: [RO<u32>; 1],
    pub SREPHY: RW<u32>,
    pub SRCAN: RW<u32>,
    pub SRADC: RW<u32>,
    pub SRACMP: RW<u32>,
    pub SRPWM: RW<u32>,
    pub SRQEI: RW<u32>,
    Reserved26: [RO<u32>; 4],
    pub SREEPROM: RW<u32>,
    Reserved27: [RO<u32>; 6],
    pub SRCCM: RW<u32>,
    Reserved28: [RO<u32>; 9],
    pub SREMAC: RW<u32>,
    Reserved29: [RO<u32>; 24],
    pub RCGCWD: RW<u32>,
    pub RCGCTIMER: RW<u32>,
    pub RCGCGPIO: RW<u32>,
    pub RCGCDMA: RW<u32>,
    pub RCGCEPI: RW<u32>,
    pub RCGCHIB: RW<u32>,
    pub RCGCUART: RW<u32>,
    pub RCGCSSI: RW<u32>,
    pub RCGCI2C: RW<u32>,
    Reserved30: [RO<u32>; 1],
    pub RCGCUSB: RW<u32>,
    Reserved31: [RO<u32>; 1],
    pub RCGCEPHY: RW<u32>,
    pub RCGCCAN: RW<u32>,
    pub RCGCADC: RW<u32>,
    pub RCGCACMP: RW<u32>,
    pub RCGCPWM: RW<u32>,
    pub RCGCQEI: RW<u32>,
    Reserved32: [RO<u32>; 4],
    pub RCGCEEPROM: RW<u32>,
    Reserved33: [RO<u32>; 6],
    pub RCGCCCM: RW<u32>,
    Reserved34: [RO<u32>; 9],
    pub RCGCEMAC: RW<u32>,
    Reserved35: [RO<u32>; 24],
    pub SCGCWD: RW<u32>,
    pub SCGCTIMER: RW<u32>,
    pub SCGCGPIO: RW<u32>,
    pub SCGCDMA: RW<u32>,
    pub SCGCEPI: RW<u32>,
    pub SCGCHIB: RW<u32>,
    pub SCGCUART: RW<u32>,
    pub SCGCSSI: RW<u32>,
    pub SCGCI2C: RW<u32>,
    Reserved36: [RO<u32>; 1],
    pub SCGCUSB: RW<u32>,
    Reserved37: [RO<u32>; 1],
    pub SCGCEPHY: RW<u32>,
    pub SCGCCAN: RW<u32>,
    pub SCGCADC: RW<u32>,
    pub SCGCACMP: RW<u32>,
    pub SCGCPWM: RW<u32>,
    pub SCGCQEI: RW<u32>,
    Reserved38: [RO<u32>; 4],
    pub SCGCEEPROM: RW<u32>,
    Reserved39: [RO<u32>; 6],
    pub SCGCCCM: RW<u32>,
    Reserved40: [RO<u32>; 9],
    pub SCGCEMAC: RW<u32>,
    Reserved41: [RO<u32>; 24],
    pub DCGCWD: RW<u32>,
    pub DCGCTIMER: RW<u32>,
    pub DCGCGPIO: RW<u32>,
    pub DCGCDMA: RW<u32>,
    pub DCGCEPI: RW<u32>,
    pub DCGCHIB: RW<u32>,
    pub DCGCUART: RW<u32>,
    pub DCGCSSI: RW<u32>,
    pub DCGCI2C: RW<u32>,
    Reserved42: [RO<u32>; 1],
    pub DCGCUSB: RW<u32>,
    Reserved43: [RO<u32>; 1],
    pub DCGCEPHY: RW<u32>,
    pub DCGCCAN: RW<u32>,
    pub DCGCADC: RW<u32>,
    pub DCGCACMP: RW<u32>,
    pub DCGCPWM: RW<u32>,
    pub DCGCQEI: RW<u32>,
    Reserved44: [RO<u32>; 4],
    pub DCGCEEPROM: RW<u32>,
    Reserved45: [RO<u32>; 6],
    pub DCGCCCM: RW<u32>,
    Reserved46: [RO<u32>; 9],
    pub DCGCEMAC: RW<u32>,
    Reserved47: [RO<u32>; 24],
    pub PCWD: RW<u32>,
    pub PCTIMER: RW<u32>,
    pub PCGPIO: RW<u32>,
    pub PCDMA: RW<u32>,
    pub PCEPI: RW<u32>,
    pub PCHIB: RW<u32>,
    pub PCUART: RW<u32>,
    pub PCSSI: RW<u32>,
    pub PCI2C: RW<u32>,
    Reserved48: [RO<u32>; 1],
    pub PCUSB: RW<u32>,
    Reserved49: [RO<u32>; 1],
    pub PCEPHY: RW<u32>,
    pub PCCAN: RW<u32>,
    pub PCADC: RW<u32>,
    pub PCACMP: RW<u32>,
    pub PCPWM: RW<u32>,
    pub PCQEI: RW<u32>,
    Reserved50: [RO<u32>; 4],
    pub PCEEPROM: RW<u32>,
    Reserved51: [RO<u32>; 6],
    pub PCCCM: RW<u32>,
    Reserved52: [RO<u32>; 9],
    pub PCEMAC: RW<u32>,
    Reserved53: [RO<u32>; 24],
    pub PRWD: RO<u32>,
    pub PRTIMER: RO<u32>,
    pub PRGPIO: RO<u32>,
    pub PRDMA: RO<u32>,
    pub PREPI: RO<u32>,
    pub PRHIB: RO<u32>,
    pub PRUART: RO<u32>,
    pub PRSSI: RO<u32>,
    pub PRI2C: RO<u32>,
    Reserved54: [RO<u32>; 1],
    pub PRUSB: RO<u32>,
    Reserved55: [RO<u32>; 1],
    pub PREPHY: RO<u32>,
    pub PRCAN: RO<u32>,
    pub PRADC: RO<u32>,
    pub PRACMP: RO<u32>,
    pub PRPWM: RO<u32>,
    pub PRQEI: RO<u32>,
    Reserved56: [RO<u32>; 4],
    pub PREEPROM: RO<u32>,
    Reserved57: [RO<u32>; 6],
    pub PRCCM: RO<u32>,
    Reserved58: [RO<u32>; 9],
    pub PREMAC: RO<u32>,
    Reserved59: [RO<u32>; 288],
    pub UNIQUEID0: RO<u32>,
    pub UNIQUEID1: RO<u32>,
    pub UNIQUEID2: RO<u32>,
    pub UNIQUEID3: RO<u32>,
}

impl SystemControl {
    /// Sets the main oscillator to the external crystal oscillator, and then boosts the main
    /// oscillator up to the desired system clock frequency using the PLL. This function also sets
    /// the new memory timings. This function should only ever be called on a tm4c129 chip, and
    /// only once after a power-on reset. Currently this function only supports using clock
    /// frequencies which are multiples of 5 Mhz, though the hardware does allow more fine grained
    /// control.
    ///
    /// Returns 0 if the parameters were invalid, and otherwise returns the actual programmed clock
    /// frequency.
    ///
    /// # Arguments
    ///
    /// * `cpu_clock_freq` - The desired system clock frequency in Hertz
    /// * `xtal_freq` - The frequency of the external crystal oscillator in Hertz
    pub fn tm4c129_config_sysclk(&mut self, cpu_clock_freq: u32, xtal_freq: u32) -> u32 {
        // First find the Q, N, MINT, and MFRAC values and determine that they're valid. These
        // determine the final CPU clock frequency.

        // These Q and N values divide the input crystal frequency down to F_in
        let mut F_in: u32 = 5_000_000;
        let Q: u32 = xtal_freq/F_in - 1;
        let N: u32 = 0;
        if (Q<<8) & !Q_BITS > 0 || N & !N_BITS > 0 {
            return 0;
        }

        // Recompute F_in in case the external oscillator frequency isn't a multiple of the
        // original F_in value.
        F_in = xtal_freq / ((Q+1) * (N+1));

        // Multiply up F_in to the final PLL frequency of 480 MHz using the MINT and MFRAC fields.
        // MFRAC remains 0 to reduce jitter (see reference manual).
        let mut pll_freq: u32 = 480_000_000;
        let MINT:  u32 = pll_freq / F_in;
        let MFRAC: u32 = 0;

        // Calculate the actual PLL frequency being programmed. This might not equal the requested
        // frequency since this function only supports system clock frequencies that are multiples
        // of F_in.
        pll_freq = F_in * (MINT + MFRAC/1024);

        // Find by how much to divide down the PLL output
        let pll_freq_div: u32 = pll_freq / cpu_clock_freq;

        // Find the actual CPU clock frequency in case it wasn't an even factor of the PLL
        // frequency.
        let actual_cpu_freq: u32 = pll_freq / pll_freq_div;

        // From the actual frequency, find the memory timing parameters to program. See table 5-12
        // in the reference manual.
        let FBCHT_EBCHT;
        let FBCE_EBCE;
        let FWS_EWS;
        match actual_cpu_freq {
            0           ..= 16_000_000  => {FBCHT_EBCHT=0x0; FBCE_EBCE=1; FWS_EWS=0x0;},
            16_000_001  ..= 40_000_000  => {FBCHT_EBCHT=0x2; FBCE_EBCE=0; FWS_EWS=0x1;},
            40_000_001  ..= 60_000_000  => {FBCHT_EBCHT=0x3; FBCE_EBCE=0; FWS_EWS=0x2;},
            60_000_001  ..= 80_000_001  => {FBCHT_EBCHT=0x4; FBCE_EBCE=0; FWS_EWS=0x3;},
            80_000_001  ..= 100_000_000 => {FBCHT_EBCHT=0x5; FBCE_EBCE=0; FWS_EWS=0x4;},
            100_000_001 ..= 120_000_000 => {FBCHT_EBCHT=0x6; FBCE_EBCE=0; FWS_EWS=0x5;},
            _ => {return 0;}, // Other configurations are invalid
        }

        // Set the high frequency mode if the external crystal is faster than 10 MHz.
        if xtal_freq >= 10_000_000 {
            unsafe { self.MOSCCTL.modify(|x| x | OSCRNG_BIT) };
        } else {
            unsafe { self.MOSCCTL.modify(|x| x & !OSCRNG_BIT) };
        }

        // Clear the main oscillator ready bit, enable the external crystal, power on the main
        // oscillator, and then wait for it to become ready.
        unsafe {
            self.MISC.write(MOSCPUPRIS_BIT);
            self.MOSCCTL.modify(|x| x & !NOXTAL_BIT);
            self.MOSCCTL.modify(|x| x & !PWRDN_BIT);
        }
        while self.RIS.read() & MOSCPUPRIS_BIT == 0 {};

        // Set the PLL source to the main oscillator.
        unsafe {
            self.RSCLKCFG.modify(|x| (x & !PLLSRC_BITS) | PLLSRC_MOSC_SELECT);
        }

        // Program PLL settings to set the new clock frequency
        unsafe {
            self.PLLFREQ0.modify(|x| (x & !MDIV_BITS) | (MFRAC<<10) | (MINT) | PLLPWR_BIT);
            self.PLLFREQ1.modify(|x| (x & !(Q_BITS|N_BITS) | (Q<<8) | (N)));
            self.RSCLKCFG.modify(|x| x | NEWFREQ_BIT);
        }

        // Wait for the PLL to lock to the new frequency. If this bit never gets set there was some
        // error in the PLL configuration.
        while self.PLLSTAT.read() & PLL_LOCK == 0 {};

        // Set the memory timing parameters for the new clock frequency
        unsafe {
            // EEPROM timings
            self.MEMTIM0.modify(|x| (x & !EEPROM_TIMING_BITS) | (FBCHT_EBCHT<<22) | (FBCE_EBCE<<21) | (FWS_EWS<<16));
            // Flash timings
            self.MEMTIM0.modify(|x| (x & !FLASH_TIMING_BITS) | (FBCHT_EBCHT<<6) | (FBCE_EBCE<<5) | FWS_EWS);
        }

        // When using the PLL as the clock source, divide by pll_freq_div
        unsafe {
            self.RSCLKCFG.modify(|x| (x & !PSYSDIV_BITS) | pll_freq_div);
        }

        // Tell the system to use the PLL and commit to the new memory timing values
        unsafe {
            self.RSCLKCFG.modify(|x| x | USEPLL_BIT | MEMTIMU_BIT);
        }

        // Return the new clock frequency
        actual_cpu_freq
    }

    pub fn enable_gpio_clock(&mut self, port: GpioPort) {
        let port_bitmask: u32 = match port {
            GpioPort::GpioA => 1<<0,
            GpioPort::GpioB => 1<<1,
            GpioPort::GpioC => 1<<2,
            GpioPort::GpioD => 1<<3,
            GpioPort::GpioE => 1<<4,
            GpioPort::GpioF => 1<<5,
            GpioPort::GpioG => 1<<6,
            GpioPort::GpioH => 1<<7,
            GpioPort::GpioJ => 1<<8,
            GpioPort::GpioK => 1<<9,
            GpioPort::GpioL => 1<<10,
            GpioPort::GpioM => 1<<11,
            GpioPort::GpioN => 1<<12,
            GpioPort::GpioP => 1<<13,
            GpioPort::GpioQ => 1<<14,
        };

        unsafe {
            self.RCGCGPIO.modify(|x| x | port_bitmask)
        }
    }
}
