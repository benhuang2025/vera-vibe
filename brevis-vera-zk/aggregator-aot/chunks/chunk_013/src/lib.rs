pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2149160u32;
pub const PC_MAX: u32 = 2157856u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 107usize] = [
        block_0x0020cb28,
        block_0x0020cb38,
        block_0x0020cb68,
        block_0x0020cc00,
        block_0x0020cc18,
        block_0x0020cc28,
        block_0x0020cc34,
        block_0x0020cec8,
        block_0x0020ced8,
        block_0x0020cfb0,
        block_0x0020d04c,
        block_0x0020d078,
        block_0x0020d08c,
        block_0x0020d0a0,
        block_0x0020d0b4,
        block_0x0020d108,
        block_0x0020d15c,
        block_0x0020d1b0,
        block_0x0020d21c,
        block_0x0020d270,
        block_0x0020d2c4,
        block_0x0020d318,
        block_0x0020d36c,
        block_0x0020d3c0,
        block_0x0020d414,
        block_0x0020d480,
        block_0x0020d494,
        block_0x0020d4e8,
        block_0x0020d53c,
        block_0x0020d5a8,
        block_0x0020d5bc,
        block_0x0020d5d0,
        block_0x0020d624,
        block_0x0020d678,
        block_0x0020d6cc,
        block_0x0020d720,
        block_0x0020d774,
        block_0x0020d7c8,
        block_0x0020d81c,
        block_0x0020d870,
        block_0x0020d8c4,
        block_0x0020d918,
        block_0x0020d96c,
        block_0x0020d9c0,
        block_0x0020da14,
        block_0x0020da68,
        block_0x0020dabc,
        block_0x0020db10,
        block_0x0020db7c,
        block_0x0020db90,
        block_0x0020dbe4,
        block_0x0020dc38,
        block_0x0020dc8c,
        block_0x0020dce0,
        block_0x0020dd34,
        block_0x0020dd88,
        block_0x0020dddc,
        block_0x0020de30,
        block_0x0020de84,
        block_0x0020ded8,
        block_0x0020df2c,
        block_0x0020df80,
        block_0x0020dfd4,
        block_0x0020e028,
        block_0x0020e094,
        block_0x0020e0e8,
        block_0x0020e13c,
        block_0x0020e190,
        block_0x0020e1e4,
        block_0x0020e238,
        block_0x0020e28c,
        block_0x0020e2e0,
        block_0x0020e334,
        block_0x0020e388,
        block_0x0020e3dc,
        block_0x0020e430,
        block_0x0020e484,
        block_0x0020e4d8,
        block_0x0020e52c,
        block_0x0020e580,
        block_0x0020e5d4,
        block_0x0020e628,
        block_0x0020e694,
        block_0x0020e698,
        block_0x0020e6ac,
        block_0x0020e6f4,
        block_0x0020e748,
        block_0x0020e74c,
        block_0x0020e760,
        block_0x0020e7a8,
        block_0x0020e7fc,
        block_0x0020e810,
        block_0x0020e864,
        block_0x0020e8d0,
        block_0x0020e8e8,
        block_0x0020e914,
        block_0x0020e928,
        block_0x0020e9bc,
        block_0x0020ea10,
        block_0x0020ea7c,
        block_0x0020eb10,
        block_0x0020eb64,
        block_0x0020ebb8,
        block_0x0020ec0c,
        block_0x0020ec78,
        block_0x0020eccc,
        block_0x0020ed20,
    ];
    #[repr(C)]
    struct Run {
        start_word: u32,
        len: u16,
        fn_offset: u16,
    }
    const RUNS: [Run; 105usize] = [
        Run {
            start_word: 0u32,
            len: 1i32 as u16,
            fn_offset: 0usize as u16,
        },
        Run {
            start_word: 4u32,
            len: 1i32 as u16,
            fn_offset: 1usize as u16,
        },
        Run {
            start_word: 16u32,
            len: 1i32 as u16,
            fn_offset: 2usize as u16,
        },
        Run {
            start_word: 54u32,
            len: 1i32 as u16,
            fn_offset: 3usize as u16,
        },
        Run {
            start_word: 60u32,
            len: 1i32 as u16,
            fn_offset: 4usize as u16,
        },
        Run {
            start_word: 64u32,
            len: 1i32 as u16,
            fn_offset: 5usize as u16,
        },
        Run {
            start_word: 67u32,
            len: 1i32 as u16,
            fn_offset: 6usize as u16,
        },
        Run {
            start_word: 232u32,
            len: 1i32 as u16,
            fn_offset: 7usize as u16,
        },
        Run {
            start_word: 236u32,
            len: 1i32 as u16,
            fn_offset: 8usize as u16,
        },
        Run {
            start_word: 290u32,
            len: 1i32 as u16,
            fn_offset: 9usize as u16,
        },
        Run {
            start_word: 329u32,
            len: 1i32 as u16,
            fn_offset: 10usize as u16,
        },
        Run {
            start_word: 340u32,
            len: 1i32 as u16,
            fn_offset: 11usize as u16,
        },
        Run {
            start_word: 345u32,
            len: 1i32 as u16,
            fn_offset: 12usize as u16,
        },
        Run {
            start_word: 350u32,
            len: 1i32 as u16,
            fn_offset: 13usize as u16,
        },
        Run {
            start_word: 355u32,
            len: 1i32 as u16,
            fn_offset: 14usize as u16,
        },
        Run {
            start_word: 376u32,
            len: 1i32 as u16,
            fn_offset: 15usize as u16,
        },
        Run {
            start_word: 397u32,
            len: 1i32 as u16,
            fn_offset: 16usize as u16,
        },
        Run {
            start_word: 418u32,
            len: 1i32 as u16,
            fn_offset: 17usize as u16,
        },
        Run {
            start_word: 445u32,
            len: 1i32 as u16,
            fn_offset: 18usize as u16,
        },
        Run {
            start_word: 466u32,
            len: 1i32 as u16,
            fn_offset: 19usize as u16,
        },
        Run {
            start_word: 487u32,
            len: 1i32 as u16,
            fn_offset: 20usize as u16,
        },
        Run {
            start_word: 508u32,
            len: 1i32 as u16,
            fn_offset: 21usize as u16,
        },
        Run {
            start_word: 529u32,
            len: 1i32 as u16,
            fn_offset: 22usize as u16,
        },
        Run {
            start_word: 550u32,
            len: 1i32 as u16,
            fn_offset: 23usize as u16,
        },
        Run {
            start_word: 571u32,
            len: 1i32 as u16,
            fn_offset: 24usize as u16,
        },
        Run {
            start_word: 598u32,
            len: 1i32 as u16,
            fn_offset: 25usize as u16,
        },
        Run {
            start_word: 603u32,
            len: 1i32 as u16,
            fn_offset: 26usize as u16,
        },
        Run {
            start_word: 624u32,
            len: 1i32 as u16,
            fn_offset: 27usize as u16,
        },
        Run {
            start_word: 645u32,
            len: 1i32 as u16,
            fn_offset: 28usize as u16,
        },
        Run {
            start_word: 672u32,
            len: 1i32 as u16,
            fn_offset: 29usize as u16,
        },
        Run {
            start_word: 677u32,
            len: 1i32 as u16,
            fn_offset: 30usize as u16,
        },
        Run {
            start_word: 682u32,
            len: 1i32 as u16,
            fn_offset: 31usize as u16,
        },
        Run {
            start_word: 703u32,
            len: 1i32 as u16,
            fn_offset: 32usize as u16,
        },
        Run {
            start_word: 724u32,
            len: 1i32 as u16,
            fn_offset: 33usize as u16,
        },
        Run {
            start_word: 745u32,
            len: 1i32 as u16,
            fn_offset: 34usize as u16,
        },
        Run {
            start_word: 766u32,
            len: 1i32 as u16,
            fn_offset: 35usize as u16,
        },
        Run {
            start_word: 787u32,
            len: 1i32 as u16,
            fn_offset: 36usize as u16,
        },
        Run {
            start_word: 808u32,
            len: 1i32 as u16,
            fn_offset: 37usize as u16,
        },
        Run {
            start_word: 829u32,
            len: 1i32 as u16,
            fn_offset: 38usize as u16,
        },
        Run {
            start_word: 850u32,
            len: 1i32 as u16,
            fn_offset: 39usize as u16,
        },
        Run {
            start_word: 871u32,
            len: 1i32 as u16,
            fn_offset: 40usize as u16,
        },
        Run {
            start_word: 892u32,
            len: 1i32 as u16,
            fn_offset: 41usize as u16,
        },
        Run {
            start_word: 913u32,
            len: 1i32 as u16,
            fn_offset: 42usize as u16,
        },
        Run {
            start_word: 934u32,
            len: 1i32 as u16,
            fn_offset: 43usize as u16,
        },
        Run {
            start_word: 955u32,
            len: 1i32 as u16,
            fn_offset: 44usize as u16,
        },
        Run {
            start_word: 976u32,
            len: 1i32 as u16,
            fn_offset: 45usize as u16,
        },
        Run {
            start_word: 997u32,
            len: 1i32 as u16,
            fn_offset: 46usize as u16,
        },
        Run {
            start_word: 1018u32,
            len: 1i32 as u16,
            fn_offset: 47usize as u16,
        },
        Run {
            start_word: 1045u32,
            len: 1i32 as u16,
            fn_offset: 48usize as u16,
        },
        Run {
            start_word: 1050u32,
            len: 1i32 as u16,
            fn_offset: 49usize as u16,
        },
        Run {
            start_word: 1071u32,
            len: 1i32 as u16,
            fn_offset: 50usize as u16,
        },
        Run {
            start_word: 1092u32,
            len: 1i32 as u16,
            fn_offset: 51usize as u16,
        },
        Run {
            start_word: 1113u32,
            len: 1i32 as u16,
            fn_offset: 52usize as u16,
        },
        Run {
            start_word: 1134u32,
            len: 1i32 as u16,
            fn_offset: 53usize as u16,
        },
        Run {
            start_word: 1155u32,
            len: 1i32 as u16,
            fn_offset: 54usize as u16,
        },
        Run {
            start_word: 1176u32,
            len: 1i32 as u16,
            fn_offset: 55usize as u16,
        },
        Run {
            start_word: 1197u32,
            len: 1i32 as u16,
            fn_offset: 56usize as u16,
        },
        Run {
            start_word: 1218u32,
            len: 1i32 as u16,
            fn_offset: 57usize as u16,
        },
        Run {
            start_word: 1239u32,
            len: 1i32 as u16,
            fn_offset: 58usize as u16,
        },
        Run {
            start_word: 1260u32,
            len: 1i32 as u16,
            fn_offset: 59usize as u16,
        },
        Run {
            start_word: 1281u32,
            len: 1i32 as u16,
            fn_offset: 60usize as u16,
        },
        Run {
            start_word: 1302u32,
            len: 1i32 as u16,
            fn_offset: 61usize as u16,
        },
        Run {
            start_word: 1323u32,
            len: 1i32 as u16,
            fn_offset: 62usize as u16,
        },
        Run {
            start_word: 1344u32,
            len: 1i32 as u16,
            fn_offset: 63usize as u16,
        },
        Run {
            start_word: 1371u32,
            len: 1i32 as u16,
            fn_offset: 64usize as u16,
        },
        Run {
            start_word: 1392u32,
            len: 1i32 as u16,
            fn_offset: 65usize as u16,
        },
        Run {
            start_word: 1413u32,
            len: 1i32 as u16,
            fn_offset: 66usize as u16,
        },
        Run {
            start_word: 1434u32,
            len: 1i32 as u16,
            fn_offset: 67usize as u16,
        },
        Run {
            start_word: 1455u32,
            len: 1i32 as u16,
            fn_offset: 68usize as u16,
        },
        Run {
            start_word: 1476u32,
            len: 1i32 as u16,
            fn_offset: 69usize as u16,
        },
        Run {
            start_word: 1497u32,
            len: 1i32 as u16,
            fn_offset: 70usize as u16,
        },
        Run {
            start_word: 1518u32,
            len: 1i32 as u16,
            fn_offset: 71usize as u16,
        },
        Run {
            start_word: 1539u32,
            len: 1i32 as u16,
            fn_offset: 72usize as u16,
        },
        Run {
            start_word: 1560u32,
            len: 1i32 as u16,
            fn_offset: 73usize as u16,
        },
        Run {
            start_word: 1581u32,
            len: 1i32 as u16,
            fn_offset: 74usize as u16,
        },
        Run {
            start_word: 1602u32,
            len: 1i32 as u16,
            fn_offset: 75usize as u16,
        },
        Run {
            start_word: 1623u32,
            len: 1i32 as u16,
            fn_offset: 76usize as u16,
        },
        Run {
            start_word: 1644u32,
            len: 1i32 as u16,
            fn_offset: 77usize as u16,
        },
        Run {
            start_word: 1665u32,
            len: 1i32 as u16,
            fn_offset: 78usize as u16,
        },
        Run {
            start_word: 1686u32,
            len: 1i32 as u16,
            fn_offset: 79usize as u16,
        },
        Run {
            start_word: 1707u32,
            len: 1i32 as u16,
            fn_offset: 80usize as u16,
        },
        Run {
            start_word: 1728u32,
            len: 1i32 as u16,
            fn_offset: 81usize as u16,
        },
        Run {
            start_word: 1755u32,
            len: 2i32 as u16,
            fn_offset: 82usize as u16,
        },
        Run {
            start_word: 1761u32,
            len: 1i32 as u16,
            fn_offset: 84usize as u16,
        },
        Run {
            start_word: 1779u32,
            len: 1i32 as u16,
            fn_offset: 85usize as u16,
        },
        Run {
            start_word: 1800u32,
            len: 2i32 as u16,
            fn_offset: 86usize as u16,
        },
        Run {
            start_word: 1806u32,
            len: 1i32 as u16,
            fn_offset: 88usize as u16,
        },
        Run {
            start_word: 1824u32,
            len: 1i32 as u16,
            fn_offset: 89usize as u16,
        },
        Run {
            start_word: 1845u32,
            len: 1i32 as u16,
            fn_offset: 90usize as u16,
        },
        Run {
            start_word: 1850u32,
            len: 1i32 as u16,
            fn_offset: 91usize as u16,
        },
        Run {
            start_word: 1871u32,
            len: 1i32 as u16,
            fn_offset: 92usize as u16,
        },
        Run {
            start_word: 1898u32,
            len: 1i32 as u16,
            fn_offset: 93usize as u16,
        },
        Run {
            start_word: 1904u32,
            len: 1i32 as u16,
            fn_offset: 94usize as u16,
        },
        Run {
            start_word: 1915u32,
            len: 1i32 as u16,
            fn_offset: 95usize as u16,
        },
        Run {
            start_word: 1920u32,
            len: 1i32 as u16,
            fn_offset: 96usize as u16,
        },
        Run {
            start_word: 1957u32,
            len: 1i32 as u16,
            fn_offset: 97usize as u16,
        },
        Run {
            start_word: 1978u32,
            len: 1i32 as u16,
            fn_offset: 98usize as u16,
        },
        Run {
            start_word: 2005u32,
            len: 1i32 as u16,
            fn_offset: 99usize as u16,
        },
        Run {
            start_word: 2042u32,
            len: 1i32 as u16,
            fn_offset: 100usize as u16,
        },
        Run {
            start_word: 2063u32,
            len: 1i32 as u16,
            fn_offset: 101usize as u16,
        },
        Run {
            start_word: 2084u32,
            len: 1i32 as u16,
            fn_offset: 102usize as u16,
        },
        Run {
            start_word: 2105u32,
            len: 1i32 as u16,
            fn_offset: 103usize as u16,
        },
        Run {
            start_word: 2132u32,
            len: 1i32 as u16,
            fn_offset: 104usize as u16,
        },
        Run {
            start_word: 2153u32,
            len: 1i32 as u16,
            fn_offset: 105usize as u16,
        },
        Run {
            start_word: 2174u32,
            len: 1i32 as u16,
            fn_offset: 106usize as u16,
        },
    ];
    if pc < 2149160u32 || pc > 2157856u32 {
        return None;
    }
    let word_offset = ((pc - 2149160u32) >> 2) as u32;
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
#[inline(always)]
pub fn block_0x0020cb28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(10usize, 10usize, 4294967295u32, 2149164u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2149168u32);
    emu.apc_no_count(1usize, 2149168u32, 40960u32, 2149172u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2149176u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965492u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020cb38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(10usize, 8usize, 32u32, 2149180u32);
    emu.lw_no_count(1usize, 2usize, 204u32, 2149184u32)?;
    emu.lw_no_count(8usize, 2usize, 200u32, 2149188u32)?;
    emu.lw_no_count(9usize, 2usize, 196u32, 2149192u32)?;
    emu.lw_no_count(18usize, 2usize, 192u32, 2149196u32)?;
    emu.lw_no_count(19usize, 2usize, 188u32, 2149200u32)?;
    emu.lw_no_count(20usize, 2usize, 184u32, 2149204u32)?;
    emu.lw_no_count(21usize, 2usize, 180u32, 2149208u32)?;
    emu.lw_no_count(22usize, 2usize, 176u32, 2149212u32)?;
    emu.lw_no_count(23usize, 2usize, 172u32, 2149216u32)?;
    emu.adi_no_count(2usize, 2usize, 208u32, 2149220u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2149224u32;
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
pub fn block_0x0020cb68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 38u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2149228u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2149232u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2149236u32)?;
    emu.lw_no_count(12usize, 10usize, 0u32, 2149240u32)?;
    emu.lw_no_count(13usize, 10usize, 4u32, 2149244u32)?;
    emu.lw_no_count(14usize, 10usize, 8u32, 2149248u32)?;
    emu.lw_no_count(15usize, 10usize, 12u32, 2149252u32)?;
    emu.lw_no_count(16usize, 11usize, 0u32, 2149256u32)?;
    emu.lw_no_count(17usize, 11usize, 4u32, 2149260u32)?;
    emu.lw_no_count(5usize, 11usize, 8u32, 2149264u32)?;
    emu.lw_no_count(6usize, 11usize, 12u32, 2149268u32)?;
    emu.lw_no_count(7usize, 10usize, 16u32, 2149272u32)?;
    emu.lw_no_count(28usize, 10usize, 20u32, 2149276u32)?;
    emu.lw_no_count(29usize, 10usize, 24u32, 2149280u32)?;
    emu.lw_no_count(10usize, 10usize, 28u32, 2149284u32)?;
    emu.lw_no_count(30usize, 11usize, 16u32, 2149288u32)?;
    emu.lw_no_count(31usize, 11usize, 20u32, 2149292u32)?;
    emu.lw_no_count(8usize, 11usize, 24u32, 2149296u32)?;
    emu.lw_no_count(11usize, 11usize, 28u32, 2149300u32)?;
    emu.xrr_no_count(13usize, 17usize, 13usize, 2149304u32);
    emu.xrr_no_count(12usize, 16usize, 12usize, 2149308u32);
    emu.xrr_no_count(14usize, 5usize, 14usize, 2149312u32);
    emu.xrr_no_count(15usize, 6usize, 15usize, 2149316u32);
    emu.xrr_no_count(16usize, 30usize, 7usize, 2149320u32);
    emu.xrr_no_count(17usize, 31usize, 28usize, 2149324u32);
    emu.xrr_no_count(5usize, 8usize, 29usize, 2149328u32);
    emu.orr_no_count(12usize, 12usize, 13usize, 2149332u32);
    emu.orr_no_count(14usize, 14usize, 15usize, 2149336u32);
    emu.orr_no_count(13usize, 16usize, 17usize, 2149340u32);
    emu.orr_no_count(12usize, 12usize, 14usize, 2149344u32);
    emu.orr_no_count(13usize, 13usize, 5usize, 2149348u32);
    emu.orr_no_count(12usize, 12usize, 13usize, 2149352u32);
    emu.xrr_no_count(10usize, 11usize, 10usize, 2149356u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2149360u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2149364u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2149368u32);
    emu.apc_no_count(1usize, 2149368u32, 40960u32, 2149372u32);
    emu.add_memory_rw_events(38usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2149376u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965312u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020cc00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 255u32, 2149380u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2149384u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2149388u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2149392u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2149396u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2149400u32;
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
#[inline(always)]
pub fn block_0x0020cc18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2149404u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2149408u32)?;
    emu.apc_no_count(1usize, 2149408u32, 4294963200u32, 2149412u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2149416u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965676u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020cc28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2149420u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2149424u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2149428u32;
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
pub fn block_0x0020cc34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 165u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967216u32, 2149432u32);
    emu.sw_no_count(1usize, 2usize, 76u32, 2149436u32)?;
    emu.sw_no_count(8usize, 2usize, 72u32, 2149440u32)?;
    emu.sw_no_count(9usize, 2usize, 68u32, 2149444u32)?;
    emu.sw_no_count(18usize, 2usize, 64u32, 2149448u32)?;
    emu.sw_no_count(19usize, 2usize, 60u32, 2149452u32)?;
    emu.sw_no_count(20usize, 2usize, 56u32, 2149456u32)?;
    emu.sw_no_count(21usize, 2usize, 52u32, 2149460u32)?;
    emu.sw_no_count(22usize, 2usize, 48u32, 2149464u32)?;
    emu.sw_no_count(23usize, 2usize, 44u32, 2149468u32)?;
    emu.sw_no_count(24usize, 2usize, 40u32, 2149472u32)?;
    emu.sw_no_count(25usize, 2usize, 36u32, 2149476u32)?;
    emu.sw_no_count(26usize, 2usize, 32u32, 2149480u32)?;
    emu.sw_no_count(27usize, 2usize, 28u32, 2149484u32)?;
    emu.sw_no_count(10usize, 2usize, 24u32, 2149488u32)?;
    emu.lbu_no_count(10usize, 11usize, 0u32, 2149492u32);
    emu.lbu_no_count(12usize, 11usize, 1u32, 2149496u32);
    emu.lbu_no_count(13usize, 11usize, 2u32, 2149500u32);
    emu.lbu_no_count(14usize, 11usize, 3u32, 2149504u32);
    emu.lbu_no_count(15usize, 11usize, 4u32, 2149508u32);
    emu.lbu_no_count(16usize, 11usize, 5u32, 2149512u32);
    emu.lbu_no_count(17usize, 11usize, 6u32, 2149516u32);
    emu.lbu_no_count(5usize, 11usize, 7u32, 2149520u32);
    emu.lbu_no_count(6usize, 11usize, 8u32, 2149524u32);
    emu.lbu_no_count(7usize, 11usize, 9u32, 2149528u32);
    emu.lbu_no_count(28usize, 11usize, 10u32, 2149532u32);
    emu.lbu_no_count(29usize, 11usize, 11u32, 2149536u32);
    emu.lbu_no_count(30usize, 11usize, 12u32, 2149540u32);
    emu.lbu_no_count(31usize, 11usize, 13u32, 2149544u32);
    emu.lbu_no_count(18usize, 11usize, 14u32, 2149548u32);
    emu.lbu_no_count(19usize, 11usize, 15u32, 2149552u32);
    emu.sli_no_count(13usize, 13usize, 8u32, 2149556u32);
    emu.sli_no_count(12usize, 12usize, 16u32, 2149560u32);
    emu.sli_no_count(10usize, 10usize, 24u32, 2149564u32);
    emu.orr_no_count(13usize, 13usize, 14usize, 2149568u32);
    emu.orr_no_count(9usize, 10usize, 12usize, 2149572u32);
    emu.lbu_no_count(20usize, 11usize, 16u32, 2149576u32);
    emu.lbu_no_count(21usize, 11usize, 17u32, 2149580u32);
    emu.lbu_no_count(22usize, 11usize, 18u32, 2149584u32);
    emu.lbu_no_count(23usize, 11usize, 19u32, 2149588u32);
    emu.sli_no_count(17usize, 17usize, 8u32, 2149592u32);
    emu.sli_no_count(16usize, 16usize, 16u32, 2149596u32);
    emu.sli_no_count(15usize, 15usize, 24u32, 2149600u32);
    emu.sli_no_count(28usize, 28usize, 8u32, 2149604u32);
    emu.sli_no_count(7usize, 7usize, 16u32, 2149608u32);
    emu.sli_no_count(6usize, 6usize, 24u32, 2149612u32);
    emu.orr_no_count(14usize, 17usize, 5usize, 2149616u32);
    emu.orr_no_count(15usize, 15usize, 16usize, 2149620u32);
    emu.orr_no_count(10usize, 28usize, 29usize, 2149624u32);
    emu.orr_no_count(12usize, 6usize, 7usize, 2149628u32);
    emu.lbu_no_count(7usize, 11usize, 20u32, 2149632u32);
    emu.lbu_no_count(28usize, 11usize, 21u32, 2149636u32);
    emu.lbu_no_count(29usize, 11usize, 22u32, 2149640u32);
    emu.lbu_no_count(24usize, 11usize, 23u32, 2149644u32);
    emu.sli_no_count(18usize, 18usize, 8u32, 2149648u32);
    emu.sli_no_count(31usize, 31usize, 16u32, 2149652u32);
    emu.sli_no_count(30usize, 30usize, 24u32, 2149656u32);
    emu.sli_no_count(22usize, 22usize, 8u32, 2149660u32);
    emu.sli_no_count(21usize, 21usize, 16u32, 2149664u32);
    emu.sli_no_count(20usize, 20usize, 24u32, 2149668u32);
    emu.orr_no_count(16usize, 18usize, 19usize, 2149672u32);
    emu.orr_no_count(17usize, 30usize, 31usize, 2149676u32);
    emu.orr_no_count(5usize, 22usize, 23usize, 2149680u32);
    emu.orr_no_count(6usize, 20usize, 21usize, 2149684u32);
    emu.lbu_no_count(30usize, 11usize, 24u32, 2149688u32);
    emu.lbu_no_count(31usize, 11usize, 25u32, 2149692u32);
    emu.lbu_no_count(18usize, 11usize, 26u32, 2149696u32);
    emu.lbu_no_count(19usize, 11usize, 27u32, 2149700u32);
    emu.sli_no_count(29usize, 29usize, 8u32, 2149704u32);
    emu.sli_no_count(28usize, 28usize, 16u32, 2149708u32);
    emu.sli_no_count(20usize, 7usize, 24u32, 2149712u32);
    emu.sli_no_count(18usize, 18usize, 8u32, 2149716u32);
    emu.orr_no_count(7usize, 29usize, 24usize, 2149720u32);
    emu.orr_no_count(28usize, 20usize, 28usize, 2149724u32);
    emu.orr_no_count(29usize, 18usize, 19usize, 2149728u32);
    emu.lbu_no_count(18usize, 11usize, 28u32, 2149732u32);
    emu.lbu_no_count(19usize, 11usize, 29u32, 2149736u32);
    emu.lbu_no_count(20usize, 11usize, 30u32, 2149740u32);
    emu.lbu_no_count(11usize, 11usize, 31u32, 2149744u32);
    emu.sli_no_count(31usize, 31usize, 16u32, 2149748u32);
    emu.sli_no_count(30usize, 30usize, 24u32, 2149752u32);
    emu.orr_no_count(30usize, 30usize, 31usize, 2149756u32);
    emu.sli_no_count(20usize, 20usize, 8u32, 2149760u32);
    emu.orr_no_count(11usize, 20usize, 11usize, 2149764u32);
    let a = 0u32.wrapping_add(60612608u32);
    emu.write_reg_no_count(31usize, a);
    emu.pc = 2149768u32;
    emu.update_insn_clock();
    emu.sli_no_count(19usize, 19usize, 16u32, 2149772u32);
    emu.sli_no_count(18usize, 18usize, 24u32, 2149776u32);
    emu.orr_no_count(18usize, 18usize, 19usize, 2149780u32);
    let a = 0u32.wrapping_add(205926400u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2149784u32;
    emu.update_insn_clock();
    emu.orr_no_count(1usize, 9usize, 13usize, 2149788u32);
    let a = 0u32.wrapping_add(1491623936u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2149792u32;
    emu.update_insn_clock();
    emu.orr_no_count(15usize, 15usize, 14usize, 2149796u32);
    emu.sw_no_count(15usize, 2usize, 8u32, 2149800u32)?;
    let a = 0u32.wrapping_add(1125711872u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2149804u32;
    emu.update_insn_clock();
    emu.adi_no_count(25usize, 31usize, 4294965935u32, 2149808u32);
    emu.adi_no_count(26usize, 19usize, 1342u32, 2149812u32);
    emu.adi_no_count(27usize, 13usize, 380u32, 2149816u32);
    emu.adi_no_count(13usize, 14usize, 1363u32, 2149820u32);
    emu.orr_no_count(19usize, 12usize, 10usize, 2149824u32);
    emu.orr_no_count(20usize, 17usize, 16usize, 2149828u32);
    emu.orr_no_count(21usize, 6usize, 5usize, 2149832u32);
    emu.orr_no_count(22usize, 28usize, 7usize, 2149836u32);
    emu.orr_no_count(23usize, 30usize, 29usize, 2149840u32);
    emu.orr_no_count(24usize, 18usize, 11usize, 2149844u32);
    emu.adr_no_count(25usize, 24usize, 25usize, 2149848u32);
    emu.sw_no_count(25usize, 2usize, 20u32, 2149852u32)?;
    emu.sltru_no_count(10usize, 25usize, 24usize, 2149856u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2149860u32);
    emu.adr_no_count(11usize, 23usize, 10usize, 2149864u32);
    emu.sltru_no_count(12usize, 11usize, 23usize, 2149868u32);
    emu.adr_no_count(26usize, 11usize, 26usize, 2149872u32);
    emu.sw_no_count(26usize, 2usize, 16u32, 2149876u32)?;
    emu.adr_no_count(10usize, 10usize, 12usize, 2149880u32);
    emu.sltru_no_count(11usize, 26usize, 11usize, 2149884u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2149888u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2149892u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2149896u32);
    emu.adr_no_count(11usize, 22usize, 10usize, 2149900u32);
    emu.sltru_no_count(12usize, 11usize, 22usize, 2149904u32);
    emu.adr_no_count(27usize, 11usize, 27usize, 2149908u32);
    emu.sw_no_count(27usize, 2usize, 12u32, 2149912u32)?;
    emu.adr_no_count(10usize, 10usize, 12usize, 2149916u32);
    emu.sltru_no_count(11usize, 27usize, 11usize, 2149920u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2149924u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2149928u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2149932u32);
    emu.adr_no_count(11usize, 21usize, 10usize, 2149936u32);
    emu.sltru_no_count(12usize, 11usize, 21usize, 2149940u32);
    emu.adr_no_count(18usize, 11usize, 13usize, 2149944u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2149948u32);
    emu.sltru_no_count(11usize, 18usize, 11usize, 2149952u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2149956u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2149960u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2149964u32);
    emu.adr_no_count(11usize, 20usize, 10usize, 2149968u32);
    emu.sltru_no_count(12usize, 11usize, 20usize, 2149972u32);
    emu.adi_no_count(8usize, 11usize, 1u32, 2149976u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2149980u32);
    emu.sltiu_no_count(11usize, 8usize, 1u32, 2149984u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2149988u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2149992u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2149996u32);
    emu.adr_no_count(11usize, 19usize, 10usize, 2150000u32);
    emu.sltru_no_count(12usize, 11usize, 19usize, 2150004u32);
    emu.adi_no_count(9usize, 11usize, 1u32, 2150008u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2150012u32);
    emu.sltiu_no_count(11usize, 9usize, 1u32, 2150016u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2150020u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2150024u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2150028u32);
    emu.adr_no_count(25usize, 10usize, 15usize, 2150032u32);
    emu.sltru_no_count(11usize, 25usize, 10usize, 2150036u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2150040u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2150044u32);
    emu.adi_no_count(27usize, 1usize, 0u32, 2150048u32);
    emu.adr_no_count(11usize, 1usize, 10usize, 2150052u32);
    emu.sltru_no_count(12usize, 11usize, 1usize, 2150056u32);
    emu.adi_no_count(26usize, 11usize, 1u32, 2150060u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2150064u32);
    emu.sltiu_no_count(11usize, 26usize, 1u32, 2150068u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2150072u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2150076u32);
    emu.sri_no_count(10usize, 10usize, 31u32, 2150080u32);
    emu.apc_no_count(1usize, 2150080u32, 36864u32, 2150084u32);
    emu.add_memory_rw_events(165usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2150088u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1380u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020cec8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(10usize, 10usize, 4294967295u32, 2150092u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2150096u32);
    emu.apc_no_count(1usize, 2150096u32, 36864u32, 2150100u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2150104u32;
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
#[inline(never)]
pub fn block_0x0020ced8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 54u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 255u32, 2150108u32);
    emu.lw_no_count(11usize, 2usize, 20u32, 2150112u32)?;
    emu.xrr_no_count(11usize, 24usize, 11usize, 2150116u32);
    emu.lw_no_count(12usize, 2usize, 16u32, 2150120u32)?;
    emu.xrr_no_count(12usize, 23usize, 12usize, 2150124u32);
    emu.lw_no_count(13usize, 2usize, 12u32, 2150128u32)?;
    emu.xrr_no_count(13usize, 22usize, 13usize, 2150132u32);
    emu.xrr_no_count(14usize, 21usize, 18usize, 2150136u32);
    emu.xrr_no_count(15usize, 20usize, 8usize, 2150140u32);
    emu.xrr_no_count(16usize, 19usize, 9usize, 2150144u32);
    emu.lw_no_count(6usize, 2usize, 8u32, 2150148u32)?;
    emu.xrr_no_count(17usize, 6usize, 25usize, 2150152u32);
    emu.xrr_no_count(5usize, 27usize, 26usize, 2150156u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2150160u32);
    emu.anr_no_count(11usize, 11usize, 10usize, 2150164u32);
    emu.anr_no_count(12usize, 12usize, 10usize, 2150168u32);
    emu.anr_no_count(13usize, 13usize, 10usize, 2150172u32);
    emu.anr_no_count(14usize, 14usize, 10usize, 2150176u32);
    emu.anr_no_count(15usize, 15usize, 10usize, 2150180u32);
    emu.anr_no_count(16usize, 16usize, 10usize, 2150184u32);
    emu.anr_no_count(17usize, 17usize, 10usize, 2150188u32);
    emu.anr_no_count(10usize, 5usize, 10usize, 2150192u32);
    emu.xrr_no_count(11usize, 11usize, 24usize, 2150196u32);
    emu.xrr_no_count(12usize, 12usize, 23usize, 2150200u32);
    emu.xrr_no_count(13usize, 13usize, 22usize, 2150204u32);
    emu.xrr_no_count(14usize, 14usize, 21usize, 2150208u32);
    emu.xrr_no_count(15usize, 15usize, 20usize, 2150212u32);
    emu.xrr_no_count(16usize, 16usize, 19usize, 2150216u32);
    emu.xrr_no_count(17usize, 17usize, 6usize, 2150220u32);
    emu.xrr_no_count(10usize, 10usize, 27usize, 2150224u32);
    emu.lw_no_count(5usize, 2usize, 24u32, 2150228u32)?;
    emu.sw_no_count(11usize, 5usize, 0u32, 2150232u32)?;
    emu.sw_no_count(12usize, 5usize, 4u32, 2150236u32)?;
    emu.sw_no_count(13usize, 5usize, 8u32, 2150240u32)?;
    emu.sw_no_count(14usize, 5usize, 12u32, 2150244u32)?;
    emu.sw_no_count(15usize, 5usize, 16u32, 2150248u32)?;
    emu.sw_no_count(16usize, 5usize, 20u32, 2150252u32)?;
    emu.sw_no_count(17usize, 5usize, 24u32, 2150256u32)?;
    emu.sw_no_count(10usize, 5usize, 28u32, 2150260u32)?;
    emu.lw_no_count(1usize, 2usize, 76u32, 2150264u32)?;
    emu.lw_no_count(8usize, 2usize, 72u32, 2150268u32)?;
    emu.lw_no_count(9usize, 2usize, 68u32, 2150272u32)?;
    emu.lw_no_count(18usize, 2usize, 64u32, 2150276u32)?;
    emu.lw_no_count(19usize, 2usize, 60u32, 2150280u32)?;
    emu.lw_no_count(20usize, 2usize, 56u32, 2150284u32)?;
    emu.lw_no_count(21usize, 2usize, 52u32, 2150288u32)?;
    emu.lw_no_count(22usize, 2usize, 48u32, 2150292u32)?;
    emu.lw_no_count(23usize, 2usize, 44u32, 2150296u32)?;
    emu.lw_no_count(24usize, 2usize, 40u32, 2150300u32)?;
    emu.lw_no_count(25usize, 2usize, 36u32, 2150304u32)?;
    emu.lw_no_count(26usize, 2usize, 32u32, 2150308u32)?;
    emu.lw_no_count(27usize, 2usize, 28u32, 2150312u32)?;
    emu.adi_no_count(2usize, 2usize, 80u32, 2150316u32);
    emu.add_memory_rw_events(54usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2150320u32;
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
pub fn block_0x0020cfb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 39u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2150324u32);
    emu.sw_no_count(8usize, 2usize, 12u32, 2150328u32)?;
    emu.lw_no_count(12usize, 10usize, 0u32, 2150332u32)?;
    emu.lw_no_count(13usize, 10usize, 4u32, 2150336u32)?;
    emu.lw_no_count(14usize, 10usize, 8u32, 2150340u32)?;
    emu.lw_no_count(15usize, 10usize, 12u32, 2150344u32)?;
    emu.lw_no_count(16usize, 11usize, 0u32, 2150348u32)?;
    emu.lw_no_count(17usize, 11usize, 4u32, 2150352u32)?;
    emu.lw_no_count(5usize, 11usize, 8u32, 2150356u32)?;
    emu.lw_no_count(6usize, 11usize, 12u32, 2150360u32)?;
    emu.lw_no_count(7usize, 10usize, 16u32, 2150364u32)?;
    emu.lw_no_count(28usize, 10usize, 20u32, 2150368u32)?;
    emu.lw_no_count(29usize, 10usize, 24u32, 2150372u32)?;
    emu.lw_no_count(10usize, 10usize, 28u32, 2150376u32)?;
    emu.lw_no_count(30usize, 11usize, 16u32, 2150380u32)?;
    emu.lw_no_count(31usize, 11usize, 20u32, 2150384u32)?;
    emu.lw_no_count(8usize, 11usize, 24u32, 2150388u32)?;
    emu.lw_no_count(11usize, 11usize, 28u32, 2150392u32)?;
    emu.xrr_no_count(13usize, 17usize, 13usize, 2150396u32);
    emu.xrr_no_count(12usize, 16usize, 12usize, 2150400u32);
    emu.xrr_no_count(14usize, 5usize, 14usize, 2150404u32);
    emu.xrr_no_count(15usize, 6usize, 15usize, 2150408u32);
    emu.xrr_no_count(16usize, 30usize, 7usize, 2150412u32);
    emu.xrr_no_count(17usize, 31usize, 28usize, 2150416u32);
    emu.xrr_no_count(5usize, 8usize, 29usize, 2150420u32);
    emu.orr_no_count(12usize, 12usize, 13usize, 2150424u32);
    emu.orr_no_count(14usize, 14usize, 15usize, 2150428u32);
    emu.orr_no_count(13usize, 16usize, 17usize, 2150432u32);
    emu.orr_no_count(12usize, 12usize, 14usize, 2150436u32);
    emu.orr_no_count(13usize, 13usize, 5usize, 2150440u32);
    emu.orr_no_count(12usize, 12usize, 13usize, 2150444u32);
    emu.xrr_no_count(10usize, 11usize, 10usize, 2150448u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2150452u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2150456u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2150460u32);
    emu.lw_no_count(8usize, 2usize, 12u32, 2150464u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2150468u32);
    emu.apc_no_count(6usize, 2150468u32, 36864u32, 2150472u32);
    emu.add_memory_rw_events(39usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2150476u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1012u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020d04c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294966928u32, 2150480u32);
    emu.sw_no_count(1usize, 2usize, 364u32, 2150484u32)?;
    emu.sw_no_count(8usize, 2usize, 360u32, 2150488u32)?;
    emu.sw_no_count(9usize, 2usize, 356u32, 2150492u32)?;
    emu.sw_no_count(18usize, 2usize, 352u32, 2150496u32)?;
    emu.adi_no_count(8usize, 11usize, 0u32, 2150500u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2150504u32);
    emu.adi_no_count(10usize, 2usize, 320u32, 2150508u32);
    emu.adi_no_count(12usize, 11usize, 0u32, 2150512u32);
    emu.apc_no_count(1usize, 2150512u32, 4294955008u32, 2150516u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2150520u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966212u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020d078(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 288u32, 2150524u32);
    emu.adi_no_count(12usize, 2usize, 320u32, 2150528u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2150532u32);
    emu.apc_no_count(1usize, 2150532u32, 4294955008u32, 2150536u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2150540u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966192u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020d08c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 256u32, 2150544u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2150548u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2150552u32);
    emu.apc_no_count(1usize, 2150552u32, 4294955008u32, 2150556u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2150560u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966172u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020d0a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2150564u32);
    emu.adi_no_count(12usize, 2usize, 256u32, 2150568u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2150572u32);
    emu.apc_no_count(1usize, 2150572u32, 4294955008u32, 2150576u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2150580u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966152u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020d0b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 16u32, 2150584u32)?;
    emu.lw_no_count(11usize, 2usize, 20u32, 2150588u32)?;
    emu.lw_no_count(12usize, 2usize, 24u32, 2150592u32)?;
    emu.lw_no_count(13usize, 2usize, 28u32, 2150596u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2150600u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2150604u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2150608u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2150612u32)?;
    emu.lw_no_count(10usize, 2usize, 0u32, 2150616u32)?;
    emu.lw_no_count(11usize, 2usize, 4u32, 2150620u32)?;
    emu.lw_no_count(12usize, 2usize, 8u32, 2150624u32)?;
    emu.lw_no_count(13usize, 2usize, 12u32, 2150628u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2150632u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2150636u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2150640u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2150644u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2150648u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2150652u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2150656u32);
    emu.apc_no_count(1usize, 2150656u32, 4294955008u32, 2150660u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2150664u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966068u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020d108(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2150668u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2150672u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2150676u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2150680u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2150684u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2150688u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2150692u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2150696u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2150700u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2150704u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2150708u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2150712u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2150716u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2150720u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2150724u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2150728u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2150732u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2150736u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2150740u32);
    emu.apc_no_count(1usize, 2150740u32, 4294955008u32, 2150744u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2150748u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965984u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020d15c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2150752u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2150756u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2150760u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2150764u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2150768u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2150772u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2150776u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2150780u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2150784u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2150788u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2150792u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2150796u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2150800u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2150804u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2150808u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2150812u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2150816u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2150820u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2150824u32);
    emu.apc_no_count(1usize, 2150824u32, 4294955008u32, 2150828u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2150832u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965900u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0020d1b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2150836u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2150840u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2150844u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2150848u32)?;
    emu.lw_no_count(14usize, 2usize, 320u32, 2150852u32)?;
    emu.lw_no_count(15usize, 2usize, 324u32, 2150856u32)?;
    emu.lw_no_count(16usize, 2usize, 328u32, 2150860u32)?;
    emu.lw_no_count(17usize, 2usize, 332u32, 2150864u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2150868u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2150872u32)?;
    emu.sw_no_count(14usize, 2usize, 288u32, 2150876u32)?;
    emu.sw_no_count(17usize, 2usize, 300u32, 2150880u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2150884u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2150888u32)?;
    emu.sw_no_count(14usize, 2usize, 320u32, 2150892u32)?;
    emu.sw_no_count(15usize, 2usize, 324u32, 2150896u32)?;
    emu.sw_no_count(16usize, 2usize, 328u32, 2150900u32)?;
    emu.sw_no_count(17usize, 2usize, 332u32, 2150904u32)?;
    emu.sw_no_count(10usize, 2usize, 336u32, 2150908u32)?;
    emu.sw_no_count(11usize, 2usize, 340u32, 2150912u32)?;
    emu.sw_no_count(12usize, 2usize, 344u32, 2150916u32)?;
    emu.sw_no_count(13usize, 2usize, 348u32, 2150920u32)?;
    emu.adi_no_count(10usize, 2usize, 32u32, 2150924u32);
    emu.adi_no_count(11usize, 2usize, 0u32, 2150928u32);
    emu.adi_no_count(12usize, 2usize, 320u32, 2150932u32);
    emu.apc_no_count(1usize, 2150932u32, 4294955008u32, 2150936u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2150940u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965792u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020d21c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 48u32, 2150944u32)?;
    emu.lw_no_count(11usize, 2usize, 52u32, 2150948u32)?;
    emu.lw_no_count(12usize, 2usize, 56u32, 2150952u32)?;
    emu.lw_no_count(13usize, 2usize, 60u32, 2150956u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2150960u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2150964u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2150968u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2150972u32)?;
    emu.lw_no_count(10usize, 2usize, 32u32, 2150976u32)?;
    emu.lw_no_count(11usize, 2usize, 36u32, 2150980u32)?;
    emu.lw_no_count(12usize, 2usize, 40u32, 2150984u32)?;
    emu.lw_no_count(13usize, 2usize, 44u32, 2150988u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2150992u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2150996u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2151000u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2151004u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2151008u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2151012u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2151016u32);
    emu.apc_no_count(1usize, 2151016u32, 4294955008u32, 2151020u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2151024u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965708u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020d270(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2151028u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2151032u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2151036u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2151040u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2151044u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2151048u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2151052u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2151056u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2151060u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2151064u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2151068u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2151072u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2151076u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2151080u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2151084u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2151088u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2151092u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2151096u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2151100u32);
    emu.apc_no_count(1usize, 2151100u32, 4294955008u32, 2151104u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2151108u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965624u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020d2c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2151112u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2151116u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2151120u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2151124u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2151128u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2151132u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2151136u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2151140u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2151144u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2151148u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2151152u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2151156u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2151160u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2151164u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2151168u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2151172u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2151176u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2151180u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2151184u32);
    emu.apc_no_count(1usize, 2151184u32, 4294955008u32, 2151188u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2151192u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965540u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020d318(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2151196u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2151200u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2151204u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2151208u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2151212u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2151216u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2151220u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2151224u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2151228u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2151232u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2151236u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2151240u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2151244u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2151248u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2151252u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2151256u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2151260u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2151264u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2151268u32);
    emu.apc_no_count(1usize, 2151268u32, 4294955008u32, 2151272u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2151276u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965456u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020d36c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2151280u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2151284u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2151288u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2151292u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2151296u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2151300u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2151304u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2151308u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2151312u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2151316u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2151320u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2151324u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2151328u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2151332u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2151336u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2151340u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2151344u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2151348u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2151352u32);
    emu.apc_no_count(1usize, 2151352u32, 4294955008u32, 2151356u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2151360u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965372u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020d3c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2151364u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2151368u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2151372u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2151376u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2151380u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2151384u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2151388u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2151392u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2151396u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2151400u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2151404u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2151408u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2151412u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2151416u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2151420u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2151424u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2151428u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2151432u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2151436u32);
    emu.apc_no_count(1usize, 2151436u32, 4294955008u32, 2151440u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2151444u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965288u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0020d414(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2151448u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2151452u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2151456u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2151460u32)?;
    emu.lw_no_count(14usize, 2usize, 320u32, 2151464u32)?;
    emu.lw_no_count(15usize, 2usize, 324u32, 2151468u32)?;
    emu.lw_no_count(16usize, 2usize, 328u32, 2151472u32)?;
    emu.lw_no_count(17usize, 2usize, 332u32, 2151476u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2151480u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2151484u32)?;
    emu.sw_no_count(14usize, 2usize, 288u32, 2151488u32)?;
    emu.sw_no_count(17usize, 2usize, 300u32, 2151492u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2151496u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2151500u32)?;
    emu.sw_no_count(14usize, 2usize, 256u32, 2151504u32)?;
    emu.sw_no_count(15usize, 2usize, 260u32, 2151508u32)?;
    emu.sw_no_count(16usize, 2usize, 264u32, 2151512u32)?;
    emu.sw_no_count(17usize, 2usize, 268u32, 2151516u32)?;
    emu.sw_no_count(10usize, 2usize, 272u32, 2151520u32)?;
    emu.sw_no_count(11usize, 2usize, 276u32, 2151524u32)?;
    emu.sw_no_count(12usize, 2usize, 280u32, 2151528u32)?;
    emu.sw_no_count(13usize, 2usize, 284u32, 2151532u32)?;
    emu.adi_no_count(10usize, 2usize, 288u32, 2151536u32);
    emu.adi_no_count(11usize, 2usize, 256u32, 2151540u32);
    emu.adi_no_count(12usize, 2usize, 32u32, 2151544u32);
    emu.apc_no_count(1usize, 2151544u32, 4294950912u32, 2151548u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2151552u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1980u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020d480(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 320u32, 2151556u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2151560u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2151564u32);
    emu.apc_no_count(1usize, 2151564u32, 4294950912u32, 2151568u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2151572u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1960u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020d494(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2151576u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2151580u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2151584u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2151588u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2151592u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2151596u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2151600u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2151604u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2151608u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2151612u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2151616u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2151620u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2151624u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2151628u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2151632u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2151636u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2151640u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2151644u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2151648u32);
    emu.apc_no_count(1usize, 2151648u32, 4294950912u32, 2151652u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2151656u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1876u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020d4e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2151660u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2151664u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2151668u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2151672u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2151676u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2151680u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2151684u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2151688u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2151692u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2151696u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2151700u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2151704u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2151708u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2151712u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2151716u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2151720u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2151724u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2151728u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2151732u32);
    emu.apc_no_count(1usize, 2151732u32, 4294950912u32, 2151736u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2151740u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1792u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0020d53c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2151744u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2151748u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2151752u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2151756u32)?;
    emu.lw_no_count(14usize, 2usize, 320u32, 2151760u32)?;
    emu.lw_no_count(15usize, 2usize, 324u32, 2151764u32)?;
    emu.lw_no_count(16usize, 2usize, 328u32, 2151768u32)?;
    emu.lw_no_count(17usize, 2usize, 332u32, 2151772u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2151776u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2151780u32)?;
    emu.sw_no_count(14usize, 2usize, 288u32, 2151784u32)?;
    emu.sw_no_count(17usize, 2usize, 300u32, 2151788u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2151792u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2151796u32)?;
    emu.sw_no_count(14usize, 2usize, 320u32, 2151800u32)?;
    emu.sw_no_count(15usize, 2usize, 324u32, 2151804u32)?;
    emu.sw_no_count(16usize, 2usize, 328u32, 2151808u32)?;
    emu.sw_no_count(17usize, 2usize, 332u32, 2151812u32)?;
    emu.sw_no_count(10usize, 2usize, 336u32, 2151816u32)?;
    emu.sw_no_count(11usize, 2usize, 340u32, 2151820u32)?;
    emu.sw_no_count(12usize, 2usize, 344u32, 2151824u32)?;
    emu.sw_no_count(13usize, 2usize, 348u32, 2151828u32)?;
    emu.adi_no_count(10usize, 2usize, 64u32, 2151832u32);
    emu.adi_no_count(11usize, 2usize, 320u32, 2151836u32);
    emu.adi_no_count(12usize, 2usize, 0u32, 2151840u32);
    emu.apc_no_count(1usize, 2151840u32, 4294950912u32, 2151844u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2151848u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1684u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020d5a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 320u32, 2151852u32);
    emu.adi_no_count(11usize, 2usize, 64u32, 2151856u32);
    emu.adi_no_count(12usize, 2usize, 64u32, 2151860u32);
    emu.apc_no_count(1usize, 2151860u32, 4294950912u32, 2151864u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2151868u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1664u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020d5bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 96u32, 2151872u32);
    emu.adi_no_count(11usize, 2usize, 320u32, 2151876u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2151880u32);
    emu.apc_no_count(1usize, 2151880u32, 4294950912u32, 2151884u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2151888u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1644u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020d5d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 112u32, 2151892u32)?;
    emu.lw_no_count(11usize, 2usize, 116u32, 2151896u32)?;
    emu.lw_no_count(12usize, 2usize, 120u32, 2151900u32)?;
    emu.lw_no_count(13usize, 2usize, 124u32, 2151904u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2151908u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2151912u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2151916u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2151920u32)?;
    emu.lw_no_count(10usize, 2usize, 96u32, 2151924u32)?;
    emu.lw_no_count(11usize, 2usize, 100u32, 2151928u32)?;
    emu.lw_no_count(12usize, 2usize, 104u32, 2151932u32)?;
    emu.lw_no_count(13usize, 2usize, 108u32, 2151936u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2151940u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2151944u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2151948u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2151952u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2151956u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2151960u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2151964u32);
    emu.apc_no_count(1usize, 2151964u32, 4294950912u32, 2151968u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2151972u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1560u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020d624(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2151976u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2151980u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2151984u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2151988u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2151992u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2151996u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2152000u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2152004u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2152008u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2152012u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2152016u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2152020u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2152024u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2152028u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2152032u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2152036u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2152040u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2152044u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2152048u32);
    emu.apc_no_count(1usize, 2152048u32, 4294950912u32, 2152052u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2152056u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1476u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020d678(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2152060u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2152064u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2152068u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2152072u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2152076u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2152080u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2152084u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2152088u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2152092u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2152096u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2152100u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2152104u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2152108u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2152112u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2152116u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2152120u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2152124u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2152128u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2152132u32);
    emu.apc_no_count(1usize, 2152132u32, 4294950912u32, 2152136u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2152140u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1392u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020d6cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2152144u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2152148u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2152152u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2152156u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2152160u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2152164u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2152168u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2152172u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2152176u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2152180u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2152184u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2152188u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2152192u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2152196u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2152200u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2152204u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2152208u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2152212u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2152216u32);
    emu.apc_no_count(1usize, 2152216u32, 4294950912u32, 2152220u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2152224u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1308u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020d720(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2152228u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2152232u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2152236u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2152240u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2152244u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2152248u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2152252u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2152256u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2152260u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2152264u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2152268u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2152272u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2152276u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2152280u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2152284u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2152288u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2152292u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2152296u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2152300u32);
    emu.apc_no_count(1usize, 2152300u32, 4294950912u32, 2152304u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2152308u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1224u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020d774(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2152312u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2152316u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2152320u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2152324u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2152328u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2152332u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2152336u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2152340u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2152344u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2152348u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2152352u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2152356u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2152360u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2152364u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2152368u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2152372u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2152376u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2152380u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2152384u32);
    emu.apc_no_count(1usize, 2152384u32, 4294950912u32, 2152388u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2152392u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1140u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020d7c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2152396u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2152400u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2152404u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2152408u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2152412u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2152416u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2152420u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2152424u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2152428u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2152432u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2152436u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2152440u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2152444u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2152448u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2152452u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2152456u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2152460u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2152464u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2152468u32);
    emu.apc_no_count(1usize, 2152468u32, 4294950912u32, 2152472u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2152476u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1056u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020d81c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2152480u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2152484u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2152488u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2152492u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2152496u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2152500u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2152504u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2152508u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2152512u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2152516u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2152520u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2152524u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2152528u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2152532u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2152536u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2152540u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2152544u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2152548u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2152552u32);
    emu.apc_no_count(1usize, 2152552u32, 4294950912u32, 2152556u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2152560u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(972u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020d870(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2152564u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2152568u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2152572u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2152576u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2152580u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2152584u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2152588u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2152592u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2152596u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2152600u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2152604u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2152608u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2152612u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2152616u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2152620u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2152624u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2152628u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2152632u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2152636u32);
    emu.apc_no_count(1usize, 2152636u32, 4294950912u32, 2152640u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2152644u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(888u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020d8c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2152648u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2152652u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2152656u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2152660u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2152664u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2152668u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2152672u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2152676u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2152680u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2152684u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2152688u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2152692u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2152696u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2152700u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2152704u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2152708u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2152712u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2152716u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2152720u32);
    emu.apc_no_count(1usize, 2152720u32, 4294950912u32, 2152724u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2152728u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(804u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020d918(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2152732u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2152736u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2152740u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2152744u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2152748u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2152752u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2152756u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2152760u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2152764u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2152768u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2152772u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2152776u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2152780u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2152784u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2152788u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2152792u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2152796u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2152800u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2152804u32);
    emu.apc_no_count(1usize, 2152804u32, 4294950912u32, 2152808u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2152812u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(720u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020d96c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2152816u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2152820u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2152824u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2152828u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2152832u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2152836u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2152840u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2152844u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2152848u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2152852u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2152856u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2152860u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2152864u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2152868u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2152872u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2152876u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2152880u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2152884u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2152888u32);
    emu.apc_no_count(1usize, 2152888u32, 4294950912u32, 2152892u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2152896u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(636u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020d9c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2152900u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2152904u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2152908u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2152912u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2152916u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2152920u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2152924u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2152928u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2152932u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2152936u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2152940u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2152944u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2152948u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2152952u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2152956u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2152960u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2152964u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2152968u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2152972u32);
    emu.apc_no_count(1usize, 2152972u32, 4294950912u32, 2152976u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2152980u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(552u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020da14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2152984u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2152988u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2152992u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2152996u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2153000u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2153004u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2153008u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2153012u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2153016u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2153020u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2153024u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2153028u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2153032u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2153036u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2153040u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2153044u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2153048u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2153052u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2153056u32);
    emu.apc_no_count(1usize, 2153056u32, 4294950912u32, 2153060u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2153064u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(468u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020da68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2153068u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2153072u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2153076u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2153080u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2153084u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2153088u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2153092u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2153096u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2153100u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2153104u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2153108u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2153112u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2153116u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2153120u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2153124u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2153128u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2153132u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2153136u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2153140u32);
    emu.apc_no_count(1usize, 2153140u32, 4294950912u32, 2153144u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2153148u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(384u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020dabc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2153152u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2153156u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2153160u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2153164u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2153168u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2153172u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2153176u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2153180u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2153184u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2153188u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2153192u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2153196u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2153200u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2153204u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2153208u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2153212u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2153216u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2153220u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2153224u32);
    emu.apc_no_count(1usize, 2153224u32, 4294950912u32, 2153228u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2153232u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(300u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0020db10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2153236u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2153240u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2153244u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2153248u32)?;
    emu.lw_no_count(14usize, 2usize, 320u32, 2153252u32)?;
    emu.lw_no_count(15usize, 2usize, 324u32, 2153256u32)?;
    emu.lw_no_count(16usize, 2usize, 328u32, 2153260u32)?;
    emu.lw_no_count(17usize, 2usize, 332u32, 2153264u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2153268u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2153272u32)?;
    emu.sw_no_count(14usize, 2usize, 288u32, 2153276u32)?;
    emu.sw_no_count(17usize, 2usize, 300u32, 2153280u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2153284u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2153288u32)?;
    emu.sw_no_count(14usize, 2usize, 256u32, 2153292u32)?;
    emu.sw_no_count(15usize, 2usize, 260u32, 2153296u32)?;
    emu.sw_no_count(16usize, 2usize, 264u32, 2153300u32)?;
    emu.sw_no_count(17usize, 2usize, 268u32, 2153304u32)?;
    emu.sw_no_count(10usize, 2usize, 272u32, 2153308u32)?;
    emu.sw_no_count(11usize, 2usize, 276u32, 2153312u32)?;
    emu.sw_no_count(12usize, 2usize, 280u32, 2153316u32)?;
    emu.sw_no_count(13usize, 2usize, 284u32, 2153320u32)?;
    emu.adi_no_count(10usize, 2usize, 288u32, 2153324u32);
    emu.adi_no_count(11usize, 2usize, 256u32, 2153328u32);
    emu.adi_no_count(12usize, 2usize, 96u32, 2153332u32);
    emu.apc_no_count(1usize, 2153332u32, 4294950912u32, 2153336u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2153340u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(192u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020db7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 320u32, 2153344u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2153348u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2153352u32);
    emu.apc_no_count(1usize, 2153352u32, 4294950912u32, 2153356u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2153360u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(172u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020db90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2153364u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2153368u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2153372u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2153376u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2153380u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2153384u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2153388u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2153392u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2153396u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2153400u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2153404u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2153408u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2153412u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2153416u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2153420u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2153424u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2153428u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2153432u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2153436u32);
    emu.apc_no_count(1usize, 2153436u32, 4294950912u32, 2153440u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2153444u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(88u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020dbe4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2153448u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2153452u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2153456u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2153460u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2153464u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2153468u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2153472u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2153476u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2153480u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2153484u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2153488u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2153492u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2153496u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2153500u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2153504u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2153508u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2153512u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2153516u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2153520u32);
    emu.apc_no_count(1usize, 2153520u32, 4294950912u32, 2153524u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2153528u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020dc38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2153532u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2153536u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2153540u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2153544u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2153548u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2153552u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2153556u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2153560u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2153564u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2153568u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2153572u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2153576u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2153580u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2153584u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2153588u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2153592u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2153596u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2153600u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2153604u32);
    emu.apc_no_count(1usize, 2153604u32, 4294950912u32, 2153608u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2153612u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967216u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020dc8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2153616u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2153620u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2153624u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2153628u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2153632u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2153636u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2153640u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2153644u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2153648u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2153652u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2153656u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2153660u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2153664u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2153668u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2153672u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2153676u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2153680u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2153684u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2153688u32);
    emu.apc_no_count(1usize, 2153688u32, 4294950912u32, 2153692u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2153696u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967132u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020dce0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2153700u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2153704u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2153708u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2153712u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2153716u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2153720u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2153724u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2153728u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2153732u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2153736u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2153740u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2153744u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2153748u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2153752u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2153756u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2153760u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2153764u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2153768u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2153772u32);
    emu.apc_no_count(1usize, 2153772u32, 4294950912u32, 2153776u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2153780u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967048u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020dd34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2153784u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2153788u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2153792u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2153796u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2153800u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2153804u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2153808u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2153812u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2153816u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2153820u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2153824u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2153828u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2153832u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2153836u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2153840u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2153844u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2153848u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2153852u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2153856u32);
    emu.apc_no_count(1usize, 2153856u32, 4294950912u32, 2153860u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2153864u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966964u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020dd88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2153868u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2153872u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2153876u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2153880u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2153884u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2153888u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2153892u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2153896u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2153900u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2153904u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2153908u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2153912u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2153916u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2153920u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2153924u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2153928u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2153932u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2153936u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2153940u32);
    emu.apc_no_count(1usize, 2153940u32, 4294950912u32, 2153944u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2153948u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966880u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020dddc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2153952u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2153956u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2153960u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2153964u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2153968u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2153972u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2153976u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2153980u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2153984u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2153988u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2153992u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2153996u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2154000u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2154004u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2154008u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2154012u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2154016u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2154020u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2154024u32);
    emu.apc_no_count(1usize, 2154024u32, 4294950912u32, 2154028u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2154032u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966796u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020de30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2154036u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2154040u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2154044u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2154048u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2154052u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2154056u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2154060u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2154064u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2154068u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2154072u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2154076u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2154080u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2154084u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2154088u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2154092u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2154096u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2154100u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2154104u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2154108u32);
    emu.apc_no_count(1usize, 2154108u32, 4294950912u32, 2154112u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2154116u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966712u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020de84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2154120u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2154124u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2154128u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2154132u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2154136u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2154140u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2154144u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2154148u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2154152u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2154156u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2154160u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2154164u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2154168u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2154172u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2154176u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2154180u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2154184u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2154188u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2154192u32);
    emu.apc_no_count(1usize, 2154192u32, 4294950912u32, 2154196u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2154200u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966628u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020ded8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2154204u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2154208u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2154212u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2154216u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2154220u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2154224u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2154228u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2154232u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2154236u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2154240u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2154244u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2154248u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2154252u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2154256u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2154260u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2154264u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2154268u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2154272u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2154276u32);
    emu.apc_no_count(1usize, 2154276u32, 4294950912u32, 2154280u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2154284u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966544u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020df2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2154288u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2154292u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2154296u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2154300u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2154304u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2154308u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2154312u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2154316u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2154320u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2154324u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2154328u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2154332u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2154336u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2154340u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2154344u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2154348u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2154352u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2154356u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2154360u32);
    emu.apc_no_count(1usize, 2154360u32, 4294950912u32, 2154364u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2154368u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966460u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020df80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2154372u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2154376u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2154380u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2154384u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2154388u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2154392u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2154396u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2154400u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2154404u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2154408u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2154412u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2154416u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2154420u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2154424u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2154428u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2154432u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2154436u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2154440u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2154444u32);
    emu.apc_no_count(1usize, 2154444u32, 4294950912u32, 2154448u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2154452u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966376u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020dfd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2154456u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2154460u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2154464u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2154468u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2154472u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2154476u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2154480u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2154484u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2154488u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2154492u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2154496u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2154500u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2154504u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2154508u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2154512u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2154516u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2154520u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2154524u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2154528u32);
    emu.apc_no_count(1usize, 2154528u32, 4294950912u32, 2154532u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2154536u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966292u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0020e028(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2154540u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2154544u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2154548u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2154552u32)?;
    emu.lw_no_count(14usize, 2usize, 320u32, 2154556u32)?;
    emu.lw_no_count(15usize, 2usize, 324u32, 2154560u32)?;
    emu.lw_no_count(16usize, 2usize, 328u32, 2154564u32)?;
    emu.lw_no_count(17usize, 2usize, 332u32, 2154568u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2154572u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2154576u32)?;
    emu.sw_no_count(14usize, 2usize, 288u32, 2154580u32)?;
    emu.sw_no_count(17usize, 2usize, 300u32, 2154584u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2154588u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2154592u32)?;
    emu.sw_no_count(14usize, 2usize, 128u32, 2154596u32)?;
    emu.sw_no_count(15usize, 2usize, 132u32, 2154600u32)?;
    emu.sw_no_count(16usize, 2usize, 136u32, 2154604u32)?;
    emu.sw_no_count(17usize, 2usize, 140u32, 2154608u32)?;
    emu.sw_no_count(10usize, 2usize, 144u32, 2154612u32)?;
    emu.sw_no_count(11usize, 2usize, 148u32, 2154616u32)?;
    emu.sw_no_count(12usize, 2usize, 152u32, 2154620u32)?;
    emu.sw_no_count(13usize, 2usize, 156u32, 2154624u32)?;
    emu.adi_no_count(10usize, 2usize, 160u32, 2154628u32);
    emu.adi_no_count(11usize, 2usize, 64u32, 2154632u32);
    emu.adi_no_count(12usize, 2usize, 128u32, 2154636u32);
    emu.apc_no_count(1usize, 2154636u32, 4294950912u32, 2154640u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2154644u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966184u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020e094(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 144u32, 2154648u32)?;
    emu.lw_no_count(11usize, 2usize, 148u32, 2154652u32)?;
    emu.lw_no_count(12usize, 2usize, 152u32, 2154656u32)?;
    emu.lw_no_count(13usize, 2usize, 156u32, 2154660u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2154664u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2154668u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2154672u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2154676u32)?;
    emu.lw_no_count(10usize, 2usize, 128u32, 2154680u32)?;
    emu.lw_no_count(11usize, 2usize, 132u32, 2154684u32)?;
    emu.lw_no_count(12usize, 2usize, 136u32, 2154688u32)?;
    emu.lw_no_count(13usize, 2usize, 140u32, 2154692u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2154696u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2154700u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2154704u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2154708u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2154712u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2154716u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2154720u32);
    emu.apc_no_count(1usize, 2154720u32, 4294950912u32, 2154724u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2154728u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966100u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020e0e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2154732u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2154736u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2154740u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2154744u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2154748u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2154752u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2154756u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2154760u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2154764u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2154768u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2154772u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2154776u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2154780u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2154784u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2154788u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2154792u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2154796u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2154800u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2154804u32);
    emu.apc_no_count(1usize, 2154804u32, 4294950912u32, 2154808u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2154812u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966016u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020e13c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2154816u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2154820u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2154824u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2154828u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2154832u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2154836u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2154840u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2154844u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2154848u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2154852u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2154856u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2154860u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2154864u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2154868u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2154872u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2154876u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2154880u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2154884u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2154888u32);
    emu.apc_no_count(1usize, 2154888u32, 4294950912u32, 2154892u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2154896u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965932u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020e190(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2154900u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2154904u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2154908u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2154912u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2154916u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2154920u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2154924u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2154928u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2154932u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2154936u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2154940u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2154944u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2154948u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2154952u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2154956u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2154960u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2154964u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2154968u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2154972u32);
    emu.apc_no_count(1usize, 2154972u32, 4294950912u32, 2154976u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2154980u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965848u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020e1e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2154984u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2154988u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2154992u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2154996u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2155000u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2155004u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2155008u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2155012u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2155016u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2155020u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2155024u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2155028u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2155032u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2155036u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2155040u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2155044u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2155048u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2155052u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2155056u32);
    emu.apc_no_count(1usize, 2155056u32, 4294950912u32, 2155060u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2155064u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965764u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020e238(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2155068u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2155072u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2155076u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2155080u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2155084u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2155088u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2155092u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2155096u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2155100u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2155104u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2155108u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2155112u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2155116u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2155120u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2155124u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2155128u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2155132u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2155136u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2155140u32);
    emu.apc_no_count(1usize, 2155140u32, 4294950912u32, 2155144u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2155148u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965680u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020e28c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2155152u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2155156u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2155160u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2155164u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2155168u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2155172u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2155176u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2155180u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2155184u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2155188u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2155192u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2155196u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2155200u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2155204u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2155208u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2155212u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2155216u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2155220u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2155224u32);
    emu.apc_no_count(1usize, 2155224u32, 4294950912u32, 2155228u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2155232u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965596u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020e2e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2155236u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2155240u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2155244u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2155248u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2155252u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2155256u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2155260u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2155264u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2155268u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2155272u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2155276u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2155280u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2155284u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2155288u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2155292u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2155296u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2155300u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2155304u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2155308u32);
    emu.apc_no_count(1usize, 2155308u32, 4294950912u32, 2155312u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2155316u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965512u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020e334(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2155320u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2155324u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2155328u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2155332u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2155336u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2155340u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2155344u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2155348u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2155352u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2155356u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2155360u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2155364u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2155368u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2155372u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2155376u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2155380u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2155384u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2155388u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2155392u32);
    emu.apc_no_count(1usize, 2155392u32, 4294950912u32, 2155396u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2155400u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965428u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020e388(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2155404u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2155408u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2155412u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2155416u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2155420u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2155424u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2155428u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2155432u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2155436u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2155440u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2155444u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2155448u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2155452u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2155456u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2155460u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2155464u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2155468u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2155472u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2155476u32);
    emu.apc_no_count(1usize, 2155476u32, 4294950912u32, 2155480u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2155484u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965344u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020e3dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2155488u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2155492u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2155496u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2155500u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2155504u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2155508u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2155512u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2155516u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2155520u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2155524u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2155528u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2155532u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2155536u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2155540u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2155544u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2155548u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2155552u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2155556u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2155560u32);
    emu.apc_no_count(1usize, 2155560u32, 4294950912u32, 2155564u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2155568u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965260u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020e430(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2155572u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2155576u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2155580u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2155584u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2155588u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2155592u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2155596u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2155600u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2155604u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2155608u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2155612u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2155616u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2155620u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2155624u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2155628u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2155632u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2155636u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2155640u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2155644u32);
    emu.apc_no_count(1usize, 2155644u32, 4294946816u32, 2155648u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2155652u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1976u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020e484(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2155656u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2155660u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2155664u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2155668u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2155672u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2155676u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2155680u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2155684u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2155688u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2155692u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2155696u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2155700u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2155704u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2155708u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2155712u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2155716u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2155720u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2155724u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2155728u32);
    emu.apc_no_count(1usize, 2155728u32, 4294946816u32, 2155732u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2155736u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1892u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020e4d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2155740u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2155744u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2155748u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2155752u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2155756u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2155760u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2155764u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2155768u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2155772u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2155776u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2155780u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2155784u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2155788u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2155792u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2155796u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2155800u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2155804u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2155808u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2155812u32);
    emu.apc_no_count(1usize, 2155812u32, 4294946816u32, 2155816u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2155820u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1808u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020e52c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2155824u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2155828u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2155832u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2155836u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2155840u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2155844u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2155848u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2155852u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2155856u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2155860u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2155864u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2155868u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2155872u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2155876u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2155880u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2155884u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2155888u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2155892u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2155896u32);
    emu.apc_no_count(1usize, 2155896u32, 4294946816u32, 2155900u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2155904u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1724u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020e580(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2155908u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2155912u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2155916u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2155920u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2155924u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2155928u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2155932u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2155936u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2155940u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2155944u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2155948u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2155952u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2155956u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2155960u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2155964u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2155968u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2155972u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2155976u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2155980u32);
    emu.apc_no_count(1usize, 2155980u32, 4294946816u32, 2155984u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2155988u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1640u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020e5d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2155992u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2155996u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2156000u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2156004u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2156008u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2156012u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2156016u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2156020u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2156024u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2156028u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2156032u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2156036u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2156040u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2156044u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2156048u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2156052u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2156056u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2156060u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2156064u32);
    emu.apc_no_count(1usize, 2156064u32, 4294946816u32, 2156068u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2156072u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1556u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0020e628(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2156076u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2156080u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2156084u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2156088u32)?;
    emu.lw_no_count(14usize, 2usize, 320u32, 2156092u32)?;
    emu.lw_no_count(15usize, 2usize, 324u32, 2156096u32)?;
    emu.lw_no_count(16usize, 2usize, 328u32, 2156100u32)?;
    emu.lw_no_count(17usize, 2usize, 332u32, 2156104u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2156108u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2156112u32)?;
    emu.sw_no_count(14usize, 2usize, 288u32, 2156116u32)?;
    emu.sw_no_count(17usize, 2usize, 300u32, 2156120u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2156124u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2156128u32)?;
    emu.sw_no_count(14usize, 2usize, 256u32, 2156132u32)?;
    emu.sw_no_count(15usize, 2usize, 260u32, 2156136u32)?;
    emu.sw_no_count(16usize, 2usize, 264u32, 2156140u32)?;
    emu.sw_no_count(17usize, 2usize, 268u32, 2156144u32)?;
    emu.sw_no_count(10usize, 2usize, 272u32, 2156148u32)?;
    emu.sw_no_count(11usize, 2usize, 276u32, 2156152u32)?;
    emu.sw_no_count(12usize, 2usize, 280u32, 2156156u32)?;
    emu.sw_no_count(13usize, 2usize, 284u32, 2156160u32)?;
    emu.adi_no_count(10usize, 2usize, 288u32, 2156164u32);
    emu.adi_no_count(11usize, 2usize, 256u32, 2156168u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2156172u32);
    emu.apc_no_count(1usize, 2156172u32, 4294946816u32, 2156176u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2156180u32;
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
#[inline(always)]
pub fn block_0x0020e694(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 143u32, 2156184u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2156184u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e698));
}
#[inline(always)]
pub fn block_0x0020e698(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 320u32, 2156188u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2156192u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2156196u32);
    emu.apc_no_count(1usize, 2156196u32, 4294946816u32, 2156200u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2156204u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1424u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020e6ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2156208u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2156212u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2156216u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2156220u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2156224u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2156228u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2156232u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2156236u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2156240u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2156244u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2156248u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2156252u32)?;
    emu.adi_no_count(18usize, 18usize, 4294967295u32, 2156256u32);
    emu.sw_no_count(10usize, 2usize, 288u32, 2156260u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2156264u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2156268u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2156272u32)?;
    emu.add_memory_rw_events(17usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2156184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e698));
    } else {
        emu.pc = 2156276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e6f4));
    }
}
#[inline]
pub fn block_0x0020e6f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 304u32, 2156280u32)?;
    emu.lw_no_count(11usize, 2usize, 308u32, 2156284u32)?;
    emu.lw_no_count(12usize, 2usize, 312u32, 2156288u32)?;
    emu.lw_no_count(13usize, 2usize, 316u32, 2156292u32)?;
    emu.sw_no_count(10usize, 2usize, 240u32, 2156296u32)?;
    emu.sw_no_count(11usize, 2usize, 244u32, 2156300u32)?;
    emu.sw_no_count(12usize, 2usize, 248u32, 2156304u32)?;
    emu.sw_no_count(13usize, 2usize, 252u32, 2156308u32)?;
    emu.lw_no_count(10usize, 2usize, 288u32, 2156312u32)?;
    emu.lw_no_count(11usize, 2usize, 292u32, 2156316u32)?;
    emu.lw_no_count(12usize, 2usize, 296u32, 2156320u32)?;
    emu.lw_no_count(13usize, 2usize, 300u32, 2156324u32)?;
    emu.sw_no_count(10usize, 2usize, 224u32, 2156328u32)?;
    emu.sw_no_count(11usize, 2usize, 228u32, 2156332u32)?;
    emu.sw_no_count(12usize, 2usize, 232u32, 2156336u32)?;
    emu.sw_no_count(13usize, 2usize, 236u32, 2156340u32)?;
    emu.adi_no_count(10usize, 2usize, 288u32, 2156344u32);
    emu.adi_no_count(11usize, 2usize, 224u32, 2156348u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2156352u32);
    emu.apc_no_count(1usize, 2156352u32, 4294946816u32, 2156356u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2156360u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1268u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020e748(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 47u32, 2156364u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2156364u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e74c));
}
#[inline(always)]
pub fn block_0x0020e74c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 320u32, 2156368u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2156372u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2156376u32);
    emu.apc_no_count(1usize, 2156376u32, 4294946816u32, 2156380u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2156384u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1244u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020e760(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2156388u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2156392u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2156396u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2156400u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2156404u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2156408u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2156412u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2156416u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2156420u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2156424u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2156428u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2156432u32)?;
    emu.adi_no_count(18usize, 18usize, 4294967295u32, 2156436u32);
    emu.sw_no_count(10usize, 2usize, 288u32, 2156440u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2156444u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2156448u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2156452u32)?;
    emu.add_memory_rw_events(17usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2156364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e74c));
    } else {
        emu.pc = 2156456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e7a8));
    }
}
#[inline]
pub fn block_0x0020e7a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 304u32, 2156460u32)?;
    emu.lw_no_count(11usize, 2usize, 308u32, 2156464u32)?;
    emu.lw_no_count(12usize, 2usize, 312u32, 2156468u32)?;
    emu.lw_no_count(13usize, 2usize, 316u32, 2156472u32)?;
    emu.sw_no_count(10usize, 2usize, 208u32, 2156476u32)?;
    emu.sw_no_count(11usize, 2usize, 212u32, 2156480u32)?;
    emu.sw_no_count(12usize, 2usize, 216u32, 2156484u32)?;
    emu.sw_no_count(13usize, 2usize, 220u32, 2156488u32)?;
    emu.lw_no_count(10usize, 2usize, 288u32, 2156492u32)?;
    emu.lw_no_count(11usize, 2usize, 292u32, 2156496u32)?;
    emu.lw_no_count(12usize, 2usize, 296u32, 2156500u32)?;
    emu.lw_no_count(13usize, 2usize, 300u32, 2156504u32)?;
    emu.sw_no_count(10usize, 2usize, 192u32, 2156508u32)?;
    emu.sw_no_count(11usize, 2usize, 196u32, 2156512u32)?;
    emu.sw_no_count(12usize, 2usize, 200u32, 2156516u32)?;
    emu.sw_no_count(13usize, 2usize, 204u32, 2156520u32)?;
    emu.adi_no_count(10usize, 2usize, 288u32, 2156524u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2156528u32);
    emu.adi_no_count(12usize, 2usize, 192u32, 2156532u32);
    emu.apc_no_count(1usize, 2156532u32, 4294946816u32, 2156536u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2156540u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1088u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020e7fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 320u32, 2156544u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2156548u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2156552u32);
    emu.apc_no_count(1usize, 2156552u32, 4294946816u32, 2156556u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2156560u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1068u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020e810(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2156564u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2156568u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2156572u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2156576u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2156580u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2156584u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2156588u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2156592u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2156596u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2156600u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2156604u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2156608u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2156612u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2156616u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2156620u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2156624u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2156628u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2156632u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2156636u32);
    emu.apc_no_count(1usize, 2156636u32, 4294946816u32, 2156640u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2156644u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(984u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0020e864(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2156648u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2156652u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2156656u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2156660u32)?;
    emu.lw_no_count(14usize, 2usize, 320u32, 2156664u32)?;
    emu.lw_no_count(15usize, 2usize, 324u32, 2156668u32)?;
    emu.lw_no_count(16usize, 2usize, 328u32, 2156672u32)?;
    emu.lw_no_count(17usize, 2usize, 332u32, 2156676u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2156680u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2156684u32)?;
    emu.sw_no_count(14usize, 2usize, 288u32, 2156688u32)?;
    emu.sw_no_count(17usize, 2usize, 300u32, 2156692u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2156696u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2156700u32)?;
    emu.sw_no_count(14usize, 2usize, 320u32, 2156704u32)?;
    emu.sw_no_count(15usize, 2usize, 324u32, 2156708u32)?;
    emu.sw_no_count(16usize, 2usize, 328u32, 2156712u32)?;
    emu.sw_no_count(17usize, 2usize, 332u32, 2156716u32)?;
    emu.sw_no_count(10usize, 2usize, 336u32, 2156720u32)?;
    emu.sw_no_count(11usize, 2usize, 340u32, 2156724u32)?;
    emu.sw_no_count(12usize, 2usize, 344u32, 2156728u32)?;
    emu.sw_no_count(13usize, 2usize, 348u32, 2156732u32)?;
    emu.adi_no_count(11usize, 2usize, 320u32, 2156736u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2156740u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2156744u32);
    emu.apc_no_count(1usize, 2156744u32, 4294946816u32, 2156748u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2156752u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(876u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020e8d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 364u32, 2156756u32)?;
    emu.lw_no_count(8usize, 2usize, 360u32, 2156760u32)?;
    emu.lw_no_count(9usize, 2usize, 356u32, 2156764u32)?;
    emu.lw_no_count(18usize, 2usize, 352u32, 2156768u32)?;
    emu.adi_no_count(2usize, 2usize, 368u32, 2156772u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2156776u32;
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
pub fn block_0x0020e8e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967056u32, 2156780u32);
    emu.sw_no_count(1usize, 2usize, 236u32, 2156784u32)?;
    emu.sw_no_count(8usize, 2usize, 232u32, 2156788u32)?;
    emu.sw_no_count(9usize, 2usize, 228u32, 2156792u32)?;
    emu.sw_no_count(18usize, 2usize, 224u32, 2156796u32)?;
    emu.adi_no_count(9usize, 11usize, 0u32, 2156800u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2156804u32);
    emu.adi_no_count(10usize, 2usize, 192u32, 2156808u32);
    emu.adi_no_count(12usize, 11usize, 0u32, 2156812u32);
    emu.apc_no_count(1usize, 2156812u32, 4294946816u32, 2156816u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2156820u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(808u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020e914(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2156824u32);
    emu.adi_no_count(12usize, 2usize, 192u32, 2156828u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2156832u32);
    emu.apc_no_count(1usize, 2156832u32, 4294946816u32, 2156836u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2156840u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(788u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0020e928(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 0u32, 2156844u32)?;
    emu.lw_no_count(11usize, 2usize, 0u32, 2156848u32)?;
    emu.lw_no_count(12usize, 2usize, 4u32, 2156852u32)?;
    emu.lw_no_count(13usize, 2usize, 24u32, 2156856u32)?;
    emu.lw_no_count(14usize, 2usize, 24u32, 2156860u32)?;
    emu.lw_no_count(15usize, 2usize, 28u32, 2156864u32)?;
    emu.lw_no_count(16usize, 2usize, 28u32, 2156868u32)?;
    emu.lw_no_count(17usize, 2usize, 16u32, 2156872u32)?;
    emu.lw_no_count(5usize, 2usize, 16u32, 2156876u32)?;
    emu.lw_no_count(6usize, 2usize, 20u32, 2156880u32)?;
    emu.lw_no_count(7usize, 2usize, 20u32, 2156884u32)?;
    emu.lw_no_count(28usize, 2usize, 8u32, 2156888u32)?;
    emu.lw_no_count(29usize, 2usize, 8u32, 2156892u32)?;
    emu.lw_no_count(30usize, 2usize, 12u32, 2156896u32)?;
    emu.lw_no_count(31usize, 2usize, 12u32, 2156900u32)?;
    emu.sw_no_count(17usize, 2usize, 144u32, 2156904u32)?;
    emu.sw_no_count(6usize, 2usize, 148u32, 2156908u32)?;
    emu.sw_no_count(13usize, 2usize, 152u32, 2156912u32)?;
    emu.sw_no_count(15usize, 2usize, 156u32, 2156916u32)?;
    emu.lw_no_count(13usize, 2usize, 4u32, 2156920u32)?;
    emu.sw_no_count(10usize, 2usize, 128u32, 2156924u32)?;
    emu.sw_no_count(12usize, 2usize, 132u32, 2156928u32)?;
    emu.sw_no_count(28usize, 2usize, 136u32, 2156932u32)?;
    emu.sw_no_count(30usize, 2usize, 140u32, 2156936u32)?;
    emu.sw_no_count(5usize, 2usize, 176u32, 2156940u32)?;
    emu.sw_no_count(7usize, 2usize, 180u32, 2156944u32)?;
    emu.sw_no_count(14usize, 2usize, 184u32, 2156948u32)?;
    emu.sw_no_count(16usize, 2usize, 188u32, 2156952u32)?;
    emu.sw_no_count(11usize, 2usize, 160u32, 2156956u32)?;
    emu.sw_no_count(13usize, 2usize, 164u32, 2156960u32)?;
    emu.sw_no_count(29usize, 2usize, 168u32, 2156964u32)?;
    emu.sw_no_count(31usize, 2usize, 172u32, 2156968u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2156972u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2156976u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2156980u32);
    emu.apc_no_count(1usize, 2156980u32, 4294946816u32, 2156984u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2156988u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(640u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020e9bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2156992u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2156996u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2157000u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2157004u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2157008u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2157012u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2157016u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2157020u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2157024u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2157028u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2157032u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2157036u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2157040u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2157044u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2157048u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2157052u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2157056u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2157060u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2157064u32);
    emu.apc_no_count(1usize, 2157064u32, 4294946816u32, 2157068u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2157072u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(556u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0020ea10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2157076u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2157080u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2157084u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2157088u32)?;
    emu.lw_no_count(14usize, 2usize, 192u32, 2157092u32)?;
    emu.lw_no_count(15usize, 2usize, 196u32, 2157096u32)?;
    emu.lw_no_count(16usize, 2usize, 200u32, 2157100u32)?;
    emu.lw_no_count(17usize, 2usize, 204u32, 2157104u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2157108u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2157112u32)?;
    emu.sw_no_count(14usize, 2usize, 160u32, 2157116u32)?;
    emu.sw_no_count(17usize, 2usize, 172u32, 2157120u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2157124u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2157128u32)?;
    emu.sw_no_count(14usize, 2usize, 192u32, 2157132u32)?;
    emu.sw_no_count(15usize, 2usize, 196u32, 2157136u32)?;
    emu.sw_no_count(16usize, 2usize, 200u32, 2157140u32)?;
    emu.sw_no_count(17usize, 2usize, 204u32, 2157144u32)?;
    emu.sw_no_count(10usize, 2usize, 208u32, 2157148u32)?;
    emu.sw_no_count(11usize, 2usize, 212u32, 2157152u32)?;
    emu.sw_no_count(12usize, 2usize, 216u32, 2157156u32)?;
    emu.sw_no_count(13usize, 2usize, 220u32, 2157160u32)?;
    emu.adi_no_count(10usize, 2usize, 32u32, 2157164u32);
    emu.adi_no_count(11usize, 2usize, 128u32, 2157168u32);
    emu.adi_no_count(12usize, 2usize, 192u32, 2157172u32);
    emu.apc_no_count(1usize, 2157172u32, 4294946816u32, 2157176u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2157180u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(448u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0020ea7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 32u32, 2157184u32)?;
    emu.lw_no_count(11usize, 2usize, 32u32, 2157188u32)?;
    emu.lw_no_count(12usize, 2usize, 36u32, 2157192u32)?;
    emu.lw_no_count(13usize, 2usize, 56u32, 2157196u32)?;
    emu.lw_no_count(14usize, 2usize, 56u32, 2157200u32)?;
    emu.lw_no_count(15usize, 2usize, 60u32, 2157204u32)?;
    emu.lw_no_count(16usize, 2usize, 60u32, 2157208u32)?;
    emu.lw_no_count(17usize, 2usize, 48u32, 2157212u32)?;
    emu.lw_no_count(5usize, 2usize, 48u32, 2157216u32)?;
    emu.lw_no_count(6usize, 2usize, 52u32, 2157220u32)?;
    emu.lw_no_count(7usize, 2usize, 52u32, 2157224u32)?;
    emu.lw_no_count(28usize, 2usize, 40u32, 2157228u32)?;
    emu.lw_no_count(29usize, 2usize, 40u32, 2157232u32)?;
    emu.lw_no_count(30usize, 2usize, 44u32, 2157236u32)?;
    emu.lw_no_count(31usize, 2usize, 44u32, 2157240u32)?;
    emu.sw_no_count(17usize, 2usize, 144u32, 2157244u32)?;
    emu.sw_no_count(6usize, 2usize, 148u32, 2157248u32)?;
    emu.sw_no_count(13usize, 2usize, 152u32, 2157252u32)?;
    emu.sw_no_count(15usize, 2usize, 156u32, 2157256u32)?;
    emu.lw_no_count(13usize, 2usize, 36u32, 2157260u32)?;
    emu.sw_no_count(10usize, 2usize, 128u32, 2157264u32)?;
    emu.sw_no_count(12usize, 2usize, 132u32, 2157268u32)?;
    emu.sw_no_count(28usize, 2usize, 136u32, 2157272u32)?;
    emu.sw_no_count(30usize, 2usize, 140u32, 2157276u32)?;
    emu.sw_no_count(5usize, 2usize, 176u32, 2157280u32)?;
    emu.sw_no_count(7usize, 2usize, 180u32, 2157284u32)?;
    emu.sw_no_count(14usize, 2usize, 184u32, 2157288u32)?;
    emu.sw_no_count(16usize, 2usize, 188u32, 2157292u32)?;
    emu.sw_no_count(11usize, 2usize, 160u32, 2157296u32)?;
    emu.sw_no_count(13usize, 2usize, 164u32, 2157300u32)?;
    emu.sw_no_count(29usize, 2usize, 168u32, 2157304u32)?;
    emu.sw_no_count(31usize, 2usize, 172u32, 2157308u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2157312u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2157316u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2157320u32);
    emu.apc_no_count(1usize, 2157320u32, 4294946816u32, 2157324u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2157328u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(300u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020eb10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2157332u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2157336u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2157340u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2157344u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2157348u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2157352u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2157356u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2157360u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2157364u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2157368u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2157372u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2157376u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2157380u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2157384u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2157388u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2157392u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2157396u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2157400u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2157404u32);
    emu.apc_no_count(1usize, 2157404u32, 4294946816u32, 2157408u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2157412u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(216u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020eb64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2157416u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2157420u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2157424u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2157428u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2157432u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2157436u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2157440u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2157444u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2157448u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2157452u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2157456u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2157460u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2157464u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2157468u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2157472u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2157476u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2157480u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2157484u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2157488u32);
    emu.apc_no_count(1usize, 2157488u32, 4294946816u32, 2157492u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2157496u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(132u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020ebb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2157500u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2157504u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2157508u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2157512u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2157516u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2157520u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2157524u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2157528u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2157532u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2157536u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2157540u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2157544u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2157548u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2157552u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2157556u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2157560u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2157564u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2157568u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2157572u32);
    emu.apc_no_count(1usize, 2157572u32, 4294946816u32, 2157576u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2157580u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(48u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0020ec0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2157584u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2157588u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2157592u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2157596u32)?;
    emu.lw_no_count(14usize, 2usize, 192u32, 2157600u32)?;
    emu.lw_no_count(15usize, 2usize, 196u32, 2157604u32)?;
    emu.lw_no_count(16usize, 2usize, 200u32, 2157608u32)?;
    emu.lw_no_count(17usize, 2usize, 204u32, 2157612u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2157616u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2157620u32)?;
    emu.sw_no_count(14usize, 2usize, 160u32, 2157624u32)?;
    emu.sw_no_count(17usize, 2usize, 172u32, 2157628u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2157632u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2157636u32)?;
    emu.sw_no_count(14usize, 2usize, 192u32, 2157640u32)?;
    emu.sw_no_count(15usize, 2usize, 196u32, 2157644u32)?;
    emu.sw_no_count(16usize, 2usize, 200u32, 2157648u32)?;
    emu.sw_no_count(17usize, 2usize, 204u32, 2157652u32)?;
    emu.sw_no_count(10usize, 2usize, 208u32, 2157656u32)?;
    emu.sw_no_count(11usize, 2usize, 212u32, 2157660u32)?;
    emu.sw_no_count(12usize, 2usize, 216u32, 2157664u32)?;
    emu.sw_no_count(13usize, 2usize, 220u32, 2157668u32)?;
    emu.adi_no_count(10usize, 2usize, 64u32, 2157672u32);
    emu.adi_no_count(11usize, 2usize, 128u32, 2157676u32);
    emu.adi_no_count(12usize, 2usize, 192u32, 2157680u32);
    emu.apc_no_count(1usize, 2157680u32, 4294946816u32, 2157684u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2157688u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967236u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020ec78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 80u32, 2157692u32)?;
    emu.lw_no_count(11usize, 2usize, 84u32, 2157696u32)?;
    emu.lw_no_count(12usize, 2usize, 88u32, 2157700u32)?;
    emu.lw_no_count(13usize, 2usize, 92u32, 2157704u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2157708u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2157712u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2157716u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2157720u32)?;
    emu.lw_no_count(10usize, 2usize, 64u32, 2157724u32)?;
    emu.lw_no_count(11usize, 2usize, 68u32, 2157728u32)?;
    emu.lw_no_count(12usize, 2usize, 72u32, 2157732u32)?;
    emu.lw_no_count(13usize, 2usize, 76u32, 2157736u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2157740u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2157744u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2157748u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2157752u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2157756u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2157760u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2157764u32);
    emu.apc_no_count(1usize, 2157764u32, 4294946816u32, 2157768u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2157772u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967152u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020eccc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2157776u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2157780u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2157784u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2157788u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2157792u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2157796u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2157800u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2157804u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2157808u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2157812u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2157816u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2157820u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2157824u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2157828u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2157832u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2157836u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2157840u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2157844u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2157848u32);
    emu.apc_no_count(1usize, 2157848u32, 4294946816u32, 2157852u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2157856u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967068u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020ed20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2157860u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2157864u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2157868u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2157872u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2157876u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2157880u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2157884u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2157888u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2157892u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2157896u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2157900u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2157904u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2157908u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2157912u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2157916u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2157920u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2157924u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2157928u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2157932u32);
    emu.apc_no_count(1usize, 2157932u32, 4294946816u32, 2157936u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2157940u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966984u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
