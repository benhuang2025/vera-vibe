pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2110292u32;
pub const PC_MAX: u32 = 2116088u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 107usize] = [
        block_0x00203354,
        block_0x00203368,
        block_0x0020337c,
        block_0x00203390,
        block_0x002033a4,
        block_0x002033b8,
        block_0x002033cc,
        block_0x002033e0,
        block_0x002033f4,
        block_0x00203408,
        block_0x0020341c,
        block_0x00203430,
        block_0x00203444,
        block_0x00203458,
        block_0x00203470,
        block_0x00203484,
        block_0x00203490,
        block_0x002034ac,
        block_0x002034c4,
        block_0x002034cc,
        block_0x002034d4,
        block_0x002034e8,
        block_0x002034fc,
        block_0x00203500,
        block_0x00203518,
        block_0x00203540,
        block_0x00203570,
        block_0x00203578,
        block_0x00203598,
        block_0x002035b4,
        block_0x002035cc,
        block_0x002035e4,
        block_0x002035f8,
        block_0x00203604,
        block_0x00203618,
        block_0x0020362c,
        block_0x00203640,
        block_0x00203644,
        block_0x00203654,
        block_0x00203664,
        block_0x00203674,
        block_0x00203684,
        block_0x00203698,
        block_0x002036a4,
        block_0x002036b8,
        block_0x002036f8,
        block_0x002037c4,
        block_0x00203858,
        block_0x002038ec,
        block_0x00203980,
        block_0x00203a14,
        block_0x00203a28,
        block_0x00203abc,
        block_0x00203ad0,
        block_0x00203b64,
        block_0x00203bf8,
        block_0x00203c0c,
        block_0x00203ca0,
        block_0x00203cb4,
        block_0x00203d48,
        block_0x00203ddc,
        block_0x00203df0,
        block_0x00203e84,
        block_0x00203e98,
        block_0x00203f4c,
        block_0x00203fa0,
        block_0x00203fb0,
        block_0x00204004,
        block_0x00204098,
        block_0x002040ac,
        block_0x002040bc,
        block_0x00204110,
        block_0x00204144,
        block_0x002041d8,
        block_0x002041ec,
        block_0x002041fc,
        block_0x00204250,
        block_0x00204260,
        block_0x002042b4,
        block_0x002042c8,
        block_0x0020435c,
        block_0x002043f0,
        block_0x00204404,
        block_0x00204458,
        block_0x002044ac,
        block_0x002044c0,
        block_0x002044d4,
        block_0x002044e8,
        block_0x002044fc,
        block_0x0020452c,
        block_0x00204570,
        block_0x00204580,
        block_0x00204590,
        block_0x00204624,
        block_0x00204634,
        block_0x002046c8,
        block_0x002046d8,
        block_0x0020478c,
        block_0x002047e0,
        block_0x002047f0,
        block_0x00204844,
        block_0x002048d8,
        block_0x0020492c,
        block_0x00204980,
        block_0x00204994,
        block_0x002049a4,
        block_0x002049f8,
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
            start_word: 5u32,
            len: 1i32 as u16,
            fn_offset: 1usize as u16,
        },
        Run {
            start_word: 10u32,
            len: 1i32 as u16,
            fn_offset: 2usize as u16,
        },
        Run {
            start_word: 15u32,
            len: 1i32 as u16,
            fn_offset: 3usize as u16,
        },
        Run {
            start_word: 20u32,
            len: 1i32 as u16,
            fn_offset: 4usize as u16,
        },
        Run {
            start_word: 25u32,
            len: 1i32 as u16,
            fn_offset: 5usize as u16,
        },
        Run {
            start_word: 30u32,
            len: 1i32 as u16,
            fn_offset: 6usize as u16,
        },
        Run {
            start_word: 35u32,
            len: 1i32 as u16,
            fn_offset: 7usize as u16,
        },
        Run {
            start_word: 40u32,
            len: 1i32 as u16,
            fn_offset: 8usize as u16,
        },
        Run {
            start_word: 45u32,
            len: 1i32 as u16,
            fn_offset: 9usize as u16,
        },
        Run {
            start_word: 50u32,
            len: 1i32 as u16,
            fn_offset: 10usize as u16,
        },
        Run {
            start_word: 55u32,
            len: 1i32 as u16,
            fn_offset: 11usize as u16,
        },
        Run {
            start_word: 60u32,
            len: 1i32 as u16,
            fn_offset: 12usize as u16,
        },
        Run {
            start_word: 65u32,
            len: 1i32 as u16,
            fn_offset: 13usize as u16,
        },
        Run {
            start_word: 71u32,
            len: 1i32 as u16,
            fn_offset: 14usize as u16,
        },
        Run {
            start_word: 76u32,
            len: 1i32 as u16,
            fn_offset: 15usize as u16,
        },
        Run {
            start_word: 79u32,
            len: 1i32 as u16,
            fn_offset: 16usize as u16,
        },
        Run {
            start_word: 86u32,
            len: 1i32 as u16,
            fn_offset: 17usize as u16,
        },
        Run {
            start_word: 92u32,
            len: 1i32 as u16,
            fn_offset: 18usize as u16,
        },
        Run {
            start_word: 94u32,
            len: 1i32 as u16,
            fn_offset: 19usize as u16,
        },
        Run {
            start_word: 96u32,
            len: 1i32 as u16,
            fn_offset: 20usize as u16,
        },
        Run {
            start_word: 101u32,
            len: 1i32 as u16,
            fn_offset: 21usize as u16,
        },
        Run {
            start_word: 106u32,
            len: 2i32 as u16,
            fn_offset: 22usize as u16,
        },
        Run {
            start_word: 113u32,
            len: 1i32 as u16,
            fn_offset: 24usize as u16,
        },
        Run {
            start_word: 123u32,
            len: 1i32 as u16,
            fn_offset: 25usize as u16,
        },
        Run {
            start_word: 135u32,
            len: 1i32 as u16,
            fn_offset: 26usize as u16,
        },
        Run {
            start_word: 137u32,
            len: 1i32 as u16,
            fn_offset: 27usize as u16,
        },
        Run {
            start_word: 145u32,
            len: 1i32 as u16,
            fn_offset: 28usize as u16,
        },
        Run {
            start_word: 152u32,
            len: 1i32 as u16,
            fn_offset: 29usize as u16,
        },
        Run {
            start_word: 158u32,
            len: 1i32 as u16,
            fn_offset: 30usize as u16,
        },
        Run {
            start_word: 164u32,
            len: 1i32 as u16,
            fn_offset: 31usize as u16,
        },
        Run {
            start_word: 169u32,
            len: 1i32 as u16,
            fn_offset: 32usize as u16,
        },
        Run {
            start_word: 172u32,
            len: 1i32 as u16,
            fn_offset: 33usize as u16,
        },
        Run {
            start_word: 177u32,
            len: 1i32 as u16,
            fn_offset: 34usize as u16,
        },
        Run {
            start_word: 182u32,
            len: 1i32 as u16,
            fn_offset: 35usize as u16,
        },
        Run {
            start_word: 187u32,
            len: 2i32 as u16,
            fn_offset: 36usize as u16,
        },
        Run {
            start_word: 192u32,
            len: 1i32 as u16,
            fn_offset: 38usize as u16,
        },
        Run {
            start_word: 196u32,
            len: 1i32 as u16,
            fn_offset: 39usize as u16,
        },
        Run {
            start_word: 200u32,
            len: 1i32 as u16,
            fn_offset: 40usize as u16,
        },
        Run {
            start_word: 204u32,
            len: 1i32 as u16,
            fn_offset: 41usize as u16,
        },
        Run {
            start_word: 209u32,
            len: 1i32 as u16,
            fn_offset: 42usize as u16,
        },
        Run {
            start_word: 212u32,
            len: 1i32 as u16,
            fn_offset: 43usize as u16,
        },
        Run {
            start_word: 217u32,
            len: 1i32 as u16,
            fn_offset: 44usize as u16,
        },
        Run {
            start_word: 233u32,
            len: 1i32 as u16,
            fn_offset: 45usize as u16,
        },
        Run {
            start_word: 284u32,
            len: 1i32 as u16,
            fn_offset: 46usize as u16,
        },
        Run {
            start_word: 321u32,
            len: 1i32 as u16,
            fn_offset: 47usize as u16,
        },
        Run {
            start_word: 358u32,
            len: 1i32 as u16,
            fn_offset: 48usize as u16,
        },
        Run {
            start_word: 395u32,
            len: 1i32 as u16,
            fn_offset: 49usize as u16,
        },
        Run {
            start_word: 432u32,
            len: 1i32 as u16,
            fn_offset: 50usize as u16,
        },
        Run {
            start_word: 437u32,
            len: 1i32 as u16,
            fn_offset: 51usize as u16,
        },
        Run {
            start_word: 474u32,
            len: 1i32 as u16,
            fn_offset: 52usize as u16,
        },
        Run {
            start_word: 479u32,
            len: 1i32 as u16,
            fn_offset: 53usize as u16,
        },
        Run {
            start_word: 516u32,
            len: 1i32 as u16,
            fn_offset: 54usize as u16,
        },
        Run {
            start_word: 553u32,
            len: 1i32 as u16,
            fn_offset: 55usize as u16,
        },
        Run {
            start_word: 558u32,
            len: 1i32 as u16,
            fn_offset: 56usize as u16,
        },
        Run {
            start_word: 595u32,
            len: 1i32 as u16,
            fn_offset: 57usize as u16,
        },
        Run {
            start_word: 600u32,
            len: 1i32 as u16,
            fn_offset: 58usize as u16,
        },
        Run {
            start_word: 637u32,
            len: 1i32 as u16,
            fn_offset: 59usize as u16,
        },
        Run {
            start_word: 674u32,
            len: 1i32 as u16,
            fn_offset: 60usize as u16,
        },
        Run {
            start_word: 679u32,
            len: 1i32 as u16,
            fn_offset: 61usize as u16,
        },
        Run {
            start_word: 716u32,
            len: 1i32 as u16,
            fn_offset: 62usize as u16,
        },
        Run {
            start_word: 721u32,
            len: 1i32 as u16,
            fn_offset: 63usize as u16,
        },
        Run {
            start_word: 766u32,
            len: 1i32 as u16,
            fn_offset: 64usize as u16,
        },
        Run {
            start_word: 787u32,
            len: 1i32 as u16,
            fn_offset: 65usize as u16,
        },
        Run {
            start_word: 791u32,
            len: 1i32 as u16,
            fn_offset: 66usize as u16,
        },
        Run {
            start_word: 812u32,
            len: 1i32 as u16,
            fn_offset: 67usize as u16,
        },
        Run {
            start_word: 849u32,
            len: 1i32 as u16,
            fn_offset: 68usize as u16,
        },
        Run {
            start_word: 854u32,
            len: 1i32 as u16,
            fn_offset: 69usize as u16,
        },
        Run {
            start_word: 858u32,
            len: 1i32 as u16,
            fn_offset: 70usize as u16,
        },
        Run {
            start_word: 879u32,
            len: 1i32 as u16,
            fn_offset: 71usize as u16,
        },
        Run {
            start_word: 892u32,
            len: 1i32 as u16,
            fn_offset: 72usize as u16,
        },
        Run {
            start_word: 929u32,
            len: 1i32 as u16,
            fn_offset: 73usize as u16,
        },
        Run {
            start_word: 934u32,
            len: 1i32 as u16,
            fn_offset: 74usize as u16,
        },
        Run {
            start_word: 938u32,
            len: 1i32 as u16,
            fn_offset: 75usize as u16,
        },
        Run {
            start_word: 959u32,
            len: 1i32 as u16,
            fn_offset: 76usize as u16,
        },
        Run {
            start_word: 963u32,
            len: 1i32 as u16,
            fn_offset: 77usize as u16,
        },
        Run {
            start_word: 984u32,
            len: 1i32 as u16,
            fn_offset: 78usize as u16,
        },
        Run {
            start_word: 989u32,
            len: 1i32 as u16,
            fn_offset: 79usize as u16,
        },
        Run {
            start_word: 1026u32,
            len: 1i32 as u16,
            fn_offset: 80usize as u16,
        },
        Run {
            start_word: 1063u32,
            len: 1i32 as u16,
            fn_offset: 81usize as u16,
        },
        Run {
            start_word: 1068u32,
            len: 1i32 as u16,
            fn_offset: 82usize as u16,
        },
        Run {
            start_word: 1089u32,
            len: 1i32 as u16,
            fn_offset: 83usize as u16,
        },
        Run {
            start_word: 1110u32,
            len: 1i32 as u16,
            fn_offset: 84usize as u16,
        },
        Run {
            start_word: 1115u32,
            len: 1i32 as u16,
            fn_offset: 85usize as u16,
        },
        Run {
            start_word: 1120u32,
            len: 1i32 as u16,
            fn_offset: 86usize as u16,
        },
        Run {
            start_word: 1125u32,
            len: 1i32 as u16,
            fn_offset: 87usize as u16,
        },
        Run {
            start_word: 1130u32,
            len: 1i32 as u16,
            fn_offset: 88usize as u16,
        },
        Run {
            start_word: 1142u32,
            len: 1i32 as u16,
            fn_offset: 89usize as u16,
        },
        Run {
            start_word: 1159u32,
            len: 1i32 as u16,
            fn_offset: 90usize as u16,
        },
        Run {
            start_word: 1163u32,
            len: 1i32 as u16,
            fn_offset: 91usize as u16,
        },
        Run {
            start_word: 1167u32,
            len: 1i32 as u16,
            fn_offset: 92usize as u16,
        },
        Run {
            start_word: 1204u32,
            len: 1i32 as u16,
            fn_offset: 93usize as u16,
        },
        Run {
            start_word: 1208u32,
            len: 1i32 as u16,
            fn_offset: 94usize as u16,
        },
        Run {
            start_word: 1245u32,
            len: 1i32 as u16,
            fn_offset: 95usize as u16,
        },
        Run {
            start_word: 1249u32,
            len: 1i32 as u16,
            fn_offset: 96usize as u16,
        },
        Run {
            start_word: 1294u32,
            len: 1i32 as u16,
            fn_offset: 97usize as u16,
        },
        Run {
            start_word: 1315u32,
            len: 1i32 as u16,
            fn_offset: 98usize as u16,
        },
        Run {
            start_word: 1319u32,
            len: 1i32 as u16,
            fn_offset: 99usize as u16,
        },
        Run {
            start_word: 1340u32,
            len: 1i32 as u16,
            fn_offset: 100usize as u16,
        },
        Run {
            start_word: 1377u32,
            len: 1i32 as u16,
            fn_offset: 101usize as u16,
        },
        Run {
            start_word: 1398u32,
            len: 1i32 as u16,
            fn_offset: 102usize as u16,
        },
        Run {
            start_word: 1419u32,
            len: 1i32 as u16,
            fn_offset: 103usize as u16,
        },
        Run {
            start_word: 1424u32,
            len: 1i32 as u16,
            fn_offset: 104usize as u16,
        },
        Run {
            start_word: 1428u32,
            len: 1i32 as u16,
            fn_offset: 105usize as u16,
        },
        Run {
            start_word: 1449u32,
            len: 1i32 as u16,
            fn_offset: 106usize as u16,
        },
    ];
    if pc < 2110292u32 || pc > 2116088u32 {
        return None;
    }
    let word_offset = ((pc - 2110292u32) >> 2) as u32;
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
pub fn block_0x00203354(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 300u32, 2110296u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2110300u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2110304u32);
    emu.apc_no_count(1usize, 2110304u32, 24576u32, 2110308u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110312u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1284u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203368(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 396u32, 2110316u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2110320u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2110324u32);
    emu.apc_no_count(1usize, 2110324u32, 24576u32, 2110328u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110332u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1264u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020337c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 492u32, 2110336u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2110340u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2110344u32);
    emu.apc_no_count(1usize, 2110344u32, 24576u32, 2110348u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110352u32;
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
#[inline(always)]
pub fn block_0x00203390(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 588u32, 2110356u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2110360u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2110364u32);
    emu.apc_no_count(1usize, 2110364u32, 24576u32, 2110368u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110372u32;
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
#[inline(always)]
pub fn block_0x002033a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 684u32, 2110376u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2110380u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2110384u32);
    emu.apc_no_count(1usize, 2110384u32, 24576u32, 2110388u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110392u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1204u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002033b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 780u32, 2110396u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2110400u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2110404u32);
    emu.apc_no_count(1usize, 2110404u32, 24576u32, 2110408u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110412u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1184u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002033cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 876u32, 2110416u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2110420u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2110424u32);
    emu.apc_no_count(1usize, 2110424u32, 24576u32, 2110428u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110432u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1164u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002033e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 972u32, 2110436u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2110440u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2110444u32);
    emu.apc_no_count(1usize, 2110444u32, 24576u32, 2110448u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110452u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1144u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002033f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1068u32, 2110456u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2110460u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2110464u32);
    emu.apc_no_count(1usize, 2110464u32, 24576u32, 2110468u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110472u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1124u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203408(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1164u32, 2110476u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2110480u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2110484u32);
    emu.apc_no_count(1usize, 2110484u32, 24576u32, 2110488u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110492u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1104u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020341c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1260u32, 2110496u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2110500u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2110504u32);
    emu.apc_no_count(1usize, 2110504u32, 24576u32, 2110508u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110512u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1084u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203430(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1356u32, 2110516u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2110520u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2110524u32);
    emu.apc_no_count(1usize, 2110524u32, 24576u32, 2110528u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110532u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1064u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203444(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1452u32, 2110536u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2110540u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2110544u32);
    emu.apc_no_count(1usize, 2110544u32, 24576u32, 2110548u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110552u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1044u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203458(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2110556u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2110560u32);
    emu.adi_no_count(8usize, 2usize, 12u32, 2110564u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2110568u32);
    emu.apc_no_count(1usize, 2110568u32, 24576u32, 2110572u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110576u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1020u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203470(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 96u32, 2110580u32);
    emu.adi_no_count(10usize, 20usize, 0u32, 2110584u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2110588u32);
    emu.apc_no_count(1usize, 2110588u32, 24576u32, 2110592u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110596u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1000u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203484(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 2u32, 2110600u32);
    emu.adi_no_count(20usize, 0usize, 16u32, 2110604u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2110608u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2110668u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002034cc));
}
#[inline(always)]
pub fn block_0x00203490(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 9usize, 4u32, 2110612u32);
    emu.sli_no_count(11usize, 9usize, 6u32, 2110616u32);
    emu.sbr_no_count(11usize, 11usize, 10usize, 2110620u32);
    emu.adr_no_count(11usize, 8usize, 11usize, 2110624u32);
    emu.adi_no_count(10usize, 2usize, 1932u32, 2110628u32);
    emu.apc_no_count(1usize, 2110628u32, 4096u32, 2110632u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110636u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(136u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002034ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 9usize, 1u32, 2110640u32);
    emu.adi_no_count(11usize, 2usize, 1932u32, 2110644u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2110648u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2110652u32);
    emu.apc_no_count(1usize, 2110652u32, 24576u32, 2110656u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110660u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(936u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002034c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 19usize, 96u32, 2110664u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2110720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203500));
    } else {
        emu.pc = 2110668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002034cc));
    }
}
#[inline(always)]
pub fn block_0x002034cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 9usize, 1u32, 2110672u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2110608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203490));
    } else {
        emu.pc = 2110676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002034d4));
    }
}
#[inline(always)]
pub fn block_0x002034d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 19usize, 4294967200u32, 2110680u32);
    emu.adi_no_count(10usize, 2usize, 2028u32, 2110684u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2110688u32);
    emu.apc_no_count(1usize, 2110688u32, 24576u32, 2110692u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110696u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(900u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002034e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1932u32, 2110700u32);
    emu.adi_no_count(11usize, 2usize, 2028u32, 2110704u32);
    emu.adi_no_count(12usize, 18usize, 0u32, 2110708u32);
    emu.apc_no_count(1usize, 2110708u32, 0u32, 2110712u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110716u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(516u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002034fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2110720u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2110636u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002034ac));
}
#[inline(always)]
pub fn block_0x00203500(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2110724u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966680u32, 2110728u32);
    emu.adi_no_count(10usize, 2usize, 1548u32, 2110732u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2110736u32);
    emu.apc_no_count(1usize, 2110736u32, 24576u32, 2110740u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110744u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(852u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00203518(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 2usize, 1676u32, 2110748u32);
    emu.adi_no_count(20usize, 2usize, 1708u32, 2110752u32);
    emu.adi_no_count(21usize, 2usize, 2047u32, 2110756u32);
    emu.adi_no_count(21usize, 21usize, 13u32, 2110760u32);
    emu.adi_no_count(22usize, 2usize, 2047u32, 2110764u32);
    emu.adi_no_count(22usize, 22usize, 45u32, 2110768u32);
    emu.adi_no_count(26usize, 0usize, 252u32, 2110772u32);
    emu.adi_no_count(10usize, 0usize, 31u32, 2110776u32);
    emu.adi_no_count(27usize, 2usize, 172u32, 2110780u32);
    emu.adi_no_count(8usize, 0usize, 16u32, 2110784u32);
    emu.add_memory_rw_events(10usize);
    emu.pc = 2110784u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203540));
}
#[inline]
pub fn block_0x00203540(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 8u32, 2110788u32)?;
    emu.adr_no_count(10usize, 11usize, 10usize, 2110792u32);
    emu.lbu_no_count(10usize, 10usize, 0u32, 2110796u32);
    emu.ani_no_count(11usize, 26usize, 4u32, 2110800u32);
    emu.srr_no_count(9usize, 10usize, 11usize, 2110804u32);
    emu.ani_no_count(9usize, 9usize, 15u32, 2110808u32);
    emu.adi_no_count(10usize, 2usize, 1644u32, 2110812u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2110816u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2110820u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966680u32, 2110824u32);
    emu.apc_no_count(1usize, 2110824u32, 24576u32, 2110828u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110832u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(764u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203570(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2110836u32);
    emu.adi_no_count(23usize, 27usize, 0u32, 2110840u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2110840u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203578));
}
#[inline(always)]
pub fn block_0x00203578(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 1u32, 2110844u32);
    emu.adi_no_count(24usize, 23usize, 4294967232u32, 2110848u32);
    emu.xrr_no_count(10usize, 10usize, 9usize, 2110852u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2110856u32);
    emu.sli_no_count(10usize, 10usize, 23u32, 2110860u32);
    emu.sri_no_count(10usize, 10usize, 31u32, 2110864u32);
    emu.apc_no_count(1usize, 2110864u32, 81920u32, 2110868u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110872u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967272u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203598(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(25usize, 10usize, 255u32, 2110876u32);
    emu.adi_no_count(10usize, 2usize, 2028u32, 2110880u32);
    emu.adi_no_count(11usize, 2usize, 1644u32, 2110884u32);
    emu.adi_no_count(12usize, 24usize, 0u32, 2110888u32);
    emu.adi_no_count(13usize, 25usize, 0u32, 2110892u32);
    emu.apc_no_count(1usize, 2110892u32, 57344u32, 2110896u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110900u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(768u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002035b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 23usize, 4294967264u32, 2110904u32);
    emu.adi_no_count(10usize, 21usize, 0u32, 2110908u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2110912u32);
    emu.adi_no_count(13usize, 25usize, 0u32, 2110916u32);
    emu.apc_no_count(1usize, 2110916u32, 57344u32, 2110920u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110924u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(744u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002035cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 22usize, 0u32, 2110928u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2110932u32);
    emu.adi_no_count(12usize, 23usize, 0u32, 2110936u32);
    emu.adi_no_count(13usize, 25usize, 0u32, 2110940u32);
    emu.apc_no_count(1usize, 2110940u32, 57344u32, 2110944u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110948u32;
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
#[inline(always)]
pub fn block_0x002035e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1644u32, 2110952u32);
    emu.adi_no_count(11usize, 2usize, 2028u32, 2110956u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2110960u32);
    emu.apc_no_count(1usize, 2110960u32, 24576u32, 2110964u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110968u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(628u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002035f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(23usize, 23usize, 96u32, 2110972u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2110976u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2110840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203578));
    } else {
        emu.pc = 2110980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203604));
    }
}
#[inline(always)]
pub fn block_0x00203604(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 2028u32, 2110984u32);
    emu.adi_no_count(11usize, 2usize, 1548u32, 2110988u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2110992u32);
    emu.apc_no_count(1usize, 2110992u32, 24576u32, 2110996u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111000u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(596u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203618(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1932u32, 2111004u32);
    emu.adi_no_count(11usize, 2usize, 2028u32, 2111008u32);
    emu.adi_no_count(12usize, 2usize, 1644u32, 2111012u32);
    emu.apc_no_count(1usize, 2111012u32, 0u32, 2111016u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111020u32;
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
#[inline(always)]
pub fn block_0x0020362c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1548u32, 2111024u32);
    emu.adi_no_count(11usize, 2usize, 1932u32, 2111028u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2111032u32);
    emu.apc_no_count(1usize, 2111032u32, 24576u32, 2111036u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111040u32;
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
#[inline(always)]
pub fn block_0x00203640(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a == b {
        emu.pc = 2111140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002036a4));
    } else {
        emu.pc = 2111044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203644));
    }
}
#[inline(always)]
pub fn block_0x00203644(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 2028u32, 2111048u32);
    emu.adi_no_count(11usize, 2usize, 1548u32, 2111052u32);
    emu.apc_no_count(1usize, 2111052u32, 4096u32, 2111056u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111060u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967008u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203654(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1932u32, 2111064u32);
    emu.adi_no_count(11usize, 2usize, 2028u32, 2111068u32);
    emu.apc_no_count(1usize, 2111068u32, 4096u32, 2111072u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111076u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966992u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203664(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1836u32, 2111080u32);
    emu.adi_no_count(11usize, 2usize, 1932u32, 2111084u32);
    emu.apc_no_count(1usize, 2111084u32, 4096u32, 2111088u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111092u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966976u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203674(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1740u32, 2111096u32);
    emu.adi_no_count(11usize, 2usize, 1836u32, 2111100u32);
    emu.apc_no_count(1usize, 2111100u32, 4096u32, 2111104u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111108u32;
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
#[inline(always)]
pub fn block_0x00203684(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1548u32, 2111112u32);
    emu.adi_no_count(11usize, 2usize, 1740u32, 2111116u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2111120u32);
    emu.apc_no_count(1usize, 2111120u32, 24576u32, 2111124u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111128u32;
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
#[inline(always)]
pub fn block_0x00203698(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 26usize, 4294967292u32, 2111132u32);
    emu.sri_no_count(10usize, 26usize, 3u32, 2111136u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2111140u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2110784u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203540));
}
#[inline(always)]
pub fn block_0x002036a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 2usize, 1548u32, 2111144u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2111148u32);
    emu.lw_no_count(10usize, 2usize, 4u32, 2111152u32)?;
    emu.apc_no_count(1usize, 2111152u32, 24576u32, 2111156u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111160u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(436u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002036b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 144u32, 2111164u32);
    emu.lw_no_count(1usize, 2usize, 2028u32, 2111168u32)?;
    emu.lw_no_count(8usize, 2usize, 2024u32, 2111172u32)?;
    emu.lw_no_count(9usize, 2usize, 2020u32, 2111176u32)?;
    emu.lw_no_count(18usize, 2usize, 2016u32, 2111180u32)?;
    emu.lw_no_count(19usize, 2usize, 2012u32, 2111184u32)?;
    emu.lw_no_count(20usize, 2usize, 2008u32, 2111188u32)?;
    emu.lw_no_count(21usize, 2usize, 2004u32, 2111192u32)?;
    emu.lw_no_count(22usize, 2usize, 2000u32, 2111196u32)?;
    emu.lw_no_count(23usize, 2usize, 1996u32, 2111200u32)?;
    emu.lw_no_count(24usize, 2usize, 1992u32, 2111204u32)?;
    emu.lw_no_count(25usize, 2usize, 1988u32, 2111208u32)?;
    emu.lw_no_count(26usize, 2usize, 1984u32, 2111212u32)?;
    emu.lw_no_count(27usize, 2usize, 1980u32, 2111216u32)?;
    emu.adi_no_count(2usize, 2usize, 2032u32, 2111220u32);
    emu.add_memory_rw_events(16usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111224u32;
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
pub fn block_0x002036f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 51u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294966672u32, 2111228u32);
    emu.sw_no_count(1usize, 2usize, 620u32, 2111232u32)?;
    emu.sw_no_count(8usize, 2usize, 616u32, 2111236u32)?;
    emu.sw_no_count(9usize, 2usize, 612u32, 2111240u32)?;
    emu.sw_no_count(18usize, 2usize, 608u32, 2111244u32)?;
    emu.sw_no_count(19usize, 2usize, 604u32, 2111248u32)?;
    emu.sw_no_count(20usize, 2usize, 600u32, 2111252u32)?;
    emu.sw_no_count(21usize, 2usize, 596u32, 2111256u32)?;
    emu.sw_no_count(22usize, 2usize, 592u32, 2111260u32)?;
    emu.sw_no_count(23usize, 2usize, 588u32, 2111264u32)?;
    emu.sw_no_count(24usize, 2usize, 584u32, 2111268u32)?;
    emu.adi_no_count(9usize, 12usize, 0u32, 2111272u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2111276u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2111280u32);
    emu.lw_no_count(10usize, 11usize, 16u32, 2111284u32)?;
    emu.lw_no_count(11usize, 11usize, 20u32, 2111288u32)?;
    emu.lw_no_count(12usize, 18usize, 24u32, 2111292u32)?;
    emu.lw_no_count(13usize, 18usize, 28u32, 2111296u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2111300u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2111304u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2111308u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2111312u32)?;
    emu.lw_no_count(10usize, 18usize, 0u32, 2111316u32)?;
    emu.lw_no_count(11usize, 18usize, 4u32, 2111320u32)?;
    emu.lw_no_count(12usize, 18usize, 8u32, 2111324u32)?;
    emu.lw_no_count(13usize, 18usize, 12u32, 2111328u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2111332u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2111336u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2111340u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2111344u32)?;
    emu.lw_no_count(10usize, 9usize, 16u32, 2111348u32)?;
    emu.lw_no_count(11usize, 9usize, 20u32, 2111352u32)?;
    emu.lw_no_count(12usize, 9usize, 24u32, 2111356u32)?;
    emu.lw_no_count(13usize, 9usize, 28u32, 2111360u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2111364u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2111368u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2111372u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2111376u32)?;
    emu.lw_no_count(10usize, 9usize, 0u32, 2111380u32)?;
    emu.lw_no_count(11usize, 9usize, 4u32, 2111384u32)?;
    emu.lw_no_count(12usize, 9usize, 8u32, 2111388u32)?;
    emu.lw_no_count(13usize, 9usize, 12u32, 2111392u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2111396u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2111400u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2111404u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2111408u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2111412u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2111416u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2111420u32);
    emu.apc_no_count(1usize, 2111420u32, 28672u32, 2111424u32);
    emu.add_memory_rw_events(51usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111428u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967192u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x002037c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 48u32, 2111432u32)?;
    emu.lw_no_count(11usize, 18usize, 52u32, 2111436u32)?;
    emu.lw_no_count(12usize, 18usize, 56u32, 2111440u32)?;
    emu.lw_no_count(13usize, 18usize, 60u32, 2111444u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2111448u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2111452u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2111456u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2111460u32)?;
    emu.lw_no_count(10usize, 18usize, 32u32, 2111464u32)?;
    emu.lw_no_count(11usize, 18usize, 36u32, 2111468u32)?;
    emu.lw_no_count(12usize, 18usize, 40u32, 2111472u32)?;
    emu.lw_no_count(13usize, 18usize, 44u32, 2111476u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2111480u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2111484u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2111488u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2111492u32)?;
    emu.lw_no_count(10usize, 9usize, 48u32, 2111496u32)?;
    emu.lw_no_count(11usize, 9usize, 52u32, 2111500u32)?;
    emu.lw_no_count(12usize, 9usize, 56u32, 2111504u32)?;
    emu.lw_no_count(13usize, 9usize, 60u32, 2111508u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2111512u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2111516u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2111520u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2111524u32)?;
    emu.lw_no_count(10usize, 9usize, 32u32, 2111528u32)?;
    emu.lw_no_count(11usize, 9usize, 36u32, 2111532u32)?;
    emu.lw_no_count(12usize, 9usize, 40u32, 2111536u32)?;
    emu.lw_no_count(13usize, 9usize, 44u32, 2111540u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2111544u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2111548u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2111552u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2111556u32)?;
    emu.adi_no_count(10usize, 2usize, 40u32, 2111560u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2111564u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2111568u32);
    emu.apc_no_count(1usize, 2111568u32, 28672u32, 2111572u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111576u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967044u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00203858(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 80u32, 2111580u32)?;
    emu.lw_no_count(11usize, 18usize, 84u32, 2111584u32)?;
    emu.lw_no_count(12usize, 18usize, 88u32, 2111588u32)?;
    emu.lw_no_count(13usize, 18usize, 92u32, 2111592u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2111596u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2111600u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2111604u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2111608u32)?;
    emu.lw_no_count(10usize, 18usize, 64u32, 2111612u32)?;
    emu.lw_no_count(11usize, 18usize, 68u32, 2111616u32)?;
    emu.lw_no_count(12usize, 18usize, 72u32, 2111620u32)?;
    emu.lw_no_count(13usize, 18usize, 76u32, 2111624u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2111628u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2111632u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2111636u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2111640u32)?;
    emu.lw_no_count(10usize, 9usize, 80u32, 2111644u32)?;
    emu.lw_no_count(11usize, 9usize, 84u32, 2111648u32)?;
    emu.lw_no_count(12usize, 9usize, 88u32, 2111652u32)?;
    emu.lw_no_count(13usize, 9usize, 92u32, 2111656u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2111660u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2111664u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2111668u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2111672u32)?;
    emu.lw_no_count(10usize, 9usize, 64u32, 2111676u32)?;
    emu.lw_no_count(11usize, 9usize, 68u32, 2111680u32)?;
    emu.lw_no_count(12usize, 9usize, 72u32, 2111684u32)?;
    emu.lw_no_count(13usize, 9usize, 76u32, 2111688u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2111692u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2111696u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2111700u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2111704u32)?;
    emu.adi_no_count(10usize, 2usize, 72u32, 2111708u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2111712u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2111716u32);
    emu.apc_no_count(1usize, 2111716u32, 28672u32, 2111720u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111724u32;
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
#[inline(never)]
pub fn block_0x002038ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 16u32, 2111728u32)?;
    emu.lw_no_count(11usize, 18usize, 20u32, 2111732u32)?;
    emu.lw_no_count(12usize, 18usize, 24u32, 2111736u32)?;
    emu.lw_no_count(13usize, 18usize, 28u32, 2111740u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2111744u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2111748u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2111752u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2111756u32)?;
    emu.lw_no_count(10usize, 18usize, 0u32, 2111760u32)?;
    emu.lw_no_count(11usize, 18usize, 4u32, 2111764u32)?;
    emu.lw_no_count(12usize, 18usize, 8u32, 2111768u32)?;
    emu.lw_no_count(13usize, 18usize, 12u32, 2111772u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2111776u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2111780u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2111784u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2111788u32)?;
    emu.lw_no_count(10usize, 18usize, 48u32, 2111792u32)?;
    emu.lw_no_count(11usize, 18usize, 52u32, 2111796u32)?;
    emu.lw_no_count(12usize, 18usize, 56u32, 2111800u32)?;
    emu.lw_no_count(13usize, 18usize, 60u32, 2111804u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2111808u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2111812u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2111816u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2111820u32)?;
    emu.lw_no_count(10usize, 18usize, 32u32, 2111824u32)?;
    emu.lw_no_count(11usize, 18usize, 36u32, 2111828u32)?;
    emu.lw_no_count(12usize, 18usize, 40u32, 2111832u32)?;
    emu.lw_no_count(13usize, 18usize, 44u32, 2111836u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2111840u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2111844u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2111848u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2111852u32)?;
    emu.adi_no_count(10usize, 2usize, 456u32, 2111856u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2111860u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2111864u32);
    emu.apc_no_count(1usize, 2111864u32, 28672u32, 2111868u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111872u32;
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
#[inline(never)]
pub fn block_0x00203980(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 16u32, 2111876u32)?;
    emu.lw_no_count(11usize, 9usize, 20u32, 2111880u32)?;
    emu.lw_no_count(12usize, 9usize, 24u32, 2111884u32)?;
    emu.lw_no_count(13usize, 9usize, 28u32, 2111888u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2111892u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2111896u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2111900u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2111904u32)?;
    emu.lw_no_count(10usize, 9usize, 0u32, 2111908u32)?;
    emu.lw_no_count(11usize, 9usize, 4u32, 2111912u32)?;
    emu.lw_no_count(12usize, 9usize, 8u32, 2111916u32)?;
    emu.lw_no_count(13usize, 9usize, 12u32, 2111920u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2111924u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2111928u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2111932u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2111936u32)?;
    emu.lw_no_count(10usize, 9usize, 48u32, 2111940u32)?;
    emu.lw_no_count(11usize, 9usize, 52u32, 2111944u32)?;
    emu.lw_no_count(12usize, 9usize, 56u32, 2111948u32)?;
    emu.lw_no_count(13usize, 9usize, 60u32, 2111952u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2111956u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2111960u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2111964u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2111968u32)?;
    emu.lw_no_count(10usize, 9usize, 32u32, 2111972u32)?;
    emu.lw_no_count(11usize, 9usize, 36u32, 2111976u32)?;
    emu.lw_no_count(12usize, 9usize, 40u32, 2111980u32)?;
    emu.lw_no_count(13usize, 9usize, 44u32, 2111984u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2111988u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2111992u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2111996u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2112000u32)?;
    emu.adi_no_count(10usize, 2usize, 488u32, 2112004u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2112008u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2112012u32);
    emu.apc_no_count(1usize, 2112012u32, 28672u32, 2112016u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112020u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965308u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203a14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 360u32, 2112024u32);
    emu.adi_no_count(11usize, 2usize, 456u32, 2112028u32);
    emu.adi_no_count(12usize, 2usize, 488u32, 2112032u32);
    emu.apc_no_count(1usize, 2112032u32, 28672u32, 2112036u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112040u32;
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
#[inline(never)]
pub fn block_0x00203a28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 24u32, 2112044u32)?;
    emu.lw_no_count(11usize, 2usize, 28u32, 2112048u32)?;
    emu.lw_no_count(12usize, 2usize, 32u32, 2112052u32)?;
    emu.lw_no_count(13usize, 2usize, 36u32, 2112056u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2112060u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2112064u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2112068u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2112072u32)?;
    emu.lw_no_count(10usize, 2usize, 8u32, 2112076u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2112080u32)?;
    emu.lw_no_count(12usize, 2usize, 16u32, 2112084u32)?;
    emu.lw_no_count(13usize, 2usize, 20u32, 2112088u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2112092u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2112096u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2112100u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2112104u32)?;
    emu.lw_no_count(10usize, 2usize, 56u32, 2112108u32)?;
    emu.lw_no_count(11usize, 2usize, 60u32, 2112112u32)?;
    emu.lw_no_count(12usize, 2usize, 64u32, 2112116u32)?;
    emu.lw_no_count(13usize, 2usize, 68u32, 2112120u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2112124u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2112128u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2112132u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2112136u32)?;
    emu.lw_no_count(10usize, 2usize, 40u32, 2112140u32)?;
    emu.lw_no_count(11usize, 2usize, 44u32, 2112144u32)?;
    emu.lw_no_count(12usize, 2usize, 48u32, 2112148u32)?;
    emu.lw_no_count(13usize, 2usize, 52u32, 2112152u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2112156u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2112160u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2112164u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2112168u32)?;
    emu.adi_no_count(10usize, 2usize, 488u32, 2112172u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2112176u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2112180u32);
    emu.apc_no_count(1usize, 2112180u32, 24576u32, 2112184u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112188u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1940u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203abc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 104u32, 2112192u32);
    emu.adi_no_count(11usize, 2usize, 360u32, 2112196u32);
    emu.adi_no_count(12usize, 2usize, 488u32, 2112200u32);
    emu.apc_no_count(1usize, 2112200u32, 28672u32, 2112204u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112208u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965844u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00203ad0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 48u32, 2112212u32)?;
    emu.lw_no_count(11usize, 18usize, 52u32, 2112216u32)?;
    emu.lw_no_count(12usize, 18usize, 56u32, 2112220u32)?;
    emu.lw_no_count(13usize, 18usize, 60u32, 2112224u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2112228u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2112232u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2112236u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2112240u32)?;
    emu.lw_no_count(10usize, 18usize, 32u32, 2112244u32)?;
    emu.lw_no_count(11usize, 18usize, 36u32, 2112248u32)?;
    emu.lw_no_count(12usize, 18usize, 40u32, 2112252u32)?;
    emu.lw_no_count(13usize, 18usize, 44u32, 2112256u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2112260u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2112264u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2112268u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2112272u32)?;
    emu.lw_no_count(10usize, 18usize, 80u32, 2112276u32)?;
    emu.lw_no_count(11usize, 18usize, 84u32, 2112280u32)?;
    emu.lw_no_count(12usize, 18usize, 88u32, 2112284u32)?;
    emu.lw_no_count(13usize, 18usize, 92u32, 2112288u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2112292u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2112296u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2112300u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2112304u32)?;
    emu.lw_no_count(10usize, 18usize, 64u32, 2112308u32)?;
    emu.lw_no_count(11usize, 18usize, 68u32, 2112312u32)?;
    emu.lw_no_count(12usize, 18usize, 72u32, 2112316u32)?;
    emu.lw_no_count(13usize, 18usize, 76u32, 2112320u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2112324u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2112328u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2112332u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2112336u32)?;
    emu.adi_no_count(10usize, 2usize, 456u32, 2112340u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2112344u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2112348u32);
    emu.apc_no_count(1usize, 2112348u32, 24576u32, 2112352u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112356u32;
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
pub fn block_0x00203b64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 48u32, 2112360u32)?;
    emu.lw_no_count(11usize, 9usize, 52u32, 2112364u32)?;
    emu.lw_no_count(12usize, 9usize, 56u32, 2112368u32)?;
    emu.lw_no_count(13usize, 9usize, 60u32, 2112372u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2112376u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2112380u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2112384u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2112388u32)?;
    emu.lw_no_count(10usize, 9usize, 32u32, 2112392u32)?;
    emu.lw_no_count(11usize, 9usize, 36u32, 2112396u32)?;
    emu.lw_no_count(12usize, 9usize, 40u32, 2112400u32)?;
    emu.lw_no_count(13usize, 9usize, 44u32, 2112404u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2112408u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2112412u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2112416u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2112420u32)?;
    emu.lw_no_count(10usize, 9usize, 80u32, 2112424u32)?;
    emu.lw_no_count(11usize, 9usize, 84u32, 2112428u32)?;
    emu.lw_no_count(12usize, 9usize, 88u32, 2112432u32)?;
    emu.lw_no_count(13usize, 9usize, 92u32, 2112436u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2112440u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2112444u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2112448u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2112452u32)?;
    emu.lw_no_count(10usize, 9usize, 64u32, 2112456u32)?;
    emu.lw_no_count(11usize, 9usize, 68u32, 2112460u32)?;
    emu.lw_no_count(12usize, 9usize, 72u32, 2112464u32)?;
    emu.lw_no_count(13usize, 9usize, 76u32, 2112468u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2112472u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2112476u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2112480u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2112484u32)?;
    emu.adi_no_count(10usize, 2usize, 488u32, 2112488u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2112492u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2112496u32);
    emu.apc_no_count(1usize, 2112496u32, 24576u32, 2112500u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112504u32;
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
pub fn block_0x00203bf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 360u32, 2112508u32);
    emu.adi_no_count(11usize, 2usize, 456u32, 2112512u32);
    emu.adi_no_count(12usize, 2usize, 488u32, 2112516u32);
    emu.apc_no_count(1usize, 2112516u32, 28672u32, 2112520u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112524u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966096u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00203c0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 88u32, 2112528u32)?;
    emu.lw_no_count(11usize, 2usize, 92u32, 2112532u32)?;
    emu.lw_no_count(12usize, 2usize, 96u32, 2112536u32)?;
    emu.lw_no_count(13usize, 2usize, 100u32, 2112540u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2112544u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2112548u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2112552u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2112556u32)?;
    emu.lw_no_count(10usize, 2usize, 72u32, 2112560u32)?;
    emu.lw_no_count(11usize, 2usize, 76u32, 2112564u32)?;
    emu.lw_no_count(12usize, 2usize, 80u32, 2112568u32)?;
    emu.lw_no_count(13usize, 2usize, 84u32, 2112572u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2112576u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2112580u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2112584u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2112588u32)?;
    emu.lw_no_count(10usize, 2usize, 56u32, 2112592u32)?;
    emu.lw_no_count(11usize, 2usize, 60u32, 2112596u32)?;
    emu.lw_no_count(12usize, 2usize, 64u32, 2112600u32)?;
    emu.lw_no_count(13usize, 2usize, 68u32, 2112604u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2112608u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2112612u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2112616u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2112620u32)?;
    emu.lw_no_count(10usize, 2usize, 40u32, 2112624u32)?;
    emu.lw_no_count(11usize, 2usize, 44u32, 2112628u32)?;
    emu.lw_no_count(12usize, 2usize, 48u32, 2112632u32)?;
    emu.lw_no_count(13usize, 2usize, 52u32, 2112636u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2112640u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2112644u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2112648u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2112652u32)?;
    emu.adi_no_count(10usize, 2usize, 488u32, 2112656u32);
    emu.adi_no_count(11usize, 2usize, 552u32, 2112660u32);
    emu.adi_no_count(12usize, 2usize, 520u32, 2112664u32);
    emu.apc_no_count(1usize, 2112664u32, 24576u32, 2112668u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112672u32;
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
#[inline(always)]
pub fn block_0x00203ca0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 136u32, 2112676u32);
    emu.adi_no_count(11usize, 2usize, 360u32, 2112680u32);
    emu.adi_no_count(12usize, 2usize, 488u32, 2112684u32);
    emu.apc_no_count(1usize, 2112684u32, 28672u32, 2112688u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112692u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965360u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00203cb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 16u32, 2112696u32)?;
    emu.lw_no_count(11usize, 18usize, 20u32, 2112700u32)?;
    emu.lw_no_count(12usize, 18usize, 24u32, 2112704u32)?;
    emu.lw_no_count(13usize, 18usize, 28u32, 2112708u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2112712u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2112716u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2112720u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2112724u32)?;
    emu.lw_no_count(10usize, 18usize, 0u32, 2112728u32)?;
    emu.lw_no_count(11usize, 18usize, 4u32, 2112732u32)?;
    emu.lw_no_count(12usize, 18usize, 8u32, 2112736u32)?;
    emu.lw_no_count(13usize, 18usize, 12u32, 2112740u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2112744u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2112748u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2112752u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2112756u32)?;
    emu.lw_no_count(10usize, 18usize, 80u32, 2112760u32)?;
    emu.lw_no_count(11usize, 18usize, 84u32, 2112764u32)?;
    emu.lw_no_count(12usize, 18usize, 88u32, 2112768u32)?;
    emu.lw_no_count(13usize, 18usize, 92u32, 2112772u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2112776u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2112780u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2112784u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2112788u32)?;
    emu.lw_no_count(10usize, 18usize, 64u32, 2112792u32)?;
    emu.lw_no_count(11usize, 18usize, 68u32, 2112796u32)?;
    emu.lw_no_count(12usize, 18usize, 72u32, 2112800u32)?;
    emu.lw_no_count(13usize, 18usize, 76u32, 2112804u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2112808u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2112812u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2112816u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2112820u32)?;
    emu.adi_no_count(10usize, 2usize, 456u32, 2112824u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2112828u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2112832u32);
    emu.apc_no_count(1usize, 2112832u32, 24576u32, 2112836u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112840u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1288u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00203d48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 16u32, 2112844u32)?;
    emu.lw_no_count(11usize, 9usize, 20u32, 2112848u32)?;
    emu.lw_no_count(12usize, 9usize, 24u32, 2112852u32)?;
    emu.lw_no_count(13usize, 9usize, 28u32, 2112856u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2112860u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2112864u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2112868u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2112872u32)?;
    emu.lw_no_count(10usize, 9usize, 0u32, 2112876u32)?;
    emu.lw_no_count(11usize, 9usize, 4u32, 2112880u32)?;
    emu.lw_no_count(12usize, 9usize, 8u32, 2112884u32)?;
    emu.lw_no_count(13usize, 9usize, 12u32, 2112888u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2112892u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2112896u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2112900u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2112904u32)?;
    emu.lw_no_count(10usize, 9usize, 80u32, 2112908u32)?;
    emu.lw_no_count(11usize, 9usize, 84u32, 2112912u32)?;
    emu.lw_no_count(12usize, 9usize, 88u32, 2112916u32)?;
    emu.lw_no_count(13usize, 9usize, 92u32, 2112920u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2112924u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2112928u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2112932u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2112936u32)?;
    emu.lw_no_count(10usize, 9usize, 64u32, 2112940u32)?;
    emu.lw_no_count(11usize, 9usize, 68u32, 2112944u32)?;
    emu.lw_no_count(12usize, 9usize, 72u32, 2112948u32)?;
    emu.lw_no_count(13usize, 9usize, 76u32, 2112952u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2112956u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2112960u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2112964u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2112968u32)?;
    emu.adi_no_count(10usize, 2usize, 488u32, 2112972u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2112976u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2112980u32);
    emu.apc_no_count(1usize, 2112980u32, 24576u32, 2112984u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112988u32;
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
#[inline(always)]
pub fn block_0x00203ddc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 360u32, 2112992u32);
    emu.adi_no_count(11usize, 2usize, 456u32, 2112996u32);
    emu.adi_no_count(12usize, 2usize, 488u32, 2113000u32);
    emu.apc_no_count(1usize, 2113000u32, 28672u32, 2113004u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113008u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965612u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00203df0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 24u32, 2113012u32)?;
    emu.lw_no_count(11usize, 2usize, 28u32, 2113016u32)?;
    emu.lw_no_count(12usize, 2usize, 32u32, 2113020u32)?;
    emu.lw_no_count(13usize, 2usize, 36u32, 2113024u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2113028u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2113032u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2113036u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2113040u32)?;
    emu.lw_no_count(10usize, 2usize, 8u32, 2113044u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2113048u32)?;
    emu.lw_no_count(12usize, 2usize, 16u32, 2113052u32)?;
    emu.lw_no_count(13usize, 2usize, 20u32, 2113056u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2113060u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2113064u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2113068u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2113072u32)?;
    emu.lw_no_count(10usize, 2usize, 88u32, 2113076u32)?;
    emu.lw_no_count(11usize, 2usize, 92u32, 2113080u32)?;
    emu.lw_no_count(12usize, 2usize, 96u32, 2113084u32)?;
    emu.lw_no_count(13usize, 2usize, 100u32, 2113088u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2113092u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2113096u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2113100u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2113104u32)?;
    emu.lw_no_count(10usize, 2usize, 72u32, 2113108u32)?;
    emu.lw_no_count(11usize, 2usize, 76u32, 2113112u32)?;
    emu.lw_no_count(12usize, 2usize, 80u32, 2113116u32)?;
    emu.lw_no_count(13usize, 2usize, 84u32, 2113120u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2113124u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2113128u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2113132u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2113136u32)?;
    emu.adi_no_count(10usize, 2usize, 488u32, 2113140u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2113144u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2113148u32);
    emu.apc_no_count(1usize, 2113148u32, 24576u32, 2113152u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113156u32;
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
#[inline(always)]
pub fn block_0x00203e84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 168u32, 2113160u32);
    emu.adi_no_count(11usize, 2usize, 360u32, 2113164u32);
    emu.adi_no_count(12usize, 2usize, 488u32, 2113168u32);
    emu.apc_no_count(1usize, 2113168u32, 24576u32, 2113172u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113176u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1676u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00203e98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 45u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 88u32, 2113180u32)?;
    emu.lw_no_count(11usize, 2usize, 92u32, 2113184u32)?;
    emu.lw_no_count(12usize, 2usize, 96u32, 2113188u32)?;
    emu.lw_no_count(13usize, 2usize, 100u32, 2113192u32)?;
    emu.lw_no_count(14usize, 2usize, 72u32, 2113196u32)?;
    emu.lw_no_count(15usize, 2usize, 76u32, 2113200u32)?;
    emu.lw_no_count(16usize, 2usize, 80u32, 2113204u32)?;
    emu.lw_no_count(17usize, 2usize, 84u32, 2113208u32)?;
    let a = 0u32.wrapping_add(3694133248u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2113212u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(75976704u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2113216u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(3852607488u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2113220u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4146147328u32);
    emu.write_reg_no_count(28usize, a);
    emu.pc = 2113224u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2901409792u32);
    emu.write_reg_no_count(29usize, a);
    emu.pc = 2113228u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2021928960u32);
    emu.write_reg_no_count(30usize, a);
    emu.pc = 2113232u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 2usize, 536u32, 2113236u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2113240u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2113244u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2113248u32)?;
    let a = 0u32.wrapping_add(3634159616u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2113252u32;
    emu.update_insn_clock();
    emu.sw_no_count(14usize, 2usize, 520u32, 2113256u32)?;
    emu.sw_no_count(15usize, 2usize, 524u32, 2113260u32)?;
    emu.sw_no_count(16usize, 2usize, 528u32, 2113264u32)?;
    emu.sw_no_count(17usize, 2usize, 532u32, 2113268u32)?;
    let a = 0u32.wrapping_add(700760064u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2113272u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 5usize, 1565u32, 2113276u32);
    emu.adi_no_count(21usize, 6usize, 4294965300u32, 2113280u32);
    emu.adi_no_count(23usize, 7usize, 171u32, 2113284u32);
    emu.adi_no_count(24usize, 28usize, 4294966998u32, 2113288u32);
    emu.adi_no_count(9usize, 29usize, 1485u32, 2113292u32);
    emu.adi_no_count(18usize, 30usize, 144u32, 2113296u32);
    emu.adi_no_count(20usize, 10usize, 4294967138u32, 2113300u32);
    emu.adi_no_count(22usize, 11usize, 4294966751u32, 2113304u32);
    emu.sw_no_count(24usize, 2usize, 568u32, 2113308u32)?;
    emu.sw_no_count(23usize, 2usize, 572u32, 2113312u32)?;
    emu.sw_no_count(21usize, 2usize, 576u32, 2113316u32)?;
    emu.sw_no_count(19usize, 2usize, 580u32, 2113320u32)?;
    emu.sw_no_count(22usize, 2usize, 552u32, 2113324u32)?;
    emu.sw_no_count(20usize, 2usize, 556u32, 2113328u32)?;
    emu.sw_no_count(18usize, 2usize, 560u32, 2113332u32)?;
    emu.sw_no_count(9usize, 2usize, 564u32, 2113336u32)?;
    emu.adi_no_count(10usize, 2usize, 488u32, 2113340u32);
    emu.adi_no_count(11usize, 2usize, 552u32, 2113344u32);
    emu.adi_no_count(12usize, 2usize, 520u32, 2113348u32);
    emu.apc_no_count(1usize, 2113348u32, 28672u32, 2113352u32);
    emu.add_memory_rw_events(45usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113356u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965264u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00203f4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 184u32, 2113360u32)?;
    emu.lw_no_count(11usize, 2usize, 188u32, 2113364u32)?;
    emu.lw_no_count(12usize, 2usize, 192u32, 2113368u32)?;
    emu.lw_no_count(13usize, 2usize, 196u32, 2113372u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2113376u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2113380u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2113384u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2113388u32)?;
    emu.lw_no_count(10usize, 2usize, 168u32, 2113392u32)?;
    emu.lw_no_count(11usize, 2usize, 172u32, 2113396u32)?;
    emu.lw_no_count(12usize, 2usize, 176u32, 2113400u32)?;
    emu.lw_no_count(13usize, 2usize, 180u32, 2113404u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2113408u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2113412u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2113416u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2113420u32)?;
    emu.adi_no_count(10usize, 2usize, 200u32, 2113424u32);
    emu.adi_no_count(11usize, 2usize, 552u32, 2113428u32);
    emu.adi_no_count(12usize, 2usize, 488u32, 2113432u32);
    emu.apc_no_count(1usize, 2113432u32, 24576u32, 2113436u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113440u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x00203fa0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 520u32, 2113444u32);
    emu.adi_no_count(11usize, 2usize, 200u32, 2113448u32);
    emu.apc_no_count(1usize, 2113448u32, 57344u32, 2113452u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113456u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965476u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00203fb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 216u32, 2113460u32)?;
    emu.lw_no_count(11usize, 2usize, 220u32, 2113464u32)?;
    emu.lw_no_count(12usize, 2usize, 224u32, 2113468u32)?;
    emu.lw_no_count(13usize, 2usize, 228u32, 2113472u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2113476u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2113480u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2113484u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2113488u32)?;
    emu.lw_no_count(10usize, 2usize, 200u32, 2113492u32)?;
    emu.lw_no_count(11usize, 2usize, 204u32, 2113496u32)?;
    emu.lw_no_count(12usize, 2usize, 208u32, 2113500u32)?;
    emu.lw_no_count(13usize, 2usize, 212u32, 2113504u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2113508u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2113512u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2113516u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2113520u32)?;
    emu.adi_no_count(10usize, 2usize, 232u32, 2113524u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2113528u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2113532u32);
    emu.apc_no_count(1usize, 2113532u32, 24576u32, 2113536u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113540u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(588u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00204004(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 56u32, 2113544u32)?;
    emu.lw_no_count(11usize, 2usize, 60u32, 2113548u32)?;
    emu.lw_no_count(12usize, 2usize, 64u32, 2113552u32)?;
    emu.lw_no_count(13usize, 2usize, 68u32, 2113556u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2113560u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2113564u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2113568u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2113572u32)?;
    emu.lw_no_count(10usize, 2usize, 40u32, 2113576u32)?;
    emu.lw_no_count(11usize, 2usize, 44u32, 2113580u32)?;
    emu.lw_no_count(12usize, 2usize, 48u32, 2113584u32)?;
    emu.lw_no_count(13usize, 2usize, 52u32, 2113588u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2113592u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2113596u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2113600u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2113604u32)?;
    emu.lw_no_count(10usize, 2usize, 248u32, 2113608u32)?;
    emu.lw_no_count(11usize, 2usize, 252u32, 2113612u32)?;
    emu.lw_no_count(12usize, 2usize, 256u32, 2113616u32)?;
    emu.lw_no_count(13usize, 2usize, 260u32, 2113620u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2113624u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2113628u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2113632u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2113636u32)?;
    emu.lw_no_count(10usize, 2usize, 232u32, 2113640u32)?;
    emu.lw_no_count(11usize, 2usize, 236u32, 2113644u32)?;
    emu.lw_no_count(12usize, 2usize, 240u32, 2113648u32)?;
    emu.lw_no_count(13usize, 2usize, 244u32, 2113652u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2113656u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2113660u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2113664u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2113668u32)?;
    emu.adi_no_count(10usize, 2usize, 264u32, 2113672u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2113676u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2113680u32);
    emu.apc_no_count(1usize, 2113680u32, 24576u32, 2113684u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113688u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1164u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204098(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 296u32, 2113692u32);
    emu.adi_no_count(11usize, 2usize, 40u32, 2113696u32);
    emu.adi_no_count(12usize, 2usize, 232u32, 2113700u32);
    emu.apc_no_count(1usize, 2113700u32, 24576u32, 2113704u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113708u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(420u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002040ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 520u32, 2113712u32);
    emu.adi_no_count(11usize, 2usize, 72u32, 2113716u32);
    emu.apc_no_count(1usize, 2113716u32, 53248u32, 2113720u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113724u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(2008u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002040bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 88u32, 2113728u32)?;
    emu.lw_no_count(11usize, 2usize, 92u32, 2113732u32)?;
    emu.lw_no_count(12usize, 2usize, 96u32, 2113736u32)?;
    emu.lw_no_count(13usize, 2usize, 100u32, 2113740u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2113744u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2113748u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2113752u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2113756u32)?;
    emu.lw_no_count(10usize, 2usize, 72u32, 2113760u32)?;
    emu.lw_no_count(11usize, 2usize, 76u32, 2113764u32)?;
    emu.lw_no_count(12usize, 2usize, 80u32, 2113768u32)?;
    emu.lw_no_count(13usize, 2usize, 84u32, 2113772u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2113776u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2113780u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2113784u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2113788u32)?;
    emu.adi_no_count(10usize, 2usize, 328u32, 2113792u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2113796u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2113800u32);
    emu.apc_no_count(1usize, 2113800u32, 24576u32, 2113804u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113808u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(320u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00204110(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(24usize, 2usize, 568u32, 2113812u32)?;
    emu.sw_no_count(23usize, 2usize, 572u32, 2113816u32)?;
    emu.sw_no_count(21usize, 2usize, 576u32, 2113820u32)?;
    emu.sw_no_count(19usize, 2usize, 580u32, 2113824u32)?;
    emu.sw_no_count(22usize, 2usize, 552u32, 2113828u32)?;
    emu.sw_no_count(20usize, 2usize, 556u32, 2113832u32)?;
    emu.sw_no_count(18usize, 2usize, 560u32, 2113836u32)?;
    emu.sw_no_count(9usize, 2usize, 564u32, 2113840u32)?;
    emu.adi_no_count(10usize, 2usize, 456u32, 2113844u32);
    emu.adi_no_count(11usize, 2usize, 552u32, 2113848u32);
    emu.adi_no_count(12usize, 2usize, 168u32, 2113852u32);
    emu.apc_no_count(1usize, 2113852u32, 24576u32, 2113856u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113860u32;
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
#[inline(never)]
pub fn block_0x00204144(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 24u32, 2113864u32)?;
    emu.lw_no_count(11usize, 2usize, 28u32, 2113868u32)?;
    emu.lw_no_count(12usize, 2usize, 32u32, 2113872u32)?;
    emu.lw_no_count(13usize, 2usize, 36u32, 2113876u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2113880u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2113884u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2113888u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2113892u32)?;
    emu.lw_no_count(10usize, 2usize, 8u32, 2113896u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2113900u32)?;
    emu.lw_no_count(12usize, 2usize, 16u32, 2113904u32)?;
    emu.lw_no_count(13usize, 2usize, 20u32, 2113908u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2113912u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2113916u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2113920u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2113924u32)?;
    emu.lw_no_count(10usize, 2usize, 344u32, 2113928u32)?;
    emu.lw_no_count(11usize, 2usize, 348u32, 2113932u32)?;
    emu.lw_no_count(12usize, 2usize, 352u32, 2113936u32)?;
    emu.lw_no_count(13usize, 2usize, 356u32, 2113940u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2113944u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2113948u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2113952u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2113956u32)?;
    emu.lw_no_count(10usize, 2usize, 328u32, 2113960u32)?;
    emu.lw_no_count(11usize, 2usize, 332u32, 2113964u32)?;
    emu.lw_no_count(12usize, 2usize, 336u32, 2113968u32)?;
    emu.lw_no_count(13usize, 2usize, 340u32, 2113972u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2113976u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2113980u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2113984u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2113988u32)?;
    emu.adi_no_count(10usize, 2usize, 488u32, 2113992u32);
    emu.adi_no_count(11usize, 2usize, 552u32, 2113996u32);
    emu.adi_no_count(12usize, 2usize, 520u32, 2114000u32);
    emu.apc_no_count(1usize, 2114000u32, 24576u32, 2114004u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114008u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(120u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002041d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 360u32, 2114012u32);
    emu.adi_no_count(11usize, 2usize, 456u32, 2114016u32);
    emu.adi_no_count(12usize, 2usize, 488u32, 2114020u32);
    emu.apc_no_count(1usize, 2114020u32, 24576u32, 2114024u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114028u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(824u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002041ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 520u32, 2114032u32);
    emu.adi_no_count(11usize, 2usize, 360u32, 2114036u32);
    emu.apc_no_count(1usize, 2114036u32, 53248u32, 2114040u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114044u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1688u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002041fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 376u32, 2114048u32)?;
    emu.lw_no_count(11usize, 2usize, 380u32, 2114052u32)?;
    emu.lw_no_count(12usize, 2usize, 384u32, 2114056u32)?;
    emu.lw_no_count(13usize, 2usize, 388u32, 2114060u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2114064u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2114068u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2114072u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2114076u32)?;
    emu.lw_no_count(10usize, 2usize, 360u32, 2114080u32)?;
    emu.lw_no_count(11usize, 2usize, 364u32, 2114084u32)?;
    emu.lw_no_count(12usize, 2usize, 368u32, 2114088u32)?;
    emu.lw_no_count(13usize, 2usize, 372u32, 2114092u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2114096u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2114100u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2114104u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2114108u32)?;
    emu.adi_no_count(10usize, 2usize, 392u32, 2114112u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2114116u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2114120u32);
    emu.apc_no_count(1usize, 2114120u32, 24576u32, 2114124u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114128u32;
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
pub fn block_0x00204250(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 520u32, 2114132u32);
    emu.adi_no_count(11usize, 2usize, 8u32, 2114136u32);
    emu.apc_no_count(1usize, 2114136u32, 53248u32, 2114140u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114144u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1588u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00204260(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 24u32, 2114148u32)?;
    emu.lw_no_count(11usize, 2usize, 28u32, 2114152u32)?;
    emu.lw_no_count(12usize, 2usize, 32u32, 2114156u32)?;
    emu.lw_no_count(13usize, 2usize, 36u32, 2114160u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2114164u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2114168u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2114172u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2114176u32)?;
    emu.lw_no_count(10usize, 2usize, 8u32, 2114180u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2114184u32)?;
    emu.lw_no_count(12usize, 2usize, 16u32, 2114188u32)?;
    emu.lw_no_count(13usize, 2usize, 20u32, 2114192u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2114196u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2114200u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2114204u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2114208u32)?;
    emu.adi_no_count(10usize, 2usize, 488u32, 2114212u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2114216u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2114220u32);
    emu.apc_no_count(1usize, 2114220u32, 24576u32, 2114224u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114228u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967196u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002042b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 424u32, 2114232u32);
    emu.adi_no_count(11usize, 2usize, 488u32, 2114236u32);
    emu.adi_no_count(12usize, 2usize, 328u32, 2114240u32);
    emu.apc_no_count(1usize, 2114240u32, 24576u32, 2114244u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114248u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(604u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x002042c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 312u32, 2114252u32)?;
    emu.lw_no_count(11usize, 2usize, 316u32, 2114256u32)?;
    emu.lw_no_count(12usize, 2usize, 320u32, 2114260u32)?;
    emu.lw_no_count(13usize, 2usize, 324u32, 2114264u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2114268u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2114272u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2114276u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2114280u32)?;
    emu.lw_no_count(10usize, 2usize, 296u32, 2114284u32)?;
    emu.lw_no_count(11usize, 2usize, 300u32, 2114288u32)?;
    emu.lw_no_count(12usize, 2usize, 304u32, 2114292u32)?;
    emu.lw_no_count(13usize, 2usize, 308u32, 2114296u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2114300u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2114304u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2114308u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2114312u32)?;
    emu.lw_no_count(10usize, 2usize, 120u32, 2114316u32)?;
    emu.lw_no_count(11usize, 2usize, 124u32, 2114320u32)?;
    emu.lw_no_count(12usize, 2usize, 128u32, 2114324u32)?;
    emu.lw_no_count(13usize, 2usize, 132u32, 2114328u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2114332u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2114336u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2114340u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2114344u32)?;
    emu.lw_no_count(10usize, 2usize, 104u32, 2114348u32)?;
    emu.lw_no_count(11usize, 2usize, 108u32, 2114352u32)?;
    emu.lw_no_count(12usize, 2usize, 112u32, 2114356u32)?;
    emu.lw_no_count(13usize, 2usize, 116u32, 2114360u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2114364u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2114368u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2114372u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2114376u32)?;
    emu.adi_no_count(10usize, 2usize, 456u32, 2114380u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2114384u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2114388u32);
    emu.apc_no_count(1usize, 2114388u32, 24576u32, 2114392u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114396u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1024u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0020435c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 152u32, 2114400u32)?;
    emu.lw_no_count(11usize, 2usize, 156u32, 2114404u32)?;
    emu.lw_no_count(12usize, 2usize, 160u32, 2114408u32)?;
    emu.lw_no_count(13usize, 2usize, 164u32, 2114412u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2114416u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2114420u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2114424u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2114428u32)?;
    emu.lw_no_count(10usize, 2usize, 136u32, 2114432u32)?;
    emu.lw_no_count(11usize, 2usize, 140u32, 2114436u32)?;
    emu.lw_no_count(12usize, 2usize, 144u32, 2114440u32)?;
    emu.lw_no_count(13usize, 2usize, 148u32, 2114444u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2114448u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2114452u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2114456u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2114460u32)?;
    emu.lw_no_count(10usize, 2usize, 408u32, 2114464u32)?;
    emu.lw_no_count(11usize, 2usize, 412u32, 2114468u32)?;
    emu.lw_no_count(12usize, 2usize, 416u32, 2114472u32)?;
    emu.lw_no_count(13usize, 2usize, 420u32, 2114476u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2114480u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2114484u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2114488u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2114492u32)?;
    emu.lw_no_count(10usize, 2usize, 392u32, 2114496u32)?;
    emu.lw_no_count(11usize, 2usize, 396u32, 2114500u32)?;
    emu.lw_no_count(12usize, 2usize, 400u32, 2114504u32)?;
    emu.lw_no_count(13usize, 2usize, 404u32, 2114508u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2114512u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2114516u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2114520u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2114524u32)?;
    emu.adi_no_count(10usize, 2usize, 488u32, 2114528u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2114532u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2114536u32);
    emu.apc_no_count(1usize, 2114536u32, 24576u32, 2114540u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114544u32;
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
pub fn block_0x002043f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 2usize, 456u32, 2114548u32);
    emu.adi_no_count(12usize, 2usize, 488u32, 2114552u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2114556u32);
    emu.apc_no_count(1usize, 2114556u32, 24576u32, 2114560u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114564u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(288u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00204404(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 280u32, 2114568u32)?;
    emu.lw_no_count(11usize, 2usize, 284u32, 2114572u32)?;
    emu.lw_no_count(12usize, 2usize, 288u32, 2114576u32)?;
    emu.lw_no_count(13usize, 2usize, 292u32, 2114580u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2114584u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2114588u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2114592u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2114596u32)?;
    emu.lw_no_count(10usize, 2usize, 264u32, 2114600u32)?;
    emu.lw_no_count(11usize, 2usize, 268u32, 2114604u32)?;
    emu.lw_no_count(12usize, 2usize, 272u32, 2114608u32)?;
    emu.lw_no_count(13usize, 2usize, 276u32, 2114612u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2114616u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2114620u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2114624u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2114628u32)?;
    emu.adi_no_count(10usize, 2usize, 488u32, 2114632u32);
    emu.adi_no_count(11usize, 2usize, 296u32, 2114636u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2114640u32);
    emu.apc_no_count(1usize, 2114640u32, 24576u32, 2114644u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114648u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(772u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00204458(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 440u32, 2114652u32)?;
    emu.lw_no_count(11usize, 2usize, 444u32, 2114656u32)?;
    emu.lw_no_count(12usize, 2usize, 448u32, 2114660u32)?;
    emu.lw_no_count(13usize, 2usize, 452u32, 2114664u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2114668u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2114672u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2114676u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2114680u32)?;
    emu.lw_no_count(10usize, 2usize, 424u32, 2114684u32)?;
    emu.lw_no_count(11usize, 2usize, 428u32, 2114688u32)?;
    emu.lw_no_count(12usize, 2usize, 432u32, 2114692u32)?;
    emu.lw_no_count(13usize, 2usize, 436u32, 2114696u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2114700u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2114704u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2114708u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2114712u32)?;
    emu.adi_no_count(10usize, 2usize, 520u32, 2114716u32);
    emu.adi_no_count(11usize, 2usize, 552u32, 2114720u32);
    emu.adi_no_count(12usize, 2usize, 392u32, 2114724u32);
    emu.apc_no_count(1usize, 2114724u32, 24576u32, 2114728u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114732u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(688u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002044ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 32u32, 2114736u32);
    emu.adi_no_count(11usize, 2usize, 488u32, 2114740u32);
    emu.adi_no_count(12usize, 2usize, 520u32, 2114744u32);
    emu.apc_no_count(1usize, 2114744u32, 24576u32, 2114748u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114752u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966672u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002044c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 520u32, 2114756u32);
    emu.adi_no_count(11usize, 2usize, 264u32, 2114760u32);
    emu.adi_no_count(12usize, 2usize, 136u32, 2114764u32);
    emu.apc_no_count(1usize, 2114764u32, 24576u32, 2114768u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114772u32;
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
#[inline(always)]
pub fn block_0x002044d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 552u32, 2114776u32);
    emu.adi_no_count(11usize, 2usize, 104u32, 2114780u32);
    emu.adi_no_count(12usize, 2usize, 424u32, 2114784u32);
    emu.apc_no_count(1usize, 2114784u32, 24576u32, 2114788u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114792u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(628u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002044e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 64u32, 2114796u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2114800u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2114804u32);
    emu.apc_no_count(1usize, 2114804u32, 24576u32, 2114808u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114812u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966612u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002044fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 620u32, 2114816u32)?;
    emu.lw_no_count(8usize, 2usize, 616u32, 2114820u32)?;
    emu.lw_no_count(9usize, 2usize, 612u32, 2114824u32)?;
    emu.lw_no_count(18usize, 2usize, 608u32, 2114828u32)?;
    emu.lw_no_count(19usize, 2usize, 604u32, 2114832u32)?;
    emu.lw_no_count(20usize, 2usize, 600u32, 2114836u32)?;
    emu.lw_no_count(21usize, 2usize, 596u32, 2114840u32)?;
    emu.lw_no_count(22usize, 2usize, 592u32, 2114844u32)?;
    emu.lw_no_count(23usize, 2usize, 588u32, 2114848u32)?;
    emu.lw_no_count(24usize, 2usize, 584u32, 2114852u32)?;
    emu.adi_no_count(2usize, 2usize, 624u32, 2114856u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114860u32;
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
pub fn block_0x0020452c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294966576u32, 2114864u32);
    emu.sw_no_count(1usize, 2usize, 716u32, 2114868u32)?;
    emu.sw_no_count(8usize, 2usize, 712u32, 2114872u32)?;
    emu.sw_no_count(9usize, 2usize, 708u32, 2114876u32)?;
    emu.sw_no_count(18usize, 2usize, 704u32, 2114880u32)?;
    emu.sw_no_count(19usize, 2usize, 700u32, 2114884u32)?;
    emu.sw_no_count(20usize, 2usize, 696u32, 2114888u32)?;
    emu.sw_no_count(21usize, 2usize, 692u32, 2114892u32)?;
    emu.sw_no_count(22usize, 2usize, 688u32, 2114896u32)?;
    emu.sw_no_count(23usize, 2usize, 684u32, 2114900u32)?;
    emu.sw_no_count(24usize, 2usize, 680u32, 2114904u32)?;
    emu.sw_no_count(25usize, 2usize, 676u32, 2114908u32)?;
    emu.adi_no_count(9usize, 11usize, 0u32, 2114912u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2114916u32);
    emu.adi_no_count(10usize, 2usize, 4u32, 2114920u32);
    emu.apc_no_count(1usize, 2114920u32, 53248u32, 2114924u32);
    emu.add_memory_rw_events(17usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114928u32;
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
#[inline(always)]
pub fn block_0x00204570(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 9usize, 32u32, 2114932u32);
    emu.adi_no_count(10usize, 2usize, 36u32, 2114936u32);
    emu.apc_no_count(1usize, 2114936u32, 53248u32, 2114940u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114944u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1208u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204580(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 9usize, 64u32, 2114948u32);
    emu.adi_no_count(10usize, 2usize, 68u32, 2114952u32);
    emu.apc_no_count(1usize, 2114952u32, 53248u32, 2114956u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114960u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1192u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00204590(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 16u32, 2114964u32)?;
    emu.lw_no_count(11usize, 9usize, 20u32, 2114968u32)?;
    emu.lw_no_count(12usize, 9usize, 24u32, 2114972u32)?;
    emu.lw_no_count(13usize, 9usize, 28u32, 2114976u32)?;
    emu.sw_no_count(10usize, 2usize, 628u32, 2114980u32)?;
    emu.sw_no_count(11usize, 2usize, 632u32, 2114984u32)?;
    emu.sw_no_count(12usize, 2usize, 636u32, 2114988u32)?;
    emu.sw_no_count(13usize, 2usize, 640u32, 2114992u32)?;
    emu.lw_no_count(10usize, 9usize, 0u32, 2114996u32)?;
    emu.lw_no_count(11usize, 9usize, 4u32, 2115000u32)?;
    emu.lw_no_count(12usize, 9usize, 8u32, 2115004u32)?;
    emu.lw_no_count(13usize, 9usize, 12u32, 2115008u32)?;
    emu.sw_no_count(10usize, 2usize, 612u32, 2115012u32)?;
    emu.sw_no_count(11usize, 2usize, 616u32, 2115016u32)?;
    emu.sw_no_count(12usize, 2usize, 620u32, 2115020u32)?;
    emu.sw_no_count(13usize, 2usize, 624u32, 2115024u32)?;
    emu.lw_no_count(10usize, 9usize, 48u32, 2115028u32)?;
    emu.lw_no_count(11usize, 9usize, 52u32, 2115032u32)?;
    emu.lw_no_count(12usize, 9usize, 56u32, 2115036u32)?;
    emu.lw_no_count(13usize, 9usize, 60u32, 2115040u32)?;
    emu.sw_no_count(10usize, 2usize, 660u32, 2115044u32)?;
    emu.sw_no_count(11usize, 2usize, 664u32, 2115048u32)?;
    emu.sw_no_count(12usize, 2usize, 668u32, 2115052u32)?;
    emu.sw_no_count(13usize, 2usize, 672u32, 2115056u32)?;
    emu.lw_no_count(10usize, 9usize, 32u32, 2115060u32)?;
    emu.lw_no_count(11usize, 9usize, 36u32, 2115064u32)?;
    emu.lw_no_count(12usize, 9usize, 40u32, 2115068u32)?;
    emu.lw_no_count(13usize, 9usize, 44u32, 2115072u32)?;
    emu.sw_no_count(10usize, 2usize, 644u32, 2115076u32)?;
    emu.sw_no_count(11usize, 2usize, 648u32, 2115080u32)?;
    emu.sw_no_count(12usize, 2usize, 652u32, 2115084u32)?;
    emu.sw_no_count(13usize, 2usize, 656u32, 2115088u32)?;
    emu.adi_no_count(10usize, 2usize, 580u32, 2115092u32);
    emu.adi_no_count(11usize, 2usize, 612u32, 2115096u32);
    emu.adi_no_count(12usize, 2usize, 644u32, 2115100u32);
    emu.apc_no_count(1usize, 2115100u32, 24576u32, 2115104u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2115108u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(312u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204624(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 100u32, 2115112u32);
    emu.adi_no_count(11usize, 2usize, 580u32, 2115116u32);
    emu.apc_no_count(1usize, 2115116u32, 53248u32, 2115120u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2115124u32;
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
#[inline(never)]
pub fn block_0x00204634(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 16u32, 2115128u32)?;
    emu.lw_no_count(11usize, 9usize, 20u32, 2115132u32)?;
    emu.lw_no_count(12usize, 9usize, 24u32, 2115136u32)?;
    emu.lw_no_count(13usize, 9usize, 28u32, 2115140u32)?;
    emu.sw_no_count(10usize, 2usize, 628u32, 2115144u32)?;
    emu.sw_no_count(11usize, 2usize, 632u32, 2115148u32)?;
    emu.sw_no_count(12usize, 2usize, 636u32, 2115152u32)?;
    emu.sw_no_count(13usize, 2usize, 640u32, 2115156u32)?;
    emu.lw_no_count(10usize, 9usize, 0u32, 2115160u32)?;
    emu.lw_no_count(11usize, 9usize, 4u32, 2115164u32)?;
    emu.lw_no_count(12usize, 9usize, 8u32, 2115168u32)?;
    emu.lw_no_count(13usize, 9usize, 12u32, 2115172u32)?;
    emu.sw_no_count(10usize, 2usize, 612u32, 2115176u32)?;
    emu.sw_no_count(11usize, 2usize, 616u32, 2115180u32)?;
    emu.sw_no_count(12usize, 2usize, 620u32, 2115184u32)?;
    emu.sw_no_count(13usize, 2usize, 624u32, 2115188u32)?;
    emu.lw_no_count(10usize, 9usize, 80u32, 2115192u32)?;
    emu.lw_no_count(11usize, 9usize, 84u32, 2115196u32)?;
    emu.lw_no_count(12usize, 9usize, 88u32, 2115200u32)?;
    emu.lw_no_count(13usize, 9usize, 92u32, 2115204u32)?;
    emu.sw_no_count(10usize, 2usize, 660u32, 2115208u32)?;
    emu.sw_no_count(11usize, 2usize, 664u32, 2115212u32)?;
    emu.sw_no_count(12usize, 2usize, 668u32, 2115216u32)?;
    emu.sw_no_count(13usize, 2usize, 672u32, 2115220u32)?;
    emu.lw_no_count(10usize, 9usize, 64u32, 2115224u32)?;
    emu.lw_no_count(11usize, 9usize, 68u32, 2115228u32)?;
    emu.lw_no_count(12usize, 9usize, 72u32, 2115232u32)?;
    emu.lw_no_count(13usize, 9usize, 76u32, 2115236u32)?;
    emu.sw_no_count(10usize, 2usize, 644u32, 2115240u32)?;
    emu.sw_no_count(11usize, 2usize, 648u32, 2115244u32)?;
    emu.sw_no_count(12usize, 2usize, 652u32, 2115248u32)?;
    emu.sw_no_count(13usize, 2usize, 656u32, 2115252u32)?;
    emu.adi_no_count(10usize, 2usize, 580u32, 2115256u32);
    emu.adi_no_count(11usize, 2usize, 612u32, 2115260u32);
    emu.adi_no_count(12usize, 2usize, 644u32, 2115264u32);
    emu.apc_no_count(1usize, 2115264u32, 24576u32, 2115268u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2115272u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(148u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002046c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 132u32, 2115276u32);
    emu.adi_no_count(11usize, 2usize, 580u32, 2115280u32);
    emu.apc_no_count(1usize, 2115280u32, 53248u32, 2115284u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2115288u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(444u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x002046d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 45u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 84u32, 2115292u32)?;
    emu.lw_no_count(11usize, 2usize, 88u32, 2115296u32)?;
    emu.lw_no_count(12usize, 2usize, 92u32, 2115300u32)?;
    emu.lw_no_count(13usize, 2usize, 96u32, 2115304u32)?;
    emu.lw_no_count(14usize, 2usize, 68u32, 2115308u32)?;
    emu.lw_no_count(15usize, 2usize, 72u32, 2115312u32)?;
    emu.lw_no_count(16usize, 2usize, 76u32, 2115316u32)?;
    emu.lw_no_count(17usize, 2usize, 80u32, 2115320u32)?;
    let a = 0u32.wrapping_add(3694133248u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2115324u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(75976704u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2115328u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(3852607488u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2115332u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4146147328u32);
    emu.write_reg_no_count(28usize, a);
    emu.pc = 2115336u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2901409792u32);
    emu.write_reg_no_count(29usize, a);
    emu.pc = 2115340u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2021928960u32);
    emu.write_reg_no_count(30usize, a);
    emu.pc = 2115344u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 2usize, 628u32, 2115348u32)?;
    emu.sw_no_count(11usize, 2usize, 632u32, 2115352u32)?;
    emu.sw_no_count(12usize, 2usize, 636u32, 2115356u32)?;
    emu.sw_no_count(13usize, 2usize, 640u32, 2115360u32)?;
    let a = 0u32.wrapping_add(3634159616u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2115364u32;
    emu.update_insn_clock();
    emu.sw_no_count(14usize, 2usize, 612u32, 2115368u32)?;
    emu.sw_no_count(15usize, 2usize, 616u32, 2115372u32)?;
    emu.sw_no_count(16usize, 2usize, 620u32, 2115376u32)?;
    emu.sw_no_count(17usize, 2usize, 624u32, 2115380u32)?;
    let a = 0u32.wrapping_add(700760064u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2115384u32;
    emu.update_insn_clock();
    emu.adi_no_count(20usize, 5usize, 1565u32, 2115388u32);
    emu.adi_no_count(22usize, 6usize, 4294965300u32, 2115392u32);
    emu.adi_no_count(24usize, 7usize, 171u32, 2115396u32);
    emu.adi_no_count(25usize, 28usize, 4294966998u32, 2115400u32);
    emu.adi_no_count(18usize, 29usize, 1485u32, 2115404u32);
    emu.adi_no_count(19usize, 30usize, 144u32, 2115408u32);
    emu.adi_no_count(21usize, 10usize, 4294967138u32, 2115412u32);
    emu.adi_no_count(23usize, 11usize, 4294966751u32, 2115416u32);
    emu.sw_no_count(25usize, 2usize, 660u32, 2115420u32)?;
    emu.sw_no_count(24usize, 2usize, 664u32, 2115424u32)?;
    emu.sw_no_count(22usize, 2usize, 668u32, 2115428u32)?;
    emu.sw_no_count(20usize, 2usize, 672u32, 2115432u32)?;
    emu.sw_no_count(23usize, 2usize, 644u32, 2115436u32)?;
    emu.sw_no_count(21usize, 2usize, 648u32, 2115440u32)?;
    emu.sw_no_count(19usize, 2usize, 652u32, 2115444u32)?;
    emu.sw_no_count(18usize, 2usize, 656u32, 2115448u32)?;
    emu.adi_no_count(10usize, 2usize, 580u32, 2115452u32);
    emu.adi_no_count(11usize, 2usize, 644u32, 2115456u32);
    emu.adi_no_count(12usize, 2usize, 612u32, 2115460u32);
    emu.apc_no_count(1usize, 2115460u32, 24576u32, 2115464u32);
    emu.add_memory_rw_events(45usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2115468u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967248u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020478c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 148u32, 2115472u32)?;
    emu.lw_no_count(11usize, 2usize, 152u32, 2115476u32)?;
    emu.lw_no_count(12usize, 2usize, 156u32, 2115480u32)?;
    emu.lw_no_count(13usize, 2usize, 160u32, 2115484u32)?;
    emu.sw_no_count(10usize, 2usize, 660u32, 2115488u32)?;
    emu.sw_no_count(11usize, 2usize, 664u32, 2115492u32)?;
    emu.sw_no_count(12usize, 2usize, 668u32, 2115496u32)?;
    emu.sw_no_count(13usize, 2usize, 672u32, 2115500u32)?;
    emu.lw_no_count(10usize, 2usize, 132u32, 2115504u32)?;
    emu.lw_no_count(11usize, 2usize, 136u32, 2115508u32)?;
    emu.lw_no_count(12usize, 2usize, 140u32, 2115512u32)?;
    emu.lw_no_count(13usize, 2usize, 144u32, 2115516u32)?;
    emu.sw_no_count(10usize, 2usize, 644u32, 2115520u32)?;
    emu.sw_no_count(11usize, 2usize, 648u32, 2115524u32)?;
    emu.sw_no_count(12usize, 2usize, 652u32, 2115528u32)?;
    emu.sw_no_count(13usize, 2usize, 656u32, 2115532u32)?;
    emu.adi_no_count(10usize, 2usize, 164u32, 2115536u32);
    emu.adi_no_count(11usize, 2usize, 580u32, 2115540u32);
    emu.adi_no_count(12usize, 2usize, 644u32, 2115544u32);
    emu.apc_no_count(1usize, 2115544u32, 24576u32, 2115548u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2115552u32;
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
pub fn block_0x002047e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 612u32, 2115556u32);
    emu.adi_no_count(11usize, 2usize, 164u32, 2115560u32);
    emu.apc_no_count(1usize, 2115560u32, 53248u32, 2115564u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2115568u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(164u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002047f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 180u32, 2115572u32)?;
    emu.lw_no_count(11usize, 2usize, 184u32, 2115576u32)?;
    emu.lw_no_count(12usize, 2usize, 188u32, 2115580u32)?;
    emu.lw_no_count(13usize, 2usize, 192u32, 2115584u32)?;
    emu.sw_no_count(10usize, 2usize, 660u32, 2115588u32)?;
    emu.sw_no_count(11usize, 2usize, 664u32, 2115592u32)?;
    emu.sw_no_count(12usize, 2usize, 668u32, 2115596u32)?;
    emu.sw_no_count(13usize, 2usize, 672u32, 2115600u32)?;
    emu.lw_no_count(10usize, 2usize, 164u32, 2115604u32)?;
    emu.lw_no_count(11usize, 2usize, 168u32, 2115608u32)?;
    emu.lw_no_count(12usize, 2usize, 172u32, 2115612u32)?;
    emu.lw_no_count(13usize, 2usize, 176u32, 2115616u32)?;
    emu.sw_no_count(10usize, 2usize, 644u32, 2115620u32)?;
    emu.sw_no_count(11usize, 2usize, 648u32, 2115624u32)?;
    emu.sw_no_count(12usize, 2usize, 652u32, 2115628u32)?;
    emu.sw_no_count(13usize, 2usize, 656u32, 2115632u32)?;
    emu.adi_no_count(10usize, 2usize, 196u32, 2115636u32);
    emu.adi_no_count(11usize, 2usize, 612u32, 2115640u32);
    emu.adi_no_count(12usize, 2usize, 644u32, 2115644u32);
    emu.apc_no_count(1usize, 2115644u32, 24576u32, 2115648u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2115652u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965772u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00204844(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 52u32, 2115656u32)?;
    emu.lw_no_count(11usize, 2usize, 56u32, 2115660u32)?;
    emu.lw_no_count(12usize, 2usize, 60u32, 2115664u32)?;
    emu.lw_no_count(13usize, 2usize, 64u32, 2115668u32)?;
    emu.sw_no_count(10usize, 2usize, 628u32, 2115672u32)?;
    emu.sw_no_count(11usize, 2usize, 632u32, 2115676u32)?;
    emu.sw_no_count(12usize, 2usize, 636u32, 2115680u32)?;
    emu.sw_no_count(13usize, 2usize, 640u32, 2115684u32)?;
    emu.lw_no_count(10usize, 2usize, 36u32, 2115688u32)?;
    emu.lw_no_count(11usize, 2usize, 40u32, 2115692u32)?;
    emu.lw_no_count(12usize, 2usize, 44u32, 2115696u32)?;
    emu.lw_no_count(13usize, 2usize, 48u32, 2115700u32)?;
    emu.sw_no_count(10usize, 2usize, 612u32, 2115704u32)?;
    emu.sw_no_count(11usize, 2usize, 616u32, 2115708u32)?;
    emu.sw_no_count(12usize, 2usize, 620u32, 2115712u32)?;
    emu.sw_no_count(13usize, 2usize, 624u32, 2115716u32)?;
    emu.lw_no_count(10usize, 2usize, 212u32, 2115720u32)?;
    emu.lw_no_count(11usize, 2usize, 216u32, 2115724u32)?;
    emu.lw_no_count(12usize, 2usize, 220u32, 2115728u32)?;
    emu.lw_no_count(13usize, 2usize, 224u32, 2115732u32)?;
    emu.sw_no_count(10usize, 2usize, 660u32, 2115736u32)?;
    emu.sw_no_count(11usize, 2usize, 664u32, 2115740u32)?;
    emu.sw_no_count(12usize, 2usize, 668u32, 2115744u32)?;
    emu.sw_no_count(13usize, 2usize, 672u32, 2115748u32)?;
    emu.lw_no_count(10usize, 2usize, 196u32, 2115752u32)?;
    emu.lw_no_count(11usize, 2usize, 200u32, 2115756u32)?;
    emu.lw_no_count(12usize, 2usize, 204u32, 2115760u32)?;
    emu.lw_no_count(13usize, 2usize, 208u32, 2115764u32)?;
    emu.sw_no_count(10usize, 2usize, 644u32, 2115768u32)?;
    emu.sw_no_count(11usize, 2usize, 648u32, 2115772u32)?;
    emu.sw_no_count(12usize, 2usize, 652u32, 2115776u32)?;
    emu.sw_no_count(13usize, 2usize, 656u32, 2115780u32)?;
    emu.adi_no_count(10usize, 2usize, 228u32, 2115784u32);
    emu.adi_no_count(11usize, 2usize, 612u32, 2115788u32);
    emu.adi_no_count(12usize, 2usize, 644u32, 2115792u32);
    emu.apc_no_count(1usize, 2115792u32, 24576u32, 2115796u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2115800u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966348u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002048d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 52u32, 2115804u32)?;
    emu.lw_no_count(11usize, 2usize, 56u32, 2115808u32)?;
    emu.lw_no_count(12usize, 2usize, 60u32, 2115812u32)?;
    emu.lw_no_count(13usize, 2usize, 64u32, 2115816u32)?;
    emu.sw_no_count(10usize, 2usize, 660u32, 2115820u32)?;
    emu.sw_no_count(11usize, 2usize, 664u32, 2115824u32)?;
    emu.sw_no_count(12usize, 2usize, 668u32, 2115828u32)?;
    emu.sw_no_count(13usize, 2usize, 672u32, 2115832u32)?;
    emu.lw_no_count(10usize, 2usize, 36u32, 2115836u32)?;
    emu.lw_no_count(11usize, 2usize, 40u32, 2115840u32)?;
    emu.lw_no_count(12usize, 2usize, 44u32, 2115844u32)?;
    emu.lw_no_count(13usize, 2usize, 48u32, 2115848u32)?;
    emu.sw_no_count(10usize, 2usize, 644u32, 2115852u32)?;
    emu.sw_no_count(11usize, 2usize, 648u32, 2115856u32)?;
    emu.sw_no_count(12usize, 2usize, 652u32, 2115860u32)?;
    emu.sw_no_count(13usize, 2usize, 656u32, 2115864u32)?;
    emu.adi_no_count(10usize, 2usize, 260u32, 2115868u32);
    emu.adi_no_count(11usize, 2usize, 644u32, 2115872u32);
    emu.adi_no_count(12usize, 2usize, 196u32, 2115876u32);
    emu.apc_no_count(1usize, 2115876u32, 24576u32, 2115880u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2115884u32;
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
pub fn block_0x0020492c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 244u32, 2115888u32)?;
    emu.lw_no_count(11usize, 2usize, 248u32, 2115892u32)?;
    emu.lw_no_count(12usize, 2usize, 252u32, 2115896u32)?;
    emu.lw_no_count(13usize, 2usize, 256u32, 2115900u32)?;
    emu.sw_no_count(10usize, 2usize, 660u32, 2115904u32)?;
    emu.sw_no_count(11usize, 2usize, 664u32, 2115908u32)?;
    emu.sw_no_count(12usize, 2usize, 668u32, 2115912u32)?;
    emu.sw_no_count(13usize, 2usize, 672u32, 2115916u32)?;
    emu.lw_no_count(10usize, 2usize, 228u32, 2115920u32)?;
    emu.lw_no_count(11usize, 2usize, 232u32, 2115924u32)?;
    emu.lw_no_count(12usize, 2usize, 236u32, 2115928u32)?;
    emu.lw_no_count(13usize, 2usize, 240u32, 2115932u32)?;
    emu.sw_no_count(10usize, 2usize, 644u32, 2115936u32)?;
    emu.sw_no_count(11usize, 2usize, 648u32, 2115940u32)?;
    emu.sw_no_count(12usize, 2usize, 652u32, 2115944u32)?;
    emu.sw_no_count(13usize, 2usize, 656u32, 2115948u32)?;
    emu.adi_no_count(10usize, 2usize, 292u32, 2115952u32);
    emu.adi_no_count(11usize, 2usize, 260u32, 2115956u32);
    emu.adi_no_count(12usize, 2usize, 644u32, 2115960u32);
    emu.apc_no_count(1usize, 2115960u32, 24576u32, 2115964u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2115968u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966748u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204980(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 324u32, 2115972u32);
    emu.adi_no_count(11usize, 2usize, 228u32, 2115976u32);
    emu.adi_no_count(12usize, 2usize, 100u32, 2115980u32);
    emu.apc_no_count(1usize, 2115980u32, 24576u32, 2115984u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2115988u32;
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
#[inline(always)]
pub fn block_0x00204994(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 612u32, 2115992u32);
    emu.adi_no_count(11usize, 2usize, 68u32, 2115996u32);
    emu.apc_no_count(1usize, 2115996u32, 53248u32, 2116000u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116004u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967024u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002049a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 84u32, 2116008u32)?;
    emu.lw_no_count(11usize, 2usize, 88u32, 2116012u32)?;
    emu.lw_no_count(12usize, 2usize, 92u32, 2116016u32)?;
    emu.lw_no_count(13usize, 2usize, 96u32, 2116020u32)?;
    emu.sw_no_count(10usize, 2usize, 660u32, 2116024u32)?;
    emu.sw_no_count(11usize, 2usize, 664u32, 2116028u32)?;
    emu.sw_no_count(12usize, 2usize, 668u32, 2116032u32)?;
    emu.sw_no_count(13usize, 2usize, 672u32, 2116036u32)?;
    emu.lw_no_count(10usize, 2usize, 68u32, 2116040u32)?;
    emu.lw_no_count(11usize, 2usize, 72u32, 2116044u32)?;
    emu.lw_no_count(12usize, 2usize, 76u32, 2116048u32)?;
    emu.lw_no_count(13usize, 2usize, 80u32, 2116052u32)?;
    emu.sw_no_count(10usize, 2usize, 644u32, 2116056u32)?;
    emu.sw_no_count(11usize, 2usize, 648u32, 2116060u32)?;
    emu.sw_no_count(12usize, 2usize, 652u32, 2116064u32)?;
    emu.sw_no_count(13usize, 2usize, 656u32, 2116068u32)?;
    emu.adi_no_count(10usize, 2usize, 356u32, 2116072u32);
    emu.adi_no_count(11usize, 2usize, 612u32, 2116076u32);
    emu.adi_no_count(12usize, 2usize, 644u32, 2116080u32);
    emu.apc_no_count(1usize, 2116080u32, 24576u32, 2116084u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116088u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965336u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002049f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(25usize, 2usize, 660u32, 2116092u32)?;
    emu.sw_no_count(24usize, 2usize, 664u32, 2116096u32)?;
    emu.sw_no_count(22usize, 2usize, 668u32, 2116100u32)?;
    emu.sw_no_count(20usize, 2usize, 672u32, 2116104u32)?;
    emu.sw_no_count(23usize, 2usize, 644u32, 2116108u32)?;
    emu.sw_no_count(21usize, 2usize, 648u32, 2116112u32)?;
    emu.sw_no_count(19usize, 2usize, 652u32, 2116116u32)?;
    emu.sw_no_count(18usize, 2usize, 656u32, 2116120u32)?;
    emu.adi_no_count(10usize, 2usize, 420u32, 2116124u32);
    emu.adi_no_count(11usize, 2usize, 644u32, 2116128u32);
    emu.adi_no_count(12usize, 2usize, 132u32, 2116132u32);
    emu.apc_no_count(1usize, 2116132u32, 24576u32, 2116136u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116140u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966576u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
