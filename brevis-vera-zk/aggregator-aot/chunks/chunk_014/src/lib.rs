pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2157940u32;
pub const PC_MAX: u32 = 2166080u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 131usize] = [
        block_0x0020ed74,
        block_0x0020edc8,
        block_0x0020ee1c,
        block_0x0020ee70,
        block_0x0020eec4,
        block_0x0020ef18,
        block_0x0020ef84,
        block_0x0020efd8,
        block_0x0020f02c,
        block_0x0020f080,
        block_0x0020f0d4,
        block_0x0020f128,
        block_0x0020f17c,
        block_0x0020f1d0,
        block_0x0020f224,
        block_0x0020f278,
        block_0x0020f2cc,
        block_0x0020f320,
        block_0x0020f374,
        block_0x0020f3c8,
        block_0x0020f41c,
        block_0x0020f470,
        block_0x0020f4c4,
        block_0x0020f530,
        block_0x0020f544,
        block_0x0020f598,
        block_0x0020f5ec,
        block_0x0020f640,
        block_0x0020f694,
        block_0x0020f6e8,
        block_0x0020f73c,
        block_0x0020f790,
        block_0x0020f7e4,
        block_0x0020f838,
        block_0x0020f88c,
        block_0x0020f8e0,
        block_0x0020f934,
        block_0x0020f988,
        block_0x0020f9dc,
        block_0x0020fa30,
        block_0x0020fa84,
        block_0x0020fad8,
        block_0x0020fb2c,
        block_0x0020fb80,
        block_0x0020fbd4,
        block_0x0020fc28,
        block_0x0020fc7c,
        block_0x0020fcd0,
        block_0x0020fd24,
        block_0x0020fd78,
        block_0x0020fdcc,
        block_0x0020fe20,
        block_0x0020fe74,
        block_0x0020fec8,
        block_0x0020ff1c,
        block_0x0020ff70,
        block_0x0020ffdc,
        block_0x0020ffe0,
        block_0x0020fff4,
        block_0x0021003c,
        block_0x00210090,
        block_0x00210094,
        block_0x002100a8,
        block_0x002100f0,
        block_0x00210164,
        block_0x002101f0,
        block_0x0021020c,
        block_0x00210438,
        block_0x00210474,
        block_0x002104ac,
        block_0x002104ec,
        block_0x002104f0,
        block_0x00210534,
        block_0x00210538,
        block_0x00210588,
        block_0x0021058c,
        block_0x002105fc,
        block_0x00210600,
        block_0x00210650,
        block_0x00210654,
        block_0x00210690,
        block_0x00210694,
        block_0x002106e4,
        block_0x002106e8,
        block_0x00210720,
        block_0x00210724,
        block_0x00210774,
        block_0x00210778,
        block_0x002107a0,
        block_0x002107a8,
        block_0x002107ac,
        block_0x002107e0,
        block_0x002107e4,
        block_0x00210808,
        block_0x0021080c,
        block_0x00210828,
        block_0x00210830,
        block_0x00210834,
        block_0x00210864,
        block_0x00210868,
        block_0x002108b0,
        block_0x002108b4,
        block_0x002109ec,
        block_0x00210a2c,
        block_0x00210a30,
        block_0x00210a74,
        block_0x00210a78,
        block_0x00210ac8,
        block_0x00210acc,
        block_0x00210b08,
        block_0x00210b0c,
        block_0x00210b5c,
        block_0x00210b60,
        block_0x00210b9c,
        block_0x00210ba0,
        block_0x00210bf0,
        block_0x00210bf4,
        block_0x00210c2c,
        block_0x00210c30,
        block_0x00210c80,
        block_0x00210c84,
        block_0x00210cac,
        block_0x00210cb4,
        block_0x00210cb8,
        block_0x00210cec,
        block_0x00210cf0,
        block_0x00210d14,
        block_0x00210d18,
        block_0x00210d28,
        block_0x00210d38,
        block_0x00210d40,
    ];
    #[repr(C)]
    struct Run {
        start_word: u32,
        len: u16,
        fn_offset: u16,
    }
    const RUNS: [Run; 102usize] = [
        Run {
            start_word: 0u32,
            len: 1i32 as u16,
            fn_offset: 0usize as u16,
        },
        Run {
            start_word: 21u32,
            len: 1i32 as u16,
            fn_offset: 1usize as u16,
        },
        Run {
            start_word: 42u32,
            len: 1i32 as u16,
            fn_offset: 2usize as u16,
        },
        Run {
            start_word: 63u32,
            len: 1i32 as u16,
            fn_offset: 3usize as u16,
        },
        Run {
            start_word: 84u32,
            len: 1i32 as u16,
            fn_offset: 4usize as u16,
        },
        Run {
            start_word: 105u32,
            len: 1i32 as u16,
            fn_offset: 5usize as u16,
        },
        Run {
            start_word: 132u32,
            len: 1i32 as u16,
            fn_offset: 6usize as u16,
        },
        Run {
            start_word: 153u32,
            len: 1i32 as u16,
            fn_offset: 7usize as u16,
        },
        Run {
            start_word: 174u32,
            len: 1i32 as u16,
            fn_offset: 8usize as u16,
        },
        Run {
            start_word: 195u32,
            len: 1i32 as u16,
            fn_offset: 9usize as u16,
        },
        Run {
            start_word: 216u32,
            len: 1i32 as u16,
            fn_offset: 10usize as u16,
        },
        Run {
            start_word: 237u32,
            len: 1i32 as u16,
            fn_offset: 11usize as u16,
        },
        Run {
            start_word: 258u32,
            len: 1i32 as u16,
            fn_offset: 12usize as u16,
        },
        Run {
            start_word: 279u32,
            len: 1i32 as u16,
            fn_offset: 13usize as u16,
        },
        Run {
            start_word: 300u32,
            len: 1i32 as u16,
            fn_offset: 14usize as u16,
        },
        Run {
            start_word: 321u32,
            len: 1i32 as u16,
            fn_offset: 15usize as u16,
        },
        Run {
            start_word: 342u32,
            len: 1i32 as u16,
            fn_offset: 16usize as u16,
        },
        Run {
            start_word: 363u32,
            len: 1i32 as u16,
            fn_offset: 17usize as u16,
        },
        Run {
            start_word: 384u32,
            len: 1i32 as u16,
            fn_offset: 18usize as u16,
        },
        Run {
            start_word: 405u32,
            len: 1i32 as u16,
            fn_offset: 19usize as u16,
        },
        Run {
            start_word: 426u32,
            len: 1i32 as u16,
            fn_offset: 20usize as u16,
        },
        Run {
            start_word: 447u32,
            len: 1i32 as u16,
            fn_offset: 21usize as u16,
        },
        Run {
            start_word: 468u32,
            len: 1i32 as u16,
            fn_offset: 22usize as u16,
        },
        Run {
            start_word: 495u32,
            len: 1i32 as u16,
            fn_offset: 23usize as u16,
        },
        Run {
            start_word: 500u32,
            len: 1i32 as u16,
            fn_offset: 24usize as u16,
        },
        Run {
            start_word: 521u32,
            len: 1i32 as u16,
            fn_offset: 25usize as u16,
        },
        Run {
            start_word: 542u32,
            len: 1i32 as u16,
            fn_offset: 26usize as u16,
        },
        Run {
            start_word: 563u32,
            len: 1i32 as u16,
            fn_offset: 27usize as u16,
        },
        Run {
            start_word: 584u32,
            len: 1i32 as u16,
            fn_offset: 28usize as u16,
        },
        Run {
            start_word: 605u32,
            len: 1i32 as u16,
            fn_offset: 29usize as u16,
        },
        Run {
            start_word: 626u32,
            len: 1i32 as u16,
            fn_offset: 30usize as u16,
        },
        Run {
            start_word: 647u32,
            len: 1i32 as u16,
            fn_offset: 31usize as u16,
        },
        Run {
            start_word: 668u32,
            len: 1i32 as u16,
            fn_offset: 32usize as u16,
        },
        Run {
            start_word: 689u32,
            len: 1i32 as u16,
            fn_offset: 33usize as u16,
        },
        Run {
            start_word: 710u32,
            len: 1i32 as u16,
            fn_offset: 34usize as u16,
        },
        Run {
            start_word: 731u32,
            len: 1i32 as u16,
            fn_offset: 35usize as u16,
        },
        Run {
            start_word: 752u32,
            len: 1i32 as u16,
            fn_offset: 36usize as u16,
        },
        Run {
            start_word: 773u32,
            len: 1i32 as u16,
            fn_offset: 37usize as u16,
        },
        Run {
            start_word: 794u32,
            len: 1i32 as u16,
            fn_offset: 38usize as u16,
        },
        Run {
            start_word: 815u32,
            len: 1i32 as u16,
            fn_offset: 39usize as u16,
        },
        Run {
            start_word: 836u32,
            len: 1i32 as u16,
            fn_offset: 40usize as u16,
        },
        Run {
            start_word: 857u32,
            len: 1i32 as u16,
            fn_offset: 41usize as u16,
        },
        Run {
            start_word: 878u32,
            len: 1i32 as u16,
            fn_offset: 42usize as u16,
        },
        Run {
            start_word: 899u32,
            len: 1i32 as u16,
            fn_offset: 43usize as u16,
        },
        Run {
            start_word: 920u32,
            len: 1i32 as u16,
            fn_offset: 44usize as u16,
        },
        Run {
            start_word: 941u32,
            len: 1i32 as u16,
            fn_offset: 45usize as u16,
        },
        Run {
            start_word: 962u32,
            len: 1i32 as u16,
            fn_offset: 46usize as u16,
        },
        Run {
            start_word: 983u32,
            len: 1i32 as u16,
            fn_offset: 47usize as u16,
        },
        Run {
            start_word: 1004u32,
            len: 1i32 as u16,
            fn_offset: 48usize as u16,
        },
        Run {
            start_word: 1025u32,
            len: 1i32 as u16,
            fn_offset: 49usize as u16,
        },
        Run {
            start_word: 1046u32,
            len: 1i32 as u16,
            fn_offset: 50usize as u16,
        },
        Run {
            start_word: 1067u32,
            len: 1i32 as u16,
            fn_offset: 51usize as u16,
        },
        Run {
            start_word: 1088u32,
            len: 1i32 as u16,
            fn_offset: 52usize as u16,
        },
        Run {
            start_word: 1109u32,
            len: 1i32 as u16,
            fn_offset: 53usize as u16,
        },
        Run {
            start_word: 1130u32,
            len: 1i32 as u16,
            fn_offset: 54usize as u16,
        },
        Run {
            start_word: 1151u32,
            len: 1i32 as u16,
            fn_offset: 55usize as u16,
        },
        Run {
            start_word: 1178u32,
            len: 2i32 as u16,
            fn_offset: 56usize as u16,
        },
        Run {
            start_word: 1184u32,
            len: 1i32 as u16,
            fn_offset: 58usize as u16,
        },
        Run {
            start_word: 1202u32,
            len: 1i32 as u16,
            fn_offset: 59usize as u16,
        },
        Run {
            start_word: 1223u32,
            len: 2i32 as u16,
            fn_offset: 60usize as u16,
        },
        Run {
            start_word: 1229u32,
            len: 1i32 as u16,
            fn_offset: 62usize as u16,
        },
        Run {
            start_word: 1247u32,
            len: 1i32 as u16,
            fn_offset: 63usize as u16,
        },
        Run {
            start_word: 1276u32,
            len: 1i32 as u16,
            fn_offset: 64usize as u16,
        },
        Run {
            start_word: 1311u32,
            len: 1i32 as u16,
            fn_offset: 65usize as u16,
        },
        Run {
            start_word: 1318u32,
            len: 1i32 as u16,
            fn_offset: 66usize as u16,
        },
        Run {
            start_word: 1457u32,
            len: 1i32 as u16,
            fn_offset: 67usize as u16,
        },
        Run {
            start_word: 1472u32,
            len: 1i32 as u16,
            fn_offset: 68usize as u16,
        },
        Run {
            start_word: 1486u32,
            len: 1i32 as u16,
            fn_offset: 69usize as u16,
        },
        Run {
            start_word: 1502u32,
            len: 2i32 as u16,
            fn_offset: 70usize as u16,
        },
        Run {
            start_word: 1520u32,
            len: 2i32 as u16,
            fn_offset: 72usize as u16,
        },
        Run {
            start_word: 1541u32,
            len: 2i32 as u16,
            fn_offset: 74usize as u16,
        },
        Run {
            start_word: 1570u32,
            len: 2i32 as u16,
            fn_offset: 76usize as u16,
        },
        Run {
            start_word: 1591u32,
            len: 2i32 as u16,
            fn_offset: 78usize as u16,
        },
        Run {
            start_word: 1607u32,
            len: 2i32 as u16,
            fn_offset: 80usize as u16,
        },
        Run {
            start_word: 1628u32,
            len: 2i32 as u16,
            fn_offset: 82usize as u16,
        },
        Run {
            start_word: 1643u32,
            len: 2i32 as u16,
            fn_offset: 84usize as u16,
        },
        Run {
            start_word: 1664u32,
            len: 2i32 as u16,
            fn_offset: 86usize as u16,
        },
        Run {
            start_word: 1675u32,
            len: 1i32 as u16,
            fn_offset: 88usize as u16,
        },
        Run {
            start_word: 1677u32,
            len: 2i32 as u16,
            fn_offset: 89usize as u16,
        },
        Run {
            start_word: 1691u32,
            len: 2i32 as u16,
            fn_offset: 91usize as u16,
        },
        Run {
            start_word: 1701u32,
            len: 2i32 as u16,
            fn_offset: 93usize as u16,
        },
        Run {
            start_word: 1709u32,
            len: 1i32 as u16,
            fn_offset: 95usize as u16,
        },
        Run {
            start_word: 1711u32,
            len: 2i32 as u16,
            fn_offset: 96usize as u16,
        },
        Run {
            start_word: 1724u32,
            len: 2i32 as u16,
            fn_offset: 98usize as u16,
        },
        Run {
            start_word: 1743u32,
            len: 2i32 as u16,
            fn_offset: 100usize as u16,
        },
        Run {
            start_word: 1822u32,
            len: 1i32 as u16,
            fn_offset: 102usize as u16,
        },
        Run {
            start_word: 1838u32,
            len: 2i32 as u16,
            fn_offset: 103usize as u16,
        },
        Run {
            start_word: 1856u32,
            len: 2i32 as u16,
            fn_offset: 105usize as u16,
        },
        Run {
            start_word: 1877u32,
            len: 2i32 as u16,
            fn_offset: 107usize as u16,
        },
        Run {
            start_word: 1893u32,
            len: 2i32 as u16,
            fn_offset: 109usize as u16,
        },
        Run {
            start_word: 1914u32,
            len: 2i32 as u16,
            fn_offset: 111usize as u16,
        },
        Run {
            start_word: 1930u32,
            len: 2i32 as u16,
            fn_offset: 113usize as u16,
        },
        Run {
            start_word: 1951u32,
            len: 2i32 as u16,
            fn_offset: 115usize as u16,
        },
        Run {
            start_word: 1966u32,
            len: 2i32 as u16,
            fn_offset: 117usize as u16,
        },
        Run {
            start_word: 1987u32,
            len: 2i32 as u16,
            fn_offset: 119usize as u16,
        },
        Run {
            start_word: 1998u32,
            len: 1i32 as u16,
            fn_offset: 121usize as u16,
        },
        Run {
            start_word: 2000u32,
            len: 2i32 as u16,
            fn_offset: 122usize as u16,
        },
        Run {
            start_word: 2014u32,
            len: 2i32 as u16,
            fn_offset: 124usize as u16,
        },
        Run {
            start_word: 2024u32,
            len: 2i32 as u16,
            fn_offset: 126usize as u16,
        },
        Run {
            start_word: 2029u32,
            len: 1i32 as u16,
            fn_offset: 128usize as u16,
        },
        Run {
            start_word: 2033u32,
            len: 1i32 as u16,
            fn_offset: 129usize as u16,
        },
        Run {
            start_word: 2035u32,
            len: 1i32 as u16,
            fn_offset: 130usize as u16,
        },
    ];
    if pc < 2157940u32 || pc > 2166080u32 {
        return None;
    }
    let word_offset = ((pc - 2157940u32) >> 2) as u32;
    let mut lo = 0usize;
    let mut hi = RUNS.len();
    while lo < hi {
        let mid = (lo + hi) >> 1;
        let run = &RUNS[mid];
        if word_offset < run.start_word {
            hi = mid;
        } else if word_offset >= run.start_word + run.len as u32 {
            lo = mid + 1;
        } else {
            let fn_idx = (run.fn_offset as usize)
                + (word_offset - run.start_word) as usize;
            return Some(FN[fn_idx]);
        }
    }
    None
}
#[inline]
pub fn block_0x0020ed74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2157944u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2157948u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2157952u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2157956u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2157960u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2157964u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2157968u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2157972u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2157976u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2157980u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2157984u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2157988u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2157992u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2157996u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2158000u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2158004u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2158008u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2158012u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2158016u32);
    emu.apc_no_count(1usize, 2158016u32, 4294946816u32, 2158020u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158024u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966900u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020edc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2158028u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2158032u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2158036u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2158040u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2158044u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2158048u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2158052u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2158056u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2158060u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2158064u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2158068u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2158072u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2158076u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2158080u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2158084u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2158088u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2158092u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2158096u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2158100u32);
    emu.apc_no_count(1usize, 2158100u32, 4294946816u32, 2158104u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158108u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966816u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020ee1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2158112u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2158116u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2158120u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2158124u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2158128u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2158132u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2158136u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2158140u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2158144u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2158148u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2158152u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2158156u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2158160u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2158164u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2158168u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2158172u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2158176u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2158180u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2158184u32);
    emu.apc_no_count(1usize, 2158184u32, 4294946816u32, 2158188u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158192u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966732u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020ee70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2158196u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2158200u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2158204u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2158208u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2158212u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2158216u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2158220u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2158224u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2158228u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2158232u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2158236u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2158240u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2158244u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2158248u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2158252u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2158256u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2158260u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2158264u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2158268u32);
    emu.apc_no_count(1usize, 2158268u32, 4294946816u32, 2158272u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158276u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966648u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020eec4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2158280u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2158284u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2158288u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2158292u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2158296u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2158300u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2158304u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2158308u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2158312u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2158316u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2158320u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2158324u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2158328u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2158332u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2158336u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2158340u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2158344u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2158348u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2158352u32);
    emu.apc_no_count(1usize, 2158352u32, 4294946816u32, 2158356u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158360u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966564u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0020ef18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2158364u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2158368u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2158372u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2158376u32)?;
    emu.lw_no_count(14usize, 2usize, 192u32, 2158380u32)?;
    emu.lw_no_count(15usize, 2usize, 196u32, 2158384u32)?;
    emu.lw_no_count(16usize, 2usize, 200u32, 2158388u32)?;
    emu.lw_no_count(17usize, 2usize, 204u32, 2158392u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2158396u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2158400u32)?;
    emu.sw_no_count(14usize, 2usize, 160u32, 2158404u32)?;
    emu.sw_no_count(17usize, 2usize, 172u32, 2158408u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2158412u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2158416u32)?;
    emu.sw_no_count(14usize, 2usize, 192u32, 2158420u32)?;
    emu.sw_no_count(15usize, 2usize, 196u32, 2158424u32)?;
    emu.sw_no_count(16usize, 2usize, 200u32, 2158428u32)?;
    emu.sw_no_count(17usize, 2usize, 204u32, 2158432u32)?;
    emu.sw_no_count(10usize, 2usize, 208u32, 2158436u32)?;
    emu.sw_no_count(11usize, 2usize, 212u32, 2158440u32)?;
    emu.sw_no_count(12usize, 2usize, 216u32, 2158444u32)?;
    emu.sw_no_count(13usize, 2usize, 220u32, 2158448u32)?;
    emu.adi_no_count(10usize, 2usize, 96u32, 2158452u32);
    emu.adi_no_count(11usize, 2usize, 192u32, 2158456u32);
    emu.adi_no_count(12usize, 2usize, 64u32, 2158460u32);
    emu.apc_no_count(1usize, 2158460u32, 4294946816u32, 2158464u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158468u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966456u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020ef84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 112u32, 2158472u32)?;
    emu.lw_no_count(11usize, 2usize, 116u32, 2158476u32)?;
    emu.lw_no_count(12usize, 2usize, 120u32, 2158480u32)?;
    emu.lw_no_count(13usize, 2usize, 124u32, 2158484u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2158488u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2158492u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2158496u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2158500u32)?;
    emu.lw_no_count(10usize, 2usize, 96u32, 2158504u32)?;
    emu.lw_no_count(11usize, 2usize, 100u32, 2158508u32)?;
    emu.lw_no_count(12usize, 2usize, 104u32, 2158512u32)?;
    emu.lw_no_count(13usize, 2usize, 108u32, 2158516u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2158520u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2158524u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2158528u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2158532u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2158536u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2158540u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2158544u32);
    emu.apc_no_count(1usize, 2158544u32, 4294946816u32, 2158548u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158552u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966372u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020efd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2158556u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2158560u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2158564u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2158568u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2158572u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2158576u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2158580u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2158584u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2158588u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2158592u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2158596u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2158600u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2158604u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2158608u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2158612u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2158616u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2158620u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2158624u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2158628u32);
    emu.apc_no_count(1usize, 2158628u32, 4294946816u32, 2158632u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158636u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966288u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f02c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2158640u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2158644u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2158648u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2158652u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2158656u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2158660u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2158664u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2158668u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2158672u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2158676u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2158680u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2158684u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2158688u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2158692u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2158696u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2158700u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2158704u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2158708u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2158712u32);
    emu.apc_no_count(1usize, 2158712u32, 4294946816u32, 2158716u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158720u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966204u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f080(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2158724u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2158728u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2158732u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2158736u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2158740u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2158744u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2158748u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2158752u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2158756u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2158760u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2158764u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2158768u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2158772u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2158776u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2158780u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2158784u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2158788u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2158792u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2158796u32);
    emu.apc_no_count(1usize, 2158796u32, 4294946816u32, 2158800u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158804u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966120u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f0d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2158808u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2158812u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2158816u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2158820u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2158824u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2158828u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2158832u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2158836u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2158840u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2158844u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2158848u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2158852u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2158856u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2158860u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2158864u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2158868u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2158872u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2158876u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2158880u32);
    emu.apc_no_count(1usize, 2158880u32, 4294946816u32, 2158884u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158888u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966036u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f128(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2158892u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2158896u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2158900u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2158904u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2158908u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2158912u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2158916u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2158920u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2158924u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2158928u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2158932u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2158936u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2158940u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2158944u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2158948u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2158952u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2158956u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2158960u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2158964u32);
    emu.apc_no_count(1usize, 2158964u32, 4294946816u32, 2158968u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158972u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965952u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f17c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2158976u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2158980u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2158984u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2158988u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2158992u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2158996u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2159000u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2159004u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2159008u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2159012u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2159016u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2159020u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2159024u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2159028u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2159032u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2159036u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2159040u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2159044u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2159048u32);
    emu.apc_no_count(1usize, 2159048u32, 4294946816u32, 2159052u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2159056u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965868u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f1d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2159060u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2159064u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2159068u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2159072u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2159076u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2159080u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2159084u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2159088u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2159092u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2159096u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2159100u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2159104u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2159108u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2159112u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2159116u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2159120u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2159124u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2159128u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2159132u32);
    emu.apc_no_count(1usize, 2159132u32, 4294946816u32, 2159136u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2159140u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965784u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f224(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2159144u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2159148u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2159152u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2159156u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2159160u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2159164u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2159168u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2159172u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2159176u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2159180u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2159184u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2159188u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2159192u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2159196u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2159200u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2159204u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2159208u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2159212u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2159216u32);
    emu.apc_no_count(1usize, 2159216u32, 4294946816u32, 2159220u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2159224u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965700u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f278(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2159228u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2159232u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2159236u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2159240u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2159244u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2159248u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2159252u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2159256u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2159260u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2159264u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2159268u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2159272u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2159276u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2159280u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2159284u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2159288u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2159292u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2159296u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2159300u32);
    emu.apc_no_count(1usize, 2159300u32, 4294946816u32, 2159304u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2159308u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965616u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f2cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2159312u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2159316u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2159320u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2159324u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2159328u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2159332u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2159336u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2159340u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2159344u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2159348u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2159352u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2159356u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2159360u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2159364u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2159368u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2159372u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2159376u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2159380u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2159384u32);
    emu.apc_no_count(1usize, 2159384u32, 4294946816u32, 2159388u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2159392u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965532u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f320(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2159396u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2159400u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2159404u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2159408u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2159412u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2159416u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2159420u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2159424u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2159428u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2159432u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2159436u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2159440u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2159444u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2159448u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2159452u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2159456u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2159460u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2159464u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2159468u32);
    emu.apc_no_count(1usize, 2159468u32, 4294946816u32, 2159472u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2159476u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965448u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f374(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2159480u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2159484u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2159488u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2159492u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2159496u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2159500u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2159504u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2159508u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2159512u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2159516u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2159520u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2159524u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2159528u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2159532u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2159536u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2159540u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2159544u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2159548u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2159552u32);
    emu.apc_no_count(1usize, 2159552u32, 4294946816u32, 2159556u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2159560u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965364u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f3c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2159564u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2159568u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2159572u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2159576u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2159580u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2159584u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2159588u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2159592u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2159596u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2159600u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2159604u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2159608u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2159612u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2159616u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2159620u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2159624u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2159628u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2159632u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2159636u32);
    emu.apc_no_count(1usize, 2159636u32, 4294946816u32, 2159640u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2159644u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965280u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f41c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2159648u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2159652u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2159656u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2159660u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2159664u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2159668u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2159672u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2159676u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2159680u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2159684u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2159688u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2159692u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2159696u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2159700u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2159704u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2159708u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2159712u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2159716u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2159720u32);
    emu.apc_no_count(1usize, 2159720u32, 4294942720u32, 2159724u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2159728u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1996u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f470(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2159732u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2159736u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2159740u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2159744u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2159748u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2159752u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2159756u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2159760u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2159764u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2159768u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2159772u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2159776u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2159780u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2159784u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2159788u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2159792u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2159796u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2159800u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2159804u32);
    emu.apc_no_count(1usize, 2159804u32, 4294942720u32, 2159808u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2159812u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1912u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0020f4c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2159816u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2159820u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2159824u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2159828u32)?;
    emu.lw_no_count(14usize, 2usize, 192u32, 2159832u32)?;
    emu.lw_no_count(15usize, 2usize, 196u32, 2159836u32)?;
    emu.lw_no_count(16usize, 2usize, 200u32, 2159840u32)?;
    emu.lw_no_count(17usize, 2usize, 204u32, 2159844u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2159848u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2159852u32)?;
    emu.sw_no_count(14usize, 2usize, 160u32, 2159856u32)?;
    emu.sw_no_count(17usize, 2usize, 172u32, 2159860u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2159864u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2159868u32)?;
    emu.sw_no_count(14usize, 2usize, 192u32, 2159872u32)?;
    emu.sw_no_count(15usize, 2usize, 196u32, 2159876u32)?;
    emu.sw_no_count(16usize, 2usize, 200u32, 2159880u32)?;
    emu.sw_no_count(17usize, 2usize, 204u32, 2159884u32)?;
    emu.sw_no_count(10usize, 2usize, 208u32, 2159888u32)?;
    emu.sw_no_count(11usize, 2usize, 212u32, 2159892u32)?;
    emu.sw_no_count(12usize, 2usize, 216u32, 2159896u32)?;
    emu.sw_no_count(13usize, 2usize, 220u32, 2159900u32)?;
    emu.adi_no_count(10usize, 2usize, 160u32, 2159904u32);
    emu.adi_no_count(11usize, 2usize, 192u32, 2159908u32);
    emu.adi_no_count(12usize, 2usize, 96u32, 2159912u32);
    emu.apc_no_count(1usize, 2159912u32, 4294942720u32, 2159916u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2159920u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1804u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020f530(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 192u32, 2159924u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2159928u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2159932u32);
    emu.apc_no_count(1usize, 2159932u32, 4294942720u32, 2159936u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2159940u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1784u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f544(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2159944u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2159948u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2159952u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2159956u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2159960u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2159964u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2159968u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2159972u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2159976u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2159980u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2159984u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2159988u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2159992u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2159996u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2160000u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2160004u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2160008u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2160012u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2160016u32);
    emu.apc_no_count(1usize, 2160016u32, 4294942720u32, 2160020u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160024u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1700u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f598(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2160028u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2160032u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2160036u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2160040u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2160044u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2160048u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2160052u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2160056u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2160060u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2160064u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2160068u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2160072u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2160076u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2160080u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2160084u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2160088u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2160092u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2160096u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2160100u32);
    emu.apc_no_count(1usize, 2160100u32, 4294942720u32, 2160104u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160108u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1616u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f5ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2160112u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2160116u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2160120u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2160124u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2160128u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2160132u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2160136u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2160140u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2160144u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2160148u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2160152u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2160156u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2160160u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2160164u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2160168u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2160172u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2160176u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2160180u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2160184u32);
    emu.apc_no_count(1usize, 2160184u32, 4294942720u32, 2160188u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160192u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1532u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f640(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2160196u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2160200u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2160204u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2160208u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2160212u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2160216u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2160220u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2160224u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2160228u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2160232u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2160236u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2160240u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2160244u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2160248u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2160252u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2160256u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2160260u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2160264u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2160268u32);
    emu.apc_no_count(1usize, 2160268u32, 4294942720u32, 2160272u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160276u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1448u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f694(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2160280u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2160284u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2160288u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2160292u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2160296u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2160300u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2160304u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2160308u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2160312u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2160316u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2160320u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2160324u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2160328u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2160332u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2160336u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2160340u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2160344u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2160348u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2160352u32);
    emu.apc_no_count(1usize, 2160352u32, 4294942720u32, 2160356u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160360u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1364u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f6e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2160364u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2160368u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2160372u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2160376u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2160380u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2160384u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2160388u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2160392u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2160396u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2160400u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2160404u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2160408u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2160412u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2160416u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2160420u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2160424u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2160428u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2160432u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2160436u32);
    emu.apc_no_count(1usize, 2160436u32, 4294942720u32, 2160440u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160444u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1280u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f73c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2160448u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2160452u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2160456u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2160460u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2160464u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2160468u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2160472u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2160476u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2160480u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2160484u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2160488u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2160492u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2160496u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2160500u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2160504u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2160508u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2160512u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2160516u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2160520u32);
    emu.apc_no_count(1usize, 2160520u32, 4294942720u32, 2160524u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160528u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1196u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f790(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2160532u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2160536u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2160540u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2160544u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2160548u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2160552u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2160556u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2160560u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2160564u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2160568u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2160572u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2160576u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2160580u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2160584u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2160588u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2160592u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2160596u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2160600u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2160604u32);
    emu.apc_no_count(1usize, 2160604u32, 4294942720u32, 2160608u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160612u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1112u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f7e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2160616u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2160620u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2160624u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2160628u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2160632u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2160636u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2160640u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2160644u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2160648u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2160652u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2160656u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2160660u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2160664u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2160668u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2160672u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2160676u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2160680u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2160684u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2160688u32);
    emu.apc_no_count(1usize, 2160688u32, 4294942720u32, 2160692u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160696u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1028u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f838(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2160700u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2160704u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2160708u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2160712u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2160716u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2160720u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2160724u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2160728u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2160732u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2160736u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2160740u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2160744u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2160748u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2160752u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2160756u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2160760u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2160764u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2160768u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2160772u32);
    emu.apc_no_count(1usize, 2160772u32, 4294942720u32, 2160776u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160780u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(944u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f88c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2160784u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2160788u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2160792u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2160796u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2160800u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2160804u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2160808u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2160812u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2160816u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2160820u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2160824u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2160828u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2160832u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2160836u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2160840u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2160844u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2160848u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2160852u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2160856u32);
    emu.apc_no_count(1usize, 2160856u32, 4294942720u32, 2160860u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160864u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(860u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f8e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2160868u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2160872u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2160876u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2160880u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2160884u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2160888u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2160892u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2160896u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2160900u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2160904u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2160908u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2160912u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2160916u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2160920u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2160924u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2160928u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2160932u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2160936u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2160940u32);
    emu.apc_no_count(1usize, 2160940u32, 4294942720u32, 2160944u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160948u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(776u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f934(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2160952u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2160956u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2160960u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2160964u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2160968u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2160972u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2160976u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2160980u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2160984u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2160988u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2160992u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2160996u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2161000u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2161004u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2161008u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2161012u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2161016u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2161020u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2161024u32);
    emu.apc_no_count(1usize, 2161024u32, 4294942720u32, 2161028u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161032u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(692u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f988(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2161036u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2161040u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2161044u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2161048u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2161052u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2161056u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2161060u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2161064u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2161068u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2161072u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2161076u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2161080u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2161084u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2161088u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2161092u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2161096u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2161100u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2161104u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2161108u32);
    emu.apc_no_count(1usize, 2161108u32, 4294942720u32, 2161112u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161116u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(608u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f9dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2161120u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2161124u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2161128u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2161132u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2161136u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2161140u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2161144u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2161148u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2161152u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2161156u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2161160u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2161164u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2161168u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2161172u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2161176u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2161180u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2161184u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2161188u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2161192u32);
    emu.apc_no_count(1usize, 2161192u32, 4294942720u32, 2161196u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161200u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(524u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020fa30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2161204u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2161208u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2161212u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2161216u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2161220u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2161224u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2161228u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2161232u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2161236u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2161240u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2161244u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2161248u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2161252u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2161256u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2161260u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2161264u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2161268u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2161272u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2161276u32);
    emu.apc_no_count(1usize, 2161276u32, 4294942720u32, 2161280u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161284u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(440u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020fa84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2161288u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2161292u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2161296u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2161300u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2161304u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2161308u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2161312u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2161316u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2161320u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2161324u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2161328u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2161332u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2161336u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2161340u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2161344u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2161348u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2161352u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2161356u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2161360u32);
    emu.apc_no_count(1usize, 2161360u32, 4294942720u32, 2161364u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161368u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(356u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020fad8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2161372u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2161376u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2161380u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2161384u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2161388u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2161392u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2161396u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2161400u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2161404u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2161408u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2161412u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2161416u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2161420u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2161424u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2161428u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2161432u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2161436u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2161440u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2161444u32);
    emu.apc_no_count(1usize, 2161444u32, 4294942720u32, 2161448u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161452u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(272u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020fb2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2161456u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2161460u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2161464u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2161468u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2161472u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2161476u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2161480u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2161484u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2161488u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2161492u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2161496u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2161500u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2161504u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2161508u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2161512u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2161516u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2161520u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2161524u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2161528u32);
    emu.apc_no_count(1usize, 2161528u32, 4294942720u32, 2161532u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161536u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(188u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020fb80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2161540u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2161544u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2161548u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2161552u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2161556u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2161560u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2161564u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2161568u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2161572u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2161576u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2161580u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2161584u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2161588u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2161592u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2161596u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2161600u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2161604u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2161608u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2161612u32);
    emu.apc_no_count(1usize, 2161612u32, 4294942720u32, 2161616u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161620u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(104u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020fbd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2161624u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2161628u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2161632u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2161636u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2161640u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2161644u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2161648u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2161652u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2161656u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2161660u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2161664u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2161668u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2161672u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2161676u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2161680u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2161684u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2161688u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2161692u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2161696u32);
    emu.apc_no_count(1usize, 2161696u32, 4294942720u32, 2161700u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161704u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(20u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020fc28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2161708u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2161712u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2161716u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2161720u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2161724u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2161728u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2161732u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2161736u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2161740u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2161744u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2161748u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2161752u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2161756u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2161760u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2161764u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2161768u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2161772u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2161776u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2161780u32);
    emu.apc_no_count(1usize, 2161780u32, 4294942720u32, 2161784u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161788u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967232u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020fc7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2161792u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2161796u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2161800u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2161804u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2161808u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2161812u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2161816u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2161820u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2161824u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2161828u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2161832u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2161836u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2161840u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2161844u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2161848u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2161852u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2161856u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2161860u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2161864u32);
    emu.apc_no_count(1usize, 2161864u32, 4294942720u32, 2161868u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161872u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967148u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020fcd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2161876u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2161880u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2161884u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2161888u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2161892u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2161896u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2161900u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2161904u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2161908u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2161912u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2161916u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2161920u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2161924u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2161928u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2161932u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2161936u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2161940u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2161944u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2161948u32);
    emu.apc_no_count(1usize, 2161948u32, 4294942720u32, 2161952u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161956u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967064u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020fd24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2161960u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2161964u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2161968u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2161972u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2161976u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2161980u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2161984u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2161988u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2161992u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2161996u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2162000u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2162004u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2162008u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2162012u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2162016u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2162020u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2162024u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2162028u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2162032u32);
    emu.apc_no_count(1usize, 2162032u32, 4294942720u32, 2162036u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162040u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966980u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020fd78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2162044u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2162048u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2162052u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2162056u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2162060u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2162064u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2162068u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2162072u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2162076u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2162080u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2162084u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2162088u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2162092u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2162096u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2162100u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2162104u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2162108u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2162112u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2162116u32);
    emu.apc_no_count(1usize, 2162116u32, 4294942720u32, 2162120u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162124u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966896u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020fdcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2162128u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2162132u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2162136u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2162140u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2162144u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2162148u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2162152u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2162156u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2162160u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2162164u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2162168u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2162172u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2162176u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2162180u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2162184u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2162188u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2162192u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2162196u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2162200u32);
    emu.apc_no_count(1usize, 2162200u32, 4294942720u32, 2162204u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162208u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966812u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020fe20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2162212u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2162216u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2162220u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2162224u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2162228u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2162232u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2162236u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2162240u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2162244u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2162248u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2162252u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2162256u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2162260u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2162264u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2162268u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2162272u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2162276u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2162280u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2162284u32);
    emu.apc_no_count(1usize, 2162284u32, 4294942720u32, 2162288u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162292u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966728u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020fe74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2162296u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2162300u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2162304u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2162308u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2162312u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2162316u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2162320u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2162324u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2162328u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2162332u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2162336u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2162340u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2162344u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2162348u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2162352u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2162356u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2162360u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2162364u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2162368u32);
    emu.apc_no_count(1usize, 2162368u32, 4294942720u32, 2162372u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162376u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966644u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020fec8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2162380u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2162384u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2162388u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2162392u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2162396u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2162400u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2162404u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2162408u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2162412u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2162416u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2162420u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2162424u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2162428u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2162432u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2162436u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2162440u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2162444u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2162448u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2162452u32);
    emu.apc_no_count(1usize, 2162452u32, 4294942720u32, 2162456u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162460u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966560u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020ff1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2162464u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2162468u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2162472u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2162476u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2162480u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2162484u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2162488u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2162492u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2162496u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2162500u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2162504u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2162508u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2162512u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2162516u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2162520u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2162524u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2162528u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2162532u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2162536u32);
    emu.apc_no_count(1usize, 2162536u32, 4294942720u32, 2162540u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162544u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966476u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0020ff70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2162548u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2162552u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2162556u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2162560u32)?;
    emu.lw_no_count(14usize, 2usize, 192u32, 2162564u32)?;
    emu.lw_no_count(15usize, 2usize, 196u32, 2162568u32)?;
    emu.lw_no_count(16usize, 2usize, 200u32, 2162572u32)?;
    emu.lw_no_count(17usize, 2usize, 204u32, 2162576u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2162580u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2162584u32)?;
    emu.sw_no_count(14usize, 2usize, 160u32, 2162588u32)?;
    emu.sw_no_count(17usize, 2usize, 172u32, 2162592u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2162596u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2162600u32)?;
    emu.sw_no_count(14usize, 2usize, 192u32, 2162604u32)?;
    emu.sw_no_count(15usize, 2usize, 196u32, 2162608u32)?;
    emu.sw_no_count(16usize, 2usize, 200u32, 2162612u32)?;
    emu.sw_no_count(17usize, 2usize, 204u32, 2162616u32)?;
    emu.sw_no_count(10usize, 2usize, 208u32, 2162620u32)?;
    emu.sw_no_count(11usize, 2usize, 212u32, 2162624u32)?;
    emu.sw_no_count(12usize, 2usize, 216u32, 2162628u32)?;
    emu.sw_no_count(13usize, 2usize, 220u32, 2162632u32)?;
    emu.adi_no_count(10usize, 2usize, 160u32, 2162636u32);
    emu.adi_no_count(11usize, 2usize, 192u32, 2162640u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2162644u32);
    emu.apc_no_count(1usize, 2162644u32, 4294942720u32, 2162648u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162652u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966368u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020ffdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 96u32, 2162656u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2162656u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ffe0));
}
#[inline(always)]
pub fn block_0x0020ffe0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 192u32, 2162660u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2162664u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2162668u32);
    emu.apc_no_count(1usize, 2162668u32, 4294942720u32, 2162672u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162676u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966344u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020fff4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2162680u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2162684u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2162688u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2162692u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2162696u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2162700u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2162704u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2162708u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2162712u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2162716u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2162720u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2162724u32)?;
    emu.adi_no_count(18usize, 18usize, 4294967295u32, 2162728u32);
    emu.sw_no_count(10usize, 2usize, 160u32, 2162732u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2162736u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2162740u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2162744u32)?;
    emu.add_memory_rw_events(17usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2162656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ffe0));
    } else {
        emu.pc = 2162748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021003c));
    }
}
#[inline]
pub fn block_0x0021003c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 176u32, 2162752u32)?;
    emu.lw_no_count(11usize, 2usize, 180u32, 2162756u32)?;
    emu.lw_no_count(12usize, 2usize, 184u32, 2162760u32)?;
    emu.lw_no_count(13usize, 2usize, 188u32, 2162764u32)?;
    emu.sw_no_count(10usize, 2usize, 208u32, 2162768u32)?;
    emu.sw_no_count(11usize, 2usize, 212u32, 2162772u32)?;
    emu.sw_no_count(12usize, 2usize, 216u32, 2162776u32)?;
    emu.sw_no_count(13usize, 2usize, 220u32, 2162780u32)?;
    emu.lw_no_count(10usize, 2usize, 160u32, 2162784u32)?;
    emu.lw_no_count(11usize, 2usize, 164u32, 2162788u32)?;
    emu.lw_no_count(12usize, 2usize, 168u32, 2162792u32)?;
    emu.lw_no_count(13usize, 2usize, 172u32, 2162796u32)?;
    emu.sw_no_count(10usize, 2usize, 192u32, 2162800u32)?;
    emu.sw_no_count(11usize, 2usize, 196u32, 2162804u32)?;
    emu.sw_no_count(12usize, 2usize, 200u32, 2162808u32)?;
    emu.sw_no_count(13usize, 2usize, 204u32, 2162812u32)?;
    emu.adi_no_count(10usize, 2usize, 160u32, 2162816u32);
    emu.adi_no_count(11usize, 2usize, 192u32, 2162820u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2162824u32);
    emu.apc_no_count(1usize, 2162824u32, 4294942720u32, 2162828u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162832u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966188u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00210090(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 94u32, 2162836u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2162836u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210094));
}
#[inline(always)]
pub fn block_0x00210094(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 192u32, 2162840u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2162844u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2162848u32);
    emu.apc_no_count(1usize, 2162848u32, 4294942720u32, 2162852u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162856u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966164u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002100a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2162860u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2162864u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2162868u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2162872u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2162876u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2162880u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2162884u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2162888u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2162892u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2162896u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2162900u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2162904u32)?;
    emu.adi_no_count(18usize, 18usize, 4294967295u32, 2162908u32);
    emu.sw_no_count(10usize, 2usize, 160u32, 2162912u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2162916u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2162920u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2162924u32)?;
    emu.add_memory_rw_events(17usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2162836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210094));
    } else {
        emu.pc = 2162928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002100f0));
    }
}
#[inline(never)]
pub fn block_0x002100f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 29u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 176u32, 2162932u32)?;
    emu.lw_no_count(11usize, 2usize, 180u32, 2162936u32)?;
    emu.lw_no_count(12usize, 2usize, 184u32, 2162940u32)?;
    emu.lw_no_count(13usize, 2usize, 188u32, 2162944u32)?;
    emu.lw_no_count(14usize, 2usize, 160u32, 2162948u32)?;
    emu.lw_no_count(15usize, 2usize, 164u32, 2162952u32)?;
    emu.lw_no_count(16usize, 2usize, 168u32, 2162956u32)?;
    emu.lw_no_count(17usize, 2usize, 172u32, 2162960u32)?;
    emu.sw_no_count(10usize, 2usize, 144u32, 2162964u32)?;
    emu.sw_no_count(11usize, 2usize, 148u32, 2162968u32)?;
    emu.sw_no_count(12usize, 2usize, 152u32, 2162972u32)?;
    emu.sw_no_count(13usize, 2usize, 156u32, 2162976u32)?;
    emu.sw_no_count(14usize, 2usize, 128u32, 2162980u32)?;
    emu.sw_no_count(15usize, 2usize, 132u32, 2162984u32)?;
    emu.sw_no_count(16usize, 2usize, 136u32, 2162988u32)?;
    emu.sw_no_count(17usize, 2usize, 140u32, 2162992u32)?;
    emu.sw_no_count(10usize, 8usize, 16u32, 2162996u32)?;
    emu.sw_no_count(11usize, 8usize, 20u32, 2163000u32)?;
    emu.sw_no_count(12usize, 8usize, 24u32, 2163004u32)?;
    emu.sw_no_count(13usize, 8usize, 28u32, 2163008u32)?;
    emu.sw_no_count(14usize, 8usize, 0u32, 2163012u32)?;
    emu.sw_no_count(15usize, 8usize, 4u32, 2163016u32)?;
    emu.sw_no_count(16usize, 8usize, 8u32, 2163020u32)?;
    emu.sw_no_count(17usize, 8usize, 12u32, 2163024u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2163028u32);
    emu.adi_no_count(11usize, 2usize, 128u32, 2163032u32);
    emu.adi_no_count(12usize, 2usize, 128u32, 2163036u32);
    emu.apc_no_count(1usize, 2163036u32, 4294942720u32, 2163040u32);
    emu.add_memory_rw_events(29usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2163044u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965976u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00210164(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 35u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 192u32, 2163048u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2163052u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2163056u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2163060u32)?;
    emu.lw_no_count(14usize, 2usize, 208u32, 2163064u32)?;
    emu.lw_no_count(15usize, 2usize, 212u32, 2163068u32)?;
    emu.lw_no_count(16usize, 2usize, 216u32, 2163072u32)?;
    emu.lw_no_count(17usize, 2usize, 220u32, 2163076u32)?;
    emu.lw_no_count(5usize, 9usize, 0u32, 2163080u32)?;
    emu.lw_no_count(6usize, 9usize, 4u32, 2163084u32)?;
    emu.lw_no_count(7usize, 9usize, 8u32, 2163088u32)?;
    emu.lw_no_count(28usize, 9usize, 12u32, 2163092u32)?;
    emu.lw_no_count(29usize, 9usize, 16u32, 2163096u32)?;
    emu.lw_no_count(30usize, 9usize, 20u32, 2163100u32)?;
    emu.lw_no_count(31usize, 9usize, 24u32, 2163104u32)?;
    emu.lw_no_count(9usize, 9usize, 28u32, 2163108u32)?;
    emu.xrr_no_count(11usize, 6usize, 11usize, 2163112u32);
    emu.xrr_no_count(10usize, 5usize, 10usize, 2163116u32);
    emu.xrr_no_count(12usize, 7usize, 12usize, 2163120u32);
    emu.xrr_no_count(13usize, 28usize, 13usize, 2163124u32);
    emu.xrr_no_count(14usize, 29usize, 14usize, 2163128u32);
    emu.xrr_no_count(15usize, 30usize, 15usize, 2163132u32);
    emu.xrr_no_count(16usize, 31usize, 16usize, 2163136u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2163140u32);
    emu.orr_no_count(12usize, 12usize, 13usize, 2163144u32);
    emu.orr_no_count(14usize, 14usize, 15usize, 2163148u32);
    emu.orr_no_count(10usize, 10usize, 12usize, 2163152u32);
    emu.orr_no_count(11usize, 14usize, 16usize, 2163156u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2163160u32);
    emu.xrr_no_count(11usize, 9usize, 17usize, 2163164u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2163168u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2163172u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2163176u32);
    emu.apc_no_count(1usize, 2163176u32, 24576u32, 2163180u32);
    emu.add_memory_rw_events(35usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2163184u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(592u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002101f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(10usize, 8usize, 32u32, 2163188u32);
    emu.lw_no_count(1usize, 2usize, 236u32, 2163192u32)?;
    emu.lw_no_count(8usize, 2usize, 232u32, 2163196u32)?;
    emu.lw_no_count(9usize, 2usize, 228u32, 2163200u32)?;
    emu.lw_no_count(18usize, 2usize, 224u32, 2163204u32)?;
    emu.adi_no_count(2usize, 2usize, 240u32, 2163208u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2163212u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(0u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0021020c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 139u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967216u32, 2163216u32);
    emu.sw_no_count(1usize, 2usize, 76u32, 2163220u32)?;
    emu.sw_no_count(8usize, 2usize, 72u32, 2163224u32)?;
    emu.sw_no_count(9usize, 2usize, 68u32, 2163228u32)?;
    emu.sw_no_count(18usize, 2usize, 64u32, 2163232u32)?;
    emu.sw_no_count(19usize, 2usize, 60u32, 2163236u32)?;
    emu.sw_no_count(20usize, 2usize, 56u32, 2163240u32)?;
    emu.sw_no_count(21usize, 2usize, 52u32, 2163244u32)?;
    emu.sw_no_count(22usize, 2usize, 48u32, 2163248u32)?;
    emu.sw_no_count(23usize, 2usize, 44u32, 2163252u32)?;
    emu.sw_no_count(24usize, 2usize, 40u32, 2163256u32)?;
    emu.sw_no_count(25usize, 2usize, 36u32, 2163260u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2163264u32);
    emu.lbu_no_count(10usize, 11usize, 0u32, 2163268u32);
    emu.lbu_no_count(12usize, 11usize, 1u32, 2163272u32);
    emu.lbu_no_count(13usize, 11usize, 2u32, 2163276u32);
    emu.lbu_no_count(14usize, 11usize, 3u32, 2163280u32);
    emu.lbu_no_count(15usize, 11usize, 4u32, 2163284u32);
    emu.lbu_no_count(16usize, 11usize, 5u32, 2163288u32);
    emu.lbu_no_count(17usize, 11usize, 6u32, 2163292u32);
    emu.lbu_no_count(5usize, 11usize, 7u32, 2163296u32);
    emu.lbu_no_count(6usize, 11usize, 8u32, 2163300u32);
    emu.lbu_no_count(7usize, 11usize, 9u32, 2163304u32);
    emu.lbu_no_count(28usize, 11usize, 10u32, 2163308u32);
    emu.lbu_no_count(29usize, 11usize, 11u32, 2163312u32);
    emu.lbu_no_count(30usize, 11usize, 12u32, 2163316u32);
    emu.lbu_no_count(31usize, 11usize, 13u32, 2163320u32);
    emu.lbu_no_count(9usize, 11usize, 14u32, 2163324u32);
    emu.lbu_no_count(18usize, 11usize, 15u32, 2163328u32);
    emu.sli_no_count(13usize, 13usize, 8u32, 2163332u32);
    emu.sli_no_count(12usize, 12usize, 16u32, 2163336u32);
    emu.sli_no_count(19usize, 10usize, 24u32, 2163340u32);
    emu.sli_no_count(17usize, 17usize, 8u32, 2163344u32);
    emu.orr_no_count(10usize, 13usize, 14usize, 2163348u32);
    emu.orr_no_count(12usize, 19usize, 12usize, 2163352u32);
    emu.orr_no_count(13usize, 17usize, 5usize, 2163356u32);
    emu.lbu_no_count(14usize, 11usize, 16u32, 2163360u32);
    emu.lbu_no_count(17usize, 11usize, 17u32, 2163364u32);
    emu.lbu_no_count(5usize, 11usize, 18u32, 2163368u32);
    emu.lbu_no_count(19usize, 11usize, 19u32, 2163372u32);
    emu.sli_no_count(16usize, 16usize, 16u32, 2163376u32);
    emu.sli_no_count(15usize, 15usize, 24u32, 2163380u32);
    emu.sli_no_count(28usize, 28usize, 8u32, 2163384u32);
    emu.sli_no_count(7usize, 7usize, 16u32, 2163388u32);
    emu.sli_no_count(6usize, 6usize, 24u32, 2163392u32);
    emu.sli_no_count(9usize, 9usize, 8u32, 2163396u32);
    emu.orr_no_count(15usize, 15usize, 16usize, 2163400u32);
    emu.orr_no_count(16usize, 28usize, 29usize, 2163404u32);
    emu.orr_no_count(6usize, 6usize, 7usize, 2163408u32);
    emu.orr_no_count(7usize, 9usize, 18usize, 2163412u32);
    emu.lbu_no_count(28usize, 11usize, 20u32, 2163416u32);
    emu.lbu_no_count(29usize, 11usize, 21u32, 2163420u32);
    emu.lbu_no_count(9usize, 11usize, 22u32, 2163424u32);
    emu.lbu_no_count(18usize, 11usize, 23u32, 2163428u32);
    emu.sli_no_count(31usize, 31usize, 16u32, 2163432u32);
    emu.sli_no_count(30usize, 30usize, 24u32, 2163436u32);
    emu.sli_no_count(5usize, 5usize, 8u32, 2163440u32);
    emu.sli_no_count(17usize, 17usize, 16u32, 2163444u32);
    emu.sli_no_count(14usize, 14usize, 24u32, 2163448u32);
    emu.sli_no_count(9usize, 9usize, 8u32, 2163452u32);
    emu.orr_no_count(30usize, 30usize, 31usize, 2163456u32);
    emu.orr_no_count(5usize, 5usize, 19usize, 2163460u32);
    emu.orr_no_count(14usize, 14usize, 17usize, 2163464u32);
    emu.orr_no_count(17usize, 9usize, 18usize, 2163468u32);
    emu.lbu_no_count(31usize, 11usize, 24u32, 2163472u32);
    emu.lbu_no_count(9usize, 11usize, 25u32, 2163476u32);
    emu.lbu_no_count(18usize, 11usize, 26u32, 2163480u32);
    emu.lbu_no_count(19usize, 11usize, 27u32, 2163484u32);
    emu.sli_no_count(29usize, 29usize, 16u32, 2163488u32);
    emu.sli_no_count(28usize, 28usize, 24u32, 2163492u32);
    emu.sli_no_count(18usize, 18usize, 8u32, 2163496u32);
    emu.orr_no_count(28usize, 28usize, 29usize, 2163500u32);
    emu.orr_no_count(29usize, 18usize, 19usize, 2163504u32);
    emu.lbu_no_count(18usize, 11usize, 30u32, 2163508u32);
    emu.lbu_no_count(19usize, 11usize, 31u32, 2163512u32);
    emu.sli_no_count(9usize, 9usize, 16u32, 2163516u32);
    emu.sli_no_count(31usize, 31usize, 24u32, 2163520u32);
    emu.orr_no_count(31usize, 31usize, 9usize, 2163524u32);
    emu.lbu_no_count(9usize, 11usize, 29u32, 2163528u32);
    emu.lbu_no_count(11usize, 11usize, 28u32, 2163532u32);
    emu.sli_no_count(18usize, 18usize, 8u32, 2163536u32);
    emu.orr_no_count(24usize, 18usize, 19usize, 2163540u32);
    emu.sli_no_count(9usize, 9usize, 16u32, 2163544u32);
    emu.sli_no_count(11usize, 11usize, 24u32, 2163548u32);
    emu.orr_no_count(11usize, 11usize, 9usize, 2163552u32);
    emu.orr_no_count(18usize, 12usize, 10usize, 2163556u32);
    emu.orr_no_count(19usize, 15usize, 13usize, 2163560u32);
    emu.orr_no_count(20usize, 6usize, 16usize, 2163564u32);
    emu.orr_no_count(21usize, 30usize, 7usize, 2163568u32);
    emu.orr_no_count(22usize, 14usize, 5usize, 2163572u32);
    emu.orr_no_count(23usize, 28usize, 17usize, 2163576u32);
    emu.orr_no_count(25usize, 31usize, 29usize, 2163580u32);
    emu.orr_no_count(24usize, 11usize, 24usize, 2163584u32);
    emu.adi_no_count(10usize, 24usize, 1u32, 2163588u32);
    emu.sltiu_no_count(10usize, 10usize, 1u32, 2163592u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2163596u32);
    emu.adr_no_count(11usize, 25usize, 10usize, 2163600u32);
    emu.sltru_no_count(12usize, 11usize, 25usize, 2163604u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2163608u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2163612u32);
    emu.sltiu_no_count(11usize, 11usize, 1u32, 2163616u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2163620u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2163624u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2163628u32);
    emu.adr_no_count(11usize, 23usize, 10usize, 2163632u32);
    emu.sltru_no_count(12usize, 11usize, 23usize, 2163636u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2163640u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2163644u32);
    emu.sltiu_no_count(11usize, 11usize, 1u32, 2163648u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2163652u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2163656u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2163660u32);
    emu.adr_no_count(11usize, 10usize, 22usize, 2163664u32);
    emu.sltru_no_count(11usize, 11usize, 10usize, 2163668u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2163672u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2163676u32);
    emu.adr_no_count(11usize, 10usize, 21usize, 2163680u32);
    emu.sltru_no_count(11usize, 11usize, 10usize, 2163684u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2163688u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2163692u32);
    emu.adr_no_count(11usize, 10usize, 20usize, 2163696u32);
    emu.sltru_no_count(11usize, 11usize, 10usize, 2163700u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2163704u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2163708u32);
    emu.adr_no_count(11usize, 19usize, 10usize, 2163712u32);
    emu.sltru_no_count(12usize, 11usize, 19usize, 2163716u32);
    emu.sltiu_no_count(11usize, 11usize, 1u32, 2163720u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2163724u32);
    emu.sbr_no_count(10usize, 10usize, 11usize, 2163728u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2163732u32);
    emu.adr_no_count(11usize, 18usize, 10usize, 2163736u32);
    emu.sltru_no_count(12usize, 11usize, 18usize, 2163740u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2163744u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2163748u32);
    emu.sltiu_no_count(11usize, 11usize, 1u32, 2163752u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2163756u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2163760u32);
    emu.apc_no_count(1usize, 2163760u32, 24576u32, 2163764u32);
    emu.add_memory_rw_events(139usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2163768u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(8u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00210438(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2163772u32);
    emu.sw_no_count(24usize, 2usize, 4u32, 2163776u32)?;
    emu.sw_no_count(25usize, 2usize, 8u32, 2163780u32)?;
    emu.sw_no_count(23usize, 2usize, 12u32, 2163784u32)?;
    emu.sw_no_count(22usize, 2usize, 16u32, 2163788u32)?;
    emu.sw_no_count(21usize, 2usize, 20u32, 2163792u32)?;
    emu.sw_no_count(20usize, 2usize, 24u32, 2163796u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2163800u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2163804u32)?;
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2163808u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966896u32, 2163812u32);
    emu.adi_no_count(11usize, 2usize, 4u32, 2163816u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2163820u32);
    emu.apc_no_count(1usize, 2163820u32, 4294938624u32, 2163824u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2163828u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1992u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00210474(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(9usize, 8usize, 32u32, 2163832u32);
    emu.lw_no_count(1usize, 2usize, 76u32, 2163836u32)?;
    emu.lw_no_count(8usize, 2usize, 72u32, 2163840u32)?;
    emu.lw_no_count(9usize, 2usize, 68u32, 2163844u32)?;
    emu.lw_no_count(18usize, 2usize, 64u32, 2163848u32)?;
    emu.lw_no_count(19usize, 2usize, 60u32, 2163852u32)?;
    emu.lw_no_count(20usize, 2usize, 56u32, 2163856u32)?;
    emu.lw_no_count(21usize, 2usize, 52u32, 2163860u32)?;
    emu.lw_no_count(22usize, 2usize, 48u32, 2163864u32)?;
    emu.lw_no_count(23usize, 2usize, 44u32, 2163868u32)?;
    emu.lw_no_count(24usize, 2usize, 40u32, 2163872u32)?;
    emu.lw_no_count(25usize, 2usize, 36u32, 2163876u32)?;
    emu.adi_no_count(2usize, 2usize, 80u32, 2163880u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2163884u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(0u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002104ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 11usize, 0u32, 2163888u32)?;
    emu.lw_no_count(5usize, 11usize, 4u32, 2163892u32)?;
    emu.lw_no_count(14usize, 11usize, 12u32, 2163896u32)?;
    emu.lw_no_count(12usize, 11usize, 16u32, 2163900u32)?;
    emu.lw_no_count(13usize, 11usize, 20u32, 2163904u32)?;
    emu.lw_no_count(7usize, 11usize, 24u32, 2163908u32)?;
    emu.lw_no_count(6usize, 11usize, 28u32, 2163912u32)?;
    emu.adr_no_count(14usize, 16usize, 14usize, 2163916u32);
    emu.sltru_no_count(15usize, 14usize, 16usize, 2163920u32);
    emu.adr_no_count(17usize, 5usize, 15usize, 2163924u32);
    emu.sltru_no_count(15usize, 17usize, 5usize, 2163928u32);
    emu.adr_no_count(12usize, 17usize, 12usize, 2163932u32);
    emu.sltru_no_count(17usize, 12usize, 17usize, 2163936u32);
    emu.adr_no_count(13usize, 15usize, 13usize, 2163940u32);
    emu.adr_no_count(13usize, 13usize, 17usize, 2163944u32);
    emu.add_memory_rw_events(15usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2163952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002104f0));
    } else {
        emu.pc = 2163948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002104ec));
    }
}
#[inline(always)]
pub fn block_0x002104ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(17usize, 13usize, 15usize, 2163952u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2163952u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002104f0));
}
#[inline]
pub fn block_0x002104f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 11usize, 8u32, 2163956u32)?;
    emu.sbr_no_count(28usize, 0usize, 16usize, 2163960u32);
    emu.sbr_no_count(29usize, 5usize, 16usize, 2163964u32);
    emu.adi_no_count(11usize, 0usize, 4294967295u32, 2163968u32);
    emu.sltru_no_count(28usize, 29usize, 28usize, 2163972u32);
    emu.mulhu_no_count(30usize, 16usize, 11usize, 2163976u32);
    emu.sbr_no_count(30usize, 30usize, 5usize, 2163980u32);
    emu.adr_no_count(28usize, 30usize, 28usize, 2163984u32);
    emu.sbr_no_count(30usize, 0usize, 5usize, 2163988u32);
    emu.adr_no_count(7usize, 16usize, 7usize, 2163992u32);
    emu.mulhu_no_count(31usize, 5usize, 11usize, 2163996u32);
    emu.adr_no_count(5usize, 29usize, 6usize, 2164000u32);
    emu.sltru_no_count(16usize, 7usize, 16usize, 2164004u32);
    emu.sltru_no_count(6usize, 28usize, 30usize, 2164008u32);
    emu.adr_no_count(5usize, 5usize, 16usize, 2164012u32);
    emu.adr_no_count(6usize, 31usize, 6usize, 2164016u32);
    emu.add_memory_rw_events(16usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2164024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210538));
    } else {
        emu.pc = 2164020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210534));
    }
}
#[inline(always)]
pub fn block_0x00210534(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 5usize, 29usize, 2164024u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2164024u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210538));
}
#[inline]
pub fn block_0x00210538(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(30usize, 28usize, 16usize, 2164028u32);
    emu.adr_no_count(17usize, 7usize, 17usize, 2164032u32);
    emu.adr_no_count(16usize, 13usize, 15usize, 2164036u32);
    emu.sltru_no_count(28usize, 30usize, 28usize, 2164040u32);
    emu.sltru_no_count(7usize, 17usize, 7usize, 2164044u32);
    emu.sltru_no_count(13usize, 16usize, 13usize, 2164048u32);
    emu.adr_no_count(28usize, 6usize, 28usize, 2164052u32);
    emu.adr_no_count(6usize, 5usize, 7usize, 2164056u32);
    emu.adr_no_count(13usize, 14usize, 13usize, 2164060u32);
    emu.sltru_no_count(5usize, 6usize, 5usize, 2164064u32);
    emu.anr_no_count(29usize, 7usize, 5usize, 2164068u32);
    emu.sltru_no_count(7usize, 13usize, 14usize, 2164072u32);
    emu.adr_no_count(13usize, 17usize, 13usize, 2164076u32);
    emu.sltru_no_count(5usize, 13usize, 17usize, 2164080u32);
    emu.adr_no_count(17usize, 6usize, 7usize, 2164084u32);
    emu.adr_no_count(29usize, 30usize, 29usize, 2164088u32);
    emu.sltru_no_count(7usize, 29usize, 30usize, 2164092u32);
    emu.adr_no_count(17usize, 17usize, 5usize, 2164096u32);
    emu.adr_no_count(7usize, 28usize, 7usize, 2164100u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2164108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021058c));
    } else {
        emu.pc = 2164104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210588));
    }
}
#[inline(always)]
pub fn block_0x00210588(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 17usize, 6usize, 2164108u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2164108u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021058c));
}
#[inline(never)]
pub fn block_0x0021058c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2164112u32);
    emu.sw_no_count(8usize, 2usize, 44u32, 2164116u32)?;
    emu.sw_no_count(9usize, 2usize, 40u32, 2164120u32)?;
    emu.sw_no_count(18usize, 2usize, 36u32, 2164124u32)?;
    emu.sw_no_count(19usize, 2usize, 32u32, 2164128u32)?;
    emu.sw_no_count(20usize, 2usize, 28u32, 2164132u32)?;
    emu.sw_no_count(21usize, 2usize, 24u32, 2164136u32)?;
    emu.sw_no_count(22usize, 2usize, 20u32, 2164140u32)?;
    emu.sw_no_count(23usize, 2usize, 16u32, 2164144u32)?;
    emu.sw_no_count(24usize, 2usize, 12u32, 2164148u32)?;
    emu.sw_no_count(25usize, 2usize, 8u32, 2164152u32)?;
    emu.sw_no_count(26usize, 2usize, 4u32, 2164156u32)?;
    emu.sw_no_count(27usize, 2usize, 0u32, 2164160u32)?;
    emu.sbr_no_count(6usize, 0usize, 15usize, 2164164u32);
    emu.sbr_no_count(30usize, 14usize, 15usize, 2164168u32);
    emu.mulhu_no_count(28usize, 15usize, 11usize, 2164172u32);
    emu.sltru_no_count(6usize, 30usize, 6usize, 2164176u32);
    emu.sbr_no_count(28usize, 28usize, 14usize, 2164180u32);
    emu.adr_no_count(6usize, 28usize, 6usize, 2164184u32);
    emu.sbr_no_count(31usize, 0usize, 14usize, 2164188u32);
    emu.mulhu_no_count(8usize, 14usize, 11usize, 2164192u32);
    emu.adr_no_count(28usize, 29usize, 15usize, 2164196u32);
    emu.sltru_no_count(15usize, 28usize, 29usize, 2164200u32);
    emu.adr_no_count(14usize, 7usize, 30usize, 2164204u32);
    emu.sltru_no_count(29usize, 6usize, 31usize, 2164208u32);
    emu.adr_no_count(14usize, 14usize, 15usize, 2164212u32);
    emu.adr_no_count(29usize, 8usize, 29usize, 2164216u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2164224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210600));
    } else {
        emu.pc = 2164220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002105fc));
    }
}
#[inline(always)]
pub fn block_0x002105fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 14usize, 7usize, 2164224u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2164224u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210600));
}
#[inline]
pub fn block_0x00210600(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(7usize, 6usize, 15usize, 2164228u32);
    emu.adr_no_count(5usize, 28usize, 5usize, 2164232u32);
    emu.adr_no_count(15usize, 17usize, 12usize, 2164236u32);
    emu.sltru_no_count(6usize, 7usize, 6usize, 2164240u32);
    emu.sltru_no_count(28usize, 5usize, 28usize, 2164244u32);
    emu.sltru_no_count(17usize, 15usize, 17usize, 2164248u32);
    emu.adr_no_count(30usize, 29usize, 6usize, 2164252u32);
    emu.adr_no_count(29usize, 14usize, 28usize, 2164256u32);
    emu.adr_no_count(17usize, 16usize, 17usize, 2164260u32);
    emu.sltru_no_count(14usize, 29usize, 14usize, 2164264u32);
    emu.anr_no_count(28usize, 28usize, 14usize, 2164268u32);
    emu.sltru_no_count(6usize, 17usize, 16usize, 2164272u32);
    emu.adr_no_count(14usize, 17usize, 5usize, 2164276u32);
    emu.sltru_no_count(5usize, 14usize, 17usize, 2164280u32);
    emu.adr_no_count(17usize, 6usize, 29usize, 2164284u32);
    emu.adr_no_count(29usize, 7usize, 28usize, 2164288u32);
    emu.sltru_no_count(7usize, 29usize, 7usize, 2164292u32);
    emu.adr_no_count(17usize, 17usize, 5usize, 2164296u32);
    emu.adr_no_count(7usize, 30usize, 7usize, 2164300u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2164308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210654));
    } else {
        emu.pc = 2164304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210650));
    }
}
#[inline(always)]
pub fn block_0x00210650(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 17usize, 6usize, 2164308u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2164308u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210654));
}
#[inline]
pub fn block_0x00210654(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(30usize, 16usize, 12usize, 2164312u32);
    emu.sbr_no_count(6usize, 0usize, 12usize, 2164316u32);
    emu.mulhu_no_count(28usize, 12usize, 11usize, 2164320u32);
    emu.sltru_no_count(6usize, 30usize, 6usize, 2164324u32);
    emu.sbr_no_count(28usize, 28usize, 16usize, 2164328u32);
    emu.adr_no_count(6usize, 28usize, 6usize, 2164332u32);
    emu.sbr_no_count(31usize, 0usize, 16usize, 2164336u32);
    emu.mulhu_no_count(8usize, 16usize, 11usize, 2164340u32);
    emu.adr_no_count(28usize, 29usize, 12usize, 2164344u32);
    emu.sltru_no_count(12usize, 28usize, 29usize, 2164348u32);
    emu.adr_no_count(16usize, 7usize, 30usize, 2164352u32);
    emu.sltru_no_count(29usize, 6usize, 31usize, 2164356u32);
    emu.adr_no_count(16usize, 16usize, 12usize, 2164360u32);
    emu.adr_no_count(29usize, 8usize, 29usize, 2164364u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2164372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210694));
    } else {
        emu.pc = 2164368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210690));
    }
}
#[inline(always)]
pub fn block_0x00210690(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 16usize, 7usize, 2164372u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2164372u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210694));
}
#[inline]
pub fn block_0x00210694(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(30usize, 6usize, 12usize, 2164376u32);
    emu.adr_no_count(5usize, 28usize, 5usize, 2164380u32);
    emu.adr_no_count(12usize, 17usize, 13usize, 2164384u32);
    emu.sltru_no_count(6usize, 30usize, 6usize, 2164388u32);
    emu.sltru_no_count(7usize, 5usize, 28usize, 2164392u32);
    emu.sltru_no_count(17usize, 12usize, 17usize, 2164396u32);
    emu.adr_no_count(6usize, 29usize, 6usize, 2164400u32);
    emu.adr_no_count(29usize, 16usize, 7usize, 2164404u32);
    emu.adr_no_count(31usize, 15usize, 17usize, 2164408u32);
    emu.sltru_no_count(16usize, 29usize, 16usize, 2164412u32);
    emu.sltru_no_count(28usize, 31usize, 15usize, 2164416u32);
    emu.adr_no_count(17usize, 31usize, 5usize, 2164420u32);
    emu.anr_no_count(7usize, 7usize, 16usize, 2164424u32);
    emu.sltru_no_count(5usize, 17usize, 31usize, 2164428u32);
    emu.adr_no_count(16usize, 28usize, 5usize, 2164432u32);
    emu.adr_no_count(16usize, 16usize, 29usize, 2164436u32);
    emu.adr_no_count(7usize, 30usize, 7usize, 2164440u32);
    emu.sltru_no_count(29usize, 7usize, 30usize, 2164444u32);
    emu.adr_no_count(6usize, 6usize, 29usize, 2164448u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2164456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002106e8));
    } else {
        emu.pc = 2164452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002106e4));
    }
}
#[inline(always)]
pub fn block_0x002106e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 16usize, 28usize, 2164456u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2164456u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002106e8));
}
#[inline]
pub fn block_0x002106e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(31usize, 15usize, 13usize, 2164460u32);
    emu.sbr_no_count(28usize, 0usize, 13usize, 2164464u32);
    emu.mulhu_no_count(29usize, 13usize, 11usize, 2164468u32);
    emu.sltru_no_count(28usize, 31usize, 28usize, 2164472u32);
    emu.sbr_no_count(29usize, 29usize, 15usize, 2164476u32);
    emu.adr_no_count(28usize, 29usize, 28usize, 2164480u32);
    emu.sbr_no_count(8usize, 0usize, 15usize, 2164484u32);
    emu.adr_no_count(29usize, 7usize, 13usize, 2164488u32);
    emu.sltru_no_count(30usize, 29usize, 7usize, 2164492u32);
    emu.adr_no_count(13usize, 6usize, 31usize, 2164496u32);
    emu.adr_no_count(13usize, 13usize, 30usize, 2164500u32);
    emu.sltru_no_count(7usize, 28usize, 8usize, 2164504u32);
    emu.mulhu_no_count(11usize, 15usize, 11usize, 2164508u32);
    emu.add_memory_rw_events(13usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2164516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210724));
    } else {
        emu.pc = 2164512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210720));
    }
}
#[inline(always)]
pub fn block_0x00210720(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(30usize, 13usize, 6usize, 2164516u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2164516u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210724));
}
#[inline]
pub fn block_0x00210724(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(7usize, 11usize, 7usize, 2164520u32);
    emu.adr_no_count(6usize, 28usize, 30usize, 2164524u32);
    emu.adr_no_count(15usize, 29usize, 5usize, 2164528u32);
    emu.adi_no_count(11usize, 14usize, 1u32, 2164532u32);
    emu.sltru_no_count(14usize, 6usize, 28usize, 2164536u32);
    emu.sltru_no_count(28usize, 15usize, 29usize, 2164540u32);
    emu.sltiu_no_count(29usize, 11usize, 1u32, 2164544u32);
    emu.adr_no_count(5usize, 13usize, 28usize, 2164548u32);
    emu.adr_no_count(12usize, 12usize, 29usize, 2164552u32);
    emu.sltru_no_count(13usize, 5usize, 13usize, 2164556u32);
    emu.orr_no_count(29usize, 11usize, 12usize, 2164560u32);
    emu.anr_no_count(31usize, 28usize, 13usize, 2164564u32);
    emu.sltiu_no_count(30usize, 29usize, 1u32, 2164568u32);
    emu.adi_no_count(30usize, 30usize, 4294967295u32, 2164572u32);
    emu.adr_no_count(28usize, 17usize, 30usize, 2164576u32);
    emu.sltru_no_count(13usize, 28usize, 17usize, 2164580u32);
    emu.adr_no_count(29usize, 16usize, 30usize, 2164584u32);
    emu.adr_no_count(29usize, 29usize, 13usize, 2164588u32);
    emu.adr_no_count(17usize, 6usize, 31usize, 2164592u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(29usize);
    if a == b {
        emu.pc = 2164600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210778));
    } else {
        emu.pc = 2164596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210774));
    }
}
#[inline(always)]
pub fn block_0x00210774(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(13usize, 29usize, 16usize, 2164600u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2164600u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210778));
}
#[inline]
pub fn block_0x00210778(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(7usize, 7usize, 14usize, 2164604u32);
    emu.adr_no_count(16usize, 30usize, 13usize, 2164608u32);
    emu.adi_no_count(13usize, 28usize, 1u32, 2164612u32);
    emu.sltru_no_count(14usize, 16usize, 30usize, 2164616u32);
    emu.adr_no_count(30usize, 30usize, 14usize, 2164620u32);
    emu.sltiu_no_count(14usize, 13usize, 1u32, 2164624u32);
    emu.adr_no_count(14usize, 29usize, 14usize, 2164628u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2164632u32);
    emu.sltru_no_count(6usize, 17usize, 6usize, 2164636u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2164648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002107a8));
    } else {
        emu.pc = 2164640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002107a0));
    }
}
#[inline(always)]
pub fn block_0x002107a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 14usize, 29usize, 2164644u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2164648u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2164652u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002107ac));
}
#[inline(always)]
pub fn block_0x002107a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 13usize, 28usize, 2164652u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2164652u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002107ac));
}
#[inline]
pub fn block_0x002107ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(29usize, 16usize, 1u32, 2164656u32);
    emu.sbr_no_count(29usize, 30usize, 29usize, 2164660u32);
    emu.adi_no_count(16usize, 16usize, 4294967295u32, 2164664u32);
    emu.adr_no_count(28usize, 16usize, 28usize, 2164668u32);
    emu.sltru_no_count(16usize, 28usize, 16usize, 2164672u32);
    emu.adr_no_count(16usize, 29usize, 16usize, 2164676u32);
    emu.sai_no_count(28usize, 16usize, 1055u32, 2164680u32);
    emu.adr_no_count(15usize, 28usize, 15usize, 2164684u32);
    emu.sltru_no_count(29usize, 15usize, 28usize, 2164688u32);
    emu.adr_no_count(16usize, 28usize, 5usize, 2164692u32);
    emu.adr_no_count(16usize, 16usize, 29usize, 2164696u32);
    emu.adr_no_count(7usize, 7usize, 6usize, 2164700u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2164708u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002107e4));
    } else {
        emu.pc = 2164704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002107e0));
    }
}
#[inline(always)]
pub fn block_0x002107e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(29usize, 16usize, 28usize, 2164708u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2164708u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002107e4));
}
#[inline]
pub fn block_0x002107e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(29usize, 28usize, 29usize, 2164712u32);
    emu.sltru_no_count(5usize, 29usize, 28usize, 2164716u32);
    emu.adr_no_count(5usize, 28usize, 5usize, 2164720u32);
    emu.sai_no_count(28usize, 5usize, 1055u32, 2164724u32);
    emu.adr_no_count(30usize, 7usize, 28usize, 2164728u32);
    emu.adr_no_count(6usize, 17usize, 28usize, 2164732u32);
    emu.sltru_no_count(5usize, 6usize, 17usize, 2164736u32);
    emu.adr_no_count(30usize, 30usize, 5usize, 2164740u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(30usize);
    if a == b {
        emu.pc = 2164748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021080c));
    } else {
        emu.pc = 2164744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210808));
    }
}
#[inline(always)]
pub fn block_0x00210808(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 30usize, 7usize, 2164748u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2164748u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021080c));
}
#[inline(always)]
pub fn block_0x0021080c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(5usize, 28usize, 5usize, 2164752u32);
    emu.sltru_no_count(17usize, 0usize, 6usize, 2164756u32);
    emu.sltru_no_count(7usize, 5usize, 28usize, 2164760u32);
    emu.adr_no_count(17usize, 30usize, 17usize, 2164764u32);
    emu.adr_no_count(28usize, 28usize, 7usize, 2164768u32);
    emu.adi_no_count(29usize, 6usize, 4294967295u32, 2164772u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2164784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210830));
    } else {
        emu.pc = 2164776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210828));
    }
}
#[inline(always)]
pub fn block_0x00210828(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(6usize, 17usize, 30usize, 2164780u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2164784u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2164788u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210834));
}
#[inline(always)]
pub fn block_0x00210830(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(6usize, 29usize, 6usize, 2164788u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2164788u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210834));
}
#[inline]
pub fn block_0x00210834(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(7usize, 5usize, 4294967295u32, 2164792u32);
    emu.sltiu_no_count(5usize, 5usize, 1u32, 2164796u32);
    emu.adr_no_count(6usize, 7usize, 6usize, 2164800u32);
    emu.sbr_no_count(5usize, 28usize, 5usize, 2164804u32);
    emu.sltru_no_count(6usize, 6usize, 7usize, 2164808u32);
    emu.adr_no_count(5usize, 5usize, 6usize, 2164812u32);
    emu.sai_no_count(30usize, 5usize, 1055u32, 2164816u32);
    emu.adr_no_count(11usize, 30usize, 11usize, 2164820u32);
    emu.sltru_no_count(5usize, 11usize, 30usize, 2164824u32);
    emu.adr_no_count(12usize, 30usize, 12usize, 2164828u32);
    emu.adr_no_count(12usize, 12usize, 5usize, 2164832u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(30usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2164840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210868));
    } else {
        emu.pc = 2164836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210864));
    }
}
#[inline(always)]
pub fn block_0x00210864(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 12usize, 30usize, 2164840u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2164840u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210868));
}
#[inline]
pub fn block_0x00210868(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(6usize, 30usize, 13usize, 2164844u32);
    emu.sltru_no_count(7usize, 6usize, 30usize, 2164848u32);
    emu.adr_no_count(13usize, 6usize, 5usize, 2164852u32);
    emu.adr_no_count(5usize, 14usize, 7usize, 2164856u32);
    emu.sltru_no_count(6usize, 13usize, 6usize, 2164860u32);
    emu.sltru_no_count(14usize, 0usize, 5usize, 2164864u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2164868u32);
    emu.anr_no_count(7usize, 14usize, 7usize, 2164872u32);
    emu.adr_no_count(14usize, 5usize, 6usize, 2164876u32);
    emu.sltru_no_count(5usize, 14usize, 5usize, 2164880u32);
    emu.anr_no_count(5usize, 6usize, 5usize, 2164884u32);
    emu.adr_no_count(5usize, 7usize, 5usize, 2164888u32);
    emu.sltru_no_count(7usize, 5usize, 7usize, 2164892u32);
    emu.adr_no_count(6usize, 5usize, 15usize, 2164896u32);
    emu.sltru_no_count(5usize, 6usize, 5usize, 2164900u32);
    emu.adr_no_count(16usize, 7usize, 16usize, 2164904u32);
    emu.adr_no_count(28usize, 16usize, 5usize, 2164908u32);
    emu.add_memory_rw_events(17usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2164916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002108b4));
    } else {
        emu.pc = 2164912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002108b0));
    }
}
#[inline(always)]
pub fn block_0x002108b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 28usize, 7usize, 2164916u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2164916u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002108b4));
}
#[inline(never)]
pub fn block_0x002108b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 78u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(7usize, 30usize, 1u32, 2164920u32);
    emu.sbr_no_count(16usize, 29usize, 30usize, 2164924u32);
    emu.adr_no_count(17usize, 30usize, 17usize, 2164928u32);
    emu.sri_no_count(8usize, 28usize, 24u32, 2164932u32);
    emu.sri_no_count(9usize, 6usize, 24u32, 2164936u32);
    emu.sri_no_count(31usize, 14usize, 24u32, 2164940u32);
    emu.sri_no_count(30usize, 13usize, 24u32, 2164944u32);
    emu.sri_no_count(29usize, 12usize, 24u32, 2164948u32);
    emu.sri_no_count(15usize, 11usize, 24u32, 2164952u32);
    emu.sri_no_count(18usize, 28usize, 8u32, 2164956u32);
    emu.sri_no_count(19usize, 28usize, 16u32, 2164960u32);
    emu.sri_no_count(20usize, 6usize, 8u32, 2164964u32);
    emu.sri_no_count(21usize, 6usize, 16u32, 2164968u32);
    emu.sri_no_count(22usize, 14usize, 8u32, 2164972u32);
    emu.sri_no_count(23usize, 14usize, 16u32, 2164976u32);
    emu.sri_no_count(24usize, 13usize, 8u32, 2164980u32);
    emu.sri_no_count(25usize, 13usize, 16u32, 2164984u32);
    emu.sri_no_count(26usize, 12usize, 8u32, 2164988u32);
    emu.sri_no_count(27usize, 12usize, 16u32, 2164992u32);
    emu.sb_no_count(8usize, 10usize, 8u32, 2164996u32);
    emu.sb_no_count(19usize, 10usize, 9u32, 2165000u32);
    emu.sb_no_count(18usize, 10usize, 10u32, 2165004u32);
    emu.sb_no_count(28usize, 10usize, 11u32, 2165008u32);
    emu.sri_no_count(28usize, 11usize, 8u32, 2165012u32);
    emu.sb_no_count(9usize, 10usize, 12u32, 2165016u32);
    emu.sb_no_count(21usize, 10usize, 13u32, 2165020u32);
    emu.sb_no_count(20usize, 10usize, 14u32, 2165024u32);
    emu.sb_no_count(6usize, 10usize, 15u32, 2165028u32);
    emu.sri_no_count(6usize, 11usize, 16u32, 2165032u32);
    emu.sltru_no_count(7usize, 16usize, 7usize, 2165036u32);
    emu.adr_no_count(5usize, 16usize, 5usize, 2165040u32);
    emu.sb_no_count(31usize, 10usize, 16u32, 2165044u32);
    emu.sb_no_count(23usize, 10usize, 17u32, 2165048u32);
    emu.sb_no_count(22usize, 10usize, 18u32, 2165052u32);
    emu.sb_no_count(14usize, 10usize, 19u32, 2165056u32);
    emu.sb_no_count(30usize, 10usize, 20u32, 2165060u32);
    emu.sb_no_count(25usize, 10usize, 21u32, 2165064u32);
    emu.sb_no_count(24usize, 10usize, 22u32, 2165068u32);
    emu.sb_no_count(13usize, 10usize, 23u32, 2165072u32);
    emu.sb_no_count(29usize, 10usize, 24u32, 2165076u32);
    emu.sb_no_count(27usize, 10usize, 25u32, 2165080u32);
    emu.sb_no_count(26usize, 10usize, 26u32, 2165084u32);
    emu.sb_no_count(12usize, 10usize, 27u32, 2165088u32);
    emu.adr_no_count(17usize, 17usize, 7usize, 2165092u32);
    emu.sltru_no_count(12usize, 5usize, 16usize, 2165096u32);
    emu.sri_no_count(13usize, 5usize, 24u32, 2165100u32);
    emu.sri_no_count(14usize, 5usize, 8u32, 2165104u32);
    emu.sri_no_count(16usize, 5usize, 16u32, 2165108u32);
    emu.adr_no_count(12usize, 17usize, 12usize, 2165112u32);
    emu.sb_no_count(13usize, 10usize, 4u32, 2165116u32);
    emu.sb_no_count(16usize, 10usize, 5u32, 2165120u32);
    emu.sb_no_count(14usize, 10usize, 6u32, 2165124u32);
    emu.sb_no_count(5usize, 10usize, 7u32, 2165128u32);
    emu.sri_no_count(13usize, 12usize, 24u32, 2165132u32);
    emu.sri_no_count(14usize, 12usize, 8u32, 2165136u32);
    emu.sri_no_count(16usize, 12usize, 16u32, 2165140u32);
    emu.sb_no_count(13usize, 10usize, 0u32, 2165144u32);
    emu.sb_no_count(16usize, 10usize, 1u32, 2165148u32);
    emu.sb_no_count(14usize, 10usize, 2u32, 2165152u32);
    emu.sb_no_count(12usize, 10usize, 3u32, 2165156u32);
    emu.sb_no_count(15usize, 10usize, 28u32, 2165160u32);
    emu.sb_no_count(6usize, 10usize, 29u32, 2165164u32);
    emu.sb_no_count(28usize, 10usize, 30u32, 2165168u32);
    emu.sb_no_count(11usize, 10usize, 31u32, 2165172u32);
    emu.lw_no_count(8usize, 2usize, 44u32, 2165176u32)?;
    emu.lw_no_count(9usize, 2usize, 40u32, 2165180u32)?;
    emu.lw_no_count(18usize, 2usize, 36u32, 2165184u32)?;
    emu.lw_no_count(19usize, 2usize, 32u32, 2165188u32)?;
    emu.lw_no_count(20usize, 2usize, 28u32, 2165192u32)?;
    emu.lw_no_count(21usize, 2usize, 24u32, 2165196u32)?;
    emu.lw_no_count(22usize, 2usize, 20u32, 2165200u32)?;
    emu.lw_no_count(23usize, 2usize, 16u32, 2165204u32)?;
    emu.lw_no_count(24usize, 2usize, 12u32, 2165208u32)?;
    emu.lw_no_count(25usize, 2usize, 8u32, 2165212u32)?;
    emu.lw_no_count(26usize, 2usize, 4u32, 2165216u32)?;
    emu.lw_no_count(27usize, 2usize, 0u32, 2165220u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2165224u32);
    emu.add_memory_rw_events(78usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2165228u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(0u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002109ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 10usize, 0u32, 2165232u32)?;
    emu.lw_no_count(17usize, 10usize, 4u32, 2165236u32)?;
    emu.lw_no_count(13usize, 10usize, 12u32, 2165240u32)?;
    emu.lw_no_count(11usize, 10usize, 16u32, 2165244u32)?;
    emu.lw_no_count(12usize, 10usize, 20u32, 2165248u32)?;
    emu.lw_no_count(7usize, 10usize, 24u32, 2165252u32)?;
    emu.lw_no_count(5usize, 10usize, 28u32, 2165256u32)?;
    emu.adr_no_count(13usize, 15usize, 13usize, 2165260u32);
    emu.sltru_no_count(14usize, 13usize, 15usize, 2165264u32);
    emu.adr_no_count(16usize, 17usize, 14usize, 2165268u32);
    emu.sltru_no_count(14usize, 16usize, 17usize, 2165272u32);
    emu.adr_no_count(11usize, 16usize, 11usize, 2165276u32);
    emu.sltru_no_count(16usize, 11usize, 16usize, 2165280u32);
    emu.adr_no_count(12usize, 14usize, 12usize, 2165284u32);
    emu.adr_no_count(12usize, 12usize, 16usize, 2165288u32);
    emu.add_memory_rw_events(15usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2165296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210a30));
    } else {
        emu.pc = 2165292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210a2c));
    }
}
#[inline(always)]
pub fn block_0x00210a2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 12usize, 14usize, 2165296u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2165296u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210a30));
}
#[inline]
pub fn block_0x00210a30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 10usize, 8u32, 2165300u32)?;
    emu.sbr_no_count(6usize, 0usize, 15usize, 2165304u32);
    emu.sbr_no_count(28usize, 17usize, 15usize, 2165308u32);
    emu.adi_no_count(10usize, 0usize, 4294967295u32, 2165312u32);
    emu.sltru_no_count(6usize, 28usize, 6usize, 2165316u32);
    emu.mulhu_no_count(29usize, 15usize, 10usize, 2165320u32);
    emu.sbr_no_count(29usize, 29usize, 17usize, 2165324u32);
    emu.adr_no_count(6usize, 29usize, 6usize, 2165328u32);
    emu.sbr_no_count(29usize, 0usize, 17usize, 2165332u32);
    emu.adr_no_count(7usize, 15usize, 7usize, 2165336u32);
    emu.mulhu_no_count(30usize, 17usize, 10usize, 2165340u32);
    emu.adr_no_count(17usize, 28usize, 5usize, 2165344u32);
    emu.sltru_no_count(15usize, 7usize, 15usize, 2165348u32);
    emu.sltru_no_count(5usize, 6usize, 29usize, 2165352u32);
    emu.adr_no_count(17usize, 17usize, 15usize, 2165356u32);
    emu.adr_no_count(5usize, 30usize, 5usize, 2165360u32);
    emu.add_memory_rw_events(16usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2165368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210a78));
    } else {
        emu.pc = 2165364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210a74));
    }
}
#[inline(always)]
pub fn block_0x00210a74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 17usize, 28usize, 2165368u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2165368u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210a78));
}
#[inline]
pub fn block_0x00210a78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(29usize, 6usize, 15usize, 2165372u32);
    emu.adr_no_count(16usize, 7usize, 16usize, 2165376u32);
    emu.adr_no_count(15usize, 12usize, 14usize, 2165380u32);
    emu.sltru_no_count(6usize, 29usize, 6usize, 2165384u32);
    emu.sltru_no_count(7usize, 16usize, 7usize, 2165388u32);
    emu.sltru_no_count(12usize, 15usize, 12usize, 2165392u32);
    emu.adr_no_count(6usize, 5usize, 6usize, 2165396u32);
    emu.adr_no_count(5usize, 17usize, 7usize, 2165400u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2165404u32);
    emu.sltru_no_count(17usize, 5usize, 17usize, 2165408u32);
    emu.anr_no_count(28usize, 7usize, 17usize, 2165412u32);
    emu.sltru_no_count(7usize, 12usize, 13usize, 2165416u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2165420u32);
    emu.sltru_no_count(17usize, 12usize, 16usize, 2165424u32);
    emu.adr_no_count(16usize, 5usize, 7usize, 2165428u32);
    emu.adr_no_count(28usize, 29usize, 28usize, 2165432u32);
    emu.sltru_no_count(7usize, 28usize, 29usize, 2165436u32);
    emu.adr_no_count(16usize, 16usize, 17usize, 2165440u32);
    emu.adr_no_count(6usize, 6usize, 7usize, 2165444u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2165452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210acc));
    } else {
        emu.pc = 2165448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210ac8));
    }
}
#[inline(always)]
pub fn block_0x00210ac8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(17usize, 16usize, 5usize, 2165452u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2165452u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210acc));
}
#[inline]
pub fn block_0x00210acc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(5usize, 0usize, 14usize, 2165456u32);
    emu.sbr_no_count(29usize, 13usize, 14usize, 2165460u32);
    emu.mulhu_no_count(7usize, 14usize, 10usize, 2165464u32);
    emu.sltru_no_count(5usize, 29usize, 5usize, 2165468u32);
    emu.sbr_no_count(7usize, 7usize, 13usize, 2165472u32);
    emu.adr_no_count(5usize, 7usize, 5usize, 2165476u32);
    emu.sbr_no_count(30usize, 0usize, 13usize, 2165480u32);
    emu.mulhu_no_count(31usize, 13usize, 10usize, 2165484u32);
    emu.adr_no_count(7usize, 28usize, 14usize, 2165488u32);
    emu.sltru_no_count(14usize, 7usize, 28usize, 2165492u32);
    emu.adr_no_count(13usize, 6usize, 29usize, 2165496u32);
    emu.sltru_no_count(28usize, 5usize, 30usize, 2165500u32);
    emu.adr_no_count(13usize, 13usize, 14usize, 2165504u32);
    emu.adr_no_count(28usize, 31usize, 28usize, 2165508u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2165516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210b0c));
    } else {
        emu.pc = 2165512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210b08));
    }
}
#[inline(always)]
pub fn block_0x00210b08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(14usize, 13usize, 6usize, 2165516u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2165516u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210b0c));
}
#[inline]
pub fn block_0x00210b0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(6usize, 5usize, 14usize, 2165520u32);
    emu.adr_no_count(17usize, 7usize, 17usize, 2165524u32);
    emu.adr_no_count(14usize, 16usize, 11usize, 2165528u32);
    emu.sltru_no_count(5usize, 6usize, 5usize, 2165532u32);
    emu.sltru_no_count(7usize, 17usize, 7usize, 2165536u32);
    emu.sltru_no_count(16usize, 14usize, 16usize, 2165540u32);
    emu.adr_no_count(29usize, 28usize, 5usize, 2165544u32);
    emu.adr_no_count(28usize, 13usize, 7usize, 2165548u32);
    emu.adr_no_count(16usize, 15usize, 16usize, 2165552u32);
    emu.sltru_no_count(13usize, 28usize, 13usize, 2165556u32);
    emu.anr_no_count(7usize, 7usize, 13usize, 2165560u32);
    emu.sltru_no_count(5usize, 16usize, 15usize, 2165564u32);
    emu.adr_no_count(13usize, 16usize, 17usize, 2165568u32);
    emu.sltru_no_count(17usize, 13usize, 16usize, 2165572u32);
    emu.adr_no_count(16usize, 5usize, 28usize, 2165576u32);
    emu.adr_no_count(28usize, 6usize, 7usize, 2165580u32);
    emu.sltru_no_count(6usize, 28usize, 6usize, 2165584u32);
    emu.adr_no_count(16usize, 16usize, 17usize, 2165588u32);
    emu.adr_no_count(6usize, 29usize, 6usize, 2165592u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2165600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210b60));
    } else {
        emu.pc = 2165596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210b5c));
    }
}
#[inline(always)]
pub fn block_0x00210b5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(17usize, 16usize, 5usize, 2165600u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2165600u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210b60));
}
#[inline]
pub fn block_0x00210b60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(29usize, 15usize, 11usize, 2165604u32);
    emu.sbr_no_count(5usize, 0usize, 11usize, 2165608u32);
    emu.mulhu_no_count(7usize, 11usize, 10usize, 2165612u32);
    emu.sltru_no_count(5usize, 29usize, 5usize, 2165616u32);
    emu.sbr_no_count(7usize, 7usize, 15usize, 2165620u32);
    emu.adr_no_count(5usize, 7usize, 5usize, 2165624u32);
    emu.sbr_no_count(30usize, 0usize, 15usize, 2165628u32);
    emu.mulhu_no_count(31usize, 15usize, 10usize, 2165632u32);
    emu.adr_no_count(7usize, 28usize, 11usize, 2165636u32);
    emu.sltru_no_count(15usize, 7usize, 28usize, 2165640u32);
    emu.adr_no_count(11usize, 6usize, 29usize, 2165644u32);
    emu.sltru_no_count(28usize, 5usize, 30usize, 2165648u32);
    emu.adr_no_count(11usize, 11usize, 15usize, 2165652u32);
    emu.adr_no_count(28usize, 31usize, 28usize, 2165656u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2165664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210ba0));
    } else {
        emu.pc = 2165660u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210b9c));
    }
}
#[inline(always)]
pub fn block_0x00210b9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 11usize, 6usize, 2165664u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2165664u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210ba0));
}
#[inline]
pub fn block_0x00210ba0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(29usize, 5usize, 15usize, 2165668u32);
    emu.adr_no_count(17usize, 7usize, 17usize, 2165672u32);
    emu.adr_no_count(15usize, 16usize, 12usize, 2165676u32);
    emu.sltru_no_count(5usize, 29usize, 5usize, 2165680u32);
    emu.sltru_no_count(6usize, 17usize, 7usize, 2165684u32);
    emu.sltru_no_count(16usize, 15usize, 16usize, 2165688u32);
    emu.adr_no_count(28usize, 28usize, 5usize, 2165692u32);
    emu.adr_no_count(30usize, 11usize, 6usize, 2165696u32);
    emu.adr_no_count(5usize, 14usize, 16usize, 2165700u32);
    emu.sltru_no_count(11usize, 30usize, 11usize, 2165704u32);
    emu.sltru_no_count(7usize, 5usize, 14usize, 2165708u32);
    emu.adr_no_count(16usize, 5usize, 17usize, 2165712u32);
    emu.anr_no_count(6usize, 6usize, 11usize, 2165716u32);
    emu.sltru_no_count(5usize, 16usize, 5usize, 2165720u32);
    emu.adr_no_count(11usize, 7usize, 5usize, 2165724u32);
    emu.adr_no_count(11usize, 11usize, 30usize, 2165728u32);
    emu.adr_no_count(6usize, 29usize, 6usize, 2165732u32);
    emu.sltru_no_count(17usize, 6usize, 29usize, 2165736u32);
    emu.adr_no_count(17usize, 28usize, 17usize, 2165740u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2165748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210bf4));
    } else {
        emu.pc = 2165744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210bf0));
    }
}
#[inline(always)]
pub fn block_0x00210bf0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 11usize, 7usize, 2165748u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2165748u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210bf4));
}
#[inline]
pub fn block_0x00210bf4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(28usize, 14usize, 12usize, 2165752u32);
    emu.sbr_no_count(7usize, 0usize, 12usize, 2165756u32);
    emu.mulhu_no_count(29usize, 12usize, 10usize, 2165760u32);
    emu.sltru_no_count(7usize, 28usize, 7usize, 2165764u32);
    emu.sbr_no_count(29usize, 29usize, 14usize, 2165768u32);
    emu.adr_no_count(7usize, 29usize, 7usize, 2165772u32);
    emu.sbr_no_count(30usize, 0usize, 14usize, 2165776u32);
    emu.adr_no_count(29usize, 6usize, 12usize, 2165780u32);
    emu.sltru_no_count(12usize, 29usize, 6usize, 2165784u32);
    emu.adr_no_count(28usize, 17usize, 28usize, 2165788u32);
    emu.adr_no_count(28usize, 28usize, 12usize, 2165792u32);
    emu.sltru_no_count(6usize, 7usize, 30usize, 2165796u32);
    emu.mulhu_no_count(10usize, 14usize, 10usize, 2165800u32);
    emu.add_memory_rw_events(13usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2165808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210c30));
    } else {
        emu.pc = 2165804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210c2c));
    }
}
#[inline(always)]
pub fn block_0x00210c2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 28usize, 17usize, 2165808u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2165808u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210c30));
}
#[inline]
pub fn block_0x00210c30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(6usize, 10usize, 6usize, 2165812u32);
    emu.adr_no_count(17usize, 7usize, 12usize, 2165816u32);
    emu.adr_no_count(12usize, 29usize, 5usize, 2165820u32);
    emu.adi_no_count(10usize, 13usize, 1u32, 2165824u32);
    emu.sltru_no_count(30usize, 17usize, 7usize, 2165828u32);
    emu.sltru_no_count(13usize, 12usize, 29usize, 2165832u32);
    emu.sltiu_no_count(5usize, 10usize, 1u32, 2165836u32);
    emu.adr_no_count(14usize, 28usize, 13usize, 2165840u32);
    emu.adr_no_count(15usize, 15usize, 5usize, 2165844u32);
    emu.sltru_no_count(5usize, 14usize, 28usize, 2165848u32);
    emu.orr_no_count(15usize, 10usize, 15usize, 2165852u32);
    emu.anr_no_count(13usize, 13usize, 5usize, 2165856u32);
    emu.sltiu_no_count(15usize, 15usize, 1u32, 2165860u32);
    emu.adi_no_count(15usize, 15usize, 4294967295u32, 2165864u32);
    emu.adr_no_count(5usize, 16usize, 15usize, 2165868u32);
    emu.sltru_no_count(16usize, 5usize, 16usize, 2165872u32);
    emu.adr_no_count(7usize, 11usize, 15usize, 2165876u32);
    emu.adr_no_count(7usize, 7usize, 16usize, 2165880u32);
    emu.adr_no_count(13usize, 17usize, 13usize, 2165884u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2165892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210c84));
    } else {
        emu.pc = 2165888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210c80));
    }
}
#[inline(always)]
pub fn block_0x00210c80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 7usize, 11usize, 2165892u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2165892u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210c84));
}
#[inline]
pub fn block_0x00210c84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 6usize, 30usize, 2165896u32);
    emu.adr_no_count(16usize, 15usize, 16usize, 2165900u32);
    emu.adi_no_count(28usize, 5usize, 1u32, 2165904u32);
    emu.sltru_no_count(6usize, 16usize, 15usize, 2165908u32);
    emu.adr_no_count(6usize, 15usize, 6usize, 2165912u32);
    emu.sltiu_no_count(15usize, 28usize, 1u32, 2165916u32);
    emu.adr_no_count(15usize, 7usize, 15usize, 2165920u32);
    emu.adi_no_count(29usize, 15usize, 4294967295u32, 2165924u32);
    emu.sltru_no_count(15usize, 13usize, 17usize, 2165928u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(29usize);
    if a == b {
        emu.pc = 2165940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210cb4));
    } else {
        emu.pc = 2165932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210cac));
    }
}
#[inline(always)]
pub fn block_0x00210cac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(17usize, 29usize, 7usize, 2165936u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2165940u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2165944u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210cb8));
}
#[inline(always)]
pub fn block_0x00210cb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(17usize, 28usize, 5usize, 2165944u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2165944u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210cb8));
}
#[inline]
pub fn block_0x00210cb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(5usize, 16usize, 1u32, 2165948u32);
    emu.sbr_no_count(5usize, 6usize, 5usize, 2165952u32);
    emu.adi_no_count(16usize, 16usize, 4294967295u32, 2165956u32);
    emu.adr_no_count(17usize, 16usize, 17usize, 2165960u32);
    emu.sltru_no_count(16usize, 17usize, 16usize, 2165964u32);
    emu.adr_no_count(16usize, 5usize, 16usize, 2165968u32);
    emu.sai_no_count(16usize, 16usize, 1055u32, 2165972u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2165976u32);
    emu.sltru_no_count(12usize, 12usize, 16usize, 2165980u32);
    emu.adr_no_count(14usize, 16usize, 14usize, 2165984u32);
    emu.adr_no_count(14usize, 14usize, 12usize, 2165988u32);
    emu.adr_no_count(11usize, 11usize, 15usize, 2165992u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2166000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210cf0));
    } else {
        emu.pc = 2165996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210cec));
    }
}
#[inline(always)]
pub fn block_0x00210cec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 14usize, 16usize, 2166000u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2166000u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210cf0));
}
#[inline]
pub fn block_0x00210cf0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(12usize, 16usize, 12usize, 2166004u32);
    emu.sltru_no_count(12usize, 12usize, 16usize, 2166008u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2166012u32);
    emu.sai_no_count(15usize, 12usize, 1055u32, 2166016u32);
    emu.adr_no_count(16usize, 11usize, 15usize, 2166020u32);
    emu.adr_no_count(14usize, 13usize, 15usize, 2166024u32);
    emu.sltru_no_count(12usize, 14usize, 13usize, 2166028u32);
    emu.adr_no_count(13usize, 16usize, 12usize, 2166032u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2166040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210d18));
    } else {
        emu.pc = 2166036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210d14));
    }
}
#[inline(always)]
pub fn block_0x00210d14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 13usize, 11usize, 2166040u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2166040u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210d18));
}
#[inline(always)]
pub fn block_0x00210d18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(12usize, 15usize, 12usize, 2166044u32);
    emu.sltru_no_count(11usize, 12usize, 15usize, 2166048u32);
    emu.adr_no_count(11usize, 15usize, 11usize, 2166052u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2166072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210d38));
    } else {
        emu.pc = 2166056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210d28));
    }
}
#[inline(always)]
pub fn block_0x00210d28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(14usize, 0usize, 14usize, 2166060u32);
    emu.adr_no_count(14usize, 13usize, 14usize, 2166064u32);
    emu.sltru_no_count(13usize, 14usize, 13usize, 2166068u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2166072u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2166080u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210d40));
}
#[inline(always)]
pub fn block_0x00210d38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 14usize, 4294967295u32, 2166076u32);
    emu.sltru_no_count(13usize, 13usize, 14usize, 2166080u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2166080u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210d40));
}
#[inline]
pub fn block_0x00210d40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 12usize, 4294967295u32, 2166084u32);
    emu.sltiu_no_count(12usize, 12usize, 1u32, 2166088u32);
    emu.adr_no_count(13usize, 14usize, 13usize, 2166092u32);
    emu.sltru_no_count(13usize, 13usize, 14usize, 2166096u32);
    emu.sbr_no_count(11usize, 11usize, 12usize, 2166100u32);
    emu.adr_no_count(11usize, 11usize, 13usize, 2166104u32);
    emu.sri_no_count(11usize, 11usize, 31u32, 2166108u32);
    emu.adr_no_count(10usize, 11usize, 10usize, 2166112u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2166116u32);
    emu.apc_no_count(6usize, 2166116u32, 20480u32, 2166120u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2166124u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1728u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
