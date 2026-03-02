pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2160788u32;
pub const PC_MAX: u32 = 2168928u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 131usize] = [
        block_0x0020f894,
        block_0x0020f8e8,
        block_0x0020f93c,
        block_0x0020f990,
        block_0x0020f9e4,
        block_0x0020fa38,
        block_0x0020faa4,
        block_0x0020faf8,
        block_0x0020fb4c,
        block_0x0020fba0,
        block_0x0020fbf4,
        block_0x0020fc48,
        block_0x0020fc9c,
        block_0x0020fcf0,
        block_0x0020fd44,
        block_0x0020fd98,
        block_0x0020fdec,
        block_0x0020fe40,
        block_0x0020fe94,
        block_0x0020fee8,
        block_0x0020ff3c,
        block_0x0020ff90,
        block_0x0020ffe4,
        block_0x00210050,
        block_0x00210064,
        block_0x002100b8,
        block_0x0021010c,
        block_0x00210160,
        block_0x002101b4,
        block_0x00210208,
        block_0x0021025c,
        block_0x002102b0,
        block_0x00210304,
        block_0x00210358,
        block_0x002103ac,
        block_0x00210400,
        block_0x00210454,
        block_0x002104a8,
        block_0x002104fc,
        block_0x00210550,
        block_0x002105a4,
        block_0x002105f8,
        block_0x0021064c,
        block_0x002106a0,
        block_0x002106f4,
        block_0x00210748,
        block_0x0021079c,
        block_0x002107f0,
        block_0x00210844,
        block_0x00210898,
        block_0x002108ec,
        block_0x00210940,
        block_0x00210994,
        block_0x002109e8,
        block_0x00210a3c,
        block_0x00210a90,
        block_0x00210afc,
        block_0x00210b00,
        block_0x00210b14,
        block_0x00210b5c,
        block_0x00210bb0,
        block_0x00210bb4,
        block_0x00210bc8,
        block_0x00210c10,
        block_0x00210c84,
        block_0x00210d10,
        block_0x00210d2c,
        block_0x00210f58,
        block_0x00210f94,
        block_0x00210fcc,
        block_0x0021100c,
        block_0x00211010,
        block_0x00211054,
        block_0x00211058,
        block_0x002110a8,
        block_0x002110ac,
        block_0x0021111c,
        block_0x00211120,
        block_0x00211170,
        block_0x00211174,
        block_0x002111b0,
        block_0x002111b4,
        block_0x00211204,
        block_0x00211208,
        block_0x00211240,
        block_0x00211244,
        block_0x00211294,
        block_0x00211298,
        block_0x002112c0,
        block_0x002112c8,
        block_0x002112cc,
        block_0x00211300,
        block_0x00211304,
        block_0x00211328,
        block_0x0021132c,
        block_0x00211348,
        block_0x00211350,
        block_0x00211354,
        block_0x00211384,
        block_0x00211388,
        block_0x002113d0,
        block_0x002113d4,
        block_0x0021150c,
        block_0x0021154c,
        block_0x00211550,
        block_0x00211594,
        block_0x00211598,
        block_0x002115e8,
        block_0x002115ec,
        block_0x00211628,
        block_0x0021162c,
        block_0x0021167c,
        block_0x00211680,
        block_0x002116bc,
        block_0x002116c0,
        block_0x00211710,
        block_0x00211714,
        block_0x0021174c,
        block_0x00211750,
        block_0x002117a0,
        block_0x002117a4,
        block_0x002117cc,
        block_0x002117d4,
        block_0x002117d8,
        block_0x0021180c,
        block_0x00211810,
        block_0x00211834,
        block_0x00211838,
        block_0x00211848,
        block_0x00211858,
        block_0x00211860,
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
    if pc < 2160788u32 || pc > 2168928u32 {
        return None;
    }
    let word_offset = ((pc - 2160788u32) >> 2) as u32;
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
pub fn block_0x0020f894(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2160792u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2160796u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2160800u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2160804u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2160808u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2160812u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2160816u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2160820u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2160824u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2160828u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2160832u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2160836u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2160840u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2160844u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2160848u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2160852u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2160856u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2160860u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2160864u32);
    emu.apc_no_count(1usize, 2160864u32, 4294946816u32, 2160868u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160872u32;
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
pub fn block_0x0020f8e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2160876u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2160880u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2160884u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2160888u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2160892u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2160896u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2160900u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2160904u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2160908u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2160912u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2160916u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2160920u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2160924u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2160928u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2160932u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2160936u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2160940u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2160944u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2160948u32);
    emu.apc_no_count(1usize, 2160948u32, 4294946816u32, 2160952u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160956u32;
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
pub fn block_0x0020f93c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2160960u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2160964u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2160968u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2160972u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2160976u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2160980u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2160984u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2160988u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2160992u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2160996u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2161000u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2161004u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2161008u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2161012u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2161016u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2161020u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2161024u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2161028u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2161032u32);
    emu.apc_no_count(1usize, 2161032u32, 4294946816u32, 2161036u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161040u32;
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
pub fn block_0x0020f990(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2161044u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2161048u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2161052u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2161056u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2161060u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2161064u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2161068u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2161072u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2161076u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2161080u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2161084u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2161088u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2161092u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2161096u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2161100u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2161104u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2161108u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2161112u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2161116u32);
    emu.apc_no_count(1usize, 2161116u32, 4294946816u32, 2161120u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161124u32;
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
pub fn block_0x0020f9e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2161128u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2161132u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2161136u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2161140u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2161144u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2161148u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2161152u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2161156u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2161160u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2161164u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2161168u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2161172u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2161176u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2161180u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2161184u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2161188u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2161192u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2161196u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2161200u32);
    emu.apc_no_count(1usize, 2161200u32, 4294946816u32, 2161204u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161208u32;
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
pub fn block_0x0020fa38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2161212u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2161216u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2161220u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2161224u32)?;
    emu.lw_no_count(14usize, 2usize, 192u32, 2161228u32)?;
    emu.lw_no_count(15usize, 2usize, 196u32, 2161232u32)?;
    emu.lw_no_count(16usize, 2usize, 200u32, 2161236u32)?;
    emu.lw_no_count(17usize, 2usize, 204u32, 2161240u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2161244u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2161248u32)?;
    emu.sw_no_count(14usize, 2usize, 160u32, 2161252u32)?;
    emu.sw_no_count(17usize, 2usize, 172u32, 2161256u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2161260u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2161264u32)?;
    emu.sw_no_count(14usize, 2usize, 192u32, 2161268u32)?;
    emu.sw_no_count(15usize, 2usize, 196u32, 2161272u32)?;
    emu.sw_no_count(16usize, 2usize, 200u32, 2161276u32)?;
    emu.sw_no_count(17usize, 2usize, 204u32, 2161280u32)?;
    emu.sw_no_count(10usize, 2usize, 208u32, 2161284u32)?;
    emu.sw_no_count(11usize, 2usize, 212u32, 2161288u32)?;
    emu.sw_no_count(12usize, 2usize, 216u32, 2161292u32)?;
    emu.sw_no_count(13usize, 2usize, 220u32, 2161296u32)?;
    emu.adi_no_count(10usize, 2usize, 96u32, 2161300u32);
    emu.adi_no_count(11usize, 2usize, 192u32, 2161304u32);
    emu.adi_no_count(12usize, 2usize, 64u32, 2161308u32);
    emu.apc_no_count(1usize, 2161308u32, 4294946816u32, 2161312u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161316u32;
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
pub fn block_0x0020faa4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 112u32, 2161320u32)?;
    emu.lw_no_count(11usize, 2usize, 116u32, 2161324u32)?;
    emu.lw_no_count(12usize, 2usize, 120u32, 2161328u32)?;
    emu.lw_no_count(13usize, 2usize, 124u32, 2161332u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2161336u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2161340u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2161344u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2161348u32)?;
    emu.lw_no_count(10usize, 2usize, 96u32, 2161352u32)?;
    emu.lw_no_count(11usize, 2usize, 100u32, 2161356u32)?;
    emu.lw_no_count(12usize, 2usize, 104u32, 2161360u32)?;
    emu.lw_no_count(13usize, 2usize, 108u32, 2161364u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2161368u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2161372u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2161376u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2161380u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2161384u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2161388u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2161392u32);
    emu.apc_no_count(1usize, 2161392u32, 4294946816u32, 2161396u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161400u32;
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
pub fn block_0x0020faf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2161404u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2161408u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2161412u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2161416u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2161420u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2161424u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2161428u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2161432u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2161436u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2161440u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2161444u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2161448u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2161452u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2161456u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2161460u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2161464u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2161468u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2161472u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2161476u32);
    emu.apc_no_count(1usize, 2161476u32, 4294946816u32, 2161480u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161484u32;
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
pub fn block_0x0020fb4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2161488u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2161492u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2161496u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2161500u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2161504u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2161508u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2161512u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2161516u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2161520u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2161524u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2161528u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2161532u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2161536u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2161540u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2161544u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2161548u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2161552u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2161556u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2161560u32);
    emu.apc_no_count(1usize, 2161560u32, 4294946816u32, 2161564u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161568u32;
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
pub fn block_0x0020fba0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2161572u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2161576u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2161580u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2161584u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2161588u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2161592u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2161596u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2161600u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2161604u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2161608u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2161612u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2161616u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2161620u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2161624u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2161628u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2161632u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2161636u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2161640u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2161644u32);
    emu.apc_no_count(1usize, 2161644u32, 4294946816u32, 2161648u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161652u32;
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
pub fn block_0x0020fbf4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2161656u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2161660u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2161664u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2161668u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2161672u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2161676u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2161680u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2161684u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2161688u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2161692u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2161696u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2161700u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2161704u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2161708u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2161712u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2161716u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2161720u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2161724u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2161728u32);
    emu.apc_no_count(1usize, 2161728u32, 4294946816u32, 2161732u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161736u32;
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
pub fn block_0x0020fc48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2161740u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2161744u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2161748u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2161752u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2161756u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2161760u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2161764u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2161768u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2161772u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2161776u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2161780u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2161784u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2161788u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2161792u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2161796u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2161800u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2161804u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2161808u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2161812u32);
    emu.apc_no_count(1usize, 2161812u32, 4294946816u32, 2161816u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161820u32;
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
pub fn block_0x0020fc9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2161824u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2161828u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2161832u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2161836u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2161840u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2161844u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2161848u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2161852u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2161856u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2161860u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2161864u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2161868u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2161872u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2161876u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2161880u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2161884u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2161888u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2161892u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2161896u32);
    emu.apc_no_count(1usize, 2161896u32, 4294946816u32, 2161900u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161904u32;
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
pub fn block_0x0020fcf0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2161908u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2161912u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2161916u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2161920u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2161924u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2161928u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2161932u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2161936u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2161940u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2161944u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2161948u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2161952u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2161956u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2161960u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2161964u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2161968u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2161972u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2161976u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2161980u32);
    emu.apc_no_count(1usize, 2161980u32, 4294946816u32, 2161984u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161988u32;
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
pub fn block_0x0020fd44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2161992u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2161996u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2162000u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2162004u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2162008u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2162012u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2162016u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2162020u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2162024u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2162028u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2162032u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2162036u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2162040u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2162044u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2162048u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2162052u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2162056u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2162060u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2162064u32);
    emu.apc_no_count(1usize, 2162064u32, 4294946816u32, 2162068u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162072u32;
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
pub fn block_0x0020fd98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2162076u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2162080u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2162084u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2162088u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2162092u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2162096u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2162100u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2162104u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2162108u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2162112u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2162116u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2162120u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2162124u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2162128u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2162132u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2162136u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2162140u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2162144u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2162148u32);
    emu.apc_no_count(1usize, 2162148u32, 4294946816u32, 2162152u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162156u32;
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
pub fn block_0x0020fdec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2162160u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2162164u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2162168u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2162172u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2162176u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2162180u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2162184u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2162188u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2162192u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2162196u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2162200u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2162204u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2162208u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2162212u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2162216u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2162220u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2162224u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2162228u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2162232u32);
    emu.apc_no_count(1usize, 2162232u32, 4294946816u32, 2162236u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162240u32;
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
pub fn block_0x0020fe40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2162244u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2162248u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2162252u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2162256u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2162260u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2162264u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2162268u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2162272u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2162276u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2162280u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2162284u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2162288u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2162292u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2162296u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2162300u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2162304u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2162308u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2162312u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2162316u32);
    emu.apc_no_count(1usize, 2162316u32, 4294946816u32, 2162320u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162324u32;
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
pub fn block_0x0020fe94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2162328u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2162332u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2162336u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2162340u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2162344u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2162348u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2162352u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2162356u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2162360u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2162364u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2162368u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2162372u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2162376u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2162380u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2162384u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2162388u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2162392u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2162396u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2162400u32);
    emu.apc_no_count(1usize, 2162400u32, 4294946816u32, 2162404u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162408u32;
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
pub fn block_0x0020fee8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2162412u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2162416u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2162420u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2162424u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2162428u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2162432u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2162436u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2162440u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2162444u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2162448u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2162452u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2162456u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2162460u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2162464u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2162468u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2162472u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2162476u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2162480u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2162484u32);
    emu.apc_no_count(1usize, 2162484u32, 4294946816u32, 2162488u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162492u32;
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
pub fn block_0x0020ff3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2162496u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2162500u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2162504u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2162508u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2162512u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2162516u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2162520u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2162524u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2162528u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2162532u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2162536u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2162540u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2162544u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2162548u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2162552u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2162556u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2162560u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2162564u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2162568u32);
    emu.apc_no_count(1usize, 2162568u32, 4294942720u32, 2162572u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162576u32;
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
pub fn block_0x0020ff90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2162580u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2162584u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2162588u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2162592u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2162596u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2162600u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2162604u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2162608u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2162612u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2162616u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2162620u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2162624u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2162628u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2162632u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2162636u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2162640u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2162644u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2162648u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2162652u32);
    emu.apc_no_count(1usize, 2162652u32, 4294942720u32, 2162656u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162660u32;
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
pub fn block_0x0020ffe4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2162664u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2162668u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2162672u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2162676u32)?;
    emu.lw_no_count(14usize, 2usize, 192u32, 2162680u32)?;
    emu.lw_no_count(15usize, 2usize, 196u32, 2162684u32)?;
    emu.lw_no_count(16usize, 2usize, 200u32, 2162688u32)?;
    emu.lw_no_count(17usize, 2usize, 204u32, 2162692u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2162696u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2162700u32)?;
    emu.sw_no_count(14usize, 2usize, 160u32, 2162704u32)?;
    emu.sw_no_count(17usize, 2usize, 172u32, 2162708u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2162712u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2162716u32)?;
    emu.sw_no_count(14usize, 2usize, 192u32, 2162720u32)?;
    emu.sw_no_count(15usize, 2usize, 196u32, 2162724u32)?;
    emu.sw_no_count(16usize, 2usize, 200u32, 2162728u32)?;
    emu.sw_no_count(17usize, 2usize, 204u32, 2162732u32)?;
    emu.sw_no_count(10usize, 2usize, 208u32, 2162736u32)?;
    emu.sw_no_count(11usize, 2usize, 212u32, 2162740u32)?;
    emu.sw_no_count(12usize, 2usize, 216u32, 2162744u32)?;
    emu.sw_no_count(13usize, 2usize, 220u32, 2162748u32)?;
    emu.adi_no_count(10usize, 2usize, 160u32, 2162752u32);
    emu.adi_no_count(11usize, 2usize, 192u32, 2162756u32);
    emu.adi_no_count(12usize, 2usize, 96u32, 2162760u32);
    emu.apc_no_count(1usize, 2162760u32, 4294942720u32, 2162764u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162768u32;
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
pub fn block_0x00210050(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 192u32, 2162772u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2162776u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2162780u32);
    emu.apc_no_count(1usize, 2162780u32, 4294942720u32, 2162784u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162788u32;
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
pub fn block_0x00210064(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2162792u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2162796u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2162800u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2162804u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2162808u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2162812u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2162816u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2162820u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2162824u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2162828u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2162832u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2162836u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2162840u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2162844u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2162848u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2162852u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2162856u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2162860u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2162864u32);
    emu.apc_no_count(1usize, 2162864u32, 4294942720u32, 2162868u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162872u32;
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
pub fn block_0x002100b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2162876u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2162880u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2162884u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2162888u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2162892u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2162896u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2162900u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2162904u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2162908u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2162912u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2162916u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2162920u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2162924u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2162928u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2162932u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2162936u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2162940u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2162944u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2162948u32);
    emu.apc_no_count(1usize, 2162948u32, 4294942720u32, 2162952u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162956u32;
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
pub fn block_0x0021010c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2162960u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2162964u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2162968u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2162972u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2162976u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2162980u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2162984u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2162988u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2162992u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2162996u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2163000u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2163004u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2163008u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2163012u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2163016u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2163020u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2163024u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2163028u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2163032u32);
    emu.apc_no_count(1usize, 2163032u32, 4294942720u32, 2163036u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2163040u32;
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
pub fn block_0x00210160(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2163044u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2163048u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2163052u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2163056u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2163060u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2163064u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2163068u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2163072u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2163076u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2163080u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2163084u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2163088u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2163092u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2163096u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2163100u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2163104u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2163108u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2163112u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2163116u32);
    emu.apc_no_count(1usize, 2163116u32, 4294942720u32, 2163120u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2163124u32;
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
pub fn block_0x002101b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2163128u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2163132u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2163136u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2163140u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2163144u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2163148u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2163152u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2163156u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2163160u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2163164u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2163168u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2163172u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2163176u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2163180u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2163184u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2163188u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2163192u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2163196u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2163200u32);
    emu.apc_no_count(1usize, 2163200u32, 4294942720u32, 2163204u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2163208u32;
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
pub fn block_0x00210208(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2163212u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2163216u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2163220u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2163224u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2163228u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2163232u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2163236u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2163240u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2163244u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2163248u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2163252u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2163256u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2163260u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2163264u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2163268u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2163272u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2163276u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2163280u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2163284u32);
    emu.apc_no_count(1usize, 2163284u32, 4294942720u32, 2163288u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2163292u32;
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
pub fn block_0x0021025c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2163296u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2163300u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2163304u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2163308u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2163312u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2163316u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2163320u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2163324u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2163328u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2163332u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2163336u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2163340u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2163344u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2163348u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2163352u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2163356u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2163360u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2163364u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2163368u32);
    emu.apc_no_count(1usize, 2163368u32, 4294942720u32, 2163372u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2163376u32;
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
pub fn block_0x002102b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2163380u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2163384u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2163388u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2163392u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2163396u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2163400u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2163404u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2163408u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2163412u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2163416u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2163420u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2163424u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2163428u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2163432u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2163436u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2163440u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2163444u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2163448u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2163452u32);
    emu.apc_no_count(1usize, 2163452u32, 4294942720u32, 2163456u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2163460u32;
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
pub fn block_0x00210304(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2163464u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2163468u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2163472u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2163476u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2163480u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2163484u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2163488u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2163492u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2163496u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2163500u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2163504u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2163508u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2163512u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2163516u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2163520u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2163524u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2163528u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2163532u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2163536u32);
    emu.apc_no_count(1usize, 2163536u32, 4294942720u32, 2163540u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2163544u32;
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
pub fn block_0x00210358(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2163548u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2163552u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2163556u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2163560u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2163564u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2163568u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2163572u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2163576u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2163580u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2163584u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2163588u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2163592u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2163596u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2163600u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2163604u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2163608u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2163612u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2163616u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2163620u32);
    emu.apc_no_count(1usize, 2163620u32, 4294942720u32, 2163624u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2163628u32;
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
pub fn block_0x002103ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2163632u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2163636u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2163640u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2163644u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2163648u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2163652u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2163656u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2163660u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2163664u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2163668u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2163672u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2163676u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2163680u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2163684u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2163688u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2163692u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2163696u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2163700u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2163704u32);
    emu.apc_no_count(1usize, 2163704u32, 4294942720u32, 2163708u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2163712u32;
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
pub fn block_0x00210400(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2163716u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2163720u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2163724u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2163728u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2163732u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2163736u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2163740u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2163744u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2163748u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2163752u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2163756u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2163760u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2163764u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2163768u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2163772u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2163776u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2163780u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2163784u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2163788u32);
    emu.apc_no_count(1usize, 2163788u32, 4294942720u32, 2163792u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2163796u32;
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
pub fn block_0x00210454(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2163800u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2163804u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2163808u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2163812u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2163816u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2163820u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2163824u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2163828u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2163832u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2163836u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2163840u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2163844u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2163848u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2163852u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2163856u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2163860u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2163864u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2163868u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2163872u32);
    emu.apc_no_count(1usize, 2163872u32, 4294942720u32, 2163876u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2163880u32;
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
pub fn block_0x002104a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2163884u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2163888u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2163892u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2163896u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2163900u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2163904u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2163908u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2163912u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2163916u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2163920u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2163924u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2163928u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2163932u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2163936u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2163940u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2163944u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2163948u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2163952u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2163956u32);
    emu.apc_no_count(1usize, 2163956u32, 4294942720u32, 2163960u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2163964u32;
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
pub fn block_0x002104fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2163968u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2163972u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2163976u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2163980u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2163984u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2163988u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2163992u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2163996u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2164000u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2164004u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2164008u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2164012u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2164016u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2164020u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2164024u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2164028u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2164032u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2164036u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2164040u32);
    emu.apc_no_count(1usize, 2164040u32, 4294942720u32, 2164044u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2164048u32;
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
pub fn block_0x00210550(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2164052u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2164056u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2164060u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2164064u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2164068u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2164072u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2164076u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2164080u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2164084u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2164088u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2164092u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2164096u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2164100u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2164104u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2164108u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2164112u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2164116u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2164120u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2164124u32);
    emu.apc_no_count(1usize, 2164124u32, 4294942720u32, 2164128u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2164132u32;
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
pub fn block_0x002105a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2164136u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2164140u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2164144u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2164148u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2164152u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2164156u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2164160u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2164164u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2164168u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2164172u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2164176u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2164180u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2164184u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2164188u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2164192u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2164196u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2164200u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2164204u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2164208u32);
    emu.apc_no_count(1usize, 2164208u32, 4294942720u32, 2164212u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2164216u32;
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
pub fn block_0x002105f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2164220u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2164224u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2164228u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2164232u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2164236u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2164240u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2164244u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2164248u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2164252u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2164256u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2164260u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2164264u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2164268u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2164272u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2164276u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2164280u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2164284u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2164288u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2164292u32);
    emu.apc_no_count(1usize, 2164292u32, 4294942720u32, 2164296u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2164300u32;
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
pub fn block_0x0021064c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2164304u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2164308u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2164312u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2164316u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2164320u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2164324u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2164328u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2164332u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2164336u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2164340u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2164344u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2164348u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2164352u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2164356u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2164360u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2164364u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2164368u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2164372u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2164376u32);
    emu.apc_no_count(1usize, 2164376u32, 4294942720u32, 2164380u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2164384u32;
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
pub fn block_0x002106a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2164388u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2164392u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2164396u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2164400u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2164404u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2164408u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2164412u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2164416u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2164420u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2164424u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2164428u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2164432u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2164436u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2164440u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2164444u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2164448u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2164452u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2164456u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2164460u32);
    emu.apc_no_count(1usize, 2164460u32, 4294942720u32, 2164464u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2164468u32;
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
pub fn block_0x002106f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2164472u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2164476u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2164480u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2164484u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2164488u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2164492u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2164496u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2164500u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2164504u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2164508u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2164512u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2164516u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2164520u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2164524u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2164528u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2164532u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2164536u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2164540u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2164544u32);
    emu.apc_no_count(1usize, 2164544u32, 4294942720u32, 2164548u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2164552u32;
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
pub fn block_0x00210748(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2164556u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2164560u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2164564u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2164568u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2164572u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2164576u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2164580u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2164584u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2164588u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2164592u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2164596u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2164600u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2164604u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2164608u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2164612u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2164616u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2164620u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2164624u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2164628u32);
    emu.apc_no_count(1usize, 2164628u32, 4294942720u32, 2164632u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2164636u32;
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
pub fn block_0x0021079c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2164640u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2164644u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2164648u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2164652u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2164656u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2164660u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2164664u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2164668u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2164672u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2164676u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2164680u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2164684u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2164688u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2164692u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2164696u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2164700u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2164704u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2164708u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2164712u32);
    emu.apc_no_count(1usize, 2164712u32, 4294942720u32, 2164716u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2164720u32;
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
pub fn block_0x002107f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2164724u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2164728u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2164732u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2164736u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2164740u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2164744u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2164748u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2164752u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2164756u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2164760u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2164764u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2164768u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2164772u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2164776u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2164780u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2164784u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2164788u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2164792u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2164796u32);
    emu.apc_no_count(1usize, 2164796u32, 4294942720u32, 2164800u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2164804u32;
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
pub fn block_0x00210844(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2164808u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2164812u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2164816u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2164820u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2164824u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2164828u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2164832u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2164836u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2164840u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2164844u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2164848u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2164852u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2164856u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2164860u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2164864u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2164868u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2164872u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2164876u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2164880u32);
    emu.apc_no_count(1usize, 2164880u32, 4294942720u32, 2164884u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2164888u32;
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
pub fn block_0x00210898(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2164892u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2164896u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2164900u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2164904u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2164908u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2164912u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2164916u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2164920u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2164924u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2164928u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2164932u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2164936u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2164940u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2164944u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2164948u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2164952u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2164956u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2164960u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2164964u32);
    emu.apc_no_count(1usize, 2164964u32, 4294942720u32, 2164968u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2164972u32;
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
pub fn block_0x002108ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2164976u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2164980u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2164984u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2164988u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2164992u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2164996u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2165000u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2165004u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2165008u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2165012u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2165016u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2165020u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2165024u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2165028u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2165032u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2165036u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2165040u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2165044u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2165048u32);
    emu.apc_no_count(1usize, 2165048u32, 4294942720u32, 2165052u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2165056u32;
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
pub fn block_0x00210940(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2165060u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2165064u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2165068u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2165072u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2165076u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2165080u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2165084u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2165088u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2165092u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2165096u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2165100u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2165104u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2165108u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2165112u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2165116u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2165120u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2165124u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2165128u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2165132u32);
    emu.apc_no_count(1usize, 2165132u32, 4294942720u32, 2165136u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2165140u32;
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
pub fn block_0x00210994(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2165144u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2165148u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2165152u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2165156u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2165160u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2165164u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2165168u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2165172u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2165176u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2165180u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2165184u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2165188u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2165192u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2165196u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2165200u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2165204u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2165208u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2165212u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2165216u32);
    emu.apc_no_count(1usize, 2165216u32, 4294942720u32, 2165220u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2165224u32;
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
pub fn block_0x002109e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2165228u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2165232u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2165236u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2165240u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2165244u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2165248u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2165252u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2165256u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2165260u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2165264u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2165268u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2165272u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2165276u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2165280u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2165284u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2165288u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2165292u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2165296u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2165300u32);
    emu.apc_no_count(1usize, 2165300u32, 4294942720u32, 2165304u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2165308u32;
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
pub fn block_0x00210a3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2165312u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2165316u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2165320u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2165324u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2165328u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2165332u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2165336u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2165340u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2165344u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2165348u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2165352u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2165356u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2165360u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2165364u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2165368u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2165372u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2165376u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2165380u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2165384u32);
    emu.apc_no_count(1usize, 2165384u32, 4294942720u32, 2165388u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2165392u32;
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
pub fn block_0x00210a90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2165396u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2165400u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2165404u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2165408u32)?;
    emu.lw_no_count(14usize, 2usize, 192u32, 2165412u32)?;
    emu.lw_no_count(15usize, 2usize, 196u32, 2165416u32)?;
    emu.lw_no_count(16usize, 2usize, 200u32, 2165420u32)?;
    emu.lw_no_count(17usize, 2usize, 204u32, 2165424u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2165428u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2165432u32)?;
    emu.sw_no_count(14usize, 2usize, 160u32, 2165436u32)?;
    emu.sw_no_count(17usize, 2usize, 172u32, 2165440u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2165444u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2165448u32)?;
    emu.sw_no_count(14usize, 2usize, 192u32, 2165452u32)?;
    emu.sw_no_count(15usize, 2usize, 196u32, 2165456u32)?;
    emu.sw_no_count(16usize, 2usize, 200u32, 2165460u32)?;
    emu.sw_no_count(17usize, 2usize, 204u32, 2165464u32)?;
    emu.sw_no_count(10usize, 2usize, 208u32, 2165468u32)?;
    emu.sw_no_count(11usize, 2usize, 212u32, 2165472u32)?;
    emu.sw_no_count(12usize, 2usize, 216u32, 2165476u32)?;
    emu.sw_no_count(13usize, 2usize, 220u32, 2165480u32)?;
    emu.adi_no_count(10usize, 2usize, 160u32, 2165484u32);
    emu.adi_no_count(11usize, 2usize, 192u32, 2165488u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2165492u32);
    emu.apc_no_count(1usize, 2165492u32, 4294942720u32, 2165496u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2165500u32;
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
pub fn block_0x00210afc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 96u32, 2165504u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2165504u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210b00));
}
#[inline(always)]
pub fn block_0x00210b00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 192u32, 2165508u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2165512u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2165516u32);
    emu.apc_no_count(1usize, 2165516u32, 4294942720u32, 2165520u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2165524u32;
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
pub fn block_0x00210b14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2165528u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2165532u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2165536u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2165540u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2165544u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2165548u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2165552u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2165556u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2165560u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2165564u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2165568u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2165572u32)?;
    emu.adi_no_count(18usize, 18usize, 4294967295u32, 2165576u32);
    emu.sw_no_count(10usize, 2usize, 160u32, 2165580u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2165584u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2165588u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2165592u32)?;
    emu.add_memory_rw_events(17usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2165504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210b00));
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
#[inline]
pub fn block_0x00210b5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 176u32, 2165600u32)?;
    emu.lw_no_count(11usize, 2usize, 180u32, 2165604u32)?;
    emu.lw_no_count(12usize, 2usize, 184u32, 2165608u32)?;
    emu.lw_no_count(13usize, 2usize, 188u32, 2165612u32)?;
    emu.sw_no_count(10usize, 2usize, 208u32, 2165616u32)?;
    emu.sw_no_count(11usize, 2usize, 212u32, 2165620u32)?;
    emu.sw_no_count(12usize, 2usize, 216u32, 2165624u32)?;
    emu.sw_no_count(13usize, 2usize, 220u32, 2165628u32)?;
    emu.lw_no_count(10usize, 2usize, 160u32, 2165632u32)?;
    emu.lw_no_count(11usize, 2usize, 164u32, 2165636u32)?;
    emu.lw_no_count(12usize, 2usize, 168u32, 2165640u32)?;
    emu.lw_no_count(13usize, 2usize, 172u32, 2165644u32)?;
    emu.sw_no_count(10usize, 2usize, 192u32, 2165648u32)?;
    emu.sw_no_count(11usize, 2usize, 196u32, 2165652u32)?;
    emu.sw_no_count(12usize, 2usize, 200u32, 2165656u32)?;
    emu.sw_no_count(13usize, 2usize, 204u32, 2165660u32)?;
    emu.adi_no_count(10usize, 2usize, 160u32, 2165664u32);
    emu.adi_no_count(11usize, 2usize, 192u32, 2165668u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2165672u32);
    emu.apc_no_count(1usize, 2165672u32, 4294942720u32, 2165676u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2165680u32;
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
pub fn block_0x00210bb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 94u32, 2165684u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2165684u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210bb4));
}
#[inline(always)]
pub fn block_0x00210bb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 192u32, 2165688u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2165692u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2165696u32);
    emu.apc_no_count(1usize, 2165696u32, 4294942720u32, 2165700u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2165704u32;
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
pub fn block_0x00210bc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2165708u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2165712u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2165716u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2165720u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2165724u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2165728u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2165732u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2165736u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2165740u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2165744u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2165748u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2165752u32)?;
    emu.adi_no_count(18usize, 18usize, 4294967295u32, 2165756u32);
    emu.sw_no_count(10usize, 2usize, 160u32, 2165760u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2165764u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2165768u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2165772u32)?;
    emu.add_memory_rw_events(17usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2165684u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210bb4));
    } else {
        emu.pc = 2165776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210c10));
    }
}
#[inline(never)]
pub fn block_0x00210c10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 29u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 176u32, 2165780u32)?;
    emu.lw_no_count(11usize, 2usize, 180u32, 2165784u32)?;
    emu.lw_no_count(12usize, 2usize, 184u32, 2165788u32)?;
    emu.lw_no_count(13usize, 2usize, 188u32, 2165792u32)?;
    emu.lw_no_count(14usize, 2usize, 160u32, 2165796u32)?;
    emu.lw_no_count(15usize, 2usize, 164u32, 2165800u32)?;
    emu.lw_no_count(16usize, 2usize, 168u32, 2165804u32)?;
    emu.lw_no_count(17usize, 2usize, 172u32, 2165808u32)?;
    emu.sw_no_count(10usize, 2usize, 144u32, 2165812u32)?;
    emu.sw_no_count(11usize, 2usize, 148u32, 2165816u32)?;
    emu.sw_no_count(12usize, 2usize, 152u32, 2165820u32)?;
    emu.sw_no_count(13usize, 2usize, 156u32, 2165824u32)?;
    emu.sw_no_count(14usize, 2usize, 128u32, 2165828u32)?;
    emu.sw_no_count(15usize, 2usize, 132u32, 2165832u32)?;
    emu.sw_no_count(16usize, 2usize, 136u32, 2165836u32)?;
    emu.sw_no_count(17usize, 2usize, 140u32, 2165840u32)?;
    emu.sw_no_count(10usize, 8usize, 16u32, 2165844u32)?;
    emu.sw_no_count(11usize, 8usize, 20u32, 2165848u32)?;
    emu.sw_no_count(12usize, 8usize, 24u32, 2165852u32)?;
    emu.sw_no_count(13usize, 8usize, 28u32, 2165856u32)?;
    emu.sw_no_count(14usize, 8usize, 0u32, 2165860u32)?;
    emu.sw_no_count(15usize, 8usize, 4u32, 2165864u32)?;
    emu.sw_no_count(16usize, 8usize, 8u32, 2165868u32)?;
    emu.sw_no_count(17usize, 8usize, 12u32, 2165872u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2165876u32);
    emu.adi_no_count(11usize, 2usize, 128u32, 2165880u32);
    emu.adi_no_count(12usize, 2usize, 128u32, 2165884u32);
    emu.apc_no_count(1usize, 2165884u32, 4294942720u32, 2165888u32);
    emu.add_memory_rw_events(29usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2165892u32;
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
pub fn block_0x00210c84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 35u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 192u32, 2165896u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2165900u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2165904u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2165908u32)?;
    emu.lw_no_count(14usize, 2usize, 208u32, 2165912u32)?;
    emu.lw_no_count(15usize, 2usize, 212u32, 2165916u32)?;
    emu.lw_no_count(16usize, 2usize, 216u32, 2165920u32)?;
    emu.lw_no_count(17usize, 2usize, 220u32, 2165924u32)?;
    emu.lw_no_count(5usize, 9usize, 0u32, 2165928u32)?;
    emu.lw_no_count(6usize, 9usize, 4u32, 2165932u32)?;
    emu.lw_no_count(7usize, 9usize, 8u32, 2165936u32)?;
    emu.lw_no_count(28usize, 9usize, 12u32, 2165940u32)?;
    emu.lw_no_count(29usize, 9usize, 16u32, 2165944u32)?;
    emu.lw_no_count(30usize, 9usize, 20u32, 2165948u32)?;
    emu.lw_no_count(31usize, 9usize, 24u32, 2165952u32)?;
    emu.lw_no_count(9usize, 9usize, 28u32, 2165956u32)?;
    emu.xrr_no_count(11usize, 6usize, 11usize, 2165960u32);
    emu.xrr_no_count(10usize, 5usize, 10usize, 2165964u32);
    emu.xrr_no_count(12usize, 7usize, 12usize, 2165968u32);
    emu.xrr_no_count(13usize, 28usize, 13usize, 2165972u32);
    emu.xrr_no_count(14usize, 29usize, 14usize, 2165976u32);
    emu.xrr_no_count(15usize, 30usize, 15usize, 2165980u32);
    emu.xrr_no_count(16usize, 31usize, 16usize, 2165984u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2165988u32);
    emu.orr_no_count(12usize, 12usize, 13usize, 2165992u32);
    emu.orr_no_count(14usize, 14usize, 15usize, 2165996u32);
    emu.orr_no_count(10usize, 10usize, 12usize, 2166000u32);
    emu.orr_no_count(11usize, 14usize, 16usize, 2166004u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2166008u32);
    emu.xrr_no_count(11usize, 9usize, 17usize, 2166012u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2166016u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2166020u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2166024u32);
    emu.apc_no_count(1usize, 2166024u32, 28672u32, 2166028u32);
    emu.add_memory_rw_events(35usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2166032u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965380u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00210d10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(10usize, 8usize, 32u32, 2166036u32);
    emu.lw_no_count(1usize, 2usize, 236u32, 2166040u32)?;
    emu.lw_no_count(8usize, 2usize, 232u32, 2166044u32)?;
    emu.lw_no_count(9usize, 2usize, 228u32, 2166048u32)?;
    emu.lw_no_count(18usize, 2usize, 224u32, 2166052u32)?;
    emu.adi_no_count(2usize, 2usize, 240u32, 2166056u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2166060u32;
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
pub fn block_0x00210d2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 139u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967216u32, 2166064u32);
    emu.sw_no_count(1usize, 2usize, 76u32, 2166068u32)?;
    emu.sw_no_count(8usize, 2usize, 72u32, 2166072u32)?;
    emu.sw_no_count(9usize, 2usize, 68u32, 2166076u32)?;
    emu.sw_no_count(18usize, 2usize, 64u32, 2166080u32)?;
    emu.sw_no_count(19usize, 2usize, 60u32, 2166084u32)?;
    emu.sw_no_count(20usize, 2usize, 56u32, 2166088u32)?;
    emu.sw_no_count(21usize, 2usize, 52u32, 2166092u32)?;
    emu.sw_no_count(22usize, 2usize, 48u32, 2166096u32)?;
    emu.sw_no_count(23usize, 2usize, 44u32, 2166100u32)?;
    emu.sw_no_count(24usize, 2usize, 40u32, 2166104u32)?;
    emu.sw_no_count(25usize, 2usize, 36u32, 2166108u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2166112u32);
    emu.lbu_no_count(10usize, 11usize, 0u32, 2166116u32);
    emu.lbu_no_count(12usize, 11usize, 1u32, 2166120u32);
    emu.lbu_no_count(13usize, 11usize, 2u32, 2166124u32);
    emu.lbu_no_count(14usize, 11usize, 3u32, 2166128u32);
    emu.lbu_no_count(15usize, 11usize, 4u32, 2166132u32);
    emu.lbu_no_count(16usize, 11usize, 5u32, 2166136u32);
    emu.lbu_no_count(17usize, 11usize, 6u32, 2166140u32);
    emu.lbu_no_count(5usize, 11usize, 7u32, 2166144u32);
    emu.lbu_no_count(6usize, 11usize, 8u32, 2166148u32);
    emu.lbu_no_count(7usize, 11usize, 9u32, 2166152u32);
    emu.lbu_no_count(28usize, 11usize, 10u32, 2166156u32);
    emu.lbu_no_count(29usize, 11usize, 11u32, 2166160u32);
    emu.lbu_no_count(30usize, 11usize, 12u32, 2166164u32);
    emu.lbu_no_count(31usize, 11usize, 13u32, 2166168u32);
    emu.lbu_no_count(9usize, 11usize, 14u32, 2166172u32);
    emu.lbu_no_count(18usize, 11usize, 15u32, 2166176u32);
    emu.sli_no_count(13usize, 13usize, 8u32, 2166180u32);
    emu.sli_no_count(12usize, 12usize, 16u32, 2166184u32);
    emu.sli_no_count(19usize, 10usize, 24u32, 2166188u32);
    emu.sli_no_count(17usize, 17usize, 8u32, 2166192u32);
    emu.orr_no_count(10usize, 13usize, 14usize, 2166196u32);
    emu.orr_no_count(12usize, 19usize, 12usize, 2166200u32);
    emu.orr_no_count(13usize, 17usize, 5usize, 2166204u32);
    emu.lbu_no_count(14usize, 11usize, 16u32, 2166208u32);
    emu.lbu_no_count(17usize, 11usize, 17u32, 2166212u32);
    emu.lbu_no_count(5usize, 11usize, 18u32, 2166216u32);
    emu.lbu_no_count(19usize, 11usize, 19u32, 2166220u32);
    emu.sli_no_count(16usize, 16usize, 16u32, 2166224u32);
    emu.sli_no_count(15usize, 15usize, 24u32, 2166228u32);
    emu.sli_no_count(28usize, 28usize, 8u32, 2166232u32);
    emu.sli_no_count(7usize, 7usize, 16u32, 2166236u32);
    emu.sli_no_count(6usize, 6usize, 24u32, 2166240u32);
    emu.sli_no_count(9usize, 9usize, 8u32, 2166244u32);
    emu.orr_no_count(15usize, 15usize, 16usize, 2166248u32);
    emu.orr_no_count(16usize, 28usize, 29usize, 2166252u32);
    emu.orr_no_count(6usize, 6usize, 7usize, 2166256u32);
    emu.orr_no_count(7usize, 9usize, 18usize, 2166260u32);
    emu.lbu_no_count(28usize, 11usize, 20u32, 2166264u32);
    emu.lbu_no_count(29usize, 11usize, 21u32, 2166268u32);
    emu.lbu_no_count(9usize, 11usize, 22u32, 2166272u32);
    emu.lbu_no_count(18usize, 11usize, 23u32, 2166276u32);
    emu.sli_no_count(31usize, 31usize, 16u32, 2166280u32);
    emu.sli_no_count(30usize, 30usize, 24u32, 2166284u32);
    emu.sli_no_count(5usize, 5usize, 8u32, 2166288u32);
    emu.sli_no_count(17usize, 17usize, 16u32, 2166292u32);
    emu.sli_no_count(14usize, 14usize, 24u32, 2166296u32);
    emu.sli_no_count(9usize, 9usize, 8u32, 2166300u32);
    emu.orr_no_count(30usize, 30usize, 31usize, 2166304u32);
    emu.orr_no_count(5usize, 5usize, 19usize, 2166308u32);
    emu.orr_no_count(14usize, 14usize, 17usize, 2166312u32);
    emu.orr_no_count(17usize, 9usize, 18usize, 2166316u32);
    emu.lbu_no_count(31usize, 11usize, 24u32, 2166320u32);
    emu.lbu_no_count(9usize, 11usize, 25u32, 2166324u32);
    emu.lbu_no_count(18usize, 11usize, 26u32, 2166328u32);
    emu.lbu_no_count(19usize, 11usize, 27u32, 2166332u32);
    emu.sli_no_count(29usize, 29usize, 16u32, 2166336u32);
    emu.sli_no_count(28usize, 28usize, 24u32, 2166340u32);
    emu.sli_no_count(18usize, 18usize, 8u32, 2166344u32);
    emu.orr_no_count(28usize, 28usize, 29usize, 2166348u32);
    emu.orr_no_count(29usize, 18usize, 19usize, 2166352u32);
    emu.lbu_no_count(18usize, 11usize, 30u32, 2166356u32);
    emu.lbu_no_count(19usize, 11usize, 31u32, 2166360u32);
    emu.sli_no_count(9usize, 9usize, 16u32, 2166364u32);
    emu.sli_no_count(31usize, 31usize, 24u32, 2166368u32);
    emu.orr_no_count(31usize, 31usize, 9usize, 2166372u32);
    emu.lbu_no_count(9usize, 11usize, 29u32, 2166376u32);
    emu.lbu_no_count(11usize, 11usize, 28u32, 2166380u32);
    emu.sli_no_count(18usize, 18usize, 8u32, 2166384u32);
    emu.orr_no_count(24usize, 18usize, 19usize, 2166388u32);
    emu.sli_no_count(9usize, 9usize, 16u32, 2166392u32);
    emu.sli_no_count(11usize, 11usize, 24u32, 2166396u32);
    emu.orr_no_count(11usize, 11usize, 9usize, 2166400u32);
    emu.orr_no_count(18usize, 12usize, 10usize, 2166404u32);
    emu.orr_no_count(19usize, 15usize, 13usize, 2166408u32);
    emu.orr_no_count(20usize, 6usize, 16usize, 2166412u32);
    emu.orr_no_count(21usize, 30usize, 7usize, 2166416u32);
    emu.orr_no_count(22usize, 14usize, 5usize, 2166420u32);
    emu.orr_no_count(23usize, 28usize, 17usize, 2166424u32);
    emu.orr_no_count(25usize, 31usize, 29usize, 2166428u32);
    emu.orr_no_count(24usize, 11usize, 24usize, 2166432u32);
    emu.adi_no_count(10usize, 24usize, 1u32, 2166436u32);
    emu.sltiu_no_count(10usize, 10usize, 1u32, 2166440u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2166444u32);
    emu.adr_no_count(11usize, 25usize, 10usize, 2166448u32);
    emu.sltru_no_count(12usize, 11usize, 25usize, 2166452u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2166456u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2166460u32);
    emu.sltiu_no_count(11usize, 11usize, 1u32, 2166464u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2166468u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2166472u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2166476u32);
    emu.adr_no_count(11usize, 23usize, 10usize, 2166480u32);
    emu.sltru_no_count(12usize, 11usize, 23usize, 2166484u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2166488u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2166492u32);
    emu.sltiu_no_count(11usize, 11usize, 1u32, 2166496u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2166500u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2166504u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2166508u32);
    emu.adr_no_count(11usize, 10usize, 22usize, 2166512u32);
    emu.sltru_no_count(11usize, 11usize, 10usize, 2166516u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2166520u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2166524u32);
    emu.adr_no_count(11usize, 10usize, 21usize, 2166528u32);
    emu.sltru_no_count(11usize, 11usize, 10usize, 2166532u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2166536u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2166540u32);
    emu.adr_no_count(11usize, 10usize, 20usize, 2166544u32);
    emu.sltru_no_count(11usize, 11usize, 10usize, 2166548u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2166552u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2166556u32);
    emu.adr_no_count(11usize, 19usize, 10usize, 2166560u32);
    emu.sltru_no_count(12usize, 11usize, 19usize, 2166564u32);
    emu.sltiu_no_count(11usize, 11usize, 1u32, 2166568u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2166572u32);
    emu.sbr_no_count(10usize, 10usize, 11usize, 2166576u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2166580u32);
    emu.adr_no_count(11usize, 18usize, 10usize, 2166584u32);
    emu.sltru_no_count(12usize, 11usize, 18usize, 2166588u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2166592u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2166596u32);
    emu.sltiu_no_count(11usize, 11usize, 1u32, 2166600u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2166604u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2166608u32);
    emu.apc_no_count(1usize, 2166608u32, 24576u32, 2166612u32);
    emu.add_memory_rw_events(139usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2166616u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1596u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00210f58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2166620u32);
    emu.sw_no_count(24usize, 2usize, 4u32, 2166624u32)?;
    emu.sw_no_count(25usize, 2usize, 8u32, 2166628u32)?;
    emu.sw_no_count(23usize, 2usize, 12u32, 2166632u32)?;
    emu.sw_no_count(22usize, 2usize, 16u32, 2166636u32)?;
    emu.sw_no_count(21usize, 2usize, 20u32, 2166640u32)?;
    emu.sw_no_count(20usize, 2usize, 24u32, 2166644u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2166648u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2166652u32)?;
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2166656u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 920u32, 2166660u32);
    emu.adi_no_count(11usize, 2usize, 4u32, 2166664u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2166668u32);
    emu.apc_no_count(1usize, 2166668u32, 4294938624u32, 2166672u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2166676u32;
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
pub fn block_0x00210f94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(9usize, 8usize, 32u32, 2166680u32);
    emu.lw_no_count(1usize, 2usize, 76u32, 2166684u32)?;
    emu.lw_no_count(8usize, 2usize, 72u32, 2166688u32)?;
    emu.lw_no_count(9usize, 2usize, 68u32, 2166692u32)?;
    emu.lw_no_count(18usize, 2usize, 64u32, 2166696u32)?;
    emu.lw_no_count(19usize, 2usize, 60u32, 2166700u32)?;
    emu.lw_no_count(20usize, 2usize, 56u32, 2166704u32)?;
    emu.lw_no_count(21usize, 2usize, 52u32, 2166708u32)?;
    emu.lw_no_count(22usize, 2usize, 48u32, 2166712u32)?;
    emu.lw_no_count(23usize, 2usize, 44u32, 2166716u32)?;
    emu.lw_no_count(24usize, 2usize, 40u32, 2166720u32)?;
    emu.lw_no_count(25usize, 2usize, 36u32, 2166724u32)?;
    emu.adi_no_count(2usize, 2usize, 80u32, 2166728u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2166732u32;
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
pub fn block_0x00210fcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 11usize, 0u32, 2166736u32)?;
    emu.lw_no_count(5usize, 11usize, 4u32, 2166740u32)?;
    emu.lw_no_count(14usize, 11usize, 12u32, 2166744u32)?;
    emu.lw_no_count(12usize, 11usize, 16u32, 2166748u32)?;
    emu.lw_no_count(13usize, 11usize, 20u32, 2166752u32)?;
    emu.lw_no_count(7usize, 11usize, 24u32, 2166756u32)?;
    emu.lw_no_count(6usize, 11usize, 28u32, 2166760u32)?;
    emu.adr_no_count(14usize, 16usize, 14usize, 2166764u32);
    emu.sltru_no_count(15usize, 14usize, 16usize, 2166768u32);
    emu.adr_no_count(17usize, 5usize, 15usize, 2166772u32);
    emu.sltru_no_count(15usize, 17usize, 5usize, 2166776u32);
    emu.adr_no_count(12usize, 17usize, 12usize, 2166780u32);
    emu.sltru_no_count(17usize, 12usize, 17usize, 2166784u32);
    emu.adr_no_count(13usize, 15usize, 13usize, 2166788u32);
    emu.adr_no_count(13usize, 13usize, 17usize, 2166792u32);
    emu.add_memory_rw_events(15usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2166800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211010));
    } else {
        emu.pc = 2166796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021100c));
    }
}
#[inline(always)]
pub fn block_0x0021100c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(17usize, 13usize, 15usize, 2166800u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2166800u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211010));
}
#[inline]
pub fn block_0x00211010(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 11usize, 8u32, 2166804u32)?;
    emu.sbr_no_count(28usize, 0usize, 16usize, 2166808u32);
    emu.sbr_no_count(29usize, 5usize, 16usize, 2166812u32);
    emu.adi_no_count(11usize, 0usize, 4294967295u32, 2166816u32);
    emu.sltru_no_count(28usize, 29usize, 28usize, 2166820u32);
    emu.mulhu_no_count(30usize, 16usize, 11usize, 2166824u32);
    emu.sbr_no_count(30usize, 30usize, 5usize, 2166828u32);
    emu.adr_no_count(28usize, 30usize, 28usize, 2166832u32);
    emu.sbr_no_count(30usize, 0usize, 5usize, 2166836u32);
    emu.adr_no_count(7usize, 16usize, 7usize, 2166840u32);
    emu.mulhu_no_count(31usize, 5usize, 11usize, 2166844u32);
    emu.adr_no_count(5usize, 29usize, 6usize, 2166848u32);
    emu.sltru_no_count(16usize, 7usize, 16usize, 2166852u32);
    emu.sltru_no_count(6usize, 28usize, 30usize, 2166856u32);
    emu.adr_no_count(5usize, 5usize, 16usize, 2166860u32);
    emu.adr_no_count(6usize, 31usize, 6usize, 2166864u32);
    emu.add_memory_rw_events(16usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2166872u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211058));
    } else {
        emu.pc = 2166868u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211054));
    }
}
#[inline(always)]
pub fn block_0x00211054(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 5usize, 29usize, 2166872u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2166872u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211058));
}
#[inline]
pub fn block_0x00211058(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(30usize, 28usize, 16usize, 2166876u32);
    emu.adr_no_count(17usize, 7usize, 17usize, 2166880u32);
    emu.adr_no_count(16usize, 13usize, 15usize, 2166884u32);
    emu.sltru_no_count(28usize, 30usize, 28usize, 2166888u32);
    emu.sltru_no_count(7usize, 17usize, 7usize, 2166892u32);
    emu.sltru_no_count(13usize, 16usize, 13usize, 2166896u32);
    emu.adr_no_count(28usize, 6usize, 28usize, 2166900u32);
    emu.adr_no_count(6usize, 5usize, 7usize, 2166904u32);
    emu.adr_no_count(13usize, 14usize, 13usize, 2166908u32);
    emu.sltru_no_count(5usize, 6usize, 5usize, 2166912u32);
    emu.anr_no_count(29usize, 7usize, 5usize, 2166916u32);
    emu.sltru_no_count(7usize, 13usize, 14usize, 2166920u32);
    emu.adr_no_count(13usize, 17usize, 13usize, 2166924u32);
    emu.sltru_no_count(5usize, 13usize, 17usize, 2166928u32);
    emu.adr_no_count(17usize, 6usize, 7usize, 2166932u32);
    emu.adr_no_count(29usize, 30usize, 29usize, 2166936u32);
    emu.sltru_no_count(7usize, 29usize, 30usize, 2166940u32);
    emu.adr_no_count(17usize, 17usize, 5usize, 2166944u32);
    emu.adr_no_count(7usize, 28usize, 7usize, 2166948u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2166956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002110ac));
    } else {
        emu.pc = 2166952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002110a8));
    }
}
#[inline(always)]
pub fn block_0x002110a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 17usize, 6usize, 2166956u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2166956u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002110ac));
}
#[inline(never)]
pub fn block_0x002110ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2166960u32);
    emu.sw_no_count(8usize, 2usize, 44u32, 2166964u32)?;
    emu.sw_no_count(9usize, 2usize, 40u32, 2166968u32)?;
    emu.sw_no_count(18usize, 2usize, 36u32, 2166972u32)?;
    emu.sw_no_count(19usize, 2usize, 32u32, 2166976u32)?;
    emu.sw_no_count(20usize, 2usize, 28u32, 2166980u32)?;
    emu.sw_no_count(21usize, 2usize, 24u32, 2166984u32)?;
    emu.sw_no_count(22usize, 2usize, 20u32, 2166988u32)?;
    emu.sw_no_count(23usize, 2usize, 16u32, 2166992u32)?;
    emu.sw_no_count(24usize, 2usize, 12u32, 2166996u32)?;
    emu.sw_no_count(25usize, 2usize, 8u32, 2167000u32)?;
    emu.sw_no_count(26usize, 2usize, 4u32, 2167004u32)?;
    emu.sw_no_count(27usize, 2usize, 0u32, 2167008u32)?;
    emu.sbr_no_count(6usize, 0usize, 15usize, 2167012u32);
    emu.sbr_no_count(30usize, 14usize, 15usize, 2167016u32);
    emu.mulhu_no_count(28usize, 15usize, 11usize, 2167020u32);
    emu.sltru_no_count(6usize, 30usize, 6usize, 2167024u32);
    emu.sbr_no_count(28usize, 28usize, 14usize, 2167028u32);
    emu.adr_no_count(6usize, 28usize, 6usize, 2167032u32);
    emu.sbr_no_count(31usize, 0usize, 14usize, 2167036u32);
    emu.mulhu_no_count(8usize, 14usize, 11usize, 2167040u32);
    emu.adr_no_count(28usize, 29usize, 15usize, 2167044u32);
    emu.sltru_no_count(15usize, 28usize, 29usize, 2167048u32);
    emu.adr_no_count(14usize, 7usize, 30usize, 2167052u32);
    emu.sltru_no_count(29usize, 6usize, 31usize, 2167056u32);
    emu.adr_no_count(14usize, 14usize, 15usize, 2167060u32);
    emu.adr_no_count(29usize, 8usize, 29usize, 2167064u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2167072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211120));
    } else {
        emu.pc = 2167068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021111c));
    }
}
#[inline(always)]
pub fn block_0x0021111c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 14usize, 7usize, 2167072u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2167072u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211120));
}
#[inline]
pub fn block_0x00211120(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(7usize, 6usize, 15usize, 2167076u32);
    emu.adr_no_count(5usize, 28usize, 5usize, 2167080u32);
    emu.adr_no_count(15usize, 17usize, 12usize, 2167084u32);
    emu.sltru_no_count(6usize, 7usize, 6usize, 2167088u32);
    emu.sltru_no_count(28usize, 5usize, 28usize, 2167092u32);
    emu.sltru_no_count(17usize, 15usize, 17usize, 2167096u32);
    emu.adr_no_count(30usize, 29usize, 6usize, 2167100u32);
    emu.adr_no_count(29usize, 14usize, 28usize, 2167104u32);
    emu.adr_no_count(17usize, 16usize, 17usize, 2167108u32);
    emu.sltru_no_count(14usize, 29usize, 14usize, 2167112u32);
    emu.anr_no_count(28usize, 28usize, 14usize, 2167116u32);
    emu.sltru_no_count(6usize, 17usize, 16usize, 2167120u32);
    emu.adr_no_count(14usize, 17usize, 5usize, 2167124u32);
    emu.sltru_no_count(5usize, 14usize, 17usize, 2167128u32);
    emu.adr_no_count(17usize, 6usize, 29usize, 2167132u32);
    emu.adr_no_count(29usize, 7usize, 28usize, 2167136u32);
    emu.sltru_no_count(7usize, 29usize, 7usize, 2167140u32);
    emu.adr_no_count(17usize, 17usize, 5usize, 2167144u32);
    emu.adr_no_count(7usize, 30usize, 7usize, 2167148u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2167156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211174));
    } else {
        emu.pc = 2167152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211170));
    }
}
#[inline(always)]
pub fn block_0x00211170(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 17usize, 6usize, 2167156u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2167156u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211174));
}
#[inline]
pub fn block_0x00211174(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(30usize, 16usize, 12usize, 2167160u32);
    emu.sbr_no_count(6usize, 0usize, 12usize, 2167164u32);
    emu.mulhu_no_count(28usize, 12usize, 11usize, 2167168u32);
    emu.sltru_no_count(6usize, 30usize, 6usize, 2167172u32);
    emu.sbr_no_count(28usize, 28usize, 16usize, 2167176u32);
    emu.adr_no_count(6usize, 28usize, 6usize, 2167180u32);
    emu.sbr_no_count(31usize, 0usize, 16usize, 2167184u32);
    emu.mulhu_no_count(8usize, 16usize, 11usize, 2167188u32);
    emu.adr_no_count(28usize, 29usize, 12usize, 2167192u32);
    emu.sltru_no_count(12usize, 28usize, 29usize, 2167196u32);
    emu.adr_no_count(16usize, 7usize, 30usize, 2167200u32);
    emu.sltru_no_count(29usize, 6usize, 31usize, 2167204u32);
    emu.adr_no_count(16usize, 16usize, 12usize, 2167208u32);
    emu.adr_no_count(29usize, 8usize, 29usize, 2167212u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2167220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002111b4));
    } else {
        emu.pc = 2167216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002111b0));
    }
}
#[inline(always)]
pub fn block_0x002111b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 16usize, 7usize, 2167220u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2167220u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002111b4));
}
#[inline]
pub fn block_0x002111b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(30usize, 6usize, 12usize, 2167224u32);
    emu.adr_no_count(5usize, 28usize, 5usize, 2167228u32);
    emu.adr_no_count(12usize, 17usize, 13usize, 2167232u32);
    emu.sltru_no_count(6usize, 30usize, 6usize, 2167236u32);
    emu.sltru_no_count(7usize, 5usize, 28usize, 2167240u32);
    emu.sltru_no_count(17usize, 12usize, 17usize, 2167244u32);
    emu.adr_no_count(6usize, 29usize, 6usize, 2167248u32);
    emu.adr_no_count(29usize, 16usize, 7usize, 2167252u32);
    emu.adr_no_count(31usize, 15usize, 17usize, 2167256u32);
    emu.sltru_no_count(16usize, 29usize, 16usize, 2167260u32);
    emu.sltru_no_count(28usize, 31usize, 15usize, 2167264u32);
    emu.adr_no_count(17usize, 31usize, 5usize, 2167268u32);
    emu.anr_no_count(7usize, 7usize, 16usize, 2167272u32);
    emu.sltru_no_count(5usize, 17usize, 31usize, 2167276u32);
    emu.adr_no_count(16usize, 28usize, 5usize, 2167280u32);
    emu.adr_no_count(16usize, 16usize, 29usize, 2167284u32);
    emu.adr_no_count(7usize, 30usize, 7usize, 2167288u32);
    emu.sltru_no_count(29usize, 7usize, 30usize, 2167292u32);
    emu.adr_no_count(6usize, 6usize, 29usize, 2167296u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2167304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211208));
    } else {
        emu.pc = 2167300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211204));
    }
}
#[inline(always)]
pub fn block_0x00211204(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 16usize, 28usize, 2167304u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2167304u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211208));
}
#[inline]
pub fn block_0x00211208(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(31usize, 15usize, 13usize, 2167308u32);
    emu.sbr_no_count(28usize, 0usize, 13usize, 2167312u32);
    emu.mulhu_no_count(29usize, 13usize, 11usize, 2167316u32);
    emu.sltru_no_count(28usize, 31usize, 28usize, 2167320u32);
    emu.sbr_no_count(29usize, 29usize, 15usize, 2167324u32);
    emu.adr_no_count(28usize, 29usize, 28usize, 2167328u32);
    emu.sbr_no_count(8usize, 0usize, 15usize, 2167332u32);
    emu.adr_no_count(29usize, 7usize, 13usize, 2167336u32);
    emu.sltru_no_count(30usize, 29usize, 7usize, 2167340u32);
    emu.adr_no_count(13usize, 6usize, 31usize, 2167344u32);
    emu.adr_no_count(13usize, 13usize, 30usize, 2167348u32);
    emu.sltru_no_count(7usize, 28usize, 8usize, 2167352u32);
    emu.mulhu_no_count(11usize, 15usize, 11usize, 2167356u32);
    emu.add_memory_rw_events(13usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2167364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211244));
    } else {
        emu.pc = 2167360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211240));
    }
}
#[inline(always)]
pub fn block_0x00211240(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(30usize, 13usize, 6usize, 2167364u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2167364u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211244));
}
#[inline]
pub fn block_0x00211244(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(7usize, 11usize, 7usize, 2167368u32);
    emu.adr_no_count(6usize, 28usize, 30usize, 2167372u32);
    emu.adr_no_count(15usize, 29usize, 5usize, 2167376u32);
    emu.adi_no_count(11usize, 14usize, 1u32, 2167380u32);
    emu.sltru_no_count(14usize, 6usize, 28usize, 2167384u32);
    emu.sltru_no_count(28usize, 15usize, 29usize, 2167388u32);
    emu.sltiu_no_count(29usize, 11usize, 1u32, 2167392u32);
    emu.adr_no_count(5usize, 13usize, 28usize, 2167396u32);
    emu.adr_no_count(12usize, 12usize, 29usize, 2167400u32);
    emu.sltru_no_count(13usize, 5usize, 13usize, 2167404u32);
    emu.orr_no_count(29usize, 11usize, 12usize, 2167408u32);
    emu.anr_no_count(31usize, 28usize, 13usize, 2167412u32);
    emu.sltiu_no_count(30usize, 29usize, 1u32, 2167416u32);
    emu.adi_no_count(30usize, 30usize, 4294967295u32, 2167420u32);
    emu.adr_no_count(28usize, 17usize, 30usize, 2167424u32);
    emu.sltru_no_count(13usize, 28usize, 17usize, 2167428u32);
    emu.adr_no_count(29usize, 16usize, 30usize, 2167432u32);
    emu.adr_no_count(29usize, 29usize, 13usize, 2167436u32);
    emu.adr_no_count(17usize, 6usize, 31usize, 2167440u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(29usize);
    if a == b {
        emu.pc = 2167448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211298));
    } else {
        emu.pc = 2167444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211294));
    }
}
#[inline(always)]
pub fn block_0x00211294(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(13usize, 29usize, 16usize, 2167448u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2167448u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211298));
}
#[inline]
pub fn block_0x00211298(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(7usize, 7usize, 14usize, 2167452u32);
    emu.adr_no_count(16usize, 30usize, 13usize, 2167456u32);
    emu.adi_no_count(13usize, 28usize, 1u32, 2167460u32);
    emu.sltru_no_count(14usize, 16usize, 30usize, 2167464u32);
    emu.adr_no_count(30usize, 30usize, 14usize, 2167468u32);
    emu.sltiu_no_count(14usize, 13usize, 1u32, 2167472u32);
    emu.adr_no_count(14usize, 29usize, 14usize, 2167476u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2167480u32);
    emu.sltru_no_count(6usize, 17usize, 6usize, 2167484u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2167496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002112c8));
    } else {
        emu.pc = 2167488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002112c0));
    }
}
#[inline(always)]
pub fn block_0x002112c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 14usize, 29usize, 2167492u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2167496u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2167500u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002112cc));
}
#[inline(always)]
pub fn block_0x002112c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 13usize, 28usize, 2167500u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2167500u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002112cc));
}
#[inline]
pub fn block_0x002112cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(29usize, 16usize, 1u32, 2167504u32);
    emu.sbr_no_count(29usize, 30usize, 29usize, 2167508u32);
    emu.adi_no_count(16usize, 16usize, 4294967295u32, 2167512u32);
    emu.adr_no_count(28usize, 16usize, 28usize, 2167516u32);
    emu.sltru_no_count(16usize, 28usize, 16usize, 2167520u32);
    emu.adr_no_count(16usize, 29usize, 16usize, 2167524u32);
    emu.sai_no_count(28usize, 16usize, 1055u32, 2167528u32);
    emu.adr_no_count(15usize, 28usize, 15usize, 2167532u32);
    emu.sltru_no_count(29usize, 15usize, 28usize, 2167536u32);
    emu.adr_no_count(16usize, 28usize, 5usize, 2167540u32);
    emu.adr_no_count(16usize, 16usize, 29usize, 2167544u32);
    emu.adr_no_count(7usize, 7usize, 6usize, 2167548u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2167556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211304));
    } else {
        emu.pc = 2167552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211300));
    }
}
#[inline(always)]
pub fn block_0x00211300(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(29usize, 16usize, 28usize, 2167556u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2167556u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211304));
}
#[inline]
pub fn block_0x00211304(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(29usize, 28usize, 29usize, 2167560u32);
    emu.sltru_no_count(5usize, 29usize, 28usize, 2167564u32);
    emu.adr_no_count(5usize, 28usize, 5usize, 2167568u32);
    emu.sai_no_count(28usize, 5usize, 1055u32, 2167572u32);
    emu.adr_no_count(30usize, 7usize, 28usize, 2167576u32);
    emu.adr_no_count(6usize, 17usize, 28usize, 2167580u32);
    emu.sltru_no_count(5usize, 6usize, 17usize, 2167584u32);
    emu.adr_no_count(30usize, 30usize, 5usize, 2167588u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(30usize);
    if a == b {
        emu.pc = 2167596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021132c));
    } else {
        emu.pc = 2167592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211328));
    }
}
#[inline(always)]
pub fn block_0x00211328(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 30usize, 7usize, 2167596u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2167596u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021132c));
}
#[inline(always)]
pub fn block_0x0021132c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(5usize, 28usize, 5usize, 2167600u32);
    emu.sltru_no_count(17usize, 0usize, 6usize, 2167604u32);
    emu.sltru_no_count(7usize, 5usize, 28usize, 2167608u32);
    emu.adr_no_count(17usize, 30usize, 17usize, 2167612u32);
    emu.adr_no_count(28usize, 28usize, 7usize, 2167616u32);
    emu.adi_no_count(29usize, 6usize, 4294967295u32, 2167620u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2167632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211350));
    } else {
        emu.pc = 2167624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211348));
    }
}
#[inline(always)]
pub fn block_0x00211348(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(6usize, 17usize, 30usize, 2167628u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2167632u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2167636u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211354));
}
#[inline(always)]
pub fn block_0x00211350(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(6usize, 29usize, 6usize, 2167636u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2167636u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211354));
}
#[inline]
pub fn block_0x00211354(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(7usize, 5usize, 4294967295u32, 2167640u32);
    emu.sltiu_no_count(5usize, 5usize, 1u32, 2167644u32);
    emu.adr_no_count(6usize, 7usize, 6usize, 2167648u32);
    emu.sbr_no_count(5usize, 28usize, 5usize, 2167652u32);
    emu.sltru_no_count(6usize, 6usize, 7usize, 2167656u32);
    emu.adr_no_count(5usize, 5usize, 6usize, 2167660u32);
    emu.sai_no_count(30usize, 5usize, 1055u32, 2167664u32);
    emu.adr_no_count(11usize, 30usize, 11usize, 2167668u32);
    emu.sltru_no_count(5usize, 11usize, 30usize, 2167672u32);
    emu.adr_no_count(12usize, 30usize, 12usize, 2167676u32);
    emu.adr_no_count(12usize, 12usize, 5usize, 2167680u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(30usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2167688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211388));
    } else {
        emu.pc = 2167684u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211384));
    }
}
#[inline(always)]
pub fn block_0x00211384(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 12usize, 30usize, 2167688u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2167688u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211388));
}
#[inline]
pub fn block_0x00211388(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(6usize, 30usize, 13usize, 2167692u32);
    emu.sltru_no_count(7usize, 6usize, 30usize, 2167696u32);
    emu.adr_no_count(13usize, 6usize, 5usize, 2167700u32);
    emu.adr_no_count(5usize, 14usize, 7usize, 2167704u32);
    emu.sltru_no_count(6usize, 13usize, 6usize, 2167708u32);
    emu.sltru_no_count(14usize, 0usize, 5usize, 2167712u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2167716u32);
    emu.anr_no_count(7usize, 14usize, 7usize, 2167720u32);
    emu.adr_no_count(14usize, 5usize, 6usize, 2167724u32);
    emu.sltru_no_count(5usize, 14usize, 5usize, 2167728u32);
    emu.anr_no_count(5usize, 6usize, 5usize, 2167732u32);
    emu.adr_no_count(5usize, 7usize, 5usize, 2167736u32);
    emu.sltru_no_count(7usize, 5usize, 7usize, 2167740u32);
    emu.adr_no_count(6usize, 5usize, 15usize, 2167744u32);
    emu.sltru_no_count(5usize, 6usize, 5usize, 2167748u32);
    emu.adr_no_count(16usize, 7usize, 16usize, 2167752u32);
    emu.adr_no_count(28usize, 16usize, 5usize, 2167756u32);
    emu.add_memory_rw_events(17usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2167764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002113d4));
    } else {
        emu.pc = 2167760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002113d0));
    }
}
#[inline(always)]
pub fn block_0x002113d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 28usize, 7usize, 2167764u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2167764u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002113d4));
}
#[inline(never)]
pub fn block_0x002113d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 78u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(7usize, 30usize, 1u32, 2167768u32);
    emu.sbr_no_count(16usize, 29usize, 30usize, 2167772u32);
    emu.adr_no_count(17usize, 30usize, 17usize, 2167776u32);
    emu.sri_no_count(8usize, 28usize, 24u32, 2167780u32);
    emu.sri_no_count(9usize, 6usize, 24u32, 2167784u32);
    emu.sri_no_count(31usize, 14usize, 24u32, 2167788u32);
    emu.sri_no_count(30usize, 13usize, 24u32, 2167792u32);
    emu.sri_no_count(29usize, 12usize, 24u32, 2167796u32);
    emu.sri_no_count(15usize, 11usize, 24u32, 2167800u32);
    emu.sri_no_count(18usize, 28usize, 8u32, 2167804u32);
    emu.sri_no_count(19usize, 28usize, 16u32, 2167808u32);
    emu.sri_no_count(20usize, 6usize, 8u32, 2167812u32);
    emu.sri_no_count(21usize, 6usize, 16u32, 2167816u32);
    emu.sri_no_count(22usize, 14usize, 8u32, 2167820u32);
    emu.sri_no_count(23usize, 14usize, 16u32, 2167824u32);
    emu.sri_no_count(24usize, 13usize, 8u32, 2167828u32);
    emu.sri_no_count(25usize, 13usize, 16u32, 2167832u32);
    emu.sri_no_count(26usize, 12usize, 8u32, 2167836u32);
    emu.sri_no_count(27usize, 12usize, 16u32, 2167840u32);
    emu.sb_no_count(8usize, 10usize, 8u32, 2167844u32);
    emu.sb_no_count(19usize, 10usize, 9u32, 2167848u32);
    emu.sb_no_count(18usize, 10usize, 10u32, 2167852u32);
    emu.sb_no_count(28usize, 10usize, 11u32, 2167856u32);
    emu.sri_no_count(28usize, 11usize, 8u32, 2167860u32);
    emu.sb_no_count(9usize, 10usize, 12u32, 2167864u32);
    emu.sb_no_count(21usize, 10usize, 13u32, 2167868u32);
    emu.sb_no_count(20usize, 10usize, 14u32, 2167872u32);
    emu.sb_no_count(6usize, 10usize, 15u32, 2167876u32);
    emu.sri_no_count(6usize, 11usize, 16u32, 2167880u32);
    emu.sltru_no_count(7usize, 16usize, 7usize, 2167884u32);
    emu.adr_no_count(5usize, 16usize, 5usize, 2167888u32);
    emu.sb_no_count(31usize, 10usize, 16u32, 2167892u32);
    emu.sb_no_count(23usize, 10usize, 17u32, 2167896u32);
    emu.sb_no_count(22usize, 10usize, 18u32, 2167900u32);
    emu.sb_no_count(14usize, 10usize, 19u32, 2167904u32);
    emu.sb_no_count(30usize, 10usize, 20u32, 2167908u32);
    emu.sb_no_count(25usize, 10usize, 21u32, 2167912u32);
    emu.sb_no_count(24usize, 10usize, 22u32, 2167916u32);
    emu.sb_no_count(13usize, 10usize, 23u32, 2167920u32);
    emu.sb_no_count(29usize, 10usize, 24u32, 2167924u32);
    emu.sb_no_count(27usize, 10usize, 25u32, 2167928u32);
    emu.sb_no_count(26usize, 10usize, 26u32, 2167932u32);
    emu.sb_no_count(12usize, 10usize, 27u32, 2167936u32);
    emu.adr_no_count(17usize, 17usize, 7usize, 2167940u32);
    emu.sltru_no_count(12usize, 5usize, 16usize, 2167944u32);
    emu.sri_no_count(13usize, 5usize, 24u32, 2167948u32);
    emu.sri_no_count(14usize, 5usize, 8u32, 2167952u32);
    emu.sri_no_count(16usize, 5usize, 16u32, 2167956u32);
    emu.adr_no_count(12usize, 17usize, 12usize, 2167960u32);
    emu.sb_no_count(13usize, 10usize, 4u32, 2167964u32);
    emu.sb_no_count(16usize, 10usize, 5u32, 2167968u32);
    emu.sb_no_count(14usize, 10usize, 6u32, 2167972u32);
    emu.sb_no_count(5usize, 10usize, 7u32, 2167976u32);
    emu.sri_no_count(13usize, 12usize, 24u32, 2167980u32);
    emu.sri_no_count(14usize, 12usize, 8u32, 2167984u32);
    emu.sri_no_count(16usize, 12usize, 16u32, 2167988u32);
    emu.sb_no_count(13usize, 10usize, 0u32, 2167992u32);
    emu.sb_no_count(16usize, 10usize, 1u32, 2167996u32);
    emu.sb_no_count(14usize, 10usize, 2u32, 2168000u32);
    emu.sb_no_count(12usize, 10usize, 3u32, 2168004u32);
    emu.sb_no_count(15usize, 10usize, 28u32, 2168008u32);
    emu.sb_no_count(6usize, 10usize, 29u32, 2168012u32);
    emu.sb_no_count(28usize, 10usize, 30u32, 2168016u32);
    emu.sb_no_count(11usize, 10usize, 31u32, 2168020u32);
    emu.lw_no_count(8usize, 2usize, 44u32, 2168024u32)?;
    emu.lw_no_count(9usize, 2usize, 40u32, 2168028u32)?;
    emu.lw_no_count(18usize, 2usize, 36u32, 2168032u32)?;
    emu.lw_no_count(19usize, 2usize, 32u32, 2168036u32)?;
    emu.lw_no_count(20usize, 2usize, 28u32, 2168040u32)?;
    emu.lw_no_count(21usize, 2usize, 24u32, 2168044u32)?;
    emu.lw_no_count(22usize, 2usize, 20u32, 2168048u32)?;
    emu.lw_no_count(23usize, 2usize, 16u32, 2168052u32)?;
    emu.lw_no_count(24usize, 2usize, 12u32, 2168056u32)?;
    emu.lw_no_count(25usize, 2usize, 8u32, 2168060u32)?;
    emu.lw_no_count(26usize, 2usize, 4u32, 2168064u32)?;
    emu.lw_no_count(27usize, 2usize, 0u32, 2168068u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2168072u32);
    emu.add_memory_rw_events(78usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2168076u32;
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
pub fn block_0x0021150c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 10usize, 0u32, 2168080u32)?;
    emu.lw_no_count(17usize, 10usize, 4u32, 2168084u32)?;
    emu.lw_no_count(13usize, 10usize, 12u32, 2168088u32)?;
    emu.lw_no_count(11usize, 10usize, 16u32, 2168092u32)?;
    emu.lw_no_count(12usize, 10usize, 20u32, 2168096u32)?;
    emu.lw_no_count(7usize, 10usize, 24u32, 2168100u32)?;
    emu.lw_no_count(5usize, 10usize, 28u32, 2168104u32)?;
    emu.adr_no_count(13usize, 15usize, 13usize, 2168108u32);
    emu.sltru_no_count(14usize, 13usize, 15usize, 2168112u32);
    emu.adr_no_count(16usize, 17usize, 14usize, 2168116u32);
    emu.sltru_no_count(14usize, 16usize, 17usize, 2168120u32);
    emu.adr_no_count(11usize, 16usize, 11usize, 2168124u32);
    emu.sltru_no_count(16usize, 11usize, 16usize, 2168128u32);
    emu.adr_no_count(12usize, 14usize, 12usize, 2168132u32);
    emu.adr_no_count(12usize, 12usize, 16usize, 2168136u32);
    emu.add_memory_rw_events(15usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2168144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211550));
    } else {
        emu.pc = 2168140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021154c));
    }
}
#[inline(always)]
pub fn block_0x0021154c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 12usize, 14usize, 2168144u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2168144u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211550));
}
#[inline]
pub fn block_0x00211550(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 10usize, 8u32, 2168148u32)?;
    emu.sbr_no_count(6usize, 0usize, 15usize, 2168152u32);
    emu.sbr_no_count(28usize, 17usize, 15usize, 2168156u32);
    emu.adi_no_count(10usize, 0usize, 4294967295u32, 2168160u32);
    emu.sltru_no_count(6usize, 28usize, 6usize, 2168164u32);
    emu.mulhu_no_count(29usize, 15usize, 10usize, 2168168u32);
    emu.sbr_no_count(29usize, 29usize, 17usize, 2168172u32);
    emu.adr_no_count(6usize, 29usize, 6usize, 2168176u32);
    emu.sbr_no_count(29usize, 0usize, 17usize, 2168180u32);
    emu.adr_no_count(7usize, 15usize, 7usize, 2168184u32);
    emu.mulhu_no_count(30usize, 17usize, 10usize, 2168188u32);
    emu.adr_no_count(17usize, 28usize, 5usize, 2168192u32);
    emu.sltru_no_count(15usize, 7usize, 15usize, 2168196u32);
    emu.sltru_no_count(5usize, 6usize, 29usize, 2168200u32);
    emu.adr_no_count(17usize, 17usize, 15usize, 2168204u32);
    emu.adr_no_count(5usize, 30usize, 5usize, 2168208u32);
    emu.add_memory_rw_events(16usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2168216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211598));
    } else {
        emu.pc = 2168212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211594));
    }
}
#[inline(always)]
pub fn block_0x00211594(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 17usize, 28usize, 2168216u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2168216u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211598));
}
#[inline]
pub fn block_0x00211598(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(29usize, 6usize, 15usize, 2168220u32);
    emu.adr_no_count(16usize, 7usize, 16usize, 2168224u32);
    emu.adr_no_count(15usize, 12usize, 14usize, 2168228u32);
    emu.sltru_no_count(6usize, 29usize, 6usize, 2168232u32);
    emu.sltru_no_count(7usize, 16usize, 7usize, 2168236u32);
    emu.sltru_no_count(12usize, 15usize, 12usize, 2168240u32);
    emu.adr_no_count(6usize, 5usize, 6usize, 2168244u32);
    emu.adr_no_count(5usize, 17usize, 7usize, 2168248u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2168252u32);
    emu.sltru_no_count(17usize, 5usize, 17usize, 2168256u32);
    emu.anr_no_count(28usize, 7usize, 17usize, 2168260u32);
    emu.sltru_no_count(7usize, 12usize, 13usize, 2168264u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2168268u32);
    emu.sltru_no_count(17usize, 12usize, 16usize, 2168272u32);
    emu.adr_no_count(16usize, 5usize, 7usize, 2168276u32);
    emu.adr_no_count(28usize, 29usize, 28usize, 2168280u32);
    emu.sltru_no_count(7usize, 28usize, 29usize, 2168284u32);
    emu.adr_no_count(16usize, 16usize, 17usize, 2168288u32);
    emu.adr_no_count(6usize, 6usize, 7usize, 2168292u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2168300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002115ec));
    } else {
        emu.pc = 2168296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002115e8));
    }
}
#[inline(always)]
pub fn block_0x002115e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(17usize, 16usize, 5usize, 2168300u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2168300u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002115ec));
}
#[inline]
pub fn block_0x002115ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(5usize, 0usize, 14usize, 2168304u32);
    emu.sbr_no_count(29usize, 13usize, 14usize, 2168308u32);
    emu.mulhu_no_count(7usize, 14usize, 10usize, 2168312u32);
    emu.sltru_no_count(5usize, 29usize, 5usize, 2168316u32);
    emu.sbr_no_count(7usize, 7usize, 13usize, 2168320u32);
    emu.adr_no_count(5usize, 7usize, 5usize, 2168324u32);
    emu.sbr_no_count(30usize, 0usize, 13usize, 2168328u32);
    emu.mulhu_no_count(31usize, 13usize, 10usize, 2168332u32);
    emu.adr_no_count(7usize, 28usize, 14usize, 2168336u32);
    emu.sltru_no_count(14usize, 7usize, 28usize, 2168340u32);
    emu.adr_no_count(13usize, 6usize, 29usize, 2168344u32);
    emu.sltru_no_count(28usize, 5usize, 30usize, 2168348u32);
    emu.adr_no_count(13usize, 13usize, 14usize, 2168352u32);
    emu.adr_no_count(28usize, 31usize, 28usize, 2168356u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2168364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021162c));
    } else {
        emu.pc = 2168360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211628));
    }
}
#[inline(always)]
pub fn block_0x00211628(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(14usize, 13usize, 6usize, 2168364u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2168364u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021162c));
}
#[inline]
pub fn block_0x0021162c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(6usize, 5usize, 14usize, 2168368u32);
    emu.adr_no_count(17usize, 7usize, 17usize, 2168372u32);
    emu.adr_no_count(14usize, 16usize, 11usize, 2168376u32);
    emu.sltru_no_count(5usize, 6usize, 5usize, 2168380u32);
    emu.sltru_no_count(7usize, 17usize, 7usize, 2168384u32);
    emu.sltru_no_count(16usize, 14usize, 16usize, 2168388u32);
    emu.adr_no_count(29usize, 28usize, 5usize, 2168392u32);
    emu.adr_no_count(28usize, 13usize, 7usize, 2168396u32);
    emu.adr_no_count(16usize, 15usize, 16usize, 2168400u32);
    emu.sltru_no_count(13usize, 28usize, 13usize, 2168404u32);
    emu.anr_no_count(7usize, 7usize, 13usize, 2168408u32);
    emu.sltru_no_count(5usize, 16usize, 15usize, 2168412u32);
    emu.adr_no_count(13usize, 16usize, 17usize, 2168416u32);
    emu.sltru_no_count(17usize, 13usize, 16usize, 2168420u32);
    emu.adr_no_count(16usize, 5usize, 28usize, 2168424u32);
    emu.adr_no_count(28usize, 6usize, 7usize, 2168428u32);
    emu.sltru_no_count(6usize, 28usize, 6usize, 2168432u32);
    emu.adr_no_count(16usize, 16usize, 17usize, 2168436u32);
    emu.adr_no_count(6usize, 29usize, 6usize, 2168440u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2168448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211680));
    } else {
        emu.pc = 2168444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021167c));
    }
}
#[inline(always)]
pub fn block_0x0021167c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(17usize, 16usize, 5usize, 2168448u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2168448u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211680));
}
#[inline]
pub fn block_0x00211680(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(29usize, 15usize, 11usize, 2168452u32);
    emu.sbr_no_count(5usize, 0usize, 11usize, 2168456u32);
    emu.mulhu_no_count(7usize, 11usize, 10usize, 2168460u32);
    emu.sltru_no_count(5usize, 29usize, 5usize, 2168464u32);
    emu.sbr_no_count(7usize, 7usize, 15usize, 2168468u32);
    emu.adr_no_count(5usize, 7usize, 5usize, 2168472u32);
    emu.sbr_no_count(30usize, 0usize, 15usize, 2168476u32);
    emu.mulhu_no_count(31usize, 15usize, 10usize, 2168480u32);
    emu.adr_no_count(7usize, 28usize, 11usize, 2168484u32);
    emu.sltru_no_count(15usize, 7usize, 28usize, 2168488u32);
    emu.adr_no_count(11usize, 6usize, 29usize, 2168492u32);
    emu.sltru_no_count(28usize, 5usize, 30usize, 2168496u32);
    emu.adr_no_count(11usize, 11usize, 15usize, 2168500u32);
    emu.adr_no_count(28usize, 31usize, 28usize, 2168504u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2168512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002116c0));
    } else {
        emu.pc = 2168508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002116bc));
    }
}
#[inline(always)]
pub fn block_0x002116bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 11usize, 6usize, 2168512u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2168512u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002116c0));
}
#[inline]
pub fn block_0x002116c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(29usize, 5usize, 15usize, 2168516u32);
    emu.adr_no_count(17usize, 7usize, 17usize, 2168520u32);
    emu.adr_no_count(15usize, 16usize, 12usize, 2168524u32);
    emu.sltru_no_count(5usize, 29usize, 5usize, 2168528u32);
    emu.sltru_no_count(6usize, 17usize, 7usize, 2168532u32);
    emu.sltru_no_count(16usize, 15usize, 16usize, 2168536u32);
    emu.adr_no_count(28usize, 28usize, 5usize, 2168540u32);
    emu.adr_no_count(30usize, 11usize, 6usize, 2168544u32);
    emu.adr_no_count(5usize, 14usize, 16usize, 2168548u32);
    emu.sltru_no_count(11usize, 30usize, 11usize, 2168552u32);
    emu.sltru_no_count(7usize, 5usize, 14usize, 2168556u32);
    emu.adr_no_count(16usize, 5usize, 17usize, 2168560u32);
    emu.anr_no_count(6usize, 6usize, 11usize, 2168564u32);
    emu.sltru_no_count(5usize, 16usize, 5usize, 2168568u32);
    emu.adr_no_count(11usize, 7usize, 5usize, 2168572u32);
    emu.adr_no_count(11usize, 11usize, 30usize, 2168576u32);
    emu.adr_no_count(6usize, 29usize, 6usize, 2168580u32);
    emu.sltru_no_count(17usize, 6usize, 29usize, 2168584u32);
    emu.adr_no_count(17usize, 28usize, 17usize, 2168588u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2168596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211714));
    } else {
        emu.pc = 2168592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211710));
    }
}
#[inline(always)]
pub fn block_0x00211710(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 11usize, 7usize, 2168596u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2168596u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211714));
}
#[inline]
pub fn block_0x00211714(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(28usize, 14usize, 12usize, 2168600u32);
    emu.sbr_no_count(7usize, 0usize, 12usize, 2168604u32);
    emu.mulhu_no_count(29usize, 12usize, 10usize, 2168608u32);
    emu.sltru_no_count(7usize, 28usize, 7usize, 2168612u32);
    emu.sbr_no_count(29usize, 29usize, 14usize, 2168616u32);
    emu.adr_no_count(7usize, 29usize, 7usize, 2168620u32);
    emu.sbr_no_count(30usize, 0usize, 14usize, 2168624u32);
    emu.adr_no_count(29usize, 6usize, 12usize, 2168628u32);
    emu.sltru_no_count(12usize, 29usize, 6usize, 2168632u32);
    emu.adr_no_count(28usize, 17usize, 28usize, 2168636u32);
    emu.adr_no_count(28usize, 28usize, 12usize, 2168640u32);
    emu.sltru_no_count(6usize, 7usize, 30usize, 2168644u32);
    emu.mulhu_no_count(10usize, 14usize, 10usize, 2168648u32);
    emu.add_memory_rw_events(13usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2168656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211750));
    } else {
        emu.pc = 2168652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021174c));
    }
}
#[inline(always)]
pub fn block_0x0021174c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 28usize, 17usize, 2168656u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2168656u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211750));
}
#[inline]
pub fn block_0x00211750(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(6usize, 10usize, 6usize, 2168660u32);
    emu.adr_no_count(17usize, 7usize, 12usize, 2168664u32);
    emu.adr_no_count(12usize, 29usize, 5usize, 2168668u32);
    emu.adi_no_count(10usize, 13usize, 1u32, 2168672u32);
    emu.sltru_no_count(30usize, 17usize, 7usize, 2168676u32);
    emu.sltru_no_count(13usize, 12usize, 29usize, 2168680u32);
    emu.sltiu_no_count(5usize, 10usize, 1u32, 2168684u32);
    emu.adr_no_count(14usize, 28usize, 13usize, 2168688u32);
    emu.adr_no_count(15usize, 15usize, 5usize, 2168692u32);
    emu.sltru_no_count(5usize, 14usize, 28usize, 2168696u32);
    emu.orr_no_count(15usize, 10usize, 15usize, 2168700u32);
    emu.anr_no_count(13usize, 13usize, 5usize, 2168704u32);
    emu.sltiu_no_count(15usize, 15usize, 1u32, 2168708u32);
    emu.adi_no_count(15usize, 15usize, 4294967295u32, 2168712u32);
    emu.adr_no_count(5usize, 16usize, 15usize, 2168716u32);
    emu.sltru_no_count(16usize, 5usize, 16usize, 2168720u32);
    emu.adr_no_count(7usize, 11usize, 15usize, 2168724u32);
    emu.adr_no_count(7usize, 7usize, 16usize, 2168728u32);
    emu.adr_no_count(13usize, 17usize, 13usize, 2168732u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2168740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002117a4));
    } else {
        emu.pc = 2168736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002117a0));
    }
}
#[inline(always)]
pub fn block_0x002117a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 7usize, 11usize, 2168740u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2168740u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002117a4));
}
#[inline]
pub fn block_0x002117a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 6usize, 30usize, 2168744u32);
    emu.adr_no_count(16usize, 15usize, 16usize, 2168748u32);
    emu.adi_no_count(28usize, 5usize, 1u32, 2168752u32);
    emu.sltru_no_count(6usize, 16usize, 15usize, 2168756u32);
    emu.adr_no_count(6usize, 15usize, 6usize, 2168760u32);
    emu.sltiu_no_count(15usize, 28usize, 1u32, 2168764u32);
    emu.adr_no_count(15usize, 7usize, 15usize, 2168768u32);
    emu.adi_no_count(29usize, 15usize, 4294967295u32, 2168772u32);
    emu.sltru_no_count(15usize, 13usize, 17usize, 2168776u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(29usize);
    if a == b {
        emu.pc = 2168788u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002117d4));
    } else {
        emu.pc = 2168780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002117cc));
    }
}
#[inline(always)]
pub fn block_0x002117cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(17usize, 29usize, 7usize, 2168784u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2168788u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2168792u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002117d8));
}
#[inline(always)]
pub fn block_0x002117d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(17usize, 28usize, 5usize, 2168792u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2168792u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002117d8));
}
#[inline]
pub fn block_0x002117d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(5usize, 16usize, 1u32, 2168796u32);
    emu.sbr_no_count(5usize, 6usize, 5usize, 2168800u32);
    emu.adi_no_count(16usize, 16usize, 4294967295u32, 2168804u32);
    emu.adr_no_count(17usize, 16usize, 17usize, 2168808u32);
    emu.sltru_no_count(16usize, 17usize, 16usize, 2168812u32);
    emu.adr_no_count(16usize, 5usize, 16usize, 2168816u32);
    emu.sai_no_count(16usize, 16usize, 1055u32, 2168820u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2168824u32);
    emu.sltru_no_count(12usize, 12usize, 16usize, 2168828u32);
    emu.adr_no_count(14usize, 16usize, 14usize, 2168832u32);
    emu.adr_no_count(14usize, 14usize, 12usize, 2168836u32);
    emu.adr_no_count(11usize, 11usize, 15usize, 2168840u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2168848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211810));
    } else {
        emu.pc = 2168844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021180c));
    }
}
#[inline(always)]
pub fn block_0x0021180c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 14usize, 16usize, 2168848u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2168848u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211810));
}
#[inline]
pub fn block_0x00211810(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(12usize, 16usize, 12usize, 2168852u32);
    emu.sltru_no_count(12usize, 12usize, 16usize, 2168856u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2168860u32);
    emu.sai_no_count(15usize, 12usize, 1055u32, 2168864u32);
    emu.adr_no_count(16usize, 11usize, 15usize, 2168868u32);
    emu.adr_no_count(14usize, 13usize, 15usize, 2168872u32);
    emu.sltru_no_count(12usize, 14usize, 13usize, 2168876u32);
    emu.adr_no_count(13usize, 16usize, 12usize, 2168880u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2168888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211838));
    } else {
        emu.pc = 2168884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211834));
    }
}
#[inline(always)]
pub fn block_0x00211834(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 13usize, 11usize, 2168888u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2168888u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211838));
}
#[inline(always)]
pub fn block_0x00211838(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(12usize, 15usize, 12usize, 2168892u32);
    emu.sltru_no_count(11usize, 12usize, 15usize, 2168896u32);
    emu.adr_no_count(11usize, 15usize, 11usize, 2168900u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2168920u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211858));
    } else {
        emu.pc = 2168904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211848));
    }
}
#[inline(always)]
pub fn block_0x00211848(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(14usize, 0usize, 14usize, 2168908u32);
    emu.adr_no_count(14usize, 13usize, 14usize, 2168912u32);
    emu.sltru_no_count(13usize, 14usize, 13usize, 2168916u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2168920u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2168928u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211860));
}
#[inline(always)]
pub fn block_0x00211858(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 14usize, 4294967295u32, 2168924u32);
    emu.sltru_no_count(13usize, 13usize, 14usize, 2168928u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2168928u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211860));
}
#[inline]
pub fn block_0x00211860(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 12usize, 4294967295u32, 2168932u32);
    emu.sltiu_no_count(12usize, 12usize, 1u32, 2168936u32);
    emu.adr_no_count(13usize, 14usize, 13usize, 2168940u32);
    emu.sltru_no_count(13usize, 13usize, 14usize, 2168944u32);
    emu.sbr_no_count(11usize, 11usize, 12usize, 2168948u32);
    emu.adr_no_count(11usize, 11usize, 13usize, 2168952u32);
    emu.sri_no_count(11usize, 11usize, 31u32, 2168956u32);
    emu.adr_no_count(10usize, 11usize, 10usize, 2168960u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2168964u32);
    emu.apc_no_count(6usize, 2168964u32, 24576u32, 2168968u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2168972u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966516u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
