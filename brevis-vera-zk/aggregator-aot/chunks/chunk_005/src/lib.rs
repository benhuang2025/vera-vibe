pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2114360u32;
pub const PC_MAX: u32 = 2120812u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 108usize] = [
        block_0x00204338,
        block_0x00204404,
        block_0x00204498,
        block_0x0020452c,
        block_0x002045c0,
        block_0x00204654,
        block_0x00204668,
        block_0x002046fc,
        block_0x00204710,
        block_0x002047a4,
        block_0x00204838,
        block_0x0020484c,
        block_0x002048e0,
        block_0x002048f4,
        block_0x00204988,
        block_0x00204a1c,
        block_0x00204a30,
        block_0x00204ac4,
        block_0x00204ad8,
        block_0x00204b8c,
        block_0x00204be0,
        block_0x00204bf0,
        block_0x00204c44,
        block_0x00204cd8,
        block_0x00204cec,
        block_0x00204cfc,
        block_0x00204d50,
        block_0x00204d84,
        block_0x00204e18,
        block_0x00204e2c,
        block_0x00204e3c,
        block_0x00204e90,
        block_0x00204ea0,
        block_0x00204ef4,
        block_0x00204f08,
        block_0x00204f9c,
        block_0x00205030,
        block_0x00205044,
        block_0x00205098,
        block_0x002050ec,
        block_0x00205100,
        block_0x00205114,
        block_0x00205128,
        block_0x0020513c,
        block_0x0020516c,
        block_0x002051b0,
        block_0x002051c0,
        block_0x002051d0,
        block_0x00205264,
        block_0x00205274,
        block_0x00205308,
        block_0x00205318,
        block_0x002053cc,
        block_0x00205420,
        block_0x00205430,
        block_0x00205484,
        block_0x00205518,
        block_0x0020556c,
        block_0x002055c0,
        block_0x002055d4,
        block_0x002055e4,
        block_0x00205638,
        block_0x0020566c,
        block_0x00205700,
        block_0x00205714,
        block_0x00205724,
        block_0x00205778,
        block_0x00205788,
        block_0x002057dc,
        block_0x002057f0,
        block_0x00205844,
        block_0x00205858,
        block_0x002058ec,
        block_0x002058fc,
        block_0x00205950,
        block_0x00205964,
        block_0x00205978,
        block_0x00205988,
        block_0x00205998,
        block_0x00205a0c,
        block_0x00205a28,
        block_0x00205a30,
        block_0x00205a38,
        block_0x00205a54,
        block_0x00205a5c,
        block_0x00205a60,
        block_0x00205a7c,
        block_0x00205ab0,
        block_0x00205abc,
        block_0x00205ae0,
        block_0x00205aec,
        block_0x00205afc,
        block_0x00205b2c,
        block_0x00205b38,
        block_0x00205b4c,
        block_0x00205b58,
        block_0x00205b74,
        block_0x00205b78,
        block_0x00205ba0,
        block_0x00205bac,
        block_0x00205bb4,
        block_0x00205bd0,
        block_0x00205bec,
        block_0x00205c14,
        block_0x00205c20,
        block_0x00205c30,
        block_0x00205c60,
        block_0x00205c6c,
    ];
    #[repr(C)]
    struct Run {
        start_word: u32,
        len: u16,
        fn_offset: u16,
    }
    const RUNS: [Run; 106usize] = [
        Run {
            start_word: 0u32,
            len: 1i32 as u16,
            fn_offset: 0usize as u16,
        },
        Run {
            start_word: 51u32,
            len: 1i32 as u16,
            fn_offset: 1usize as u16,
        },
        Run {
            start_word: 88u32,
            len: 1i32 as u16,
            fn_offset: 2usize as u16,
        },
        Run {
            start_word: 125u32,
            len: 1i32 as u16,
            fn_offset: 3usize as u16,
        },
        Run {
            start_word: 162u32,
            len: 1i32 as u16,
            fn_offset: 4usize as u16,
        },
        Run {
            start_word: 199u32,
            len: 1i32 as u16,
            fn_offset: 5usize as u16,
        },
        Run {
            start_word: 204u32,
            len: 1i32 as u16,
            fn_offset: 6usize as u16,
        },
        Run {
            start_word: 241u32,
            len: 1i32 as u16,
            fn_offset: 7usize as u16,
        },
        Run {
            start_word: 246u32,
            len: 1i32 as u16,
            fn_offset: 8usize as u16,
        },
        Run {
            start_word: 283u32,
            len: 1i32 as u16,
            fn_offset: 9usize as u16,
        },
        Run {
            start_word: 320u32,
            len: 1i32 as u16,
            fn_offset: 10usize as u16,
        },
        Run {
            start_word: 325u32,
            len: 1i32 as u16,
            fn_offset: 11usize as u16,
        },
        Run {
            start_word: 362u32,
            len: 1i32 as u16,
            fn_offset: 12usize as u16,
        },
        Run {
            start_word: 367u32,
            len: 1i32 as u16,
            fn_offset: 13usize as u16,
        },
        Run {
            start_word: 404u32,
            len: 1i32 as u16,
            fn_offset: 14usize as u16,
        },
        Run {
            start_word: 441u32,
            len: 1i32 as u16,
            fn_offset: 15usize as u16,
        },
        Run {
            start_word: 446u32,
            len: 1i32 as u16,
            fn_offset: 16usize as u16,
        },
        Run {
            start_word: 483u32,
            len: 1i32 as u16,
            fn_offset: 17usize as u16,
        },
        Run {
            start_word: 488u32,
            len: 1i32 as u16,
            fn_offset: 18usize as u16,
        },
        Run {
            start_word: 533u32,
            len: 1i32 as u16,
            fn_offset: 19usize as u16,
        },
        Run {
            start_word: 554u32,
            len: 1i32 as u16,
            fn_offset: 20usize as u16,
        },
        Run {
            start_word: 558u32,
            len: 1i32 as u16,
            fn_offset: 21usize as u16,
        },
        Run {
            start_word: 579u32,
            len: 1i32 as u16,
            fn_offset: 22usize as u16,
        },
        Run {
            start_word: 616u32,
            len: 1i32 as u16,
            fn_offset: 23usize as u16,
        },
        Run {
            start_word: 621u32,
            len: 1i32 as u16,
            fn_offset: 24usize as u16,
        },
        Run {
            start_word: 625u32,
            len: 1i32 as u16,
            fn_offset: 25usize as u16,
        },
        Run {
            start_word: 646u32,
            len: 1i32 as u16,
            fn_offset: 26usize as u16,
        },
        Run {
            start_word: 659u32,
            len: 1i32 as u16,
            fn_offset: 27usize as u16,
        },
        Run {
            start_word: 696u32,
            len: 1i32 as u16,
            fn_offset: 28usize as u16,
        },
        Run {
            start_word: 701u32,
            len: 1i32 as u16,
            fn_offset: 29usize as u16,
        },
        Run {
            start_word: 705u32,
            len: 1i32 as u16,
            fn_offset: 30usize as u16,
        },
        Run {
            start_word: 726u32,
            len: 1i32 as u16,
            fn_offset: 31usize as u16,
        },
        Run {
            start_word: 730u32,
            len: 1i32 as u16,
            fn_offset: 32usize as u16,
        },
        Run {
            start_word: 751u32,
            len: 1i32 as u16,
            fn_offset: 33usize as u16,
        },
        Run {
            start_word: 756u32,
            len: 1i32 as u16,
            fn_offset: 34usize as u16,
        },
        Run {
            start_word: 793u32,
            len: 1i32 as u16,
            fn_offset: 35usize as u16,
        },
        Run {
            start_word: 830u32,
            len: 1i32 as u16,
            fn_offset: 36usize as u16,
        },
        Run {
            start_word: 835u32,
            len: 1i32 as u16,
            fn_offset: 37usize as u16,
        },
        Run {
            start_word: 856u32,
            len: 1i32 as u16,
            fn_offset: 38usize as u16,
        },
        Run {
            start_word: 877u32,
            len: 1i32 as u16,
            fn_offset: 39usize as u16,
        },
        Run {
            start_word: 882u32,
            len: 1i32 as u16,
            fn_offset: 40usize as u16,
        },
        Run {
            start_word: 887u32,
            len: 1i32 as u16,
            fn_offset: 41usize as u16,
        },
        Run {
            start_word: 892u32,
            len: 1i32 as u16,
            fn_offset: 42usize as u16,
        },
        Run {
            start_word: 897u32,
            len: 1i32 as u16,
            fn_offset: 43usize as u16,
        },
        Run {
            start_word: 909u32,
            len: 1i32 as u16,
            fn_offset: 44usize as u16,
        },
        Run {
            start_word: 926u32,
            len: 1i32 as u16,
            fn_offset: 45usize as u16,
        },
        Run {
            start_word: 930u32,
            len: 1i32 as u16,
            fn_offset: 46usize as u16,
        },
        Run {
            start_word: 934u32,
            len: 1i32 as u16,
            fn_offset: 47usize as u16,
        },
        Run {
            start_word: 971u32,
            len: 1i32 as u16,
            fn_offset: 48usize as u16,
        },
        Run {
            start_word: 975u32,
            len: 1i32 as u16,
            fn_offset: 49usize as u16,
        },
        Run {
            start_word: 1012u32,
            len: 1i32 as u16,
            fn_offset: 50usize as u16,
        },
        Run {
            start_word: 1016u32,
            len: 1i32 as u16,
            fn_offset: 51usize as u16,
        },
        Run {
            start_word: 1061u32,
            len: 1i32 as u16,
            fn_offset: 52usize as u16,
        },
        Run {
            start_word: 1082u32,
            len: 1i32 as u16,
            fn_offset: 53usize as u16,
        },
        Run {
            start_word: 1086u32,
            len: 1i32 as u16,
            fn_offset: 54usize as u16,
        },
        Run {
            start_word: 1107u32,
            len: 1i32 as u16,
            fn_offset: 55usize as u16,
        },
        Run {
            start_word: 1144u32,
            len: 1i32 as u16,
            fn_offset: 56usize as u16,
        },
        Run {
            start_word: 1165u32,
            len: 1i32 as u16,
            fn_offset: 57usize as u16,
        },
        Run {
            start_word: 1186u32,
            len: 1i32 as u16,
            fn_offset: 58usize as u16,
        },
        Run {
            start_word: 1191u32,
            len: 1i32 as u16,
            fn_offset: 59usize as u16,
        },
        Run {
            start_word: 1195u32,
            len: 1i32 as u16,
            fn_offset: 60usize as u16,
        },
        Run {
            start_word: 1216u32,
            len: 1i32 as u16,
            fn_offset: 61usize as u16,
        },
        Run {
            start_word: 1229u32,
            len: 1i32 as u16,
            fn_offset: 62usize as u16,
        },
        Run {
            start_word: 1266u32,
            len: 1i32 as u16,
            fn_offset: 63usize as u16,
        },
        Run {
            start_word: 1271u32,
            len: 1i32 as u16,
            fn_offset: 64usize as u16,
        },
        Run {
            start_word: 1275u32,
            len: 1i32 as u16,
            fn_offset: 65usize as u16,
        },
        Run {
            start_word: 1296u32,
            len: 1i32 as u16,
            fn_offset: 66usize as u16,
        },
        Run {
            start_word: 1300u32,
            len: 1i32 as u16,
            fn_offset: 67usize as u16,
        },
        Run {
            start_word: 1321u32,
            len: 1i32 as u16,
            fn_offset: 68usize as u16,
        },
        Run {
            start_word: 1326u32,
            len: 1i32 as u16,
            fn_offset: 69usize as u16,
        },
        Run {
            start_word: 1347u32,
            len: 1i32 as u16,
            fn_offset: 70usize as u16,
        },
        Run {
            start_word: 1352u32,
            len: 1i32 as u16,
            fn_offset: 71usize as u16,
        },
        Run {
            start_word: 1389u32,
            len: 1i32 as u16,
            fn_offset: 72usize as u16,
        },
        Run {
            start_word: 1393u32,
            len: 1i32 as u16,
            fn_offset: 73usize as u16,
        },
        Run {
            start_word: 1414u32,
            len: 1i32 as u16,
            fn_offset: 74usize as u16,
        },
        Run {
            start_word: 1419u32,
            len: 1i32 as u16,
            fn_offset: 75usize as u16,
        },
        Run {
            start_word: 1424u32,
            len: 1i32 as u16,
            fn_offset: 76usize as u16,
        },
        Run {
            start_word: 1428u32,
            len: 1i32 as u16,
            fn_offset: 77usize as u16,
        },
        Run {
            start_word: 1432u32,
            len: 1i32 as u16,
            fn_offset: 78usize as u16,
        },
        Run {
            start_word: 1461u32,
            len: 1i32 as u16,
            fn_offset: 79usize as u16,
        },
        Run {
            start_word: 1468u32,
            len: 1i32 as u16,
            fn_offset: 80usize as u16,
        },
        Run {
            start_word: 1470u32,
            len: 1i32 as u16,
            fn_offset: 81usize as u16,
        },
        Run {
            start_word: 1472u32,
            len: 1i32 as u16,
            fn_offset: 82usize as u16,
        },
        Run {
            start_word: 1479u32,
            len: 1i32 as u16,
            fn_offset: 83usize as u16,
        },
        Run {
            start_word: 1481u32,
            len: 2i32 as u16,
            fn_offset: 84usize as u16,
        },
        Run {
            start_word: 1489u32,
            len: 1i32 as u16,
            fn_offset: 86usize as u16,
        },
        Run {
            start_word: 1502u32,
            len: 1i32 as u16,
            fn_offset: 87usize as u16,
        },
        Run {
            start_word: 1505u32,
            len: 1i32 as u16,
            fn_offset: 88usize as u16,
        },
        Run {
            start_word: 1514u32,
            len: 1i32 as u16,
            fn_offset: 89usize as u16,
        },
        Run {
            start_word: 1517u32,
            len: 1i32 as u16,
            fn_offset: 90usize as u16,
        },
        Run {
            start_word: 1521u32,
            len: 1i32 as u16,
            fn_offset: 91usize as u16,
        },
        Run {
            start_word: 1533u32,
            len: 1i32 as u16,
            fn_offset: 92usize as u16,
        },
        Run {
            start_word: 1536u32,
            len: 1i32 as u16,
            fn_offset: 93usize as u16,
        },
        Run {
            start_word: 1541u32,
            len: 1i32 as u16,
            fn_offset: 94usize as u16,
        },
        Run {
            start_word: 1544u32,
            len: 1i32 as u16,
            fn_offset: 95usize as u16,
        },
        Run {
            start_word: 1551u32,
            len: 2i32 as u16,
            fn_offset: 96usize as u16,
        },
        Run {
            start_word: 1562u32,
            len: 1i32 as u16,
            fn_offset: 98usize as u16,
        },
        Run {
            start_word: 1565u32,
            len: 1i32 as u16,
            fn_offset: 99usize as u16,
        },
        Run {
            start_word: 1567u32,
            len: 1i32 as u16,
            fn_offset: 100usize as u16,
        },
        Run {
            start_word: 1574u32,
            len: 1i32 as u16,
            fn_offset: 101usize as u16,
        },
        Run {
            start_word: 1581u32,
            len: 1i32 as u16,
            fn_offset: 102usize as u16,
        },
        Run {
            start_word: 1591u32,
            len: 1i32 as u16,
            fn_offset: 103usize as u16,
        },
        Run {
            start_word: 1594u32,
            len: 1i32 as u16,
            fn_offset: 104usize as u16,
        },
        Run {
            start_word: 1598u32,
            len: 1i32 as u16,
            fn_offset: 105usize as u16,
        },
        Run {
            start_word: 1610u32,
            len: 1i32 as u16,
            fn_offset: 106usize as u16,
        },
        Run {
            start_word: 1613u32,
            len: 1i32 as u16,
            fn_offset: 107usize as u16,
        },
    ];
    if pc < 2114360u32 || pc > 2120812u32 {
        return None;
    }
    let word_offset = ((pc - 2114360u32) >> 2) as u32;
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
#[inline(never)]
pub fn block_0x00204338(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 51u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294966672u32, 2114364u32);
    emu.sw_no_count(1usize, 2usize, 620u32, 2114368u32)?;
    emu.sw_no_count(8usize, 2usize, 616u32, 2114372u32)?;
    emu.sw_no_count(9usize, 2usize, 612u32, 2114376u32)?;
    emu.sw_no_count(18usize, 2usize, 608u32, 2114380u32)?;
    emu.sw_no_count(19usize, 2usize, 604u32, 2114384u32)?;
    emu.sw_no_count(20usize, 2usize, 600u32, 2114388u32)?;
    emu.sw_no_count(21usize, 2usize, 596u32, 2114392u32)?;
    emu.sw_no_count(22usize, 2usize, 592u32, 2114396u32)?;
    emu.sw_no_count(23usize, 2usize, 588u32, 2114400u32)?;
    emu.sw_no_count(24usize, 2usize, 584u32, 2114404u32)?;
    emu.adi_no_count(9usize, 12usize, 0u32, 2114408u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2114412u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2114416u32);
    emu.lw_no_count(10usize, 11usize, 16u32, 2114420u32)?;
    emu.lw_no_count(11usize, 11usize, 20u32, 2114424u32)?;
    emu.lw_no_count(12usize, 18usize, 24u32, 2114428u32)?;
    emu.lw_no_count(13usize, 18usize, 28u32, 2114432u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2114436u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2114440u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2114444u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2114448u32)?;
    emu.lw_no_count(10usize, 18usize, 0u32, 2114452u32)?;
    emu.lw_no_count(11usize, 18usize, 4u32, 2114456u32)?;
    emu.lw_no_count(12usize, 18usize, 8u32, 2114460u32)?;
    emu.lw_no_count(13usize, 18usize, 12u32, 2114464u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2114468u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2114472u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2114476u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2114480u32)?;
    emu.lw_no_count(10usize, 9usize, 16u32, 2114484u32)?;
    emu.lw_no_count(11usize, 9usize, 20u32, 2114488u32)?;
    emu.lw_no_count(12usize, 9usize, 24u32, 2114492u32)?;
    emu.lw_no_count(13usize, 9usize, 28u32, 2114496u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2114500u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2114504u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2114508u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2114512u32)?;
    emu.lw_no_count(10usize, 9usize, 0u32, 2114516u32)?;
    emu.lw_no_count(11usize, 9usize, 4u32, 2114520u32)?;
    emu.lw_no_count(12usize, 9usize, 8u32, 2114524u32)?;
    emu.lw_no_count(13usize, 9usize, 12u32, 2114528u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2114532u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2114536u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2114540u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2114544u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2114548u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2114552u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2114556u32);
    emu.apc_no_count(1usize, 2114556u32, 24576u32, 2114560u32);
    emu.add_memory_rw_events(51usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114564u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965268u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00204404(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 48u32, 2114568u32)?;
    emu.lw_no_count(11usize, 18usize, 52u32, 2114572u32)?;
    emu.lw_no_count(12usize, 18usize, 56u32, 2114576u32)?;
    emu.lw_no_count(13usize, 18usize, 60u32, 2114580u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2114584u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2114588u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2114592u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2114596u32)?;
    emu.lw_no_count(10usize, 18usize, 32u32, 2114600u32)?;
    emu.lw_no_count(11usize, 18usize, 36u32, 2114604u32)?;
    emu.lw_no_count(12usize, 18usize, 40u32, 2114608u32)?;
    emu.lw_no_count(13usize, 18usize, 44u32, 2114612u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2114616u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2114620u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2114624u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2114628u32)?;
    emu.lw_no_count(10usize, 9usize, 48u32, 2114632u32)?;
    emu.lw_no_count(11usize, 9usize, 52u32, 2114636u32)?;
    emu.lw_no_count(12usize, 9usize, 56u32, 2114640u32)?;
    emu.lw_no_count(13usize, 9usize, 60u32, 2114644u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2114648u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2114652u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2114656u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2114660u32)?;
    emu.lw_no_count(10usize, 9usize, 32u32, 2114664u32)?;
    emu.lw_no_count(11usize, 9usize, 36u32, 2114668u32)?;
    emu.lw_no_count(12usize, 9usize, 40u32, 2114672u32)?;
    emu.lw_no_count(13usize, 9usize, 44u32, 2114676u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2114680u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2114684u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2114688u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2114692u32)?;
    emu.adi_no_count(10usize, 2usize, 40u32, 2114696u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2114700u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2114704u32);
    emu.apc_no_count(1usize, 2114704u32, 20480u32, 2114708u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114712u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1920u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00204498(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 80u32, 2114716u32)?;
    emu.lw_no_count(11usize, 18usize, 84u32, 2114720u32)?;
    emu.lw_no_count(12usize, 18usize, 88u32, 2114724u32)?;
    emu.lw_no_count(13usize, 18usize, 92u32, 2114728u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2114732u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2114736u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2114740u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2114744u32)?;
    emu.lw_no_count(10usize, 18usize, 64u32, 2114748u32)?;
    emu.lw_no_count(11usize, 18usize, 68u32, 2114752u32)?;
    emu.lw_no_count(12usize, 18usize, 72u32, 2114756u32)?;
    emu.lw_no_count(13usize, 18usize, 76u32, 2114760u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2114764u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2114768u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2114772u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2114776u32)?;
    emu.lw_no_count(10usize, 9usize, 80u32, 2114780u32)?;
    emu.lw_no_count(11usize, 9usize, 84u32, 2114784u32)?;
    emu.lw_no_count(12usize, 9usize, 88u32, 2114788u32)?;
    emu.lw_no_count(13usize, 9usize, 92u32, 2114792u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2114796u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2114800u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2114804u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2114808u32)?;
    emu.lw_no_count(10usize, 9usize, 64u32, 2114812u32)?;
    emu.lw_no_count(11usize, 9usize, 68u32, 2114816u32)?;
    emu.lw_no_count(12usize, 9usize, 72u32, 2114820u32)?;
    emu.lw_no_count(13usize, 9usize, 76u32, 2114824u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2114828u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2114832u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2114836u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2114840u32)?;
    emu.adi_no_count(10usize, 2usize, 72u32, 2114844u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2114848u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2114852u32);
    emu.apc_no_count(1usize, 2114852u32, 20480u32, 2114856u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114860u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1772u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0020452c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 16u32, 2114864u32)?;
    emu.lw_no_count(11usize, 18usize, 20u32, 2114868u32)?;
    emu.lw_no_count(12usize, 18usize, 24u32, 2114872u32)?;
    emu.lw_no_count(13usize, 18usize, 28u32, 2114876u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2114880u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2114884u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2114888u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2114892u32)?;
    emu.lw_no_count(10usize, 18usize, 0u32, 2114896u32)?;
    emu.lw_no_count(11usize, 18usize, 4u32, 2114900u32)?;
    emu.lw_no_count(12usize, 18usize, 8u32, 2114904u32)?;
    emu.lw_no_count(13usize, 18usize, 12u32, 2114908u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2114912u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2114916u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2114920u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2114924u32)?;
    emu.lw_no_count(10usize, 18usize, 48u32, 2114928u32)?;
    emu.lw_no_count(11usize, 18usize, 52u32, 2114932u32)?;
    emu.lw_no_count(12usize, 18usize, 56u32, 2114936u32)?;
    emu.lw_no_count(13usize, 18usize, 60u32, 2114940u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2114944u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2114948u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2114952u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2114956u32)?;
    emu.lw_no_count(10usize, 18usize, 32u32, 2114960u32)?;
    emu.lw_no_count(11usize, 18usize, 36u32, 2114964u32)?;
    emu.lw_no_count(12usize, 18usize, 40u32, 2114968u32)?;
    emu.lw_no_count(13usize, 18usize, 44u32, 2114972u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2114976u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2114980u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2114984u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2114988u32)?;
    emu.adi_no_count(10usize, 2usize, 456u32, 2114992u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2114996u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2115000u32);
    emu.apc_no_count(1usize, 2115000u32, 20480u32, 2115004u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2115008u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(332u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x002045c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 16u32, 2115012u32)?;
    emu.lw_no_count(11usize, 9usize, 20u32, 2115016u32)?;
    emu.lw_no_count(12usize, 9usize, 24u32, 2115020u32)?;
    emu.lw_no_count(13usize, 9usize, 28u32, 2115024u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2115028u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2115032u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2115036u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2115040u32)?;
    emu.lw_no_count(10usize, 9usize, 0u32, 2115044u32)?;
    emu.lw_no_count(11usize, 9usize, 4u32, 2115048u32)?;
    emu.lw_no_count(12usize, 9usize, 8u32, 2115052u32)?;
    emu.lw_no_count(13usize, 9usize, 12u32, 2115056u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2115060u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2115064u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2115068u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2115072u32)?;
    emu.lw_no_count(10usize, 9usize, 48u32, 2115076u32)?;
    emu.lw_no_count(11usize, 9usize, 52u32, 2115080u32)?;
    emu.lw_no_count(12usize, 9usize, 56u32, 2115084u32)?;
    emu.lw_no_count(13usize, 9usize, 60u32, 2115088u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2115092u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2115096u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2115100u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2115104u32)?;
    emu.lw_no_count(10usize, 9usize, 32u32, 2115108u32)?;
    emu.lw_no_count(11usize, 9usize, 36u32, 2115112u32)?;
    emu.lw_no_count(12usize, 9usize, 40u32, 2115116u32)?;
    emu.lw_no_count(13usize, 9usize, 44u32, 2115120u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2115124u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2115128u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2115132u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2115136u32)?;
    emu.adi_no_count(10usize, 2usize, 488u32, 2115140u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2115144u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2115148u32);
    emu.apc_no_count(1usize, 2115148u32, 20480u32, 2115152u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2115156u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(184u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204654(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 360u32, 2115160u32);
    emu.adi_no_count(11usize, 2usize, 456u32, 2115164u32);
    emu.adi_no_count(12usize, 2usize, 488u32, 2115168u32);
    emu.apc_no_count(1usize, 2115168u32, 20480u32, 2115172u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2115176u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1456u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00204668(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 24u32, 2115180u32)?;
    emu.lw_no_count(11usize, 2usize, 28u32, 2115184u32)?;
    emu.lw_no_count(12usize, 2usize, 32u32, 2115188u32)?;
    emu.lw_no_count(13usize, 2usize, 36u32, 2115192u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2115196u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2115200u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2115204u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2115208u32)?;
    emu.lw_no_count(10usize, 2usize, 8u32, 2115212u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2115216u32)?;
    emu.lw_no_count(12usize, 2usize, 16u32, 2115220u32)?;
    emu.lw_no_count(13usize, 2usize, 20u32, 2115224u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2115228u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2115232u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2115236u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2115240u32)?;
    emu.lw_no_count(10usize, 2usize, 56u32, 2115244u32)?;
    emu.lw_no_count(11usize, 2usize, 60u32, 2115248u32)?;
    emu.lw_no_count(12usize, 2usize, 64u32, 2115252u32)?;
    emu.lw_no_count(13usize, 2usize, 68u32, 2115256u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2115260u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2115264u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2115268u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2115272u32)?;
    emu.lw_no_count(10usize, 2usize, 40u32, 2115276u32)?;
    emu.lw_no_count(11usize, 2usize, 44u32, 2115280u32)?;
    emu.lw_no_count(12usize, 2usize, 48u32, 2115284u32)?;
    emu.lw_no_count(13usize, 2usize, 52u32, 2115288u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2115292u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2115296u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2115300u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2115304u32)?;
    emu.adi_no_count(10usize, 2usize, 488u32, 2115308u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2115312u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2115316u32);
    emu.apc_no_count(1usize, 2115316u32, 20480u32, 2115320u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2115324u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(16u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002046fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 104u32, 2115328u32);
    emu.adi_no_count(11usize, 2usize, 360u32, 2115332u32);
    emu.adi_no_count(12usize, 2usize, 488u32, 2115336u32);
    emu.apc_no_count(1usize, 2115336u32, 20480u32, 2115340u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2115344u32;
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
#[inline(never)]
pub fn block_0x00204710(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 48u32, 2115348u32)?;
    emu.lw_no_count(11usize, 18usize, 52u32, 2115352u32)?;
    emu.lw_no_count(12usize, 18usize, 56u32, 2115356u32)?;
    emu.lw_no_count(13usize, 18usize, 60u32, 2115360u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2115364u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2115368u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2115372u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2115376u32)?;
    emu.lw_no_count(10usize, 18usize, 32u32, 2115380u32)?;
    emu.lw_no_count(11usize, 18usize, 36u32, 2115384u32)?;
    emu.lw_no_count(12usize, 18usize, 40u32, 2115388u32)?;
    emu.lw_no_count(13usize, 18usize, 44u32, 2115392u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2115396u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2115400u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2115404u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2115408u32)?;
    emu.lw_no_count(10usize, 18usize, 80u32, 2115412u32)?;
    emu.lw_no_count(11usize, 18usize, 84u32, 2115416u32)?;
    emu.lw_no_count(12usize, 18usize, 88u32, 2115420u32)?;
    emu.lw_no_count(13usize, 18usize, 92u32, 2115424u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2115428u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2115432u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2115436u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2115440u32)?;
    emu.lw_no_count(10usize, 18usize, 64u32, 2115444u32)?;
    emu.lw_no_count(11usize, 18usize, 68u32, 2115448u32)?;
    emu.lw_no_count(12usize, 18usize, 72u32, 2115452u32)?;
    emu.lw_no_count(13usize, 18usize, 76u32, 2115456u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2115460u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2115464u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2115468u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2115472u32)?;
    emu.adi_no_count(10usize, 2usize, 456u32, 2115476u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2115480u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2115484u32);
    emu.apc_no_count(1usize, 2115484u32, 20480u32, 2115488u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2115492u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967144u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x002047a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 48u32, 2115496u32)?;
    emu.lw_no_count(11usize, 9usize, 52u32, 2115500u32)?;
    emu.lw_no_count(12usize, 9usize, 56u32, 2115504u32)?;
    emu.lw_no_count(13usize, 9usize, 60u32, 2115508u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2115512u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2115516u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2115520u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2115524u32)?;
    emu.lw_no_count(10usize, 9usize, 32u32, 2115528u32)?;
    emu.lw_no_count(11usize, 9usize, 36u32, 2115532u32)?;
    emu.lw_no_count(12usize, 9usize, 40u32, 2115536u32)?;
    emu.lw_no_count(13usize, 9usize, 44u32, 2115540u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2115544u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2115548u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2115552u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2115556u32)?;
    emu.lw_no_count(10usize, 9usize, 80u32, 2115560u32)?;
    emu.lw_no_count(11usize, 9usize, 84u32, 2115564u32)?;
    emu.lw_no_count(12usize, 9usize, 88u32, 2115568u32)?;
    emu.lw_no_count(13usize, 9usize, 92u32, 2115572u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2115576u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2115580u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2115584u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2115588u32)?;
    emu.lw_no_count(10usize, 9usize, 64u32, 2115592u32)?;
    emu.lw_no_count(11usize, 9usize, 68u32, 2115596u32)?;
    emu.lw_no_count(12usize, 9usize, 72u32, 2115600u32)?;
    emu.lw_no_count(13usize, 9usize, 76u32, 2115604u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2115608u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2115612u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2115616u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2115620u32)?;
    emu.adi_no_count(10usize, 2usize, 488u32, 2115624u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2115628u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2115632u32);
    emu.apc_no_count(1usize, 2115632u32, 20480u32, 2115636u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2115640u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966996u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204838(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 360u32, 2115644u32);
    emu.adi_no_count(11usize, 2usize, 456u32, 2115648u32);
    emu.adi_no_count(12usize, 2usize, 488u32, 2115652u32);
    emu.apc_no_count(1usize, 2115652u32, 20480u32, 2115656u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2115660u32;
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
#[inline(never)]
pub fn block_0x0020484c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 88u32, 2115664u32)?;
    emu.lw_no_count(11usize, 2usize, 92u32, 2115668u32)?;
    emu.lw_no_count(12usize, 2usize, 96u32, 2115672u32)?;
    emu.lw_no_count(13usize, 2usize, 100u32, 2115676u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2115680u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2115684u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2115688u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2115692u32)?;
    emu.lw_no_count(10usize, 2usize, 72u32, 2115696u32)?;
    emu.lw_no_count(11usize, 2usize, 76u32, 2115700u32)?;
    emu.lw_no_count(12usize, 2usize, 80u32, 2115704u32)?;
    emu.lw_no_count(13usize, 2usize, 84u32, 2115708u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2115712u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2115716u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2115720u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2115724u32)?;
    emu.lw_no_count(10usize, 2usize, 56u32, 2115728u32)?;
    emu.lw_no_count(11usize, 2usize, 60u32, 2115732u32)?;
    emu.lw_no_count(12usize, 2usize, 64u32, 2115736u32)?;
    emu.lw_no_count(13usize, 2usize, 68u32, 2115740u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2115744u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2115748u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2115752u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2115756u32)?;
    emu.lw_no_count(10usize, 2usize, 40u32, 2115760u32)?;
    emu.lw_no_count(11usize, 2usize, 44u32, 2115764u32)?;
    emu.lw_no_count(12usize, 2usize, 48u32, 2115768u32)?;
    emu.lw_no_count(13usize, 2usize, 52u32, 2115772u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2115776u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2115780u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2115784u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2115788u32)?;
    emu.adi_no_count(10usize, 2usize, 488u32, 2115792u32);
    emu.adi_no_count(11usize, 2usize, 552u32, 2115796u32);
    emu.adi_no_count(12usize, 2usize, 520u32, 2115800u32);
    emu.apc_no_count(1usize, 2115800u32, 20480u32, 2115804u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2115808u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966828u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002048e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 136u32, 2115812u32);
    emu.adi_no_count(11usize, 2usize, 360u32, 2115816u32);
    emu.adi_no_count(12usize, 2usize, 488u32, 2115820u32);
    emu.apc_no_count(1usize, 2115820u32, 20480u32, 2115824u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2115828u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(236u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x002048f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 16u32, 2115832u32)?;
    emu.lw_no_count(11usize, 18usize, 20u32, 2115836u32)?;
    emu.lw_no_count(12usize, 18usize, 24u32, 2115840u32)?;
    emu.lw_no_count(13usize, 18usize, 28u32, 2115844u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2115848u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2115852u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2115856u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2115860u32)?;
    emu.lw_no_count(10usize, 18usize, 0u32, 2115864u32)?;
    emu.lw_no_count(11usize, 18usize, 4u32, 2115868u32)?;
    emu.lw_no_count(12usize, 18usize, 8u32, 2115872u32)?;
    emu.lw_no_count(13usize, 18usize, 12u32, 2115876u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2115880u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2115884u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2115888u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2115892u32)?;
    emu.lw_no_count(10usize, 18usize, 80u32, 2115896u32)?;
    emu.lw_no_count(11usize, 18usize, 84u32, 2115900u32)?;
    emu.lw_no_count(12usize, 18usize, 88u32, 2115904u32)?;
    emu.lw_no_count(13usize, 18usize, 92u32, 2115908u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2115912u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2115916u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2115920u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2115924u32)?;
    emu.lw_no_count(10usize, 18usize, 64u32, 2115928u32)?;
    emu.lw_no_count(11usize, 18usize, 68u32, 2115932u32)?;
    emu.lw_no_count(12usize, 18usize, 72u32, 2115936u32)?;
    emu.lw_no_count(13usize, 18usize, 76u32, 2115940u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2115944u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2115948u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2115952u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2115956u32)?;
    emu.adi_no_count(10usize, 2usize, 456u32, 2115960u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2115964u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2115968u32);
    emu.apc_no_count(1usize, 2115968u32, 20480u32, 2115972u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2115976u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966660u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00204988(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 16u32, 2115980u32)?;
    emu.lw_no_count(11usize, 9usize, 20u32, 2115984u32)?;
    emu.lw_no_count(12usize, 9usize, 24u32, 2115988u32)?;
    emu.lw_no_count(13usize, 9usize, 28u32, 2115992u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2115996u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2116000u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2116004u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2116008u32)?;
    emu.lw_no_count(10usize, 9usize, 0u32, 2116012u32)?;
    emu.lw_no_count(11usize, 9usize, 4u32, 2116016u32)?;
    emu.lw_no_count(12usize, 9usize, 8u32, 2116020u32)?;
    emu.lw_no_count(13usize, 9usize, 12u32, 2116024u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2116028u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2116032u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2116036u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2116040u32)?;
    emu.lw_no_count(10usize, 9usize, 80u32, 2116044u32)?;
    emu.lw_no_count(11usize, 9usize, 84u32, 2116048u32)?;
    emu.lw_no_count(12usize, 9usize, 88u32, 2116052u32)?;
    emu.lw_no_count(13usize, 9usize, 92u32, 2116056u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2116060u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2116064u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2116068u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2116072u32)?;
    emu.lw_no_count(10usize, 9usize, 64u32, 2116076u32)?;
    emu.lw_no_count(11usize, 9usize, 68u32, 2116080u32)?;
    emu.lw_no_count(12usize, 9usize, 72u32, 2116084u32)?;
    emu.lw_no_count(13usize, 9usize, 76u32, 2116088u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2116092u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2116096u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2116100u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2116104u32)?;
    emu.adi_no_count(10usize, 2usize, 488u32, 2116108u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2116112u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2116116u32);
    emu.apc_no_count(1usize, 2116116u32, 20480u32, 2116120u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116124u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966512u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204a1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 360u32, 2116128u32);
    emu.adi_no_count(11usize, 2usize, 456u32, 2116132u32);
    emu.adi_no_count(12usize, 2usize, 488u32, 2116136u32);
    emu.apc_no_count(1usize, 2116136u32, 20480u32, 2116140u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116144u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(488u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00204a30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 24u32, 2116148u32)?;
    emu.lw_no_count(11usize, 2usize, 28u32, 2116152u32)?;
    emu.lw_no_count(12usize, 2usize, 32u32, 2116156u32)?;
    emu.lw_no_count(13usize, 2usize, 36u32, 2116160u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2116164u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2116168u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2116172u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2116176u32)?;
    emu.lw_no_count(10usize, 2usize, 8u32, 2116180u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2116184u32)?;
    emu.lw_no_count(12usize, 2usize, 16u32, 2116188u32)?;
    emu.lw_no_count(13usize, 2usize, 20u32, 2116192u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2116196u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2116200u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2116204u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2116208u32)?;
    emu.lw_no_count(10usize, 2usize, 88u32, 2116212u32)?;
    emu.lw_no_count(11usize, 2usize, 92u32, 2116216u32)?;
    emu.lw_no_count(12usize, 2usize, 96u32, 2116220u32)?;
    emu.lw_no_count(13usize, 2usize, 100u32, 2116224u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2116228u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2116232u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2116236u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2116240u32)?;
    emu.lw_no_count(10usize, 2usize, 72u32, 2116244u32)?;
    emu.lw_no_count(11usize, 2usize, 76u32, 2116248u32)?;
    emu.lw_no_count(12usize, 2usize, 80u32, 2116252u32)?;
    emu.lw_no_count(13usize, 2usize, 84u32, 2116256u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2116260u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2116264u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2116268u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2116272u32)?;
    emu.adi_no_count(10usize, 2usize, 488u32, 2116276u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2116280u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2116284u32);
    emu.apc_no_count(1usize, 2116284u32, 20480u32, 2116288u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116292u32;
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
#[inline(always)]
pub fn block_0x00204ac4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 168u32, 2116296u32);
    emu.adi_no_count(11usize, 2usize, 360u32, 2116300u32);
    emu.adi_no_count(12usize, 2usize, 488u32, 2116304u32);
    emu.apc_no_count(1usize, 2116304u32, 20480u32, 2116308u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116312u32;
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
#[inline(never)]
pub fn block_0x00204ad8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 45u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 88u32, 2116316u32)?;
    emu.lw_no_count(11usize, 2usize, 92u32, 2116320u32)?;
    emu.lw_no_count(12usize, 2usize, 96u32, 2116324u32)?;
    emu.lw_no_count(13usize, 2usize, 100u32, 2116328u32)?;
    emu.lw_no_count(14usize, 2usize, 72u32, 2116332u32)?;
    emu.lw_no_count(15usize, 2usize, 76u32, 2116336u32)?;
    emu.lw_no_count(16usize, 2usize, 80u32, 2116340u32)?;
    emu.lw_no_count(17usize, 2usize, 84u32, 2116344u32)?;
    let a = 0u32.wrapping_add(3694133248u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2116348u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(75976704u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2116352u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(3852607488u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2116356u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4146147328u32);
    emu.write_reg_no_count(28usize, a);
    emu.pc = 2116360u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2901409792u32);
    emu.write_reg_no_count(29usize, a);
    emu.pc = 2116364u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2021928960u32);
    emu.write_reg_no_count(30usize, a);
    emu.pc = 2116368u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 2usize, 536u32, 2116372u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2116376u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2116380u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2116384u32)?;
    let a = 0u32.wrapping_add(3634159616u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2116388u32;
    emu.update_insn_clock();
    emu.sw_no_count(14usize, 2usize, 520u32, 2116392u32)?;
    emu.sw_no_count(15usize, 2usize, 524u32, 2116396u32)?;
    emu.sw_no_count(16usize, 2usize, 528u32, 2116400u32)?;
    emu.sw_no_count(17usize, 2usize, 532u32, 2116404u32)?;
    let a = 0u32.wrapping_add(700760064u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2116408u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 5usize, 1565u32, 2116412u32);
    emu.adi_no_count(21usize, 6usize, 4294965300u32, 2116416u32);
    emu.adi_no_count(23usize, 7usize, 171u32, 2116420u32);
    emu.adi_no_count(24usize, 28usize, 4294966998u32, 2116424u32);
    emu.adi_no_count(9usize, 29usize, 1485u32, 2116428u32);
    emu.adi_no_count(18usize, 30usize, 144u32, 2116432u32);
    emu.adi_no_count(20usize, 10usize, 4294967138u32, 2116436u32);
    emu.adi_no_count(22usize, 11usize, 4294966751u32, 2116440u32);
    emu.sw_no_count(24usize, 2usize, 568u32, 2116444u32)?;
    emu.sw_no_count(23usize, 2usize, 572u32, 2116448u32)?;
    emu.sw_no_count(21usize, 2usize, 576u32, 2116452u32)?;
    emu.sw_no_count(19usize, 2usize, 580u32, 2116456u32)?;
    emu.sw_no_count(22usize, 2usize, 552u32, 2116460u32)?;
    emu.sw_no_count(20usize, 2usize, 556u32, 2116464u32)?;
    emu.sw_no_count(18usize, 2usize, 560u32, 2116468u32)?;
    emu.sw_no_count(9usize, 2usize, 564u32, 2116472u32)?;
    emu.adi_no_count(10usize, 2usize, 488u32, 2116476u32);
    emu.adi_no_count(11usize, 2usize, 552u32, 2116480u32);
    emu.adi_no_count(12usize, 2usize, 520u32, 2116484u32);
    emu.apc_no_count(1usize, 2116484u32, 20480u32, 2116488u32);
    emu.add_memory_rw_events(45usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116492u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(140u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00204b8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 184u32, 2116496u32)?;
    emu.lw_no_count(11usize, 2usize, 188u32, 2116500u32)?;
    emu.lw_no_count(12usize, 2usize, 192u32, 2116504u32)?;
    emu.lw_no_count(13usize, 2usize, 196u32, 2116508u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2116512u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2116516u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2116520u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2116524u32)?;
    emu.lw_no_count(10usize, 2usize, 168u32, 2116528u32)?;
    emu.lw_no_count(11usize, 2usize, 172u32, 2116532u32)?;
    emu.lw_no_count(12usize, 2usize, 176u32, 2116536u32)?;
    emu.lw_no_count(13usize, 2usize, 180u32, 2116540u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2116544u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2116548u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2116552u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2116556u32)?;
    emu.adi_no_count(10usize, 2usize, 200u32, 2116560u32);
    emu.adi_no_count(11usize, 2usize, 552u32, 2116564u32);
    emu.adi_no_count(12usize, 2usize, 488u32, 2116568u32);
    emu.apc_no_count(1usize, 2116568u32, 20480u32, 2116572u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116576u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966784u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204be0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 520u32, 2116580u32);
    emu.adi_no_count(11usize, 2usize, 200u32, 2116584u32);
    emu.apc_no_count(1usize, 2116584u32, 49152u32, 2116588u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116592u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(352u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00204bf0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 216u32, 2116596u32)?;
    emu.lw_no_count(11usize, 2usize, 220u32, 2116600u32)?;
    emu.lw_no_count(12usize, 2usize, 224u32, 2116604u32)?;
    emu.lw_no_count(13usize, 2usize, 228u32, 2116608u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2116612u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2116616u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2116620u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2116624u32)?;
    emu.lw_no_count(10usize, 2usize, 200u32, 2116628u32)?;
    emu.lw_no_count(11usize, 2usize, 204u32, 2116632u32)?;
    emu.lw_no_count(12usize, 2usize, 208u32, 2116636u32)?;
    emu.lw_no_count(13usize, 2usize, 212u32, 2116640u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2116644u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2116648u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2116652u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2116656u32)?;
    emu.adi_no_count(10usize, 2usize, 232u32, 2116660u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2116664u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2116668u32);
    emu.apc_no_count(1usize, 2116668u32, 20480u32, 2116672u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116676u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965960u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00204c44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 56u32, 2116680u32)?;
    emu.lw_no_count(11usize, 2usize, 60u32, 2116684u32)?;
    emu.lw_no_count(12usize, 2usize, 64u32, 2116688u32)?;
    emu.lw_no_count(13usize, 2usize, 68u32, 2116692u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2116696u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2116700u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2116704u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2116708u32)?;
    emu.lw_no_count(10usize, 2usize, 40u32, 2116712u32)?;
    emu.lw_no_count(11usize, 2usize, 44u32, 2116716u32)?;
    emu.lw_no_count(12usize, 2usize, 48u32, 2116720u32)?;
    emu.lw_no_count(13usize, 2usize, 52u32, 2116724u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2116728u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2116732u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2116736u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2116740u32)?;
    emu.lw_no_count(10usize, 2usize, 248u32, 2116744u32)?;
    emu.lw_no_count(11usize, 2usize, 252u32, 2116748u32)?;
    emu.lw_no_count(12usize, 2usize, 256u32, 2116752u32)?;
    emu.lw_no_count(13usize, 2usize, 260u32, 2116756u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2116760u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2116764u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2116768u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2116772u32)?;
    emu.lw_no_count(10usize, 2usize, 232u32, 2116776u32)?;
    emu.lw_no_count(11usize, 2usize, 236u32, 2116780u32)?;
    emu.lw_no_count(12usize, 2usize, 240u32, 2116784u32)?;
    emu.lw_no_count(13usize, 2usize, 244u32, 2116788u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2116792u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2116796u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2116800u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2116804u32)?;
    emu.adi_no_count(10usize, 2usize, 264u32, 2116808u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2116812u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2116816u32);
    emu.apc_no_count(1usize, 2116816u32, 20480u32, 2116820u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116824u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966536u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204cd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 296u32, 2116828u32);
    emu.adi_no_count(11usize, 2usize, 40u32, 2116832u32);
    emu.adi_no_count(12usize, 2usize, 232u32, 2116836u32);
    emu.apc_no_count(1usize, 2116836u32, 20480u32, 2116840u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116844u32;
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
#[inline(always)]
pub fn block_0x00204cec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 520u32, 2116848u32);
    emu.adi_no_count(11usize, 2usize, 72u32, 2116852u32);
    emu.apc_no_count(1usize, 2116852u32, 49152u32, 2116856u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116860u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(84u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00204cfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 88u32, 2116864u32)?;
    emu.lw_no_count(11usize, 2usize, 92u32, 2116868u32)?;
    emu.lw_no_count(12usize, 2usize, 96u32, 2116872u32)?;
    emu.lw_no_count(13usize, 2usize, 100u32, 2116876u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2116880u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2116884u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2116888u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2116892u32)?;
    emu.lw_no_count(10usize, 2usize, 72u32, 2116896u32)?;
    emu.lw_no_count(11usize, 2usize, 76u32, 2116900u32)?;
    emu.lw_no_count(12usize, 2usize, 80u32, 2116904u32)?;
    emu.lw_no_count(13usize, 2usize, 84u32, 2116908u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2116912u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2116916u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2116920u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2116924u32)?;
    emu.adi_no_count(10usize, 2usize, 328u32, 2116928u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2116932u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2116936u32);
    emu.apc_no_count(1usize, 2116936u32, 20480u32, 2116940u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116944u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965692u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00204d50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(24usize, 2usize, 568u32, 2116948u32)?;
    emu.sw_no_count(23usize, 2usize, 572u32, 2116952u32)?;
    emu.sw_no_count(21usize, 2usize, 576u32, 2116956u32)?;
    emu.sw_no_count(19usize, 2usize, 580u32, 2116960u32)?;
    emu.sw_no_count(22usize, 2usize, 552u32, 2116964u32)?;
    emu.sw_no_count(20usize, 2usize, 556u32, 2116968u32)?;
    emu.sw_no_count(18usize, 2usize, 560u32, 2116972u32)?;
    emu.sw_no_count(9usize, 2usize, 564u32, 2116976u32)?;
    emu.adi_no_count(10usize, 2usize, 456u32, 2116980u32);
    emu.adi_no_count(11usize, 2usize, 552u32, 2116984u32);
    emu.adi_no_count(12usize, 2usize, 168u32, 2116988u32);
    emu.apc_no_count(1usize, 2116988u32, 20480u32, 2116992u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116996u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966932u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00204d84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 24u32, 2117000u32)?;
    emu.lw_no_count(11usize, 2usize, 28u32, 2117004u32)?;
    emu.lw_no_count(12usize, 2usize, 32u32, 2117008u32)?;
    emu.lw_no_count(13usize, 2usize, 36u32, 2117012u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2117016u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2117020u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2117024u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2117028u32)?;
    emu.lw_no_count(10usize, 2usize, 8u32, 2117032u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2117036u32)?;
    emu.lw_no_count(12usize, 2usize, 16u32, 2117040u32)?;
    emu.lw_no_count(13usize, 2usize, 20u32, 2117044u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2117048u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2117052u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2117056u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2117060u32)?;
    emu.lw_no_count(10usize, 2usize, 344u32, 2117064u32)?;
    emu.lw_no_count(11usize, 2usize, 348u32, 2117068u32)?;
    emu.lw_no_count(12usize, 2usize, 352u32, 2117072u32)?;
    emu.lw_no_count(13usize, 2usize, 356u32, 2117076u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2117080u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2117084u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2117088u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2117092u32)?;
    emu.lw_no_count(10usize, 2usize, 328u32, 2117096u32)?;
    emu.lw_no_count(11usize, 2usize, 332u32, 2117100u32)?;
    emu.lw_no_count(12usize, 2usize, 336u32, 2117104u32)?;
    emu.lw_no_count(13usize, 2usize, 340u32, 2117108u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2117112u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2117116u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2117120u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2117124u32)?;
    emu.adi_no_count(10usize, 2usize, 488u32, 2117128u32);
    emu.adi_no_count(11usize, 2usize, 552u32, 2117132u32);
    emu.adi_no_count(12usize, 2usize, 520u32, 2117136u32);
    emu.apc_no_count(1usize, 2117136u32, 20480u32, 2117140u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117144u32;
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
#[inline(always)]
pub fn block_0x00204e18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 360u32, 2117148u32);
    emu.adi_no_count(11usize, 2usize, 456u32, 2117152u32);
    emu.adi_no_count(12usize, 2usize, 488u32, 2117156u32);
    emu.apc_no_count(1usize, 2117156u32, 20480u32, 2117160u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117164u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966196u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204e2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 520u32, 2117168u32);
    emu.adi_no_count(11usize, 2usize, 360u32, 2117172u32);
    emu.apc_no_count(1usize, 2117172u32, 49152u32, 2117176u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117180u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967060u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00204e3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 376u32, 2117184u32)?;
    emu.lw_no_count(11usize, 2usize, 380u32, 2117188u32)?;
    emu.lw_no_count(12usize, 2usize, 384u32, 2117192u32)?;
    emu.lw_no_count(13usize, 2usize, 388u32, 2117196u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2117200u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2117204u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2117208u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2117212u32)?;
    emu.lw_no_count(10usize, 2usize, 360u32, 2117216u32)?;
    emu.lw_no_count(11usize, 2usize, 364u32, 2117220u32)?;
    emu.lw_no_count(12usize, 2usize, 368u32, 2117224u32)?;
    emu.lw_no_count(13usize, 2usize, 372u32, 2117228u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2117232u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2117236u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2117240u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2117244u32)?;
    emu.adi_no_count(10usize, 2usize, 392u32, 2117248u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2117252u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2117256u32);
    emu.apc_no_count(1usize, 2117256u32, 20480u32, 2117260u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117264u32;
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
#[inline(always)]
pub fn block_0x00204e90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 520u32, 2117268u32);
    emu.adi_no_count(11usize, 2usize, 8u32, 2117272u32);
    emu.apc_no_count(1usize, 2117272u32, 49152u32, 2117276u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117280u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966960u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00204ea0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 24u32, 2117284u32)?;
    emu.lw_no_count(11usize, 2usize, 28u32, 2117288u32)?;
    emu.lw_no_count(12usize, 2usize, 32u32, 2117292u32)?;
    emu.lw_no_count(13usize, 2usize, 36u32, 2117296u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2117300u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2117304u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2117308u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2117312u32)?;
    emu.lw_no_count(10usize, 2usize, 8u32, 2117316u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2117320u32)?;
    emu.lw_no_count(12usize, 2usize, 16u32, 2117324u32)?;
    emu.lw_no_count(13usize, 2usize, 20u32, 2117328u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2117332u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2117336u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2117340u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2117344u32)?;
    emu.adi_no_count(10usize, 2usize, 488u32, 2117348u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2117352u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2117356u32);
    emu.apc_no_count(1usize, 2117356u32, 20480u32, 2117360u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117364u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965272u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204ef4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 424u32, 2117368u32);
    emu.adi_no_count(11usize, 2usize, 488u32, 2117372u32);
    emu.adi_no_count(12usize, 2usize, 328u32, 2117376u32);
    emu.apc_no_count(1usize, 2117376u32, 20480u32, 2117380u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117384u32;
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
pub fn block_0x00204f08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 312u32, 2117388u32)?;
    emu.lw_no_count(11usize, 2usize, 316u32, 2117392u32)?;
    emu.lw_no_count(12usize, 2usize, 320u32, 2117396u32)?;
    emu.lw_no_count(13usize, 2usize, 324u32, 2117400u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2117404u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2117408u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2117412u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2117416u32)?;
    emu.lw_no_count(10usize, 2usize, 296u32, 2117420u32)?;
    emu.lw_no_count(11usize, 2usize, 300u32, 2117424u32)?;
    emu.lw_no_count(12usize, 2usize, 304u32, 2117428u32)?;
    emu.lw_no_count(13usize, 2usize, 308u32, 2117432u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2117436u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2117440u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2117444u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2117448u32)?;
    emu.lw_no_count(10usize, 2usize, 120u32, 2117452u32)?;
    emu.lw_no_count(11usize, 2usize, 124u32, 2117456u32)?;
    emu.lw_no_count(12usize, 2usize, 128u32, 2117460u32)?;
    emu.lw_no_count(13usize, 2usize, 132u32, 2117464u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2117468u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2117472u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2117476u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2117480u32)?;
    emu.lw_no_count(10usize, 2usize, 104u32, 2117484u32)?;
    emu.lw_no_count(11usize, 2usize, 108u32, 2117488u32)?;
    emu.lw_no_count(12usize, 2usize, 112u32, 2117492u32)?;
    emu.lw_no_count(13usize, 2usize, 116u32, 2117496u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2117500u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2117504u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2117508u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2117512u32)?;
    emu.adi_no_count(10usize, 2usize, 456u32, 2117516u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2117520u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2117524u32);
    emu.apc_no_count(1usize, 2117524u32, 20480u32, 2117528u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117532u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966396u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00204f9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 152u32, 2117536u32)?;
    emu.lw_no_count(11usize, 2usize, 156u32, 2117540u32)?;
    emu.lw_no_count(12usize, 2usize, 160u32, 2117544u32)?;
    emu.lw_no_count(13usize, 2usize, 164u32, 2117548u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2117552u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2117556u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2117560u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2117564u32)?;
    emu.lw_no_count(10usize, 2usize, 136u32, 2117568u32)?;
    emu.lw_no_count(11usize, 2usize, 140u32, 2117572u32)?;
    emu.lw_no_count(12usize, 2usize, 144u32, 2117576u32)?;
    emu.lw_no_count(13usize, 2usize, 148u32, 2117580u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2117584u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2117588u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2117592u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2117596u32)?;
    emu.lw_no_count(10usize, 2usize, 408u32, 2117600u32)?;
    emu.lw_no_count(11usize, 2usize, 412u32, 2117604u32)?;
    emu.lw_no_count(12usize, 2usize, 416u32, 2117608u32)?;
    emu.lw_no_count(13usize, 2usize, 420u32, 2117612u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2117616u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2117620u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2117624u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2117628u32)?;
    emu.lw_no_count(10usize, 2usize, 392u32, 2117632u32)?;
    emu.lw_no_count(11usize, 2usize, 396u32, 2117636u32)?;
    emu.lw_no_count(12usize, 2usize, 400u32, 2117640u32)?;
    emu.lw_no_count(13usize, 2usize, 404u32, 2117644u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2117648u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2117652u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2117656u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2117660u32)?;
    emu.adi_no_count(10usize, 2usize, 488u32, 2117664u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2117668u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2117672u32);
    emu.apc_no_count(1usize, 2117672u32, 20480u32, 2117676u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117680u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966248u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205030(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 2usize, 456u32, 2117684u32);
    emu.adi_no_count(12usize, 2usize, 488u32, 2117688u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2117692u32);
    emu.apc_no_count(1usize, 2117692u32, 20480u32, 2117696u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117700u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965660u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00205044(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 280u32, 2117704u32)?;
    emu.lw_no_count(11usize, 2usize, 284u32, 2117708u32)?;
    emu.lw_no_count(12usize, 2usize, 288u32, 2117712u32)?;
    emu.lw_no_count(13usize, 2usize, 292u32, 2117716u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2117720u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2117724u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2117728u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2117732u32)?;
    emu.lw_no_count(10usize, 2usize, 264u32, 2117736u32)?;
    emu.lw_no_count(11usize, 2usize, 268u32, 2117740u32)?;
    emu.lw_no_count(12usize, 2usize, 272u32, 2117744u32)?;
    emu.lw_no_count(13usize, 2usize, 276u32, 2117748u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2117752u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2117756u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2117760u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2117764u32)?;
    emu.adi_no_count(10usize, 2usize, 488u32, 2117768u32);
    emu.adi_no_count(11usize, 2usize, 296u32, 2117772u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2117776u32);
    emu.apc_no_count(1usize, 2117776u32, 20480u32, 2117780u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117784u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966144u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00205098(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 440u32, 2117788u32)?;
    emu.lw_no_count(11usize, 2usize, 444u32, 2117792u32)?;
    emu.lw_no_count(12usize, 2usize, 448u32, 2117796u32)?;
    emu.lw_no_count(13usize, 2usize, 452u32, 2117800u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2117804u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2117808u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2117812u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2117816u32)?;
    emu.lw_no_count(10usize, 2usize, 424u32, 2117820u32)?;
    emu.lw_no_count(11usize, 2usize, 428u32, 2117824u32)?;
    emu.lw_no_count(12usize, 2usize, 432u32, 2117828u32)?;
    emu.lw_no_count(13usize, 2usize, 436u32, 2117832u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2117836u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2117840u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2117844u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2117848u32)?;
    emu.adi_no_count(10usize, 2usize, 520u32, 2117852u32);
    emu.adi_no_count(11usize, 2usize, 552u32, 2117856u32);
    emu.adi_no_count(12usize, 2usize, 392u32, 2117860u32);
    emu.apc_no_count(1usize, 2117860u32, 20480u32, 2117864u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117868u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966060u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002050ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 32u32, 2117872u32);
    emu.adi_no_count(11usize, 2usize, 488u32, 2117876u32);
    emu.adi_no_count(12usize, 2usize, 520u32, 2117880u32);
    emu.apc_no_count(1usize, 2117880u32, 16384u32, 2117884u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117888u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1548u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205100(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 520u32, 2117892u32);
    emu.adi_no_count(11usize, 2usize, 264u32, 2117896u32);
    emu.adi_no_count(12usize, 2usize, 136u32, 2117900u32);
    emu.apc_no_count(1usize, 2117900u32, 20480u32, 2117904u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117908u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966020u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205114(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 552u32, 2117912u32);
    emu.adi_no_count(11usize, 2usize, 104u32, 2117916u32);
    emu.adi_no_count(12usize, 2usize, 424u32, 2117920u32);
    emu.apc_no_count(1usize, 2117920u32, 20480u32, 2117924u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117928u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966000u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205128(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 64u32, 2117932u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2117936u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2117940u32);
    emu.apc_no_count(1usize, 2117940u32, 16384u32, 2117944u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117948u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1488u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020513c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 620u32, 2117952u32)?;
    emu.lw_no_count(8usize, 2usize, 616u32, 2117956u32)?;
    emu.lw_no_count(9usize, 2usize, 612u32, 2117960u32)?;
    emu.lw_no_count(18usize, 2usize, 608u32, 2117964u32)?;
    emu.lw_no_count(19usize, 2usize, 604u32, 2117968u32)?;
    emu.lw_no_count(20usize, 2usize, 600u32, 2117972u32)?;
    emu.lw_no_count(21usize, 2usize, 596u32, 2117976u32)?;
    emu.lw_no_count(22usize, 2usize, 592u32, 2117980u32)?;
    emu.lw_no_count(23usize, 2usize, 588u32, 2117984u32)?;
    emu.lw_no_count(24usize, 2usize, 584u32, 2117988u32)?;
    emu.adi_no_count(2usize, 2usize, 624u32, 2117992u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117996u32;
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
pub fn block_0x0020516c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294966576u32, 2118000u32);
    emu.sw_no_count(1usize, 2usize, 716u32, 2118004u32)?;
    emu.sw_no_count(8usize, 2usize, 712u32, 2118008u32)?;
    emu.sw_no_count(9usize, 2usize, 708u32, 2118012u32)?;
    emu.sw_no_count(18usize, 2usize, 704u32, 2118016u32)?;
    emu.sw_no_count(19usize, 2usize, 700u32, 2118020u32)?;
    emu.sw_no_count(20usize, 2usize, 696u32, 2118024u32)?;
    emu.sw_no_count(21usize, 2usize, 692u32, 2118028u32)?;
    emu.sw_no_count(22usize, 2usize, 688u32, 2118032u32)?;
    emu.sw_no_count(23usize, 2usize, 684u32, 2118036u32)?;
    emu.sw_no_count(24usize, 2usize, 680u32, 2118040u32)?;
    emu.sw_no_count(25usize, 2usize, 676u32, 2118044u32)?;
    emu.adi_no_count(9usize, 11usize, 0u32, 2118048u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2118052u32);
    emu.adi_no_count(10usize, 2usize, 4u32, 2118056u32);
    emu.apc_no_count(1usize, 2118056u32, 49152u32, 2118060u32);
    emu.add_memory_rw_events(17usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118064u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966596u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002051b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 9usize, 32u32, 2118068u32);
    emu.adi_no_count(10usize, 2usize, 36u32, 2118072u32);
    emu.apc_no_count(1usize, 2118072u32, 49152u32, 2118076u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118080u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966580u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002051c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 9usize, 64u32, 2118084u32);
    emu.adi_no_count(10usize, 2usize, 68u32, 2118088u32);
    emu.apc_no_count(1usize, 2118088u32, 49152u32, 2118092u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118096u32;
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
pub fn block_0x002051d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 16u32, 2118100u32)?;
    emu.lw_no_count(11usize, 9usize, 20u32, 2118104u32)?;
    emu.lw_no_count(12usize, 9usize, 24u32, 2118108u32)?;
    emu.lw_no_count(13usize, 9usize, 28u32, 2118112u32)?;
    emu.sw_no_count(10usize, 2usize, 628u32, 2118116u32)?;
    emu.sw_no_count(11usize, 2usize, 632u32, 2118120u32)?;
    emu.sw_no_count(12usize, 2usize, 636u32, 2118124u32)?;
    emu.sw_no_count(13usize, 2usize, 640u32, 2118128u32)?;
    emu.lw_no_count(10usize, 9usize, 0u32, 2118132u32)?;
    emu.lw_no_count(11usize, 9usize, 4u32, 2118136u32)?;
    emu.lw_no_count(12usize, 9usize, 8u32, 2118140u32)?;
    emu.lw_no_count(13usize, 9usize, 12u32, 2118144u32)?;
    emu.sw_no_count(10usize, 2usize, 612u32, 2118148u32)?;
    emu.sw_no_count(11usize, 2usize, 616u32, 2118152u32)?;
    emu.sw_no_count(12usize, 2usize, 620u32, 2118156u32)?;
    emu.sw_no_count(13usize, 2usize, 624u32, 2118160u32)?;
    emu.lw_no_count(10usize, 9usize, 48u32, 2118164u32)?;
    emu.lw_no_count(11usize, 9usize, 52u32, 2118168u32)?;
    emu.lw_no_count(12usize, 9usize, 56u32, 2118172u32)?;
    emu.lw_no_count(13usize, 9usize, 60u32, 2118176u32)?;
    emu.sw_no_count(10usize, 2usize, 660u32, 2118180u32)?;
    emu.sw_no_count(11usize, 2usize, 664u32, 2118184u32)?;
    emu.sw_no_count(12usize, 2usize, 668u32, 2118188u32)?;
    emu.sw_no_count(13usize, 2usize, 672u32, 2118192u32)?;
    emu.lw_no_count(10usize, 9usize, 32u32, 2118196u32)?;
    emu.lw_no_count(11usize, 9usize, 36u32, 2118200u32)?;
    emu.lw_no_count(12usize, 9usize, 40u32, 2118204u32)?;
    emu.lw_no_count(13usize, 9usize, 44u32, 2118208u32)?;
    emu.sw_no_count(10usize, 2usize, 644u32, 2118212u32)?;
    emu.sw_no_count(11usize, 2usize, 648u32, 2118216u32)?;
    emu.sw_no_count(12usize, 2usize, 652u32, 2118220u32)?;
    emu.sw_no_count(13usize, 2usize, 656u32, 2118224u32)?;
    emu.adi_no_count(10usize, 2usize, 580u32, 2118228u32);
    emu.adi_no_count(11usize, 2usize, 612u32, 2118232u32);
    emu.adi_no_count(12usize, 2usize, 644u32, 2118236u32);
    emu.apc_no_count(1usize, 2118236u32, 20480u32, 2118240u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118244u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965684u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205264(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 100u32, 2118248u32);
    emu.adi_no_count(11usize, 2usize, 580u32, 2118252u32);
    emu.apc_no_count(1usize, 2118252u32, 49152u32, 2118256u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118260u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965980u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00205274(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 16u32, 2118264u32)?;
    emu.lw_no_count(11usize, 9usize, 20u32, 2118268u32)?;
    emu.lw_no_count(12usize, 9usize, 24u32, 2118272u32)?;
    emu.lw_no_count(13usize, 9usize, 28u32, 2118276u32)?;
    emu.sw_no_count(10usize, 2usize, 628u32, 2118280u32)?;
    emu.sw_no_count(11usize, 2usize, 632u32, 2118284u32)?;
    emu.sw_no_count(12usize, 2usize, 636u32, 2118288u32)?;
    emu.sw_no_count(13usize, 2usize, 640u32, 2118292u32)?;
    emu.lw_no_count(10usize, 9usize, 0u32, 2118296u32)?;
    emu.lw_no_count(11usize, 9usize, 4u32, 2118300u32)?;
    emu.lw_no_count(12usize, 9usize, 8u32, 2118304u32)?;
    emu.lw_no_count(13usize, 9usize, 12u32, 2118308u32)?;
    emu.sw_no_count(10usize, 2usize, 612u32, 2118312u32)?;
    emu.sw_no_count(11usize, 2usize, 616u32, 2118316u32)?;
    emu.sw_no_count(12usize, 2usize, 620u32, 2118320u32)?;
    emu.sw_no_count(13usize, 2usize, 624u32, 2118324u32)?;
    emu.lw_no_count(10usize, 9usize, 80u32, 2118328u32)?;
    emu.lw_no_count(11usize, 9usize, 84u32, 2118332u32)?;
    emu.lw_no_count(12usize, 9usize, 88u32, 2118336u32)?;
    emu.lw_no_count(13usize, 9usize, 92u32, 2118340u32)?;
    emu.sw_no_count(10usize, 2usize, 660u32, 2118344u32)?;
    emu.sw_no_count(11usize, 2usize, 664u32, 2118348u32)?;
    emu.sw_no_count(12usize, 2usize, 668u32, 2118352u32)?;
    emu.sw_no_count(13usize, 2usize, 672u32, 2118356u32)?;
    emu.lw_no_count(10usize, 9usize, 64u32, 2118360u32)?;
    emu.lw_no_count(11usize, 9usize, 68u32, 2118364u32)?;
    emu.lw_no_count(12usize, 9usize, 72u32, 2118368u32)?;
    emu.lw_no_count(13usize, 9usize, 76u32, 2118372u32)?;
    emu.sw_no_count(10usize, 2usize, 644u32, 2118376u32)?;
    emu.sw_no_count(11usize, 2usize, 648u32, 2118380u32)?;
    emu.sw_no_count(12usize, 2usize, 652u32, 2118384u32)?;
    emu.sw_no_count(13usize, 2usize, 656u32, 2118388u32)?;
    emu.adi_no_count(10usize, 2usize, 580u32, 2118392u32);
    emu.adi_no_count(11usize, 2usize, 612u32, 2118396u32);
    emu.adi_no_count(12usize, 2usize, 644u32, 2118400u32);
    emu.apc_no_count(1usize, 2118400u32, 20480u32, 2118404u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118408u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965520u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205308(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 132u32, 2118412u32);
    emu.adi_no_count(11usize, 2usize, 580u32, 2118416u32);
    emu.apc_no_count(1usize, 2118416u32, 49152u32, 2118420u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118424u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965816u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00205318(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 45u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 84u32, 2118428u32)?;
    emu.lw_no_count(11usize, 2usize, 88u32, 2118432u32)?;
    emu.lw_no_count(12usize, 2usize, 92u32, 2118436u32)?;
    emu.lw_no_count(13usize, 2usize, 96u32, 2118440u32)?;
    emu.lw_no_count(14usize, 2usize, 68u32, 2118444u32)?;
    emu.lw_no_count(15usize, 2usize, 72u32, 2118448u32)?;
    emu.lw_no_count(16usize, 2usize, 76u32, 2118452u32)?;
    emu.lw_no_count(17usize, 2usize, 80u32, 2118456u32)?;
    let a = 0u32.wrapping_add(3694133248u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2118460u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(75976704u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2118464u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(3852607488u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2118468u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4146147328u32);
    emu.write_reg_no_count(28usize, a);
    emu.pc = 2118472u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2901409792u32);
    emu.write_reg_no_count(29usize, a);
    emu.pc = 2118476u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2021928960u32);
    emu.write_reg_no_count(30usize, a);
    emu.pc = 2118480u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 2usize, 628u32, 2118484u32)?;
    emu.sw_no_count(11usize, 2usize, 632u32, 2118488u32)?;
    emu.sw_no_count(12usize, 2usize, 636u32, 2118492u32)?;
    emu.sw_no_count(13usize, 2usize, 640u32, 2118496u32)?;
    let a = 0u32.wrapping_add(3634159616u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2118500u32;
    emu.update_insn_clock();
    emu.sw_no_count(14usize, 2usize, 612u32, 2118504u32)?;
    emu.sw_no_count(15usize, 2usize, 616u32, 2118508u32)?;
    emu.sw_no_count(16usize, 2usize, 620u32, 2118512u32)?;
    emu.sw_no_count(17usize, 2usize, 624u32, 2118516u32)?;
    let a = 0u32.wrapping_add(700760064u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2118520u32;
    emu.update_insn_clock();
    emu.adi_no_count(20usize, 5usize, 1565u32, 2118524u32);
    emu.adi_no_count(22usize, 6usize, 4294965300u32, 2118528u32);
    emu.adi_no_count(24usize, 7usize, 171u32, 2118532u32);
    emu.adi_no_count(25usize, 28usize, 4294966998u32, 2118536u32);
    emu.adi_no_count(18usize, 29usize, 1485u32, 2118540u32);
    emu.adi_no_count(19usize, 30usize, 144u32, 2118544u32);
    emu.adi_no_count(21usize, 10usize, 4294967138u32, 2118548u32);
    emu.adi_no_count(23usize, 11usize, 4294966751u32, 2118552u32);
    emu.sw_no_count(25usize, 2usize, 660u32, 2118556u32)?;
    emu.sw_no_count(24usize, 2usize, 664u32, 2118560u32)?;
    emu.sw_no_count(22usize, 2usize, 668u32, 2118564u32)?;
    emu.sw_no_count(20usize, 2usize, 672u32, 2118568u32)?;
    emu.sw_no_count(23usize, 2usize, 644u32, 2118572u32)?;
    emu.sw_no_count(21usize, 2usize, 648u32, 2118576u32)?;
    emu.sw_no_count(19usize, 2usize, 652u32, 2118580u32)?;
    emu.sw_no_count(18usize, 2usize, 656u32, 2118584u32)?;
    emu.adi_no_count(10usize, 2usize, 580u32, 2118588u32);
    emu.adi_no_count(11usize, 2usize, 644u32, 2118592u32);
    emu.adi_no_count(12usize, 2usize, 612u32, 2118596u32);
    emu.apc_no_count(1usize, 2118596u32, 20480u32, 2118600u32);
    emu.add_memory_rw_events(45usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118604u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965324u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002053cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 148u32, 2118608u32)?;
    emu.lw_no_count(11usize, 2usize, 152u32, 2118612u32)?;
    emu.lw_no_count(12usize, 2usize, 156u32, 2118616u32)?;
    emu.lw_no_count(13usize, 2usize, 160u32, 2118620u32)?;
    emu.sw_no_count(10usize, 2usize, 660u32, 2118624u32)?;
    emu.sw_no_count(11usize, 2usize, 664u32, 2118628u32)?;
    emu.sw_no_count(12usize, 2usize, 668u32, 2118632u32)?;
    emu.sw_no_count(13usize, 2usize, 672u32, 2118636u32)?;
    emu.lw_no_count(10usize, 2usize, 132u32, 2118640u32)?;
    emu.lw_no_count(11usize, 2usize, 136u32, 2118644u32)?;
    emu.lw_no_count(12usize, 2usize, 140u32, 2118648u32)?;
    emu.lw_no_count(13usize, 2usize, 144u32, 2118652u32)?;
    emu.sw_no_count(10usize, 2usize, 644u32, 2118656u32)?;
    emu.sw_no_count(11usize, 2usize, 648u32, 2118660u32)?;
    emu.sw_no_count(12usize, 2usize, 652u32, 2118664u32)?;
    emu.sw_no_count(13usize, 2usize, 656u32, 2118668u32)?;
    emu.adi_no_count(10usize, 2usize, 164u32, 2118672u32);
    emu.adi_no_count(11usize, 2usize, 580u32, 2118676u32);
    emu.adi_no_count(12usize, 2usize, 644u32, 2118680u32);
    emu.apc_no_count(1usize, 2118680u32, 16384u32, 2118684u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118688u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1472u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205420(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 612u32, 2118692u32);
    emu.adi_no_count(11usize, 2usize, 164u32, 2118696u32);
    emu.apc_no_count(1usize, 2118696u32, 49152u32, 2118700u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118704u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965536u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00205430(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 180u32, 2118708u32)?;
    emu.lw_no_count(11usize, 2usize, 184u32, 2118712u32)?;
    emu.lw_no_count(12usize, 2usize, 188u32, 2118716u32)?;
    emu.lw_no_count(13usize, 2usize, 192u32, 2118720u32)?;
    emu.sw_no_count(10usize, 2usize, 660u32, 2118724u32)?;
    emu.sw_no_count(11usize, 2usize, 664u32, 2118728u32)?;
    emu.sw_no_count(12usize, 2usize, 668u32, 2118732u32)?;
    emu.sw_no_count(13usize, 2usize, 672u32, 2118736u32)?;
    emu.lw_no_count(10usize, 2usize, 164u32, 2118740u32)?;
    emu.lw_no_count(11usize, 2usize, 168u32, 2118744u32)?;
    emu.lw_no_count(12usize, 2usize, 172u32, 2118748u32)?;
    emu.lw_no_count(13usize, 2usize, 176u32, 2118752u32)?;
    emu.sw_no_count(10usize, 2usize, 644u32, 2118756u32)?;
    emu.sw_no_count(11usize, 2usize, 648u32, 2118760u32)?;
    emu.sw_no_count(12usize, 2usize, 652u32, 2118764u32)?;
    emu.sw_no_count(13usize, 2usize, 656u32, 2118768u32)?;
    emu.adi_no_count(10usize, 2usize, 196u32, 2118772u32);
    emu.adi_no_count(11usize, 2usize, 612u32, 2118776u32);
    emu.adi_no_count(12usize, 2usize, 644u32, 2118780u32);
    emu.apc_no_count(1usize, 2118780u32, 16384u32, 2118784u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118788u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(648u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00205484(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 52u32, 2118792u32)?;
    emu.lw_no_count(11usize, 2usize, 56u32, 2118796u32)?;
    emu.lw_no_count(12usize, 2usize, 60u32, 2118800u32)?;
    emu.lw_no_count(13usize, 2usize, 64u32, 2118804u32)?;
    emu.sw_no_count(10usize, 2usize, 628u32, 2118808u32)?;
    emu.sw_no_count(11usize, 2usize, 632u32, 2118812u32)?;
    emu.sw_no_count(12usize, 2usize, 636u32, 2118816u32)?;
    emu.sw_no_count(13usize, 2usize, 640u32, 2118820u32)?;
    emu.lw_no_count(10usize, 2usize, 36u32, 2118824u32)?;
    emu.lw_no_count(11usize, 2usize, 40u32, 2118828u32)?;
    emu.lw_no_count(12usize, 2usize, 44u32, 2118832u32)?;
    emu.lw_no_count(13usize, 2usize, 48u32, 2118836u32)?;
    emu.sw_no_count(10usize, 2usize, 612u32, 2118840u32)?;
    emu.sw_no_count(11usize, 2usize, 616u32, 2118844u32)?;
    emu.sw_no_count(12usize, 2usize, 620u32, 2118848u32)?;
    emu.sw_no_count(13usize, 2usize, 624u32, 2118852u32)?;
    emu.lw_no_count(10usize, 2usize, 212u32, 2118856u32)?;
    emu.lw_no_count(11usize, 2usize, 216u32, 2118860u32)?;
    emu.lw_no_count(12usize, 2usize, 220u32, 2118864u32)?;
    emu.lw_no_count(13usize, 2usize, 224u32, 2118868u32)?;
    emu.sw_no_count(10usize, 2usize, 660u32, 2118872u32)?;
    emu.sw_no_count(11usize, 2usize, 664u32, 2118876u32)?;
    emu.sw_no_count(12usize, 2usize, 668u32, 2118880u32)?;
    emu.sw_no_count(13usize, 2usize, 672u32, 2118884u32)?;
    emu.lw_no_count(10usize, 2usize, 196u32, 2118888u32)?;
    emu.lw_no_count(11usize, 2usize, 200u32, 2118892u32)?;
    emu.lw_no_count(12usize, 2usize, 204u32, 2118896u32)?;
    emu.lw_no_count(13usize, 2usize, 208u32, 2118900u32)?;
    emu.sw_no_count(10usize, 2usize, 644u32, 2118904u32)?;
    emu.sw_no_count(11usize, 2usize, 648u32, 2118908u32)?;
    emu.sw_no_count(12usize, 2usize, 652u32, 2118912u32)?;
    emu.sw_no_count(13usize, 2usize, 656u32, 2118916u32)?;
    emu.adi_no_count(10usize, 2usize, 228u32, 2118920u32);
    emu.adi_no_count(11usize, 2usize, 612u32, 2118924u32);
    emu.adi_no_count(12usize, 2usize, 644u32, 2118928u32);
    emu.apc_no_count(1usize, 2118928u32, 16384u32, 2118932u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118936u32;
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
pub fn block_0x00205518(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 52u32, 2118940u32)?;
    emu.lw_no_count(11usize, 2usize, 56u32, 2118944u32)?;
    emu.lw_no_count(12usize, 2usize, 60u32, 2118948u32)?;
    emu.lw_no_count(13usize, 2usize, 64u32, 2118952u32)?;
    emu.sw_no_count(10usize, 2usize, 660u32, 2118956u32)?;
    emu.sw_no_count(11usize, 2usize, 664u32, 2118960u32)?;
    emu.sw_no_count(12usize, 2usize, 668u32, 2118964u32)?;
    emu.sw_no_count(13usize, 2usize, 672u32, 2118968u32)?;
    emu.lw_no_count(10usize, 2usize, 36u32, 2118972u32)?;
    emu.lw_no_count(11usize, 2usize, 40u32, 2118976u32)?;
    emu.lw_no_count(12usize, 2usize, 44u32, 2118980u32)?;
    emu.lw_no_count(13usize, 2usize, 48u32, 2118984u32)?;
    emu.sw_no_count(10usize, 2usize, 644u32, 2118988u32)?;
    emu.sw_no_count(11usize, 2usize, 648u32, 2118992u32)?;
    emu.sw_no_count(12usize, 2usize, 652u32, 2118996u32)?;
    emu.sw_no_count(13usize, 2usize, 656u32, 2119000u32)?;
    emu.adi_no_count(10usize, 2usize, 260u32, 2119004u32);
    emu.adi_no_count(11usize, 2usize, 644u32, 2119008u32);
    emu.adi_no_count(12usize, 2usize, 196u32, 2119012u32);
    emu.apc_no_count(1usize, 2119012u32, 16384u32, 2119016u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119020u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(416u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020556c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 244u32, 2119024u32)?;
    emu.lw_no_count(11usize, 2usize, 248u32, 2119028u32)?;
    emu.lw_no_count(12usize, 2usize, 252u32, 2119032u32)?;
    emu.lw_no_count(13usize, 2usize, 256u32, 2119036u32)?;
    emu.sw_no_count(10usize, 2usize, 660u32, 2119040u32)?;
    emu.sw_no_count(11usize, 2usize, 664u32, 2119044u32)?;
    emu.sw_no_count(12usize, 2usize, 668u32, 2119048u32)?;
    emu.sw_no_count(13usize, 2usize, 672u32, 2119052u32)?;
    emu.lw_no_count(10usize, 2usize, 228u32, 2119056u32)?;
    emu.lw_no_count(11usize, 2usize, 232u32, 2119060u32)?;
    emu.lw_no_count(12usize, 2usize, 236u32, 2119064u32)?;
    emu.lw_no_count(13usize, 2usize, 240u32, 2119068u32)?;
    emu.sw_no_count(10usize, 2usize, 644u32, 2119072u32)?;
    emu.sw_no_count(11usize, 2usize, 648u32, 2119076u32)?;
    emu.sw_no_count(12usize, 2usize, 652u32, 2119080u32)?;
    emu.sw_no_count(13usize, 2usize, 656u32, 2119084u32)?;
    emu.adi_no_count(10usize, 2usize, 292u32, 2119088u32);
    emu.adi_no_count(11usize, 2usize, 260u32, 2119092u32);
    emu.adi_no_count(12usize, 2usize, 644u32, 2119096u32);
    emu.apc_no_count(1usize, 2119096u32, 16384u32, 2119100u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119104u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1624u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002055c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 324u32, 2119108u32);
    emu.adi_no_count(11usize, 2usize, 228u32, 2119112u32);
    emu.adi_no_count(12usize, 2usize, 100u32, 2119116u32);
    emu.apc_no_count(1usize, 2119116u32, 16384u32, 2119120u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119124u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1604u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002055d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 612u32, 2119128u32);
    emu.adi_no_count(11usize, 2usize, 68u32, 2119132u32);
    emu.apc_no_count(1usize, 2119132u32, 45056u32, 2119136u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119140u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1900u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002055e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 84u32, 2119144u32)?;
    emu.lw_no_count(11usize, 2usize, 88u32, 2119148u32)?;
    emu.lw_no_count(12usize, 2usize, 92u32, 2119152u32)?;
    emu.lw_no_count(13usize, 2usize, 96u32, 2119156u32)?;
    emu.sw_no_count(10usize, 2usize, 660u32, 2119160u32)?;
    emu.sw_no_count(11usize, 2usize, 664u32, 2119164u32)?;
    emu.sw_no_count(12usize, 2usize, 668u32, 2119168u32)?;
    emu.sw_no_count(13usize, 2usize, 672u32, 2119172u32)?;
    emu.lw_no_count(10usize, 2usize, 68u32, 2119176u32)?;
    emu.lw_no_count(11usize, 2usize, 72u32, 2119180u32)?;
    emu.lw_no_count(12usize, 2usize, 76u32, 2119184u32)?;
    emu.lw_no_count(13usize, 2usize, 80u32, 2119188u32)?;
    emu.sw_no_count(10usize, 2usize, 644u32, 2119192u32)?;
    emu.sw_no_count(11usize, 2usize, 648u32, 2119196u32)?;
    emu.sw_no_count(12usize, 2usize, 652u32, 2119200u32)?;
    emu.sw_no_count(13usize, 2usize, 656u32, 2119204u32)?;
    emu.adi_no_count(10usize, 2usize, 356u32, 2119208u32);
    emu.adi_no_count(11usize, 2usize, 612u32, 2119212u32);
    emu.adi_no_count(12usize, 2usize, 644u32, 2119216u32);
    emu.apc_no_count(1usize, 2119216u32, 16384u32, 2119220u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119224u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(212u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00205638(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(25usize, 2usize, 660u32, 2119228u32)?;
    emu.sw_no_count(24usize, 2usize, 664u32, 2119232u32)?;
    emu.sw_no_count(22usize, 2usize, 668u32, 2119236u32)?;
    emu.sw_no_count(20usize, 2usize, 672u32, 2119240u32)?;
    emu.sw_no_count(23usize, 2usize, 644u32, 2119244u32)?;
    emu.sw_no_count(21usize, 2usize, 648u32, 2119248u32)?;
    emu.sw_no_count(19usize, 2usize, 652u32, 2119252u32)?;
    emu.sw_no_count(18usize, 2usize, 656u32, 2119256u32)?;
    emu.adi_no_count(10usize, 2usize, 420u32, 2119260u32);
    emu.adi_no_count(11usize, 2usize, 644u32, 2119264u32);
    emu.adi_no_count(12usize, 2usize, 132u32, 2119268u32);
    emu.apc_no_count(1usize, 2119268u32, 16384u32, 2119272u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119276u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1452u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0020566c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 20u32, 2119280u32)?;
    emu.lw_no_count(11usize, 2usize, 24u32, 2119284u32)?;
    emu.lw_no_count(12usize, 2usize, 28u32, 2119288u32)?;
    emu.lw_no_count(13usize, 2usize, 32u32, 2119292u32)?;
    emu.sw_no_count(10usize, 2usize, 628u32, 2119296u32)?;
    emu.sw_no_count(11usize, 2usize, 632u32, 2119300u32)?;
    emu.sw_no_count(12usize, 2usize, 636u32, 2119304u32)?;
    emu.sw_no_count(13usize, 2usize, 640u32, 2119308u32)?;
    emu.lw_no_count(10usize, 2usize, 4u32, 2119312u32)?;
    emu.lw_no_count(11usize, 2usize, 8u32, 2119316u32)?;
    emu.lw_no_count(12usize, 2usize, 12u32, 2119320u32)?;
    emu.lw_no_count(13usize, 2usize, 16u32, 2119324u32)?;
    emu.sw_no_count(10usize, 2usize, 612u32, 2119328u32)?;
    emu.sw_no_count(11usize, 2usize, 616u32, 2119332u32)?;
    emu.sw_no_count(12usize, 2usize, 620u32, 2119336u32)?;
    emu.sw_no_count(13usize, 2usize, 624u32, 2119340u32)?;
    emu.lw_no_count(10usize, 2usize, 372u32, 2119344u32)?;
    emu.lw_no_count(11usize, 2usize, 376u32, 2119348u32)?;
    emu.lw_no_count(12usize, 2usize, 380u32, 2119352u32)?;
    emu.lw_no_count(13usize, 2usize, 384u32, 2119356u32)?;
    emu.sw_no_count(10usize, 2usize, 660u32, 2119360u32)?;
    emu.sw_no_count(11usize, 2usize, 664u32, 2119364u32)?;
    emu.sw_no_count(12usize, 2usize, 668u32, 2119368u32)?;
    emu.sw_no_count(13usize, 2usize, 672u32, 2119372u32)?;
    emu.lw_no_count(10usize, 2usize, 356u32, 2119376u32)?;
    emu.lw_no_count(11usize, 2usize, 360u32, 2119380u32)?;
    emu.lw_no_count(12usize, 2usize, 364u32, 2119384u32)?;
    emu.lw_no_count(13usize, 2usize, 368u32, 2119388u32)?;
    emu.sw_no_count(10usize, 2usize, 644u32, 2119392u32)?;
    emu.sw_no_count(11usize, 2usize, 648u32, 2119396u32)?;
    emu.sw_no_count(12usize, 2usize, 652u32, 2119400u32)?;
    emu.sw_no_count(13usize, 2usize, 656u32, 2119404u32)?;
    emu.adi_no_count(10usize, 2usize, 580u32, 2119408u32);
    emu.adi_no_count(11usize, 2usize, 644u32, 2119412u32);
    emu.adi_no_count(12usize, 2usize, 612u32, 2119416u32);
    emu.apc_no_count(1usize, 2119416u32, 16384u32, 2119420u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119424u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(12u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205700(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 388u32, 2119428u32);
    emu.adi_no_count(11usize, 2usize, 420u32, 2119432u32);
    emu.adi_no_count(12usize, 2usize, 580u32, 2119436u32);
    emu.apc_no_count(1usize, 2119436u32, 16384u32, 2119440u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119444u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(716u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205714(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 612u32, 2119448u32);
    emu.adi_no_count(11usize, 2usize, 388u32, 2119452u32);
    emu.apc_no_count(1usize, 2119452u32, 45056u32, 2119456u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119460u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1580u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00205724(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 404u32, 2119464u32)?;
    emu.lw_no_count(11usize, 2usize, 408u32, 2119468u32)?;
    emu.lw_no_count(12usize, 2usize, 412u32, 2119472u32)?;
    emu.lw_no_count(13usize, 2usize, 416u32, 2119476u32)?;
    emu.sw_no_count(10usize, 2usize, 660u32, 2119480u32)?;
    emu.sw_no_count(11usize, 2usize, 664u32, 2119484u32)?;
    emu.sw_no_count(12usize, 2usize, 668u32, 2119488u32)?;
    emu.sw_no_count(13usize, 2usize, 672u32, 2119492u32)?;
    emu.lw_no_count(10usize, 2usize, 388u32, 2119496u32)?;
    emu.lw_no_count(11usize, 2usize, 392u32, 2119500u32)?;
    emu.lw_no_count(12usize, 2usize, 396u32, 2119504u32)?;
    emu.lw_no_count(13usize, 2usize, 400u32, 2119508u32)?;
    emu.sw_no_count(10usize, 2usize, 644u32, 2119512u32)?;
    emu.sw_no_count(11usize, 2usize, 648u32, 2119516u32)?;
    emu.sw_no_count(12usize, 2usize, 652u32, 2119520u32)?;
    emu.sw_no_count(13usize, 2usize, 656u32, 2119524u32)?;
    emu.adi_no_count(10usize, 2usize, 452u32, 2119528u32);
    emu.adi_no_count(11usize, 2usize, 612u32, 2119532u32);
    emu.adi_no_count(12usize, 2usize, 644u32, 2119536u32);
    emu.apc_no_count(1usize, 2119536u32, 16384u32, 2119540u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119544u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967188u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205778(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 612u32, 2119548u32);
    emu.adi_no_count(11usize, 2usize, 4u32, 2119552u32);
    emu.apc_no_count(1usize, 2119552u32, 45056u32, 2119556u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119560u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1480u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00205788(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 20u32, 2119564u32)?;
    emu.lw_no_count(11usize, 2usize, 24u32, 2119568u32)?;
    emu.lw_no_count(12usize, 2usize, 28u32, 2119572u32)?;
    emu.lw_no_count(13usize, 2usize, 32u32, 2119576u32)?;
    emu.sw_no_count(10usize, 2usize, 660u32, 2119580u32)?;
    emu.sw_no_count(11usize, 2usize, 664u32, 2119584u32)?;
    emu.sw_no_count(12usize, 2usize, 668u32, 2119588u32)?;
    emu.sw_no_count(13usize, 2usize, 672u32, 2119592u32)?;
    emu.lw_no_count(10usize, 2usize, 4u32, 2119596u32)?;
    emu.lw_no_count(11usize, 2usize, 8u32, 2119600u32)?;
    emu.lw_no_count(12usize, 2usize, 12u32, 2119604u32)?;
    emu.lw_no_count(13usize, 2usize, 16u32, 2119608u32)?;
    emu.sw_no_count(10usize, 2usize, 644u32, 2119612u32)?;
    emu.sw_no_count(11usize, 2usize, 648u32, 2119616u32)?;
    emu.sw_no_count(12usize, 2usize, 652u32, 2119620u32)?;
    emu.sw_no_count(13usize, 2usize, 656u32, 2119624u32)?;
    emu.adi_no_count(10usize, 2usize, 580u32, 2119628u32);
    emu.adi_no_count(11usize, 2usize, 612u32, 2119632u32);
    emu.adi_no_count(12usize, 2usize, 644u32, 2119636u32);
    emu.apc_no_count(1usize, 2119636u32, 16384u32, 2119640u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119644u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967088u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002057dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 484u32, 2119648u32);
    emu.adi_no_count(11usize, 2usize, 580u32, 2119652u32);
    emu.adi_no_count(12usize, 2usize, 356u32, 2119656u32);
    emu.apc_no_count(1usize, 2119656u32, 16384u32, 2119660u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119664u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(496u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002057f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 468u32, 2119668u32)?;
    emu.lw_no_count(11usize, 2usize, 472u32, 2119672u32)?;
    emu.lw_no_count(12usize, 2usize, 476u32, 2119676u32)?;
    emu.lw_no_count(13usize, 2usize, 480u32, 2119680u32)?;
    emu.sw_no_count(10usize, 2usize, 660u32, 2119684u32)?;
    emu.sw_no_count(11usize, 2usize, 664u32, 2119688u32)?;
    emu.sw_no_count(12usize, 2usize, 668u32, 2119692u32)?;
    emu.sw_no_count(13usize, 2usize, 672u32, 2119696u32)?;
    emu.lw_no_count(10usize, 2usize, 452u32, 2119700u32)?;
    emu.lw_no_count(11usize, 2usize, 456u32, 2119704u32)?;
    emu.lw_no_count(12usize, 2usize, 460u32, 2119708u32)?;
    emu.lw_no_count(13usize, 2usize, 464u32, 2119712u32)?;
    emu.sw_no_count(10usize, 2usize, 644u32, 2119716u32)?;
    emu.sw_no_count(11usize, 2usize, 648u32, 2119720u32)?;
    emu.sw_no_count(12usize, 2usize, 652u32, 2119724u32)?;
    emu.sw_no_count(13usize, 2usize, 656u32, 2119728u32)?;
    emu.adi_no_count(10usize, 2usize, 612u32, 2119732u32);
    emu.adi_no_count(11usize, 2usize, 484u32, 2119736u32);
    emu.adi_no_count(12usize, 2usize, 644u32, 2119740u32);
    emu.apc_no_count(1usize, 2119740u32, 16384u32, 2119744u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119748u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(980u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205844(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 516u32, 2119752u32);
    emu.adi_no_count(11usize, 2usize, 292u32, 2119756u32);
    emu.adi_no_count(12usize, 2usize, 612u32, 2119760u32);
    emu.apc_no_count(1usize, 2119760u32, 16384u32, 2119764u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119768u32;
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
#[inline(never)]
pub fn block_0x00205858(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 48u32, 2119772u32)?;
    emu.lw_no_count(11usize, 9usize, 52u32, 2119776u32)?;
    emu.lw_no_count(12usize, 9usize, 56u32, 2119780u32)?;
    emu.lw_no_count(13usize, 9usize, 60u32, 2119784u32)?;
    emu.sw_no_count(10usize, 2usize, 628u32, 2119788u32)?;
    emu.sw_no_count(11usize, 2usize, 632u32, 2119792u32)?;
    emu.sw_no_count(12usize, 2usize, 636u32, 2119796u32)?;
    emu.sw_no_count(13usize, 2usize, 640u32, 2119800u32)?;
    emu.lw_no_count(10usize, 9usize, 32u32, 2119804u32)?;
    emu.lw_no_count(11usize, 9usize, 36u32, 2119808u32)?;
    emu.lw_no_count(12usize, 9usize, 40u32, 2119812u32)?;
    emu.lw_no_count(13usize, 9usize, 44u32, 2119816u32)?;
    emu.sw_no_count(10usize, 2usize, 612u32, 2119820u32)?;
    emu.sw_no_count(11usize, 2usize, 616u32, 2119824u32)?;
    emu.sw_no_count(12usize, 2usize, 620u32, 2119828u32)?;
    emu.sw_no_count(13usize, 2usize, 624u32, 2119832u32)?;
    emu.lw_no_count(10usize, 9usize, 80u32, 2119836u32)?;
    emu.lw_no_count(11usize, 9usize, 84u32, 2119840u32)?;
    emu.lw_no_count(12usize, 9usize, 88u32, 2119844u32)?;
    emu.lw_no_count(13usize, 9usize, 92u32, 2119848u32)?;
    emu.sw_no_count(10usize, 2usize, 660u32, 2119852u32)?;
    emu.sw_no_count(11usize, 2usize, 664u32, 2119856u32)?;
    emu.sw_no_count(12usize, 2usize, 668u32, 2119860u32)?;
    emu.sw_no_count(13usize, 2usize, 672u32, 2119864u32)?;
    emu.lw_no_count(10usize, 9usize, 64u32, 2119868u32)?;
    emu.lw_no_count(11usize, 9usize, 68u32, 2119872u32)?;
    emu.lw_no_count(12usize, 9usize, 72u32, 2119876u32)?;
    emu.lw_no_count(13usize, 9usize, 76u32, 2119880u32)?;
    emu.sw_no_count(10usize, 2usize, 644u32, 2119884u32)?;
    emu.sw_no_count(11usize, 2usize, 648u32, 2119888u32)?;
    emu.sw_no_count(12usize, 2usize, 652u32, 2119892u32)?;
    emu.sw_no_count(13usize, 2usize, 656u32, 2119896u32)?;
    emu.adi_no_count(10usize, 2usize, 580u32, 2119900u32);
    emu.adi_no_count(11usize, 2usize, 612u32, 2119904u32);
    emu.adi_no_count(12usize, 2usize, 644u32, 2119908u32);
    emu.apc_no_count(1usize, 2119908u32, 16384u32, 2119912u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119916u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(812u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002058ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 548u32, 2119920u32);
    emu.adi_no_count(11usize, 2usize, 580u32, 2119924u32);
    emu.apc_no_count(1usize, 2119924u32, 45056u32, 2119928u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119932u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1108u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002058fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 564u32, 2119936u32)?;
    emu.lw_no_count(11usize, 2usize, 568u32, 2119940u32)?;
    emu.lw_no_count(12usize, 2usize, 572u32, 2119944u32)?;
    emu.lw_no_count(13usize, 2usize, 576u32, 2119948u32)?;
    emu.sw_no_count(10usize, 2usize, 660u32, 2119952u32)?;
    emu.sw_no_count(11usize, 2usize, 664u32, 2119956u32)?;
    emu.sw_no_count(12usize, 2usize, 668u32, 2119960u32)?;
    emu.sw_no_count(13usize, 2usize, 672u32, 2119964u32)?;
    emu.lw_no_count(10usize, 2usize, 548u32, 2119968u32)?;
    emu.lw_no_count(11usize, 2usize, 552u32, 2119972u32)?;
    emu.lw_no_count(12usize, 2usize, 556u32, 2119976u32)?;
    emu.lw_no_count(13usize, 2usize, 560u32, 2119980u32)?;
    emu.sw_no_count(10usize, 2usize, 644u32, 2119984u32)?;
    emu.sw_no_count(11usize, 2usize, 648u32, 2119988u32)?;
    emu.sw_no_count(12usize, 2usize, 652u32, 2119992u32)?;
    emu.sw_no_count(13usize, 2usize, 656u32, 2119996u32)?;
    emu.adi_no_count(10usize, 2usize, 612u32, 2120000u32);
    emu.adi_no_count(11usize, 2usize, 452u32, 2120004u32);
    emu.adi_no_count(12usize, 2usize, 644u32, 2120008u32);
    emu.apc_no_count(1usize, 2120008u32, 16384u32, 2120012u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120016u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(712u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205950(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 2usize, 324u32, 2120020u32);
    emu.adi_no_count(12usize, 2usize, 612u32, 2120024u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2120028u32);
    emu.apc_no_count(1usize, 2120028u32, 16384u32, 2120032u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120036u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(124u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205964(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 644u32, 2120040u32);
    emu.adi_no_count(11usize, 2usize, 548u32, 2120044u32);
    emu.adi_no_count(12usize, 2usize, 36u32, 2120048u32);
    emu.apc_no_count(1usize, 2120048u32, 16384u32, 2120052u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120056u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(672u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205978(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 612u32, 2120060u32);
    emu.adi_no_count(11usize, 2usize, 644u32, 2120064u32);
    emu.apc_no_count(1usize, 2120064u32, 45056u32, 2120068u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120072u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(968u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205988(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 64u32, 2120076u32);
    emu.adi_no_count(11usize, 2usize, 612u32, 2120080u32);
    emu.apc_no_count(1usize, 2120080u32, 45056u32, 2120084u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120088u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(952u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00205998(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 29u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 532u32, 2120092u32)?;
    emu.lw_no_count(11usize, 2usize, 536u32, 2120096u32)?;
    emu.lw_no_count(12usize, 2usize, 540u32, 2120100u32)?;
    emu.lw_no_count(13usize, 2usize, 544u32, 2120104u32)?;
    emu.sw_no_count(10usize, 8usize, 48u32, 2120108u32)?;
    emu.sw_no_count(11usize, 8usize, 52u32, 2120112u32)?;
    emu.sw_no_count(12usize, 8usize, 56u32, 2120116u32)?;
    emu.sw_no_count(13usize, 8usize, 60u32, 2120120u32)?;
    emu.lw_no_count(10usize, 2usize, 516u32, 2120124u32)?;
    emu.lw_no_count(11usize, 2usize, 520u32, 2120128u32)?;
    emu.lw_no_count(12usize, 2usize, 524u32, 2120132u32)?;
    emu.lw_no_count(13usize, 2usize, 528u32, 2120136u32)?;
    emu.sw_no_count(10usize, 8usize, 32u32, 2120140u32)?;
    emu.sw_no_count(11usize, 8usize, 36u32, 2120144u32)?;
    emu.sw_no_count(12usize, 8usize, 40u32, 2120148u32)?;
    emu.sw_no_count(13usize, 8usize, 44u32, 2120152u32)?;
    emu.lw_no_count(1usize, 2usize, 716u32, 2120156u32)?;
    emu.lw_no_count(8usize, 2usize, 712u32, 2120160u32)?;
    emu.lw_no_count(9usize, 2usize, 708u32, 2120164u32)?;
    emu.lw_no_count(18usize, 2usize, 704u32, 2120168u32)?;
    emu.lw_no_count(19usize, 2usize, 700u32, 2120172u32)?;
    emu.lw_no_count(20usize, 2usize, 696u32, 2120176u32)?;
    emu.lw_no_count(21usize, 2usize, 692u32, 2120180u32)?;
    emu.lw_no_count(22usize, 2usize, 688u32, 2120184u32)?;
    emu.lw_no_count(23usize, 2usize, 684u32, 2120188u32)?;
    emu.lw_no_count(24usize, 2usize, 680u32, 2120192u32)?;
    emu.lw_no_count(25usize, 2usize, 676u32, 2120196u32)?;
    emu.adi_no_count(2usize, 2usize, 720u32, 2120200u32);
    emu.add_memory_rw_events(29usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120204u32;
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
pub fn block_0x00205a0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2120208u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2120212u32)?;
    emu.lw_no_count(12usize, 10usize, 0u32, 2120216u32)?;
    emu.lw_no_count(14usize, 12usize, 0u32, 2120220u32)?;
    emu.adi_no_count(13usize, 0usize, 1u32, 2120224u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2120228u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2120276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205a54));
    } else {
        emu.pc = 2120232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205a28));
    }
}
#[inline(always)]
pub fn block_0x00205a28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2120236u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(13usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2120284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205a5c));
    } else {
        emu.pc = 2120240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205a30));
    }
}
#[inline(always)]
pub fn block_0x00205a30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 2u32, 2120244u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2120316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205a7c));
    } else {
        emu.pc = 2120248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205a38));
    }
}
#[inline(always)]
pub fn block_0x00205a38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2120252u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 227u32, 2120256u32);
    emu.adi_no_count(12usize, 0usize, 12u32, 2120260u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2120264u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2120268u32);
    emu.apc_no_count(6usize, 2120268u32, 98304u32, 2120272u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2120276u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965916u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205a54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 14usize, 4294967294u32, 2120280u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(13usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2120240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205a30));
    } else {
        emu.pc = 2120284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205a5c));
    }
}
#[inline(always)]
pub fn block_0x00205a5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2120380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205abc));
    } else {
        emu.pc = 2120288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205a60));
    }
}
#[inline(always)]
pub fn block_0x00205a60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2120292u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 201u32, 2120296u32);
    emu.adi_no_count(12usize, 0usize, 26u32, 2120300u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2120304u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2120308u32);
    emu.apc_no_count(6usize, 2120308u32, 98304u32, 2120312u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2120316u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965876u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00205a7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4u32, 2120320u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2120324u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2120328u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 256u32, 2120332u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2120336u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 266u32, 2120340u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2120344u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 240u32, 2120348u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2120352u32);
    emu.adi_no_count(14usize, 0usize, 3u32, 2120356u32);
    emu.adi_no_count(15usize, 2usize, 8u32, 2120360u32);
    emu.apc_no_count(1usize, 2120360u32, 98304u32, 2120364u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120368u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965924u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205ab0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2120372u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2120376u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120380u32;
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
pub fn block_0x00205abc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(12usize, 2usize, 4u32, 2120384u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2120388u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965820u32, 2120392u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2120396u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 136u32, 2120400u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2120404u32);
    emu.adi_no_count(13usize, 2usize, 4u32, 2120408u32);
    emu.apc_no_count(1usize, 2120408u32, 98304u32, 2120412u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120416u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966528u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205ae0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2120420u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2120424u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120428u32;
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
pub fn block_0x00205aec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2120432u32)?;
    emu.lw_no_count(13usize, 12usize, 0u32, 2120436u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2120440u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2120504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205b38));
    } else {
        emu.pc = 2120444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205afc));
    }
}
#[inline]
pub fn block_0x00205afc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2120448u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2120452u32)?;
    emu.adi_no_count(12usize, 12usize, 4u32, 2120456u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2120460u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2120464u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965792u32, 2120468u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2120472u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 744u32, 2120476u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2120480u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2120484u32);
    emu.apc_no_count(1usize, 2120484u32, 98304u32, 2120488u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120492u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966452u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205b2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2120496u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2120500u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120504u32;
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
pub fn block_0x00205b38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2120508u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965808u32, 2120512u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2120516u32);
    emu.apc_no_count(6usize, 2120516u32, 98304u32, 2120520u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2120524u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965668u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205b4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2120528u32)?;
    emu.apc_no_count(6usize, 2120528u32, 4096u32, 2120532u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2120536u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1828u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205b58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2120540u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2120544u32)?;
    emu.lw_no_count(12usize, 10usize, 0u32, 2120548u32)?;
    emu.lw_no_count(13usize, 12usize, 0u32, 2120552u32)?;
    emu.adi_no_count(14usize, 0usize, 1u32, 2120556u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2120560u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(14usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2120620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205bac));
    } else {
        emu.pc = 2120564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205b74));
    }
}
#[inline(always)]
pub fn block_0x00205b74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2120656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205bd0));
    } else {
        emu.pc = 2120568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205b78));
    }
}
#[inline]
pub fn block_0x00205b78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4u32, 2120572u32);
    emu.sw_no_count(12usize, 2usize, 4u32, 2120576u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2120580u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965820u32, 2120584u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2120588u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 136u32, 2120592u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2120596u32);
    emu.adi_no_count(13usize, 2usize, 4u32, 2120600u32);
    emu.apc_no_count(1usize, 2120600u32, 98304u32, 2120604u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120608u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966336u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205ba0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2120612u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2120616u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120620u32;
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
pub fn block_0x00205bac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 2u32, 2120624u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2120684u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205bec));
    } else {
        emu.pc = 2120628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205bb4));
    }
}
#[inline(always)]
pub fn block_0x00205bb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2120632u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 269u32, 2120636u32);
    emu.adi_no_count(12usize, 0usize, 19u32, 2120640u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2120644u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2120648u32);
    emu.apc_no_count(6usize, 2120648u32, 98304u32, 2120652u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2120656u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965536u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205bd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2120660u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 227u32, 2120664u32);
    emu.adi_no_count(12usize, 0usize, 12u32, 2120668u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2120672u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2120676u32);
    emu.apc_no_count(6usize, 2120676u32, 98304u32, 2120680u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2120684u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965508u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00205bec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4u32, 2120688u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2120692u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2120696u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 304u32, 2120700u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2120704u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 288u32, 2120708u32);
    emu.adi_no_count(12usize, 0usize, 9u32, 2120712u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2120716u32);
    emu.apc_no_count(1usize, 2120716u32, 98304u32, 2120720u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120724u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966220u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205c14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2120728u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2120732u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120736u32;
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
pub fn block_0x00205c20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2120740u32)?;
    emu.lbu_no_count(13usize, 12usize, 0u32, 2120744u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2120748u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2120812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205c6c));
    } else {
        emu.pc = 2120752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205c30));
    }
}
#[inline]
pub fn block_0x00205c30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2120756u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2120760u32)?;
    emu.adi_no_count(12usize, 12usize, 1u32, 2120764u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2120768u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2120772u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965792u32, 2120776u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2120780u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 60u32, 2120784u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2120788u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2120792u32);
    emu.apc_no_count(1usize, 2120792u32, 98304u32, 2120796u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120800u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966144u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205c60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2120804u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2120808u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120812u32;
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
pub fn block_0x00205c6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2120816u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965808u32, 2120820u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2120824u32);
    emu.apc_no_count(6usize, 2120824u32, 98304u32, 2120828u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2120832u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965360u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
