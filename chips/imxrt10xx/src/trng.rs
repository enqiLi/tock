#![allow(non_snake_case)]

use kernel::debug;
use kernel::hil;
use kernel::hil::entropy::Continue;
use kernel::utilities::cells::OptionalCell;
use kernel::utilities::registers::interfaces::{ReadWriteable, Readable};
use kernel::utilities::registers::register_bitfields;
use kernel::utilities::registers::register_structs;
use kernel::utilities::registers::{ReadOnly, ReadWrite};
use kernel::utilities::StaticRef;
use kernel::ErrorCode;

register_structs! {
    TRNGRegisters {
    // Miscellaneous control
    (0x000 => TRNG0_MCTL: ReadWrite<u32, Control::Register>),

        // Statistical check miscellaneous
    (0x004 => TRNG0_SCMISC: ReadWrite<u32, Check::Register>),
    // Poker range
    // (0x008 => TRNG0_PKRRNG: ReadWrite<u32, PokerRange::Register>),
    // Poker maximum limit
    // (0x00C => TRNG0_PKRMAX: ReadWrite<u32, PokerMax::Register>),
    // Poker square calculation result
    // (0x00C => TRNG0_PKRSQ: ReadOnly<u32, PokerSquare::Register>),
    // Seed control
    // (0x010 => TRNG0_SDCTL: ReadWrite<u32, SeedControl::Register>),
    // Sparse bit limit
    // (0x014 => TRNG0_SBLIM: ReadWrite<u32, BitLimit::Register>),
    // Total Samples
    // (0x014 => TRNG0_TOTSAM: ReadOnly<u32, TotalSamples::Register>),
    // Frequency Count Minimum Limit
    // (0x018 => TRNG0_FRQMIN: ReadWrite<u32, FrequencyCountMinLim::Register>),
    // Frequency Count Maximum Limit
    // (0x01C => TRNG0_FRQMAX: ReadWrite<u32, FrequencyCountMaxLim::Register>),
    // Frequency Count
    // (0x01C => TRNG0_FRQCNT: ReadWrite<u32, FrequencyCount::Register>),
    // Statistical Check Monobit Count
    // (0x020 => TRNG0_SCMC: ReadOnly<u32, MonobitCount::Register>),
    // Statistical Check Monobit Limit
    // (0x020 => TRNG0_SCML: ReadWrite<u32, MonobitLimit::Register>),
    // Statistical Check Run Length 1 Limit
    // (0x024 => TRNG0_SCR1L: ReadWrite<u32, CheckRunLen1Lim::Register>),
    // Statistical Check Run Length 1 Count
    // (0x024 => TRNG0_SCR1C: ReadOnly<u32, CheckRunLen1Count::Register>),
    // Statistical Check Run Length 2 Limit
    // (0x028 => TRNG0_SCR2L: ReadWrite<u32, CheckRunLen2Lim::Register>),
    // Statistical Check Run Length 2 Count
    // (0x028 => TRNG0_SCR2C: ReadOnly<u32, CheckRunLen2Count::Register>),
    // Statistical Check Run Length 3 Count
    // (0x02C => TRNG0_SCR3C: ReadOnly<u32, CheckRunLen3Count::Register>),
    // Statistical Check Run Length 3 Limit
    // (0x02C => TRNG0_SCR3L: ReadWrite<u32, CheckRunLen3Lim::Register>),
    // Statistical Check Run Length 4 Count
    // (0x030 => TRNG0_SCR4C: ReadOnly<u32, CheckRunLen4Count::Register>),
    // Statistical Check Run Length 4 Limit
    // (0x030 => TRNG0_SCR4L: ReadWrite<u32, CheckRunLen4Lim::Register>),
    // Statistical Check Run Length 5 Limit
    // (0x034 => TRNG0_SCR5L: ReadWrite<u32, CheckRunLen5Lim::Register>),
    // Statistical Check Run Length 5 Count
    // (0x034 => TRNG0_SCR5C: ReadOnly<u32, CheckRunLen5Count::Register>),
    // Statistical Check Run Length 6+ Limit
    // (0x038 => TRNG0_SCR6PL: ReadWrite<u32, CheckRunLen6PLim::Register>),
    // Statistical Check Run Length 6 Count
    // (0x038 => TRNG0_SCR6PC: ReadOnly<u32, CheckRunLen6PCount::Register>),
    (0x008 => _reserved0),
    // Status
    (0x03C => TRNG0_STATUS: ReadOnly<u32, Status::Register>),
    // Entropy Read Registers
    (0x040 => TRNG0_ENT: [ReadOnly<u32, EntropyRead::Register>; 16]),
    // // Statistical Check Poker Count 1 and 0
    // (0x080 => TRNG0_PKRCNT10: ReadOnly<u32, PokerCount10::Register>),
    // // Statistical Check Poker Count 3 and 2
    // (0x084 => TRNG0_PKRCNT32: ReadOnly<u32, PokerCount32::Register>),
    // // Statistical Check Poker Count 5 and 4
    // (0x088 => TRNG0_PKRCNT54: ReadOnly<u32, PokerCount54::Register>),
    // // Statistical Check Poker Count 7 and 6
    // (0x08C => TRNG0_PKRCNT76: ReadOnly<u32, PokerCount76::Register>),
    // // Statistical Check Poker Count 9 and 8
    // (0x090 => TRNG0_PKRCNT98: ReadOnly<u32, PokerCount98::Register>),
    // // Statistical Check Poker Count B and A
    // (0x094 => TRNG0_PKRCNTBA: ReadOnly<u32, PokerCountBA::Register>),
    // // Statistical Check Poker Count D and C
    // (0x098 => TRNG0_PKRCNTDC: ReadOnly<u32, PokerCountDC::Register>),
    // // Statistical Check Poker Count F and E
    // (0x09C => TRNG0_PKRCNTFE: ReadOnly<u32, PokerCountFE::Register>),

    (0x080 => _reserved1),
    // Security Configuration
    // (0x0B0 => TRNG0_SEC_CFG: ReadWrite<u32, SecurityConfig::Register>),
    // Interrupt Control
    (0x0B4 => TRNG0_INT_CTRL: ReadWrite<u32, InterruptControl::Register>),
    // Mask
    (0x0B8 => TRNG0_INT_MASK: ReadWrite<u32, InterruptMask::Register>),
    // Interrupt Status
    (0x0BC => TRNG0_INT_STATUS: ReadWrite<u32, InterruptStatus::Register>),
    // Version ID (MS)
    // (0x0F0 => TRNG0_VID1: ReadOnly<u32, VID1::Register>),
    // // Version ID (LS)
    // (0x0F4 => TRNG0_VID2: ReadOnly<u32, VID2::Register>),
    (0x0C0 => _reserved2),
        (0x100 => @END),

    }
}

register_bitfields! {
    u32,

    Control [
        PRGM OFFSET(16) NUMBITS(1) [
        ProgramMode = 1,
        RunMode = 0,
    ],
    TSTOP_OK OFFSET(13) NUMBITS(1) [
            OK = 1,
    ],
    ERR OFFSET(12) NUMBITS(1) [],
    TST_OUT OFFSET(11) NUMBITS(1) [],
    ENT_VAL OFFSET(10) NUMBITS(1) [],
    FCT_VAL OFFSET(9) NUMBITS(1) [],
    FCT_FAIL OFFSET(8) NUMBITS(1) [],
    FOR_SCLK OFFSET(7) NUMBITS(1) [],
    RST_DEF OFFSET(6) NUMBITS(1) [],
    TRNG_ACC OFFSET(5) NUMBITS(1) [
            Acc = 1,
            Deny = 0,

    ],
    UNUSED OFFSET(4) NUMBITS(1) [],
    OSC_DIV OFFSET(2) NUMBITS(2) [],
    SAMP_MODE OFFSET(0) NUMBITS(2) [ VON = 2,],
    ],

    Check [
        RTY_CT OFFSET(16) NUMBITS(4) [],
    LRUN_MAX OFFSET(0) NUMBITS(8) [],
    ],

    // PokerRange [
    //     PKR_RNG OFFSET(0) NUMBITS(16) [],
    // ],

    // PokerMax [
    //     PKR_MAX OFFSET(0) NUMBITS(24) [],
    // ],

    // PokerSquare [
    //     PKR_SQ OFFSET(0) NUMBITS(24) [],
    // ],

    // SeedControl [
    //     ENT_DLY OFFSET(16) NUMBITS(16) [],
    // SAMP_SIZE OFFSET(0) NUMBITS(16) [],
    // ],

    // BitLimit [
    //     SB_LIM OFFSET(0) NUMBITS(10) [],
    // ],

    // TotalSamples [
    //     TOT_SAMP OFFSET(0) NUMBITS(20) [],
    // ],

    // FrequencyCountMinLim [
    //     FRQ_MIN OFFSET(0) NUMBITS(22) [],
    // ],

    // FrequencyCount [
    //     FRQ_CNT OFFSET(0) NUMBITS(22) [],
    // ],

    // FrequencyCountMaxLim [
    //     FRQ_MAX OFFSET(0) NUMBITS(22) [],
    // ],

    // MonobitCount [
    //     MONO_CT OFFSET(0) NUMBITS(16) [],
    // ],

    // MonobitLimit [
    //     MONO_RNG OFFSET(16) NUMBITS(16) [],
    // MONO_MAX OFFSET(0) NUMBITS(16) [],
    // ],

    // CheckRunLen1Count [
    //     R1_1_CT OFFSET(16) NUMBITS(15) [],
    // R1_0_CT OFFSET(0) NUMBITS(15) [],
    // ],

    // CheckRunLen1Lim [
    //     R1_RNG OFFSET(16) NUMBITS(15) [],
    // R1_MAX OFFSET(0) NUMBITS(15) [],
    // ],

    // CheckRunLen2Count [
    //     R2_1_CT OFFSET(16) NUMBITS(14) [],
    // R2_0_CT OFFSET(0) NUMBITS(14) [],
    // ],

    // CheckRunLen2Lim [
    //     R2_RNG OFFSET(16) NUMBITS(14) [],
    // R2_MAX OFFSET(0) NUMBITS(14) [],
    // ],

    // CheckRunLen3Count [
    //     R3_1_CT OFFSET(16) NUMBITS(13) [],
    // R3_0_CT OFFSET(0) NUMBITS(13) [],
    // ],

    // CheckRunLen3Lim [
    //     R3_RNG OFFSET(16) NUMBITS(13) [],
    // R3_MAX OFFSET(0) NUMBITS(13) [],
    // ],

    // CheckRunLen4Count [
    //     R4_1_CT OFFSET(16) NUMBITS(12) [],
    // R4_0_CT OFFSET(0) NUMBITS(12) [],
    // ],

    // CheckRunLen4Lim [
    //     R4_RNG OFFSET(16) NUMBITS(12) [],
    // R4_MAX OFFSET(0) NUMBITS(12) [],
    // ],

    // CheckRunLen5Count [
    //     R5_1_CT OFFSET(16) NUMBITS(11) [],
    // R5_0_CT OFFSET(0) NUMBITS(11) [],
    // ],

    // CheckRunLen5Lim [
    //     R5_RNG OFFSET(16) NUMBITS(11) [],
    // R5_MAX OFFSET(0) NUMBITS(11) [],
    // ],

    // CheckRunLen6PCount [
    //     R6p_1_CT OFFSET(16) NUMBITS(10) [],
    // R6P_0_CT OFFSET(0) NUMBITS(10) [],
    // ],

    // CheckRunLen6PLim [
    //     R6P_RNG OFFSET(16) NUMBITS(10) [],
    // R6P_MAX OFFSET(0) NUMBITS(10) [],
    // ],

    Status [
        RETRY_CT OFFSET(16) NUMBITS(4) [],
    TFMB OFFSET(15) NUMBITS(1) [
            Fail = 1
        ],
    TFP OFFSET(14) NUMBITS(1) [
            Fail = 1
        ],
    TFLR OFFSET(13) NUMBITS(1) [
            Fail = 1
        ],
    TFSB OFFSET(12) NUMBITS(1) [
            Fail = 1
        ],
    TF6PBR1 OFFSET(11) NUMBITS(1) [
            Fail = 1
        ],
    TF6PBR0 OFFSET(10) NUMBITS(1) [
            Fail = 1
        ],
    TF5BR1 OFFSET(9) NUMBITS(1) [
            Fail = 1
        ],
    TF5BR0 OFFSET(8) NUMBITS(1) [
            Fail = 1
        ],
    TF4BR1 OFFSET(7) NUMBITS(1) [
            Fail = 1
        ],
    TF4BR0 OFFSET(6) NUMBITS(1) [
            Fail = 1
        ],
    TF3BR1 OFFSET(5) NUMBITS(1) [
            Fail = 1
        ],
    TF3BR0 OFFSET(4) NUMBITS(1) [
            Fail = 1
        ],
    TF2BR1 OFFSET(3) NUMBITS(1) [
            Fail = 1
        ],
    TF2BR0 OFFSET(2) NUMBITS(1) [
            Fail = 1
        ],
    TF1BR1 OFFSET(1) NUMBITS(1) [
            Fail = 1,
        ],
    TF1BR0 OFFSET(0) NUMBITS(1) [
            Fail = 1,
        ]
    ],

    EntropyRead [
        ENT OFFSET(0) NUMBITS(32) [],
    ],

    // PokerCount10 [
    //     PKR_1_CT OFFSET(16) NUMBITS(16) [],
    // PKR_0_CT OFFSET(0) NUMBITS(16) [],
    // ],

    // PokerCount32 [
    //     PKR_3_CT OFFSET(16) NUMBITS(16) [],
    // PKR_2_CT OFFSET(0) NUMBITS(16) [],
    // ],

    // PokerCount54 [
    //     PKR_5_CT OFFSET(16) NUMBITS(16) [],
    // PKR_4_CT OFFSET(0) NUMBITS(16) [],
    // ],

    // PokerCount76 [
    //     PKR_7_CT OFFSET(16) NUMBITS(16) [],
    // 	PKR_6_CT OFFSET(0) NUMBITS(16) [],
    // ],

    // PokerCount98 [
    //     PKR_9_CT OFFSET(16) NUMBITS(16) [],
    // 	PKR_8_CT OFFSET(0) NUMBITS(16) [],
    // ],

    // PokerCountBA [
    //     PKR_B_CT OFFSET(16) NUMBITS(16) [],
    // 	PKR_A_CT OFFSET(0) NUMBITS(16) [],
    // ],

    // PokerCountDC [
    //     PKR_D_CT OFFSET(16) NUMBITS(16) [],
    // 	PKR_C_CT OFFSET(0) NUMBITS(16) [],
    // ],

    // PokerCountFE [
    //     PKR_F_CT OFFSET(16) NUMBITS(16) [],
    // 	PKR_E_CT OFFSET(0) NUMBITS(16) [],
    // ],

    // SecurityConfig [
    //     SK_VAL OFFSET(2) NUMBITS(1) [],
    // 	NO_PRGM OFFSET(1) NUMBITS(1) [
    //         Overide = 1,
    //         Control = 0,
    //     ],
    // 	SH0 OFFSET(0) NUMBITS(1) [],
    // ],

    InterruptControl [
        UNUSED OFFSET(3) NUMBITS(29) [],
    FRQ_CT_FAIL OFFSET(2) NUMBITS(1) [],
    ENT_VAL OFFSET(1) NUMBITS(1) [
        Invalid = 0,
        Ready = 1,
    ],
    HW_ERR OFFSET(0) NUMBITS(1) [
            Clear = 0,
            Active = 1
        ]
    ],

    InterruptMask [
    FRQ_CT_FAIL OFFSET(2) NUMBITS(1) [],
    ENT_VAL OFFSET(1) NUMBITS(1) [
        Invalid = 0,
        Ready = 1,
    ],
    HW_ERR OFFSET(0) NUMBITS(1) [
            Masked = 0,
            Active = 1
        ]
    ],

    InterruptStatus [
    FRQ_CT_FAIL OFFSET(2) NUMBITS(1) [],
    ENT_VAL OFFSET(1) NUMBITS(1) [],
    HW_ERR OFFSET(0) NUMBITS(1) [
            NoErr = 0,
            Err = 1,
        ]
    ],

    // VID1 [
    //     TRNG0_IP_ID OFFSET(16) NUMBITS(16) [],
    // 	TRNG0_MAJ_REV OFFSET(8) NUMBITS(8) [],
    // 	TRNG0_MIN_REV OFFSET(0) NUMBITS(8) [],
    // ],

    // VID2 [
    //     TRNG0_ERA OFFSET(24) NUMBITS(8) [],
    // 	TRNG0_INTG_OPT OFFSET(16) NUMBITS(8) [],
    // 	TRNG0_ECO_REV OFFSET(8) NUMBITS(8) [],
    // 	TRNG0_CONFIG_OPT OFFSET(0) NUMBITS(8) [],
    // ]
}

const TRNG_BASE: StaticRef<TRNGRegisters> =
    unsafe { StaticRef::new(0x400CC000 as *const TRNGRegisters) };

pub struct TRNG<'a> {
    registers: StaticRef<TRNGRegisters>,
    client: OptionalCell<&'a dyn hil::entropy::Client32>,
    ccm: &'static crate::ccm::Ccm,
}

impl<'a> TRNG<'a> {
    pub const fn new(ccm: &'static crate::ccm::Ccm) -> TRNG<'a> {
        TRNG {
            registers: TRNG_BASE,
            client: OptionalCell::empty(),
            ccm,
        }
    }

    pub fn interrupt_handler(&self) {
        debug!("I got an interrupt!");
        /*
        let mut iterator = TRNGIter { trng: self, idx: 0 };
        self.client.map(|client| {
            let result = client.entropy_available(&mut iterator, Ok(()));
            if let Continue::Done = result {
                self.registers.TRNG0_MCTL.modify(Control::PRGM::ProgramMode);
            } else {
                self.registers
                    .TRNG0_INT_MASK
                    .modify(InterruptMask::ENT_VAL::SET);
                self.registers
                    .TRNG0_INT_CTRL
                    .modify(InterruptControl::ENT_VAL::SET);
            }
        });
        */
    }

    // pub fn TRNG_read_entropy(&self) -> u32 {
    // 	let data: u32 = self.registers
    // 	    .TRNG0_ENT0
    // 	    .read(EntropyRead::ENT);
    // 	data
    // }
}

struct TRNGIter<'a, 'b: 'a> {
    trng: &'a TRNG<'b>,
    idx: usize,
}

impl Iterator for TRNGIter<'_, '_> {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.trng.registers.TRNG0_MCTL.is_set(Control::ENT_VAL)
        // .TRNG0_INT_STATUS.
        // is_set(InterruptStatus::ENT_VAL)
        {
            if self.idx < 15 {
                let val = Some(self.trng.registers.TRNG0_ENT[self.idx].read(EntropyRead::ENT));
                self.idx += 1;
                val
            } else if self.idx == 15 {
                let val = Some(self.trng.registers.TRNG0_ENT[self.idx].read(EntropyRead::ENT));
                self.idx = 0;
                val
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl<'a> hil::entropy::Entropy32<'a> for TRNG<'a> {
    fn get(&self) -> Result<(), ErrorCode> {
        debug!("I am born!");
        self.ccm.enable_trng_clock();
        // self.registers.TRNG0_MCTL.modify(Control::TSTOP_OK::OK);
        // debug!("Bogus read: {}", self.registers.TRNG0_ENT[0].read(EntropyRead::ENT));
        //self.registers.TRNG0_MCTL.modify(Control::TRNG_ACC::SET);
        /*for j in 0..100 {
            debug!(
                "Access bit is set! It is {}",
                self.registers.TRNG0_MCTL.read(Control::TRNG_ACC)
            );
        }*/
        //debug!("{}", self.registers.TRNG0_MCTL.is_set(Control::TRNG_ACC));

        // self.registers.TRNG0_INT_MASK.modify(InterruptMask::ENT_VAL::SET);
        // debug!("ENT_VAL in MASK is now {}", self.registers.TRNG0_INT_MASK.read(InterruptMask::ENT_VAL));
        // self.registers.TRNG0_INT_CTRL.modify(InterruptControl::ENT_VAL::SET);
        // debug!("ENT_VAL in CTRL is now {}", self.registers.TRNG0_INT_CTRL. read(InterruptControl::ENT_VAL));
        // self.registers.TRNG0_INT_MASK.modify(InterruptMask::HW_ERR::SET);
        // debug!("HW_ERR in MASK");
        // self.registers.TRNG0_INT_CTRL.modify(InterruptControl::HW_ERR::SET);
        // debug!("HW_ERR in CTRL");
        self.registers.TRNG0_MCTL.modify(Control::PRGM::SET);
        self.registers.TRNG0_MCTL.modify(Control::RST_DEF::SET);
        self.registers.TRNG0_MCTL.modify(Control::SAMP_MODE::VON);
        debug!("LAST ALIVE");
        self.registers.TRNG0_MCTL.modify(Control::PRGM::CLEAR);
        unsafe {
            core::ptr::read_volatile(0x400CC0B8 as *const u8);
            //core::ptr::write_volatile(0x400CC0B8 as *mut u32, 1u32);
        }
        debug!("DEAD");
        /*debug!(
            "RunMode is set! It is {}",
            self.registers.TRNG0_MCTL.read(Control::PRGM)
        );*/
        // debug!("{}", self.registers.TRNG0_ENT[15].read(EntropyRead::ENT));
        // debug!("{}", self.registers.TRNG0_ENT[0].read(EntropyRead::ENT));
        self.registers.TRNG0_ENT[15].read(EntropyRead::ENT); // start generating values

        /*
        loop {
            if self.registers.TRNG0_MCTL.is_set(Control::ENT_VAL) {
                debug!("This happened.");
                for i in 0..16 {
                    debug!("{}", self.registers.TRNG0_ENT[i].read(EntropyRead::ENT));
                }
                break;
            }
        }
        */
        Ok(())
    }

    fn cancel(&self) -> Result<(), ErrorCode> {
        Err(ErrorCode::FAIL)
    }

    fn set_client(&'a self, client: &'a dyn hil::entropy::Client32) {
        self.client.set(client);
    }
}
/*
impl<'a> hil::rng::Rng<'a> for TRNG<'a> {
    fn get(&self) -> Result<(), ErrorCode> {
        self.registers.TRNG0_MCTL.modify(Control::PRGM::RunMode);
        self.registers.TRNG0_MCTL.modify(Control::TSTOP_OK::OK);
        self.registers.TRNG0_MCTL.modify(Control::TRNG_ACC::Acc);
        Ok(())
    }

    fn cancel(&self) -> Result<(), ErrorCode> {
        Err(ErrorCode::FAIL)
    }

    fn set_client(&'a self, client: &'a dyn hil::rng::Client) {
        self.client.set(client);
    }
}
*/

// pub static mut TRNG: TRNG = TRNG::new();
