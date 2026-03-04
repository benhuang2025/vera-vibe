pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2111660u32;
pub const PC_MAX: u32 = 2131480u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 118usize] = [
        block_0x002038ac,
        block_0x0020390c,
        block_0x00203910,
        block_0x00203c1c,
        block_0x00207aa8,
        block_0x00207aac,
        block_0x00207b0c,
        block_0x00207b30,
        block_0x00207b34,
        block_0x00207b4c,
        block_0x00207b54,
        block_0x00207b68,
        block_0x00207b7c,
        block_0x00207b80,
        block_0x00207b98,
        block_0x00207bb4,
        block_0x00207bcc,
        block_0x00207bd4,
        block_0x00207bd8,
        block_0x00207c04,
        block_0x00207c28,
        block_0x00207c2c,
        block_0x00207c44,
        block_0x00207c4c,
        block_0x00207c60,
        block_0x00207c74,
        block_0x00207c78,
        block_0x00207ca0,
        block_0x00207cbc,
        block_0x00207ce4,
        block_0x00207cec,
        block_0x00207cf0,
        block_0x00207d1c,
        block_0x00207d20,
        block_0x00207d50,
        block_0x00207d64,
        block_0x00207d6c,
        block_0x00207da4,
        block_0x00207db4,
        block_0x00207dc4,
        block_0x00207dcc,
        block_0x00207de8,
        block_0x00207dec,
        block_0x00207e0c,
        block_0x00207e20,
        block_0x00207e44,
        block_0x00207e9c,
        block_0x00207ea8,
        block_0x00207ecc,
        block_0x00207ef0,
        block_0x00207f14,
        block_0x00207f6c,
        block_0x00207f78,
        block_0x00207f9c,
        block_0x00207fc0,
        block_0x00207fd8,
        block_0x00207ffc,
        block_0x0020803c,
        block_0x00208060,
        block_0x00208074,
        block_0x0020809c,
        block_0x002080c0,
        block_0x002080cc,
        block_0x002080ec,
        block_0x00208148,
        block_0x00208154,
        block_0x00208178,
        block_0x0020819c,
        block_0x002081c0,
        block_0x002081e4,
        block_0x002081fc,
        block_0x00208208,
        block_0x00208228,
        block_0x00208270,
        block_0x00208280,
        block_0x002082d8,
        block_0x002082dc,
        block_0x002082f0,
        block_0x002082f8,
        block_0x00208310,
        block_0x00208314,
        block_0x00208328,
        block_0x00208348,
        block_0x0020834c,
        block_0x00208358,
        block_0x00208368,
        block_0x00208378,
        block_0x0020837c,
        block_0x00208390,
        block_0x002083a4,
        block_0x002083a8,
        block_0x002083d4,
        block_0x002083f8,
        block_0x00208414,
        block_0x00208448,
        block_0x00208454,
        block_0x0020845c,
        block_0x0020847c,
        block_0x00208484,
        block_0x002084b4,
        block_0x002084f0,
        block_0x00208518,
        block_0x0020851c,
        block_0x00208540,
        block_0x00208544,
        block_0x00208554,
        block_0x0020856c,
        block_0x00208580,
        block_0x00208588,
        block_0x00208598,
        block_0x002085a0,
        block_0x002085ac,
        block_0x002085d4,
        block_0x002085e4,
        block_0x002085ec,
        block_0x002085f4,
        block_0x00208604,
        block_0x00208618,
    ];
    #[repr(C)]
    struct Run {
        start_word: u32,
        len: u16,
        fn_offset: u16,
    }
    const RUNS: [Run; 101usize] = [
        Run {
            start_word: 0u32,
            len: 1i32 as u16,
            fn_offset: 0usize as u16,
        },
        Run {
            start_word: 24u32,
            len: 2i32 as u16,
            fn_offset: 1usize as u16,
        },
        Run {
            start_word: 220u32,
            len: 1i32 as u16,
            fn_offset: 3usize as u16,
        },
        Run {
            start_word: 4223u32,
            len: 2i32 as u16,
            fn_offset: 4usize as u16,
        },
        Run {
            start_word: 4248u32,
            len: 1i32 as u16,
            fn_offset: 6usize as u16,
        },
        Run {
            start_word: 4257u32,
            len: 2i32 as u16,
            fn_offset: 7usize as u16,
        },
        Run {
            start_word: 4264u32,
            len: 1i32 as u16,
            fn_offset: 9usize as u16,
        },
        Run {
            start_word: 4266u32,
            len: 1i32 as u16,
            fn_offset: 10usize as u16,
        },
        Run {
            start_word: 4271u32,
            len: 1i32 as u16,
            fn_offset: 11usize as u16,
        },
        Run {
            start_word: 4276u32,
            len: 2i32 as u16,
            fn_offset: 12usize as u16,
        },
        Run {
            start_word: 4283u32,
            len: 1i32 as u16,
            fn_offset: 14usize as u16,
        },
        Run {
            start_word: 4290u32,
            len: 1i32 as u16,
            fn_offset: 15usize as u16,
        },
        Run {
            start_word: 4296u32,
            len: 1i32 as u16,
            fn_offset: 16usize as u16,
        },
        Run {
            start_word: 4298u32,
            len: 2i32 as u16,
            fn_offset: 17usize as u16,
        },
        Run {
            start_word: 4310u32,
            len: 1i32 as u16,
            fn_offset: 19usize as u16,
        },
        Run {
            start_word: 4319u32,
            len: 2i32 as u16,
            fn_offset: 20usize as u16,
        },
        Run {
            start_word: 4326u32,
            len: 1i32 as u16,
            fn_offset: 22usize as u16,
        },
        Run {
            start_word: 4328u32,
            len: 1i32 as u16,
            fn_offset: 23usize as u16,
        },
        Run {
            start_word: 4333u32,
            len: 1i32 as u16,
            fn_offset: 24usize as u16,
        },
        Run {
            start_word: 4338u32,
            len: 2i32 as u16,
            fn_offset: 25usize as u16,
        },
        Run {
            start_word: 4349u32,
            len: 1i32 as u16,
            fn_offset: 27usize as u16,
        },
        Run {
            start_word: 4356u32,
            len: 1i32 as u16,
            fn_offset: 28usize as u16,
        },
        Run {
            start_word: 4366u32,
            len: 1i32 as u16,
            fn_offset: 29usize as u16,
        },
        Run {
            start_word: 4368u32,
            len: 2i32 as u16,
            fn_offset: 30usize as u16,
        },
        Run {
            start_word: 4380u32,
            len: 2i32 as u16,
            fn_offset: 32usize as u16,
        },
        Run {
            start_word: 4393u32,
            len: 1i32 as u16,
            fn_offset: 34usize as u16,
        },
        Run {
            start_word: 4398u32,
            len: 1i32 as u16,
            fn_offset: 35usize as u16,
        },
        Run {
            start_word: 4400u32,
            len: 1i32 as u16,
            fn_offset: 36usize as u16,
        },
        Run {
            start_word: 4414u32,
            len: 1i32 as u16,
            fn_offset: 37usize as u16,
        },
        Run {
            start_word: 4418u32,
            len: 1i32 as u16,
            fn_offset: 38usize as u16,
        },
        Run {
            start_word: 4422u32,
            len: 1i32 as u16,
            fn_offset: 39usize as u16,
        },
        Run {
            start_word: 4424u32,
            len: 1i32 as u16,
            fn_offset: 40usize as u16,
        },
        Run {
            start_word: 4431u32,
            len: 2i32 as u16,
            fn_offset: 41usize as u16,
        },
        Run {
            start_word: 4440u32,
            len: 1i32 as u16,
            fn_offset: 43usize as u16,
        },
        Run {
            start_word: 4445u32,
            len: 1i32 as u16,
            fn_offset: 44usize as u16,
        },
        Run {
            start_word: 4454u32,
            len: 1i32 as u16,
            fn_offset: 45usize as u16,
        },
        Run {
            start_word: 4476u32,
            len: 1i32 as u16,
            fn_offset: 46usize as u16,
        },
        Run {
            start_word: 4479u32,
            len: 1i32 as u16,
            fn_offset: 47usize as u16,
        },
        Run {
            start_word: 4488u32,
            len: 1i32 as u16,
            fn_offset: 48usize as u16,
        },
        Run {
            start_word: 4497u32,
            len: 1i32 as u16,
            fn_offset: 49usize as u16,
        },
        Run {
            start_word: 4506u32,
            len: 1i32 as u16,
            fn_offset: 50usize as u16,
        },
        Run {
            start_word: 4528u32,
            len: 1i32 as u16,
            fn_offset: 51usize as u16,
        },
        Run {
            start_word: 4531u32,
            len: 1i32 as u16,
            fn_offset: 52usize as u16,
        },
        Run {
            start_word: 4540u32,
            len: 1i32 as u16,
            fn_offset: 53usize as u16,
        },
        Run {
            start_word: 4549u32,
            len: 1i32 as u16,
            fn_offset: 54usize as u16,
        },
        Run {
            start_word: 4555u32,
            len: 1i32 as u16,
            fn_offset: 55usize as u16,
        },
        Run {
            start_word: 4564u32,
            len: 1i32 as u16,
            fn_offset: 56usize as u16,
        },
        Run {
            start_word: 4580u32,
            len: 1i32 as u16,
            fn_offset: 57usize as u16,
        },
        Run {
            start_word: 4589u32,
            len: 1i32 as u16,
            fn_offset: 58usize as u16,
        },
        Run {
            start_word: 4594u32,
            len: 1i32 as u16,
            fn_offset: 59usize as u16,
        },
        Run {
            start_word: 4604u32,
            len: 1i32 as u16,
            fn_offset: 60usize as u16,
        },
        Run {
            start_word: 4613u32,
            len: 1i32 as u16,
            fn_offset: 61usize as u16,
        },
        Run {
            start_word: 4616u32,
            len: 1i32 as u16,
            fn_offset: 62usize as u16,
        },
        Run {
            start_word: 4624u32,
            len: 1i32 as u16,
            fn_offset: 63usize as u16,
        },
        Run {
            start_word: 4647u32,
            len: 1i32 as u16,
            fn_offset: 64usize as u16,
        },
        Run {
            start_word: 4650u32,
            len: 1i32 as u16,
            fn_offset: 65usize as u16,
        },
        Run {
            start_word: 4659u32,
            len: 1i32 as u16,
            fn_offset: 66usize as u16,
        },
        Run {
            start_word: 4668u32,
            len: 1i32 as u16,
            fn_offset: 67usize as u16,
        },
        Run {
            start_word: 4677u32,
            len: 1i32 as u16,
            fn_offset: 68usize as u16,
        },
        Run {
            start_word: 4686u32,
            len: 1i32 as u16,
            fn_offset: 69usize as u16,
        },
        Run {
            start_word: 4692u32,
            len: 1i32 as u16,
            fn_offset: 70usize as u16,
        },
        Run {
            start_word: 4695u32,
            len: 1i32 as u16,
            fn_offset: 71usize as u16,
        },
        Run {
            start_word: 4703u32,
            len: 1i32 as u16,
            fn_offset: 72usize as u16,
        },
        Run {
            start_word: 4721u32,
            len: 1i32 as u16,
            fn_offset: 73usize as u16,
        },
        Run {
            start_word: 4725u32,
            len: 1i32 as u16,
            fn_offset: 74usize as u16,
        },
        Run {
            start_word: 4747u32,
            len: 2i32 as u16,
            fn_offset: 75usize as u16,
        },
        Run {
            start_word: 4753u32,
            len: 1i32 as u16,
            fn_offset: 77usize as u16,
        },
        Run {
            start_word: 4755u32,
            len: 1i32 as u16,
            fn_offset: 78usize as u16,
        },
        Run {
            start_word: 4761u32,
            len: 2i32 as u16,
            fn_offset: 79usize as u16,
        },
        Run {
            start_word: 4767u32,
            len: 1i32 as u16,
            fn_offset: 81usize as u16,
        },
        Run {
            start_word: 4775u32,
            len: 2i32 as u16,
            fn_offset: 82usize as u16,
        },
        Run {
            start_word: 4779u32,
            len: 1i32 as u16,
            fn_offset: 84usize as u16,
        },
        Run {
            start_word: 4783u32,
            len: 1i32 as u16,
            fn_offset: 85usize as u16,
        },
        Run {
            start_word: 4787u32,
            len: 2i32 as u16,
            fn_offset: 86usize as u16,
        },
        Run {
            start_word: 4793u32,
            len: 1i32 as u16,
            fn_offset: 88usize as u16,
        },
        Run {
            start_word: 4798u32,
            len: 2i32 as u16,
            fn_offset: 89usize as u16,
        },
        Run {
            start_word: 4810u32,
            len: 1i32 as u16,
            fn_offset: 91usize as u16,
        },
        Run {
            start_word: 4819u32,
            len: 1i32 as u16,
            fn_offset: 92usize as u16,
        },
        Run {
            start_word: 4826u32,
            len: 1i32 as u16,
            fn_offset: 93usize as u16,
        },
        Run {
            start_word: 4839u32,
            len: 1i32 as u16,
            fn_offset: 94usize as u16,
        },
        Run {
            start_word: 4842u32,
            len: 1i32 as u16,
            fn_offset: 95usize as u16,
        },
        Run {
            start_word: 4844u32,
            len: 1i32 as u16,
            fn_offset: 96usize as u16,
        },
        Run {
            start_word: 4852u32,
            len: 1i32 as u16,
            fn_offset: 97usize as u16,
        },
        Run {
            start_word: 4854u32,
            len: 1i32 as u16,
            fn_offset: 98usize as u16,
        },
        Run {
            start_word: 4866u32,
            len: 1i32 as u16,
            fn_offset: 99usize as u16,
        },
        Run {
            start_word: 4881u32,
            len: 1i32 as u16,
            fn_offset: 100usize as u16,
        },
        Run {
            start_word: 4891u32,
            len: 2i32 as u16,
            fn_offset: 101usize as u16,
        },
        Run {
            start_word: 4901u32,
            len: 2i32 as u16,
            fn_offset: 103usize as u16,
        },
        Run {
            start_word: 4906u32,
            len: 1i32 as u16,
            fn_offset: 105usize as u16,
        },
        Run {
            start_word: 4912u32,
            len: 1i32 as u16,
            fn_offset: 106usize as u16,
        },
        Run {
            start_word: 4917u32,
            len: 1i32 as u16,
            fn_offset: 107usize as u16,
        },
        Run {
            start_word: 4919u32,
            len: 1i32 as u16,
            fn_offset: 108usize as u16,
        },
        Run {
            start_word: 4923u32,
            len: 1i32 as u16,
            fn_offset: 109usize as u16,
        },
        Run {
            start_word: 4925u32,
            len: 1i32 as u16,
            fn_offset: 110usize as u16,
        },
        Run {
            start_word: 4928u32,
            len: 1i32 as u16,
            fn_offset: 111usize as u16,
        },
        Run {
            start_word: 4938u32,
            len: 1i32 as u16,
            fn_offset: 112usize as u16,
        },
        Run {
            start_word: 4942u32,
            len: 1i32 as u16,
            fn_offset: 113usize as u16,
        },
        Run {
            start_word: 4944u32,
            len: 1i32 as u16,
            fn_offset: 114usize as u16,
        },
        Run {
            start_word: 4946u32,
            len: 1i32 as u16,
            fn_offset: 115usize as u16,
        },
        Run {
            start_word: 4950u32,
            len: 1i32 as u16,
            fn_offset: 116usize as u16,
        },
        Run {
            start_word: 4955u32,
            len: 1i32 as u16,
            fn_offset: 117usize as u16,
        },
    ];
    if pc < 2111660u32 || pc > 2131480u32 {
        return None;
    }
    let word_offset = ((pc - 2111660u32) >> 2) as u32;
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
pub fn block_0x002038ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294966720u32, 2111664u32);
    emu.sw_no_count(1usize, 2usize, 572u32, 2111668u32)?;
    emu.sw_no_count(8usize, 2usize, 568u32, 2111672u32)?;
    emu.sw_no_count(9usize, 2usize, 564u32, 2111676u32)?;
    emu.sw_no_count(18usize, 2usize, 560u32, 2111680u32)?;
    emu.sw_no_count(19usize, 2usize, 556u32, 2111684u32)?;
    emu.sw_no_count(20usize, 2usize, 552u32, 2111688u32)?;
    emu.sw_no_count(21usize, 2usize, 548u32, 2111692u32)?;
    emu.sw_no_count(22usize, 2usize, 544u32, 2111696u32)?;
    emu.sw_no_count(23usize, 2usize, 540u32, 2111700u32)?;
    emu.sw_no_count(24usize, 2usize, 536u32, 2111704u32)?;
    emu.sw_no_count(25usize, 2usize, 532u32, 2111708u32)?;
    emu.sw_no_count(26usize, 2usize, 528u32, 2111712u32)?;
    emu.sw_no_count(27usize, 2usize, 524u32, 2111716u32)?;
    emu.lw_no_count(23usize, 10usize, 0u32, 2111720u32)?;
    emu.lw_no_count(22usize, 10usize, 4u32, 2111724u32)?;
    emu.lw_no_count(21usize, 10usize, 8u32, 2111728u32)?;
    emu.lw_no_count(19usize, 10usize, 12u32, 2111732u32)?;
    emu.lw_no_count(30usize, 10usize, 16u32, 2111736u32)?;
    emu.lw_no_count(20usize, 10usize, 20u32, 2111740u32)?;
    emu.lw_no_count(26usize, 10usize, 24u32, 2111744u32)?;
    emu.sw_no_count(10usize, 2usize, 8u32, 2111748u32)?;
    emu.lw_no_count(9usize, 10usize, 28u32, 2111752u32)?;
    emu.add_memory_rw_events(23usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2111760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203910));
    } else {
        emu.pc = 2111756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020390c));
    }
}
#[inline(always)]
pub fn block_0x0020390c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2111760u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2128556u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207aac));
}
#[inline(never)]
pub fn block_0x00203910(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 195u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 6u32, 2111764u32);
    let a = 0u32.wrapping_add(607223808u32);
    emu.write_reg_no_count(28usize, a);
    emu.pc = 2111768u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1426882560u32);
    emu.write_reg_no_count(29usize, a);
    emu.pc = 2111772u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1925079040u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2111776u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2162077696u32);
    emu.write_reg_no_count(31usize, a);
    emu.pc = 2111780u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2614886400u32);
    emu.write_reg_no_count(8usize, a);
    emu.pc = 2111784u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(3248222208u32);
    emu.write_reg_no_count(18usize, a);
    emu.pc = 2111788u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(3835392000u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2111792u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4022222848u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2111796u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(264347648u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2111800u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(604807168u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2111804u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(770256896u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2111808u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1249148928u32);
    emu.write_reg_no_count(24usize, a);
    emu.pc = 2111812u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1555083264u32);
    emu.write_reg_no_count(25usize, a);
    emu.pc = 2111816u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1996066816u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2111820u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2554220544u32);
    emu.write_reg_no_count(27usize, a);
    emu.pc = 2111824u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2821832704u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2111828u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2952994816u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2111832u32;
    emu.update_insn_clock();
    emu.adr_no_count(12usize, 11usize, 12usize, 2111836u32);
    emu.sw_no_count(12usize, 2usize, 268u32, 2111840u32)?;
    let a = 0u32.wrapping_add(1116352512u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2111844u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967192u32, 2111848u32);
    emu.sw_no_count(12usize, 2usize, 264u32, 2111852u32)?;
    let a = 0u32.wrapping_add(1899446272u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2111856u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1169u32, 2111860u32);
    emu.sw_no_count(12usize, 2usize, 260u32, 2111864u32)?;
    let a = 0u32.wrapping_add(3049324544u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2111868u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966223u32, 2111872u32);
    emu.sw_no_count(12usize, 2usize, 256u32, 2111876u32)?;
    let a = 0u32.wrapping_add(3921010688u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2111880u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966181u32, 2111884u32);
    emu.sw_no_count(12usize, 2usize, 252u32, 2111888u32)?;
    let a = 0u32.wrapping_add(961986560u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2111892u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 603u32, 2111896u32);
    emu.sw_no_count(12usize, 2usize, 248u32, 2111900u32)?;
    let a = 0u32.wrapping_add(1508970496u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2111904u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 497u32, 2111908u32);
    emu.sw_no_count(12usize, 2usize, 244u32, 2111912u32)?;
    let a = 0u32.wrapping_add(2453635072u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2111916u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 676u32, 2111920u32);
    emu.sw_no_count(12usize, 2usize, 240u32, 2111924u32)?;
    let a = 0u32.wrapping_add(2870763520u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2111928u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966997u32, 2111932u32);
    emu.sw_no_count(12usize, 2usize, 236u32, 2111936u32)?;
    let a = 0u32.wrapping_add(3624382464u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2111940u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965912u32, 2111944u32);
    emu.sw_no_count(12usize, 2usize, 232u32, 2111948u32)?;
    let a = 0u32.wrapping_add(310599680u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2111952u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966017u32, 2111956u32);
    emu.sw_no_count(12usize, 2usize, 228u32, 2111960u32)?;
    emu.adi_no_count(12usize, 28usize, 1470u32, 2111964u32);
    emu.sw_no_count(12usize, 2usize, 224u32, 2111968u32)?;
    emu.adi_no_count(12usize, 29usize, 4294966723u32, 2111972u32);
    emu.sw_no_count(12usize, 2usize, 220u32, 2111976u32)?;
    emu.adi_no_count(12usize, 7usize, 4294966644u32, 2111980u32);
    emu.sw_no_count(12usize, 2usize, 216u32, 2111984u32)?;
    emu.adi_no_count(12usize, 31usize, 510u32, 2111988u32);
    emu.sw_no_count(12usize, 2usize, 212u32, 2111992u32)?;
    emu.adi_no_count(12usize, 8usize, 1703u32, 2111996u32);
    emu.sw_no_count(12usize, 2usize, 208u32, 2112000u32)?;
    emu.adi_no_count(12usize, 18usize, 372u32, 2112004u32);
    emu.sw_no_count(12usize, 2usize, 204u32, 2112008u32)?;
    emu.adi_no_count(12usize, 6usize, 4294965697u32, 2112012u32);
    emu.sw_no_count(12usize, 2usize, 200u32, 2112016u32)?;
    emu.adi_no_count(12usize, 5usize, 1926u32, 2112020u32);
    emu.sw_no_count(12usize, 2usize, 196u32, 2112024u32)?;
    emu.adi_no_count(12usize, 17usize, 4294966726u32, 2112028u32);
    emu.sw_no_count(12usize, 2usize, 192u32, 2112032u32)?;
    emu.adi_no_count(12usize, 16usize, 460u32, 2112036u32);
    emu.sw_no_count(12usize, 2usize, 188u32, 2112040u32)?;
    emu.adi_no_count(12usize, 15usize, 4294966383u32, 2112044u32);
    emu.sw_no_count(12usize, 2usize, 184u32, 2112048u32)?;
    emu.adi_no_count(12usize, 24usize, 1194u32, 2112052u32);
    emu.sw_no_count(12usize, 2usize, 180u32, 2112056u32)?;
    emu.adi_no_count(12usize, 25usize, 4294965724u32, 2112060u32);
    emu.sw_no_count(12usize, 2usize, 176u32, 2112064u32)?;
    emu.adi_no_count(12usize, 14usize, 4294965466u32, 2112068u32);
    emu.sw_no_count(12usize, 2usize, 172u32, 2112072u32)?;
    emu.adi_no_count(12usize, 27usize, 338u32, 2112076u32);
    emu.sw_no_count(12usize, 2usize, 168u32, 2112080u32)?;
    emu.adi_no_count(12usize, 13usize, 1645u32, 2112084u32);
    emu.sw_no_count(12usize, 2usize, 164u32, 2112088u32)?;
    emu.adi_no_count(10usize, 10usize, 1992u32, 2112092u32);
    emu.sw_no_count(10usize, 2usize, 160u32, 2112096u32)?;
    let a = 0u32.wrapping_add(3210313728u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2112100u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294967239u32, 2112104u32);
    emu.sw_no_count(10usize, 2usize, 156u32, 2112108u32)?;
    let a = 0u32.wrapping_add(3336572928u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2112112u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966259u32, 2112116u32);
    emu.sw_no_count(10usize, 2usize, 152u32, 2112120u32)?;
    let a = 0u32.wrapping_add(3584528384u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2112124u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 327u32, 2112128u32);
    emu.sw_no_count(10usize, 2usize, 148u32, 2112132u32)?;
    let a = 0u32.wrapping_add(113926144u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2112136u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 849u32, 2112140u32);
    emu.sw_no_count(10usize, 2usize, 144u32, 2112144u32)?;
    let a = 0u32.wrapping_add(338243584u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2112148u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965607u32, 2112152u32);
    emu.sw_no_count(10usize, 2usize, 140u32, 2112156u32)?;
    let a = 0u32.wrapping_add(666308608u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2112160u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965893u32, 2112164u32);
    emu.sw_no_count(10usize, 2usize, 136u32, 2112168u32)?;
    let a = 0u32.wrapping_add(773529600u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2112172u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 312u32, 2112176u32);
    emu.sw_no_count(10usize, 2usize, 132u32, 2112180u32)?;
    let a = 0u32.wrapping_add(1294757888u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2112184u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966780u32, 2112188u32);
    emu.sw_no_count(10usize, 2usize, 128u32, 2112192u32)?;
    let a = 0u32.wrapping_add(1396183040u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2112196u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966547u32, 2112200u32);
    emu.sw_no_count(10usize, 2usize, 124u32, 2112204u32)?;
    let a = 0u32.wrapping_add(1695182848u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2112208u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 852u32, 2112212u32);
    emu.sw_no_count(10usize, 2usize, 120u32, 2112216u32)?;
    let a = 0u32.wrapping_add(1986662400u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2112220u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965947u32, 2112224u32);
    emu.sw_no_count(10usize, 2usize, 116u32, 2112228u32)?;
    let a = 0u32.wrapping_add(2177028096u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2112232u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965550u32, 2112236u32);
    emu.sw_no_count(10usize, 2usize, 112u32, 2112240u32)?;
    let a = 0u32.wrapping_add(2456956928u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2112244u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966405u32, 2112248u32);
    emu.sw_no_count(10usize, 2usize, 108u32, 2112252u32)?;
    let a = 0u32.wrapping_add(2730487808u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2112256u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965409u32, 2112260u32);
    emu.sw_no_count(10usize, 2usize, 104u32, 2112264u32)?;
    let a = 0u32.wrapping_add(2820300800u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2112268u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1611u32, 2112272u32);
    emu.sw_no_count(10usize, 2usize, 100u32, 2112276u32)?;
    let a = 0u32.wrapping_add(3259731968u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2112280u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966128u32, 2112284u32);
    emu.sw_no_count(10usize, 2usize, 96u32, 2112288u32)?;
    let a = 0u32.wrapping_add(3345764352u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2112292u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 419u32, 2112296u32);
    emu.sw_no_count(10usize, 2usize, 92u32, 2112300u32)?;
    let a = 0u32.wrapping_add(3516067840u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2112304u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965273u32, 2112308u32);
    emu.sw_no_count(10usize, 2usize, 88u32, 2112312u32)?;
    let a = 0u32.wrapping_add(3600351232u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2112316u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1572u32, 2112320u32);
    emu.sw_no_count(10usize, 2usize, 84u32, 2112324u32)?;
    let a = 0u32.wrapping_add(4094570496u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2112328u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1413u32, 2112332u32);
    emu.sw_no_count(10usize, 2usize, 80u32, 2112336u32)?;
    let a = 0u32.wrapping_add(275423232u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2112340u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 112u32, 2112344u32);
    emu.sw_no_count(10usize, 2usize, 76u32, 2112348u32)?;
    let a = 0u32.wrapping_add(430227456u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2112352u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 278u32, 2112356u32);
    emu.sw_no_count(10usize, 2usize, 72u32, 2112360u32)?;
    let a = 0u32.wrapping_add(506949632u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2112364u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966280u32, 2112368u32);
    emu.sw_no_count(10usize, 2usize, 68u32, 2112372u32)?;
    let a = 0u32.wrapping_add(659058688u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2112376u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1868u32, 2112380u32);
    emu.sw_no_count(10usize, 2usize, 64u32, 2112384u32)?;
    let a = 0u32.wrapping_add(883998720u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2112388u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966453u32, 2112392u32);
    emu.sw_no_count(10usize, 2usize, 60u32, 2112396u32)?;
    let a = 0u32.wrapping_add(958140416u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2112400u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966451u32, 2112404u32);
    emu.sw_no_count(10usize, 2usize, 56u32, 2112408u32)?;
    let a = 0u32.wrapping_add(1322823680u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2112412u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965834u32, 2112416u32);
    emu.sw_no_count(10usize, 2usize, 52u32, 2112420u32)?;
    let a = 0u32.wrapping_add(1537003520u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2112424u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965839u32, 2112428u32);
    emu.sw_no_count(10usize, 2usize, 48u32, 2112432u32)?;
    let a = 0u32.wrapping_add(1747873792u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2112436u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294967283u32, 2112440u32);
    emu.sw_no_count(10usize, 2usize, 44u32, 2112444u32)?;
    let a = 0u32.wrapping_add(1955561472u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2112448u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 750u32, 2112452u32);
    emu.sw_no_count(10usize, 2usize, 40u32, 2112456u32)?;
    let a = 0u32.wrapping_add(2024103936u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2112460u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 879u32, 2112464u32);
    emu.sw_no_count(10usize, 2usize, 36u32, 2112468u32)?;
    let a = 0u32.wrapping_add(2227732480u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2112472u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965268u32, 2112476u32);
    emu.sw_no_count(10usize, 2usize, 32u32, 2112480u32)?;
    let a = 0u32.wrapping_add(2361851904u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2112484u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 520u32, 2112488u32);
    emu.sw_no_count(10usize, 2usize, 28u32, 2112492u32)?;
    let a = 0u32.wrapping_add(2428436480u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2112496u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294967290u32, 2112500u32);
    emu.sw_no_count(10usize, 2usize, 24u32, 2112504u32)?;
    let a = 0u32.wrapping_add(2756734976u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2112508u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966507u32, 2112512u32);
    emu.sw_no_count(10usize, 2usize, 20u32, 2112516u32)?;
    let a = 0u32.wrapping_add(3204030464u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2112520u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1015u32, 2112524u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2112528u32)?;
    let a = 0u32.wrapping_add(3329327104u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2112532u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965490u32, 2112536u32);
    emu.sw_no_count(10usize, 2usize, 12u32, 2112540u32)?;
    emu.add_memory_rw_events(195usize);
    emu.pc = 2112540u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203c1c));
}
#[inline(never)]
pub fn block_0x00203c1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4003u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(22usize, 2usize, 520u32, 2112544u32)?;
    emu.sw_no_count(21usize, 2usize, 512u32, 2112548u32)?;
    emu.sw_no_count(19usize, 2usize, 428u32, 2112552u32)?;
    emu.sw_no_count(20usize, 2usize, 516u32, 2112556u32)?;
    emu.sw_no_count(26usize, 2usize, 432u32, 2112560u32)?;
    emu.sw_no_count(9usize, 2usize, 372u32, 2112564u32)?;
    emu.lbu_no_count(16usize, 11usize, 0u32, 2112568u32);
    emu.lbu_no_count(12usize, 11usize, 1u32, 2112572u32);
    emu.lbu_no_count(13usize, 11usize, 2u32, 2112576u32);
    emu.lbu_no_count(29usize, 11usize, 3u32, 2112580u32);
    emu.lbu_no_count(14usize, 11usize, 4u32, 2112584u32);
    emu.lbu_no_count(15usize, 11usize, 5u32, 2112588u32);
    emu.lbu_no_count(10usize, 11usize, 6u32, 2112592u32);
    emu.sw_no_count(10usize, 2usize, 484u32, 2112596u32)?;
    emu.lbu_no_count(10usize, 11usize, 7u32, 2112600u32);
    emu.sw_no_count(10usize, 2usize, 464u32, 2112604u32)?;
    emu.lbu_no_count(17usize, 11usize, 8u32, 2112608u32);
    emu.lbu_no_count(6usize, 11usize, 9u32, 2112612u32);
    emu.lbu_no_count(10usize, 11usize, 10u32, 2112616u32);
    emu.sw_no_count(10usize, 2usize, 420u32, 2112620u32)?;
    emu.lbu_no_count(10usize, 11usize, 11u32, 2112624u32);
    emu.sw_no_count(10usize, 2usize, 416u32, 2112628u32)?;
    emu.lbu_no_count(5usize, 11usize, 12u32, 2112632u32);
    emu.lbu_no_count(28usize, 11usize, 13u32, 2112636u32);
    emu.lbu_no_count(10usize, 11usize, 14u32, 2112640u32);
    emu.sw_no_count(10usize, 2usize, 500u32, 2112644u32)?;
    emu.lbu_no_count(22usize, 11usize, 15u32, 2112648u32);
    emu.sri_no_count(7usize, 30usize, 6u32, 2112652u32);
    emu.adi_no_count(10usize, 30usize, 0u32, 2112656u32);
    emu.sli_no_count(30usize, 30usize, 26u32, 2112660u32);
    emu.sri_no_count(31usize, 10usize, 11u32, 2112664u32);
    emu.sli_no_count(8usize, 10usize, 21u32, 2112668u32);
    emu.sri_no_count(18usize, 10usize, 25u32, 2112672u32);
    emu.sli_no_count(19usize, 10usize, 7u32, 2112676u32);
    emu.sw_no_count(10usize, 2usize, 424u32, 2112680u32)?;
    emu.sri_no_count(21usize, 23usize, 2u32, 2112684u32);
    emu.adi_no_count(27usize, 23usize, 0u32, 2112688u32);
    emu.sli_no_count(23usize, 23usize, 30u32, 2112692u32);
    emu.sri_no_count(24usize, 27usize, 13u32, 2112696u32);
    emu.sli_no_count(25usize, 27usize, 19u32, 2112700u32);
    emu.orr_no_count(7usize, 30usize, 7usize, 2112704u32);
    emu.sri_no_count(30usize, 27usize, 22u32, 2112708u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2112712u32);
    emu.sli_no_count(8usize, 27usize, 10u32, 2112716u32);
    emu.sw_no_count(27usize, 2usize, 508u32, 2112720u32)?;
    emu.orr_no_count(18usize, 19usize, 18usize, 2112724u32);
    emu.orr_no_count(19usize, 23usize, 21usize, 2112728u32);
    emu.orr_no_count(21usize, 25usize, 24usize, 2112732u32);
    emu.orr_no_count(30usize, 8usize, 30usize, 2112736u32);
    emu.lbu_no_count(23usize, 11usize, 16u32, 2112740u32);
    emu.lbu_no_count(1usize, 11usize, 17u32, 2112744u32);
    emu.lbu_no_count(25usize, 11usize, 18u32, 2112748u32);
    emu.lbu_no_count(24usize, 11usize, 19u32, 2112752u32);
    emu.xrr_no_count(7usize, 7usize, 31usize, 2112756u32);
    emu.lw_no_count(31usize, 2usize, 512u32, 2112760u32)?;
    emu.lw_no_count(8usize, 2usize, 520u32, 2112764u32)?;
    emu.xrr_no_count(31usize, 31usize, 8usize, 2112768u32);
    emu.xrr_no_count(8usize, 19usize, 21usize, 2112772u32);
    emu.lw_no_count(19usize, 2usize, 512u32, 2112776u32)?;
    emu.lw_no_count(21usize, 2usize, 520u32, 2112780u32)?;
    emu.anr_no_count(19usize, 19usize, 21usize, 2112784u32);
    emu.anr_no_count(31usize, 31usize, 27usize, 2112788u32);
    emu.xrr_no_count(31usize, 31usize, 19usize, 2112792u32);
    emu.xrr_no_count(19usize, 26usize, 20usize, 2112796u32);
    emu.anr_no_count(19usize, 19usize, 10usize, 2112800u32);
    emu.xrr_no_count(19usize, 19usize, 26usize, 2112804u32);
    emu.xrr_no_count(7usize, 7usize, 18usize, 2112808u32);
    emu.adr_no_count(19usize, 9usize, 19usize, 2112812u32);
    emu.xrr_no_count(30usize, 8usize, 30usize, 2112816u32);
    emu.adr_no_count(7usize, 19usize, 7usize, 2112820u32);
    emu.sw_no_count(7usize, 2usize, 496u32, 2112824u32)?;
    emu.adr_no_count(30usize, 31usize, 30usize, 2112828u32);
    emu.sw_no_count(30usize, 2usize, 504u32, 2112832u32)?;
    emu.lbu_no_count(30usize, 11usize, 20u32, 2112836u32);
    emu.lbu_no_count(31usize, 11usize, 21u32, 2112840u32);
    emu.lbu_no_count(10usize, 11usize, 22u32, 2112844u32);
    emu.sw_no_count(10usize, 2usize, 488u32, 2112848u32)?;
    emu.lbu_no_count(9usize, 11usize, 23u32, 2112852u32);
    emu.sli_no_count(13usize, 13usize, 8u32, 2112856u32);
    emu.sli_no_count(12usize, 12usize, 16u32, 2112860u32);
    emu.sli_no_count(7usize, 16usize, 24u32, 2112864u32);
    emu.sli_no_count(15usize, 15usize, 16u32, 2112868u32);
    emu.sli_no_count(14usize, 14usize, 24u32, 2112872u32);
    emu.sli_no_count(8usize, 6usize, 16u32, 2112876u32);
    emu.sli_no_count(17usize, 17usize, 24u32, 2112880u32);
    emu.orr_no_count(16usize, 13usize, 29usize, 2112884u32);
    emu.orr_no_count(26usize, 7usize, 12usize, 2112888u32);
    emu.orr_no_count(6usize, 14usize, 15usize, 2112892u32);
    emu.orr_no_count(8usize, 17usize, 8usize, 2112896u32);
    emu.lbu_no_count(10usize, 11usize, 24u32, 2112900u32);
    emu.lbu_no_count(12usize, 11usize, 25u32, 2112904u32);
    emu.lbu_no_count(13usize, 11usize, 26u32, 2112908u32);
    emu.sw_no_count(13usize, 2usize, 472u32, 2112912u32)?;
    emu.lbu_no_count(7usize, 11usize, 27u32, 2112916u32);
    emu.sli_no_count(28usize, 28usize, 16u32, 2112920u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2112924u32);
    emu.sli_no_count(27usize, 1usize, 16u32, 2112928u32);
    emu.sli_no_count(23usize, 23usize, 24u32, 2112932u32);
    emu.sli_no_count(31usize, 31usize, 16u32, 2112936u32);
    emu.sli_no_count(30usize, 30usize, 24u32, 2112940u32);
    emu.sli_no_count(12usize, 12usize, 16u32, 2112944u32);
    emu.sli_no_count(15usize, 10usize, 24u32, 2112948u32);
    emu.orr_no_count(10usize, 5usize, 28usize, 2112952u32);
    emu.orr_no_count(18usize, 23usize, 27usize, 2112956u32);
    emu.orr_no_count(13usize, 30usize, 31usize, 2112960u32);
    emu.lbu_no_count(14usize, 11usize, 28u32, 2112964u32);
    emu.lbu_no_count(17usize, 11usize, 29u32, 2112968u32);
    emu.orr_no_count(15usize, 15usize, 12usize, 2112972u32);
    emu.lbu_no_count(12usize, 11usize, 30u32, 2112976u32);
    emu.sw_no_count(12usize, 2usize, 468u32, 2112980u32)?;
    emu.lbu_no_count(31usize, 11usize, 31u32, 2112984u32);
    emu.sli_no_count(17usize, 17usize, 16u32, 2112988u32);
    emu.sli_no_count(14usize, 14usize, 24u32, 2112992u32);
    emu.orr_no_count(14usize, 14usize, 17usize, 2112996u32);
    emu.sw_no_count(11usize, 2usize, 368u32, 2113000u32)?;
    emu.lbu_no_count(17usize, 11usize, 33u32, 2113004u32);
    emu.lbu_no_count(5usize, 11usize, 32u32, 2113008u32);
    emu.lbu_no_count(23usize, 11usize, 34u32, 2113012u32);
    emu.lbu_no_count(12usize, 11usize, 35u32, 2113016u32);
    emu.sli_no_count(17usize, 17usize, 16u32, 2113020u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2113024u32);
    emu.orr_no_count(1usize, 5usize, 17usize, 2113028u32);
    emu.lbu_no_count(17usize, 11usize, 37u32, 2113032u32);
    emu.lbu_no_count(5usize, 11usize, 36u32, 2113036u32);
    emu.lbu_no_count(28usize, 11usize, 38u32, 2113040u32);
    emu.sw_no_count(28usize, 2usize, 480u32, 2113044u32)?;
    emu.lbu_no_count(28usize, 11usize, 39u32, 2113048u32);
    emu.sw_no_count(28usize, 2usize, 440u32, 2113052u32)?;
    emu.sli_no_count(17usize, 17usize, 16u32, 2113056u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2113060u32);
    emu.orr_no_count(17usize, 5usize, 17usize, 2113064u32);
    emu.sw_no_count(17usize, 2usize, 444u32, 2113068u32)?;
    emu.lbu_no_count(17usize, 11usize, 41u32, 2113072u32);
    emu.lbu_no_count(5usize, 11usize, 40u32, 2113076u32);
    emu.lbu_no_count(28usize, 11usize, 42u32, 2113080u32);
    emu.sw_no_count(28usize, 2usize, 448u32, 2113084u32)?;
    emu.lbu_no_count(28usize, 11usize, 43u32, 2113088u32);
    emu.sw_no_count(28usize, 2usize, 412u32, 2113092u32)?;
    emu.sli_no_count(17usize, 17usize, 16u32, 2113096u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2113100u32);
    emu.orr_no_count(17usize, 5usize, 17usize, 2113104u32);
    emu.sw_no_count(17usize, 2usize, 436u32, 2113108u32)?;
    emu.lbu_no_count(17usize, 11usize, 45u32, 2113112u32);
    emu.lbu_no_count(5usize, 11usize, 44u32, 2113116u32);
    emu.lbu_no_count(28usize, 11usize, 46u32, 2113120u32);
    emu.sw_no_count(28usize, 2usize, 452u32, 2113124u32)?;
    emu.lbu_no_count(28usize, 11usize, 47u32, 2113128u32);
    emu.sw_no_count(28usize, 2usize, 404u32, 2113132u32)?;
    emu.sli_no_count(17usize, 17usize, 16u32, 2113136u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2113140u32);
    emu.orr_no_count(17usize, 5usize, 17usize, 2113144u32);
    emu.sw_no_count(17usize, 2usize, 408u32, 2113148u32)?;
    emu.lbu_no_count(17usize, 11usize, 49u32, 2113152u32);
    emu.lbu_no_count(5usize, 11usize, 48u32, 2113156u32);
    emu.lbu_no_count(28usize, 11usize, 50u32, 2113160u32);
    emu.sw_no_count(28usize, 2usize, 396u32, 2113164u32)?;
    emu.lbu_no_count(28usize, 11usize, 51u32, 2113168u32);
    emu.sw_no_count(28usize, 2usize, 392u32, 2113172u32)?;
    emu.sli_no_count(17usize, 17usize, 16u32, 2113176u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2113180u32);
    emu.orr_no_count(17usize, 5usize, 17usize, 2113184u32);
    emu.sw_no_count(17usize, 2usize, 400u32, 2113188u32)?;
    emu.lbu_no_count(17usize, 11usize, 53u32, 2113192u32);
    emu.lbu_no_count(5usize, 11usize, 52u32, 2113196u32);
    emu.lbu_no_count(29usize, 11usize, 54u32, 2113200u32);
    emu.sw_no_count(29usize, 2usize, 476u32, 2113204u32)?;
    emu.lbu_no_count(29usize, 11usize, 55u32, 2113208u32);
    emu.sw_no_count(29usize, 2usize, 456u32, 2113212u32)?;
    emu.sli_no_count(17usize, 17usize, 16u32, 2113216u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2113220u32);
    emu.orr_no_count(17usize, 5usize, 17usize, 2113224u32);
    emu.sw_no_count(17usize, 2usize, 460u32, 2113228u32)?;
    emu.lbu_no_count(17usize, 11usize, 57u32, 2113232u32);
    emu.lbu_no_count(30usize, 11usize, 56u32, 2113236u32);
    emu.lbu_no_count(5usize, 11usize, 58u32, 2113240u32);
    emu.lbu_no_count(20usize, 11usize, 59u32, 2113244u32);
    emu.sli_no_count(17usize, 17usize, 16u32, 2113248u32);
    emu.sli_no_count(30usize, 30usize, 24u32, 2113252u32);
    emu.orr_no_count(19usize, 30usize, 17usize, 2113256u32);
    emu.lbu_no_count(30usize, 11usize, 61u32, 2113260u32);
    emu.lbu_no_count(27usize, 11usize, 60u32, 2113264u32);
    emu.lbu_no_count(17usize, 11usize, 62u32, 2113268u32);
    emu.lbu_no_count(28usize, 11usize, 63u32, 2113272u32);
    emu.sli_no_count(30usize, 30usize, 16u32, 2113276u32);
    emu.sli_no_count(27usize, 27usize, 24u32, 2113280u32);
    emu.orr_no_count(21usize, 27usize, 30usize, 2113284u32);
    emu.orr_no_count(11usize, 26usize, 16usize, 2113288u32);
    emu.sw_no_count(11usize, 2usize, 492u32, 2113292u32)?;
    emu.sli_no_count(25usize, 25usize, 8u32, 2113296u32);
    emu.orr_no_count(11usize, 25usize, 24usize, 2113300u32);
    emu.sli_no_count(25usize, 24usize, 25u32, 2113304u32);
    emu.orr_no_count(24usize, 18usize, 11usize, 2113308u32);
    emu.sri_no_count(11usize, 24usize, 7u32, 2113312u32);
    emu.sw_no_count(24usize, 2usize, 384u32, 2113316u32)?;
    emu.orr_no_count(11usize, 25usize, 11usize, 2113320u32);
    emu.sw_no_count(11usize, 2usize, 376u32, 2113324u32)?;
    emu.lw_no_count(29usize, 2usize, 500u32, 2113328u32)?;
    emu.sli_no_count(29usize, 29usize, 8u32, 2113332u32);
    emu.orr_no_count(11usize, 29usize, 22usize, 2113336u32);
    emu.sli_no_count(29usize, 22usize, 25u32, 2113340u32);
    emu.orr_no_count(30usize, 10usize, 11usize, 2113344u32);
    emu.sri_no_count(11usize, 30usize, 7u32, 2113348u32);
    emu.sw_no_count(30usize, 2usize, 500u32, 2113352u32)?;
    emu.orr_no_count(11usize, 29usize, 11usize, 2113356u32);
    emu.sw_no_count(11usize, 2usize, 364u32, 2113360u32)?;
    emu.lw_no_count(11usize, 2usize, 420u32, 2113364u32)?;
    emu.sli_no_count(11usize, 11usize, 8u32, 2113368u32);
    emu.lw_no_count(29usize, 2usize, 416u32, 2113372u32)?;
    emu.orr_no_count(11usize, 11usize, 29usize, 2113376u32);
    emu.sli_no_count(29usize, 29usize, 25u32, 2113380u32);
    emu.orr_no_count(22usize, 8usize, 11usize, 2113384u32);
    emu.sri_no_count(11usize, 22usize, 7u32, 2113388u32);
    emu.sw_no_count(22usize, 2usize, 388u32, 2113392u32)?;
    emu.orr_no_count(11usize, 29usize, 11usize, 2113396u32);
    emu.sw_no_count(11usize, 2usize, 360u32, 2113400u32)?;
    emu.lw_no_count(11usize, 2usize, 484u32, 2113404u32)?;
    emu.sli_no_count(11usize, 11usize, 8u32, 2113408u32);
    emu.lw_no_count(16usize, 2usize, 464u32, 2113412u32)?;
    emu.orr_no_count(11usize, 11usize, 16usize, 2113416u32);
    emu.sli_no_count(16usize, 16usize, 25u32, 2113420u32);
    emu.orr_no_count(29usize, 6usize, 11usize, 2113424u32);
    emu.sri_no_count(11usize, 29usize, 7u32, 2113428u32);
    emu.sw_no_count(29usize, 2usize, 380u32, 2113432u32)?;
    emu.orr_no_count(11usize, 16usize, 11usize, 2113436u32);
    emu.sw_no_count(11usize, 2usize, 356u32, 2113440u32)?;
    emu.sri_no_count(11usize, 18usize, 18u32, 2113444u32);
    emu.sli_no_count(16usize, 24usize, 14u32, 2113448u32);
    emu.orr_no_count(11usize, 16usize, 11usize, 2113452u32);
    emu.sw_no_count(11usize, 2usize, 352u32, 2113456u32)?;
    emu.sri_no_count(10usize, 10usize, 18u32, 2113460u32);
    emu.sli_no_count(11usize, 30usize, 14u32, 2113464u32);
    emu.orr_no_count(10usize, 11usize, 10usize, 2113468u32);
    emu.sw_no_count(10usize, 2usize, 348u32, 2113472u32)?;
    emu.sri_no_count(8usize, 8usize, 18u32, 2113476u32);
    emu.sli_no_count(10usize, 22usize, 14u32, 2113480u32);
    emu.orr_no_count(10usize, 10usize, 8usize, 2113484u32);
    emu.sw_no_count(10usize, 2usize, 344u32, 2113488u32)?;
    emu.sri_no_count(10usize, 6usize, 18u32, 2113492u32);
    emu.sli_no_count(11usize, 29usize, 14u32, 2113496u32);
    emu.orr_no_count(10usize, 11usize, 10usize, 2113500u32);
    emu.sw_no_count(10usize, 2usize, 340u32, 2113504u32)?;
    emu.sli_no_count(5usize, 5usize, 8u32, 2113508u32);
    emu.orr_no_count(10usize, 5usize, 20usize, 2113512u32);
    emu.adi_no_count(8usize, 20usize, 0u32, 2113516u32);
    emu.orr_no_count(5usize, 19usize, 10usize, 2113520u32);
    emu.sri_no_count(10usize, 19usize, 17u32, 2113524u32);
    emu.sli_no_count(11usize, 5usize, 15u32, 2113528u32);
    emu.orr_no_count(10usize, 11usize, 10usize, 2113532u32);
    emu.sw_no_count(10usize, 2usize, 336u32, 2113536u32)?;
    emu.sri_no_count(10usize, 19usize, 19u32, 2113540u32);
    emu.adi_no_count(18usize, 19usize, 0u32, 2113544u32);
    emu.sli_no_count(11usize, 5usize, 13u32, 2113548u32);
    emu.orr_no_count(10usize, 11usize, 10usize, 2113552u32);
    emu.sw_no_count(10usize, 2usize, 332u32, 2113556u32)?;
    emu.sli_no_count(17usize, 17usize, 8u32, 2113560u32);
    emu.adi_no_count(25usize, 28usize, 0u32, 2113564u32);
    emu.orr_no_count(10usize, 17usize, 28usize, 2113568u32);
    emu.orr_no_count(16usize, 21usize, 10usize, 2113572u32);
    emu.sri_no_count(10usize, 21usize, 17u32, 2113576u32);
    emu.sli_no_count(11usize, 16usize, 15u32, 2113580u32);
    emu.orr_no_count(10usize, 11usize, 10usize, 2113584u32);
    emu.sw_no_count(10usize, 2usize, 328u32, 2113588u32)?;
    emu.sri_no_count(10usize, 21usize, 19u32, 2113592u32);
    emu.sli_no_count(11usize, 16usize, 13u32, 2113596u32);
    emu.orr_no_count(10usize, 11usize, 10usize, 2113600u32);
    emu.sw_no_count(10usize, 2usize, 324u32, 2113604u32)?;
    emu.sli_no_count(10usize, 23usize, 8u32, 2113608u32);
    emu.orr_no_count(10usize, 10usize, 12usize, 2113612u32);
    emu.sli_no_count(12usize, 12usize, 25u32, 2113616u32);
    emu.orr_no_count(17usize, 1usize, 10usize, 2113620u32);
    emu.sri_no_count(10usize, 17usize, 7u32, 2113624u32);
    emu.sw_no_count(17usize, 2usize, 484u32, 2113628u32)?;
    emu.orr_no_count(10usize, 12usize, 10usize, 2113632u32);
    emu.sw_no_count(10usize, 2usize, 320u32, 2113636u32)?;
    emu.lw_no_count(10usize, 2usize, 468u32, 2113640u32)?;
    emu.sli_no_count(10usize, 10usize, 8u32, 2113644u32);
    emu.orr_no_count(10usize, 10usize, 31usize, 2113648u32);
    emu.sli_no_count(31usize, 31usize, 25u32, 2113652u32);
    emu.orr_no_count(12usize, 14usize, 10usize, 2113656u32);
    emu.sri_no_count(10usize, 12usize, 7u32, 2113660u32);
    emu.sw_no_count(12usize, 2usize, 468u32, 2113664u32)?;
    emu.orr_no_count(10usize, 31usize, 10usize, 2113668u32);
    emu.sw_no_count(10usize, 2usize, 316u32, 2113672u32)?;
    emu.lw_no_count(10usize, 2usize, 472u32, 2113676u32)?;
    emu.sli_no_count(10usize, 10usize, 8u32, 2113680u32);
    emu.orr_no_count(10usize, 10usize, 7usize, 2113684u32);
    emu.sli_no_count(7usize, 7usize, 25u32, 2113688u32);
    emu.orr_no_count(6usize, 15usize, 10usize, 2113692u32);
    emu.sri_no_count(10usize, 6usize, 7u32, 2113696u32);
    emu.sw_no_count(6usize, 2usize, 464u32, 2113700u32)?;
    emu.orr_no_count(10usize, 7usize, 10usize, 2113704u32);
    emu.sw_no_count(10usize, 2usize, 312u32, 2113708u32)?;
    emu.lw_no_count(10usize, 2usize, 488u32, 2113712u32)?;
    emu.sli_no_count(10usize, 10usize, 8u32, 2113716u32);
    emu.orr_no_count(10usize, 10usize, 9usize, 2113720u32);
    emu.sli_no_count(11usize, 9usize, 25u32, 2113724u32);
    emu.orr_no_count(7usize, 13usize, 10usize, 2113728u32);
    emu.sri_no_count(10usize, 7usize, 7u32, 2113732u32);
    emu.sw_no_count(7usize, 2usize, 472u32, 2113736u32)?;
    emu.orr_no_count(10usize, 11usize, 10usize, 2113740u32);
    emu.sw_no_count(10usize, 2usize, 308u32, 2113744u32)?;
    emu.sri_no_count(10usize, 1usize, 18u32, 2113748u32);
    emu.sli_no_count(11usize, 17usize, 14u32, 2113752u32);
    emu.orr_no_count(10usize, 11usize, 10usize, 2113756u32);
    emu.sw_no_count(10usize, 2usize, 304u32, 2113760u32)?;
    emu.sri_no_count(14usize, 14usize, 18u32, 2113764u32);
    emu.sli_no_count(10usize, 12usize, 14u32, 2113768u32);
    emu.orr_no_count(10usize, 10usize, 14usize, 2113772u32);
    emu.sw_no_count(10usize, 2usize, 300u32, 2113776u32)?;
    emu.sri_no_count(15usize, 15usize, 18u32, 2113780u32);
    emu.sli_no_count(10usize, 6usize, 14u32, 2113784u32);
    emu.orr_no_count(20usize, 10usize, 15usize, 2113788u32);
    emu.sri_no_count(13usize, 13usize, 18u32, 2113792u32);
    emu.sli_no_count(10usize, 7usize, 14u32, 2113796u32);
    emu.orr_no_count(28usize, 10usize, 13usize, 2113800u32);
    emu.lw_no_count(10usize, 2usize, 396u32, 2113804u32)?;
    emu.sli_no_count(10usize, 10usize, 8u32, 2113808u32);
    emu.lw_no_count(23usize, 2usize, 392u32, 2113812u32)?;
    emu.orr_no_count(10usize, 10usize, 23usize, 2113816u32);
    emu.sli_no_count(23usize, 23usize, 25u32, 2113820u32);
    emu.lw_no_count(13usize, 2usize, 400u32, 2113824u32)?;
    emu.orr_no_count(7usize, 13usize, 10usize, 2113828u32);
    emu.sri_no_count(10usize, 7usize, 7u32, 2113832u32);
    emu.sw_no_count(7usize, 2usize, 488u32, 2113836u32)?;
    emu.orr_no_count(23usize, 23usize, 10usize, 2113840u32);
    emu.lw_no_count(10usize, 2usize, 452u32, 2113844u32)?;
    emu.sli_no_count(10usize, 10usize, 8u32, 2113848u32);
    emu.lw_no_count(11usize, 2usize, 404u32, 2113852u32)?;
    emu.orr_no_count(10usize, 10usize, 11usize, 2113856u32);
    emu.sli_no_count(11usize, 11usize, 25u32, 2113860u32);
    emu.lw_no_count(6usize, 2usize, 408u32, 2113864u32)?;
    emu.orr_no_count(12usize, 6usize, 10usize, 2113868u32);
    emu.sri_no_count(10usize, 12usize, 7u32, 2113872u32);
    emu.sw_no_count(12usize, 2usize, 452u32, 2113876u32)?;
    emu.orr_no_count(19usize, 11usize, 10usize, 2113880u32);
    emu.lw_no_count(10usize, 2usize, 448u32, 2113884u32)?;
    emu.sli_no_count(10usize, 10usize, 8u32, 2113888u32);
    emu.lw_no_count(11usize, 2usize, 412u32, 2113892u32)?;
    emu.orr_no_count(10usize, 10usize, 11usize, 2113896u32);
    emu.sli_no_count(11usize, 11usize, 25u32, 2113900u32);
    emu.lw_no_count(17usize, 2usize, 436u32, 2113904u32)?;
    emu.orr_no_count(30usize, 17usize, 10usize, 2113908u32);
    emu.sri_no_count(15usize, 30usize, 7u32, 2113912u32);
    emu.sw_no_count(30usize, 2usize, 448u32, 2113916u32)?;
    emu.orr_no_count(31usize, 11usize, 15usize, 2113920u32);
    emu.lw_no_count(10usize, 2usize, 480u32, 2113924u32)?;
    emu.sli_no_count(10usize, 10usize, 8u32, 2113928u32);
    emu.lw_no_count(11usize, 2usize, 440u32, 2113932u32)?;
    emu.orr_no_count(10usize, 10usize, 11usize, 2113936u32);
    emu.sli_no_count(11usize, 11usize, 25u32, 2113940u32);
    emu.lw_no_count(15usize, 2usize, 444u32, 2113944u32)?;
    emu.orr_no_count(26usize, 15usize, 10usize, 2113948u32);
    emu.sri_no_count(14usize, 26usize, 7u32, 2113952u32);
    emu.sw_no_count(26usize, 2usize, 480u32, 2113956u32)?;
    emu.orr_no_count(9usize, 11usize, 14usize, 2113960u32);
    emu.sri_no_count(10usize, 13usize, 18u32, 2113964u32);
    emu.sli_no_count(13usize, 7usize, 14u32, 2113968u32);
    emu.orr_no_count(22usize, 13usize, 10usize, 2113972u32);
    emu.sri_no_count(10usize, 6usize, 18u32, 2113976u32);
    emu.sli_no_count(12usize, 12usize, 14u32, 2113980u32);
    emu.orr_no_count(29usize, 12usize, 10usize, 2113984u32);
    emu.sri_no_count(10usize, 17usize, 18u32, 2113988u32);
    emu.sli_no_count(11usize, 30usize, 14u32, 2113992u32);
    emu.orr_no_count(24usize, 11usize, 10usize, 2113996u32);
    emu.sri_no_count(11usize, 15usize, 18u32, 2114000u32);
    emu.sli_no_count(10usize, 26usize, 14u32, 2114004u32);
    emu.orr_no_count(1usize, 10usize, 11usize, 2114008u32);
    emu.sli_no_count(11usize, 25usize, 25u32, 2114012u32);
    emu.sri_no_count(27usize, 16usize, 7u32, 2114016u32);
    emu.orr_no_count(27usize, 11usize, 27usize, 2114020u32);
    emu.sli_no_count(11usize, 8usize, 25u32, 2114024u32);
    emu.adi_no_count(6usize, 5usize, 0u32, 2114028u32);
    emu.sri_no_count(26usize, 5usize, 7u32, 2114032u32);
    emu.orr_no_count(26usize, 11usize, 26usize, 2114036u32);
    emu.lw_no_count(11usize, 2usize, 476u32, 2114040u32)?;
    emu.sli_no_count(11usize, 11usize, 8u32, 2114044u32);
    emu.lw_no_count(10usize, 2usize, 456u32, 2114048u32)?;
    emu.orr_no_count(11usize, 11usize, 10usize, 2114052u32);
    emu.sli_no_count(10usize, 10usize, 25u32, 2114056u32);
    emu.lw_no_count(13usize, 2usize, 460u32, 2114060u32)?;
    emu.orr_no_count(8usize, 13usize, 11usize, 2114064u32);
    emu.sri_no_count(11usize, 8usize, 7u32, 2114068u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2114072u32);
    emu.sri_no_count(11usize, 21usize, 18u32, 2114076u32);
    emu.sli_no_count(21usize, 16usize, 14u32, 2114080u32);
    emu.orr_no_count(21usize, 21usize, 11usize, 2114084u32);
    emu.sri_no_count(11usize, 18usize, 18u32, 2114088u32);
    emu.sli_no_count(25usize, 5usize, 14u32, 2114092u32);
    emu.orr_no_count(25usize, 25usize, 11usize, 2114096u32);
    emu.sri_no_count(11usize, 13usize, 18u32, 2114100u32);
    emu.sli_no_count(18usize, 8usize, 14u32, 2114104u32);
    emu.orr_no_count(12usize, 18usize, 11usize, 2114108u32);
    emu.lw_no_count(11usize, 2usize, 376u32, 2114112u32)?;
    emu.lw_no_count(13usize, 2usize, 352u32, 2114116u32)?;
    emu.xrr_no_count(30usize, 11usize, 13usize, 2114120u32);
    emu.lw_no_count(11usize, 2usize, 364u32, 2114124u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2114128u32)?;
    emu.xrr_no_count(11usize, 11usize, 13usize, 2114132u32);
    emu.lw_no_count(13usize, 2usize, 360u32, 2114136u32)?;
    emu.lw_no_count(14usize, 2usize, 344u32, 2114140u32)?;
    emu.xrr_no_count(13usize, 13usize, 14usize, 2114144u32);
    emu.lw_no_count(14usize, 2usize, 356u32, 2114148u32)?;
    emu.lw_no_count(15usize, 2usize, 340u32, 2114152u32)?;
    emu.xrr_no_count(14usize, 14usize, 15usize, 2114156u32);
    emu.lw_no_count(15usize, 2usize, 336u32, 2114160u32)?;
    emu.lw_no_count(17usize, 2usize, 332u32, 2114164u32)?;
    emu.xrr_no_count(15usize, 15usize, 17usize, 2114168u32);
    emu.lw_no_count(17usize, 2usize, 328u32, 2114172u32)?;
    emu.lw_no_count(5usize, 2usize, 324u32, 2114176u32)?;
    emu.xrr_no_count(17usize, 17usize, 5usize, 2114180u32);
    emu.lw_no_count(5usize, 2usize, 320u32, 2114184u32)?;
    emu.lw_no_count(7usize, 2usize, 304u32, 2114188u32)?;
    emu.xrr_no_count(5usize, 5usize, 7usize, 2114192u32);
    emu.lw_no_count(7usize, 2usize, 316u32, 2114196u32)?;
    emu.lw_no_count(18usize, 2usize, 300u32, 2114200u32)?;
    emu.xrr_no_count(7usize, 7usize, 18usize, 2114204u32);
    emu.lw_no_count(18usize, 2usize, 312u32, 2114208u32)?;
    emu.xrr_no_count(18usize, 18usize, 20usize, 2114212u32);
    emu.lw_no_count(20usize, 2usize, 308u32, 2114216u32)?;
    emu.xrr_no_count(20usize, 20usize, 28usize, 2114220u32);
    emu.xrr_no_count(22usize, 23usize, 22usize, 2114224u32);
    emu.xrr_no_count(23usize, 19usize, 29usize, 2114228u32);
    emu.xrr_no_count(28usize, 31usize, 24usize, 2114232u32);
    emu.sw_no_count(28usize, 2usize, 436u32, 2114236u32)?;
    emu.xrr_no_count(28usize, 9usize, 1usize, 2114240u32);
    emu.sw_no_count(28usize, 2usize, 456u32, 2114244u32)?;
    emu.xrr_no_count(28usize, 27usize, 21usize, 2114248u32);
    emu.sw_no_count(28usize, 2usize, 356u32, 2114252u32)?;
    emu.xrr_no_count(28usize, 26usize, 25usize, 2114256u32);
    emu.sw_no_count(28usize, 2usize, 364u32, 2114260u32)?;
    emu.xrr_no_count(10usize, 10usize, 12usize, 2114264u32);
    emu.sw_no_count(10usize, 2usize, 396u32, 2114268u32)?;
    emu.lw_no_count(10usize, 2usize, 264u32, 2114272u32)?;
    emu.lw_no_count(12usize, 2usize, 496u32, 2114276u32)?;
    emu.adr_no_count(10usize, 12usize, 10usize, 2114280u32);
    emu.lw_no_count(9usize, 2usize, 492u32, 2114284u32)?;
    emu.adr_no_count(10usize, 10usize, 9usize, 2114288u32);
    emu.lw_no_count(12usize, 2usize, 504u32, 2114292u32)?;
    emu.adr_no_count(12usize, 12usize, 10usize, 2114296u32);
    emu.lw_no_count(31usize, 2usize, 428u32, 2114300u32)?;
    emu.adr_no_count(28usize, 10usize, 31usize, 2114304u32);
    emu.sri_no_count(10usize, 28usize, 6u32, 2114308u32);
    emu.sli_no_count(29usize, 28usize, 26u32, 2114312u32);
    emu.orr_no_count(10usize, 29usize, 10usize, 2114316u32);
    emu.sw_no_count(10usize, 2usize, 496u32, 2114320u32)?;
    emu.sri_no_count(10usize, 28usize, 11u32, 2114324u32);
    emu.sli_no_count(29usize, 28usize, 21u32, 2114328u32);
    emu.orr_no_count(10usize, 29usize, 10usize, 2114332u32);
    emu.sw_no_count(10usize, 2usize, 504u32, 2114336u32)?;
    emu.sri_no_count(10usize, 28usize, 25u32, 2114340u32);
    emu.sli_no_count(29usize, 28usize, 7u32, 2114344u32);
    emu.sw_no_count(28usize, 2usize, 444u32, 2114348u32)?;
    emu.orr_no_count(10usize, 29usize, 10usize, 2114352u32);
    emu.sw_no_count(10usize, 2usize, 440u32, 2114356u32)?;
    emu.adi_no_count(31usize, 12usize, 0u32, 2114360u32);
    emu.sw_no_count(12usize, 2usize, 360u32, 2114364u32)?;
    emu.sri_no_count(29usize, 12usize, 2u32, 2114368u32);
    emu.sli_no_count(27usize, 12usize, 30u32, 2114372u32);
    emu.orr_no_count(1usize, 27usize, 29usize, 2114376u32);
    emu.sri_no_count(29usize, 12usize, 13u32, 2114380u32);
    emu.sli_no_count(27usize, 12usize, 19u32, 2114384u32);
    emu.orr_no_count(19usize, 27usize, 29usize, 2114388u32);
    emu.sri_no_count(29usize, 12usize, 22u32, 2114392u32);
    emu.sli_no_count(27usize, 12usize, 10u32, 2114396u32);
    emu.orr_no_count(10usize, 27usize, 29usize, 2114400u32);
    emu.sw_no_count(10usize, 2usize, 408u32, 2114404u32)?;
    emu.lw_no_count(12usize, 2usize, 520u32, 2114408u32)?;
    emu.lw_no_count(10usize, 2usize, 508u32, 2114412u32)?;
    emu.xrr_no_count(27usize, 12usize, 10usize, 2114416u32);
    emu.anr_no_count(27usize, 31usize, 27usize, 2114420u32);
    emu.lw_no_count(12usize, 2usize, 520u32, 2114424u32)?;
    emu.anr_no_count(24usize, 12usize, 10usize, 2114428u32);
    emu.xrr_no_count(10usize, 27usize, 24usize, 2114432u32);
    emu.sw_no_count(10usize, 2usize, 376u32, 2114436u32)?;
    emu.lw_no_count(31usize, 2usize, 384u32, 2114440u32)?;
    emu.sri_no_count(24usize, 31usize, 3u32, 2114444u32);
    emu.xrr_no_count(26usize, 30usize, 24usize, 2114448u32);
    emu.lw_no_count(29usize, 2usize, 500u32, 2114452u32)?;
    emu.sri_no_count(24usize, 29usize, 3u32, 2114456u32);
    emu.xrr_no_count(21usize, 11usize, 24usize, 2114460u32);
    emu.lw_no_count(30usize, 2usize, 388u32, 2114464u32)?;
    emu.sri_no_count(24usize, 30usize, 3u32, 2114468u32);
    emu.xrr_no_count(24usize, 13usize, 24usize, 2114472u32);
    emu.lw_no_count(10usize, 2usize, 380u32, 2114476u32)?;
    emu.sri_no_count(13usize, 10usize, 3u32, 2114480u32);
    emu.xrr_no_count(25usize, 14usize, 13usize, 2114484u32);
    emu.sri_no_count(14usize, 6usize, 10u32, 2114488u32);
    emu.xrr_no_count(14usize, 15usize, 14usize, 2114492u32);
    emu.sw_no_count(14usize, 2usize, 352u32, 2114496u32)?;
    emu.sri_no_count(15usize, 16usize, 10u32, 2114500u32);
    emu.xrr_no_count(27usize, 17usize, 15usize, 2114504u32);
    emu.lw_no_count(15usize, 2usize, 484u32, 2114508u32)?;
    emu.sri_no_count(15usize, 15usize, 3u32, 2114512u32);
    emu.xrr_no_count(11usize, 5usize, 15usize, 2114516u32);
    emu.sw_no_count(11usize, 2usize, 476u32, 2114520u32)?;
    emu.lw_no_count(11usize, 2usize, 468u32, 2114524u32)?;
    emu.sri_no_count(17usize, 11usize, 3u32, 2114528u32);
    emu.xrr_no_count(17usize, 7usize, 17usize, 2114532u32);
    emu.lw_no_count(14usize, 2usize, 464u32, 2114536u32)?;
    emu.sri_no_count(5usize, 14usize, 3u32, 2114540u32);
    emu.xrr_no_count(18usize, 18usize, 5usize, 2114544u32);
    emu.lw_no_count(13usize, 2usize, 472u32, 2114548u32)?;
    emu.sri_no_count(5usize, 13usize, 3u32, 2114552u32);
    emu.xrr_no_count(20usize, 20usize, 5usize, 2114556u32);
    emu.lw_no_count(12usize, 2usize, 488u32, 2114560u32)?;
    emu.sri_no_count(5usize, 12usize, 3u32, 2114564u32);
    emu.xrr_no_count(11usize, 22usize, 5usize, 2114568u32);
    emu.sw_no_count(11usize, 2usize, 460u32, 2114572u32)?;
    emu.lw_no_count(11usize, 2usize, 452u32, 2114576u32)?;
    emu.sri_no_count(5usize, 11usize, 3u32, 2114580u32);
    emu.xrr_no_count(15usize, 23usize, 5usize, 2114584u32);
    emu.sw_no_count(15usize, 2usize, 400u32, 2114588u32)?;
    emu.lw_no_count(22usize, 2usize, 448u32, 2114592u32)?;
    emu.sri_no_count(5usize, 22usize, 3u32, 2114596u32);
    emu.lw_no_count(15usize, 2usize, 436u32, 2114600u32)?;
    emu.xrr_no_count(15usize, 15usize, 5usize, 2114604u32);
    emu.sw_no_count(15usize, 2usize, 392u32, 2114608u32)?;
    emu.lw_no_count(23usize, 2usize, 480u32, 2114612u32)?;
    emu.sri_no_count(5usize, 23usize, 3u32, 2114616u32);
    emu.lw_no_count(15usize, 2usize, 456u32, 2114620u32)?;
    emu.xrr_no_count(15usize, 15usize, 5usize, 2114624u32);
    emu.sw_no_count(15usize, 2usize, 404u32, 2114628u32)?;
    emu.sw_no_count(16usize, 2usize, 420u32, 2114632u32)?;
    emu.sri_no_count(7usize, 16usize, 3u32, 2114636u32);
    emu.lw_no_count(15usize, 2usize, 356u32, 2114640u32)?;
    emu.xrr_no_count(5usize, 15usize, 7usize, 2114644u32);
    emu.sw_no_count(5usize, 2usize, 436u32, 2114648u32)?;
    emu.adi_no_count(5usize, 6usize, 0u32, 2114652u32);
    emu.sw_no_count(6usize, 2usize, 416u32, 2114656u32)?;
    emu.sri_no_count(7usize, 6usize, 3u32, 2114660u32);
    emu.lw_no_count(15usize, 2usize, 364u32, 2114664u32)?;
    emu.xrr_no_count(15usize, 15usize, 7usize, 2114668u32);
    emu.sw_no_count(15usize, 2usize, 456u32, 2114672u32)?;
    emu.sw_no_count(8usize, 2usize, 412u32, 2114676u32)?;
    emu.sri_no_count(6usize, 8usize, 3u32, 2114680u32);
    emu.lw_no_count(15usize, 2usize, 396u32, 2114684u32)?;
    emu.xrr_no_count(15usize, 15usize, 6usize, 2114688u32);
    emu.sw_no_count(15usize, 2usize, 396u32, 2114692u32)?;
    emu.lw_no_count(15usize, 2usize, 504u32, 2114696u32)?;
    emu.lw_no_count(6usize, 2usize, 496u32, 2114700u32)?;
    emu.xrr_no_count(6usize, 6usize, 15usize, 2114704u32);
    emu.lw_no_count(7usize, 2usize, 516u32, 2114708u32)?;
    emu.lw_no_count(15usize, 2usize, 424u32, 2114712u32)?;
    emu.xrr_no_count(7usize, 7usize, 15usize, 2114716u32);
    emu.anr_no_count(7usize, 28usize, 7usize, 2114720u32);
    emu.lw_no_count(28usize, 2usize, 516u32, 2114724u32)?;
    emu.xrr_no_count(7usize, 7usize, 28usize, 2114728u32);
    emu.lw_no_count(28usize, 2usize, 432u32, 2114732u32)?;
    emu.adr_no_count(28usize, 28usize, 10usize, 2114736u32);
    emu.adr_no_count(7usize, 28usize, 7usize, 2114740u32);
    emu.xrr_no_count(19usize, 1usize, 19usize, 2114744u32);
    emu.adr_no_count(28usize, 29usize, 12usize, 2114748u32);
    emu.adr_no_count(28usize, 26usize, 28usize, 2114752u32);
    emu.sw_no_count(28usize, 2usize, 504u32, 2114756u32)?;
    emu.adr_no_count(28usize, 30usize, 11usize, 2114760u32);
    emu.adi_no_count(11usize, 30usize, 0u32, 2114764u32);
    emu.adr_no_count(28usize, 21usize, 28usize, 2114768u32);
    emu.sw_no_count(28usize, 2usize, 496u32, 2114772u32)?;
    emu.adr_no_count(28usize, 10usize, 22usize, 2114776u32);
    emu.adr_no_count(28usize, 24usize, 28usize, 2114780u32);
    emu.adr_no_count(30usize, 9usize, 23usize, 2114784u32);
    emu.adr_no_count(30usize, 25usize, 30usize, 2114788u32);
    emu.adr_no_count(23usize, 14usize, 16usize, 2114792u32);
    emu.adr_no_count(23usize, 17usize, 23usize, 2114796u32);
    emu.adr_no_count(1usize, 13usize, 5usize, 2114800u32);
    emu.adr_no_count(1usize, 18usize, 1usize, 2114804u32);
    emu.adr_no_count(17usize, 31usize, 8usize, 2114808u32);
    emu.adr_no_count(12usize, 20usize, 17usize, 2114812u32);
    emu.lw_no_count(10usize, 2usize, 440u32, 2114816u32)?;
    emu.xrr_no_count(17usize, 6usize, 10usize, 2114820u32);
    emu.lw_no_count(10usize, 2usize, 408u32, 2114824u32)?;
    emu.xrr_no_count(6usize, 19usize, 10usize, 2114828u32);
    emu.lw_no_count(13usize, 2usize, 352u32, 2114832u32)?;
    emu.adr_no_count(22usize, 30usize, 13usize, 2114836u32);
    emu.adr_no_count(16usize, 28usize, 27usize, 2114840u32);
    emu.lw_no_count(10usize, 2usize, 260u32, 2114844u32)?;
    emu.adr_no_count(10usize, 7usize, 10usize, 2114848u32);
    emu.adr_no_count(10usize, 10usize, 17usize, 2114852u32);
    emu.lw_no_count(27usize, 2usize, 376u32, 2114856u32)?;
    emu.adr_no_count(27usize, 6usize, 27usize, 2114860u32);
    emu.sri_no_count(13usize, 22usize, 17u32, 2114864u32);
    emu.sli_no_count(14usize, 22usize, 15u32, 2114868u32);
    emu.orr_no_count(29usize, 14usize, 13usize, 2114872u32);
    emu.sri_no_count(13usize, 22usize, 19u32, 2114876u32);
    emu.sli_no_count(14usize, 22usize, 13u32, 2114880u32);
    emu.orr_no_count(30usize, 14usize, 13usize, 2114884u32);
    emu.sri_no_count(13usize, 16usize, 17u32, 2114888u32);
    emu.sli_no_count(14usize, 16usize, 15u32, 2114892u32);
    emu.orr_no_count(18usize, 14usize, 13usize, 2114896u32);
    emu.sri_no_count(13usize, 16usize, 19u32, 2114900u32);
    emu.sli_no_count(14usize, 16usize, 13u32, 2114904u32);
    emu.orr_no_count(17usize, 14usize, 13usize, 2114908u32);
    emu.sri_no_count(13usize, 22usize, 7u32, 2114912u32);
    emu.sli_no_count(14usize, 22usize, 25u32, 2114916u32);
    emu.orr_no_count(6usize, 14usize, 13usize, 2114920u32);
    emu.sri_no_count(13usize, 22usize, 18u32, 2114924u32);
    emu.sli_no_count(14usize, 22usize, 14u32, 2114928u32);
    emu.orr_no_count(7usize, 14usize, 13usize, 2114932u32);
    emu.adi_no_count(5usize, 16usize, 0u32, 2114936u32);
    emu.sri_no_count(13usize, 16usize, 7u32, 2114940u32);
    emu.sli_no_count(14usize, 16usize, 25u32, 2114944u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2114948u32);
    emu.sri_no_count(14usize, 16usize, 18u32, 2114952u32);
    emu.sli_no_count(16usize, 16usize, 14u32, 2114956u32);
    emu.adi_no_count(24usize, 5usize, 0u32, 2114960u32);
    emu.orr_no_count(14usize, 16usize, 14usize, 2114964u32);
    emu.xrr_no_count(16usize, 29usize, 30usize, 2114968u32);
    emu.xrr_no_count(17usize, 18usize, 17usize, 2114972u32);
    emu.xrr_no_count(6usize, 6usize, 7usize, 2114976u32);
    emu.xrr_no_count(13usize, 13usize, 14usize, 2114980u32);
    emu.adr_no_count(27usize, 27usize, 10usize, 2114984u32);
    emu.lw_no_count(14usize, 2usize, 512u32, 2114988u32)?;
    emu.adr_no_count(10usize, 10usize, 14usize, 2114992u32);
    emu.sri_no_count(14usize, 10usize, 6u32, 2114996u32);
    emu.sli_no_count(7usize, 10usize, 26u32, 2115000u32);
    emu.orr_no_count(14usize, 7usize, 14usize, 2115004u32);
    emu.sri_no_count(7usize, 10usize, 11u32, 2115008u32);
    emu.sli_no_count(28usize, 10usize, 21u32, 2115012u32);
    emu.orr_no_count(7usize, 28usize, 7usize, 2115016u32);
    emu.sri_no_count(28usize, 10usize, 25u32, 2115020u32);
    emu.sli_no_count(29usize, 10usize, 7u32, 2115024u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2115028u32);
    emu.sri_no_count(29usize, 27usize, 2u32, 2115032u32);
    emu.sli_no_count(30usize, 27usize, 30u32, 2115036u32);
    emu.orr_no_count(30usize, 30usize, 29usize, 2115040u32);
    emu.sri_no_count(29usize, 27usize, 13u32, 2115044u32);
    emu.sli_no_count(18usize, 27usize, 19u32, 2115048u32);
    emu.orr_no_count(18usize, 18usize, 29usize, 2115052u32);
    emu.sri_no_count(29usize, 27usize, 22u32, 2115056u32);
    emu.sli_no_count(19usize, 27usize, 10u32, 2115060u32);
    emu.orr_no_count(19usize, 19usize, 29usize, 2115064u32);
    emu.lw_no_count(8usize, 2usize, 508u32, 2115068u32)?;
    emu.lw_no_count(26usize, 2usize, 360u32, 2115072u32)?;
    emu.xrr_no_count(29usize, 26usize, 8usize, 2115076u32);
    emu.anr_no_count(29usize, 27usize, 29usize, 2115080u32);
    emu.anr_no_count(20usize, 26usize, 8usize, 2115084u32);
    emu.xrr_no_count(20usize, 29usize, 20usize, 2115088u32);
    emu.adi_no_count(5usize, 22usize, 0u32, 2115092u32);
    emu.sri_no_count(29usize, 22usize, 10u32, 2115096u32);
    emu.xrr_no_count(16usize, 16usize, 29usize, 2115100u32);
    emu.sw_no_count(24usize, 2usize, 492u32, 2115104u32)?;
    emu.sri_no_count(29usize, 24usize, 10u32, 2115108u32);
    emu.xrr_no_count(17usize, 17usize, 29usize, 2115112u32);
    emu.sri_no_count(29usize, 22usize, 3u32, 2115116u32);
    emu.sw_no_count(22usize, 2usize, 408u32, 2115120u32)?;
    emu.xrr_no_count(6usize, 6usize, 29usize, 2115124u32);
    emu.sw_no_count(6usize, 2usize, 364u32, 2115128u32)?;
    emu.sri_no_count(6usize, 24usize, 3u32, 2115132u32);
    emu.xrr_no_count(13usize, 13usize, 6usize, 2115136u32);
    emu.sw_no_count(13usize, 2usize, 352u32, 2115140u32)?;
    emu.xrr_no_count(13usize, 14usize, 7usize, 2115144u32);
    emu.lw_no_count(14usize, 2usize, 516u32, 2115148u32)?;
    emu.adr_no_count(14usize, 14usize, 11usize, 2115152u32);
    emu.adi_no_count(7usize, 15usize, 0u32, 2115156u32);
    emu.lw_no_count(15usize, 2usize, 444u32, 2115160u32)?;
    emu.xrr_no_count(6usize, 15usize, 7usize, 2115164u32);
    emu.anr_no_count(6usize, 10usize, 6usize, 2115168u32);
    emu.xrr_no_count(6usize, 6usize, 7usize, 2115172u32);
    emu.adi_no_count(21usize, 7usize, 0u32, 2115176u32);
    emu.adr_no_count(14usize, 14usize, 6usize, 2115180u32);
    emu.xrr_no_count(6usize, 30usize, 18usize, 2115184u32);
    emu.lw_no_count(29usize, 2usize, 496u32, 2115188u32)?;
    emu.adr_no_count(29usize, 29usize, 16usize, 2115192u32);
    emu.sw_no_count(29usize, 2usize, 496u32, 2115196u32)?;
    emu.lw_no_count(9usize, 2usize, 504u32, 2115200u32)?;
    emu.adr_no_count(18usize, 9usize, 17usize, 2115204u32);
    emu.xrr_no_count(11usize, 13usize, 28usize, 2115208u32);
    emu.xrr_no_count(13usize, 6usize, 19usize, 2115212u32);
    emu.lw_no_count(16usize, 2usize, 256u32, 2115216u32)?;
    emu.adr_no_count(14usize, 14usize, 16usize, 2115220u32);
    emu.adr_no_count(11usize, 14usize, 11usize, 2115224u32);
    emu.adr_no_count(13usize, 13usize, 20usize, 2115228u32);
    emu.sri_no_count(14usize, 29usize, 17u32, 2115232u32);
    emu.sli_no_count(16usize, 29usize, 15u32, 2115236u32);
    emu.orr_no_count(14usize, 16usize, 14usize, 2115240u32);
    emu.sri_no_count(16usize, 29usize, 19u32, 2115244u32);
    emu.sli_no_count(17usize, 29usize, 13u32, 2115248u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2115252u32);
    emu.sri_no_count(17usize, 18usize, 17u32, 2115256u32);
    emu.sli_no_count(6usize, 18usize, 15u32, 2115260u32);
    emu.orr_no_count(17usize, 6usize, 17usize, 2115264u32);
    emu.sri_no_count(6usize, 18usize, 19u32, 2115268u32);
    emu.sli_no_count(7usize, 18usize, 13u32, 2115272u32);
    emu.orr_no_count(6usize, 7usize, 6usize, 2115276u32);
    emu.sri_no_count(7usize, 18usize, 7u32, 2115280u32);
    emu.sli_no_count(28usize, 18usize, 25u32, 2115284u32);
    emu.orr_no_count(7usize, 28usize, 7usize, 2115288u32);
    emu.sri_no_count(28usize, 29usize, 7u32, 2115292u32);
    emu.sli_no_count(30usize, 29usize, 25u32, 2115296u32);
    emu.orr_no_count(28usize, 30usize, 28usize, 2115300u32);
    emu.sri_no_count(30usize, 18usize, 18u32, 2115304u32);
    emu.sli_no_count(9usize, 18usize, 14u32, 2115308u32);
    emu.adi_no_count(19usize, 18usize, 0u32, 2115312u32);
    emu.orr_no_count(30usize, 9usize, 30usize, 2115316u32);
    emu.sri_no_count(9usize, 29usize, 18u32, 2115320u32);
    emu.sli_no_count(18usize, 29usize, 14u32, 2115324u32);
    emu.orr_no_count(9usize, 18usize, 9usize, 2115328u32);
    emu.xrr_no_count(14usize, 14usize, 16usize, 2115332u32);
    emu.xrr_no_count(16usize, 17usize, 6usize, 2115336u32);
    emu.xrr_no_count(17usize, 7usize, 30usize, 2115340u32);
    emu.xrr_no_count(6usize, 28usize, 9usize, 2115344u32);
    emu.sri_no_count(7usize, 29usize, 10u32, 2115348u32);
    emu.xrr_no_count(7usize, 14usize, 7usize, 2115352u32);
    emu.sw_no_count(19usize, 2usize, 440u32, 2115356u32)?;
    emu.sri_no_count(14usize, 19usize, 10u32, 2115360u32);
    emu.xrr_no_count(16usize, 16usize, 14usize, 2115364u32);
    emu.sri_no_count(14usize, 19usize, 3u32, 2115368u32);
    emu.xrr_no_count(14usize, 17usize, 14usize, 2115372u32);
    emu.sw_no_count(14usize, 2usize, 388u32, 2115376u32)?;
    emu.sri_no_count(14usize, 29usize, 3u32, 2115380u32);
    emu.xrr_no_count(14usize, 6usize, 14usize, 2115384u32);
    emu.sw_no_count(14usize, 2usize, 348u32, 2115388u32)?;
    emu.adr_no_count(29usize, 13usize, 11usize, 2115392u32);
    emu.lw_no_count(22usize, 2usize, 520u32, 2115396u32)?;
    emu.adr_no_count(22usize, 11usize, 22usize, 2115400u32);
    emu.sri_no_count(11usize, 22usize, 6u32, 2115404u32);
    emu.sli_no_count(13usize, 22usize, 26u32, 2115408u32);
    emu.orr_no_count(13usize, 13usize, 11usize, 2115412u32);
    emu.sri_no_count(11usize, 22usize, 11u32, 2115416u32);
    emu.sli_no_count(17usize, 22usize, 21u32, 2115420u32);
    emu.orr_no_count(17usize, 17usize, 11usize, 2115424u32);
    emu.sri_no_count(11usize, 22usize, 25u32, 2115428u32);
    emu.sli_no_count(6usize, 22usize, 7u32, 2115432u32);
    emu.orr_no_count(6usize, 6usize, 11usize, 2115436u32);
    emu.adr_no_count(24usize, 15usize, 31usize, 2115440u32);
    emu.xrr_no_count(11usize, 10usize, 15usize, 2115444u32);
    emu.anr_no_count(11usize, 22usize, 11usize, 2115448u32);
    emu.xrr_no_count(28usize, 11usize, 15usize, 2115452u32);
    emu.sri_no_count(11usize, 29usize, 2u32, 2115456u32);
    emu.sli_no_count(30usize, 29usize, 30u32, 2115460u32);
    emu.orr_no_count(30usize, 30usize, 11usize, 2115464u32);
    emu.sri_no_count(11usize, 29usize, 13u32, 2115468u32);
    emu.sli_no_count(31usize, 29usize, 19u32, 2115472u32);
    emu.orr_no_count(31usize, 31usize, 11usize, 2115476u32);
    emu.sri_no_count(11usize, 29usize, 22u32, 2115480u32);
    emu.sli_no_count(9usize, 29usize, 10u32, 2115484u32);
    emu.orr_no_count(9usize, 9usize, 11usize, 2115488u32);
    emu.xrr_no_count(11usize, 27usize, 26usize, 2115492u32);
    emu.anr_no_count(11usize, 29usize, 11usize, 2115496u32);
    emu.anr_no_count(18usize, 27usize, 26usize, 2115500u32);
    emu.xrr_no_count(11usize, 11usize, 18usize, 2115504u32);
    emu.adr_no_count(18usize, 12usize, 7usize, 2115508u32);
    emu.adr_no_count(1usize, 1usize, 16usize, 2115512u32);
    emu.xrr_no_count(13usize, 13usize, 17usize, 2115516u32);
    emu.lw_no_count(12usize, 2usize, 500u32, 2115520u32)?;
    emu.adr_no_count(12usize, 21usize, 12usize, 2115524u32);
    emu.adr_no_count(12usize, 12usize, 28usize, 2115528u32);
    emu.xrr_no_count(16usize, 30usize, 31usize, 2115532u32);
    emu.xrr_no_count(31usize, 13usize, 6usize, 2115536u32);
    emu.xrr_no_count(7usize, 16usize, 9usize, 2115540u32);
    emu.adi_no_count(6usize, 18usize, 0u32, 2115544u32);
    emu.sri_no_count(13usize, 18usize, 17u32, 2115548u32);
    emu.sli_no_count(16usize, 18usize, 15u32, 2115552u32);
    emu.orr_no_count(18usize, 16usize, 13usize, 2115556u32);
    emu.sri_no_count(13usize, 6usize, 19u32, 2115560u32);
    emu.sli_no_count(16usize, 6usize, 13u32, 2115564u32);
    emu.orr_no_count(20usize, 16usize, 13usize, 2115568u32);
    emu.sri_no_count(13usize, 1usize, 17u32, 2115572u32);
    emu.sli_no_count(16usize, 1usize, 15u32, 2115576u32);
    emu.orr_no_count(21usize, 16usize, 13usize, 2115580u32);
    emu.sri_no_count(13usize, 1usize, 19u32, 2115584u32);
    emu.sli_no_count(16usize, 1usize, 13u32, 2115588u32);
    emu.orr_no_count(25usize, 16usize, 13usize, 2115592u32);
    emu.sri_no_count(13usize, 6usize, 7u32, 2115596u32);
    emu.sli_no_count(16usize, 6usize, 25u32, 2115600u32);
    emu.orr_no_count(17usize, 16usize, 13usize, 2115604u32);
    emu.sri_no_count(13usize, 6usize, 18u32, 2115608u32);
    emu.sli_no_count(16usize, 6usize, 14u32, 2115612u32);
    emu.adi_no_count(30usize, 6usize, 0u32, 2115616u32);
    emu.orr_no_count(6usize, 16usize, 13usize, 2115620u32);
    emu.sri_no_count(13usize, 1usize, 7u32, 2115624u32);
    emu.sli_no_count(16usize, 1usize, 25u32, 2115628u32);
    emu.orr_no_count(13usize, 16usize, 13usize, 2115632u32);
    emu.sri_no_count(16usize, 1usize, 18u32, 2115636u32);
    emu.sli_no_count(28usize, 1usize, 14u32, 2115640u32);
    emu.sw_no_count(1usize, 2usize, 444u32, 2115644u32)?;
    emu.orr_no_count(16usize, 28usize, 16usize, 2115648u32);
    emu.lw_no_count(28usize, 2usize, 252u32, 2115652u32)?;
    emu.adr_no_count(12usize, 12usize, 28usize, 2115656u32);
    emu.adr_no_count(12usize, 12usize, 31usize, 2115660u32);
    emu.adr_no_count(11usize, 7usize, 11usize, 2115664u32);
    emu.xrr_no_count(7usize, 18usize, 20usize, 2115668u32);
    emu.xrr_no_count(28usize, 21usize, 25usize, 2115672u32);
    emu.xrr_no_count(17usize, 17usize, 6usize, 2115676u32);
    emu.xrr_no_count(13usize, 13usize, 16usize, 2115680u32);
    emu.sw_no_count(30usize, 2usize, 504u32, 2115684u32)?;
    emu.sri_no_count(16usize, 30usize, 10u32, 2115688u32);
    emu.xrr_no_count(16usize, 7usize, 16usize, 2115692u32);
    emu.sri_no_count(6usize, 1usize, 10u32, 2115696u32);
    emu.xrr_no_count(6usize, 28usize, 6usize, 2115700u32);
    emu.sri_no_count(7usize, 30usize, 3u32, 2115704u32);
    emu.xrr_no_count(14usize, 17usize, 7usize, 2115708u32);
    emu.sw_no_count(14usize, 2usize, 332u32, 2115712u32)?;
    emu.sri_no_count(17usize, 1usize, 3u32, 2115716u32);
    emu.xrr_no_count(13usize, 13usize, 17usize, 2115720u32);
    emu.sw_no_count(13usize, 2usize, 376u32, 2115724u32)?;
    emu.adr_no_count(23usize, 23usize, 16usize, 2115728u32);
    emu.lw_no_count(15usize, 2usize, 476u32, 2115732u32)?;
    emu.lw_no_count(13usize, 2usize, 468u32, 2115736u32)?;
    emu.adr_no_count(15usize, 15usize, 13usize, 2115740u32);
    emu.adr_no_count(15usize, 15usize, 5usize, 2115744u32);
    emu.adr_no_count(5usize, 15usize, 6usize, 2115748u32);
    emu.adr_no_count(1usize, 11usize, 12usize, 2115752u32);
    emu.adr_no_count(20usize, 12usize, 8usize, 2115756u32);
    emu.sri_no_count(11usize, 20usize, 6u32, 2115760u32);
    emu.sli_no_count(12usize, 20usize, 26u32, 2115764u32);
    emu.orr_no_count(11usize, 12usize, 11usize, 2115768u32);
    emu.sri_no_count(12usize, 20usize, 11u32, 2115772u32);
    emu.sli_no_count(13usize, 20usize, 21u32, 2115776u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2115780u32);
    emu.sri_no_count(13usize, 20usize, 25u32, 2115784u32);
    emu.sli_no_count(15usize, 20usize, 7u32, 2115788u32);
    emu.orr_no_count(13usize, 15usize, 13usize, 2115792u32);
    emu.lw_no_count(7usize, 2usize, 472u32, 2115796u32)?;
    emu.adr_no_count(7usize, 7usize, 10usize, 2115800u32);
    emu.xrr_no_count(15usize, 22usize, 10usize, 2115804u32);
    emu.anr_no_count(15usize, 20usize, 15usize, 2115808u32);
    emu.xrr_no_count(10usize, 15usize, 10usize, 2115812u32);
    emu.sri_no_count(15usize, 1usize, 2u32, 2115816u32);
    emu.sli_no_count(16usize, 1usize, 30u32, 2115820u32);
    emu.orr_no_count(15usize, 16usize, 15usize, 2115824u32);
    emu.sri_no_count(16usize, 1usize, 13u32, 2115828u32);
    emu.sli_no_count(17usize, 1usize, 19u32, 2115832u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2115836u32);
    emu.sri_no_count(17usize, 1usize, 22u32, 2115840u32);
    emu.sli_no_count(6usize, 1usize, 10u32, 2115844u32);
    emu.orr_no_count(17usize, 6usize, 17usize, 2115848u32);
    emu.xrr_no_count(6usize, 29usize, 27usize, 2115852u32);
    emu.anr_no_count(6usize, 1usize, 6usize, 2115856u32);
    emu.anr_no_count(28usize, 29usize, 27usize, 2115860u32);
    emu.xrr_no_count(6usize, 6usize, 28usize, 2115864u32);
    emu.xrr_no_count(11usize, 11usize, 12usize, 2115868u32);
    emu.adr_no_count(10usize, 24usize, 10usize, 2115872u32);
    emu.xrr_no_count(12usize, 15usize, 16usize, 2115876u32);
    emu.adi_no_count(24usize, 23usize, 0u32, 2115880u32);
    emu.sri_no_count(15usize, 23usize, 17u32, 2115884u32);
    emu.sli_no_count(16usize, 23usize, 15u32, 2115888u32);
    emu.orr_no_count(15usize, 16usize, 15usize, 2115892u32);
    emu.sri_no_count(16usize, 23usize, 19u32, 2115896u32);
    emu.sli_no_count(28usize, 23usize, 13u32, 2115900u32);
    emu.orr_no_count(16usize, 28usize, 16usize, 2115904u32);
    emu.sri_no_count(28usize, 5usize, 17u32, 2115908u32);
    emu.sli_no_count(30usize, 5usize, 15u32, 2115912u32);
    emu.orr_no_count(28usize, 30usize, 28usize, 2115916u32);
    emu.sri_no_count(30usize, 5usize, 19u32, 2115920u32);
    emu.sli_no_count(31usize, 5usize, 13u32, 2115924u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2115928u32);
    emu.sri_no_count(31usize, 5usize, 7u32, 2115932u32);
    emu.sli_no_count(9usize, 5usize, 25u32, 2115936u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2115940u32);
    emu.sri_no_count(9usize, 23usize, 7u32, 2115944u32);
    emu.sli_no_count(18usize, 23usize, 25u32, 2115948u32);
    emu.orr_no_count(9usize, 18usize, 9usize, 2115952u32);
    emu.sri_no_count(18usize, 5usize, 18u32, 2115956u32);
    emu.sli_no_count(19usize, 5usize, 14u32, 2115960u32);
    emu.orr_no_count(18usize, 19usize, 18usize, 2115964u32);
    emu.sri_no_count(19usize, 23usize, 18u32, 2115968u32);
    emu.sli_no_count(23usize, 23usize, 14u32, 2115972u32);
    emu.orr_no_count(19usize, 23usize, 19usize, 2115976u32);
    emu.xrr_no_count(11usize, 11usize, 13usize, 2115980u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2115984u32);
    emu.xrr_no_count(13usize, 15usize, 16usize, 2115988u32);
    emu.xrr_no_count(15usize, 28usize, 30usize, 2115992u32);
    emu.xrr_no_count(16usize, 31usize, 18usize, 2115996u32);
    emu.xrr_no_count(17usize, 9usize, 19usize, 2116000u32);
    emu.lw_no_count(28usize, 2usize, 248u32, 2116004u32)?;
    emu.adr_no_count(10usize, 10usize, 28usize, 2116008u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2116012u32);
    emu.adr_no_count(12usize, 12usize, 6usize, 2116016u32);
    emu.sw_no_count(24usize, 2usize, 500u32, 2116020u32)?;
    emu.sri_no_count(11usize, 24usize, 10u32, 2116024u32);
    emu.xrr_no_count(11usize, 13usize, 11usize, 2116028u32);
    emu.sw_no_count(5usize, 2usize, 476u32, 2116032u32)?;
    emu.sri_no_count(13usize, 5usize, 10u32, 2116036u32);
    emu.xrr_no_count(13usize, 15usize, 13usize, 2116040u32);
    emu.sri_no_count(15usize, 5usize, 3u32, 2116044u32);
    emu.xrr_no_count(14usize, 16usize, 15usize, 2116048u32);
    emu.sw_no_count(14usize, 2usize, 384u32, 2116052u32)?;
    emu.sri_no_count(15usize, 24usize, 3u32, 2116056u32);
    emu.xrr_no_count(14usize, 17usize, 15usize, 2116060u32);
    emu.sw_no_count(14usize, 2usize, 324u32, 2116064u32)?;
    emu.adr_no_count(21usize, 12usize, 10usize, 2116068u32);
    emu.adr_no_count(19usize, 10usize, 26usize, 2116072u32);
    emu.lw_no_count(5usize, 2usize, 484u32, 2116076u32)?;
    emu.lw_no_count(10usize, 2usize, 404u32, 2116080u32)?;
    emu.adr_no_count(5usize, 10usize, 5usize, 2116084u32);
    emu.lw_no_count(10usize, 2usize, 492u32, 2116088u32)?;
    emu.adr_no_count(5usize, 5usize, 10usize, 2116092u32);
    emu.adr_no_count(24usize, 5usize, 11usize, 2116096u32);
    emu.lw_no_count(8usize, 2usize, 480u32, 2116100u32)?;
    emu.lw_no_count(10usize, 2usize, 392u32, 2116104u32)?;
    emu.adr_no_count(8usize, 10usize, 8usize, 2116108u32);
    emu.lw_no_count(10usize, 2usize, 496u32, 2116112u32)?;
    emu.adr_no_count(8usize, 8usize, 10usize, 2116116u32);
    emu.adr_no_count(26usize, 8usize, 13usize, 2116120u32);
    emu.sri_no_count(10usize, 19usize, 6u32, 2116124u32);
    emu.sli_no_count(13usize, 19usize, 26u32, 2116128u32);
    emu.orr_no_count(13usize, 13usize, 10usize, 2116132u32);
    emu.sri_no_count(10usize, 19usize, 11u32, 2116136u32);
    emu.sli_no_count(15usize, 19usize, 21u32, 2116140u32);
    emu.orr_no_count(15usize, 15usize, 10usize, 2116144u32);
    emu.sri_no_count(10usize, 19usize, 25u32, 2116148u32);
    emu.sli_no_count(11usize, 19usize, 7u32, 2116152u32);
    emu.orr_no_count(6usize, 11usize, 10usize, 2116156u32);
    emu.lw_no_count(10usize, 2usize, 464u32, 2116160u32)?;
    emu.adr_no_count(10usize, 10usize, 22usize, 2116164u32);
    emu.sw_no_count(10usize, 2usize, 464u32, 2116168u32)?;
    emu.xrr_no_count(10usize, 20usize, 22usize, 2116172u32);
    emu.anr_no_count(10usize, 19usize, 10usize, 2116176u32);
    emu.xrr_no_count(16usize, 10usize, 22usize, 2116180u32);
    emu.sri_no_count(10usize, 21usize, 2u32, 2116184u32);
    emu.sli_no_count(11usize, 21usize, 30u32, 2116188u32);
    emu.orr_no_count(31usize, 11usize, 10usize, 2116192u32);
    emu.sri_no_count(10usize, 21usize, 13u32, 2116196u32);
    emu.sli_no_count(11usize, 21usize, 19u32, 2116200u32);
    emu.orr_no_count(30usize, 11usize, 10usize, 2116204u32);
    emu.sri_no_count(10usize, 21usize, 22u32, 2116208u32);
    emu.sli_no_count(11usize, 21usize, 10u32, 2116212u32);
    emu.orr_no_count(5usize, 11usize, 10usize, 2116216u32);
    emu.xrr_no_count(10usize, 1usize, 29usize, 2116220u32);
    emu.anr_no_count(10usize, 21usize, 10usize, 2116224u32);
    emu.anr_no_count(11usize, 1usize, 29usize, 2116228u32);
    emu.xrr_no_count(8usize, 10usize, 11usize, 2116232u32);
    emu.sri_no_count(10usize, 24usize, 17u32, 2116236u32);
    emu.sli_no_count(11usize, 24usize, 15u32, 2116240u32);
    emu.orr_no_count(18usize, 11usize, 10usize, 2116244u32);
    emu.sri_no_count(10usize, 24usize, 19u32, 2116248u32);
    emu.sli_no_count(11usize, 24usize, 13u32, 2116252u32);
    emu.orr_no_count(22usize, 11usize, 10usize, 2116256u32);
    emu.sri_no_count(10usize, 26usize, 17u32, 2116260u32);
    emu.sli_no_count(11usize, 26usize, 15u32, 2116264u32);
    emu.orr_no_count(25usize, 11usize, 10usize, 2116268u32);
    emu.sri_no_count(10usize, 26usize, 19u32, 2116272u32);
    emu.sli_no_count(12usize, 26usize, 13u32, 2116276u32);
    emu.orr_no_count(12usize, 12usize, 10usize, 2116280u32);
    emu.sri_no_count(10usize, 24usize, 7u32, 2116284u32);
    emu.sli_no_count(11usize, 24usize, 25u32, 2116288u32);
    emu.orr_no_count(17usize, 11usize, 10usize, 2116292u32);
    emu.sri_no_count(10usize, 24usize, 18u32, 2116296u32);
    emu.sli_no_count(28usize, 24usize, 14u32, 2116300u32);
    emu.orr_no_count(11usize, 28usize, 10usize, 2116304u32);
    emu.adi_no_count(10usize, 26usize, 0u32, 2116308u32);
    emu.sri_no_count(28usize, 26usize, 7u32, 2116312u32);
    emu.sli_no_count(9usize, 26usize, 25u32, 2116316u32);
    emu.orr_no_count(28usize, 9usize, 28usize, 2116320u32);
    emu.sri_no_count(9usize, 26usize, 18u32, 2116324u32);
    emu.sli_no_count(26usize, 26usize, 14u32, 2116328u32);
    emu.adi_no_count(23usize, 10usize, 0u32, 2116332u32);
    emu.orr_no_count(9usize, 26usize, 9usize, 2116336u32);
    emu.xrr_no_count(13usize, 13usize, 15usize, 2116340u32);
    emu.adr_no_count(16usize, 7usize, 16usize, 2116344u32);
    emu.xrr_no_count(15usize, 31usize, 30usize, 2116348u32);
    emu.xrr_no_count(7usize, 18usize, 22usize, 2116352u32);
    emu.xrr_no_count(12usize, 25usize, 12usize, 2116356u32);
    emu.xrr_no_count(10usize, 17usize, 11usize, 2116360u32);
    emu.xrr_no_count(11usize, 28usize, 9usize, 2116364u32);
    emu.xrr_no_count(13usize, 13usize, 6usize, 2116368u32);
    emu.xrr_no_count(15usize, 15usize, 5usize, 2116372u32);
    emu.sri_no_count(5usize, 24usize, 10u32, 2116376u32);
    emu.xrr_no_count(5usize, 7usize, 5usize, 2116380u32);
    emu.sw_no_count(23usize, 2usize, 472u32, 2116384u32)?;
    emu.sri_no_count(6usize, 23usize, 10u32, 2116388u32);
    emu.xrr_no_count(12usize, 12usize, 6usize, 2116392u32);
    emu.sri_no_count(6usize, 24usize, 3u32, 2116396u32);
    emu.sw_no_count(24usize, 2usize, 404u32, 2116400u32)?;
    emu.xrr_no_count(10usize, 10usize, 6usize, 2116404u32);
    emu.sw_no_count(10usize, 2usize, 340u32, 2116408u32)?;
    emu.sri_no_count(10usize, 23usize, 3u32, 2116412u32);
    emu.xrr_no_count(10usize, 11usize, 10usize, 2116416u32);
    emu.sw_no_count(10usize, 2usize, 360u32, 2116420u32)?;
    emu.lw_no_count(10usize, 2usize, 244u32, 2116424u32)?;
    emu.adr_no_count(16usize, 16usize, 10usize, 2116428u32);
    emu.adr_no_count(13usize, 16usize, 13usize, 2116432u32);
    emu.adr_no_count(10usize, 15usize, 8usize, 2116436u32);
    emu.lw_no_count(11usize, 2usize, 448u32, 2116440u32)?;
    emu.lw_no_count(14usize, 2usize, 400u32, 2116444u32)?;
    emu.adr_no_count(11usize, 14usize, 11usize, 2116448u32);
    emu.lw_no_count(14usize, 2usize, 440u32, 2116452u32)?;
    emu.adr_no_count(11usize, 11usize, 14usize, 2116456u32);
    emu.adr_no_count(28usize, 11usize, 5usize, 2116460u32);
    emu.lw_no_count(11usize, 2usize, 452u32, 2116464u32)?;
    emu.lw_no_count(14usize, 2usize, 460u32, 2116468u32)?;
    emu.adr_no_count(11usize, 14usize, 11usize, 2116472u32);
    emu.lw_no_count(15usize, 2usize, 504u32, 2116476u32)?;
    emu.adr_no_count(11usize, 11usize, 15usize, 2116480u32);
    emu.adr_no_count(14usize, 11usize, 12usize, 2116484u32);
    emu.adr_no_count(23usize, 10usize, 13usize, 2116488u32);
    emu.adr_no_count(27usize, 13usize, 27usize, 2116492u32);
    emu.sri_no_count(11usize, 28usize, 17u32, 2116496u32);
    emu.sli_no_count(12usize, 28usize, 15u32, 2116500u32);
    emu.orr_no_count(11usize, 12usize, 11usize, 2116504u32);
    emu.sri_no_count(12usize, 28usize, 19u32, 2116508u32);
    emu.sli_no_count(13usize, 28usize, 13u32, 2116512u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2116516u32);
    emu.sri_no_count(13usize, 14usize, 17u32, 2116520u32);
    emu.sli_no_count(15usize, 14usize, 15u32, 2116524u32);
    emu.orr_no_count(13usize, 15usize, 13usize, 2116528u32);
    emu.sri_no_count(15usize, 14usize, 19u32, 2116532u32);
    emu.sli_no_count(16usize, 14usize, 13u32, 2116536u32);
    emu.orr_no_count(15usize, 16usize, 15usize, 2116540u32);
    emu.sri_no_count(16usize, 14usize, 7u32, 2116544u32);
    emu.sli_no_count(17usize, 14usize, 25u32, 2116548u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2116552u32);
    emu.sri_no_count(17usize, 28usize, 7u32, 2116556u32);
    emu.sli_no_count(5usize, 28usize, 25u32, 2116560u32);
    emu.orr_no_count(5usize, 5usize, 17usize, 2116564u32);
    emu.sri_no_count(17usize, 14usize, 18u32, 2116568u32);
    emu.sli_no_count(6usize, 14usize, 14u32, 2116572u32);
    emu.orr_no_count(6usize, 6usize, 17usize, 2116576u32);
    emu.sri_no_count(17usize, 28usize, 18u32, 2116580u32);
    emu.sli_no_count(7usize, 28usize, 14u32, 2116584u32);
    emu.adi_no_count(10usize, 28usize, 0u32, 2116588u32);
    emu.orr_no_count(7usize, 7usize, 17usize, 2116592u32);
    emu.sri_no_count(17usize, 27usize, 6u32, 2116596u32);
    emu.sli_no_count(28usize, 27usize, 26u32, 2116600u32);
    emu.orr_no_count(28usize, 28usize, 17usize, 2116604u32);
    emu.sri_no_count(17usize, 27usize, 11u32, 2116608u32);
    emu.sli_no_count(30usize, 27usize, 21u32, 2116612u32);
    emu.orr_no_count(30usize, 30usize, 17usize, 2116616u32);
    emu.sri_no_count(17usize, 27usize, 25u32, 2116620u32);
    emu.sli_no_count(31usize, 27usize, 7u32, 2116624u32);
    emu.orr_no_count(31usize, 31usize, 17usize, 2116628u32);
    emu.lw_no_count(17usize, 2usize, 468u32, 2116632u32)?;
    emu.adr_no_count(17usize, 17usize, 20usize, 2116636u32);
    emu.xrr_no_count(9usize, 19usize, 20usize, 2116640u32);
    emu.anr_no_count(9usize, 27usize, 9usize, 2116644u32);
    emu.xrr_no_count(9usize, 9usize, 20usize, 2116648u32);
    emu.sri_no_count(18usize, 23usize, 2u32, 2116652u32);
    emu.sli_no_count(20usize, 23usize, 30u32, 2116656u32);
    emu.orr_no_count(18usize, 20usize, 18usize, 2116660u32);
    emu.sri_no_count(20usize, 23usize, 13u32, 2116664u32);
    emu.sli_no_count(22usize, 23usize, 19u32, 2116668u32);
    emu.orr_no_count(20usize, 22usize, 20usize, 2116672u32);
    emu.sri_no_count(22usize, 23usize, 22u32, 2116676u32);
    emu.sli_no_count(25usize, 23usize, 10u32, 2116680u32);
    emu.orr_no_count(22usize, 25usize, 22usize, 2116684u32);
    emu.xrr_no_count(25usize, 21usize, 1usize, 2116688u32);
    emu.anr_no_count(25usize, 23usize, 25usize, 2116692u32);
    emu.anr_no_count(26usize, 21usize, 1usize, 2116696u32);
    emu.xrr_no_count(25usize, 25usize, 26usize, 2116700u32);
    emu.xrr_no_count(11usize, 11usize, 12usize, 2116704u32);
    emu.xrr_no_count(13usize, 13usize, 15usize, 2116708u32);
    emu.xrr_no_count(12usize, 16usize, 6usize, 2116712u32);
    emu.xrr_no_count(15usize, 5usize, 7usize, 2116716u32);
    emu.xrr_no_count(16usize, 28usize, 30usize, 2116720u32);
    emu.lw_no_count(8usize, 2usize, 464u32, 2116724u32)?;
    emu.adr_no_count(8usize, 8usize, 9usize, 2116728u32);
    emu.xrr_no_count(5usize, 18usize, 20usize, 2116732u32);
    emu.adi_no_count(18usize, 10usize, 0u32, 2116736u32);
    emu.sri_no_count(6usize, 10usize, 10u32, 2116740u32);
    emu.xrr_no_count(11usize, 11usize, 6usize, 2116744u32);
    emu.sw_no_count(14usize, 2usize, 460u32, 2116748u32)?;
    emu.sri_no_count(6usize, 14usize, 10u32, 2116752u32);
    emu.xrr_no_count(13usize, 13usize, 6usize, 2116756u32);
    emu.sri_no_count(6usize, 14usize, 3u32, 2116760u32);
    emu.xrr_no_count(10usize, 12usize, 6usize, 2116764u32);
    emu.sw_no_count(10usize, 2usize, 380u32, 2116768u32)?;
    emu.sri_no_count(12usize, 18usize, 3u32, 2116772u32);
    emu.sw_no_count(18usize, 2usize, 400u32, 2116776u32)?;
    emu.xrr_no_count(12usize, 15usize, 12usize, 2116780u32);
    emu.sw_no_count(12usize, 2usize, 320u32, 2116784u32)?;
    emu.xrr_no_count(12usize, 16usize, 31usize, 2116788u32);
    emu.xrr_no_count(15usize, 5usize, 22usize, 2116792u32);
    emu.lw_no_count(16usize, 2usize, 488u32, 2116796u32)?;
    emu.lw_no_count(10usize, 2usize, 396u32, 2116800u32)?;
    emu.adr_no_count(16usize, 10usize, 16usize, 2116804u32);
    emu.lw_no_count(10usize, 2usize, 444u32, 2116808u32)?;
    emu.adr_no_count(16usize, 16usize, 10usize, 2116812u32);
    emu.adr_no_count(22usize, 16usize, 11usize, 2116816u32);
    emu.lw_no_count(11usize, 2usize, 412u32, 2116820u32)?;
    emu.lw_no_count(14usize, 2usize, 456u32, 2116824u32)?;
    emu.adr_no_count(11usize, 14usize, 11usize, 2116828u32);
    emu.lw_no_count(14usize, 2usize, 500u32, 2116832u32)?;
    emu.adr_no_count(11usize, 11usize, 14usize, 2116836u32);
    emu.adr_no_count(28usize, 11usize, 13usize, 2116840u32);
    emu.lw_no_count(11usize, 2usize, 240u32, 2116844u32)?;
    emu.adr_no_count(8usize, 8usize, 11usize, 2116848u32);
    emu.adr_no_count(12usize, 8usize, 12usize, 2116852u32);
    emu.adr_no_count(11usize, 15usize, 25usize, 2116856u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2116860u32);
    emu.adr_no_count(6usize, 12usize, 29usize, 2116864u32);
    emu.sri_no_count(12usize, 22usize, 17u32, 2116868u32);
    emu.sli_no_count(13usize, 22usize, 15u32, 2116872u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2116876u32);
    emu.sri_no_count(13usize, 22usize, 19u32, 2116880u32);
    emu.sli_no_count(14usize, 22usize, 13u32, 2116884u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2116888u32);
    emu.sri_no_count(14usize, 28usize, 17u32, 2116892u32);
    emu.sli_no_count(15usize, 28usize, 15u32, 2116896u32);
    emu.orr_no_count(14usize, 15usize, 14usize, 2116900u32);
    emu.sri_no_count(15usize, 28usize, 19u32, 2116904u32);
    emu.sli_no_count(16usize, 28usize, 13u32, 2116908u32);
    emu.orr_no_count(15usize, 16usize, 15usize, 2116912u32);
    emu.sri_no_count(16usize, 22usize, 7u32, 2116916u32);
    emu.sli_no_count(5usize, 22usize, 25u32, 2116920u32);
    emu.orr_no_count(16usize, 5usize, 16usize, 2116924u32);
    emu.sri_no_count(5usize, 22usize, 18u32, 2116928u32);
    emu.sli_no_count(7usize, 22usize, 14u32, 2116932u32);
    emu.orr_no_count(5usize, 7usize, 5usize, 2116936u32);
    emu.adi_no_count(29usize, 28usize, 0u32, 2116940u32);
    emu.sri_no_count(7usize, 28usize, 7u32, 2116944u32);
    emu.sli_no_count(28usize, 28usize, 25u32, 2116948u32);
    emu.orr_no_count(7usize, 28usize, 7usize, 2116952u32);
    emu.sri_no_count(28usize, 29usize, 18u32, 2116956u32);
    emu.sli_no_count(30usize, 29usize, 14u32, 2116960u32);
    emu.orr_no_count(28usize, 30usize, 28usize, 2116964u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2116968u32);
    emu.xrr_no_count(15usize, 14usize, 15usize, 2116972u32);
    emu.xrr_no_count(13usize, 16usize, 5usize, 2116976u32);
    emu.xrr_no_count(16usize, 7usize, 28usize, 2116980u32);
    emu.sri_no_count(14usize, 6usize, 6u32, 2116984u32);
    emu.sli_no_count(5usize, 6usize, 26u32, 2116988u32);
    emu.orr_no_count(5usize, 5usize, 14usize, 2116992u32);
    emu.sri_no_count(14usize, 6usize, 11u32, 2116996u32);
    emu.sli_no_count(7usize, 6usize, 21u32, 2117000u32);
    emu.orr_no_count(7usize, 7usize, 14usize, 2117004u32);
    emu.sri_no_count(14usize, 6usize, 25u32, 2117008u32);
    emu.sli_no_count(28usize, 6usize, 7u32, 2117012u32);
    emu.orr_no_count(28usize, 28usize, 14usize, 2117016u32);
    emu.lw_no_count(14usize, 2usize, 484u32, 2117020u32)?;
    emu.adr_no_count(14usize, 14usize, 19usize, 2117024u32);
    emu.xrr_no_count(30usize, 27usize, 19usize, 2117028u32);
    emu.anr_no_count(30usize, 6usize, 30usize, 2117032u32);
    emu.xrr_no_count(30usize, 30usize, 19usize, 2117036u32);
    emu.sri_no_count(31usize, 11usize, 2u32, 2117040u32);
    emu.sli_no_count(8usize, 11usize, 30u32, 2117044u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2117048u32);
    emu.sri_no_count(8usize, 11usize, 13u32, 2117052u32);
    emu.sli_no_count(9usize, 11usize, 19u32, 2117056u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2117060u32);
    emu.sri_no_count(9usize, 11usize, 22u32, 2117064u32);
    emu.sli_no_count(19usize, 11usize, 10u32, 2117068u32);
    emu.orr_no_count(9usize, 19usize, 9usize, 2117072u32);
    emu.xrr_no_count(19usize, 23usize, 21usize, 2117076u32);
    emu.anr_no_count(19usize, 11usize, 19usize, 2117080u32);
    emu.anr_no_count(20usize, 23usize, 21usize, 2117084u32);
    emu.xrr_no_count(19usize, 19usize, 20usize, 2117088u32);
    emu.sri_no_count(20usize, 22usize, 10u32, 2117092u32);
    emu.xrr_no_count(12usize, 12usize, 20usize, 2117096u32);
    emu.sw_no_count(29usize, 2usize, 456u32, 2117100u32)?;
    emu.sri_no_count(20usize, 29usize, 10u32, 2117104u32);
    emu.xrr_no_count(15usize, 15usize, 20usize, 2117108u32);
    emu.sri_no_count(20usize, 22usize, 3u32, 2117112u32);
    emu.sw_no_count(22usize, 2usize, 396u32, 2117116u32)?;
    emu.xrr_no_count(10usize, 13usize, 20usize, 2117120u32);
    emu.sw_no_count(10usize, 2usize, 336u32, 2117124u32)?;
    emu.sri_no_count(13usize, 29usize, 3u32, 2117128u32);
    emu.xrr_no_count(10usize, 16usize, 13usize, 2117132u32);
    emu.sw_no_count(10usize, 2usize, 356u32, 2117136u32)?;
    emu.xrr_no_count(13usize, 5usize, 7usize, 2117140u32);
    emu.adr_no_count(17usize, 17usize, 30usize, 2117144u32);
    emu.xrr_no_count(16usize, 31usize, 8usize, 2117148u32);
    emu.lw_no_count(5usize, 2usize, 416u32, 2117152u32)?;
    emu.lw_no_count(10usize, 2usize, 436u32, 2117156u32)?;
    emu.adr_no_count(5usize, 10usize, 5usize, 2117160u32);
    emu.lw_no_count(10usize, 2usize, 476u32, 2117164u32)?;
    emu.adr_no_count(5usize, 5usize, 10usize, 2117168u32);
    emu.adr_no_count(26usize, 5usize, 12usize, 2117172u32);
    emu.lw_no_count(12usize, 2usize, 420u32, 2117176u32)?;
    emu.lw_no_count(29usize, 2usize, 364u32, 2117180u32)?;
    emu.adr_no_count(29usize, 29usize, 12usize, 2117184u32);
    emu.adr_no_count(29usize, 29usize, 24usize, 2117188u32);
    emu.adr_no_count(10usize, 29usize, 15usize, 2117192u32);
    emu.xrr_no_count(12usize, 13usize, 28usize, 2117196u32);
    emu.xrr_no_count(13usize, 16usize, 9usize, 2117200u32);
    emu.lw_no_count(15usize, 2usize, 236u32, 2117204u32)?;
    emu.adr_no_count(17usize, 17usize, 15usize, 2117208u32);
    emu.adr_no_count(12usize, 17usize, 12usize, 2117212u32);
    emu.adr_no_count(13usize, 13usize, 19usize, 2117216u32);
    emu.sri_no_count(15usize, 26usize, 17u32, 2117220u32);
    emu.sli_no_count(16usize, 26usize, 15u32, 2117224u32);
    emu.orr_no_count(15usize, 16usize, 15usize, 2117228u32);
    emu.sri_no_count(16usize, 26usize, 19u32, 2117232u32);
    emu.sli_no_count(17usize, 26usize, 13u32, 2117236u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2117240u32);
    emu.sri_no_count(17usize, 10usize, 17u32, 2117244u32);
    emu.sli_no_count(5usize, 10usize, 15u32, 2117248u32);
    emu.orr_no_count(17usize, 5usize, 17usize, 2117252u32);
    emu.sri_no_count(5usize, 10usize, 19u32, 2117256u32);
    emu.sli_no_count(7usize, 10usize, 13u32, 2117260u32);
    emu.orr_no_count(5usize, 7usize, 5usize, 2117264u32);
    emu.sri_no_count(7usize, 10usize, 7u32, 2117268u32);
    emu.sli_no_count(28usize, 10usize, 25u32, 2117272u32);
    emu.orr_no_count(7usize, 28usize, 7usize, 2117276u32);
    emu.sri_no_count(28usize, 26usize, 7u32, 2117280u32);
    emu.sli_no_count(29usize, 26usize, 25u32, 2117284u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2117288u32);
    emu.sri_no_count(29usize, 10usize, 18u32, 2117292u32);
    emu.sli_no_count(30usize, 10usize, 14u32, 2117296u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2117300u32);
    emu.sri_no_count(30usize, 26usize, 18u32, 2117304u32);
    emu.sli_no_count(31usize, 26usize, 14u32, 2117308u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2117312u32);
    emu.adr_no_count(25usize, 13usize, 12usize, 2117316u32);
    emu.adr_no_count(1usize, 12usize, 1usize, 2117320u32);
    emu.xrr_no_count(12usize, 15usize, 16usize, 2117324u32);
    emu.xrr_no_count(13usize, 17usize, 5usize, 2117328u32);
    emu.xrr_no_count(15usize, 7usize, 29usize, 2117332u32);
    emu.xrr_no_count(16usize, 28usize, 30usize, 2117336u32);
    emu.sri_no_count(17usize, 26usize, 10u32, 2117340u32);
    emu.xrr_no_count(17usize, 12usize, 17usize, 2117344u32);
    emu.sw_no_count(10usize, 2usize, 436u32, 2117348u32)?;
    emu.sri_no_count(12usize, 10usize, 10u32, 2117352u32);
    emu.xrr_no_count(5usize, 13usize, 12usize, 2117356u32);
    emu.sri_no_count(12usize, 10usize, 3u32, 2117360u32);
    emu.xrr_no_count(12usize, 15usize, 12usize, 2117364u32);
    emu.sw_no_count(12usize, 2usize, 364u32, 2117368u32)?;
    emu.sri_no_count(12usize, 26usize, 3u32, 2117372u32);
    emu.sw_no_count(26usize, 2usize, 392u32, 2117376u32)?;
    emu.xrr_no_count(10usize, 16usize, 12usize, 2117380u32);
    emu.sw_no_count(10usize, 2usize, 344u32, 2117384u32)?;
    emu.sri_no_count(12usize, 1usize, 6u32, 2117388u32);
    emu.sli_no_count(13usize, 1usize, 26u32, 2117392u32);
    emu.orr_no_count(15usize, 13usize, 12usize, 2117396u32);
    emu.sri_no_count(12usize, 1usize, 11u32, 2117400u32);
    emu.sli_no_count(13usize, 1usize, 21u32, 2117404u32);
    emu.orr_no_count(16usize, 13usize, 12usize, 2117408u32);
    emu.sri_no_count(12usize, 1usize, 25u32, 2117412u32);
    emu.sli_no_count(13usize, 1usize, 7u32, 2117416u32);
    emu.orr_no_count(7usize, 13usize, 12usize, 2117420u32);
    emu.lw_no_count(12usize, 2usize, 480u32, 2117424u32)?;
    emu.adr_no_count(12usize, 12usize, 27usize, 2117428u32);
    emu.xrr_no_count(13usize, 6usize, 27usize, 2117432u32);
    emu.anr_no_count(13usize, 1usize, 13usize, 2117436u32);
    emu.xrr_no_count(28usize, 13usize, 27usize, 2117440u32);
    emu.sri_no_count(13usize, 25usize, 2u32, 2117444u32);
    emu.sli_no_count(29usize, 25usize, 30u32, 2117448u32);
    emu.orr_no_count(29usize, 29usize, 13usize, 2117452u32);
    emu.sri_no_count(13usize, 25usize, 13u32, 2117456u32);
    emu.sli_no_count(30usize, 25usize, 19u32, 2117460u32);
    emu.orr_no_count(30usize, 30usize, 13usize, 2117464u32);
    emu.sri_no_count(13usize, 25usize, 22u32, 2117468u32);
    emu.sli_no_count(31usize, 25usize, 10u32, 2117472u32);
    emu.orr_no_count(31usize, 31usize, 13usize, 2117476u32);
    emu.xrr_no_count(13usize, 11usize, 23usize, 2117480u32);
    emu.anr_no_count(13usize, 25usize, 13usize, 2117484u32);
    emu.anr_no_count(8usize, 11usize, 23usize, 2117488u32);
    emu.xrr_no_count(13usize, 13usize, 8usize, 2117492u32);
    emu.lw_no_count(8usize, 2usize, 408u32, 2117496u32)?;
    emu.lw_no_count(10usize, 2usize, 352u32, 2117500u32)?;
    emu.adr_no_count(8usize, 10usize, 8usize, 2117504u32);
    emu.lw_no_count(10usize, 2usize, 472u32, 2117508u32)?;
    emu.adr_no_count(8usize, 8usize, 10usize, 2117512u32);
    emu.adr_no_count(8usize, 8usize, 17usize, 2117516u32);
    emu.lw_no_count(17usize, 2usize, 492u32, 2117520u32)?;
    emu.lw_no_count(10usize, 2usize, 348u32, 2117524u32)?;
    emu.adr_no_count(17usize, 10usize, 17usize, 2117528u32);
    emu.adr_no_count(17usize, 17usize, 18usize, 2117532u32);
    emu.adr_no_count(10usize, 17usize, 5usize, 2117536u32);
    emu.xrr_no_count(15usize, 15usize, 16usize, 2117540u32);
    emu.adr_no_count(14usize, 14usize, 28usize, 2117544u32);
    emu.xrr_no_count(17usize, 29usize, 30usize, 2117548u32);
    emu.xrr_no_count(16usize, 15usize, 7usize, 2117552u32);
    emu.xrr_no_count(17usize, 17usize, 31usize, 2117556u32);
    emu.sri_no_count(15usize, 8usize, 17u32, 2117560u32);
    emu.sli_no_count(5usize, 8usize, 15u32, 2117564u32);
    emu.orr_no_count(5usize, 5usize, 15usize, 2117568u32);
    emu.sri_no_count(15usize, 8usize, 19u32, 2117572u32);
    emu.sli_no_count(7usize, 8usize, 13u32, 2117576u32);
    emu.orr_no_count(7usize, 7usize, 15usize, 2117580u32);
    emu.sri_no_count(15usize, 10usize, 17u32, 2117584u32);
    emu.sli_no_count(28usize, 10usize, 15u32, 2117588u32);
    emu.orr_no_count(28usize, 28usize, 15usize, 2117592u32);
    emu.sri_no_count(15usize, 10usize, 19u32, 2117596u32);
    emu.sli_no_count(29usize, 10usize, 13u32, 2117600u32);
    emu.orr_no_count(29usize, 29usize, 15usize, 2117604u32);
    emu.sri_no_count(15usize, 8usize, 7u32, 2117608u32);
    emu.sli_no_count(30usize, 8usize, 25u32, 2117612u32);
    emu.orr_no_count(31usize, 30usize, 15usize, 2117616u32);
    emu.sri_no_count(15usize, 8usize, 18u32, 2117620u32);
    emu.sli_no_count(30usize, 8usize, 14u32, 2117624u32);
    emu.adi_no_count(18usize, 8usize, 0u32, 2117628u32);
    emu.orr_no_count(15usize, 30usize, 15usize, 2117632u32);
    emu.sri_no_count(30usize, 10usize, 7u32, 2117636u32);
    emu.sli_no_count(8usize, 10usize, 25u32, 2117640u32);
    emu.orr_no_count(30usize, 8usize, 30usize, 2117644u32);
    emu.sri_no_count(8usize, 10usize, 18u32, 2117648u32);
    emu.sli_no_count(9usize, 10usize, 14u32, 2117652u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2117656u32);
    emu.lw_no_count(9usize, 2usize, 232u32, 2117660u32)?;
    emu.adr_no_count(14usize, 14usize, 9usize, 2117664u32);
    emu.adr_no_count(16usize, 14usize, 16usize, 2117668u32);
    emu.adr_no_count(13usize, 17usize, 13usize, 2117672u32);
    emu.xrr_no_count(5usize, 5usize, 7usize, 2117676u32);
    emu.xrr_no_count(7usize, 28usize, 29usize, 2117680u32);
    emu.xrr_no_count(15usize, 31usize, 15usize, 2117684u32);
    emu.xrr_no_count(28usize, 30usize, 8usize, 2117688u32);
    emu.adr_no_count(14usize, 13usize, 16usize, 2117692u32);
    emu.adr_no_count(17usize, 16usize, 21usize, 2117696u32);
    emu.sw_no_count(18usize, 2usize, 468u32, 2117700u32)?;
    emu.sri_no_count(13usize, 18usize, 10u32, 2117704u32);
    emu.xrr_no_count(13usize, 5usize, 13usize, 2117708u32);
    emu.sw_no_count(10usize, 2usize, 484u32, 2117712u32)?;
    emu.sri_no_count(16usize, 10usize, 10u32, 2117716u32);
    emu.xrr_no_count(16usize, 7usize, 16usize, 2117720u32);
    emu.sri_no_count(5usize, 18usize, 3u32, 2117724u32);
    emu.xrr_no_count(15usize, 15usize, 5usize, 2117728u32);
    emu.sw_no_count(15usize, 2usize, 328u32, 2117732u32)?;
    emu.sri_no_count(15usize, 10usize, 3u32, 2117736u32);
    emu.xrr_no_count(10usize, 28usize, 15usize, 2117740u32);
    emu.sw_no_count(10usize, 2usize, 352u32, 2117744u32)?;
    emu.lw_no_count(15usize, 2usize, 496u32, 2117748u32)?;
    emu.lw_no_count(10usize, 2usize, 388u32, 2117752u32)?;
    emu.adr_no_count(15usize, 10usize, 15usize, 2117756u32);
    emu.lw_no_count(10usize, 2usize, 460u32, 2117760u32)?;
    emu.adr_no_count(15usize, 15usize, 10usize, 2117764u32);
    emu.adr_no_count(8usize, 15usize, 13usize, 2117768u32);
    emu.lw_no_count(10usize, 2usize, 332u32, 2117772u32)?;
    emu.lw_no_count(21usize, 2usize, 440u32, 2117776u32)?;
    emu.adr_no_count(21usize, 10usize, 21usize, 2117780u32);
    emu.adr_no_count(21usize, 21usize, 22usize, 2117784u32);
    emu.adr_no_count(18usize, 21usize, 16usize, 2117788u32);
    emu.sri_no_count(13usize, 17usize, 6u32, 2117792u32);
    emu.sli_no_count(15usize, 17usize, 26u32, 2117796u32);
    emu.orr_no_count(13usize, 15usize, 13usize, 2117800u32);
    emu.sri_no_count(15usize, 17usize, 11u32, 2117804u32);
    emu.sli_no_count(16usize, 17usize, 21u32, 2117808u32);
    emu.orr_no_count(15usize, 16usize, 15usize, 2117812u32);
    emu.sri_no_count(16usize, 17usize, 25u32, 2117816u32);
    emu.sli_no_count(5usize, 17usize, 7u32, 2117820u32);
    emu.orr_no_count(16usize, 5usize, 16usize, 2117824u32);
    emu.lw_no_count(5usize, 2usize, 448u32, 2117828u32)?;
    emu.adr_no_count(5usize, 5usize, 6usize, 2117832u32);
    emu.xrr_no_count(7usize, 1usize, 6usize, 2117836u32);
    emu.anr_no_count(7usize, 17usize, 7usize, 2117840u32);
    emu.xrr_no_count(6usize, 7usize, 6usize, 2117844u32);
    emu.sri_no_count(7usize, 14usize, 2u32, 2117848u32);
    emu.sli_no_count(28usize, 14usize, 30u32, 2117852u32);
    emu.orr_no_count(7usize, 28usize, 7usize, 2117856u32);
    emu.sri_no_count(28usize, 14usize, 13u32, 2117860u32);
    emu.sli_no_count(29usize, 14usize, 19u32, 2117864u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2117868u32);
    emu.sri_no_count(29usize, 14usize, 22u32, 2117872u32);
    emu.sli_no_count(30usize, 14usize, 10u32, 2117876u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2117880u32);
    emu.xrr_no_count(30usize, 25usize, 11usize, 2117884u32);
    emu.anr_no_count(30usize, 14usize, 30usize, 2117888u32);
    emu.anr_no_count(31usize, 25usize, 11usize, 2117892u32);
    emu.xrr_no_count(30usize, 30usize, 31usize, 2117896u32);
    emu.xrr_no_count(13usize, 13usize, 15usize, 2117900u32);
    emu.adr_no_count(12usize, 12usize, 6usize, 2117904u32);
    emu.xrr_no_count(15usize, 7usize, 28usize, 2117908u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2117912u32);
    emu.sri_no_count(6usize, 8usize, 17u32, 2117916u32);
    emu.sli_no_count(7usize, 8usize, 15u32, 2117920u32);
    emu.orr_no_count(6usize, 7usize, 6usize, 2117924u32);
    emu.sri_no_count(7usize, 8usize, 19u32, 2117928u32);
    emu.sli_no_count(28usize, 8usize, 13u32, 2117932u32);
    emu.orr_no_count(7usize, 28usize, 7usize, 2117936u32);
    emu.sri_no_count(28usize, 18usize, 17u32, 2117940u32);
    emu.sli_no_count(31usize, 18usize, 15u32, 2117944u32);
    emu.orr_no_count(28usize, 31usize, 28usize, 2117948u32);
    emu.sri_no_count(31usize, 18usize, 19u32, 2117952u32);
    emu.sli_no_count(8usize, 18usize, 13u32, 2117956u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2117960u32);
    emu.sri_no_count(8usize, 18usize, 7u32, 2117964u32);
    emu.sli_no_count(9usize, 18usize, 25u32, 2117968u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2117972u32);
    emu.sri_no_count(9usize, 10usize, 7u32, 2117976u32);
    emu.sli_no_count(19usize, 10usize, 25u32, 2117980u32);
    emu.orr_no_count(9usize, 19usize, 9usize, 2117984u32);
    emu.sri_no_count(19usize, 18usize, 18u32, 2117988u32);
    emu.sli_no_count(20usize, 18usize, 14u32, 2117992u32);
    emu.orr_no_count(19usize, 20usize, 19usize, 2117996u32);
    emu.sri_no_count(20usize, 10usize, 18u32, 2118000u32);
    emu.sli_no_count(21usize, 10usize, 14u32, 2118004u32);
    emu.orr_no_count(20usize, 21usize, 20usize, 2118008u32);
    emu.xrr_no_count(13usize, 13usize, 16usize, 2118012u32);
    emu.xrr_no_count(15usize, 15usize, 29usize, 2118016u32);
    emu.xrr_no_count(16usize, 6usize, 7usize, 2118020u32);
    emu.xrr_no_count(6usize, 28usize, 31usize, 2118024u32);
    emu.xrr_no_count(7usize, 8usize, 19usize, 2118028u32);
    emu.xrr_no_count(28usize, 9usize, 20usize, 2118032u32);
    emu.lw_no_count(29usize, 2usize, 228u32, 2118036u32)?;
    emu.adr_no_count(12usize, 12usize, 29usize, 2118040u32);
    emu.adr_no_count(13usize, 12usize, 13usize, 2118044u32);
    emu.adr_no_count(12usize, 15usize, 30usize, 2118048u32);
    emu.sw_no_count(10usize, 2usize, 464u32, 2118052u32)?;
    emu.sri_no_count(15usize, 10usize, 10u32, 2118056u32);
    emu.xrr_no_count(22usize, 16usize, 15usize, 2118060u32);
    emu.sw_no_count(18usize, 2usize, 480u32, 2118064u32)?;
    emu.sri_no_count(15usize, 18usize, 10u32, 2118068u32);
    emu.xrr_no_count(15usize, 6usize, 15usize, 2118072u32);
    emu.sri_no_count(16usize, 18usize, 3u32, 2118076u32);
    emu.xrr_no_count(16usize, 7usize, 16usize, 2118080u32);
    emu.sw_no_count(16usize, 2usize, 348u32, 2118084u32)?;
    emu.sri_no_count(16usize, 10usize, 3u32, 2118088u32);
    emu.xrr_no_count(10usize, 28usize, 16usize, 2118092u32);
    emu.sw_no_count(10usize, 2usize, 332u32, 2118096u32)?;
    emu.adr_no_count(18usize, 12usize, 13usize, 2118100u32);
    emu.adr_no_count(10usize, 13usize, 23usize, 2118104u32);
    emu.lw_no_count(13usize, 2usize, 504u32, 2118108u32)?;
    emu.lw_no_count(12usize, 2usize, 376u32, 2118112u32)?;
    emu.adr_no_count(13usize, 12usize, 13usize, 2118116u32);
    emu.lw_no_count(12usize, 2usize, 456u32, 2118120u32)?;
    emu.adr_no_count(13usize, 13usize, 12usize, 2118124u32);
    emu.adr_no_count(22usize, 13usize, 22usize, 2118128u32);
    emu.lw_no_count(12usize, 2usize, 324u32, 2118132u32)?;
    emu.lw_no_count(24usize, 2usize, 444u32, 2118136u32)?;
    emu.adr_no_count(24usize, 12usize, 24usize, 2118140u32);
    emu.adr_no_count(24usize, 24usize, 26usize, 2118144u32);
    emu.adr_no_count(12usize, 24usize, 15usize, 2118148u32);
    emu.sri_no_count(13usize, 10usize, 6u32, 2118152u32);
    emu.sli_no_count(15usize, 10usize, 26u32, 2118156u32);
    emu.orr_no_count(16usize, 15usize, 13usize, 2118160u32);
    emu.sri_no_count(13usize, 10usize, 11u32, 2118164u32);
    emu.sli_no_count(8usize, 10usize, 21u32, 2118168u32);
    emu.orr_no_count(8usize, 8usize, 13usize, 2118172u32);
    emu.sri_no_count(13usize, 10usize, 25u32, 2118176u32);
    emu.sli_no_count(15usize, 10usize, 7u32, 2118180u32);
    emu.orr_no_count(6usize, 15usize, 13usize, 2118184u32);
    emu.lw_no_count(15usize, 2usize, 452u32, 2118188u32)?;
    emu.adr_no_count(15usize, 15usize, 1usize, 2118192u32);
    emu.xrr_no_count(13usize, 17usize, 1usize, 2118196u32);
    emu.anr_no_count(13usize, 10usize, 13usize, 2118200u32);
    emu.xrr_no_count(28usize, 13usize, 1usize, 2118204u32);
    emu.sri_no_count(13usize, 18usize, 2u32, 2118208u32);
    emu.sli_no_count(7usize, 18usize, 30u32, 2118212u32);
    emu.orr_no_count(21usize, 7usize, 13usize, 2118216u32);
    emu.sri_no_count(13usize, 18usize, 13u32, 2118220u32);
    emu.sli_no_count(7usize, 18usize, 19u32, 2118224u32);
    emu.orr_no_count(23usize, 7usize, 13usize, 2118228u32);
    emu.sri_no_count(13usize, 18usize, 22u32, 2118232u32);
    emu.sli_no_count(7usize, 18usize, 10u32, 2118236u32);
    emu.orr_no_count(30usize, 7usize, 13usize, 2118240u32);
    emu.xrr_no_count(13usize, 14usize, 25usize, 2118244u32);
    emu.anr_no_count(13usize, 18usize, 13usize, 2118248u32);
    emu.anr_no_count(7usize, 14usize, 25usize, 2118252u32);
    emu.xrr_no_count(7usize, 13usize, 7usize, 2118256u32);
    emu.sri_no_count(13usize, 22usize, 17u32, 2118260u32);
    emu.sli_no_count(29usize, 22usize, 15u32, 2118264u32);
    emu.orr_no_count(24usize, 29usize, 13usize, 2118268u32);
    emu.sri_no_count(13usize, 22usize, 19u32, 2118272u32);
    emu.sli_no_count(29usize, 22usize, 13u32, 2118276u32);
    emu.orr_no_count(26usize, 29usize, 13usize, 2118280u32);
    emu.sri_no_count(13usize, 12usize, 17u32, 2118284u32);
    emu.sli_no_count(29usize, 12usize, 15u32, 2118288u32);
    emu.orr_no_count(27usize, 29usize, 13usize, 2118292u32);
    emu.sri_no_count(13usize, 12usize, 19u32, 2118296u32);
    emu.sli_no_count(29usize, 12usize, 13u32, 2118300u32);
    emu.orr_no_count(1usize, 29usize, 13usize, 2118304u32);
    emu.sri_no_count(13usize, 22usize, 7u32, 2118308u32);
    emu.sli_no_count(29usize, 22usize, 25u32, 2118312u32);
    emu.orr_no_count(13usize, 29usize, 13usize, 2118316u32);
    emu.sri_no_count(29usize, 22usize, 18u32, 2118320u32);
    emu.sli_no_count(31usize, 22usize, 14u32, 2118324u32);
    emu.orr_no_count(29usize, 31usize, 29usize, 2118328u32);
    emu.sri_no_count(31usize, 12usize, 7u32, 2118332u32);
    emu.sli_no_count(9usize, 12usize, 25u32, 2118336u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2118340u32);
    emu.sri_no_count(9usize, 12usize, 18u32, 2118344u32);
    emu.sli_no_count(19usize, 12usize, 14u32, 2118348u32);
    emu.orr_no_count(9usize, 19usize, 9usize, 2118352u32);
    emu.xrr_no_count(16usize, 16usize, 8usize, 2118356u32);
    emu.adr_no_count(5usize, 5usize, 28usize, 2118360u32);
    emu.xrr_no_count(28usize, 21usize, 23usize, 2118364u32);
    emu.xrr_no_count(8usize, 24usize, 26usize, 2118368u32);
    emu.xrr_no_count(19usize, 27usize, 1usize, 2118372u32);
    emu.xrr_no_count(13usize, 13usize, 29usize, 2118376u32);
    emu.xrr_no_count(29usize, 31usize, 9usize, 2118380u32);
    emu.xrr_no_count(16usize, 16usize, 6usize, 2118384u32);
    emu.xrr_no_count(6usize, 28usize, 30usize, 2118388u32);
    emu.sri_no_count(28usize, 22usize, 10u32, 2118392u32);
    emu.xrr_no_count(31usize, 8usize, 28usize, 2118396u32);
    emu.sw_no_count(12usize, 2usize, 448u32, 2118400u32)?;
    emu.sri_no_count(28usize, 12usize, 10u32, 2118404u32);
    emu.xrr_no_count(20usize, 19usize, 28usize, 2118408u32);
    emu.sri_no_count(28usize, 22usize, 3u32, 2118412u32);
    emu.sw_no_count(22usize, 2usize, 388u32, 2118416u32)?;
    emu.xrr_no_count(13usize, 13usize, 28usize, 2118420u32);
    emu.sw_no_count(13usize, 2usize, 312u32, 2118424u32)?;
    emu.sri_no_count(13usize, 12usize, 3u32, 2118428u32);
    emu.xrr_no_count(12usize, 29usize, 13usize, 2118432u32);
    emu.sw_no_count(12usize, 2usize, 324u32, 2118436u32)?;
    emu.lw_no_count(12usize, 2usize, 224u32, 2118440u32)?;
    emu.adr_no_count(5usize, 5usize, 12usize, 2118444u32);
    emu.adr_no_count(16usize, 5usize, 16usize, 2118448u32);
    emu.adr_no_count(6usize, 6usize, 7usize, 2118452u32);
    emu.lw_no_count(13usize, 2usize, 500u32, 2118456u32)?;
    emu.lw_no_count(12usize, 2usize, 384u32, 2118460u32)?;
    emu.adr_no_count(13usize, 12usize, 13usize, 2118464u32);
    emu.lw_no_count(12usize, 2usize, 436u32, 2118468u32)?;
    emu.adr_no_count(13usize, 13usize, 12usize, 2118472u32);
    emu.adr_no_count(31usize, 13usize, 31usize, 2118476u32);
    emu.lw_no_count(13usize, 2usize, 476u32, 2118480u32)?;
    emu.lw_no_count(12usize, 2usize, 340u32, 2118484u32)?;
    emu.adr_no_count(13usize, 12usize, 13usize, 2118488u32);
    emu.lw_no_count(12usize, 2usize, 468u32, 2118492u32)?;
    emu.adr_no_count(13usize, 13usize, 12usize, 2118496u32);
    emu.adr_no_count(20usize, 13usize, 20usize, 2118500u32);
    emu.adr_no_count(26usize, 6usize, 16usize, 2118504u32);
    emu.adr_no_count(11usize, 16usize, 11usize, 2118508u32);
    emu.sri_no_count(13usize, 31usize, 17u32, 2118512u32);
    emu.sli_no_count(16usize, 31usize, 15u32, 2118516u32);
    emu.orr_no_count(13usize, 16usize, 13usize, 2118520u32);
    emu.sri_no_count(16usize, 31usize, 19u32, 2118524u32);
    emu.sli_no_count(5usize, 31usize, 13u32, 2118528u32);
    emu.orr_no_count(5usize, 5usize, 16usize, 2118532u32);
    emu.sri_no_count(16usize, 20usize, 17u32, 2118536u32);
    emu.sli_no_count(6usize, 20usize, 15u32, 2118540u32);
    emu.orr_no_count(6usize, 6usize, 16usize, 2118544u32);
    emu.sri_no_count(16usize, 20usize, 19u32, 2118548u32);
    emu.sli_no_count(7usize, 20usize, 13u32, 2118552u32);
    emu.orr_no_count(7usize, 7usize, 16usize, 2118556u32);
    emu.sri_no_count(16usize, 20usize, 7u32, 2118560u32);
    emu.sli_no_count(28usize, 20usize, 25u32, 2118564u32);
    emu.orr_no_count(28usize, 28usize, 16usize, 2118568u32);
    emu.sri_no_count(16usize, 31usize, 7u32, 2118572u32);
    emu.sli_no_count(29usize, 31usize, 25u32, 2118576u32);
    emu.orr_no_count(29usize, 29usize, 16usize, 2118580u32);
    emu.sri_no_count(16usize, 20usize, 18u32, 2118584u32);
    emu.sli_no_count(30usize, 20usize, 14u32, 2118588u32);
    emu.orr_no_count(30usize, 30usize, 16usize, 2118592u32);
    emu.sri_no_count(16usize, 31usize, 18u32, 2118596u32);
    emu.sli_no_count(8usize, 31usize, 14u32, 2118600u32);
    emu.orr_no_count(8usize, 8usize, 16usize, 2118604u32);
    emu.sri_no_count(16usize, 11usize, 6u32, 2118608u32);
    emu.sli_no_count(9usize, 11usize, 26u32, 2118612u32);
    emu.orr_no_count(9usize, 9usize, 16usize, 2118616u32);
    emu.sri_no_count(16usize, 11usize, 11u32, 2118620u32);
    emu.sli_no_count(19usize, 11usize, 21u32, 2118624u32);
    emu.orr_no_count(19usize, 19usize, 16usize, 2118628u32);
    emu.sri_no_count(16usize, 11usize, 25u32, 2118632u32);
    emu.sli_no_count(21usize, 11usize, 7u32, 2118636u32);
    emu.orr_no_count(21usize, 21usize, 16usize, 2118640u32);
    emu.lw_no_count(16usize, 2usize, 488u32, 2118644u32)?;
    emu.adr_no_count(16usize, 16usize, 17usize, 2118648u32);
    emu.xrr_no_count(23usize, 10usize, 17usize, 2118652u32);
    emu.anr_no_count(23usize, 11usize, 23usize, 2118656u32);
    emu.xrr_no_count(17usize, 23usize, 17usize, 2118660u32);
    emu.sri_no_count(23usize, 26usize, 2u32, 2118664u32);
    emu.sli_no_count(24usize, 26usize, 30u32, 2118668u32);
    emu.orr_no_count(23usize, 24usize, 23usize, 2118672u32);
    emu.sri_no_count(24usize, 26usize, 13u32, 2118676u32);
    emu.sli_no_count(27usize, 26usize, 19u32, 2118680u32);
    emu.orr_no_count(24usize, 27usize, 24usize, 2118684u32);
    emu.sri_no_count(27usize, 26usize, 22u32, 2118688u32);
    emu.sli_no_count(1usize, 26usize, 10u32, 2118692u32);
    emu.orr_no_count(27usize, 1usize, 27usize, 2118696u32);
    emu.xrr_no_count(1usize, 18usize, 14usize, 2118700u32);
    emu.anr_no_count(1usize, 26usize, 1usize, 2118704u32);
    emu.anr_no_count(12usize, 18usize, 14usize, 2118708u32);
    emu.xrr_no_count(12usize, 1usize, 12usize, 2118712u32);
    emu.xrr_no_count(13usize, 13usize, 5usize, 2118716u32);
    emu.xrr_no_count(5usize, 6usize, 7usize, 2118720u32);
    emu.xrr_no_count(6usize, 28usize, 30usize, 2118724u32);
    emu.xrr_no_count(7usize, 29usize, 8usize, 2118728u32);
    emu.xrr_no_count(28usize, 9usize, 19usize, 2118732u32);
    emu.adr_no_count(15usize, 15usize, 17usize, 2118736u32);
    emu.xrr_no_count(17usize, 23usize, 24usize, 2118740u32);
    emu.sri_no_count(29usize, 31usize, 10u32, 2118744u32);
    emu.xrr_no_count(13usize, 13usize, 29usize, 2118748u32);
    emu.sri_no_count(29usize, 20usize, 10u32, 2118752u32);
    emu.xrr_no_count(19usize, 5usize, 29usize, 2118756u32);
    emu.sri_no_count(5usize, 20usize, 3u32, 2118760u32);
    emu.xrr_no_count(5usize, 6usize, 5usize, 2118764u32);
    emu.sw_no_count(5usize, 2usize, 316u32, 2118768u32)?;
    emu.sri_no_count(5usize, 31usize, 3u32, 2118772u32);
    emu.sw_no_count(31usize, 2usize, 452u32, 2118776u32)?;
    emu.xrr_no_count(5usize, 7usize, 5usize, 2118780u32);
    emu.sw_no_count(5usize, 2usize, 308u32, 2118784u32)?;
    emu.xrr_no_count(5usize, 28usize, 21usize, 2118788u32);
    emu.xrr_no_count(17usize, 17usize, 27usize, 2118792u32);
    emu.lw_no_count(6usize, 2usize, 404u32, 2118796u32)?;
    emu.lw_no_count(7usize, 2usize, 360u32, 2118800u32)?;
    emu.adr_no_count(6usize, 7usize, 6usize, 2118804u32);
    emu.lw_no_count(7usize, 2usize, 484u32, 2118808u32)?;
    emu.adr_no_count(6usize, 6usize, 7usize, 2118812u32);
    emu.adr_no_count(24usize, 6usize, 13usize, 2118816u32);
    emu.lw_no_count(13usize, 2usize, 472u32, 2118820u32)?;
    emu.lw_no_count(6usize, 2usize, 320u32, 2118824u32)?;
    emu.adr_no_count(13usize, 6usize, 13usize, 2118828u32);
    emu.lw_no_count(6usize, 2usize, 464u32, 2118832u32)?;
    emu.adr_no_count(13usize, 13usize, 6usize, 2118836u32);
    emu.adr_no_count(19usize, 13usize, 19usize, 2118840u32);
    emu.lw_no_count(13usize, 2usize, 220u32, 2118844u32)?;
    emu.adr_no_count(15usize, 15usize, 13usize, 2118848u32);
    emu.adr_no_count(15usize, 15usize, 5usize, 2118852u32);
    emu.adr_no_count(12usize, 17usize, 12usize, 2118856u32);
    emu.adr_no_count(1usize, 12usize, 15usize, 2118860u32);
    emu.adr_no_count(27usize, 15usize, 25usize, 2118864u32);
    emu.sri_no_count(12usize, 24usize, 17u32, 2118868u32);
    emu.sli_no_count(13usize, 24usize, 15u32, 2118872u32);
    emu.orr_no_count(13usize, 13usize, 12usize, 2118876u32);
    emu.sri_no_count(12usize, 24usize, 19u32, 2118880u32);
    emu.sli_no_count(15usize, 24usize, 13u32, 2118884u32);
    emu.orr_no_count(15usize, 15usize, 12usize, 2118888u32);
    emu.sri_no_count(12usize, 19usize, 17u32, 2118892u32);
    emu.sli_no_count(17usize, 19usize, 15u32, 2118896u32);
    emu.orr_no_count(17usize, 17usize, 12usize, 2118900u32);
    emu.sri_no_count(12usize, 19usize, 19u32, 2118904u32);
    emu.sli_no_count(5usize, 19usize, 13u32, 2118908u32);
    emu.orr_no_count(5usize, 5usize, 12usize, 2118912u32);
    emu.sri_no_count(12usize, 24usize, 7u32, 2118916u32);
    emu.sli_no_count(6usize, 24usize, 25u32, 2118920u32);
    emu.orr_no_count(6usize, 6usize, 12usize, 2118924u32);
    emu.sri_no_count(12usize, 24usize, 18u32, 2118928u32);
    emu.sli_no_count(7usize, 24usize, 14u32, 2118932u32);
    emu.orr_no_count(7usize, 7usize, 12usize, 2118936u32);
    emu.sri_no_count(12usize, 19usize, 7u32, 2118940u32);
    emu.sli_no_count(29usize, 19usize, 25u32, 2118944u32);
    emu.orr_no_count(12usize, 29usize, 12usize, 2118948u32);
    emu.sri_no_count(29usize, 19usize, 18u32, 2118952u32);
    emu.sli_no_count(30usize, 19usize, 14u32, 2118956u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2118960u32);
    emu.xrr_no_count(13usize, 13usize, 15usize, 2118964u32);
    emu.xrr_no_count(15usize, 17usize, 5usize, 2118968u32);
    emu.xrr_no_count(17usize, 6usize, 7usize, 2118972u32);
    emu.xrr_no_count(12usize, 12usize, 29usize, 2118976u32);
    emu.sri_no_count(5usize, 27usize, 6u32, 2118980u32);
    emu.sli_no_count(6usize, 27usize, 26u32, 2118984u32);
    emu.orr_no_count(5usize, 6usize, 5usize, 2118988u32);
    emu.sri_no_count(6usize, 27usize, 11u32, 2118992u32);
    emu.sli_no_count(7usize, 27usize, 21u32, 2118996u32);
    emu.orr_no_count(6usize, 7usize, 6usize, 2119000u32);
    emu.sri_no_count(7usize, 27usize, 25u32, 2119004u32);
    emu.sli_no_count(29usize, 27usize, 7u32, 2119008u32);
    emu.orr_no_count(7usize, 29usize, 7usize, 2119012u32);
    emu.lw_no_count(8usize, 2usize, 412u32, 2119016u32)?;
    emu.adr_no_count(8usize, 8usize, 10usize, 2119020u32);
    emu.xrr_no_count(29usize, 11usize, 10usize, 2119024u32);
    emu.anr_no_count(29usize, 27usize, 29usize, 2119028u32);
    emu.xrr_no_count(10usize, 29usize, 10usize, 2119032u32);
    emu.sri_no_count(29usize, 1usize, 2u32, 2119036u32);
    emu.sli_no_count(30usize, 1usize, 30u32, 2119040u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2119044u32);
    emu.sri_no_count(30usize, 1usize, 13u32, 2119048u32);
    emu.sli_no_count(9usize, 1usize, 19u32, 2119052u32);
    emu.orr_no_count(30usize, 9usize, 30usize, 2119056u32);
    emu.sri_no_count(9usize, 1usize, 22u32, 2119060u32);
    emu.sli_no_count(21usize, 1usize, 10u32, 2119064u32);
    emu.orr_no_count(9usize, 21usize, 9usize, 2119068u32);
    emu.xrr_no_count(21usize, 26usize, 18usize, 2119072u32);
    emu.anr_no_count(21usize, 1usize, 21usize, 2119076u32);
    emu.anr_no_count(23usize, 26usize, 18usize, 2119080u32);
    emu.xrr_no_count(21usize, 21usize, 23usize, 2119084u32);
    emu.sri_no_count(23usize, 24usize, 10u32, 2119088u32);
    emu.xrr_no_count(13usize, 13usize, 23usize, 2119092u32);
    emu.sw_no_count(19usize, 2usize, 488u32, 2119096u32)?;
    emu.sri_no_count(23usize, 19usize, 10u32, 2119100u32);
    emu.xrr_no_count(15usize, 15usize, 23usize, 2119104u32);
    emu.sri_no_count(23usize, 24usize, 3u32, 2119108u32);
    emu.sw_no_count(24usize, 2usize, 384u32, 2119112u32)?;
    emu.xrr_no_count(17usize, 17usize, 23usize, 2119116u32);
    emu.sw_no_count(17usize, 2usize, 304u32, 2119120u32)?;
    emu.sri_no_count(17usize, 19usize, 3u32, 2119124u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2119128u32);
    emu.sw_no_count(12usize, 2usize, 360u32, 2119132u32)?;
    emu.xrr_no_count(12usize, 5usize, 6usize, 2119136u32);
    emu.adr_no_count(10usize, 16usize, 10usize, 2119140u32);
    emu.xrr_no_count(17usize, 29usize, 30usize, 2119144u32);
    emu.lw_no_count(16usize, 2usize, 400u32, 2119148u32)?;
    emu.lw_no_count(5usize, 2usize, 380u32, 2119152u32)?;
    emu.adr_no_count(16usize, 5usize, 16usize, 2119156u32);
    emu.lw_no_count(5usize, 2usize, 480u32, 2119160u32)?;
    emu.adr_no_count(16usize, 16usize, 5usize, 2119164u32);
    emu.adr_no_count(5usize, 16usize, 13usize, 2119168u32);
    emu.lw_no_count(13usize, 2usize, 460u32, 2119172u32)?;
    emu.lw_no_count(16usize, 2usize, 336u32, 2119176u32)?;
    emu.adr_no_count(13usize, 16usize, 13usize, 2119180u32);
    emu.adr_no_count(13usize, 13usize, 22usize, 2119184u32);
    emu.adr_no_count(16usize, 13usize, 15usize, 2119188u32);
    emu.xrr_no_count(12usize, 12usize, 7usize, 2119192u32);
    emu.xrr_no_count(13usize, 17usize, 9usize, 2119196u32);
    emu.lw_no_count(15usize, 2usize, 216u32, 2119200u32)?;
    emu.adr_no_count(10usize, 10usize, 15usize, 2119204u32);
    emu.adr_no_count(12usize, 10usize, 12usize, 2119208u32);
    emu.adr_no_count(10usize, 13usize, 21usize, 2119212u32);
    emu.sri_no_count(13usize, 5usize, 17u32, 2119216u32);
    emu.sli_no_count(15usize, 5usize, 15u32, 2119220u32);
    emu.orr_no_count(13usize, 15usize, 13usize, 2119224u32);
    emu.sri_no_count(15usize, 5usize, 19u32, 2119228u32);
    emu.sli_no_count(17usize, 5usize, 13u32, 2119232u32);
    emu.orr_no_count(15usize, 17usize, 15usize, 2119236u32);
    emu.sri_no_count(17usize, 16usize, 17u32, 2119240u32);
    emu.sli_no_count(6usize, 16usize, 15u32, 2119244u32);
    emu.orr_no_count(17usize, 6usize, 17usize, 2119248u32);
    emu.sri_no_count(6usize, 16usize, 19u32, 2119252u32);
    emu.sli_no_count(7usize, 16usize, 13u32, 2119256u32);
    emu.orr_no_count(6usize, 7usize, 6usize, 2119260u32);
    emu.sri_no_count(7usize, 16usize, 7u32, 2119264u32);
    emu.sli_no_count(29usize, 16usize, 25u32, 2119268u32);
    emu.orr_no_count(7usize, 29usize, 7usize, 2119272u32);
    emu.sri_no_count(29usize, 5usize, 7u32, 2119276u32);
    emu.sli_no_count(30usize, 5usize, 25u32, 2119280u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2119284u32);
    emu.sri_no_count(30usize, 16usize, 18u32, 2119288u32);
    emu.sli_no_count(9usize, 16usize, 14u32, 2119292u32);
    emu.orr_no_count(30usize, 9usize, 30usize, 2119296u32);
    emu.sri_no_count(9usize, 5usize, 18u32, 2119300u32);
    emu.sli_no_count(21usize, 5usize, 14u32, 2119304u32);
    emu.orr_no_count(9usize, 21usize, 9usize, 2119308u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2119312u32);
    emu.adr_no_count(14usize, 12usize, 14usize, 2119316u32);
    emu.xrr_no_count(13usize, 13usize, 15usize, 2119320u32);
    emu.xrr_no_count(12usize, 17usize, 6usize, 2119324u32);
    emu.xrr_no_count(15usize, 7usize, 30usize, 2119328u32);
    emu.xrr_no_count(17usize, 29usize, 9usize, 2119332u32);
    emu.sri_no_count(6usize, 5usize, 10u32, 2119336u32);
    emu.xrr_no_count(13usize, 13usize, 6usize, 2119340u32);
    emu.sw_no_count(16usize, 2usize, 380u32, 2119344u32)?;
    emu.sri_no_count(6usize, 16usize, 10u32, 2119348u32);
    emu.xrr_no_count(29usize, 12usize, 6usize, 2119352u32);
    emu.sri_no_count(12usize, 16usize, 3u32, 2119356u32);
    emu.xrr_no_count(12usize, 15usize, 12usize, 2119360u32);
    emu.sw_no_count(12usize, 2usize, 320u32, 2119364u32)?;
    emu.sri_no_count(12usize, 5usize, 3u32, 2119368u32);
    emu.sw_no_count(5usize, 2usize, 412u32, 2119372u32)?;
    emu.xrr_no_count(12usize, 17usize, 12usize, 2119376u32);
    emu.sw_no_count(12usize, 2usize, 300u32, 2119380u32)?;
    emu.sri_no_count(12usize, 14usize, 6u32, 2119384u32);
    emu.sli_no_count(15usize, 14usize, 26u32, 2119388u32);
    emu.orr_no_count(12usize, 15usize, 12usize, 2119392u32);
    emu.sri_no_count(15usize, 14usize, 11u32, 2119396u32);
    emu.sli_no_count(17usize, 14usize, 21u32, 2119400u32);
    emu.orr_no_count(17usize, 17usize, 15usize, 2119404u32);
    emu.sri_no_count(15usize, 14usize, 25u32, 2119408u32);
    emu.sli_no_count(6usize, 14usize, 7u32, 2119412u32);
    emu.orr_no_count(6usize, 6usize, 15usize, 2119416u32);
    emu.lw_no_count(15usize, 2usize, 416u32, 2119420u32)?;
    emu.adr_no_count(19usize, 15usize, 11usize, 2119424u32);
    emu.xrr_no_count(15usize, 27usize, 11usize, 2119428u32);
    emu.anr_no_count(15usize, 14usize, 15usize, 2119432u32);
    emu.xrr_no_count(11usize, 15usize, 11usize, 2119436u32);
    emu.sri_no_count(15usize, 10usize, 2u32, 2119440u32);
    emu.sli_no_count(7usize, 10usize, 30u32, 2119444u32);
    emu.orr_no_count(30usize, 7usize, 15usize, 2119448u32);
    emu.sri_no_count(15usize, 10usize, 13u32, 2119452u32);
    emu.sli_no_count(7usize, 10usize, 19u32, 2119456u32);
    emu.orr_no_count(9usize, 7usize, 15usize, 2119460u32);
    emu.sri_no_count(15usize, 10usize, 22u32, 2119464u32);
    emu.sli_no_count(7usize, 10usize, 10u32, 2119468u32);
    emu.orr_no_count(21usize, 7usize, 15usize, 2119472u32);
    emu.xrr_no_count(15usize, 1usize, 26usize, 2119476u32);
    emu.anr_no_count(15usize, 10usize, 15usize, 2119480u32);
    emu.anr_no_count(7usize, 1usize, 26usize, 2119484u32);
    emu.xrr_no_count(7usize, 15usize, 7usize, 2119488u32);
    emu.lw_no_count(15usize, 2usize, 396u32, 2119492u32)?;
    emu.lw_no_count(16usize, 2usize, 356u32, 2119496u32)?;
    emu.adr_no_count(15usize, 16usize, 15usize, 2119500u32);
    emu.lw_no_count(16usize, 2usize, 448u32, 2119504u32)?;
    emu.adr_no_count(15usize, 15usize, 16usize, 2119508u32);
    emu.adr_no_count(15usize, 15usize, 13usize, 2119512u32);
    emu.lw_no_count(13usize, 2usize, 456u32, 2119516u32)?;
    emu.lw_no_count(16usize, 2usize, 344u32, 2119520u32)?;
    emu.adr_no_count(13usize, 16usize, 13usize, 2119524u32);
    emu.adr_no_count(13usize, 13usize, 31usize, 2119528u32);
    emu.adr_no_count(16usize, 13usize, 29usize, 2119532u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2119536u32);
    emu.adr_no_count(8usize, 8usize, 11usize, 2119540u32);
    emu.xrr_no_count(11usize, 30usize, 9usize, 2119544u32);
    emu.xrr_no_count(12usize, 12usize, 6usize, 2119548u32);
    emu.xrr_no_count(21usize, 11usize, 21usize, 2119552u32);
    emu.sri_no_count(11usize, 15usize, 17u32, 2119556u32);
    emu.sli_no_count(13usize, 15usize, 15u32, 2119560u32);
    emu.orr_no_count(23usize, 13usize, 11usize, 2119564u32);
    emu.sri_no_count(11usize, 15usize, 19u32, 2119568u32);
    emu.sli_no_count(13usize, 15usize, 13u32, 2119572u32);
    emu.orr_no_count(31usize, 13usize, 11usize, 2119576u32);
    emu.sri_no_count(11usize, 16usize, 17u32, 2119580u32);
    emu.sli_no_count(13usize, 16usize, 15u32, 2119584u32);
    emu.orr_no_count(25usize, 13usize, 11usize, 2119588u32);
    emu.sri_no_count(11usize, 16usize, 19u32, 2119592u32);
    emu.sli_no_count(13usize, 16usize, 13u32, 2119596u32);
    emu.orr_no_count(6usize, 13usize, 11usize, 2119600u32);
    emu.sri_no_count(11usize, 15usize, 7u32, 2119604u32);
    emu.sli_no_count(13usize, 15usize, 25u32, 2119608u32);
    emu.orr_no_count(13usize, 13usize, 11usize, 2119612u32);
    emu.sri_no_count(11usize, 15usize, 18u32, 2119616u32);
    emu.sli_no_count(17usize, 15usize, 14u32, 2119620u32);
    emu.orr_no_count(11usize, 17usize, 11usize, 2119624u32);
    emu.sri_no_count(17usize, 16usize, 7u32, 2119628u32);
    emu.sli_no_count(30usize, 16usize, 25u32, 2119632u32);
    emu.orr_no_count(17usize, 30usize, 17usize, 2119636u32);
    emu.sri_no_count(30usize, 16usize, 18u32, 2119640u32);
    emu.sli_no_count(9usize, 16usize, 14u32, 2119644u32);
    emu.orr_no_count(30usize, 9usize, 30usize, 2119648u32);
    emu.lw_no_count(28usize, 2usize, 212u32, 2119652u32)?;
    emu.adr_no_count(8usize, 8usize, 28usize, 2119656u32);
    emu.adr_no_count(12usize, 8usize, 12usize, 2119660u32);
    emu.adr_no_count(7usize, 21usize, 7usize, 2119664u32);
    emu.xrr_no_count(8usize, 23usize, 31usize, 2119668u32);
    emu.xrr_no_count(9usize, 25usize, 6usize, 2119672u32);
    emu.xrr_no_count(11usize, 13usize, 11usize, 2119676u32);
    emu.xrr_no_count(13usize, 17usize, 30usize, 2119680u32);
    emu.adr_no_count(21usize, 7usize, 12usize, 2119684u32);
    emu.adr_no_count(6usize, 12usize, 18usize, 2119688u32);
    emu.sri_no_count(12usize, 15usize, 10u32, 2119692u32);
    emu.xrr_no_count(12usize, 8usize, 12usize, 2119696u32);
    emu.sri_no_count(17usize, 16usize, 10u32, 2119700u32);
    emu.xrr_no_count(17usize, 9usize, 17usize, 2119704u32);
    emu.sri_no_count(7usize, 15usize, 3u32, 2119708u32);
    emu.sw_no_count(15usize, 2usize, 276u32, 2119712u32)?;
    emu.xrr_no_count(11usize, 11usize, 7usize, 2119716u32);
    emu.sw_no_count(11usize, 2usize, 288u32, 2119720u32)?;
    emu.sri_no_count(11usize, 16usize, 3u32, 2119724u32);
    emu.sw_no_count(16usize, 2usize, 272u32, 2119728u32)?;
    emu.xrr_no_count(11usize, 13usize, 11usize, 2119732u32);
    emu.sw_no_count(11usize, 2usize, 292u32, 2119736u32)?;
    emu.lw_no_count(11usize, 2usize, 392u32, 2119740u32)?;
    emu.lw_no_count(13usize, 2usize, 364u32, 2119744u32)?;
    emu.adr_no_count(11usize, 13usize, 11usize, 2119748u32);
    emu.sw_no_count(20usize, 2usize, 376u32, 2119752u32)?;
    emu.adr_no_count(11usize, 11usize, 20usize, 2119756u32);
    emu.adr_no_count(20usize, 11usize, 12usize, 2119760u32);
    emu.lw_no_count(11usize, 2usize, 328u32, 2119764u32)?;
    emu.lw_no_count(12usize, 2usize, 436u32, 2119768u32)?;
    emu.adr_no_count(11usize, 11usize, 12usize, 2119772u32);
    emu.adr_no_count(11usize, 11usize, 24usize, 2119776u32);
    emu.adr_no_count(22usize, 11usize, 17usize, 2119780u32);
    emu.sri_no_count(11usize, 6usize, 6u32, 2119784u32);
    emu.sli_no_count(17usize, 6usize, 26u32, 2119788u32);
    emu.orr_no_count(17usize, 17usize, 11usize, 2119792u32);
    emu.sri_no_count(11usize, 6usize, 11u32, 2119796u32);
    emu.sli_no_count(7usize, 6usize, 21u32, 2119800u32);
    emu.orr_no_count(7usize, 7usize, 11usize, 2119804u32);
    emu.sri_no_count(11usize, 6usize, 25u32, 2119808u32);
    emu.sli_no_count(30usize, 6usize, 7u32, 2119812u32);
    emu.orr_no_count(30usize, 30usize, 11usize, 2119816u32);
    emu.lw_no_count(11usize, 2usize, 420u32, 2119820u32)?;
    emu.adr_no_count(11usize, 11usize, 27usize, 2119824u32);
    emu.xrr_no_count(8usize, 14usize, 27usize, 2119828u32);
    emu.anr_no_count(8usize, 6usize, 8usize, 2119832u32);
    emu.xrr_no_count(8usize, 8usize, 27usize, 2119836u32);
    emu.sri_no_count(9usize, 21usize, 2u32, 2119840u32);
    emu.sli_no_count(18usize, 21usize, 30u32, 2119844u32);
    emu.orr_no_count(9usize, 18usize, 9usize, 2119848u32);
    emu.sri_no_count(18usize, 21usize, 13u32, 2119852u32);
    emu.sli_no_count(23usize, 21usize, 19u32, 2119856u32);
    emu.orr_no_count(18usize, 23usize, 18usize, 2119860u32);
    emu.sri_no_count(23usize, 21usize, 22u32, 2119864u32);
    emu.sli_no_count(24usize, 21usize, 10u32, 2119868u32);
    emu.orr_no_count(23usize, 24usize, 23usize, 2119872u32);
    emu.xrr_no_count(24usize, 10usize, 1usize, 2119876u32);
    emu.anr_no_count(24usize, 21usize, 24usize, 2119880u32);
    emu.anr_no_count(25usize, 10usize, 1usize, 2119884u32);
    emu.xrr_no_count(24usize, 24usize, 25usize, 2119888u32);
    emu.xrr_no_count(17usize, 17usize, 7usize, 2119892u32);
    emu.adr_no_count(8usize, 19usize, 8usize, 2119896u32);
    emu.xrr_no_count(7usize, 9usize, 18usize, 2119900u32);
    emu.sri_no_count(9usize, 20usize, 17u32, 2119904u32);
    emu.sli_no_count(18usize, 20usize, 15u32, 2119908u32);
    emu.orr_no_count(9usize, 18usize, 9usize, 2119912u32);
    emu.sri_no_count(18usize, 20usize, 19u32, 2119916u32);
    emu.sli_no_count(25usize, 20usize, 13u32, 2119920u32);
    emu.orr_no_count(18usize, 25usize, 18usize, 2119924u32);
    emu.sri_no_count(25usize, 22usize, 17u32, 2119928u32);
    emu.sli_no_count(27usize, 22usize, 15u32, 2119932u32);
    emu.orr_no_count(25usize, 27usize, 25usize, 2119936u32);
    emu.sri_no_count(27usize, 22usize, 19u32, 2119940u32);
    emu.sli_no_count(13usize, 22usize, 13u32, 2119944u32);
    emu.orr_no_count(13usize, 13usize, 27usize, 2119948u32);
    emu.sri_no_count(27usize, 22usize, 7u32, 2119952u32);
    emu.sli_no_count(29usize, 22usize, 25u32, 2119956u32);
    emu.orr_no_count(29usize, 29usize, 27usize, 2119960u32);
    emu.sri_no_count(27usize, 20usize, 7u32, 2119964u32);
    emu.sli_no_count(12usize, 20usize, 25u32, 2119968u32);
    emu.orr_no_count(12usize, 12usize, 27usize, 2119972u32);
    emu.sri_no_count(27usize, 22usize, 18u32, 2119976u32);
    emu.sli_no_count(19usize, 22usize, 14u32, 2119980u32);
    emu.orr_no_count(19usize, 19usize, 27usize, 2119984u32);
    emu.sri_no_count(27usize, 20usize, 18u32, 2119988u32);
    emu.sli_no_count(31usize, 20usize, 14u32, 2119992u32);
    emu.orr_no_count(31usize, 31usize, 27usize, 2119996u32);
    emu.xrr_no_count(17usize, 17usize, 30usize, 2120000u32);
    emu.xrr_no_count(7usize, 7usize, 23usize, 2120004u32);
    emu.xrr_no_count(30usize, 9usize, 18usize, 2120008u32);
    emu.xrr_no_count(13usize, 25usize, 13usize, 2120012u32);
    emu.xrr_no_count(29usize, 29usize, 19usize, 2120016u32);
    emu.xrr_no_count(12usize, 12usize, 31usize, 2120020u32);
    emu.lw_no_count(28usize, 2usize, 208u32, 2120024u32)?;
    emu.adr_no_count(8usize, 8usize, 28usize, 2120028u32);
    emu.adr_no_count(17usize, 8usize, 17usize, 2120032u32);
    emu.adr_no_count(7usize, 7usize, 24usize, 2120036u32);
    emu.sri_no_count(31usize, 20usize, 10u32, 2120040u32);
    emu.xrr_no_count(30usize, 30usize, 31usize, 2120044u32);
    emu.sw_no_count(22usize, 2usize, 416u32, 2120048u32)?;
    emu.sri_no_count(31usize, 22usize, 10u32, 2120052u32);
    emu.xrr_no_count(13usize, 13usize, 31usize, 2120056u32);
    emu.sri_no_count(31usize, 22usize, 3u32, 2120060u32);
    emu.xrr_no_count(28usize, 29usize, 31usize, 2120064u32);
    emu.sw_no_count(28usize, 2usize, 328u32, 2120068u32)?;
    emu.sri_no_count(29usize, 20usize, 3u32, 2120072u32);
    emu.adi_no_count(22usize, 20usize, 0u32, 2120076u32);
    emu.sw_no_count(20usize, 2usize, 284u32, 2120080u32)?;
    emu.xrr_no_count(12usize, 12usize, 29usize, 2120084u32);
    emu.sw_no_count(12usize, 2usize, 280u32, 2120088u32)?;
    emu.adr_no_count(18usize, 7usize, 17usize, 2120092u32);
    emu.adr_no_count(7usize, 17usize, 26usize, 2120096u32);
    emu.lw_no_count(12usize, 2usize, 468u32, 2120100u32)?;
    emu.lw_no_count(17usize, 2usize, 352u32, 2120104u32)?;
    emu.adr_no_count(12usize, 17usize, 12usize, 2120108u32);
    emu.lw_no_count(17usize, 2usize, 488u32, 2120112u32)?;
    emu.adr_no_count(12usize, 12usize, 17usize, 2120116u32);
    emu.adr_no_count(28usize, 12usize, 30usize, 2120120u32);
    emu.lw_no_count(12usize, 2usize, 484u32, 2120124u32)?;
    emu.lw_no_count(17usize, 2usize, 332u32, 2120128u32)?;
    emu.adr_no_count(12usize, 17usize, 12usize, 2120132u32);
    emu.adr_no_count(12usize, 12usize, 5usize, 2120136u32);
    emu.adr_no_count(20usize, 12usize, 13usize, 2120140u32);
    emu.sri_no_count(12usize, 7usize, 6u32, 2120144u32);
    emu.sli_no_count(13usize, 7usize, 26u32, 2120148u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2120152u32);
    emu.sri_no_count(13usize, 7usize, 11u32, 2120156u32);
    emu.sli_no_count(29usize, 7usize, 21u32, 2120160u32);
    emu.orr_no_count(13usize, 29usize, 13usize, 2120164u32);
    emu.sri_no_count(29usize, 7usize, 25u32, 2120168u32);
    emu.sli_no_count(30usize, 7usize, 7u32, 2120172u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2120176u32);
    emu.lw_no_count(27usize, 2usize, 408u32, 2120180u32)?;
    emu.adr_no_count(27usize, 27usize, 14usize, 2120184u32);
    emu.xrr_no_count(30usize, 6usize, 14usize, 2120188u32);
    emu.anr_no_count(30usize, 7usize, 30usize, 2120192u32);
    emu.xrr_no_count(14usize, 30usize, 14usize, 2120196u32);
    emu.sri_no_count(30usize, 18usize, 2u32, 2120200u32);
    emu.sli_no_count(31usize, 18usize, 30u32, 2120204u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2120208u32);
    emu.sri_no_count(31usize, 18usize, 13u32, 2120212u32);
    emu.sli_no_count(8usize, 18usize, 19u32, 2120216u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2120220u32);
    emu.sri_no_count(8usize, 18usize, 22u32, 2120224u32);
    emu.sli_no_count(9usize, 18usize, 10u32, 2120228u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2120232u32);
    emu.xrr_no_count(9usize, 21usize, 10usize, 2120236u32);
    emu.anr_no_count(9usize, 18usize, 9usize, 2120240u32);
    emu.anr_no_count(19usize, 21usize, 10usize, 2120244u32);
    emu.xrr_no_count(9usize, 9usize, 19usize, 2120248u32);
    emu.adi_no_count(5usize, 28usize, 0u32, 2120252u32);
    emu.sri_no_count(19usize, 28usize, 17u32, 2120256u32);
    emu.sli_no_count(24usize, 28usize, 15u32, 2120260u32);
    emu.orr_no_count(19usize, 24usize, 19usize, 2120264u32);
    emu.sri_no_count(24usize, 28usize, 19u32, 2120268u32);
    emu.sli_no_count(25usize, 28usize, 13u32, 2120272u32);
    emu.orr_no_count(24usize, 25usize, 24usize, 2120276u32);
    emu.sri_no_count(25usize, 20usize, 17u32, 2120280u32);
    emu.sli_no_count(26usize, 20usize, 15u32, 2120284u32);
    emu.orr_no_count(25usize, 26usize, 25usize, 2120288u32);
    emu.sri_no_count(26usize, 20usize, 19u32, 2120292u32);
    emu.sli_no_count(23usize, 20usize, 13u32, 2120296u32);
    emu.sw_no_count(20usize, 2usize, 420u32, 2120300u32)?;
    emu.orr_no_count(23usize, 23usize, 26usize, 2120304u32);
    emu.sri_no_count(26usize, 28usize, 7u32, 2120308u32);
    emu.sli_no_count(17usize, 28usize, 25u32, 2120312u32);
    emu.orr_no_count(17usize, 17usize, 26usize, 2120316u32);
    emu.sri_no_count(26usize, 28usize, 18u32, 2120320u32);
    emu.sli_no_count(28usize, 28usize, 14u32, 2120324u32);
    emu.orr_no_count(28usize, 28usize, 26usize, 2120328u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2120332u32);
    emu.adr_no_count(11usize, 11usize, 14usize, 2120336u32);
    emu.xrr_no_count(13usize, 30usize, 31usize, 2120340u32);
    emu.xrr_no_count(14usize, 19usize, 24usize, 2120344u32);
    emu.xrr_no_count(30usize, 25usize, 23usize, 2120348u32);
    emu.xrr_no_count(17usize, 17usize, 28usize, 2120352u32);
    emu.xrr_no_count(12usize, 12usize, 29usize, 2120356u32);
    emu.xrr_no_count(13usize, 13usize, 8usize, 2120360u32);
    emu.sri_no_count(28usize, 5usize, 10u32, 2120364u32);
    emu.xrr_no_count(25usize, 14usize, 28usize, 2120368u32);
    emu.sri_no_count(14usize, 20usize, 10u32, 2120372u32);
    emu.xrr_no_count(26usize, 30usize, 14usize, 2120376u32);
    emu.sri_no_count(14usize, 5usize, 3u32, 2120380u32);
    emu.sw_no_count(5usize, 2usize, 340u32, 2120384u32)?;
    emu.xrr_no_count(14usize, 17usize, 14usize, 2120388u32);
    emu.sw_no_count(14usize, 2usize, 296u32, 2120392u32)?;
    emu.lw_no_count(14usize, 2usize, 204u32, 2120396u32)?;
    emu.adr_no_count(11usize, 11usize, 14usize, 2120400u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2120404u32);
    emu.adr_no_count(13usize, 13usize, 9usize, 2120408u32);
    emu.lw_no_count(12usize, 2usize, 464u32, 2120412u32)?;
    emu.lw_no_count(14usize, 2usize, 348u32, 2120416u32)?;
    emu.adr_no_count(12usize, 14usize, 12usize, 2120420u32);
    emu.lw_no_count(20usize, 2usize, 380u32, 2120424u32)?;
    emu.adr_no_count(12usize, 12usize, 20usize, 2120428u32);
    emu.adr_no_count(25usize, 12usize, 25usize, 2120432u32);
    emu.lw_no_count(12usize, 2usize, 480u32, 2120436u32)?;
    emu.lw_no_count(14usize, 2usize, 312u32, 2120440u32)?;
    emu.adr_no_count(12usize, 14usize, 12usize, 2120444u32);
    emu.adr_no_count(12usize, 12usize, 15usize, 2120448u32);
    emu.adr_no_count(26usize, 12usize, 26usize, 2120452u32);
    emu.adr_no_count(8usize, 13usize, 11usize, 2120456u32);
    emu.adr_no_count(24usize, 11usize, 1usize, 2120460u32);
    emu.sri_no_count(11usize, 25usize, 17u32, 2120464u32);
    emu.sli_no_count(12usize, 25usize, 15u32, 2120468u32);
    emu.orr_no_count(11usize, 12usize, 11usize, 2120472u32);
    emu.sri_no_count(12usize, 25usize, 19u32, 2120476u32);
    emu.sli_no_count(13usize, 25usize, 13u32, 2120480u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2120484u32);
    emu.sri_no_count(13usize, 26usize, 17u32, 2120488u32);
    emu.sli_no_count(14usize, 26usize, 15u32, 2120492u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2120496u32);
    emu.sri_no_count(14usize, 26usize, 19u32, 2120500u32);
    emu.sli_no_count(17usize, 26usize, 13u32, 2120504u32);
    emu.orr_no_count(14usize, 17usize, 14usize, 2120508u32);
    emu.sri_no_count(17usize, 24usize, 6u32, 2120512u32);
    emu.sli_no_count(28usize, 24usize, 26u32, 2120516u32);
    emu.orr_no_count(17usize, 28usize, 17usize, 2120520u32);
    emu.sri_no_count(28usize, 24usize, 11u32, 2120524u32);
    emu.sli_no_count(29usize, 24usize, 21u32, 2120528u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2120532u32);
    emu.sri_no_count(29usize, 24usize, 25u32, 2120536u32);
    emu.sli_no_count(30usize, 24usize, 7u32, 2120540u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2120544u32);
    emu.lw_no_count(30usize, 2usize, 492u32, 2120548u32)?;
    emu.adr_no_count(30usize, 30usize, 6usize, 2120552u32);
    emu.xrr_no_count(31usize, 7usize, 6usize, 2120556u32);
    emu.anr_no_count(31usize, 24usize, 31usize, 2120560u32);
    emu.xrr_no_count(6usize, 31usize, 6usize, 2120564u32);
    emu.sri_no_count(31usize, 8usize, 2u32, 2120568u32);
    emu.sli_no_count(9usize, 8usize, 30u32, 2120572u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2120576u32);
    emu.sri_no_count(9usize, 8usize, 13u32, 2120580u32);
    emu.sli_no_count(19usize, 8usize, 19u32, 2120584u32);
    emu.orr_no_count(9usize, 19usize, 9usize, 2120588u32);
    emu.sri_no_count(19usize, 8usize, 22u32, 2120592u32);
    emu.sli_no_count(23usize, 8usize, 10u32, 2120596u32);
    emu.orr_no_count(19usize, 23usize, 19usize, 2120600u32);
    emu.xrr_no_count(23usize, 18usize, 21usize, 2120604u32);
    emu.anr_no_count(23usize, 8usize, 23usize, 2120608u32);
    emu.anr_no_count(1usize, 18usize, 21usize, 2120612u32);
    emu.xrr_no_count(23usize, 23usize, 1usize, 2120616u32);
    emu.xrr_no_count(11usize, 11usize, 12usize, 2120620u32);
    emu.xrr_no_count(13usize, 13usize, 14usize, 2120624u32);
    emu.xrr_no_count(12usize, 17usize, 28usize, 2120628u32);
    emu.adr_no_count(6usize, 27usize, 6usize, 2120632u32);
    emu.xrr_no_count(14usize, 31usize, 9usize, 2120636u32);
    emu.sri_no_count(17usize, 25usize, 10u32, 2120640u32);
    emu.sw_no_count(25usize, 2usize, 344u32, 2120644u32)?;
    emu.xrr_no_count(27usize, 11usize, 17usize, 2120648u32);
    emu.sri_no_count(11usize, 26usize, 10u32, 2120652u32);
    emu.sw_no_count(26usize, 2usize, 348u32, 2120656u32)?;
    emu.xrr_no_count(11usize, 13usize, 11usize, 2120660u32);
    emu.xrr_no_count(12usize, 12usize, 29usize, 2120664u32);
    emu.xrr_no_count(13usize, 14usize, 19usize, 2120668u32);
    emu.lw_no_count(14usize, 2usize, 388u32, 2120672u32)?;
    emu.lw_no_count(15usize, 2usize, 324u32, 2120676u32)?;
    emu.adr_no_count(14usize, 15usize, 14usize, 2120680u32);
    emu.adr_no_count(14usize, 14usize, 16usize, 2120684u32);
    emu.adr_no_count(27usize, 14usize, 27usize, 2120688u32);
    emu.lw_no_count(14usize, 2usize, 448u32, 2120692u32)?;
    emu.lw_no_count(15usize, 2usize, 308u32, 2120696u32)?;
    emu.adr_no_count(14usize, 15usize, 14usize, 2120700u32);
    emu.adr_no_count(14usize, 14usize, 22usize, 2120704u32);
    emu.adr_no_count(22usize, 14usize, 11usize, 2120708u32);
    emu.lw_no_count(11usize, 2usize, 200u32, 2120712u32)?;
    emu.adr_no_count(6usize, 6usize, 11usize, 2120716u32);
    emu.adr_no_count(12usize, 6usize, 12usize, 2120720u32);
    emu.adr_no_count(13usize, 13usize, 23usize, 2120724u32);
    emu.adr_no_count(14usize, 13usize, 12usize, 2120728u32);
    emu.adr_no_count(6usize, 12usize, 10usize, 2120732u32);
    emu.sri_no_count(10usize, 27usize, 17u32, 2120736u32);
    emu.sli_no_count(11usize, 27usize, 15u32, 2120740u32);
    emu.orr_no_count(10usize, 11usize, 10usize, 2120744u32);
    emu.sri_no_count(11usize, 27usize, 19u32, 2120748u32);
    emu.sli_no_count(12usize, 27usize, 13u32, 2120752u32);
    emu.orr_no_count(11usize, 12usize, 11usize, 2120756u32);
    emu.sri_no_count(12usize, 22usize, 17u32, 2120760u32);
    emu.sli_no_count(13usize, 22usize, 15u32, 2120764u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2120768u32);
    emu.sri_no_count(13usize, 22usize, 19u32, 2120772u32);
    emu.sli_no_count(17usize, 22usize, 13u32, 2120776u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2120780u32);
    emu.xrr_no_count(10usize, 10usize, 11usize, 2120784u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2120788u32);
    emu.sri_no_count(11usize, 6usize, 6u32, 2120792u32);
    emu.sli_no_count(13usize, 6usize, 26u32, 2120796u32);
    emu.orr_no_count(13usize, 13usize, 11usize, 2120800u32);
    emu.sri_no_count(11usize, 6usize, 11u32, 2120804u32);
    emu.sli_no_count(17usize, 6usize, 21u32, 2120808u32);
    emu.orr_no_count(17usize, 17usize, 11usize, 2120812u32);
    emu.sri_no_count(11usize, 6usize, 25u32, 2120816u32);
    emu.sli_no_count(28usize, 6usize, 7u32, 2120820u32);
    emu.orr_no_count(28usize, 28usize, 11usize, 2120824u32);
    emu.lw_no_count(11usize, 2usize, 496u32, 2120828u32)?;
    emu.adr_no_count(11usize, 11usize, 7usize, 2120832u32);
    emu.xrr_no_count(29usize, 24usize, 7usize, 2120836u32);
    emu.anr_no_count(29usize, 6usize, 29usize, 2120840u32);
    emu.xrr_no_count(7usize, 29usize, 7usize, 2120844u32);
    emu.sri_no_count(29usize, 14usize, 2u32, 2120848u32);
    emu.sli_no_count(31usize, 14usize, 30u32, 2120852u32);
    emu.orr_no_count(29usize, 31usize, 29usize, 2120856u32);
    emu.sri_no_count(31usize, 14usize, 13u32, 2120860u32);
    emu.sli_no_count(9usize, 14usize, 19u32, 2120864u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2120868u32);
    emu.sri_no_count(9usize, 14usize, 22u32, 2120872u32);
    emu.sli_no_count(19usize, 14usize, 10u32, 2120876u32);
    emu.orr_no_count(9usize, 19usize, 9usize, 2120880u32);
    emu.xrr_no_count(19usize, 8usize, 18usize, 2120884u32);
    emu.anr_no_count(19usize, 14usize, 19usize, 2120888u32);
    emu.anr_no_count(23usize, 8usize, 18usize, 2120892u32);
    emu.xrr_no_count(19usize, 19usize, 23usize, 2120896u32);
    emu.sri_no_count(23usize, 27usize, 10u32, 2120900u32);
    emu.sw_no_count(27usize, 2usize, 336u32, 2120904u32)?;
    emu.xrr_no_count(1usize, 10usize, 23usize, 2120908u32);
    emu.sri_no_count(10usize, 22usize, 10u32, 2120912u32);
    emu.sw_no_count(22usize, 2usize, 352u32, 2120916u32)?;
    emu.xrr_no_count(10usize, 12usize, 10usize, 2120920u32);
    emu.xrr_no_count(12usize, 13usize, 17usize, 2120924u32);
    emu.adr_no_count(7usize, 30usize, 7usize, 2120928u32);
    emu.xrr_no_count(13usize, 29usize, 31usize, 2120932u32);
    emu.lw_no_count(17usize, 2usize, 452u32, 2120936u32)?;
    emu.lw_no_count(15usize, 2usize, 316u32, 2120940u32)?;
    emu.adr_no_count(17usize, 15usize, 17usize, 2120944u32);
    emu.lw_no_count(15usize, 2usize, 416u32, 2120948u32)?;
    emu.adr_no_count(17usize, 17usize, 15usize, 2120952u32);
    emu.adr_no_count(1usize, 17usize, 1usize, 2120956u32);
    emu.lw_no_count(17usize, 2usize, 304u32, 2120960u32)?;
    emu.lw_no_count(15usize, 2usize, 376u32, 2120964u32)?;
    emu.adr_no_count(17usize, 17usize, 15usize, 2120968u32);
    emu.adr_no_count(17usize, 17usize, 5usize, 2120972u32);
    emu.adr_no_count(15usize, 17usize, 10usize, 2120976u32);
    emu.sw_no_count(15usize, 2usize, 496u32, 2120980u32)?;
    emu.xrr_no_count(10usize, 12usize, 28usize, 2120984u32);
    emu.xrr_no_count(13usize, 13usize, 9usize, 2120988u32);
    emu.lw_no_count(12usize, 2usize, 196u32, 2120992u32)?;
    emu.adr_no_count(7usize, 7usize, 12usize, 2120996u32);
    emu.adr_no_count(7usize, 7usize, 10usize, 2121000u32);
    emu.adr_no_count(10usize, 13usize, 19usize, 2121004u32);
    emu.sri_no_count(12usize, 1usize, 17u32, 2121008u32);
    emu.sli_no_count(13usize, 1usize, 15u32, 2121012u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2121016u32);
    emu.sri_no_count(13usize, 1usize, 19u32, 2121020u32);
    emu.sli_no_count(17usize, 1usize, 13u32, 2121024u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2121028u32);
    emu.sri_no_count(17usize, 15usize, 17u32, 2121032u32);
    emu.sli_no_count(28usize, 15usize, 15u32, 2121036u32);
    emu.orr_no_count(17usize, 28usize, 17usize, 2121040u32);
    emu.sri_no_count(28usize, 15usize, 19u32, 2121044u32);
    emu.sli_no_count(29usize, 15usize, 13u32, 2121048u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2121052u32);
    emu.adr_no_count(16usize, 10usize, 7usize, 2121056u32);
    emu.adr_no_count(7usize, 7usize, 21usize, 2121060u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2121064u32);
    emu.xrr_no_count(13usize, 17usize, 28usize, 2121068u32);
    emu.sri_no_count(17usize, 1usize, 10u32, 2121072u32);
    emu.sw_no_count(1usize, 2usize, 356u32, 2121076u32)?;
    emu.xrr_no_count(12usize, 12usize, 17usize, 2121080u32);
    emu.sri_no_count(17usize, 15usize, 10u32, 2121084u32);
    emu.xrr_no_count(13usize, 13usize, 17usize, 2121088u32);
    emu.sri_no_count(17usize, 7usize, 6u32, 2121092u32);
    emu.sli_no_count(28usize, 7usize, 26u32, 2121096u32);
    emu.orr_no_count(17usize, 28usize, 17usize, 2121100u32);
    emu.sri_no_count(28usize, 7usize, 11u32, 2121104u32);
    emu.sli_no_count(29usize, 7usize, 21u32, 2121108u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2121112u32);
    emu.sri_no_count(29usize, 7usize, 25u32, 2121116u32);
    emu.sli_no_count(30usize, 7usize, 7u32, 2121120u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2121124u32);
    emu.lw_no_count(30usize, 2usize, 440u32, 2121128u32)?;
    emu.adr_no_count(30usize, 30usize, 24usize, 2121132u32);
    emu.xrr_no_count(31usize, 6usize, 24usize, 2121136u32);
    emu.anr_no_count(31usize, 7usize, 31usize, 2121140u32);
    emu.xrr_no_count(31usize, 31usize, 24usize, 2121144u32);
    emu.sri_no_count(9usize, 16usize, 2u32, 2121148u32);
    emu.sli_no_count(19usize, 16usize, 30u32, 2121152u32);
    emu.orr_no_count(9usize, 19usize, 9usize, 2121156u32);
    emu.sri_no_count(19usize, 16usize, 13u32, 2121160u32);
    emu.sli_no_count(21usize, 16usize, 19u32, 2121164u32);
    emu.orr_no_count(19usize, 21usize, 19usize, 2121168u32);
    emu.sri_no_count(21usize, 16usize, 22u32, 2121172u32);
    emu.sli_no_count(23usize, 16usize, 10u32, 2121176u32);
    emu.orr_no_count(21usize, 23usize, 21usize, 2121180u32);
    emu.xrr_no_count(23usize, 14usize, 8usize, 2121184u32);
    emu.anr_no_count(23usize, 16usize, 23usize, 2121188u32);
    emu.anr_no_count(24usize, 14usize, 8usize, 2121192u32);
    emu.xrr_no_count(23usize, 23usize, 24usize, 2121196u32);
    emu.lw_no_count(24usize, 2usize, 384u32, 2121200u32)?;
    emu.lw_no_count(10usize, 2usize, 360u32, 2121204u32)?;
    emu.adr_no_count(24usize, 10usize, 24usize, 2121208u32);
    emu.lw_no_count(10usize, 2usize, 420u32, 2121212u32)?;
    emu.adr_no_count(24usize, 24usize, 10usize, 2121216u32);
    emu.adr_no_count(5usize, 24usize, 12usize, 2121220u32);
    emu.lw_no_count(12usize, 2usize, 300u32, 2121224u32)?;
    emu.lw_no_count(10usize, 2usize, 488u32, 2121228u32)?;
    emu.adr_no_count(12usize, 12usize, 10usize, 2121232u32);
    emu.adr_no_count(12usize, 12usize, 25usize, 2121236u32);
    emu.adr_no_count(15usize, 12usize, 13usize, 2121240u32);
    emu.xrr_no_count(12usize, 17usize, 28usize, 2121244u32);
    emu.adr_no_count(11usize, 11usize, 31usize, 2121248u32);
    emu.xrr_no_count(13usize, 9usize, 19usize, 2121252u32);
    emu.xrr_no_count(12usize, 12usize, 29usize, 2121256u32);
    emu.xrr_no_count(13usize, 13usize, 21usize, 2121260u32);
    emu.sri_no_count(17usize, 5usize, 17u32, 2121264u32);
    emu.sli_no_count(28usize, 5usize, 15u32, 2121268u32);
    emu.orr_no_count(17usize, 28usize, 17usize, 2121272u32);
    emu.sri_no_count(28usize, 5usize, 19u32, 2121276u32);
    emu.sli_no_count(29usize, 5usize, 13u32, 2121280u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2121284u32);
    emu.sri_no_count(29usize, 15usize, 17u32, 2121288u32);
    emu.sli_no_count(31usize, 15usize, 15u32, 2121292u32);
    emu.orr_no_count(29usize, 31usize, 29usize, 2121296u32);
    emu.sri_no_count(31usize, 15usize, 19u32, 2121300u32);
    emu.sli_no_count(9usize, 15usize, 13u32, 2121304u32);
    emu.sw_no_count(15usize, 2usize, 440u32, 2121308u32)?;
    emu.orr_no_count(31usize, 9usize, 31usize, 2121312u32);
    emu.lw_no_count(9usize, 2usize, 192u32, 2121316u32)?;
    emu.adr_no_count(11usize, 11usize, 9usize, 2121320u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2121324u32);
    emu.adr_no_count(13usize, 13usize, 23usize, 2121328u32);
    emu.xrr_no_count(12usize, 17usize, 28usize, 2121332u32);
    emu.xrr_no_count(17usize, 29usize, 31usize, 2121336u32);
    emu.adr_no_count(24usize, 13usize, 11usize, 2121340u32);
    emu.adr_no_count(18usize, 11usize, 18usize, 2121344u32);
    emu.sri_no_count(11usize, 5usize, 10u32, 2121348u32);
    emu.sw_no_count(5usize, 2usize, 332u32, 2121352u32)?;
    emu.xrr_no_count(11usize, 12usize, 11usize, 2121356u32);
    emu.sri_no_count(12usize, 15usize, 10u32, 2121360u32);
    emu.xrr_no_count(12usize, 17usize, 12usize, 2121364u32);
    emu.lw_no_count(13usize, 2usize, 412u32, 2121368u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2121372u32)?;
    emu.adr_no_count(13usize, 10usize, 13usize, 2121376u32);
    emu.adr_no_count(13usize, 13usize, 26usize, 2121380u32);
    emu.adr_no_count(10usize, 13usize, 11usize, 2121384u32);
    emu.lw_no_count(11usize, 2usize, 288u32, 2121388u32)?;
    emu.adr_no_count(11usize, 11usize, 20usize, 2121392u32);
    emu.adr_no_count(11usize, 11usize, 27usize, 2121396u32);
    emu.adr_no_count(15usize, 11usize, 12usize, 2121400u32);
    emu.sri_no_count(11usize, 18usize, 6u32, 2121404u32);
    emu.sli_no_count(12usize, 18usize, 26u32, 2121408u32);
    emu.orr_no_count(12usize, 12usize, 11usize, 2121412u32);
    emu.sri_no_count(11usize, 18usize, 11u32, 2121416u32);
    emu.sli_no_count(13usize, 18usize, 21u32, 2121420u32);
    emu.orr_no_count(13usize, 13usize, 11usize, 2121424u32);
    emu.sri_no_count(11usize, 18usize, 25u32, 2121428u32);
    emu.sli_no_count(17usize, 18usize, 7u32, 2121432u32);
    emu.orr_no_count(17usize, 17usize, 11usize, 2121436u32);
    emu.lw_no_count(11usize, 2usize, 504u32, 2121440u32)?;
    emu.adr_no_count(11usize, 11usize, 6usize, 2121444u32);
    emu.xrr_no_count(28usize, 7usize, 6usize, 2121448u32);
    emu.anr_no_count(28usize, 18usize, 28usize, 2121452u32);
    emu.xrr_no_count(6usize, 28usize, 6usize, 2121456u32);
    emu.sri_no_count(28usize, 24usize, 2u32, 2121460u32);
    emu.sli_no_count(29usize, 24usize, 30u32, 2121464u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2121468u32);
    emu.sri_no_count(29usize, 24usize, 13u32, 2121472u32);
    emu.sli_no_count(31usize, 24usize, 19u32, 2121476u32);
    emu.orr_no_count(29usize, 31usize, 29usize, 2121480u32);
    emu.sri_no_count(31usize, 24usize, 22u32, 2121484u32);
    emu.sli_no_count(9usize, 24usize, 10u32, 2121488u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2121492u32);
    emu.xrr_no_count(9usize, 16usize, 14usize, 2121496u32);
    emu.anr_no_count(9usize, 24usize, 9usize, 2121500u32);
    emu.anr_no_count(19usize, 16usize, 14usize, 2121504u32);
    emu.xrr_no_count(9usize, 9usize, 19usize, 2121508u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2121512u32);
    emu.adr_no_count(6usize, 30usize, 6usize, 2121516u32);
    emu.xrr_no_count(13usize, 28usize, 29usize, 2121520u32);
    emu.sri_no_count(28usize, 10usize, 17u32, 2121524u32);
    emu.sli_no_count(29usize, 10usize, 15u32, 2121528u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2121532u32);
    emu.sri_no_count(29usize, 10usize, 19u32, 2121536u32);
    emu.sli_no_count(30usize, 10usize, 13u32, 2121540u32);
    emu.sw_no_count(10usize, 2usize, 364u32, 2121544u32)?;
    emu.orr_no_count(29usize, 30usize, 29usize, 2121548u32);
    emu.sri_no_count(30usize, 15usize, 17u32, 2121552u32);
    emu.sli_no_count(19usize, 15usize, 15u32, 2121556u32);
    emu.orr_no_count(30usize, 19usize, 30usize, 2121560u32);
    emu.sri_no_count(19usize, 15usize, 19u32, 2121564u32);
    emu.sli_no_count(21usize, 15usize, 13u32, 2121568u32);
    emu.sw_no_count(15usize, 2usize, 492u32, 2121572u32)?;
    emu.orr_no_count(19usize, 21usize, 19usize, 2121576u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2121580u32);
    emu.xrr_no_count(13usize, 13usize, 31usize, 2121584u32);
    emu.xrr_no_count(17usize, 28usize, 29usize, 2121588u32);
    emu.xrr_no_count(28usize, 30usize, 19usize, 2121592u32);
    emu.lw_no_count(29usize, 2usize, 188u32, 2121596u32)?;
    emu.adr_no_count(6usize, 6usize, 29usize, 2121600u32);
    emu.adr_no_count(12usize, 6usize, 12usize, 2121604u32);
    emu.adr_no_count(13usize, 13usize, 9usize, 2121608u32);
    emu.sri_no_count(6usize, 10usize, 10u32, 2121612u32);
    emu.xrr_no_count(17usize, 17usize, 6usize, 2121616u32);
    emu.sri_no_count(6usize, 15usize, 10u32, 2121620u32);
    emu.xrr_no_count(6usize, 28usize, 6usize, 2121624u32);
    emu.adr_no_count(21usize, 13usize, 12usize, 2121628u32);
    emu.adr_no_count(8usize, 12usize, 8usize, 2121632u32);
    emu.lw_no_count(27usize, 2usize, 276u32, 2121636u32)?;
    emu.lw_no_count(12usize, 2usize, 292u32, 2121640u32)?;
    emu.adr_no_count(12usize, 12usize, 27usize, 2121644u32);
    emu.adr_no_count(12usize, 12usize, 22usize, 2121648u32);
    emu.adr_no_count(10usize, 12usize, 17usize, 2121652u32);
    emu.lw_no_count(22usize, 2usize, 272u32, 2121656u32)?;
    emu.lw_no_count(12usize, 2usize, 280u32, 2121660u32)?;
    emu.adr_no_count(12usize, 12usize, 22usize, 2121664u32);
    emu.adr_no_count(12usize, 12usize, 1usize, 2121668u32);
    emu.adr_no_count(15usize, 12usize, 6usize, 2121672u32);
    emu.sri_no_count(12usize, 8usize, 6u32, 2121676u32);
    emu.sli_no_count(13usize, 8usize, 26u32, 2121680u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2121684u32);
    emu.sri_no_count(13usize, 8usize, 11u32, 2121688u32);
    emu.sli_no_count(17usize, 8usize, 21u32, 2121692u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2121696u32);
    emu.sri_no_count(17usize, 8usize, 25u32, 2121700u32);
    emu.sli_no_count(6usize, 8usize, 7u32, 2121704u32);
    emu.orr_no_count(17usize, 6usize, 17usize, 2121708u32);
    emu.lw_no_count(6usize, 2usize, 444u32, 2121712u32)?;
    emu.adr_no_count(6usize, 6usize, 7usize, 2121716u32);
    emu.xrr_no_count(28usize, 18usize, 7usize, 2121720u32);
    emu.anr_no_count(28usize, 8usize, 28usize, 2121724u32);
    emu.xrr_no_count(7usize, 28usize, 7usize, 2121728u32);
    emu.sri_no_count(28usize, 21usize, 2u32, 2121732u32);
    emu.sli_no_count(29usize, 21usize, 30u32, 2121736u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2121740u32);
    emu.sri_no_count(29usize, 21usize, 13u32, 2121744u32);
    emu.sli_no_count(30usize, 21usize, 19u32, 2121748u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2121752u32);
    emu.sri_no_count(30usize, 21usize, 22u32, 2121756u32);
    emu.sli_no_count(31usize, 21usize, 10u32, 2121760u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2121764u32);
    emu.xrr_no_count(31usize, 24usize, 16usize, 2121768u32);
    emu.anr_no_count(31usize, 21usize, 31usize, 2121772u32);
    emu.anr_no_count(9usize, 24usize, 16usize, 2121776u32);
    emu.xrr_no_count(31usize, 31usize, 9usize, 2121780u32);
    emu.sri_no_count(9usize, 10usize, 17u32, 2121784u32);
    emu.sli_no_count(19usize, 10usize, 15u32, 2121788u32);
    emu.orr_no_count(9usize, 19usize, 9usize, 2121792u32);
    emu.sri_no_count(19usize, 10usize, 19u32, 2121796u32);
    emu.sli_no_count(23usize, 10usize, 13u32, 2121800u32);
    emu.sw_no_count(10usize, 2usize, 408u32, 2121804u32)?;
    emu.orr_no_count(19usize, 23usize, 19usize, 2121808u32);
    emu.sri_no_count(23usize, 15usize, 17u32, 2121812u32);
    emu.sli_no_count(26usize, 15usize, 15u32, 2121816u32);
    emu.orr_no_count(23usize, 26usize, 23usize, 2121820u32);
    emu.sri_no_count(26usize, 15usize, 19u32, 2121824u32);
    emu.sli_no_count(1usize, 15usize, 13u32, 2121828u32);
    emu.sw_no_count(15usize, 2usize, 504u32, 2121832u32)?;
    emu.orr_no_count(26usize, 1usize, 26usize, 2121836u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2121840u32);
    emu.adr_no_count(7usize, 11usize, 7usize, 2121844u32);
    emu.xrr_no_count(11usize, 28usize, 29usize, 2121848u32);
    emu.xrr_no_count(13usize, 9usize, 19usize, 2121852u32);
    emu.xrr_no_count(28usize, 23usize, 26usize, 2121856u32);
    emu.xrr_no_count(23usize, 12usize, 17usize, 2121860u32);
    emu.xrr_no_count(12usize, 11usize, 30usize, 2121864u32);
    emu.sri_no_count(11usize, 10usize, 10u32, 2121868u32);
    emu.xrr_no_count(30usize, 13usize, 11usize, 2121872u32);
    emu.sri_no_count(11usize, 15usize, 10u32, 2121876u32);
    emu.xrr_no_count(11usize, 28usize, 11usize, 2121880u32);
    emu.lw_no_count(13usize, 2usize, 184u32, 2121884u32)?;
    emu.adr_no_count(7usize, 7usize, 13usize, 2121888u32);
    emu.adr_no_count(23usize, 7usize, 23usize, 2121892u32);
    emu.adr_no_count(7usize, 12usize, 31usize, 2121896u32);
    emu.lw_no_count(20usize, 2usize, 284u32, 2121900u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2121904u32)?;
    emu.adr_no_count(12usize, 20usize, 12usize, 2121908u32);
    emu.lw_no_count(10usize, 2usize, 496u32, 2121912u32)?;
    emu.adr_no_count(12usize, 12usize, 10usize, 2121916u32);
    emu.adr_no_count(12usize, 12usize, 30usize, 2121920u32);
    emu.sw_no_count(12usize, 2usize, 444u32, 2121924u32)?;
    emu.lw_no_count(25usize, 2usize, 416u32, 2121928u32)?;
    emu.lw_no_count(12usize, 2usize, 296u32, 2121932u32)?;
    emu.adr_no_count(12usize, 25usize, 12usize, 2121936u32);
    emu.adr_no_count(12usize, 12usize, 5usize, 2121940u32);
    emu.adr_no_count(11usize, 12usize, 11usize, 2121944u32);
    emu.sw_no_count(11usize, 2usize, 360u32, 2121948u32)?;
    emu.adr_no_count(7usize, 7usize, 23usize, 2121952u32);
    emu.adr_no_count(23usize, 23usize, 14usize, 2121956u32);
    emu.sri_no_count(11usize, 23usize, 6u32, 2121960u32);
    emu.sli_no_count(12usize, 23usize, 26u32, 2121964u32);
    emu.orr_no_count(11usize, 12usize, 11usize, 2121968u32);
    emu.sri_no_count(12usize, 23usize, 11u32, 2121972u32);
    emu.sli_no_count(13usize, 23usize, 21u32, 2121976u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2121980u32);
    emu.sri_no_count(13usize, 23usize, 25u32, 2121984u32);
    emu.sli_no_count(14usize, 23usize, 7u32, 2121988u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2121992u32);
    emu.lw_no_count(17usize, 2usize, 500u32, 2121996u32)?;
    emu.adr_no_count(17usize, 17usize, 18usize, 2122000u32);
    emu.xrr_no_count(14usize, 8usize, 18usize, 2122004u32);
    emu.anr_no_count(14usize, 23usize, 14usize, 2122008u32);
    emu.xrr_no_count(14usize, 14usize, 18usize, 2122012u32);
    emu.sri_no_count(28usize, 7usize, 2u32, 2122016u32);
    emu.sli_no_count(29usize, 7usize, 30u32, 2122020u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2122024u32);
    emu.sri_no_count(29usize, 7usize, 13u32, 2122028u32);
    emu.sli_no_count(30usize, 7usize, 19u32, 2122032u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2122036u32);
    emu.sri_no_count(30usize, 7usize, 22u32, 2122040u32);
    emu.sli_no_count(31usize, 7usize, 10u32, 2122044u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2122048u32);
    emu.xrr_no_count(31usize, 21usize, 24usize, 2122052u32);
    emu.anr_no_count(31usize, 7usize, 31usize, 2122056u32);
    emu.anr_no_count(9usize, 21usize, 24usize, 2122060u32);
    emu.xrr_no_count(31usize, 31usize, 9usize, 2122064u32);
    emu.xrr_no_count(11usize, 11usize, 12usize, 2122068u32);
    emu.adr_no_count(14usize, 6usize, 14usize, 2122072u32);
    emu.xrr_no_count(12usize, 28usize, 29usize, 2122076u32);
    emu.xrr_no_count(11usize, 11usize, 13usize, 2122080u32);
    emu.xrr_no_count(12usize, 12usize, 30usize, 2122084u32);
    emu.lw_no_count(13usize, 2usize, 180u32, 2122088u32)?;
    emu.adr_no_count(14usize, 14usize, 13usize, 2122092u32);
    emu.adr_no_count(11usize, 14usize, 11usize, 2122096u32);
    emu.adr_no_count(12usize, 12usize, 31usize, 2122100u32);
    emu.adr_no_count(14usize, 12usize, 11usize, 2122104u32);
    emu.adr_no_count(11usize, 11usize, 16usize, 2122108u32);
    emu.sri_no_count(10usize, 11usize, 6u32, 2122112u32);
    emu.sli_no_count(12usize, 11usize, 26u32, 2122116u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2122120u32);
    emu.sri_no_count(12usize, 11usize, 11u32, 2122124u32);
    emu.sli_no_count(13usize, 11usize, 21u32, 2122128u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2122132u32);
    emu.sri_no_count(13usize, 11usize, 25u32, 2122136u32);
    emu.sli_no_count(6usize, 11usize, 7u32, 2122140u32);
    emu.orr_no_count(13usize, 6usize, 13usize, 2122144u32);
    emu.lw_no_count(6usize, 2usize, 476u32, 2122148u32)?;
    emu.adr_no_count(6usize, 6usize, 8usize, 2122152u32);
    emu.xrr_no_count(28usize, 23usize, 8usize, 2122156u32);
    emu.anr_no_count(28usize, 11usize, 28usize, 2122160u32);
    emu.xrr_no_count(28usize, 28usize, 8usize, 2122164u32);
    emu.sri_no_count(29usize, 14usize, 2u32, 2122168u32);
    emu.sli_no_count(30usize, 14usize, 30u32, 2122172u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2122176u32);
    emu.sri_no_count(30usize, 14usize, 13u32, 2122180u32);
    emu.sli_no_count(31usize, 14usize, 19u32, 2122184u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2122188u32);
    emu.sri_no_count(31usize, 14usize, 22u32, 2122192u32);
    emu.sli_no_count(8usize, 14usize, 10u32, 2122196u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2122200u32);
    emu.xrr_no_count(8usize, 7usize, 21usize, 2122204u32);
    emu.anr_no_count(8usize, 14usize, 8usize, 2122208u32);
    emu.anr_no_count(9usize, 7usize, 21usize, 2122212u32);
    emu.xrr_no_count(8usize, 8usize, 9usize, 2122216u32);
    emu.xrr_no_count(10usize, 10usize, 12usize, 2122220u32);
    emu.adr_no_count(17usize, 17usize, 28usize, 2122224u32);
    emu.xrr_no_count(12usize, 29usize, 30usize, 2122228u32);
    emu.xrr_no_count(10usize, 10usize, 13usize, 2122232u32);
    emu.xrr_no_count(12usize, 12usize, 31usize, 2122236u32);
    emu.lw_no_count(13usize, 2usize, 176u32, 2122240u32)?;
    emu.adr_no_count(17usize, 17usize, 13usize, 2122244u32);
    emu.adr_no_count(17usize, 17usize, 10usize, 2122248u32);
    emu.adr_no_count(10usize, 12usize, 8usize, 2122252u32);
    emu.adr_no_count(10usize, 10usize, 17usize, 2122256u32);
    emu.adr_no_count(24usize, 17usize, 24usize, 2122260u32);
    emu.sri_no_count(12usize, 24usize, 6u32, 2122264u32);
    emu.sli_no_count(13usize, 24usize, 26u32, 2122268u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2122272u32);
    emu.sri_no_count(13usize, 24usize, 11u32, 2122276u32);
    emu.sli_no_count(17usize, 24usize, 21u32, 2122280u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2122284u32);
    emu.sri_no_count(17usize, 24usize, 25u32, 2122288u32);
    emu.sli_no_count(28usize, 24usize, 7u32, 2122292u32);
    emu.orr_no_count(17usize, 28usize, 17usize, 2122296u32);
    emu.lw_no_count(28usize, 2usize, 404u32, 2122300u32)?;
    emu.adr_no_count(28usize, 28usize, 23usize, 2122304u32);
    emu.xrr_no_count(29usize, 11usize, 23usize, 2122308u32);
    emu.anr_no_count(29usize, 24usize, 29usize, 2122312u32);
    emu.xrr_no_count(29usize, 29usize, 23usize, 2122316u32);
    emu.sri_no_count(30usize, 10usize, 2u32, 2122320u32);
    emu.sli_no_count(31usize, 10usize, 30u32, 2122324u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2122328u32);
    emu.sri_no_count(31usize, 10usize, 13u32, 2122332u32);
    emu.sli_no_count(8usize, 10usize, 19u32, 2122336u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2122340u32);
    emu.sri_no_count(8usize, 10usize, 22u32, 2122344u32);
    emu.sli_no_count(9usize, 10usize, 10u32, 2122348u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2122352u32);
    emu.xrr_no_count(9usize, 14usize, 7usize, 2122356u32);
    emu.anr_no_count(9usize, 10usize, 9usize, 2122360u32);
    emu.anr_no_count(18usize, 14usize, 7usize, 2122364u32);
    emu.xrr_no_count(9usize, 9usize, 18usize, 2122368u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2122372u32);
    emu.adr_no_count(6usize, 6usize, 29usize, 2122376u32);
    emu.xrr_no_count(13usize, 30usize, 31usize, 2122380u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2122384u32);
    emu.xrr_no_count(13usize, 13usize, 8usize, 2122388u32);
    emu.lw_no_count(17usize, 2usize, 172u32, 2122392u32)?;
    emu.adr_no_count(6usize, 6usize, 17usize, 2122396u32);
    emu.adr_no_count(12usize, 6usize, 12usize, 2122400u32);
    emu.adr_no_count(13usize, 13usize, 9usize, 2122404u32);
    emu.adr_no_count(6usize, 13usize, 12usize, 2122408u32);
    emu.adr_no_count(21usize, 12usize, 21usize, 2122412u32);
    emu.sri_no_count(12usize, 21usize, 6u32, 2122416u32);
    emu.sli_no_count(13usize, 21usize, 26u32, 2122420u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2122424u32);
    emu.sri_no_count(13usize, 21usize, 11u32, 2122428u32);
    emu.sli_no_count(17usize, 21usize, 21u32, 2122432u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2122436u32);
    emu.sri_no_count(17usize, 21usize, 25u32, 2122440u32);
    emu.sli_no_count(29usize, 21usize, 7u32, 2122444u32);
    emu.orr_no_count(17usize, 29usize, 17usize, 2122448u32);
    emu.lw_no_count(29usize, 2usize, 472u32, 2122452u32)?;
    emu.adr_no_count(29usize, 29usize, 11usize, 2122456u32);
    emu.xrr_no_count(30usize, 24usize, 11usize, 2122460u32);
    emu.anr_no_count(30usize, 21usize, 30usize, 2122464u32);
    emu.xrr_no_count(11usize, 30usize, 11usize, 2122468u32);
    emu.sri_no_count(30usize, 6usize, 2u32, 2122472u32);
    emu.sli_no_count(31usize, 6usize, 30u32, 2122476u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2122480u32);
    emu.sri_no_count(31usize, 6usize, 13u32, 2122484u32);
    emu.sli_no_count(8usize, 6usize, 19u32, 2122488u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2122492u32);
    emu.sri_no_count(8usize, 6usize, 22u32, 2122496u32);
    emu.sli_no_count(9usize, 6usize, 10u32, 2122500u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2122504u32);
    emu.xrr_no_count(9usize, 10usize, 14usize, 2122508u32);
    emu.anr_no_count(9usize, 6usize, 9usize, 2122512u32);
    emu.anr_no_count(18usize, 10usize, 14usize, 2122516u32);
    emu.xrr_no_count(9usize, 9usize, 18usize, 2122520u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2122524u32);
    emu.adr_no_count(11usize, 28usize, 11usize, 2122528u32);
    emu.xrr_no_count(13usize, 30usize, 31usize, 2122532u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2122536u32);
    emu.xrr_no_count(13usize, 13usize, 8usize, 2122540u32);
    emu.lw_no_count(17usize, 2usize, 168u32, 2122544u32)?;
    emu.adr_no_count(11usize, 11usize, 17usize, 2122548u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2122552u32);
    emu.adr_no_count(13usize, 13usize, 9usize, 2122556u32);
    emu.adr_no_count(8usize, 13usize, 11usize, 2122560u32);
    emu.adr_no_count(11usize, 11usize, 7usize, 2122564u32);
    emu.sri_no_count(12usize, 11usize, 6u32, 2122568u32);
    emu.sli_no_count(13usize, 11usize, 26u32, 2122572u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2122576u32);
    emu.sri_no_count(13usize, 11usize, 11u32, 2122580u32);
    emu.sli_no_count(17usize, 11usize, 21u32, 2122584u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2122588u32);
    emu.sri_no_count(17usize, 11usize, 25u32, 2122592u32);
    emu.sli_no_count(7usize, 11usize, 7u32, 2122596u32);
    emu.orr_no_count(17usize, 7usize, 17usize, 2122600u32);
    emu.lw_no_count(28usize, 2usize, 400u32, 2122604u32)?;
    emu.adr_no_count(28usize, 28usize, 24usize, 2122608u32);
    emu.xrr_no_count(7usize, 21usize, 24usize, 2122612u32);
    emu.anr_no_count(7usize, 11usize, 7usize, 2122616u32);
    emu.xrr_no_count(7usize, 7usize, 24usize, 2122620u32);
    emu.sri_no_count(30usize, 8usize, 2u32, 2122624u32);
    emu.sli_no_count(31usize, 8usize, 30u32, 2122628u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2122632u32);
    emu.sri_no_count(31usize, 8usize, 13u32, 2122636u32);
    emu.sli_no_count(9usize, 8usize, 19u32, 2122640u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2122644u32);
    emu.sri_no_count(9usize, 8usize, 22u32, 2122648u32);
    emu.sli_no_count(18usize, 8usize, 10u32, 2122652u32);
    emu.orr_no_count(9usize, 18usize, 9usize, 2122656u32);
    emu.xrr_no_count(18usize, 6usize, 10usize, 2122660u32);
    emu.anr_no_count(18usize, 8usize, 18usize, 2122664u32);
    emu.anr_no_count(19usize, 6usize, 10usize, 2122668u32);
    emu.xrr_no_count(18usize, 18usize, 19usize, 2122672u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2122676u32);
    emu.adr_no_count(7usize, 29usize, 7usize, 2122680u32);
    emu.xrr_no_count(13usize, 30usize, 31usize, 2122684u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2122688u32);
    emu.xrr_no_count(13usize, 13usize, 9usize, 2122692u32);
    emu.lw_no_count(17usize, 2usize, 164u32, 2122696u32)?;
    emu.adr_no_count(7usize, 7usize, 17usize, 2122700u32);
    emu.adr_no_count(12usize, 7usize, 12usize, 2122704u32);
    emu.adr_no_count(13usize, 13usize, 18usize, 2122708u32);
    emu.adr_no_count(7usize, 13usize, 12usize, 2122712u32);
    emu.adr_no_count(30usize, 12usize, 14usize, 2122716u32);
    emu.sri_no_count(12usize, 30usize, 6u32, 2122720u32);
    emu.sli_no_count(13usize, 30usize, 26u32, 2122724u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2122728u32);
    emu.sri_no_count(13usize, 30usize, 11u32, 2122732u32);
    emu.sli_no_count(14usize, 30usize, 21u32, 2122736u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2122740u32);
    emu.sri_no_count(14usize, 30usize, 25u32, 2122744u32);
    emu.sli_no_count(17usize, 30usize, 7u32, 2122748u32);
    emu.orr_no_count(14usize, 17usize, 14usize, 2122752u32);
    emu.lw_no_count(17usize, 2usize, 460u32, 2122756u32)?;
    emu.adr_no_count(17usize, 17usize, 21usize, 2122760u32);
    emu.xrr_no_count(29usize, 11usize, 21usize, 2122764u32);
    emu.anr_no_count(29usize, 30usize, 29usize, 2122768u32);
    emu.xrr_no_count(29usize, 29usize, 21usize, 2122772u32);
    emu.sri_no_count(31usize, 7usize, 2u32, 2122776u32);
    emu.sli_no_count(9usize, 7usize, 30u32, 2122780u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2122784u32);
    emu.sri_no_count(9usize, 7usize, 13u32, 2122788u32);
    emu.sli_no_count(18usize, 7usize, 19u32, 2122792u32);
    emu.orr_no_count(9usize, 18usize, 9usize, 2122796u32);
    emu.sri_no_count(18usize, 7usize, 22u32, 2122800u32);
    emu.sli_no_count(19usize, 7usize, 10u32, 2122804u32);
    emu.orr_no_count(18usize, 19usize, 18usize, 2122808u32);
    emu.xrr_no_count(19usize, 8usize, 6usize, 2122812u32);
    emu.anr_no_count(19usize, 7usize, 19usize, 2122816u32);
    emu.anr_no_count(21usize, 8usize, 6usize, 2122820u32);
    emu.xrr_no_count(19usize, 19usize, 21usize, 2122824u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2122828u32);
    emu.adr_no_count(28usize, 28usize, 29usize, 2122832u32);
    emu.xrr_no_count(13usize, 31usize, 9usize, 2122836u32);
    emu.xrr_no_count(12usize, 12usize, 14usize, 2122840u32);
    emu.xrr_no_count(13usize, 13usize, 18usize, 2122844u32);
    emu.lw_no_count(14usize, 2usize, 160u32, 2122848u32)?;
    emu.adr_no_count(28usize, 28usize, 14usize, 2122852u32);
    emu.adr_no_count(12usize, 28usize, 12usize, 2122856u32);
    emu.adr_no_count(13usize, 13usize, 19usize, 2122860u32);
    emu.adr_no_count(14usize, 13usize, 12usize, 2122864u32);
    emu.adr_no_count(18usize, 12usize, 10usize, 2122868u32);
    emu.sri_no_count(10usize, 18usize, 6u32, 2122872u32);
    emu.sli_no_count(12usize, 18usize, 26u32, 2122876u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2122880u32);
    emu.sri_no_count(12usize, 18usize, 11u32, 2122884u32);
    emu.sli_no_count(13usize, 18usize, 21u32, 2122888u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2122892u32);
    emu.sri_no_count(13usize, 18usize, 25u32, 2122896u32);
    emu.sli_no_count(28usize, 18usize, 7u32, 2122900u32);
    emu.orr_no_count(13usize, 28usize, 13usize, 2122904u32);
    emu.lw_no_count(28usize, 2usize, 396u32, 2122908u32)?;
    emu.adr_no_count(28usize, 28usize, 11usize, 2122912u32);
    emu.xrr_no_count(29usize, 30usize, 11usize, 2122916u32);
    emu.anr_no_count(29usize, 18usize, 29usize, 2122920u32);
    emu.xrr_no_count(11usize, 29usize, 11usize, 2122924u32);
    emu.sri_no_count(29usize, 14usize, 2u32, 2122928u32);
    emu.sli_no_count(31usize, 14usize, 30u32, 2122932u32);
    emu.orr_no_count(29usize, 31usize, 29usize, 2122936u32);
    emu.sri_no_count(31usize, 14usize, 13u32, 2122940u32);
    emu.sli_no_count(9usize, 14usize, 19u32, 2122944u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2122948u32);
    emu.sri_no_count(9usize, 14usize, 22u32, 2122952u32);
    emu.sli_no_count(19usize, 14usize, 10u32, 2122956u32);
    emu.orr_no_count(9usize, 19usize, 9usize, 2122960u32);
    emu.xrr_no_count(19usize, 7usize, 8usize, 2122964u32);
    emu.anr_no_count(19usize, 14usize, 19usize, 2122968u32);
    emu.anr_no_count(21usize, 7usize, 8usize, 2122972u32);
    emu.xrr_no_count(19usize, 19usize, 21usize, 2122976u32);
    emu.xrr_no_count(10usize, 10usize, 12usize, 2122980u32);
    emu.adr_no_count(11usize, 17usize, 11usize, 2122984u32);
    emu.xrr_no_count(12usize, 29usize, 31usize, 2122988u32);
    emu.xrr_no_count(10usize, 10usize, 13usize, 2122992u32);
    emu.xrr_no_count(12usize, 12usize, 9usize, 2122996u32);
    emu.lw_no_count(13usize, 2usize, 156u32, 2123000u32)?;
    emu.adr_no_count(11usize, 11usize, 13usize, 2123004u32);
    emu.adr_no_count(11usize, 11usize, 10usize, 2123008u32);
    emu.adr_no_count(10usize, 12usize, 19usize, 2123012u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2123016u32);
    emu.adr_no_count(11usize, 11usize, 6usize, 2123020u32);
    emu.sri_no_count(12usize, 11usize, 6u32, 2123024u32);
    emu.sli_no_count(13usize, 11usize, 26u32, 2123028u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2123032u32);
    emu.sri_no_count(13usize, 11usize, 11u32, 2123036u32);
    emu.sli_no_count(17usize, 11usize, 21u32, 2123040u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2123044u32);
    emu.sri_no_count(17usize, 11usize, 25u32, 2123048u32);
    emu.sli_no_count(6usize, 11usize, 7u32, 2123052u32);
    emu.orr_no_count(17usize, 6usize, 17usize, 2123056u32);
    emu.lw_no_count(29usize, 2usize, 456u32, 2123060u32)?;
    emu.adr_no_count(29usize, 29usize, 30usize, 2123064u32);
    emu.xrr_no_count(6usize, 18usize, 30usize, 2123068u32);
    emu.anr_no_count(6usize, 11usize, 6usize, 2123072u32);
    emu.xrr_no_count(6usize, 6usize, 30usize, 2123076u32);
    emu.sri_no_count(30usize, 10usize, 2u32, 2123080u32);
    emu.sli_no_count(31usize, 10usize, 30u32, 2123084u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2123088u32);
    emu.sri_no_count(31usize, 10usize, 13u32, 2123092u32);
    emu.sli_no_count(9usize, 10usize, 19u32, 2123096u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2123100u32);
    emu.sri_no_count(9usize, 10usize, 22u32, 2123104u32);
    emu.sli_no_count(19usize, 10usize, 10u32, 2123108u32);
    emu.orr_no_count(9usize, 19usize, 9usize, 2123112u32);
    emu.xrr_no_count(19usize, 14usize, 7usize, 2123116u32);
    emu.anr_no_count(19usize, 10usize, 19usize, 2123120u32);
    emu.anr_no_count(21usize, 14usize, 7usize, 2123124u32);
    emu.xrr_no_count(19usize, 19usize, 21usize, 2123128u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2123132u32);
    emu.adr_no_count(6usize, 28usize, 6usize, 2123136u32);
    emu.xrr_no_count(13usize, 30usize, 31usize, 2123140u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2123144u32);
    emu.xrr_no_count(13usize, 13usize, 9usize, 2123148u32);
    emu.lw_no_count(17usize, 2usize, 152u32, 2123152u32)?;
    emu.adr_no_count(6usize, 6usize, 17usize, 2123156u32);
    emu.adr_no_count(12usize, 6usize, 12usize, 2123160u32);
    emu.adr_no_count(13usize, 13usize, 19usize, 2123164u32);
    emu.adr_no_count(6usize, 13usize, 12usize, 2123168u32);
    emu.adr_no_count(30usize, 12usize, 8usize, 2123172u32);
    emu.sri_no_count(12usize, 30usize, 6u32, 2123176u32);
    emu.sli_no_count(13usize, 30usize, 26u32, 2123180u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2123184u32);
    emu.sri_no_count(13usize, 30usize, 11u32, 2123188u32);
    emu.sli_no_count(17usize, 30usize, 21u32, 2123192u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2123196u32);
    emu.sri_no_count(17usize, 30usize, 25u32, 2123200u32);
    emu.sli_no_count(28usize, 30usize, 7u32, 2123204u32);
    emu.orr_no_count(17usize, 28usize, 17usize, 2123208u32);
    emu.lw_no_count(28usize, 2usize, 392u32, 2123212u32)?;
    emu.adr_no_count(28usize, 28usize, 18usize, 2123216u32);
    emu.xrr_no_count(31usize, 11usize, 18usize, 2123220u32);
    emu.anr_no_count(31usize, 30usize, 31usize, 2123224u32);
    emu.xrr_no_count(31usize, 31usize, 18usize, 2123228u32);
    emu.sri_no_count(8usize, 6usize, 2u32, 2123232u32);
    emu.sli_no_count(9usize, 6usize, 30u32, 2123236u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2123240u32);
    emu.sri_no_count(9usize, 6usize, 13u32, 2123244u32);
    emu.sli_no_count(18usize, 6usize, 19u32, 2123248u32);
    emu.orr_no_count(9usize, 18usize, 9usize, 2123252u32);
    emu.sri_no_count(18usize, 6usize, 22u32, 2123256u32);
    emu.sli_no_count(19usize, 6usize, 10u32, 2123260u32);
    emu.orr_no_count(18usize, 19usize, 18usize, 2123264u32);
    emu.xrr_no_count(19usize, 10usize, 14usize, 2123268u32);
    emu.anr_no_count(19usize, 6usize, 19usize, 2123272u32);
    emu.anr_no_count(21usize, 10usize, 14usize, 2123276u32);
    emu.xrr_no_count(19usize, 19usize, 21usize, 2123280u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2123284u32);
    emu.adr_no_count(29usize, 29usize, 31usize, 2123288u32);
    emu.xrr_no_count(8usize, 8usize, 9usize, 2123292u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2123296u32);
    emu.xrr_no_count(13usize, 8usize, 18usize, 2123300u32);
    emu.lw_no_count(17usize, 2usize, 148u32, 2123304u32)?;
    emu.adr_no_count(29usize, 29usize, 17usize, 2123308u32);
    emu.adr_no_count(12usize, 29usize, 12usize, 2123312u32);
    emu.adr_no_count(13usize, 13usize, 19usize, 2123316u32);
    emu.adr_no_count(8usize, 13usize, 12usize, 2123320u32);
    emu.adr_no_count(18usize, 12usize, 7usize, 2123324u32);
    emu.sri_no_count(12usize, 18usize, 6u32, 2123328u32);
    emu.sli_no_count(13usize, 18usize, 26u32, 2123332u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2123336u32);
    emu.sri_no_count(13usize, 18usize, 11u32, 2123340u32);
    emu.sli_no_count(17usize, 18usize, 21u32, 2123344u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2123348u32);
    emu.sri_no_count(17usize, 18usize, 25u32, 2123352u32);
    emu.sli_no_count(7usize, 18usize, 7u32, 2123356u32);
    emu.orr_no_count(17usize, 7usize, 17usize, 2123360u32);
    emu.lw_no_count(29usize, 2usize, 436u32, 2123364u32)?;
    emu.adr_no_count(29usize, 29usize, 11usize, 2123368u32);
    emu.xrr_no_count(7usize, 30usize, 11usize, 2123372u32);
    emu.anr_no_count(7usize, 18usize, 7usize, 2123376u32);
    emu.xrr_no_count(11usize, 7usize, 11usize, 2123380u32);
    emu.sri_no_count(7usize, 8usize, 2u32, 2123384u32);
    emu.sli_no_count(31usize, 8usize, 30u32, 2123388u32);
    emu.orr_no_count(7usize, 31usize, 7usize, 2123392u32);
    emu.sri_no_count(31usize, 8usize, 13u32, 2123396u32);
    emu.sli_no_count(9usize, 8usize, 19u32, 2123400u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2123404u32);
    emu.sri_no_count(9usize, 8usize, 22u32, 2123408u32);
    emu.sli_no_count(19usize, 8usize, 10u32, 2123412u32);
    emu.orr_no_count(9usize, 19usize, 9usize, 2123416u32);
    emu.xrr_no_count(19usize, 6usize, 10usize, 2123420u32);
    emu.anr_no_count(19usize, 8usize, 19usize, 2123424u32);
    emu.anr_no_count(21usize, 6usize, 10usize, 2123428u32);
    emu.xrr_no_count(19usize, 19usize, 21usize, 2123432u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2123436u32);
    emu.adr_no_count(11usize, 28usize, 11usize, 2123440u32);
    emu.xrr_no_count(13usize, 7usize, 31usize, 2123444u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2123448u32);
    emu.xrr_no_count(13usize, 13usize, 9usize, 2123452u32);
    emu.lw_no_count(17usize, 2usize, 144u32, 2123456u32)?;
    emu.adr_no_count(11usize, 11usize, 17usize, 2123460u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2123464u32);
    emu.adr_no_count(13usize, 13usize, 19usize, 2123468u32);
    emu.adr_no_count(7usize, 13usize, 11usize, 2123472u32);
    emu.adr_no_count(11usize, 11usize, 14usize, 2123476u32);
    emu.sri_no_count(12usize, 11usize, 6u32, 2123480u32);
    emu.sli_no_count(13usize, 11usize, 26u32, 2123484u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2123488u32);
    emu.sri_no_count(13usize, 11usize, 11u32, 2123492u32);
    emu.sli_no_count(14usize, 11usize, 21u32, 2123496u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2123500u32);
    emu.sri_no_count(14usize, 11usize, 25u32, 2123504u32);
    emu.sli_no_count(17usize, 11usize, 7u32, 2123508u32);
    emu.orr_no_count(14usize, 17usize, 14usize, 2123512u32);
    emu.lw_no_count(17usize, 2usize, 468u32, 2123516u32)?;
    emu.adr_no_count(17usize, 17usize, 30usize, 2123520u32);
    emu.xrr_no_count(28usize, 18usize, 30usize, 2123524u32);
    emu.anr_no_count(28usize, 11usize, 28usize, 2123528u32);
    emu.xrr_no_count(28usize, 28usize, 30usize, 2123532u32);
    emu.sri_no_count(30usize, 7usize, 2u32, 2123536u32);
    emu.sli_no_count(31usize, 7usize, 30u32, 2123540u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2123544u32);
    emu.sri_no_count(31usize, 7usize, 13u32, 2123548u32);
    emu.sli_no_count(9usize, 7usize, 19u32, 2123552u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2123556u32);
    emu.sri_no_count(9usize, 7usize, 22u32, 2123560u32);
    emu.sli_no_count(19usize, 7usize, 10u32, 2123564u32);
    emu.orr_no_count(9usize, 19usize, 9usize, 2123568u32);
    emu.xrr_no_count(19usize, 8usize, 6usize, 2123572u32);
    emu.anr_no_count(19usize, 7usize, 19usize, 2123576u32);
    emu.anr_no_count(21usize, 8usize, 6usize, 2123580u32);
    emu.xrr_no_count(19usize, 19usize, 21usize, 2123584u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2123588u32);
    emu.adr_no_count(28usize, 29usize, 28usize, 2123592u32);
    emu.xrr_no_count(13usize, 30usize, 31usize, 2123596u32);
    emu.xrr_no_count(12usize, 12usize, 14usize, 2123600u32);
    emu.xrr_no_count(13usize, 13usize, 9usize, 2123604u32);
    emu.lw_no_count(14usize, 2usize, 140u32, 2123608u32)?;
    emu.adr_no_count(28usize, 28usize, 14usize, 2123612u32);
    emu.adr_no_count(12usize, 28usize, 12usize, 2123616u32);
    emu.adr_no_count(13usize, 13usize, 19usize, 2123620u32);
    emu.adr_no_count(14usize, 13usize, 12usize, 2123624u32);
    emu.adr_no_count(30usize, 12usize, 10usize, 2123628u32);
    emu.sri_no_count(10usize, 30usize, 6u32, 2123632u32);
    emu.sli_no_count(12usize, 30usize, 26u32, 2123636u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2123640u32);
    emu.sri_no_count(12usize, 30usize, 11u32, 2123644u32);
    emu.sli_no_count(13usize, 30usize, 21u32, 2123648u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2123652u32);
    emu.sri_no_count(13usize, 30usize, 25u32, 2123656u32);
    emu.sli_no_count(28usize, 30usize, 7u32, 2123660u32);
    emu.orr_no_count(13usize, 28usize, 13usize, 2123664u32);
    emu.lw_no_count(28usize, 2usize, 484u32, 2123668u32)?;
    emu.adr_no_count(28usize, 28usize, 18usize, 2123672u32);
    emu.xrr_no_count(29usize, 11usize, 18usize, 2123676u32);
    emu.anr_no_count(29usize, 30usize, 29usize, 2123680u32);
    emu.xrr_no_count(29usize, 29usize, 18usize, 2123684u32);
    emu.sri_no_count(31usize, 14usize, 2u32, 2123688u32);
    emu.sli_no_count(9usize, 14usize, 30u32, 2123692u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2123696u32);
    emu.sri_no_count(9usize, 14usize, 13u32, 2123700u32);
    emu.sli_no_count(18usize, 14usize, 19u32, 2123704u32);
    emu.orr_no_count(9usize, 18usize, 9usize, 2123708u32);
    emu.sri_no_count(18usize, 14usize, 22u32, 2123712u32);
    emu.sli_no_count(19usize, 14usize, 10u32, 2123716u32);
    emu.orr_no_count(18usize, 19usize, 18usize, 2123720u32);
    emu.xrr_no_count(19usize, 7usize, 8usize, 2123724u32);
    emu.anr_no_count(19usize, 14usize, 19usize, 2123728u32);
    emu.anr_no_count(21usize, 7usize, 8usize, 2123732u32);
    emu.xrr_no_count(19usize, 19usize, 21usize, 2123736u32);
    emu.xrr_no_count(10usize, 10usize, 12usize, 2123740u32);
    emu.adr_no_count(17usize, 17usize, 29usize, 2123744u32);
    emu.xrr_no_count(12usize, 31usize, 9usize, 2123748u32);
    emu.xrr_no_count(10usize, 10usize, 13usize, 2123752u32);
    emu.xrr_no_count(12usize, 12usize, 18usize, 2123756u32);
    emu.lw_no_count(13usize, 2usize, 136u32, 2123760u32)?;
    emu.adr_no_count(17usize, 17usize, 13usize, 2123764u32);
    emu.adr_no_count(17usize, 17usize, 10usize, 2123768u32);
    emu.adr_no_count(10usize, 12usize, 19usize, 2123772u32);
    emu.adr_no_count(10usize, 10usize, 17usize, 2123776u32);
    emu.adr_no_count(18usize, 17usize, 6usize, 2123780u32);
    emu.sri_no_count(12usize, 18usize, 6u32, 2123784u32);
    emu.sli_no_count(13usize, 18usize, 26u32, 2123788u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2123792u32);
    emu.sri_no_count(13usize, 18usize, 11u32, 2123796u32);
    emu.sli_no_count(17usize, 18usize, 21u32, 2123800u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2123804u32);
    emu.sri_no_count(17usize, 18usize, 25u32, 2123808u32);
    emu.sli_no_count(6usize, 18usize, 7u32, 2123812u32);
    emu.orr_no_count(17usize, 6usize, 17usize, 2123816u32);
    emu.lw_no_count(29usize, 2usize, 464u32, 2123820u32)?;
    emu.adr_no_count(29usize, 29usize, 11usize, 2123824u32);
    emu.xrr_no_count(6usize, 30usize, 11usize, 2123828u32);
    emu.anr_no_count(6usize, 18usize, 6usize, 2123832u32);
    emu.xrr_no_count(11usize, 6usize, 11usize, 2123836u32);
    emu.sri_no_count(6usize, 10usize, 2u32, 2123840u32);
    emu.sli_no_count(31usize, 10usize, 30u32, 2123844u32);
    emu.orr_no_count(6usize, 31usize, 6usize, 2123848u32);
    emu.sri_no_count(31usize, 10usize, 13u32, 2123852u32);
    emu.sli_no_count(9usize, 10usize, 19u32, 2123856u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2123860u32);
    emu.sri_no_count(9usize, 10usize, 22u32, 2123864u32);
    emu.sli_no_count(19usize, 10usize, 10u32, 2123868u32);
    emu.orr_no_count(9usize, 19usize, 9usize, 2123872u32);
    emu.xrr_no_count(19usize, 14usize, 7usize, 2123876u32);
    emu.anr_no_count(19usize, 10usize, 19usize, 2123880u32);
    emu.anr_no_count(21usize, 14usize, 7usize, 2123884u32);
    emu.xrr_no_count(19usize, 19usize, 21usize, 2123888u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2123892u32);
    emu.adr_no_count(11usize, 28usize, 11usize, 2123896u32);
    emu.xrr_no_count(13usize, 6usize, 31usize, 2123900u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2123904u32);
    emu.xrr_no_count(13usize, 13usize, 9usize, 2123908u32);
    emu.lw_no_count(17usize, 2usize, 132u32, 2123912u32)?;
    emu.adr_no_count(11usize, 11usize, 17usize, 2123916u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2123920u32);
    emu.adr_no_count(13usize, 13usize, 19usize, 2123924u32);
    emu.adr_no_count(6usize, 13usize, 11usize, 2123928u32);
    emu.adr_no_count(11usize, 11usize, 8usize, 2123932u32);
    emu.sri_no_count(12usize, 11usize, 6u32, 2123936u32);
    emu.sli_no_count(13usize, 11usize, 26u32, 2123940u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2123944u32);
    emu.sri_no_count(13usize, 11usize, 11u32, 2123948u32);
    emu.sli_no_count(17usize, 11usize, 21u32, 2123952u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2123956u32);
    emu.sri_no_count(17usize, 11usize, 25u32, 2123960u32);
    emu.sli_no_count(28usize, 11usize, 7u32, 2123964u32);
    emu.orr_no_count(17usize, 28usize, 17usize, 2123968u32);
    emu.lw_no_count(28usize, 2usize, 480u32, 2123972u32)?;
    emu.adr_no_count(28usize, 28usize, 30usize, 2123976u32);
    emu.xrr_no_count(31usize, 18usize, 30usize, 2123980u32);
    emu.anr_no_count(31usize, 11usize, 31usize, 2123984u32);
    emu.xrr_no_count(30usize, 31usize, 30usize, 2123988u32);
    emu.sri_no_count(31usize, 6usize, 2u32, 2123992u32);
    emu.sli_no_count(8usize, 6usize, 30u32, 2123996u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2124000u32);
    emu.sri_no_count(8usize, 6usize, 13u32, 2124004u32);
    emu.sli_no_count(9usize, 6usize, 19u32, 2124008u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2124012u32);
    emu.sri_no_count(9usize, 6usize, 22u32, 2124016u32);
    emu.sli_no_count(19usize, 6usize, 10u32, 2124020u32);
    emu.orr_no_count(9usize, 19usize, 9usize, 2124024u32);
    emu.xrr_no_count(19usize, 10usize, 14usize, 2124028u32);
    emu.anr_no_count(19usize, 6usize, 19usize, 2124032u32);
    emu.anr_no_count(21usize, 10usize, 14usize, 2124036u32);
    emu.xrr_no_count(19usize, 19usize, 21usize, 2124040u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2124044u32);
    emu.adr_no_count(29usize, 29usize, 30usize, 2124048u32);
    emu.xrr_no_count(13usize, 31usize, 8usize, 2124052u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2124056u32);
    emu.xrr_no_count(13usize, 13usize, 9usize, 2124060u32);
    emu.lw_no_count(17usize, 2usize, 128u32, 2124064u32)?;
    emu.adr_no_count(29usize, 29usize, 17usize, 2124068u32);
    emu.adr_no_count(12usize, 29usize, 12usize, 2124072u32);
    emu.adr_no_count(13usize, 13usize, 19usize, 2124076u32);
    emu.adr_no_count(8usize, 13usize, 12usize, 2124080u32);
    emu.adr_no_count(30usize, 12usize, 7usize, 2124084u32);
    emu.sri_no_count(12usize, 30usize, 6u32, 2124088u32);
    emu.sli_no_count(13usize, 30usize, 26u32, 2124092u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2124096u32);
    emu.sri_no_count(13usize, 30usize, 11u32, 2124100u32);
    emu.sli_no_count(17usize, 30usize, 21u32, 2124104u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2124108u32);
    emu.sri_no_count(17usize, 30usize, 25u32, 2124112u32);
    emu.sli_no_count(7usize, 30usize, 7u32, 2124116u32);
    emu.orr_no_count(17usize, 7usize, 17usize, 2124120u32);
    emu.lw_no_count(29usize, 2usize, 388u32, 2124124u32)?;
    emu.adr_no_count(29usize, 29usize, 18usize, 2124128u32);
    emu.xrr_no_count(7usize, 11usize, 18usize, 2124132u32);
    emu.anr_no_count(7usize, 30usize, 7usize, 2124136u32);
    emu.xrr_no_count(7usize, 7usize, 18usize, 2124140u32);
    emu.sri_no_count(31usize, 8usize, 2u32, 2124144u32);
    emu.sli_no_count(9usize, 8usize, 30u32, 2124148u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2124152u32);
    emu.sri_no_count(9usize, 8usize, 13u32, 2124156u32);
    emu.sli_no_count(18usize, 8usize, 19u32, 2124160u32);
    emu.orr_no_count(9usize, 18usize, 9usize, 2124164u32);
    emu.sri_no_count(18usize, 8usize, 22u32, 2124168u32);
    emu.sli_no_count(19usize, 8usize, 10u32, 2124172u32);
    emu.orr_no_count(18usize, 19usize, 18usize, 2124176u32);
    emu.xrr_no_count(19usize, 6usize, 10usize, 2124180u32);
    emu.anr_no_count(19usize, 8usize, 19usize, 2124184u32);
    emu.anr_no_count(21usize, 6usize, 10usize, 2124188u32);
    emu.xrr_no_count(19usize, 19usize, 21usize, 2124192u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2124196u32);
    emu.adr_no_count(7usize, 28usize, 7usize, 2124200u32);
    emu.xrr_no_count(13usize, 31usize, 9usize, 2124204u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2124208u32);
    emu.xrr_no_count(13usize, 13usize, 18usize, 2124212u32);
    emu.lw_no_count(17usize, 2usize, 124u32, 2124216u32)?;
    emu.adr_no_count(7usize, 7usize, 17usize, 2124220u32);
    emu.adr_no_count(12usize, 7usize, 12usize, 2124224u32);
    emu.adr_no_count(13usize, 13usize, 19usize, 2124228u32);
    emu.adr_no_count(7usize, 13usize, 12usize, 2124232u32);
    emu.adr_no_count(18usize, 12usize, 14usize, 2124236u32);
    emu.sri_no_count(12usize, 18usize, 6u32, 2124240u32);
    emu.sli_no_count(13usize, 18usize, 26u32, 2124244u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2124248u32);
    emu.sri_no_count(13usize, 18usize, 11u32, 2124252u32);
    emu.sli_no_count(14usize, 18usize, 21u32, 2124256u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2124260u32);
    emu.sri_no_count(14usize, 18usize, 25u32, 2124264u32);
    emu.sli_no_count(17usize, 18usize, 7u32, 2124268u32);
    emu.orr_no_count(14usize, 17usize, 14usize, 2124272u32);
    emu.lw_no_count(17usize, 2usize, 448u32, 2124276u32)?;
    emu.adr_no_count(17usize, 17usize, 11usize, 2124280u32);
    emu.xrr_no_count(28usize, 30usize, 11usize, 2124284u32);
    emu.anr_no_count(28usize, 18usize, 28usize, 2124288u32);
    emu.xrr_no_count(11usize, 28usize, 11usize, 2124292u32);
    emu.sri_no_count(28usize, 7usize, 2u32, 2124296u32);
    emu.sli_no_count(31usize, 7usize, 30u32, 2124300u32);
    emu.orr_no_count(28usize, 31usize, 28usize, 2124304u32);
    emu.sri_no_count(31usize, 7usize, 13u32, 2124308u32);
    emu.sli_no_count(9usize, 7usize, 19u32, 2124312u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2124316u32);
    emu.sri_no_count(9usize, 7usize, 22u32, 2124320u32);
    emu.sli_no_count(19usize, 7usize, 10u32, 2124324u32);
    emu.orr_no_count(9usize, 19usize, 9usize, 2124328u32);
    emu.xrr_no_count(19usize, 8usize, 6usize, 2124332u32);
    emu.anr_no_count(19usize, 7usize, 19usize, 2124336u32);
    emu.anr_no_count(21usize, 8usize, 6usize, 2124340u32);
    emu.xrr_no_count(19usize, 19usize, 21usize, 2124344u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2124348u32);
    emu.adr_no_count(11usize, 29usize, 11usize, 2124352u32);
    emu.xrr_no_count(13usize, 28usize, 31usize, 2124356u32);
    emu.xrr_no_count(12usize, 12usize, 14usize, 2124360u32);
    emu.xrr_no_count(13usize, 13usize, 9usize, 2124364u32);
    emu.lw_no_count(14usize, 2usize, 120u32, 2124368u32)?;
    emu.adr_no_count(11usize, 11usize, 14usize, 2124372u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2124376u32);
    emu.adr_no_count(13usize, 13usize, 19usize, 2124380u32);
    emu.adr_no_count(14usize, 13usize, 11usize, 2124384u32);
    emu.adr_no_count(11usize, 11usize, 10usize, 2124388u32);
    emu.sri_no_count(10usize, 11usize, 6u32, 2124392u32);
    emu.sli_no_count(12usize, 11usize, 26u32, 2124396u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2124400u32);
    emu.sri_no_count(12usize, 11usize, 11u32, 2124404u32);
    emu.sli_no_count(13usize, 11usize, 21u32, 2124408u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2124412u32);
    emu.sri_no_count(13usize, 11usize, 25u32, 2124416u32);
    emu.sli_no_count(28usize, 11usize, 7u32, 2124420u32);
    emu.orr_no_count(13usize, 28usize, 13usize, 2124424u32);
    emu.lw_no_count(28usize, 2usize, 452u32, 2124428u32)?;
    emu.adr_no_count(28usize, 28usize, 30usize, 2124432u32);
    emu.xrr_no_count(29usize, 18usize, 30usize, 2124436u32);
    emu.anr_no_count(29usize, 11usize, 29usize, 2124440u32);
    emu.xrr_no_count(29usize, 29usize, 30usize, 2124444u32);
    emu.sri_no_count(30usize, 14usize, 2u32, 2124448u32);
    emu.sli_no_count(31usize, 14usize, 30u32, 2124452u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2124456u32);
    emu.sri_no_count(31usize, 14usize, 13u32, 2124460u32);
    emu.sli_no_count(9usize, 14usize, 19u32, 2124464u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2124468u32);
    emu.sri_no_count(9usize, 14usize, 22u32, 2124472u32);
    emu.sli_no_count(19usize, 14usize, 10u32, 2124476u32);
    emu.orr_no_count(9usize, 19usize, 9usize, 2124480u32);
    emu.xrr_no_count(19usize, 7usize, 8usize, 2124484u32);
    emu.anr_no_count(19usize, 14usize, 19usize, 2124488u32);
    emu.anr_no_count(21usize, 7usize, 8usize, 2124492u32);
    emu.xrr_no_count(19usize, 19usize, 21usize, 2124496u32);
    emu.xrr_no_count(10usize, 10usize, 12usize, 2124500u32);
    emu.adr_no_count(17usize, 17usize, 29usize, 2124504u32);
    emu.xrr_no_count(12usize, 30usize, 31usize, 2124508u32);
    emu.xrr_no_count(10usize, 10usize, 13usize, 2124512u32);
    emu.xrr_no_count(12usize, 12usize, 9usize, 2124516u32);
    emu.lw_no_count(13usize, 2usize, 116u32, 2124520u32)?;
    emu.adr_no_count(17usize, 17usize, 13usize, 2124524u32);
    emu.adr_no_count(17usize, 17usize, 10usize, 2124528u32);
    emu.adr_no_count(10usize, 12usize, 19usize, 2124532u32);
    emu.adr_no_count(10usize, 10usize, 17usize, 2124536u32);
    emu.adr_no_count(30usize, 17usize, 6usize, 2124540u32);
    emu.sri_no_count(12usize, 30usize, 6u32, 2124544u32);
    emu.sli_no_count(13usize, 30usize, 26u32, 2124548u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2124552u32);
    emu.sri_no_count(13usize, 30usize, 11u32, 2124556u32);
    emu.sli_no_count(17usize, 30usize, 21u32, 2124560u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2124564u32);
    emu.sri_no_count(17usize, 30usize, 25u32, 2124568u32);
    emu.sli_no_count(6usize, 30usize, 7u32, 2124572u32);
    emu.orr_no_count(17usize, 6usize, 17usize, 2124576u32);
    emu.lw_no_count(29usize, 2usize, 376u32, 2124580u32)?;
    emu.adr_no_count(29usize, 29usize, 18usize, 2124584u32);
    emu.xrr_no_count(6usize, 11usize, 18usize, 2124588u32);
    emu.anr_no_count(6usize, 30usize, 6usize, 2124592u32);
    emu.xrr_no_count(6usize, 6usize, 18usize, 2124596u32);
    emu.sri_no_count(31usize, 10usize, 2u32, 2124600u32);
    emu.sli_no_count(9usize, 10usize, 30u32, 2124604u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2124608u32);
    emu.sri_no_count(9usize, 10usize, 13u32, 2124612u32);
    emu.sli_no_count(18usize, 10usize, 19u32, 2124616u32);
    emu.orr_no_count(9usize, 18usize, 9usize, 2124620u32);
    emu.sri_no_count(18usize, 10usize, 22u32, 2124624u32);
    emu.sli_no_count(19usize, 10usize, 10u32, 2124628u32);
    emu.orr_no_count(18usize, 19usize, 18usize, 2124632u32);
    emu.xrr_no_count(19usize, 14usize, 7usize, 2124636u32);
    emu.anr_no_count(19usize, 10usize, 19usize, 2124640u32);
    emu.anr_no_count(21usize, 14usize, 7usize, 2124644u32);
    emu.xrr_no_count(19usize, 19usize, 21usize, 2124648u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2124652u32);
    emu.adr_no_count(6usize, 28usize, 6usize, 2124656u32);
    emu.xrr_no_count(13usize, 31usize, 9usize, 2124660u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2124664u32);
    emu.xrr_no_count(13usize, 13usize, 18usize, 2124668u32);
    emu.lw_no_count(17usize, 2usize, 112u32, 2124672u32)?;
    emu.adr_no_count(6usize, 6usize, 17usize, 2124676u32);
    emu.adr_no_count(12usize, 6usize, 12usize, 2124680u32);
    emu.adr_no_count(13usize, 13usize, 19usize, 2124684u32);
    emu.adr_no_count(6usize, 13usize, 12usize, 2124688u32);
    emu.adr_no_count(8usize, 12usize, 8usize, 2124692u32);
    emu.sri_no_count(12usize, 8usize, 6u32, 2124696u32);
    emu.sli_no_count(13usize, 8usize, 26u32, 2124700u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2124704u32);
    emu.sri_no_count(13usize, 8usize, 11u32, 2124708u32);
    emu.sli_no_count(17usize, 8usize, 21u32, 2124712u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2124716u32);
    emu.sri_no_count(17usize, 8usize, 25u32, 2124720u32);
    emu.sli_no_count(28usize, 8usize, 7u32, 2124724u32);
    emu.orr_no_count(17usize, 28usize, 17usize, 2124728u32);
    emu.lw_no_count(31usize, 2usize, 384u32, 2124732u32)?;
    emu.adr_no_count(31usize, 31usize, 11usize, 2124736u32);
    emu.xrr_no_count(28usize, 30usize, 11usize, 2124740u32);
    emu.anr_no_count(28usize, 8usize, 28usize, 2124744u32);
    emu.xrr_no_count(11usize, 28usize, 11usize, 2124748u32);
    emu.sri_no_count(28usize, 6usize, 2u32, 2124752u32);
    emu.sli_no_count(9usize, 6usize, 30u32, 2124756u32);
    emu.orr_no_count(28usize, 9usize, 28usize, 2124760u32);
    emu.sri_no_count(9usize, 6usize, 13u32, 2124764u32);
    emu.sli_no_count(18usize, 6usize, 19u32, 2124768u32);
    emu.orr_no_count(9usize, 18usize, 9usize, 2124772u32);
    emu.sri_no_count(18usize, 6usize, 22u32, 2124776u32);
    emu.sli_no_count(19usize, 6usize, 10u32, 2124780u32);
    emu.orr_no_count(18usize, 19usize, 18usize, 2124784u32);
    emu.xrr_no_count(19usize, 10usize, 14usize, 2124788u32);
    emu.anr_no_count(19usize, 6usize, 19usize, 2124792u32);
    emu.anr_no_count(21usize, 10usize, 14usize, 2124796u32);
    emu.xrr_no_count(19usize, 19usize, 21usize, 2124800u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2124804u32);
    emu.adr_no_count(11usize, 29usize, 11usize, 2124808u32);
    emu.xrr_no_count(13usize, 28usize, 9usize, 2124812u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2124816u32);
    emu.xrr_no_count(13usize, 13usize, 18usize, 2124820u32);
    emu.lw_no_count(17usize, 2usize, 108u32, 2124824u32)?;
    emu.adr_no_count(11usize, 11usize, 17usize, 2124828u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2124832u32);
    emu.adr_no_count(13usize, 13usize, 19usize, 2124836u32);
    emu.adr_no_count(28usize, 13usize, 11usize, 2124840u32);
    emu.adr_no_count(11usize, 11usize, 7usize, 2124844u32);
    emu.sri_no_count(12usize, 11usize, 6u32, 2124848u32);
    emu.sli_no_count(13usize, 11usize, 26u32, 2124852u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2124856u32);
    emu.sri_no_count(13usize, 11usize, 11u32, 2124860u32);
    emu.sli_no_count(17usize, 11usize, 21u32, 2124864u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2124868u32);
    emu.sri_no_count(17usize, 11usize, 25u32, 2124872u32);
    emu.sli_no_count(7usize, 11usize, 7u32, 2124876u32);
    emu.orr_no_count(17usize, 7usize, 17usize, 2124880u32);
    emu.lw_no_count(29usize, 2usize, 488u32, 2124884u32)?;
    emu.adr_no_count(29usize, 29usize, 30usize, 2124888u32);
    emu.xrr_no_count(7usize, 8usize, 30usize, 2124892u32);
    emu.anr_no_count(7usize, 11usize, 7usize, 2124896u32);
    emu.xrr_no_count(7usize, 7usize, 30usize, 2124900u32);
    emu.sri_no_count(30usize, 28usize, 2u32, 2124904u32);
    emu.sli_no_count(9usize, 28usize, 30u32, 2124908u32);
    emu.orr_no_count(30usize, 9usize, 30usize, 2124912u32);
    emu.sri_no_count(9usize, 28usize, 13u32, 2124916u32);
    emu.sli_no_count(18usize, 28usize, 19u32, 2124920u32);
    emu.orr_no_count(9usize, 18usize, 9usize, 2124924u32);
    emu.sri_no_count(18usize, 28usize, 22u32, 2124928u32);
    emu.sli_no_count(19usize, 28usize, 10u32, 2124932u32);
    emu.orr_no_count(18usize, 19usize, 18usize, 2124936u32);
    emu.xrr_no_count(19usize, 6usize, 10usize, 2124940u32);
    emu.anr_no_count(19usize, 28usize, 19usize, 2124944u32);
    emu.anr_no_count(21usize, 6usize, 10usize, 2124948u32);
    emu.xrr_no_count(19usize, 19usize, 21usize, 2124952u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2124956u32);
    emu.adr_no_count(7usize, 31usize, 7usize, 2124960u32);
    emu.xrr_no_count(13usize, 30usize, 9usize, 2124964u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2124968u32);
    emu.xrr_no_count(13usize, 13usize, 18usize, 2124972u32);
    emu.lw_no_count(17usize, 2usize, 104u32, 2124976u32)?;
    emu.adr_no_count(7usize, 7usize, 17usize, 2124980u32);
    emu.adr_no_count(12usize, 7usize, 12usize, 2124984u32);
    emu.adr_no_count(13usize, 13usize, 19usize, 2124988u32);
    emu.adr_no_count(7usize, 13usize, 12usize, 2124992u32);
    emu.adr_no_count(14usize, 12usize, 14usize, 2124996u32);
    emu.sri_no_count(12usize, 14usize, 6u32, 2125000u32);
    emu.sli_no_count(13usize, 14usize, 26u32, 2125004u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2125008u32);
    emu.sri_no_count(13usize, 14usize, 11u32, 2125012u32);
    emu.sli_no_count(17usize, 14usize, 21u32, 2125016u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2125020u32);
    emu.sri_no_count(17usize, 14usize, 25u32, 2125024u32);
    emu.sli_no_count(30usize, 14usize, 7u32, 2125028u32);
    emu.orr_no_count(17usize, 30usize, 17usize, 2125032u32);
    emu.lw_no_count(30usize, 2usize, 412u32, 2125036u32)?;
    emu.adr_no_count(30usize, 30usize, 8usize, 2125040u32);
    emu.xrr_no_count(16usize, 11usize, 8usize, 2125044u32);
    emu.anr_no_count(16usize, 14usize, 16usize, 2125048u32);
    emu.xrr_no_count(16usize, 16usize, 8usize, 2125052u32);
    emu.sri_no_count(31usize, 7usize, 2u32, 2125056u32);
    emu.sli_no_count(8usize, 7usize, 30u32, 2125060u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2125064u32);
    emu.sri_no_count(8usize, 7usize, 13u32, 2125068u32);
    emu.sli_no_count(9usize, 7usize, 19u32, 2125072u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2125076u32);
    emu.sri_no_count(9usize, 7usize, 22u32, 2125080u32);
    emu.sli_no_count(18usize, 7usize, 10u32, 2125084u32);
    emu.orr_no_count(9usize, 18usize, 9usize, 2125088u32);
    emu.xrr_no_count(18usize, 28usize, 6usize, 2125092u32);
    emu.anr_no_count(18usize, 7usize, 18usize, 2125096u32);
    emu.anr_no_count(19usize, 28usize, 6usize, 2125100u32);
    emu.xrr_no_count(18usize, 18usize, 19usize, 2125104u32);
    emu.lw_no_count(19usize, 2usize, 428u32, 2125108u32)?;
    emu.xrr_no_count(12usize, 12usize, 13usize, 2125112u32);
    emu.adr_no_count(16usize, 29usize, 16usize, 2125116u32);
    emu.xrr_no_count(13usize, 31usize, 8usize, 2125120u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2125124u32);
    emu.xrr_no_count(13usize, 13usize, 9usize, 2125128u32);
    emu.lw_no_count(17usize, 2usize, 100u32, 2125132u32)?;
    emu.adr_no_count(16usize, 16usize, 17usize, 2125136u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2125140u32);
    emu.adr_no_count(13usize, 13usize, 18usize, 2125144u32);
    emu.adr_no_count(16usize, 13usize, 12usize, 2125148u32);
    emu.adr_no_count(10usize, 12usize, 10usize, 2125152u32);
    emu.sri_no_count(12usize, 10usize, 6u32, 2125156u32);
    emu.sli_no_count(13usize, 10usize, 26u32, 2125160u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2125164u32);
    emu.sri_no_count(13usize, 10usize, 11u32, 2125168u32);
    emu.sli_no_count(17usize, 10usize, 21u32, 2125172u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2125176u32);
    emu.sri_no_count(17usize, 10usize, 25u32, 2125180u32);
    emu.sli_no_count(29usize, 10usize, 7u32, 2125184u32);
    emu.orr_no_count(17usize, 29usize, 17usize, 2125188u32);
    emu.lw_no_count(29usize, 2usize, 380u32, 2125192u32)?;
    emu.adr_no_count(29usize, 29usize, 11usize, 2125196u32);
    emu.xrr_no_count(5usize, 14usize, 11usize, 2125200u32);
    emu.anr_no_count(5usize, 10usize, 5usize, 2125204u32);
    emu.xrr_no_count(11usize, 5usize, 11usize, 2125208u32);
    emu.sri_no_count(5usize, 16usize, 2u32, 2125212u32);
    emu.sli_no_count(31usize, 16usize, 30u32, 2125216u32);
    emu.orr_no_count(5usize, 31usize, 5usize, 2125220u32);
    emu.sri_no_count(31usize, 16usize, 13u32, 2125224u32);
    emu.sli_no_count(8usize, 16usize, 19u32, 2125228u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2125232u32);
    emu.sri_no_count(8usize, 16usize, 22u32, 2125236u32);
    emu.sli_no_count(9usize, 16usize, 10u32, 2125240u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2125244u32);
    emu.xrr_no_count(9usize, 7usize, 28usize, 2125248u32);
    emu.anr_no_count(9usize, 16usize, 9usize, 2125252u32);
    emu.anr_no_count(18usize, 7usize, 28usize, 2125256u32);
    emu.xrr_no_count(9usize, 9usize, 18usize, 2125260u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2125264u32);
    emu.adr_no_count(11usize, 30usize, 11usize, 2125268u32);
    emu.xrr_no_count(13usize, 5usize, 31usize, 2125272u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2125276u32);
    emu.xrr_no_count(13usize, 13usize, 8usize, 2125280u32);
    emu.lw_no_count(17usize, 2usize, 96u32, 2125284u32)?;
    emu.adr_no_count(11usize, 11usize, 17usize, 2125288u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2125292u32);
    emu.adr_no_count(13usize, 13usize, 9usize, 2125296u32);
    emu.adr_no_count(5usize, 13usize, 11usize, 2125300u32);
    emu.adr_no_count(6usize, 11usize, 6usize, 2125304u32);
    emu.sri_no_count(11usize, 6usize, 6u32, 2125308u32);
    emu.sli_no_count(12usize, 6usize, 26u32, 2125312u32);
    emu.orr_no_count(11usize, 12usize, 11usize, 2125316u32);
    emu.sri_no_count(12usize, 6usize, 11u32, 2125320u32);
    emu.sli_no_count(13usize, 6usize, 21u32, 2125324u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2125328u32);
    emu.sri_no_count(13usize, 6usize, 25u32, 2125332u32);
    emu.sli_no_count(17usize, 6usize, 7u32, 2125336u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2125340u32);
    emu.adr_no_count(17usize, 27usize, 14usize, 2125344u32);
    emu.xrr_no_count(15usize, 10usize, 14usize, 2125348u32);
    emu.anr_no_count(15usize, 6usize, 15usize, 2125352u32);
    emu.xrr_no_count(14usize, 15usize, 14usize, 2125356u32);
    emu.sri_no_count(15usize, 5usize, 2u32, 2125360u32);
    emu.sli_no_count(30usize, 5usize, 30u32, 2125364u32);
    emu.orr_no_count(15usize, 30usize, 15usize, 2125368u32);
    emu.sri_no_count(30usize, 5usize, 13u32, 2125372u32);
    emu.sli_no_count(31usize, 5usize, 19u32, 2125376u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2125380u32);
    emu.sri_no_count(31usize, 5usize, 22u32, 2125384u32);
    emu.sli_no_count(8usize, 5usize, 10u32, 2125388u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2125392u32);
    emu.xrr_no_count(8usize, 16usize, 7usize, 2125396u32);
    emu.anr_no_count(8usize, 5usize, 8usize, 2125400u32);
    emu.anr_no_count(9usize, 16usize, 7usize, 2125404u32);
    emu.xrr_no_count(8usize, 8usize, 9usize, 2125408u32);
    emu.xrr_no_count(11usize, 11usize, 12usize, 2125412u32);
    emu.adr_no_count(14usize, 29usize, 14usize, 2125416u32);
    emu.xrr_no_count(12usize, 15usize, 30usize, 2125420u32);
    emu.xrr_no_count(11usize, 11usize, 13usize, 2125424u32);
    emu.xrr_no_count(12usize, 12usize, 31usize, 2125428u32);
    emu.lw_no_count(13usize, 2usize, 92u32, 2125432u32)?;
    emu.adr_no_count(14usize, 14usize, 13usize, 2125436u32);
    emu.adr_no_count(11usize, 14usize, 11usize, 2125440u32);
    emu.adr_no_count(12usize, 12usize, 8usize, 2125444u32);
    emu.adr_no_count(15usize, 12usize, 11usize, 2125448u32);
    emu.adr_no_count(28usize, 11usize, 28usize, 2125452u32);
    emu.sri_no_count(11usize, 28usize, 6u32, 2125456u32);
    emu.sli_no_count(12usize, 28usize, 26u32, 2125460u32);
    emu.orr_no_count(11usize, 12usize, 11usize, 2125464u32);
    emu.sri_no_count(12usize, 28usize, 11u32, 2125468u32);
    emu.sli_no_count(13usize, 28usize, 21u32, 2125472u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2125476u32);
    emu.sri_no_count(13usize, 28usize, 25u32, 2125480u32);
    emu.sli_no_count(14usize, 28usize, 7u32, 2125484u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2125488u32);
    emu.adr_no_count(29usize, 22usize, 10usize, 2125492u32);
    emu.xrr_no_count(14usize, 6usize, 10usize, 2125496u32);
    emu.anr_no_count(14usize, 28usize, 14usize, 2125500u32);
    emu.xrr_no_count(10usize, 14usize, 10usize, 2125504u32);
    emu.sri_no_count(14usize, 15usize, 2u32, 2125508u32);
    emu.sli_no_count(30usize, 15usize, 30u32, 2125512u32);
    emu.orr_no_count(14usize, 30usize, 14usize, 2125516u32);
    emu.sri_no_count(30usize, 15usize, 13u32, 2125520u32);
    emu.sli_no_count(31usize, 15usize, 19u32, 2125524u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2125528u32);
    emu.sri_no_count(31usize, 15usize, 22u32, 2125532u32);
    emu.sli_no_count(8usize, 15usize, 10u32, 2125536u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2125540u32);
    emu.xrr_no_count(8usize, 5usize, 16usize, 2125544u32);
    emu.anr_no_count(8usize, 15usize, 8usize, 2125548u32);
    emu.anr_no_count(9usize, 5usize, 16usize, 2125552u32);
    emu.xrr_no_count(8usize, 8usize, 9usize, 2125556u32);
    emu.xrr_no_count(11usize, 11usize, 12usize, 2125560u32);
    emu.adr_no_count(10usize, 17usize, 10usize, 2125564u32);
    emu.xrr_no_count(12usize, 14usize, 30usize, 2125568u32);
    emu.xrr_no_count(11usize, 11usize, 13usize, 2125572u32);
    emu.xrr_no_count(12usize, 12usize, 31usize, 2125576u32);
    emu.lw_no_count(13usize, 2usize, 88u32, 2125580u32)?;
    emu.adr_no_count(10usize, 10usize, 13usize, 2125584u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2125588u32);
    emu.adr_no_count(12usize, 12usize, 8usize, 2125592u32);
    emu.adr_no_count(14usize, 12usize, 10usize, 2125596u32);
    emu.adr_no_count(7usize, 10usize, 7usize, 2125600u32);
    emu.sri_no_count(10usize, 7usize, 6u32, 2125604u32);
    emu.sli_no_count(11usize, 7usize, 26u32, 2125608u32);
    emu.orr_no_count(10usize, 11usize, 10usize, 2125612u32);
    emu.sri_no_count(11usize, 7usize, 11u32, 2125616u32);
    emu.sli_no_count(12usize, 7usize, 21u32, 2125620u32);
    emu.orr_no_count(11usize, 12usize, 11usize, 2125624u32);
    emu.sri_no_count(12usize, 7usize, 25u32, 2125628u32);
    emu.sli_no_count(13usize, 7usize, 7u32, 2125632u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2125636u32);
    emu.adr_no_count(20usize, 20usize, 6usize, 2125640u32);
    emu.xrr_no_count(13usize, 28usize, 6usize, 2125644u32);
    emu.anr_no_count(13usize, 7usize, 13usize, 2125648u32);
    emu.xrr_no_count(13usize, 13usize, 6usize, 2125652u32);
    emu.sri_no_count(17usize, 14usize, 2u32, 2125656u32);
    emu.sli_no_count(6usize, 14usize, 30u32, 2125660u32);
    emu.orr_no_count(17usize, 6usize, 17usize, 2125664u32);
    emu.sri_no_count(6usize, 14usize, 13u32, 2125668u32);
    emu.sli_no_count(30usize, 14usize, 19u32, 2125672u32);
    emu.orr_no_count(6usize, 30usize, 6usize, 2125676u32);
    emu.sri_no_count(30usize, 14usize, 22u32, 2125680u32);
    emu.sli_no_count(31usize, 14usize, 10u32, 2125684u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2125688u32);
    emu.xrr_no_count(31usize, 15usize, 5usize, 2125692u32);
    emu.anr_no_count(31usize, 14usize, 31usize, 2125696u32);
    emu.anr_no_count(8usize, 15usize, 5usize, 2125700u32);
    emu.xrr_no_count(31usize, 31usize, 8usize, 2125704u32);
    emu.xrr_no_count(10usize, 10usize, 11usize, 2125708u32);
    emu.adr_no_count(13usize, 29usize, 13usize, 2125712u32);
    emu.xrr_no_count(11usize, 17usize, 6usize, 2125716u32);
    emu.xrr_no_count(10usize, 10usize, 12usize, 2125720u32);
    emu.xrr_no_count(11usize, 11usize, 30usize, 2125724u32);
    emu.lw_no_count(12usize, 2usize, 84u32, 2125728u32)?;
    emu.adr_no_count(13usize, 13usize, 12usize, 2125732u32);
    emu.adr_no_count(13usize, 13usize, 10usize, 2125736u32);
    emu.adr_no_count(10usize, 11usize, 31usize, 2125740u32);
    emu.adr_no_count(10usize, 10usize, 13usize, 2125744u32);
    emu.adr_no_count(16usize, 13usize, 16usize, 2125748u32);
    emu.sri_no_count(11usize, 16usize, 6u32, 2125752u32);
    emu.sli_no_count(12usize, 16usize, 26u32, 2125756u32);
    emu.orr_no_count(11usize, 12usize, 11usize, 2125760u32);
    emu.sri_no_count(12usize, 16usize, 11u32, 2125764u32);
    emu.sli_no_count(13usize, 16usize, 21u32, 2125768u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2125772u32);
    emu.sri_no_count(13usize, 16usize, 25u32, 2125776u32);
    emu.sli_no_count(17usize, 16usize, 7u32, 2125780u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2125784u32);
    emu.adr_no_count(22usize, 25usize, 28usize, 2125788u32);
    emu.xrr_no_count(17usize, 7usize, 28usize, 2125792u32);
    emu.anr_no_count(17usize, 16usize, 17usize, 2125796u32);
    emu.xrr_no_count(17usize, 17usize, 28usize, 2125800u32);
    emu.sri_no_count(6usize, 10usize, 2u32, 2125804u32);
    emu.sli_no_count(28usize, 10usize, 30u32, 2125808u32);
    emu.orr_no_count(6usize, 28usize, 6usize, 2125812u32);
    emu.sri_no_count(28usize, 10usize, 13u32, 2125816u32);
    emu.sli_no_count(29usize, 10usize, 19u32, 2125820u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2125824u32);
    emu.sri_no_count(29usize, 10usize, 22u32, 2125828u32);
    emu.sli_no_count(30usize, 10usize, 10u32, 2125832u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2125836u32);
    emu.xrr_no_count(30usize, 14usize, 15usize, 2125840u32);
    emu.anr_no_count(30usize, 10usize, 30usize, 2125844u32);
    emu.anr_no_count(31usize, 14usize, 15usize, 2125848u32);
    emu.xrr_no_count(30usize, 30usize, 31usize, 2125852u32);
    emu.xrr_no_count(11usize, 11usize, 12usize, 2125856u32);
    emu.adr_no_count(17usize, 20usize, 17usize, 2125860u32);
    emu.lw_no_count(21usize, 2usize, 512u32, 2125864u32)?;
    emu.xrr_no_count(12usize, 6usize, 28usize, 2125868u32);
    emu.xrr_no_count(11usize, 11usize, 13usize, 2125872u32);
    emu.xrr_no_count(12usize, 12usize, 29usize, 2125876u32);
    emu.lw_no_count(13usize, 2usize, 80u32, 2125880u32)?;
    emu.adr_no_count(17usize, 17usize, 13usize, 2125884u32);
    emu.adr_no_count(17usize, 17usize, 11usize, 2125888u32);
    emu.adr_no_count(11usize, 12usize, 30usize, 2125892u32);
    emu.adr_no_count(11usize, 11usize, 17usize, 2125896u32);
    emu.adr_no_count(5usize, 17usize, 5usize, 2125900u32);
    emu.sri_no_count(12usize, 5usize, 6u32, 2125904u32);
    emu.sli_no_count(13usize, 5usize, 26u32, 2125908u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2125912u32);
    emu.sri_no_count(13usize, 5usize, 11u32, 2125916u32);
    emu.sli_no_count(17usize, 5usize, 21u32, 2125920u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2125924u32);
    emu.sri_no_count(17usize, 5usize, 25u32, 2125928u32);
    emu.sli_no_count(6usize, 5usize, 7u32, 2125932u32);
    emu.orr_no_count(17usize, 6usize, 17usize, 2125936u32);
    emu.lw_no_count(6usize, 2usize, 340u32, 2125940u32)?;
    emu.adr_no_count(6usize, 6usize, 7usize, 2125944u32);
    emu.xrr_no_count(28usize, 16usize, 7usize, 2125948u32);
    emu.anr_no_count(28usize, 5usize, 28usize, 2125952u32);
    emu.xrr_no_count(7usize, 28usize, 7usize, 2125956u32);
    emu.sri_no_count(28usize, 11usize, 2u32, 2125960u32);
    emu.sli_no_count(29usize, 11usize, 30u32, 2125964u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2125968u32);
    emu.sri_no_count(29usize, 11usize, 13u32, 2125972u32);
    emu.sli_no_count(30usize, 11usize, 19u32, 2125976u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2125980u32);
    emu.sri_no_count(30usize, 11usize, 22u32, 2125984u32);
    emu.sli_no_count(31usize, 11usize, 10u32, 2125988u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2125992u32);
    emu.xrr_no_count(31usize, 10usize, 14usize, 2125996u32);
    emu.anr_no_count(31usize, 11usize, 31usize, 2126000u32);
    emu.anr_no_count(8usize, 10usize, 14usize, 2126004u32);
    emu.xrr_no_count(31usize, 31usize, 8usize, 2126008u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2126012u32);
    emu.adr_no_count(7usize, 22usize, 7usize, 2126016u32);
    emu.lw_no_count(22usize, 2usize, 520u32, 2126020u32)?;
    emu.xrr_no_count(13usize, 28usize, 29usize, 2126024u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2126028u32);
    emu.xrr_no_count(13usize, 13usize, 30usize, 2126032u32);
    emu.lw_no_count(17usize, 2usize, 76u32, 2126036u32)?;
    emu.adr_no_count(7usize, 7usize, 17usize, 2126040u32);
    emu.adr_no_count(7usize, 7usize, 12usize, 2126044u32);
    emu.adr_no_count(12usize, 13usize, 31usize, 2126048u32);
    emu.adr_no_count(12usize, 12usize, 7usize, 2126052u32);
    emu.adr_no_count(15usize, 7usize, 15usize, 2126056u32);
    emu.sri_no_count(13usize, 15usize, 6u32, 2126060u32);
    emu.sli_no_count(17usize, 15usize, 26u32, 2126064u32);
    emu.sri_no_count(7usize, 15usize, 11u32, 2126068u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2126072u32);
    emu.sli_no_count(17usize, 15usize, 21u32, 2126076u32);
    emu.orr_no_count(17usize, 17usize, 7usize, 2126080u32);
    emu.sri_no_count(7usize, 15usize, 25u32, 2126084u32);
    emu.sli_no_count(28usize, 15usize, 7u32, 2126088u32);
    emu.orr_no_count(7usize, 28usize, 7usize, 2126092u32);
    emu.lw_no_count(28usize, 2usize, 420u32, 2126096u32)?;
    emu.adr_no_count(28usize, 28usize, 16usize, 2126100u32);
    emu.xrr_no_count(29usize, 5usize, 16usize, 2126104u32);
    emu.anr_no_count(29usize, 15usize, 29usize, 2126108u32);
    emu.xrr_no_count(16usize, 29usize, 16usize, 2126112u32);
    emu.sri_no_count(29usize, 12usize, 2u32, 2126116u32);
    emu.sli_no_count(30usize, 12usize, 30u32, 2126120u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2126124u32);
    emu.sri_no_count(30usize, 12usize, 13u32, 2126128u32);
    emu.sli_no_count(31usize, 12usize, 19u32, 2126132u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2126136u32);
    emu.sri_no_count(31usize, 12usize, 22u32, 2126140u32);
    emu.sli_no_count(8usize, 12usize, 10u32, 2126144u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2126148u32);
    emu.xrr_no_count(8usize, 11usize, 10usize, 2126152u32);
    emu.anr_no_count(8usize, 12usize, 8usize, 2126156u32);
    emu.anr_no_count(9usize, 11usize, 10usize, 2126160u32);
    emu.xrr_no_count(8usize, 8usize, 9usize, 2126164u32);
    emu.xrr_no_count(13usize, 13usize, 17usize, 2126168u32);
    emu.adr_no_count(16usize, 6usize, 16usize, 2126172u32);
    emu.xrr_no_count(17usize, 29usize, 30usize, 2126176u32);
    emu.xrr_no_count(13usize, 13usize, 7usize, 2126180u32);
    emu.lw_no_count(25usize, 2usize, 344u32, 2126184u32)?;
    emu.adr_no_count(25usize, 25usize, 5usize, 2126188u32);
    emu.lw_no_count(6usize, 2usize, 72u32, 2126192u32)?;
    emu.adr_no_count(16usize, 16usize, 6usize, 2126196u32);
    emu.xrr_no_count(17usize, 17usize, 31usize, 2126200u32);
    emu.adr_no_count(16usize, 16usize, 13usize, 2126204u32);
    emu.adr_no_count(13usize, 17usize, 8usize, 2126208u32);
    emu.adr_no_count(13usize, 13usize, 16usize, 2126212u32);
    emu.adr_no_count(16usize, 16usize, 14usize, 2126216u32);
    emu.sri_no_count(14usize, 16usize, 6u32, 2126220u32);
    emu.sli_no_count(17usize, 16usize, 26u32, 2126224u32);
    emu.sri_no_count(6usize, 16usize, 11u32, 2126228u32);
    emu.sli_no_count(7usize, 16usize, 21u32, 2126232u32);
    emu.orr_no_count(14usize, 17usize, 14usize, 2126236u32);
    emu.sri_no_count(17usize, 16usize, 25u32, 2126240u32);
    emu.orr_no_count(6usize, 7usize, 6usize, 2126244u32);
    emu.sli_no_count(7usize, 16usize, 7u32, 2126248u32);
    emu.orr_no_count(17usize, 7usize, 17usize, 2126252u32);
    emu.xrr_no_count(7usize, 15usize, 5usize, 2126256u32);
    emu.anr_no_count(7usize, 16usize, 7usize, 2126260u32);
    emu.xrr_no_count(5usize, 7usize, 5usize, 2126264u32);
    emu.sri_no_count(7usize, 13usize, 2u32, 2126268u32);
    emu.sli_no_count(29usize, 13usize, 30u32, 2126272u32);
    emu.orr_no_count(7usize, 29usize, 7usize, 2126276u32);
    emu.sri_no_count(29usize, 13usize, 13u32, 2126280u32);
    emu.sli_no_count(30usize, 13usize, 19u32, 2126284u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2126288u32);
    emu.sri_no_count(30usize, 13usize, 22u32, 2126292u32);
    emu.sli_no_count(31usize, 13usize, 10u32, 2126296u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2126300u32);
    emu.xrr_no_count(31usize, 12usize, 11usize, 2126304u32);
    emu.anr_no_count(31usize, 13usize, 31usize, 2126308u32);
    emu.anr_no_count(8usize, 12usize, 11usize, 2126312u32);
    emu.xrr_no_count(31usize, 31usize, 8usize, 2126316u32);
    emu.xrr_no_count(14usize, 14usize, 6usize, 2126320u32);
    emu.adr_no_count(5usize, 28usize, 5usize, 2126324u32);
    emu.xrr_no_count(6usize, 7usize, 29usize, 2126328u32);
    emu.lw_no_count(26usize, 2usize, 348u32, 2126332u32)?;
    emu.adr_no_count(26usize, 26usize, 15usize, 2126336u32);
    emu.xrr_no_count(14usize, 14usize, 17usize, 2126340u32);
    emu.xrr_no_count(7usize, 16usize, 15usize, 2126344u32);
    emu.lw_no_count(17usize, 2usize, 68u32, 2126348u32)?;
    emu.adr_no_count(5usize, 5usize, 17usize, 2126352u32);
    emu.xrr_no_count(17usize, 6usize, 30usize, 2126356u32);
    emu.adr_no_count(5usize, 5usize, 14usize, 2126360u32);
    emu.adr_no_count(14usize, 17usize, 31usize, 2126364u32);
    emu.adr_no_count(14usize, 14usize, 5usize, 2126368u32);
    emu.adr_no_count(17usize, 5usize, 10usize, 2126372u32);
    emu.sri_no_count(10usize, 17usize, 6u32, 2126376u32);
    emu.sli_no_count(5usize, 17usize, 26u32, 2126380u32);
    emu.sri_no_count(6usize, 17usize, 11u32, 2126384u32);
    emu.sli_no_count(28usize, 17usize, 21u32, 2126388u32);
    emu.orr_no_count(10usize, 5usize, 10usize, 2126392u32);
    emu.sri_no_count(5usize, 17usize, 25u32, 2126396u32);
    emu.orr_no_count(6usize, 28usize, 6usize, 2126400u32);
    emu.sli_no_count(28usize, 17usize, 7u32, 2126404u32);
    emu.anr_no_count(7usize, 17usize, 7usize, 2126408u32);
    emu.orr_no_count(5usize, 28usize, 5usize, 2126412u32);
    emu.sri_no_count(28usize, 14usize, 2u32, 2126416u32);
    emu.xrr_no_count(15usize, 7usize, 15usize, 2126420u32);
    emu.sli_no_count(7usize, 14usize, 30u32, 2126424u32);
    emu.orr_no_count(7usize, 7usize, 28usize, 2126428u32);
    emu.sri_no_count(28usize, 14usize, 13u32, 2126432u32);
    emu.sli_no_count(29usize, 14usize, 19u32, 2126436u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2126440u32);
    emu.sri_no_count(29usize, 14usize, 22u32, 2126444u32);
    emu.sli_no_count(30usize, 14usize, 10u32, 2126448u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2126452u32);
    emu.xrr_no_count(30usize, 13usize, 12usize, 2126456u32);
    emu.anr_no_count(30usize, 14usize, 30usize, 2126460u32);
    emu.anr_no_count(31usize, 13usize, 12usize, 2126464u32);
    emu.xrr_no_count(30usize, 30usize, 31usize, 2126468u32);
    emu.xrr_no_count(10usize, 10usize, 6usize, 2126472u32);
    emu.adr_no_count(15usize, 25usize, 15usize, 2126476u32);
    emu.lw_no_count(27usize, 2usize, 336u32, 2126480u32)?;
    emu.adr_no_count(27usize, 27usize, 16usize, 2126484u32);
    emu.xrr_no_count(6usize, 7usize, 28usize, 2126488u32);
    emu.xrr_no_count(7usize, 17usize, 16usize, 2126492u32);
    emu.xrr_no_count(10usize, 10usize, 5usize, 2126496u32);
    emu.lw_no_count(5usize, 2usize, 64u32, 2126500u32)?;
    emu.adr_no_count(15usize, 15usize, 5usize, 2126504u32);
    emu.xrr_no_count(5usize, 6usize, 29usize, 2126508u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2126512u32);
    emu.adr_no_count(10usize, 5usize, 30usize, 2126516u32);
    emu.adr_no_count(10usize, 10usize, 15usize, 2126520u32);
    emu.adr_no_count(15usize, 15usize, 11usize, 2126524u32);
    emu.sri_no_count(11usize, 15usize, 6u32, 2126528u32);
    emu.sli_no_count(5usize, 15usize, 26u32, 2126532u32);
    emu.sri_no_count(6usize, 15usize, 11u32, 2126536u32);
    emu.sli_no_count(28usize, 15usize, 21u32, 2126540u32);
    emu.sri_no_count(29usize, 15usize, 25u32, 2126544u32);
    emu.orr_no_count(11usize, 5usize, 11usize, 2126548u32);
    emu.sli_no_count(5usize, 15usize, 7u32, 2126552u32);
    emu.anr_no_count(7usize, 15usize, 7usize, 2126556u32);
    emu.orr_no_count(6usize, 28usize, 6usize, 2126560u32);
    emu.sri_no_count(28usize, 10usize, 2u32, 2126564u32);
    emu.orr_no_count(29usize, 5usize, 29usize, 2126568u32);
    emu.sli_no_count(5usize, 10usize, 30u32, 2126572u32);
    emu.xrr_no_count(16usize, 7usize, 16usize, 2126576u32);
    emu.sri_no_count(7usize, 10usize, 13u32, 2126580u32);
    emu.orr_no_count(28usize, 5usize, 28usize, 2126584u32);
    emu.sli_no_count(5usize, 10usize, 19u32, 2126588u32);
    emu.orr_no_count(7usize, 5usize, 7usize, 2126592u32);
    emu.sri_no_count(5usize, 10usize, 22u32, 2126596u32);
    emu.sli_no_count(30usize, 10usize, 10u32, 2126600u32);
    emu.orr_no_count(30usize, 30usize, 5usize, 2126604u32);
    emu.xrr_no_count(5usize, 14usize, 13usize, 2126608u32);
    emu.anr_no_count(5usize, 10usize, 5usize, 2126612u32);
    emu.anr_no_count(31usize, 14usize, 13usize, 2126616u32);
    emu.xrr_no_count(31usize, 5usize, 31usize, 2126620u32);
    emu.xrr_no_count(11usize, 11usize, 6usize, 2126624u32);
    emu.lw_no_count(5usize, 2usize, 352u32, 2126628u32)?;
    emu.adr_no_count(5usize, 5usize, 17usize, 2126632u32);
    emu.adr_no_count(16usize, 26usize, 16usize, 2126636u32);
    emu.lw_no_count(26usize, 2usize, 432u32, 2126640u32)?;
    emu.xrr_no_count(6usize, 15usize, 17usize, 2126644u32);
    emu.xrr_no_count(7usize, 28usize, 7usize, 2126648u32);
    emu.xrr_no_count(11usize, 11usize, 29usize, 2126652u32);
    emu.lw_no_count(28usize, 2usize, 60u32, 2126656u32)?;
    emu.adr_no_count(16usize, 16usize, 28usize, 2126660u32);
    emu.xrr_no_count(7usize, 7usize, 30usize, 2126664u32);
    emu.adr_no_count(16usize, 16usize, 11usize, 2126668u32);
    emu.adr_no_count(11usize, 7usize, 31usize, 2126672u32);
    emu.adr_no_count(11usize, 11usize, 16usize, 2126676u32);
    emu.adr_no_count(16usize, 16usize, 12usize, 2126680u32);
    emu.sri_no_count(12usize, 16usize, 6u32, 2126684u32);
    emu.sli_no_count(7usize, 16usize, 26u32, 2126688u32);
    emu.sri_no_count(28usize, 16usize, 11u32, 2126692u32);
    emu.sli_no_count(29usize, 16usize, 21u32, 2126696u32);
    emu.sri_no_count(30usize, 16usize, 25u32, 2126700u32);
    emu.sli_no_count(31usize, 16usize, 7u32, 2126704u32);
    emu.anr_no_count(6usize, 16usize, 6usize, 2126708u32);
    emu.orr_no_count(12usize, 7usize, 12usize, 2126712u32);
    emu.sri_no_count(7usize, 11usize, 2u32, 2126716u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2126720u32);
    emu.sli_no_count(29usize, 11usize, 30u32, 2126724u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2126728u32);
    emu.sri_no_count(31usize, 11usize, 13u32, 2126732u32);
    emu.xrr_no_count(17usize, 6usize, 17usize, 2126736u32);
    emu.sli_no_count(6usize, 11usize, 19u32, 2126740u32);
    emu.orr_no_count(7usize, 29usize, 7usize, 2126744u32);
    emu.sri_no_count(29usize, 11usize, 22u32, 2126748u32);
    emu.orr_no_count(6usize, 6usize, 31usize, 2126752u32);
    emu.sli_no_count(31usize, 11usize, 10u32, 2126756u32);
    emu.orr_no_count(29usize, 31usize, 29usize, 2126760u32);
    emu.xrr_no_count(31usize, 10usize, 14usize, 2126764u32);
    emu.anr_no_count(31usize, 11usize, 31usize, 2126768u32);
    emu.anr_no_count(8usize, 10usize, 14usize, 2126772u32);
    emu.xrr_no_count(31usize, 31usize, 8usize, 2126776u32);
    emu.lw_no_count(1usize, 2usize, 356u32, 2126780u32)?;
    emu.adr_no_count(1usize, 1usize, 15usize, 2126784u32);
    emu.xrr_no_count(12usize, 12usize, 28usize, 2126788u32);
    emu.anr_no_count(28usize, 11usize, 10usize, 2126792u32);
    emu.adr_no_count(17usize, 27usize, 17usize, 2126796u32);
    emu.xrr_no_count(8usize, 16usize, 15usize, 2126800u32);
    emu.xrr_no_count(6usize, 7usize, 6usize, 2126804u32);
    emu.xrr_no_count(12usize, 12usize, 30usize, 2126808u32);
    emu.lw_no_count(7usize, 2usize, 56u32, 2126812u32)?;
    emu.adr_no_count(17usize, 17usize, 7usize, 2126816u32);
    emu.xrr_no_count(6usize, 6usize, 29usize, 2126820u32);
    emu.adr_no_count(17usize, 17usize, 12usize, 2126824u32);
    emu.adr_no_count(12usize, 6usize, 31usize, 2126828u32);
    emu.adr_no_count(12usize, 12usize, 17usize, 2126832u32);
    emu.adr_no_count(17usize, 17usize, 13usize, 2126836u32);
    emu.sri_no_count(13usize, 17usize, 6u32, 2126840u32);
    emu.sli_no_count(6usize, 17usize, 26u32, 2126844u32);
    emu.sri_no_count(7usize, 17usize, 11u32, 2126848u32);
    emu.sli_no_count(29usize, 17usize, 21u32, 2126852u32);
    emu.sri_no_count(30usize, 17usize, 25u32, 2126856u32);
    emu.sli_no_count(31usize, 17usize, 7u32, 2126860u32);
    emu.anr_no_count(8usize, 17usize, 8usize, 2126864u32);
    emu.orr_no_count(13usize, 6usize, 13usize, 2126868u32);
    emu.sri_no_count(6usize, 12usize, 2u32, 2126872u32);
    emu.orr_no_count(7usize, 29usize, 7usize, 2126876u32);
    emu.sli_no_count(29usize, 12usize, 30u32, 2126880u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2126884u32);
    emu.sri_no_count(31usize, 12usize, 13u32, 2126888u32);
    emu.xrr_no_count(8usize, 8usize, 15usize, 2126892u32);
    emu.sli_no_count(15usize, 12usize, 19u32, 2126896u32);
    emu.orr_no_count(6usize, 29usize, 6usize, 2126900u32);
    emu.sri_no_count(29usize, 12usize, 22u32, 2126904u32);
    emu.orr_no_count(31usize, 15usize, 31usize, 2126908u32);
    emu.sli_no_count(15usize, 12usize, 10u32, 2126912u32);
    emu.orr_no_count(29usize, 15usize, 29usize, 2126916u32);
    emu.xrr_no_count(15usize, 11usize, 10usize, 2126920u32);
    emu.anr_no_count(15usize, 12usize, 15usize, 2126924u32);
    emu.xrr_no_count(28usize, 15usize, 28usize, 2126928u32);
    emu.lw_no_count(15usize, 2usize, 496u32, 2126932u32)?;
    emu.adr_no_count(15usize, 15usize, 16usize, 2126936u32);
    emu.xrr_no_count(13usize, 13usize, 7usize, 2126940u32);
    emu.xrr_no_count(7usize, 17usize, 16usize, 2126944u32);
    emu.adr_no_count(5usize, 5usize, 8usize, 2126948u32);
    emu.xrr_no_count(8usize, 12usize, 11usize, 2126952u32);
    emu.xrr_no_count(6usize, 6usize, 31usize, 2126956u32);
    emu.xrr_no_count(13usize, 13usize, 30usize, 2126960u32);
    emu.lw_no_count(30usize, 2usize, 52u32, 2126964u32)?;
    emu.adr_no_count(5usize, 5usize, 30usize, 2126968u32);
    emu.xrr_no_count(6usize, 6usize, 29usize, 2126972u32);
    emu.adr_no_count(5usize, 5usize, 13usize, 2126976u32);
    emu.adr_no_count(13usize, 6usize, 28usize, 2126980u32);
    emu.adr_no_count(13usize, 13usize, 5usize, 2126984u32);
    emu.adr_no_count(5usize, 5usize, 14usize, 2126988u32);
    emu.sri_no_count(14usize, 5usize, 6u32, 2126992u32);
    emu.sli_no_count(6usize, 5usize, 26u32, 2126996u32);
    emu.sri_no_count(28usize, 5usize, 11u32, 2127000u32);
    emu.sli_no_count(29usize, 5usize, 21u32, 2127004u32);
    emu.sri_no_count(30usize, 5usize, 25u32, 2127008u32);
    emu.sli_no_count(31usize, 5usize, 7u32, 2127012u32);
    emu.anr_no_count(7usize, 5usize, 7usize, 2127016u32);
    emu.orr_no_count(14usize, 6usize, 14usize, 2127020u32);
    emu.sri_no_count(6usize, 13usize, 2u32, 2127024u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2127028u32);
    emu.sli_no_count(29usize, 13usize, 30u32, 2127032u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2127036u32);
    emu.sri_no_count(31usize, 13usize, 13u32, 2127040u32);
    emu.xrr_no_count(7usize, 7usize, 16usize, 2127044u32);
    emu.sli_no_count(16usize, 13usize, 19u32, 2127048u32);
    emu.orr_no_count(6usize, 29usize, 6usize, 2127052u32);
    emu.sri_no_count(29usize, 13usize, 22u32, 2127056u32);
    emu.orr_no_count(31usize, 16usize, 31usize, 2127060u32);
    emu.sli_no_count(16usize, 13usize, 10u32, 2127064u32);
    emu.orr_no_count(29usize, 16usize, 29usize, 2127068u32);
    emu.anr_no_count(16usize, 12usize, 11usize, 2127072u32);
    emu.anr_no_count(8usize, 13usize, 8usize, 2127076u32);
    emu.xrr_no_count(8usize, 8usize, 16usize, 2127080u32);
    emu.lw_no_count(16usize, 2usize, 332u32, 2127084u32)?;
    emu.adr_no_count(16usize, 16usize, 17usize, 2127088u32);
    emu.xrr_no_count(14usize, 14usize, 28usize, 2127092u32);
    emu.anr_no_count(28usize, 13usize, 12usize, 2127096u32);
    emu.adr_no_count(7usize, 1usize, 7usize, 2127100u32);
    emu.lw_no_count(20usize, 2usize, 516u32, 2127104u32)?;
    emu.xrr_no_count(9usize, 5usize, 17usize, 2127108u32);
    emu.xrr_no_count(6usize, 6usize, 31usize, 2127112u32);
    emu.xrr_no_count(14usize, 14usize, 30usize, 2127116u32);
    emu.lw_no_count(30usize, 2usize, 48u32, 2127120u32)?;
    emu.adr_no_count(7usize, 7usize, 30usize, 2127124u32);
    emu.xrr_no_count(6usize, 6usize, 29usize, 2127128u32);
    emu.adr_no_count(7usize, 7usize, 14usize, 2127132u32);
    emu.adr_no_count(14usize, 6usize, 8usize, 2127136u32);
    emu.adr_no_count(14usize, 14usize, 7usize, 2127140u32);
    emu.adr_no_count(10usize, 7usize, 10usize, 2127144u32);
    emu.sri_no_count(6usize, 10usize, 6u32, 2127148u32);
    emu.sli_no_count(7usize, 10usize, 26u32, 2127152u32);
    emu.sri_no_count(29usize, 10usize, 11u32, 2127156u32);
    emu.sli_no_count(30usize, 10usize, 21u32, 2127160u32);
    emu.sri_no_count(31usize, 10usize, 25u32, 2127164u32);
    emu.sli_no_count(8usize, 10usize, 7u32, 2127168u32);
    emu.anr_no_count(9usize, 10usize, 9usize, 2127172u32);
    emu.orr_no_count(6usize, 7usize, 6usize, 2127176u32);
    emu.sri_no_count(7usize, 14usize, 2u32, 2127180u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2127184u32);
    emu.sli_no_count(30usize, 14usize, 30u32, 2127188u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2127192u32);
    emu.sri_no_count(8usize, 14usize, 13u32, 2127196u32);
    emu.xrr_no_count(17usize, 9usize, 17usize, 2127200u32);
    emu.sli_no_count(9usize, 14usize, 19u32, 2127204u32);
    emu.orr_no_count(30usize, 30usize, 7usize, 2127208u32);
    emu.sri_no_count(7usize, 14usize, 22u32, 2127212u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2127216u32);
    emu.sli_no_count(9usize, 14usize, 10u32, 2127220u32);
    emu.orr_no_count(9usize, 9usize, 7usize, 2127224u32);
    emu.xrr_no_count(7usize, 13usize, 12usize, 2127228u32);
    emu.anr_no_count(7usize, 14usize, 7usize, 2127232u32);
    emu.xrr_no_count(28usize, 7usize, 28usize, 2127236u32);
    emu.lw_no_count(7usize, 2usize, 440u32, 2127240u32)?;
    emu.adr_no_count(7usize, 7usize, 5usize, 2127244u32);
    emu.xrr_no_count(6usize, 6usize, 29usize, 2127248u32);
    emu.xrr_no_count(29usize, 10usize, 5usize, 2127252u32);
    emu.adr_no_count(15usize, 15usize, 17usize, 2127256u32);
    emu.xrr_no_count(18usize, 14usize, 13usize, 2127260u32);
    emu.xrr_no_count(17usize, 30usize, 8usize, 2127264u32);
    emu.xrr_no_count(6usize, 6usize, 31usize, 2127268u32);
    emu.lw_no_count(30usize, 2usize, 44u32, 2127272u32)?;
    emu.adr_no_count(15usize, 15usize, 30usize, 2127276u32);
    emu.xrr_no_count(17usize, 17usize, 9usize, 2127280u32);
    emu.adr_no_count(6usize, 15usize, 6usize, 2127284u32);
    emu.adr_no_count(15usize, 17usize, 28usize, 2127288u32);
    emu.adr_no_count(15usize, 15usize, 6usize, 2127292u32);
    emu.adr_no_count(17usize, 6usize, 11usize, 2127296u32);
    emu.sri_no_count(11usize, 17usize, 6u32, 2127300u32);
    emu.sli_no_count(6usize, 17usize, 26u32, 2127304u32);
    emu.sri_no_count(28usize, 17usize, 11u32, 2127308u32);
    emu.sli_no_count(30usize, 17usize, 21u32, 2127312u32);
    emu.sri_no_count(31usize, 17usize, 25u32, 2127316u32);
    emu.sli_no_count(8usize, 17usize, 7u32, 2127320u32);
    emu.anr_no_count(29usize, 17usize, 29usize, 2127324u32);
    emu.orr_no_count(11usize, 6usize, 11usize, 2127328u32);
    emu.sri_no_count(6usize, 15usize, 2u32, 2127332u32);
    emu.orr_no_count(28usize, 30usize, 28usize, 2127336u32);
    emu.sli_no_count(30usize, 15usize, 30u32, 2127340u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2127344u32);
    emu.sri_no_count(8usize, 15usize, 13u32, 2127348u32);
    emu.xrr_no_count(29usize, 29usize, 5usize, 2127352u32);
    emu.sli_no_count(5usize, 15usize, 19u32, 2127356u32);
    emu.orr_no_count(6usize, 30usize, 6usize, 2127360u32);
    emu.sri_no_count(30usize, 15usize, 22u32, 2127364u32);
    emu.orr_no_count(8usize, 5usize, 8usize, 2127368u32);
    emu.sli_no_count(5usize, 15usize, 10u32, 2127372u32);
    emu.orr_no_count(30usize, 5usize, 30usize, 2127376u32);
    emu.anr_no_count(5usize, 14usize, 13usize, 2127380u32);
    emu.anr_no_count(9usize, 15usize, 18usize, 2127384u32);
    emu.xrr_no_count(9usize, 9usize, 5usize, 2127388u32);
    emu.lw_no_count(5usize, 2usize, 364u32, 2127392u32)?;
    emu.adr_no_count(5usize, 5usize, 10usize, 2127396u32);
    emu.xrr_no_count(11usize, 11usize, 28usize, 2127400u32);
    emu.anr_no_count(28usize, 15usize, 14usize, 2127404u32);
    emu.adr_no_count(16usize, 16usize, 29usize, 2127408u32);
    emu.xrr_no_count(29usize, 17usize, 10usize, 2127412u32);
    emu.xrr_no_count(6usize, 6usize, 8usize, 2127416u32);
    emu.xrr_no_count(11usize, 11usize, 31usize, 2127420u32);
    emu.lw_no_count(31usize, 2usize, 40u32, 2127424u32)?;
    emu.adr_no_count(16usize, 16usize, 31usize, 2127428u32);
    emu.xrr_no_count(6usize, 6usize, 30usize, 2127432u32);
    emu.adr_no_count(11usize, 16usize, 11usize, 2127436u32);
    emu.adr_no_count(16usize, 6usize, 9usize, 2127440u32);
    emu.adr_no_count(16usize, 16usize, 11usize, 2127444u32);
    emu.adr_no_count(12usize, 11usize, 12usize, 2127448u32);
    emu.sri_no_count(11usize, 12usize, 6u32, 2127452u32);
    emu.sli_no_count(6usize, 12usize, 26u32, 2127456u32);
    emu.sri_no_count(30usize, 12usize, 11u32, 2127460u32);
    emu.sli_no_count(31usize, 12usize, 21u32, 2127464u32);
    emu.sri_no_count(8usize, 12usize, 25u32, 2127468u32);
    emu.sli_no_count(9usize, 12usize, 7u32, 2127472u32);
    emu.anr_no_count(29usize, 12usize, 29usize, 2127476u32);
    emu.orr_no_count(11usize, 6usize, 11usize, 2127480u32);
    emu.sri_no_count(6usize, 16usize, 2u32, 2127484u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2127488u32);
    emu.sli_no_count(31usize, 16usize, 30u32, 2127492u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2127496u32);
    emu.sri_no_count(9usize, 16usize, 13u32, 2127500u32);
    emu.xrr_no_count(10usize, 29usize, 10usize, 2127504u32);
    emu.sli_no_count(29usize, 16usize, 19u32, 2127508u32);
    emu.orr_no_count(31usize, 31usize, 6usize, 2127512u32);
    emu.sri_no_count(6usize, 16usize, 22u32, 2127516u32);
    emu.orr_no_count(29usize, 29usize, 9usize, 2127520u32);
    emu.sli_no_count(9usize, 16usize, 10u32, 2127524u32);
    emu.orr_no_count(9usize, 9usize, 6usize, 2127528u32);
    emu.xrr_no_count(6usize, 15usize, 14usize, 2127532u32);
    emu.anr_no_count(6usize, 16usize, 6usize, 2127536u32);
    emu.xrr_no_count(28usize, 6usize, 28usize, 2127540u32);
    emu.lw_no_count(6usize, 2usize, 492u32, 2127544u32)?;
    emu.adr_no_count(6usize, 6usize, 17usize, 2127548u32);
    emu.xrr_no_count(11usize, 11usize, 30usize, 2127552u32);
    emu.xrr_no_count(30usize, 12usize, 17usize, 2127556u32);
    emu.adr_no_count(10usize, 7usize, 10usize, 2127560u32);
    emu.xrr_no_count(7usize, 16usize, 15usize, 2127564u32);
    emu.xrr_no_count(29usize, 31usize, 29usize, 2127568u32);
    emu.xrr_no_count(11usize, 11usize, 8usize, 2127572u32);
    emu.lw_no_count(31usize, 2usize, 36u32, 2127576u32)?;
    emu.adr_no_count(10usize, 10usize, 31usize, 2127580u32);
    emu.xrr_no_count(29usize, 29usize, 9usize, 2127584u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2127588u32);
    emu.adr_no_count(11usize, 29usize, 28usize, 2127592u32);
    emu.adr_no_count(11usize, 11usize, 10usize, 2127596u32);
    emu.adr_no_count(13usize, 10usize, 13usize, 2127600u32);
    emu.sri_no_count(10usize, 13usize, 6u32, 2127604u32);
    emu.sli_no_count(28usize, 13usize, 26u32, 2127608u32);
    emu.sri_no_count(29usize, 13usize, 11u32, 2127612u32);
    emu.sli_no_count(31usize, 13usize, 21u32, 2127616u32);
    emu.sri_no_count(8usize, 13usize, 25u32, 2127620u32);
    emu.sli_no_count(9usize, 13usize, 7u32, 2127624u32);
    emu.anr_no_count(30usize, 13usize, 30usize, 2127628u32);
    emu.orr_no_count(10usize, 28usize, 10usize, 2127632u32);
    emu.sri_no_count(28usize, 11usize, 2u32, 2127636u32);
    emu.orr_no_count(29usize, 31usize, 29usize, 2127640u32);
    emu.sli_no_count(31usize, 11usize, 30u32, 2127644u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2127648u32);
    emu.sri_no_count(9usize, 11usize, 13u32, 2127652u32);
    emu.xrr_no_count(17usize, 30usize, 17usize, 2127656u32);
    emu.sli_no_count(30usize, 11usize, 19u32, 2127660u32);
    emu.orr_no_count(28usize, 31usize, 28usize, 2127664u32);
    emu.sri_no_count(31usize, 11usize, 22u32, 2127668u32);
    emu.orr_no_count(30usize, 30usize, 9usize, 2127672u32);
    emu.sli_no_count(9usize, 11usize, 10u32, 2127676u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2127680u32);
    emu.anr_no_count(9usize, 16usize, 15usize, 2127684u32);
    emu.anr_no_count(7usize, 11usize, 7usize, 2127688u32);
    emu.xrr_no_count(9usize, 7usize, 9usize, 2127692u32);
    emu.lw_no_count(7usize, 2usize, 408u32, 2127696u32)?;
    emu.adr_no_count(7usize, 7usize, 12usize, 2127700u32);
    emu.xrr_no_count(10usize, 10usize, 29usize, 2127704u32);
    emu.anr_no_count(29usize, 11usize, 16usize, 2127708u32);
    emu.adr_no_count(17usize, 5usize, 17usize, 2127712u32);
    emu.xrr_no_count(18usize, 13usize, 12usize, 2127716u32);
    emu.xrr_no_count(5usize, 28usize, 30usize, 2127720u32);
    emu.xrr_no_count(10usize, 10usize, 8usize, 2127724u32);
    emu.lw_no_count(28usize, 2usize, 32u32, 2127728u32)?;
    emu.adr_no_count(17usize, 17usize, 28usize, 2127732u32);
    emu.xrr_no_count(5usize, 5usize, 31usize, 2127736u32);
    emu.adr_no_count(17usize, 17usize, 10usize, 2127740u32);
    emu.adr_no_count(10usize, 5usize, 9usize, 2127744u32);
    emu.adr_no_count(10usize, 10usize, 17usize, 2127748u32);
    emu.adr_no_count(5usize, 17usize, 14usize, 2127752u32);
    emu.sri_no_count(14usize, 5usize, 6u32, 2127756u32);
    emu.sli_no_count(17usize, 5usize, 26u32, 2127760u32);
    emu.sri_no_count(28usize, 5usize, 11u32, 2127764u32);
    emu.sli_no_count(30usize, 5usize, 21u32, 2127768u32);
    emu.sri_no_count(31usize, 5usize, 25u32, 2127772u32);
    emu.sli_no_count(8usize, 5usize, 7u32, 2127776u32);
    emu.anr_no_count(9usize, 5usize, 18usize, 2127780u32);
    emu.orr_no_count(17usize, 17usize, 14usize, 2127784u32);
    emu.sri_no_count(14usize, 10usize, 2u32, 2127788u32);
    emu.orr_no_count(28usize, 30usize, 28usize, 2127792u32);
    emu.sli_no_count(30usize, 10usize, 30u32, 2127796u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2127800u32);
    emu.sri_no_count(8usize, 10usize, 13u32, 2127804u32);
    emu.xrr_no_count(12usize, 9usize, 12usize, 2127808u32);
    emu.sli_no_count(9usize, 10usize, 19u32, 2127812u32);
    emu.orr_no_count(30usize, 30usize, 14usize, 2127816u32);
    emu.sri_no_count(14usize, 10usize, 22u32, 2127820u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2127824u32);
    emu.sli_no_count(9usize, 10usize, 10u32, 2127828u32);
    emu.orr_no_count(9usize, 9usize, 14usize, 2127832u32);
    emu.xrr_no_count(14usize, 11usize, 16usize, 2127836u32);
    emu.anr_no_count(14usize, 10usize, 14usize, 2127840u32);
    emu.xrr_no_count(29usize, 14usize, 29usize, 2127844u32);
    emu.lw_no_count(14usize, 2usize, 504u32, 2127848u32)?;
    emu.adr_no_count(14usize, 14usize, 13usize, 2127852u32);
    emu.xrr_no_count(17usize, 17usize, 28usize, 2127856u32);
    emu.xrr_no_count(28usize, 5usize, 13usize, 2127860u32);
    emu.adr_no_count(12usize, 6usize, 12usize, 2127864u32);
    emu.xrr_no_count(6usize, 10usize, 11usize, 2127868u32);
    emu.xrr_no_count(30usize, 30usize, 8usize, 2127872u32);
    emu.xrr_no_count(17usize, 17usize, 31usize, 2127876u32);
    emu.lw_no_count(31usize, 2usize, 28u32, 2127880u32)?;
    emu.adr_no_count(12usize, 12usize, 31usize, 2127884u32);
    emu.xrr_no_count(30usize, 30usize, 9usize, 2127888u32);
    emu.adr_no_count(12usize, 12usize, 17usize, 2127892u32);
    emu.adr_no_count(17usize, 30usize, 29usize, 2127896u32);
    emu.adr_no_count(17usize, 17usize, 12usize, 2127900u32);
    emu.adr_no_count(15usize, 12usize, 15usize, 2127904u32);
    emu.sri_no_count(12usize, 15usize, 6u32, 2127908u32);
    emu.sli_no_count(29usize, 15usize, 26u32, 2127912u32);
    emu.sri_no_count(30usize, 15usize, 11u32, 2127916u32);
    emu.sli_no_count(31usize, 15usize, 21u32, 2127920u32);
    emu.sri_no_count(8usize, 15usize, 25u32, 2127924u32);
    emu.sli_no_count(9usize, 15usize, 7u32, 2127928u32);
    emu.anr_no_count(28usize, 15usize, 28usize, 2127932u32);
    emu.orr_no_count(12usize, 29usize, 12usize, 2127936u32);
    emu.sri_no_count(29usize, 17usize, 2u32, 2127940u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2127944u32);
    emu.sli_no_count(31usize, 17usize, 30u32, 2127948u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2127952u32);
    emu.sri_no_count(9usize, 17usize, 13u32, 2127956u32);
    emu.xrr_no_count(28usize, 28usize, 13usize, 2127960u32);
    emu.sli_no_count(13usize, 17usize, 19u32, 2127964u32);
    emu.orr_no_count(29usize, 31usize, 29usize, 2127968u32);
    emu.sri_no_count(31usize, 17usize, 22u32, 2127972u32);
    emu.orr_no_count(9usize, 13usize, 9usize, 2127976u32);
    emu.sli_no_count(13usize, 17usize, 10u32, 2127980u32);
    emu.orr_no_count(31usize, 13usize, 31usize, 2127984u32);
    emu.anr_no_count(13usize, 10usize, 11usize, 2127988u32);
    emu.anr_no_count(6usize, 17usize, 6usize, 2127992u32);
    emu.xrr_no_count(6usize, 6usize, 13usize, 2127996u32);
    emu.lw_no_count(13usize, 2usize, 444u32, 2128000u32)?;
    emu.adr_no_count(13usize, 13usize, 5usize, 2128004u32);
    emu.xrr_no_count(12usize, 12usize, 30usize, 2128008u32);
    emu.anr_no_count(30usize, 17usize, 10usize, 2128012u32);
    emu.adr_no_count(7usize, 7usize, 28usize, 2128016u32);
    emu.xrr_no_count(28usize, 15usize, 5usize, 2128020u32);
    emu.xrr_no_count(29usize, 29usize, 9usize, 2128024u32);
    emu.xrr_no_count(12usize, 12usize, 8usize, 2128028u32);
    emu.lw_no_count(8usize, 2usize, 24u32, 2128032u32)?;
    emu.adr_no_count(7usize, 7usize, 8usize, 2128036u32);
    emu.xrr_no_count(29usize, 29usize, 31usize, 2128040u32);
    emu.adr_no_count(12usize, 7usize, 12usize, 2128044u32);
    emu.adr_no_count(6usize, 29usize, 6usize, 2128048u32);
    emu.adr_no_count(6usize, 6usize, 12usize, 2128052u32);
    emu.adr_no_count(16usize, 12usize, 16usize, 2128056u32);
    emu.sri_no_count(12usize, 16usize, 6u32, 2128060u32);
    emu.sli_no_count(7usize, 16usize, 26u32, 2128064u32);
    emu.sri_no_count(29usize, 16usize, 11u32, 2128068u32);
    emu.sli_no_count(31usize, 16usize, 21u32, 2128072u32);
    emu.sri_no_count(8usize, 16usize, 25u32, 2128076u32);
    emu.sli_no_count(9usize, 16usize, 7u32, 2128080u32);
    emu.anr_no_count(28usize, 16usize, 28usize, 2128084u32);
    emu.orr_no_count(7usize, 7usize, 12usize, 2128088u32);
    emu.sri_no_count(12usize, 6usize, 2u32, 2128092u32);
    emu.orr_no_count(29usize, 31usize, 29usize, 2128096u32);
    emu.sli_no_count(31usize, 6usize, 30u32, 2128100u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2128104u32);
    emu.sri_no_count(9usize, 6usize, 13u32, 2128108u32);
    emu.xrr_no_count(28usize, 28usize, 5usize, 2128112u32);
    emu.sli_no_count(5usize, 6usize, 19u32, 2128116u32);
    emu.orr_no_count(31usize, 31usize, 12usize, 2128120u32);
    emu.sri_no_count(12usize, 6usize, 22u32, 2128124u32);
    emu.orr_no_count(9usize, 5usize, 9usize, 2128128u32);
    emu.sli_no_count(5usize, 6usize, 10u32, 2128132u32);
    emu.orr_no_count(18usize, 5usize, 12usize, 2128136u32);
    emu.xrr_no_count(12usize, 17usize, 10usize, 2128140u32);
    emu.anr_no_count(12usize, 6usize, 12usize, 2128144u32);
    emu.xrr_no_count(30usize, 12usize, 30usize, 2128148u32);
    emu.lw_no_count(12usize, 2usize, 360u32, 2128152u32)?;
    emu.adr_no_count(12usize, 12usize, 15usize, 2128156u32);
    emu.lw_no_count(5usize, 2usize, 424u32, 2128160u32)?;
    emu.adr_no_count(5usize, 17usize, 5usize, 2128164u32);
    emu.xrr_no_count(7usize, 7usize, 29usize, 2128168u32);
    emu.xrr_no_count(29usize, 16usize, 15usize, 2128172u32);
    emu.adr_no_count(14usize, 14usize, 28usize, 2128176u32);
    emu.xrr_no_count(28usize, 6usize, 17usize, 2128180u32);
    emu.anr_no_count(17usize, 6usize, 17usize, 2128184u32);
    emu.adr_no_count(19usize, 6usize, 19usize, 2128188u32);
    emu.xrr_no_count(31usize, 31usize, 9usize, 2128192u32);
    emu.xrr_no_count(7usize, 7usize, 8usize, 2128196u32);
    emu.lw_no_count(8usize, 2usize, 20u32, 2128200u32)?;
    emu.adr_no_count(14usize, 14usize, 8usize, 2128204u32);
    emu.xrr_no_count(31usize, 31usize, 18usize, 2128208u32);
    emu.adr_no_count(14usize, 14usize, 7usize, 2128212u32);
    emu.adr_no_count(30usize, 31usize, 30usize, 2128216u32);
    emu.adr_no_count(30usize, 30usize, 14usize, 2128220u32);
    emu.adr_no_count(11usize, 14usize, 11usize, 2128224u32);
    emu.sri_no_count(14usize, 11usize, 6u32, 2128228u32);
    emu.sli_no_count(7usize, 11usize, 26u32, 2128232u32);
    emu.sri_no_count(31usize, 11usize, 11u32, 2128236u32);
    emu.sli_no_count(8usize, 11usize, 21u32, 2128240u32);
    emu.sri_no_count(9usize, 11usize, 25u32, 2128244u32);
    emu.sli_no_count(18usize, 11usize, 7u32, 2128248u32);
    emu.anr_no_count(29usize, 11usize, 29usize, 2128252u32);
    emu.orr_no_count(14usize, 7usize, 14usize, 2128256u32);
    emu.sri_no_count(7usize, 30usize, 2u32, 2128260u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2128264u32);
    emu.sli_no_count(8usize, 30usize, 30u32, 2128268u32);
    emu.orr_no_count(9usize, 18usize, 9usize, 2128272u32);
    emu.sri_no_count(18usize, 30usize, 13u32, 2128276u32);
    emu.xrr_no_count(15usize, 29usize, 15usize, 2128280u32);
    emu.sli_no_count(29usize, 30usize, 19u32, 2128284u32);
    emu.orr_no_count(7usize, 8usize, 7usize, 2128288u32);
    emu.sri_no_count(8usize, 30usize, 22u32, 2128292u32);
    emu.orr_no_count(29usize, 29usize, 18usize, 2128296u32);
    emu.anr_no_count(18usize, 30usize, 6usize, 2128300u32);
    emu.anr_no_count(28usize, 30usize, 28usize, 2128304u32);
    emu.xrr_no_count(6usize, 30usize, 6usize, 2128308u32);
    emu.adr_no_count(21usize, 30usize, 21usize, 2128312u32);
    emu.sli_no_count(30usize, 30usize, 10u32, 2128316u32);
    emu.orr_no_count(30usize, 30usize, 8usize, 2128320u32);
    emu.xrr_no_count(17usize, 28usize, 17usize, 2128324u32);
    emu.xrr_no_count(14usize, 14usize, 31usize, 2128328u32);
    emu.adr_no_count(13usize, 13usize, 15usize, 2128332u32);
    emu.xrr_no_count(15usize, 7usize, 29usize, 2128336u32);
    emu.adr_no_count(26usize, 11usize, 26usize, 2128340u32);
    emu.xrr_no_count(11usize, 11usize, 16usize, 2128344u32);
    emu.xrr_no_count(14usize, 14usize, 9usize, 2128348u32);
    emu.lw_no_count(9usize, 2usize, 372u32, 2128352u32)?;
    emu.lw_no_count(7usize, 2usize, 16u32, 2128356u32)?;
    emu.adr_no_count(13usize, 13usize, 7usize, 2128360u32);
    emu.xrr_no_count(15usize, 15usize, 30usize, 2128364u32);
    emu.adr_no_count(13usize, 13usize, 14usize, 2128368u32);
    emu.adr_no_count(15usize, 15usize, 17usize, 2128372u32);
    emu.adr_no_count(15usize, 15usize, 13usize, 2128376u32);
    emu.adr_no_count(10usize, 13usize, 10usize, 2128380u32);
    emu.sri_no_count(13usize, 10usize, 6u32, 2128384u32);
    emu.sli_no_count(14usize, 10usize, 26u32, 2128388u32);
    emu.sri_no_count(17usize, 10usize, 11u32, 2128392u32);
    emu.sli_no_count(7usize, 10usize, 21u32, 2128396u32);
    emu.sri_no_count(28usize, 10usize, 25u32, 2128400u32);
    emu.anr_no_count(11usize, 10usize, 11usize, 2128404u32);
    emu.adr_no_count(20usize, 10usize, 20usize, 2128408u32);
    emu.sli_no_count(10usize, 10usize, 7u32, 2128412u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2128416u32);
    emu.sri_no_count(14usize, 15usize, 2u32, 2128420u32);
    emu.orr_no_count(17usize, 7usize, 17usize, 2128424u32);
    emu.sli_no_count(7usize, 15usize, 30u32, 2128428u32);
    emu.orr_no_count(10usize, 10usize, 28usize, 2128432u32);
    emu.sri_no_count(28usize, 15usize, 13u32, 2128436u32);
    emu.orr_no_count(14usize, 7usize, 14usize, 2128440u32);
    emu.sli_no_count(7usize, 15usize, 19u32, 2128444u32);
    emu.orr_no_count(7usize, 7usize, 28usize, 2128448u32);
    emu.sri_no_count(28usize, 15usize, 22u32, 2128452u32);
    emu.anr_no_count(6usize, 15usize, 6usize, 2128456u32);
    emu.adr_no_count(22usize, 15usize, 22usize, 2128460u32);
    emu.sli_no_count(15usize, 15usize, 10u32, 2128464u32);
    emu.orr_no_count(15usize, 15usize, 28usize, 2128468u32);
    emu.xrr_no_count(6usize, 6usize, 18usize, 2128472u32);
    emu.xrr_no_count(13usize, 13usize, 17usize, 2128476u32);
    emu.xrr_no_count(11usize, 11usize, 16usize, 2128480u32);
    emu.adr_no_count(11usize, 12usize, 11usize, 2128484u32);
    emu.xrr_no_count(12usize, 14usize, 7usize, 2128488u32);
    emu.lw_no_count(14usize, 2usize, 508u32, 2128492u32)?;
    emu.adr_no_count(6usize, 6usize, 14usize, 2128496u32);
    emu.xrr_no_count(10usize, 13usize, 10usize, 2128500u32);
    emu.xrr_no_count(12usize, 12usize, 15usize, 2128504u32);
    emu.lw_no_count(13usize, 2usize, 12u32, 2128508u32)?;
    emu.adr_no_count(11usize, 11usize, 13usize, 2128512u32);
    emu.adr_no_count(10usize, 11usize, 10usize, 2128516u32);
    emu.adr_no_count(12usize, 6usize, 12usize, 2128520u32);
    emu.adr_no_count(23usize, 12usize, 10usize, 2128524u32);
    emu.adr_no_count(30usize, 5usize, 10usize, 2128528u32);
    emu.lw_no_count(10usize, 2usize, 368u32, 2128532u32)?;
    emu.adi_no_count(12usize, 10usize, 64u32, 2128536u32);
    emu.adr_no_count(9usize, 16usize, 9usize, 2128540u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2128544u32);
    emu.lw_no_count(10usize, 2usize, 268u32, 2128548u32)?;
    emu.add_memory_rw_events(4002usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2128556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207aac));
    } else {
        emu.pc = 2128552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207aa8));
    }
}
#[inline(always)]
pub fn block_0x00207aa8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2128556u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2112540u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203c1c));
}
#[inline]
pub fn block_0x00207aac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2128560u32)?;
    emu.sw_no_count(23usize, 10usize, 0u32, 2128564u32)?;
    emu.sw_no_count(22usize, 10usize, 4u32, 2128568u32)?;
    emu.sw_no_count(21usize, 10usize, 8u32, 2128572u32)?;
    emu.sw_no_count(19usize, 10usize, 12u32, 2128576u32)?;
    emu.sw_no_count(30usize, 10usize, 16u32, 2128580u32)?;
    emu.sw_no_count(20usize, 10usize, 20u32, 2128584u32)?;
    emu.sw_no_count(26usize, 10usize, 24u32, 2128588u32)?;
    emu.sw_no_count(9usize, 10usize, 28u32, 2128592u32)?;
    emu.lw_no_count(1usize, 2usize, 572u32, 2128596u32)?;
    emu.lw_no_count(8usize, 2usize, 568u32, 2128600u32)?;
    emu.lw_no_count(9usize, 2usize, 564u32, 2128604u32)?;
    emu.lw_no_count(18usize, 2usize, 560u32, 2128608u32)?;
    emu.lw_no_count(19usize, 2usize, 556u32, 2128612u32)?;
    emu.lw_no_count(20usize, 2usize, 552u32, 2128616u32)?;
    emu.lw_no_count(21usize, 2usize, 548u32, 2128620u32)?;
    emu.lw_no_count(22usize, 2usize, 544u32, 2128624u32)?;
    emu.lw_no_count(23usize, 2usize, 540u32, 2128628u32)?;
    emu.lw_no_count(24usize, 2usize, 536u32, 2128632u32)?;
    emu.lw_no_count(25usize, 2usize, 532u32, 2128636u32)?;
    emu.lw_no_count(26usize, 2usize, 528u32, 2128640u32)?;
    emu.lw_no_count(27usize, 2usize, 524u32, 2128644u32)?;
    emu.adi_no_count(2usize, 2usize, 576u32, 2128648u32);
    emu.add_memory_rw_events(24usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128652u32;
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
pub fn block_0x00207b0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2128656u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2128660u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2128664u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2128668u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2128672u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2128676u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2128680u32)?;
    emu.adi_no_count(8usize, 12usize, 0u32, 2128684u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2128716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207b4c));
    } else {
        emu.pc = 2128688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207b30));
    }
}
#[inline(always)]
pub fn block_0x00207b30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2128692u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2128692u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207b34));
}
#[inline(always)]
pub fn block_0x00207b34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2128696u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1596u32, 2128700u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2128704u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2128708u32);
    emu.apc_no_count(1usize, 2128708u32, 16384u32, 2128712u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128716u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965640u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207b4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2128720u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2128852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207bd4));
    } else {
        emu.pc = 2128724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207b54));
    }
}
#[inline(always)]
pub fn block_0x00207b54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 11usize, 0u32, 2128728u32);
    emu.adi_no_count(19usize, 13usize, 0u32, 2128732u32);
    emu.adi_no_count(20usize, 12usize, 0u32, 2128736u32);
    emu.apc_no_count(1usize, 2128736u32, 4294946816u32, 2128740u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128744u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966200u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207b68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2128748u32);
    emu.adi_no_count(9usize, 0usize, 1u32, 2128752u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2128756u32);
    emu.apc_no_count(1usize, 2128756u32, 4294942720u32, 2128760u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128764u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965628u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207b7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2128692u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207b34));
    } else {
        emu.pc = 2128768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207b80));
    }
}
#[inline(always)]
pub fn block_0x00207b80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 10usize, 8usize, 2128772u32);
    emu.adi_no_count(13usize, 10usize, 0u32, 2128776u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2128780u32);
    emu.adi_no_count(14usize, 19usize, 0u32, 2128784u32);
    emu.adi_no_count(15usize, 18usize, 0u32, 2128788u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2128792u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2128820u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207bb4));
}
#[inline(always)]
pub fn block_0x00207b98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltr_no_count(17usize, 0usize, 16usize, 2128796u32);
    emu.sbr_no_count(17usize, 0usize, 17usize, 2128800u32);
    emu.anr_no_count(16usize, 17usize, 16usize, 2128804u32);
    emu.sb_no_count(16usize, 13usize, 0u32, 2128808u32);
    emu.adi_no_count(13usize, 13usize, 1u32, 2128812u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2128816u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2128856u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207bd8));
    } else {
        emu.pc = 2128820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207bb4));
    }
}
#[inline(always)]
pub fn block_0x00207bb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(16usize, 15usize, 0u32, 2128824u32);
    emu.adr_no_count(16usize, 14usize, 16usize, 2128828u32);
    emu.sli_no_count(16usize, 16usize, 16u32, 2128832u32);
    emu.sai_no_count(16usize, 16usize, 1040u32, 2128836u32);
    emu.adi_no_count(17usize, 0usize, 255u32, 2128840u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(16usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2128792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207b98));
    } else {
        emu.pc = 2128844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207bcc));
    }
}
#[inline(always)]
pub fn block_0x00207bcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 255u32, 2128848u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2128852u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2128792u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207b98));
}
#[inline(always)]
pub fn block_0x00207bd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2128856u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2128856u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207bd8));
}
#[inline]
pub fn block_0x00207bd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(8usize, 12usize, 0u32, 2128860u32)?;
    emu.sw_no_count(10usize, 12usize, 4u32, 2128864u32)?;
    emu.sw_no_count(8usize, 12usize, 8u32, 2128868u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2128872u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2128876u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2128880u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2128884u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2128888u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2128892u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2128896u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128900u32;
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
pub fn block_0x00207c04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2128904u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2128908u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2128912u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2128916u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2128920u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2128924u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2128928u32)?;
    emu.adi_no_count(8usize, 12usize, 0u32, 2128932u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2128964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207c44));
    } else {
        emu.pc = 2128936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207c28));
    }
}
#[inline(always)]
pub fn block_0x00207c28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2128940u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2128940u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207c2c));
}
#[inline(always)]
pub fn block_0x00207c2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2128944u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1596u32, 2128948u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2128952u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2128956u32);
    emu.apc_no_count(1usize, 2128956u32, 16384u32, 2128960u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128964u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965392u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207c44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2128968u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2129132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207cec));
    } else {
        emu.pc = 2128972u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207c4c));
    }
}
#[inline(always)]
pub fn block_0x00207c4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 11usize, 0u32, 2128976u32);
    emu.adi_no_count(19usize, 13usize, 0u32, 2128980u32);
    emu.adi_no_count(20usize, 12usize, 0u32, 2128984u32);
    emu.apc_no_count(1usize, 2128984u32, 4294946816u32, 2128988u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128992u32;
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
#[inline(always)]
pub fn block_0x00207c60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2128996u32);
    emu.adi_no_count(9usize, 0usize, 1u32, 2129000u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2129004u32);
    emu.apc_no_count(1usize, 2129004u32, 4294942720u32, 2129008u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129012u32;
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
pub fn block_0x00207c74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2128940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207c2c));
    } else {
        emu.pc = 2129016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207c78));
    }
}
#[inline]
pub fn block_0x00207c78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 10usize, 8usize, 2129020u32);
    let a = 0u32.wrapping_add(1374388224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2129024u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4294955008u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2129028u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 12usize, 1311u32, 2129032u32);
    emu.adi_no_count(14usize, 14usize, 4294966685u32, 2129036u32);
    emu.adi_no_count(15usize, 10usize, 0u32, 2129040u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2129044u32);
    emu.adi_no_count(16usize, 19usize, 0u32, 2129048u32);
    emu.adi_no_count(17usize, 18usize, 0u32, 2129052u32);
    emu.add_memory_rw_events(10usize);
    let return_addr = 2129056u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2129084u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207cbc));
}
#[inline(always)]
pub fn block_0x00207ca0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltr_no_count(5usize, 5usize, 14usize, 2129060u32);
    emu.adi_no_count(5usize, 5usize, 4294967295u32, 2129064u32);
    emu.anr_no_count(5usize, 5usize, 6usize, 2129068u32);
    emu.sb_no_count(5usize, 15usize, 0u32, 2129072u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2129076u32);
    emu.adi_no_count(17usize, 17usize, 1u32, 2129080u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2129136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207cf0));
    } else {
        emu.pc = 2129084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207cbc));
    }
}
#[inline]
pub fn block_0x00207cbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(5usize, 17usize, 0u32, 2129088u32);
    emu.adi_no_count(5usize, 5usize, 4294967168u32, 2129092u32);
    emu.mul_no_count(5usize, 5usize, 16usize, 2129096u32);
    emu.mulh_no_count(6usize, 5usize, 13usize, 2129100u32);
    emu.sri_no_count(7usize, 6usize, 31u32, 2129104u32);
    emu.sai_no_count(6usize, 6usize, 1029u32, 2129108u32);
    emu.adr_no_count(6usize, 6usize, 7usize, 2129112u32);
    emu.adi_no_count(6usize, 6usize, 128u32, 2129116u32);
    emu.adi_no_count(7usize, 0usize, 255u32, 2129120u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a < b {
        emu.pc = 2129056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207ca0));
    } else {
        emu.pc = 2129124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207ce4));
    }
}
#[inline(always)]
pub fn block_0x00207ce4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(6usize, 0usize, 255u32, 2129128u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2129132u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2129056u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207ca0));
}
#[inline(always)]
pub fn block_0x00207cec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2129136u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2129136u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207cf0));
}
#[inline]
pub fn block_0x00207cf0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(8usize, 12usize, 0u32, 2129140u32)?;
    emu.sw_no_count(10usize, 12usize, 4u32, 2129144u32)?;
    emu.sw_no_count(8usize, 12usize, 8u32, 2129148u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2129152u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2129156u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2129160u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2129164u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2129168u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2129172u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2129176u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129180u32;
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
pub fn block_0x00207d1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2129384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207de8));
    } else {
        emu.pc = 2129184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207d20));
    }
}
#[inline]
pub fn block_0x00207d20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967008u32, 2129188u32);
    emu.sw_no_count(1usize, 2usize, 284u32, 2129192u32)?;
    emu.sw_no_count(8usize, 2usize, 280u32, 2129196u32)?;
    emu.sw_no_count(9usize, 2usize, 276u32, 2129200u32)?;
    emu.sw_no_count(18usize, 2usize, 272u32, 2129204u32)?;
    emu.sw_no_count(19usize, 2usize, 268u32, 2129208u32)?;
    emu.sw_no_count(20usize, 2usize, 264u32, 2129212u32)?;
    emu.adi_no_count(8usize, 12usize, 0u32, 2129216u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2129220u32);
    emu.adi_no_count(18usize, 0usize, 0u32, 2129224u32);
    emu.adi_no_count(19usize, 11usize, 3u32, 2129228u32);
    emu.adi_no_count(20usize, 2usize, 72u32, 2129232u32);
    emu.add_memory_rw_events(12usize);
    emu.pc = 2129232u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207d50));
}
#[inline(always)]
pub fn block_0x00207d50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 8u32, 2129236u32);
    emu.adi_no_count(12usize, 0usize, 256u32, 2129240u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2129244u32);
    emu.apc_no_count(1usize, 2129244u32, 4294946816u32, 2129248u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129252u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(564u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207d64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 19usize, 0u32, 2129256u32);
    emu.adi_no_count(11usize, 2usize, 8u32, 2129260u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2129260u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207d6c));
}
#[inline]
pub fn block_0x00207d6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(12usize, 10usize, 4294967295u32, 2129264u32);
    emu.lbu_no_count(13usize, 10usize, 4294967294u32, 2129268u32);
    emu.lbu_no_count(14usize, 10usize, 4294967293u32, 2129272u32);
    emu.lbu_no_count(15usize, 10usize, 0u32, 2129276u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2129280u32);
    emu.sli_no_count(13usize, 13usize, 16u32, 2129284u32);
    emu.sli_no_count(14usize, 14usize, 24u32, 2129288u32);
    emu.orr_no_count(12usize, 12usize, 15usize, 2129292u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2129296u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2129300u32);
    emu.sw_no_count(12usize, 11usize, 0u32, 2129304u32)?;
    emu.adi_no_count(11usize, 11usize, 4u32, 2129308u32);
    emu.adi_no_count(10usize, 10usize, 4u32, 2129312u32);
    emu.add_memory_rw_events(13usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2129260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207d6c));
    } else {
        emu.pc = 2129316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207da4));
    }
}
#[inline(always)]
pub fn block_0x00207da4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 18usize, 1u32, 2129320u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2129324u32);
    emu.apc_no_count(1usize, 2129324u32, 4294950912u32, 2129328u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129332u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965348u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207db4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 8u32, 2129336u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2129340u32);
    emu.apc_no_count(1usize, 2129340u32, 4294946816u32, 2129344u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129348u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(452u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207dc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 19usize, 64u32, 2129352u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2129232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207d50));
    } else {
        emu.pc = 2129356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207dcc));
    }
}
#[inline(always)]
pub fn block_0x00207dcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 284u32, 2129360u32)?;
    emu.lw_no_count(8usize, 2usize, 280u32, 2129364u32)?;
    emu.lw_no_count(9usize, 2usize, 276u32, 2129368u32)?;
    emu.lw_no_count(18usize, 2usize, 272u32, 2129372u32)?;
    emu.lw_no_count(19usize, 2usize, 268u32, 2129376u32)?;
    emu.lw_no_count(20usize, 2usize, 264u32, 2129380u32)?;
    emu.adi_no_count(2usize, 2usize, 288u32, 2129384u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2129384u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207de8));
}
#[inline(always)]
pub fn block_0x00207de8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129388u32;
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
pub fn block_0x00207dec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2129392u32)?;
    emu.lw_no_count(13usize, 10usize, 4u32, 2129396u32)?;
    emu.adi_no_count(14usize, 11usize, 0u32, 2129400u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2129404u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2129408u32);
    emu.adi_no_count(12usize, 14usize, 0u32, 2129412u32);
    emu.apc_no_count(6usize, 2129412u32, 36864u32, 2129416u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2129420u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1412u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207e0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 0u32, 2129424u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2129428u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1684u32, 2129432u32);
    emu.apc_no_count(6usize, 2129432u32, 32768u32, 2129436u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2129440u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(732u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00207e20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2129444u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2129448u32)?;
    emu.lbu_no_count(12usize, 10usize, 0u32, 2129452u32);
    emu.sli_no_count(12usize, 12usize, 2u32, 2129456u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2129460u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1612u32, 2129464u32);
    emu.adr_no_count(12usize, 12usize, 13usize, 2129468u32);
    emu.lw_no_count(12usize, 12usize, 0u32, 2129472u32)?;
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2129476u32;
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
pub fn block_0x00207e44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(12usize, 10usize, 1u32, 2129480u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2129484u32);
    let a = 0u32.wrapping_add(2166784u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2129488u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 852u32, 2129492u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2129496u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1776u32, 2129500u32);
    emu.adi_no_count(15usize, 0usize, 2u32, 2129504u32);
    emu.adi_no_count(16usize, 2usize, 32u32, 2129508u32);
    emu.adi_no_count(17usize, 0usize, 1u32, 2129512u32);
    emu.sw_no_count(10usize, 2usize, 32u32, 2129516u32)?;
    emu.sw_no_count(13usize, 2usize, 36u32, 2129520u32)?;
    emu.lw_no_count(10usize, 11usize, 0u32, 2129524u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2129528u32)?;
    emu.sw_no_count(14usize, 2usize, 8u32, 2129532u32)?;
    emu.sw_no_count(15usize, 2usize, 12u32, 2129536u32)?;
    emu.sw_no_count(16usize, 2usize, 16u32, 2129540u32)?;
    emu.sw_no_count(17usize, 2usize, 20u32, 2129544u32)?;
    emu.sb_no_count(12usize, 2usize, 0u32, 2129548u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2129552u32)?;
    emu.adi_no_count(12usize, 2usize, 8u32, 2129556u32);
    emu.apc_no_count(1usize, 2129556u32, 32768u32, 2129560u32);
    emu.add_memory_rw_events(22usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129564u32;
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
#[inline(always)]
pub fn block_0x00207e9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2129568u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2129572u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129576u32;
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
pub fn block_0x00207ea8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2129580u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1959u32, 2129584u32);
    emu.adi_no_count(12usize, 0usize, 14u32, 2129588u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2129592u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2129596u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2129600u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2129604u32);
    emu.apc_no_count(6usize, 2129604u32, 36864u32, 2129608u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2129612u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x00207ecc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2129616u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1931u32, 2129620u32);
    emu.adi_no_count(12usize, 0usize, 15u32, 2129624u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2129628u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2129632u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2129636u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2129640u32);
    emu.apc_no_count(6usize, 2129640u32, 36864u32, 2129644u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2129648u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(180u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00207ef0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2129652u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1916u32, 2129656u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2129660u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2129664u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2129668u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2129672u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2129676u32);
    emu.apc_no_count(6usize, 2129676u32, 36864u32, 2129680u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2129684u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(144u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00207f14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 4u32, 2129688u32)?;
    emu.adi_no_count(10usize, 2usize, 0u32, 2129692u32);
    let a = 0u32.wrapping_add(2170880u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2129696u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294965308u32, 2129700u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2129704u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1848u32, 2129708u32);
    emu.adi_no_count(15usize, 0usize, 2u32, 2129712u32);
    emu.adi_no_count(16usize, 2usize, 32u32, 2129716u32);
    emu.adi_no_count(17usize, 0usize, 1u32, 2129720u32);
    emu.sw_no_count(10usize, 2usize, 32u32, 2129724u32)?;
    emu.sw_no_count(13usize, 2usize, 36u32, 2129728u32)?;
    emu.lw_no_count(10usize, 11usize, 0u32, 2129732u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2129736u32)?;
    emu.sw_no_count(14usize, 2usize, 8u32, 2129740u32)?;
    emu.sw_no_count(15usize, 2usize, 12u32, 2129744u32)?;
    emu.sw_no_count(16usize, 2usize, 16u32, 2129748u32)?;
    emu.sw_no_count(17usize, 2usize, 20u32, 2129752u32)?;
    emu.sw_no_count(12usize, 2usize, 0u32, 2129756u32)?;
    emu.sw_no_count(0usize, 2usize, 24u32, 2129760u32)?;
    emu.adi_no_count(12usize, 2usize, 8u32, 2129764u32);
    emu.apc_no_count(1usize, 2129764u32, 32768u32, 2129768u32);
    emu.add_memory_rw_events(22usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129772u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(400u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207f6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2129776u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2129780u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129784u32;
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
pub fn block_0x00207f78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2183168u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2129788u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1484u32, 2129792u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2129796u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2129800u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2129804u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2129808u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2129812u32);
    emu.apc_no_count(6usize, 2129812u32, 36864u32, 2129816u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2129820u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x00207f9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2129824u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1902u32, 2129828u32);
    emu.adi_no_count(12usize, 0usize, 14u32, 2129832u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2129836u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2129840u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2129844u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2129848u32);
    emu.apc_no_count(6usize, 2129848u32, 36864u32, 2129852u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2129856u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294967268u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207fc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 8u32, 2129860u32)?;
    emu.lw_no_count(10usize, 10usize, 12u32, 2129864u32)?;
    emu.adi_no_count(13usize, 2usize, 0u32, 2129868u32);
    let a = 0u32.wrapping_add(2146304u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2129872u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 712u32, 2129876u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2129880u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2130036u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208074));
}
#[inline]
pub fn block_0x00207fd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2129884u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1946u32, 2129888u32);
    emu.adi_no_count(12usize, 0usize, 13u32, 2129892u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2129896u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2129900u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2129904u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2129908u32);
    emu.apc_no_count(6usize, 2129908u32, 36864u32, 2129912u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2129916u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294967208u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00207ffc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 8u32, 2129920u32)?;
    emu.lw_no_count(10usize, 10usize, 12u32, 2129924u32)?;
    emu.adi_no_count(13usize, 2usize, 32u32, 2129928u32);
    let a = 0u32.wrapping_add(2129920u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2129932u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 520u32, 2129936u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2129940u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 1820u32, 2129944u32);
    emu.adi_no_count(16usize, 0usize, 2u32, 2129948u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2129952u32)?;
    emu.adi_no_count(17usize, 2usize, 0u32, 2129956u32);
    emu.sw_no_count(13usize, 2usize, 0u32, 2129960u32)?;
    emu.sw_no_count(14usize, 2usize, 4u32, 2129964u32)?;
    emu.adi_no_count(13usize, 0usize, 1u32, 2129968u32);
    emu.sw_no_count(12usize, 2usize, 32u32, 2129972u32)?;
    emu.sw_no_count(10usize, 2usize, 36u32, 2129976u32)?;
    emu.add_memory_rw_events(16usize);
    let return_addr = 2129980u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2130076u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020809c));
}
#[inline]
pub fn block_0x0020803c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2129984u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1880u32, 2129988u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2129992u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2129996u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2130000u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2130004u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2130008u32);
    emu.apc_no_count(6usize, 2130008u32, 36864u32, 2130012u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2130016u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294967108u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208060(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 8u32, 2130020u32)?;
    emu.lw_no_count(10usize, 10usize, 12u32, 2130024u32)?;
    emu.adi_no_count(13usize, 2usize, 0u32, 2130028u32);
    let a = 0u32.wrapping_add(2146304u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2130032u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 592u32, 2130036u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2130036u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208074));
}
#[inline]
pub fn block_0x00208074(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2130040u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 1804u32, 2130044u32);
    emu.adi_no_count(16usize, 0usize, 2u32, 2130048u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2130052u32)?;
    emu.adi_no_count(17usize, 2usize, 32u32, 2130056u32);
    emu.sw_no_count(13usize, 2usize, 32u32, 2130060u32)?;
    emu.sw_no_count(14usize, 2usize, 36u32, 2130064u32)?;
    emu.adi_no_count(13usize, 0usize, 1u32, 2130068u32);
    emu.sw_no_count(12usize, 2usize, 0u32, 2130072u32)?;
    emu.sw_no_count(10usize, 2usize, 4u32, 2130076u32)?;
    emu.add_memory_rw_events(10usize);
    emu.pc = 2130076u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020809c));
}
#[inline]
pub fn block_0x0020809c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 11usize, 0u32, 2130080u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2130084u32)?;
    emu.sw_no_count(15usize, 2usize, 8u32, 2130088u32)?;
    emu.sw_no_count(16usize, 2usize, 12u32, 2130092u32)?;
    emu.sw_no_count(17usize, 2usize, 16u32, 2130096u32)?;
    emu.sw_no_count(13usize, 2usize, 20u32, 2130100u32)?;
    emu.adi_no_count(12usize, 2usize, 8u32, 2130104u32);
    emu.apc_no_count(1usize, 2130104u32, 32768u32, 2130108u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130112u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(60u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002080c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2130116u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2130120u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130124u32;
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
pub fn block_0x002080cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 4u32, 2130128u32)?;
    emu.lw_no_count(12usize, 10usize, 8u32, 2130132u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2130136u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2130140u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2130144u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2130148u32);
    emu.apc_no_count(6usize, 2130148u32, 36864u32, 2130152u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2130156u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966968u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002080ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 23u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 4u32, 2130160u32)?;
    emu.lw_no_count(10usize, 10usize, 8u32, 2130164u32)?;
    emu.adi_no_count(13usize, 2usize, 0u32, 2130168u32);
    let a = 0u32.wrapping_add(2129920u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2130172u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966764u32, 2130176u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2130180u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 1872u32, 2130184u32);
    emu.adi_no_count(16usize, 0usize, 1u32, 2130188u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2130192u32)?;
    emu.sw_no_count(13usize, 2usize, 32u32, 2130196u32)?;
    emu.sw_no_count(14usize, 2usize, 36u32, 2130200u32)?;
    emu.adi_no_count(13usize, 2usize, 32u32, 2130204u32);
    emu.sw_no_count(12usize, 2usize, 0u32, 2130208u32)?;
    emu.sw_no_count(10usize, 2usize, 4u32, 2130212u32)?;
    emu.lw_no_count(10usize, 11usize, 0u32, 2130216u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2130220u32)?;
    emu.sw_no_count(15usize, 2usize, 8u32, 2130224u32)?;
    emu.sw_no_count(16usize, 2usize, 12u32, 2130228u32)?;
    emu.sw_no_count(13usize, 2usize, 16u32, 2130232u32)?;
    emu.sw_no_count(16usize, 2usize, 20u32, 2130236u32)?;
    emu.adi_no_count(12usize, 2usize, 8u32, 2130240u32);
    emu.apc_no_count(1usize, 2130240u32, 32768u32, 2130244u32);
    emu.add_memory_rw_events(23usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130248u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967220u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208148(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2130252u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2130256u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130260u32;
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
pub fn block_0x00208154(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2130264u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1919u32, 2130268u32);
    emu.adi_no_count(12usize, 0usize, 12u32, 2130272u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2130276u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2130280u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2130284u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2130288u32);
    emu.apc_no_count(6usize, 2130288u32, 36864u32, 2130292u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2130296u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966828u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00208178(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2130300u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1756u32, 2130304u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2130308u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2130312u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2130316u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2130320u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2130324u32);
    emu.apc_no_count(6usize, 2130324u32, 36864u32, 2130328u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2130332u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966792u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020819c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2130336u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1708u32, 2130340u32);
    emu.adi_no_count(12usize, 0usize, 8u32, 2130344u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2130348u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2130352u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2130356u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2130360u32);
    emu.apc_no_count(6usize, 2130360u32, 36864u32, 2130364u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2130368u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966756u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002081c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2130372u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1890u32, 2130376u32);
    emu.adi_no_count(12usize, 0usize, 12u32, 2130380u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2130384u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2130388u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2130392u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2130396u32);
    emu.apc_no_count(6usize, 2130396u32, 36864u32, 2130400u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2130404u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966720u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002081e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 0u32, 2130408u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2130412u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2130416u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2130420u32);
    emu.apc_no_count(6usize, 2130420u32, 36864u32, 2130424u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2130428u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966696u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002081fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(6usize, 11usize, 12u32, 2130432u32)?;
    emu.adi_no_count(11usize, 12usize, 0u32, 2130436u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2130440u32;
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
pub fn block_0x00208208(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2130444u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2130448u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2130452u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2130456u32)?;
    emu.sli_no_count(12usize, 12usize, 1u32, 2130460u32);
    emu.sri_no_count(12usize, 12usize, 1u32, 2130464u32);
    let a = 0u32.wrapping_add(2146435072u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2130468u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2130560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208280));
    } else {
        emu.pc = 2130472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208228));
    }
}
#[inline]
pub fn block_0x00208228(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2150400u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2130476u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1844u32, 2130480u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2130484u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1732u32, 2130488u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2130492u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2130496u32)?;
    emu.adi_no_count(15usize, 2usize, 32u32, 2130500u32);
    emu.sw_no_count(10usize, 2usize, 32u32, 2130504u32)?;
    emu.sw_no_count(12usize, 2usize, 36u32, 2130508u32)?;
    emu.lw_no_count(10usize, 11usize, 0u32, 2130512u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2130516u32)?;
    emu.sw_no_count(13usize, 2usize, 8u32, 2130520u32)?;
    emu.sw_no_count(14usize, 2usize, 12u32, 2130524u32)?;
    emu.sw_no_count(15usize, 2usize, 16u32, 2130528u32)?;
    emu.sw_no_count(14usize, 2usize, 20u32, 2130532u32)?;
    emu.adi_no_count(12usize, 2usize, 8u32, 2130536u32);
    emu.apc_no_count(1usize, 2130536u32, 32768u32, 2130540u32);
    emu.add_memory_rw_events(18usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130544u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966924u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208270(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2130548u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2130552u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2130556u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130560u32;
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
pub fn block_0x00208280(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 11usize, 0u32, 2130564u32);
    emu.sw_no_count(11usize, 2usize, 0u32, 2130568u32)?;
    emu.sb_no_count(0usize, 2usize, 4u32, 2130572u32);
    let a = 0u32.wrapping_add(2150400u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2130576u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1844u32, 2130580u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2130584u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1732u32, 2130588u32);
    emu.adi_no_count(13usize, 0usize, 1u32, 2130592u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2130596u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2130600u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2130604u32)?;
    emu.adi_no_count(10usize, 2usize, 32u32, 2130608u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2130612u32)?;
    emu.sw_no_count(13usize, 2usize, 12u32, 2130616u32)?;
    emu.sw_no_count(10usize, 2usize, 16u32, 2130620u32)?;
    emu.sw_no_count(13usize, 2usize, 20u32, 2130624u32)?;
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2130628u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1684u32, 2130632u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2130636u32);
    emu.adi_no_count(12usize, 2usize, 8u32, 2130640u32);
    emu.apc_no_count(1usize, 2130640u32, 32768u32, 2130644u32);
    emu.add_memory_rw_events(22usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130648u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966820u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002082d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2130672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002082f0));
    } else {
        emu.pc = 2130652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002082dc));
    }
}
#[inline(always)]
pub fn block_0x002082dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2130656u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2130660u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2130664u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2130668u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130672u32;
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
pub fn block_0x002082f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 4u32, 2130676u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2130708u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208314));
    } else {
        emu.pc = 2130680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002082f8));
    }
}
#[inline(always)]
pub fn block_0x002082f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2130684u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1973u32, 2130688u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2130692u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2130696u32);
    emu.apc_no_count(1usize, 2130696u32, 36864u32, 2130700u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130704u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966420u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208310(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2130652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002082dc));
    } else {
        emu.pc = 2130708u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208314));
    }
}
#[inline(always)]
pub fn block_0x00208314(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2130712u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2130716u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2130720u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2130724u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130728u32;
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
pub fn block_0x00208328(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2130732u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2130736u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2130740u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2130744u32)?;
    emu.sw_no_count(18usize, 2usize, 0u32, 2130748u32)?;
    emu.adi_no_count(13usize, 0usize, 7u32, 2130752u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2130756u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2130812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020837c));
    } else {
        emu.pc = 2130760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208348));
    }
}
#[inline(always)]
pub fn block_0x00208348(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2130852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002083a4));
    } else {
        emu.pc = 2130764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020834c));
    }
}
#[inline(always)]
pub fn block_0x0020834c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 12usize, 4294967295u32, 2130768u32);
    emu.adi_no_count(10usize, 0usize, 46u32, 2130772u32);
    emu.adi_no_count(14usize, 11usize, 0u32, 2130776u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2130776u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208358));
}
#[inline(always)]
pub fn block_0x00208358(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(17usize, 14usize, 0u32, 2130780u32);
    emu.adi_no_count(13usize, 17usize, 4294967250u32, 2130784u32);
    emu.sltiu_no_count(13usize, 13usize, 1u32, 2130788u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2130856u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002083a8));
    } else {
        emu.pc = 2130792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208368));
    }
}
#[inline(always)]
pub fn block_0x00208368(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 15usize, 0u32, 2130796u32);
    emu.adi_no_count(15usize, 15usize, 4294967295u32, 2130800u32);
    emu.adi_no_count(14usize, 14usize, 1u32, 2130804u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a != b {
        emu.pc = 2130776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208358));
    } else {
        emu.pc = 2130808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208378));
    }
}
#[inline(always)]
pub fn block_0x00208378(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2130812u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2130856u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002083a8));
}
#[inline(always)]
pub fn block_0x0020837c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 46u32, 2130816u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2130820u32);
    emu.adi_no_count(18usize, 12usize, 0u32, 2130824u32);
    emu.apc_no_count(1usize, 2130824u32, 24576u32, 2130828u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130832u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967028u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208390(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 9usize, 0u32, 2130836u32);
    emu.adi_no_count(12usize, 18usize, 0u32, 2130840u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2130844u32);
    emu.sltiu_no_count(13usize, 10usize, 1u32, 2130848u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2130852u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2130856u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002083a8));
}
#[inline(always)]
pub fn block_0x002083a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 0u32, 2130856u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2130856u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002083a8));
}
#[inline]
pub fn block_0x002083a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(14usize, 8usize, 4u32, 2130860u32);
    emu.lw_no_count(10usize, 8usize, 0u32, 2130864u32)?;
    emu.orr_no_count(13usize, 13usize, 14usize, 2130868u32);
    emu.sb_no_count(13usize, 8usize, 4u32, 2130872u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2130876u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2130880u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2130884u32)?;
    emu.lw_no_count(18usize, 2usize, 0u32, 2130888u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2130892u32);
    emu.apc_no_count(6usize, 2130892u32, 36864u32, 2130896u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2130900u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966224u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002083d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(13usize, 10usize, 4u32, 2130904u32);
    emu.lw_no_count(12usize, 10usize, 0u32, 2130908u32)?;
    emu.adi_no_count(14usize, 11usize, 4294967250u32, 2130912u32);
    emu.sltiu_no_count(14usize, 14usize, 1u32, 2130916u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2130920u32);
    emu.sb_no_count(13usize, 10usize, 4u32, 2130924u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2130928u32);
    emu.apc_no_count(6usize, 2130928u32, 36864u32, 2130932u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2130936u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294967124u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002083f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2130940u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1975u32, 2130944u32);
    emu.adi_no_count(12usize, 0usize, 5u32, 2130948u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2130952u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2130956u32);
    emu.apc_no_count(6usize, 2130956u32, 36864u32, 2130960u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2130964u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966160u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00208414(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2130968u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2130972u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2130976u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2130980u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2130984u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2130988u32)?;
    emu.sw_no_count(20usize, 2usize, 24u32, 2130992u32)?;
    emu.sw_no_count(21usize, 2usize, 20u32, 2130996u32)?;
    emu.sw_no_count(22usize, 2usize, 16u32, 2131000u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2131004u32);
    emu.adi_no_count(10usize, 0usize, 128u32, 2131008u32);
    emu.sw_no_count(0usize, 2usize, 12u32, 2131012u32)?;
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2131028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208454));
    } else {
        emu.pc = 2131016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208448));
    }
}
#[inline(always)]
pub fn block_0x00208448(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 2usize, 12u32, 2131020u32);
    emu.adi_no_count(18usize, 0usize, 1u32, 2131024u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2131028u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2131184u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002084f0));
}
#[inline(always)]
pub fn block_0x00208454(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 11u32, 2131032u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2131068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020847c));
    } else {
        emu.pc = 2131036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020845c));
    }
}
#[inline(always)]
pub fn block_0x0020845c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 6u32, 2131040u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2131044u32);
    emu.ori_no_count(10usize, 10usize, 192u32, 2131048u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2131052u32);
    emu.sb_no_count(10usize, 2usize, 12u32, 2131056u32);
    emu.sb_no_count(11usize, 2usize, 13u32, 2131060u32);
    emu.adi_no_count(18usize, 0usize, 2u32, 2131064u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2131068u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2131184u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002084f0));
}
#[inline(always)]
pub fn block_0x0020847c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 16u32, 2131072u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2131124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002084b4));
    } else {
        emu.pc = 2131076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208484));
    }
}
#[inline]
pub fn block_0x00208484(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 12u32, 2131080u32);
    emu.sli_no_count(12usize, 11usize, 20u32, 2131084u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2131088u32);
    emu.ori_no_count(10usize, 10usize, 224u32, 2131092u32);
    emu.sri_no_count(12usize, 12usize, 26u32, 2131096u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2131100u32);
    emu.adi_no_count(12usize, 12usize, 128u32, 2131104u32);
    emu.sb_no_count(10usize, 2usize, 12u32, 2131108u32);
    emu.sb_no_count(12usize, 2usize, 13u32, 2131112u32);
    emu.sb_no_count(11usize, 2usize, 14u32, 2131116u32);
    emu.adi_no_count(18usize, 0usize, 3u32, 2131120u32);
    emu.add_memory_rw_events(12usize);
    let return_addr = 2131124u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2131184u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002084f0));
}
#[inline]
pub fn block_0x002084b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 18u32, 2131128u32);
    emu.sli_no_count(12usize, 11usize, 14u32, 2131132u32);
    emu.sli_no_count(13usize, 11usize, 20u32, 2131136u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2131140u32);
    emu.ori_no_count(10usize, 10usize, 240u32, 2131144u32);
    emu.sri_no_count(12usize, 12usize, 26u32, 2131148u32);
    emu.sri_no_count(13usize, 13usize, 26u32, 2131152u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2131156u32);
    emu.adi_no_count(12usize, 12usize, 128u32, 2131160u32);
    emu.adi_no_count(13usize, 13usize, 128u32, 2131164u32);
    emu.sb_no_count(10usize, 2usize, 12u32, 2131168u32);
    emu.sb_no_count(12usize, 2usize, 13u32, 2131172u32);
    emu.sb_no_count(13usize, 2usize, 14u32, 2131176u32);
    emu.sb_no_count(11usize, 2usize, 15u32, 2131180u32);
    emu.adi_no_count(18usize, 0usize, 4u32, 2131184u32);
    emu.add_memory_rw_events(15usize);
    emu.pc = 2131184u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002084f0));
}
#[inline]
pub fn block_0x002084f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(19usize, 8usize, 8u32, 2131188u32)?;
    emu.lw_no_count(10usize, 19usize, 4u32, 2131192u32)?;
    emu.lw_no_count(21usize, 19usize, 8u32, 2131196u32)?;
    emu.lw_no_count(20usize, 19usize, 12u32, 2131200u32)?;
    emu.lw_no_count(11usize, 19usize, 0u32, 2131204u32)?;
    emu.sltru_no_count(12usize, 21usize, 10usize, 2131208u32);
    emu.sltiu_no_count(13usize, 20usize, 1u32, 2131212u32);
    emu.anr_no_count(14usize, 13usize, 12usize, 2131216u32);
    emu.adi_no_count(12usize, 21usize, 0u32, 2131220u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a != b {
        emu.pc = 2131228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020851c));
    } else {
        emu.pc = 2131224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208518));
    }
}
#[inline(always)]
pub fn block_0x00208518(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2131228u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2131228u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020851c));
}
#[inline]
pub fn block_0x0020851c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2131232u32);
    emu.orr_no_count(13usize, 13usize, 21usize, 2131236u32);
    emu.sbr_no_count(13usize, 10usize, 13usize, 2131240u32);
    emu.sltru_no_count(10usize, 10usize, 13usize, 2131244u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2131248u32);
    emu.anr_no_count(22usize, 10usize, 13usize, 2131252u32);
    emu.adr_no_count(10usize, 11usize, 12usize, 2131256u32);
    emu.adi_no_count(9usize, 22usize, 0u32, 2131260u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a < b {
        emu.pc = 2131268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208544));
    } else {
        emu.pc = 2131264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208540));
    }
}
#[inline(always)]
pub fn block_0x00208540(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 18usize, 0u32, 2131268u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2131268u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208544));
}
#[inline(always)]
pub fn block_0x00208544(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 2usize, 12u32, 2131272u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2131276u32);
    emu.apc_no_count(1usize, 2131276u32, 4294946816u32, 2131280u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131284u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966076u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208554(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(9usize, 21usize, 9usize, 2131288u32);
    emu.sltru_no_count(10usize, 9usize, 21usize, 2131292u32);
    emu.adr_no_count(10usize, 20usize, 10usize, 2131296u32);
    emu.sw_no_count(9usize, 19usize, 8u32, 2131300u32)?;
    emu.sw_no_count(10usize, 19usize, 12u32, 2131304u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a >= b {
        emu.pc = 2131328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208580));
    } else {
        emu.pc = 2131308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020856c));
    }
}
#[inline(always)]
pub fn block_0x0020856c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2131312u32;
    emu.update_insn_clock();
    emu.lw_no_count(19usize, 10usize, 4294965296u32, 2131316u32)?;
    emu.ani_no_count(12usize, 19usize, 255u32, 2131320u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2131324u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2131336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208588));
    } else {
        emu.pc = 2131328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208580));
    }
}
#[inline(always)]
pub fn block_0x00208580(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2131332u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2131336u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2131372u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002085ac));
}
#[inline(always)]
pub fn block_0x00208588(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(20usize, 10usize, 4294965300u32, 2131340u32)?;
    emu.lbu_no_count(10usize, 8usize, 0u32, 2131344u32);
    emu.lw_no_count(9usize, 8usize, 4u32, 2131348u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2131412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002085d4));
    } else {
        emu.pc = 2131352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208598));
    }
}
#[inline(always)]
pub fn block_0x00208598(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 3u32, 2131356u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2131412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002085d4));
    } else {
        emu.pc = 2131360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002085a0));
    }
}
#[inline(always)]
pub fn block_0x002085a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(19usize, 8usize, 0u32, 2131364u32)?;
    emu.sw_no_count(20usize, 8usize, 4u32, 2131368u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2131372u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2131372u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002085ac));
}
#[inline]
pub fn block_0x002085ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2131376u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2131380u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2131384u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2131388u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2131392u32)?;
    emu.lw_no_count(20usize, 2usize, 24u32, 2131396u32)?;
    emu.lw_no_count(21usize, 2usize, 20u32, 2131400u32)?;
    emu.lw_no_count(22usize, 2usize, 16u32, 2131404u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2131408u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131412u32;
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
pub fn block_0x002085d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(21usize, 9usize, 4u32, 2131416u32)?;
    emu.lw_no_count(11usize, 21usize, 0u32, 2131420u32)?;
    emu.lw_no_count(18usize, 9usize, 0u32, 2131424u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2131436u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002085ec));
    } else {
        emu.pc = 2131428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002085e4));
    }
}
#[inline(always)]
pub fn block_0x002085e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2131432u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2131436u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x002085ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 21usize, 4u32, 2131440u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2131460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208604));
    } else {
        emu.pc = 2131444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002085f4));
    }
}
#[inline(always)]
pub fn block_0x002085f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 21usize, 8u32, 2131448u32)?;
    emu.adi_no_count(10usize, 18usize, 0u32, 2131452u32);
    emu.apc_no_count(1usize, 2131452u32, 4294938624u32, 2131456u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131460u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967056u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208604(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 12u32, 2131464u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2131468u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2131472u32);
    emu.apc_no_count(1usize, 2131472u32, 4294938624u32, 2131476u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131480u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967036u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208618(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2131484u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2131360u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002085a0));
}
