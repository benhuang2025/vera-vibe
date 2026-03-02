pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2105436u32;
pub const PC_MAX: u32 = 2108496u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 126usize] = [
        block_0x0020205c,
        block_0x00202090,
        block_0x002020b8,
        block_0x002020c4,
        block_0x002020c8,
        block_0x002020e0,
        block_0x002020e8,
        block_0x002020f0,
        block_0x002020f4,
        block_0x00202100,
        block_0x00202108,
        block_0x0020210c,
        block_0x00202118,
        block_0x00202124,
        block_0x0020212c,
        block_0x00202130,
        block_0x00202134,
        block_0x0020213c,
        block_0x00202140,
        block_0x0020214c,
        block_0x00202150,
        block_0x00202160,
        block_0x00202190,
        block_0x002021a4,
        block_0x002021b4,
        block_0x002021bc,
        block_0x002021c4,
        block_0x002021cc,
        block_0x002021d4,
        block_0x002021e4,
        block_0x00202214,
        block_0x00202220,
        block_0x00202234,
        block_0x00202244,
        block_0x0020224c,
        block_0x00202254,
        block_0x0020225c,
        block_0x00202264,
        block_0x00202280,
        block_0x00202298,
        block_0x002022b4,
        block_0x002022d4,
        block_0x002022fc,
        block_0x00202308,
        block_0x00202330,
        block_0x0020233c,
        block_0x00202364,
        block_0x00202370,
        block_0x0020238c,
        block_0x002023a8,
        block_0x002023d0,
        block_0x002023dc,
        block_0x002023f8,
        block_0x00202414,
        block_0x00202438,
        block_0x00202444,
        block_0x0020246c,
        block_0x00202470,
        block_0x00202474,
        block_0x00202480,
        block_0x00202490,
        block_0x002024a0,
        block_0x002024a4,
        block_0x002024b0,
        block_0x002024b8,
        block_0x002024d0,
        block_0x002024d4,
        block_0x00202504,
        block_0x00202510,
        block_0x00202514,
        block_0x00202528,
        block_0x00202538,
        block_0x00202540,
        block_0x00202548,
        block_0x00202560,
        block_0x0020256c,
        block_0x00202570,
        block_0x00202574,
        block_0x0020257c,
        block_0x00202584,
        block_0x0020258c,
        block_0x00202590,
        block_0x002025a4,
        block_0x002025b8,
        block_0x00202608,
        block_0x0020261c,
        block_0x00202620,
        block_0x0020262c,
        block_0x0020267c,
        block_0x00202690,
        block_0x002026a4,
        block_0x002026b0,
        block_0x002026c4,
        block_0x002026cc,
        block_0x002026dc,
        block_0x002026e8,
        block_0x00202720,
        block_0x00202738,
        block_0x002027c4,
        block_0x002027d8,
        block_0x002027e4,
        block_0x002027f8,
        block_0x0020280c,
        block_0x00202854,
        block_0x00202868,
        block_0x00202870,
        block_0x002029c4,
        block_0x002029d4,
        block_0x002029e4,
        block_0x002029f4,
        block_0x00202a04,
        block_0x00202a14,
        block_0x00202a24,
        block_0x00202a34,
        block_0x00202a40,
        block_0x00202a90,
        block_0x00202ac8,
        block_0x00202b00,
        block_0x00202b38,
        block_0x00202b70,
        block_0x00202ba8,
        block_0x00202be0,
        block_0x00202c18,
        block_0x00202c24,
        block_0x00202c40,
        block_0x00202c50,
    ];
    const IDX: [u16; 766usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        2u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 3u16, 0u16, 0u16,
        4u16, 5u16, 0u16, 0u16, 0u16, 0u16, 0u16, 6u16, 0u16, 7u16, 0u16, 8u16, 9u16,
        0u16, 0u16, 10u16, 0u16, 11u16, 12u16, 0u16, 0u16, 13u16, 0u16, 0u16, 14u16,
        0u16, 15u16, 16u16, 17u16, 0u16, 18u16, 19u16, 0u16, 0u16, 20u16, 21u16, 0u16,
        0u16, 0u16, 22u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 23u16, 0u16, 0u16, 0u16, 0u16, 24u16, 0u16, 0u16, 0u16, 25u16, 0u16, 26u16,
        0u16, 27u16, 0u16, 28u16, 0u16, 29u16, 0u16, 0u16, 0u16, 30u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 31u16, 0u16, 0u16, 32u16, 0u16,
        0u16, 0u16, 0u16, 33u16, 0u16, 0u16, 0u16, 34u16, 0u16, 35u16, 0u16, 36u16, 0u16,
        37u16, 0u16, 38u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 39u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 40u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 41u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 42u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 43u16, 0u16, 0u16, 44u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 45u16, 0u16, 0u16, 46u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 47u16, 0u16, 0u16, 48u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 49u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 50u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 51u16, 0u16, 0u16, 52u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 53u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 54u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 55u16, 0u16, 0u16, 56u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 57u16, 58u16, 59u16, 0u16, 0u16, 60u16, 0u16, 0u16, 0u16, 61u16,
        0u16, 0u16, 0u16, 62u16, 63u16, 0u16, 0u16, 64u16, 0u16, 65u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 66u16, 67u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 68u16, 0u16, 0u16, 69u16, 70u16, 0u16, 0u16, 0u16, 0u16, 71u16, 0u16,
        0u16, 0u16, 72u16, 0u16, 73u16, 0u16, 74u16, 0u16, 0u16, 0u16, 0u16, 0u16, 75u16,
        0u16, 0u16, 76u16, 77u16, 78u16, 0u16, 79u16, 0u16, 80u16, 0u16, 81u16, 82u16,
        0u16, 0u16, 0u16, 0u16, 83u16, 0u16, 0u16, 0u16, 0u16, 84u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 85u16, 0u16, 0u16, 0u16, 0u16, 86u16, 87u16, 0u16, 0u16, 88u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 89u16, 0u16, 0u16, 0u16, 0u16, 90u16, 0u16,
        0u16, 0u16, 0u16, 91u16, 0u16, 0u16, 92u16, 0u16, 0u16, 0u16, 0u16, 93u16, 0u16,
        94u16, 0u16, 0u16, 0u16, 95u16, 0u16, 0u16, 96u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 97u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 98u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 99u16, 0u16, 0u16,
        0u16, 0u16, 100u16, 0u16, 0u16, 101u16, 0u16, 0u16, 0u16, 0u16, 102u16, 0u16,
        0u16, 0u16, 0u16, 103u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 104u16, 0u16, 0u16, 0u16, 0u16,
        105u16, 0u16, 106u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 107u16, 0u16, 0u16, 0u16,
        108u16, 0u16, 0u16, 0u16, 109u16, 0u16, 0u16, 0u16, 110u16, 0u16, 0u16, 0u16,
        111u16, 0u16, 0u16, 0u16, 112u16, 0u16, 0u16, 0u16, 113u16, 0u16, 0u16, 0u16,
        114u16, 0u16, 0u16, 115u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 116u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 117u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 118u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        119u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 120u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 121u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 122u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 123u16, 0u16, 0u16, 124u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 125u16, 0u16, 0u16, 0u16, 126u16,
    ];
    if pc < 2105436u32 || pc > 2108496u32 {
        return None;
    }
    let word_offset = ((pc - 2105436u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x0020205c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2105440u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2105444u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2105448u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2105452u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2105456u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2105460u32)?;
    emu.sw_no_count(20usize, 2usize, 24u32, 2105464u32)?;
    emu.sw_no_count(21usize, 2usize, 20u32, 2105468u32)?;
    emu.sw_no_count(22usize, 2usize, 16u32, 2105472u32)?;
    emu.sw_no_count(23usize, 2usize, 12u32, 2105476u32)?;
    emu.sw_no_count(24usize, 2usize, 8u32, 2105480u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2105484u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2105664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202140));
    } else {
        emu.pc = 2105488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202090));
    }
}
#[inline]
pub fn block_0x00202090(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 13usize, 0u32, 2105492u32);
    emu.adi_no_count(18usize, 12usize, 0u32, 2105496u32);
    emu.adi_no_count(19usize, 11usize, 0u32, 2105500u32);
    emu.adi_no_count(21usize, 0usize, 4u32, 2105504u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(20usize, a);
    emu.pc = 2105508u32;
    emu.update_insn_clock();
    emu.adi_no_count(20usize, 20usize, 4294967116u32, 2105512u32);
    emu.adi_no_count(22usize, 0usize, 1u32, 2105516u32);
    emu.adi_no_count(23usize, 0usize, 35u32, 2105520u32);
    emu.adi_no_count(24usize, 0usize, 2u32, 2105524u32);
    emu.add_memory_rw_events(10usize);
    let return_addr = 2105528u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2105544u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002020c8));
}
#[inline(always)]
pub fn block_0x002020b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 4u32, 2105532u32)?;
    emu.lbu_no_count(11usize, 11usize, 8u32, 2105536u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2105680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202150));
    } else {
        emu.pc = 2105540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002020c4));
    }
}
#[inline(always)]
pub fn block_0x002020c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2105664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202140));
    } else {
        emu.pc = 2105544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002020c8));
    }
}
#[inline(always)]
pub fn block_0x002020c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2105548u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2105552u32);
    emu.adi_no_count(12usize, 18usize, 0u32, 2105556u32);
    emu.adi_no_count(13usize, 9usize, 0u32, 2105560u32);
    emu.apc_no_count(1usize, 2105560u32, 4096u32, 2105564u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2105568u32;
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
#[inline(always)]
pub fn block_0x002020e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 2usize, 0u32, 2105572u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2105600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202100));
    } else {
        emu.pc = 2105576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002020e8));
    }
}
#[inline(always)]
pub fn block_0x002020e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 4u32, 2105580u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2105676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020214c));
    } else {
        emu.pc = 2105584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002020f0));
    }
}
#[inline(always)]
pub fn block_0x002020f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a < b {
        emu.pc = 2105744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202190));
    } else {
        emu.pc = 2105588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002020f4));
    }
}
#[inline(always)]
pub fn block_0x002020f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(9usize, 9usize, 10usize, 2105592u32);
    emu.adr_no_count(18usize, 18usize, 10usize, 2105596u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2105600u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2105540u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002020c4));
}
#[inline(always)]
pub fn block_0x00202100(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2105604u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(22usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2105648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202130));
    } else {
        emu.pc = 2105608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202108));
    }
}
#[inline(always)]
pub fn block_0x00202108(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2105528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002020b8));
    } else {
        emu.pc = 2105612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020210c));
    }
}
#[inline(always)]
pub fn block_0x0020210c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 4u32, 2105616u32)?;
    emu.lbu_no_count(12usize, 11usize, 8u32, 2105620u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2105680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202150));
    } else {
        emu.pc = 2105624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202118));
    }
}
#[inline(always)]
pub fn block_0x00202118(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 11usize, 4u32, 2105628u32)?;
    emu.lw_no_count(12usize, 10usize, 0u32, 2105632u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2105540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002020c4));
    } else {
        emu.pc = 2105636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202124));
    }
}
#[inline(always)]
pub fn block_0x00202124(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 11usize, 0u32, 2105640u32)?;
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2105644u32;
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
pub fn block_0x0020212c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2105648u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2105540u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002020c4));
}
#[inline(always)]
pub fn block_0x00202130(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2105680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202150));
    } else {
        emu.pc = 2105652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202134));
    }
}
#[inline(always)]
pub fn block_0x00202134(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 2usize, 1u32, 2105656u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2105540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002020c4));
    } else {
        emu.pc = 2105660u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020213c));
    }
}
#[inline(always)]
pub fn block_0x0020213c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2105664u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2105680u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202150));
}
#[inline(always)]
pub fn block_0x00202140(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2105668u32);
    emu.sb_no_count(10usize, 8usize, 0u32, 2105672u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2105676u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2105696u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202160));
}
#[inline(always)]
pub fn block_0x0020214c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 20usize, 0u32, 2105680u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2105680u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202150));
}
#[inline(always)]
pub fn block_0x00202150(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2105684u32)?;
    emu.lw_no_count(10usize, 10usize, 4u32, 2105688u32)?;
    emu.sw_no_count(11usize, 8usize, 0u32, 2105692u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2105696u32)?;
    emu.add_memory_rw_events(4usize);
    emu.pc = 2105696u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202160));
}
#[inline]
pub fn block_0x00202160(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2105700u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2105704u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2105708u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2105712u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2105716u32)?;
    emu.lw_no_count(20usize, 2usize, 24u32, 2105720u32)?;
    emu.lw_no_count(21usize, 2usize, 20u32, 2105724u32)?;
    emu.lw_no_count(22usize, 2usize, 16u32, 2105728u32)?;
    emu.lw_no_count(23usize, 2usize, 12u32, 2105732u32)?;
    emu.lw_no_count(24usize, 2usize, 8u32, 2105736u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2105740u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2105744u32;
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
pub fn block_0x00202190(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2105748u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967124u32, 2105752u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2105756u32);
    emu.apc_no_count(1usize, 2105756u32, 73728u32, 2105760u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2105764u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965316u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002021a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 8u32, 2105768u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2105772u32)?;
    emu.sli_no_count(13usize, 12usize, 6u32, 2105776u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2105796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002021c4));
    } else {
        emu.pc = 2105780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002021b4));
    }
}
#[inline(always)]
pub fn block_0x002021b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 5u32, 2105784u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2105804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002021cc));
    } else {
        emu.pc = 2105788u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002021bc));
    }
}
#[inline(always)]
pub fn block_0x002021bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2105788u32, 36864u32, 2105792u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2105796u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1912u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002021c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2105796u32, 36864u32, 2105800u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2105804u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1308u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002021cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2105804u32, 36864u32, 2105808u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2105812u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1440u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002021d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2105816u32)?;
    emu.lbu_no_count(13usize, 12usize, 0u32, 2105820u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2105824u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2105888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202220));
    } else {
        emu.pc = 2105828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002021e4));
    }
}
#[inline]
pub fn block_0x002021e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2105832u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2105836u32)?;
    emu.adi_no_count(12usize, 12usize, 1u32, 2105840u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2105844u32)?;
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2105848u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 88u32, 2105852u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2105856u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294967196u32, 2105860u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2105864u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2105868u32);
    emu.apc_no_count(1usize, 2105868u32, 61440u32, 2105872u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2105876u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966240u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202214(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2105880u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2105884u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2105888u32;
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
pub fn block_0x00202220(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2105892u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 108u32, 2105896u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2105900u32);
    emu.apc_no_count(6usize, 2105900u32, 61440u32, 2105904u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2105908u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965692u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202234(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 8u32, 2105912u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2105916u32)?;
    emu.sli_no_count(13usize, 12usize, 6u32, 2105920u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2105940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202254));
    } else {
        emu.pc = 2105924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202244));
    }
}
#[inline(always)]
pub fn block_0x00202244(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 5u32, 2105928u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2105948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020225c));
    } else {
        emu.pc = 2105932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020224c));
    }
}
#[inline(always)]
pub fn block_0x0020224c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2105932u32, 36864u32, 2105936u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2105940u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1968u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202254(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2105940u32, 36864u32, 2105944u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2105948u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1444u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020225c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2105948u32, 36864u32, 2105952u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2105956u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1576u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202264(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2105960u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294967024u32, 2105964u32);
    emu.adi_no_count(12usize, 0usize, 17u32, 2105968u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2105972u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2105976u32);
    emu.apc_no_count(6usize, 2105976u32, 61440u32, 2105980u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2105984u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965616u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202280(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2105988u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2105992u32)?;
    emu.lw_no_count(12usize, 10usize, 0u32, 2105996u32)?;
    emu.lw_no_count(13usize, 12usize, 0u32, 2106000u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2106004u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2106036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002022b4));
    } else {
        emu.pc = 2106008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202298));
    }
}
#[inline(always)]
pub fn block_0x00202298(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 8u32, 2106012u32);
    emu.sli_no_count(11usize, 11usize, 2u32, 2106016u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2106020u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966912u32, 2106024u32);
    emu.adr_no_count(11usize, 11usize, 13usize, 2106028u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2106032u32)?;
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2106036u32;
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
pub fn block_0x002022b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2106040u32;
    emu.update_insn_clock();
    emu.xrr_no_count(11usize, 13usize, 11usize, 2106044u32);
    emu.sli_no_count(11usize, 11usize, 2u32, 2106048u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2106052u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966912u32, 2106056u32);
    emu.adr_no_count(11usize, 11usize, 13usize, 2106060u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2106064u32)?;
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2106068u32;
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
pub fn block_0x002022d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4u32, 2106072u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2106076u32)?;
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2106080u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967156u32, 2106084u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2106088u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294967140u32, 2106092u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2106096u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2106100u32);
    emu.apc_no_count(1usize, 2106100u32, 61440u32, 2106104u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106108u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966008u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002022fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2106112u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2106116u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106120u32;
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
pub fn block_0x00202308(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4u32, 2106124u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2106128u32)?;
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2106132u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967268u32, 2106136u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2106140u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294967252u32, 2106144u32);
    emu.adi_no_count(12usize, 0usize, 18u32, 2106148u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2106152u32);
    emu.apc_no_count(1usize, 2106152u32, 61440u32, 2106156u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106160u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965956u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202330(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2106164u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2106168u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106172u32;
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
pub fn block_0x0020233c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4u32, 2106176u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2106180u32)?;
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2106184u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967212u32, 2106188u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2106192u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294967196u32, 2106196u32);
    emu.adi_no_count(12usize, 0usize, 19u32, 2106200u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2106204u32);
    emu.apc_no_count(1usize, 2106204u32, 61440u32, 2106208u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106212u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965904u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202364(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2106216u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2106220u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106224u32;
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
pub fn block_0x00202370(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2106228u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967231u32, 2106232u32);
    emu.adi_no_count(12usize, 0usize, 19u32, 2106236u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2106240u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2106244u32);
    emu.apc_no_count(6usize, 2106244u32, 61440u32, 2106248u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2106252u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x0020238c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2106256u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 25u32, 2106260u32);
    emu.adi_no_count(12usize, 0usize, 22u32, 2106264u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2106268u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2106272u32);
    emu.apc_no_count(6usize, 2106272u32, 61440u32, 2106276u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2106280u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965320u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002023a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4u32, 2106284u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2106288u32)?;
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2106292u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967176u32, 2106296u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2106300u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294967160u32, 2106304u32);
    emu.adi_no_count(12usize, 0usize, 19u32, 2106308u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2106312u32);
    emu.apc_no_count(1usize, 2106312u32, 61440u32, 2106316u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106320u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965796u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002023d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2106324u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2106328u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106332u32;
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
pub fn block_0x002023dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2106336u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967286u32, 2106340u32);
    emu.adi_no_count(12usize, 0usize, 26u32, 2106344u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2106348u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2106352u32);
    emu.apc_no_count(6usize, 2106352u32, 57344u32, 2106356u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2106360u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(2040u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002023f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2106364u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 16u32, 2106368u32);
    emu.adi_no_count(12usize, 0usize, 9u32, 2106372u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2106376u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2106380u32);
    emu.apc_no_count(6usize, 2106380u32, 57344u32, 2106384u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2106388u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(2012u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00202414(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(12usize, 2usize, 8u32, 2106392u32)?;
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2106396u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 64u32, 2106400u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2106404u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 48u32, 2106408u32);
    emu.adi_no_count(12usize, 0usize, 6u32, 2106412u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2106416u32);
    emu.apc_no_count(1usize, 2106416u32, 61440u32, 2106420u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106424u32;
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
#[inline(always)]
pub fn block_0x00202438(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2106428u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2106432u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106436u32;
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
pub fn block_0x00202444(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2106440u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2106444u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2106448u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2106452u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2106456u32)?;
    emu.adi_no_count(11usize, 10usize, 0u32, 2106460u32);
    emu.lw_no_count(12usize, 10usize, 4u32, 2106464u32)?;
    emu.lw_no_count(10usize, 10usize, 12u32, 2106468u32)?;
    emu.adi_no_count(13usize, 0usize, 1u32, 2106472u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2106528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002024a0));
    } else {
        emu.pc = 2106476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020246c));
    }
}
#[inline(always)]
pub fn block_0x0020246c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2106532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002024a4));
    } else {
        emu.pc = 2106480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202470));
    }
}
#[inline(always)]
pub fn block_0x00202470(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2106532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002024a4));
    } else {
        emu.pc = 2106484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202474));
    }
}
#[inline(always)]
pub fn block_0x00202474(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 0usize, 0u32, 2106488u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2106492u32);
    emu.adi_no_count(9usize, 0usize, 1u32, 2106496u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2106496u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202480));
}
#[inline(always)]
pub fn block_0x00202480(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2106500u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2106504u32);
    emu.apc_no_count(1usize, 2106504u32, 4096u32, 2106508u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106512u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965860u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202490(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(8usize, 2usize, 4u32, 2106516u32)?;
    emu.sw_no_count(9usize, 2usize, 8u32, 2106520u32)?;
    emu.sw_no_count(8usize, 2usize, 12u32, 2106524u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2106528u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2106544u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002024b0));
}
#[inline(always)]
pub fn block_0x002024a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2106628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202504));
    } else {
        emu.pc = 2106532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002024a4));
    }
}
#[inline(always)]
pub fn block_0x002024a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 4u32, 2106536u32);
    emu.apc_no_count(1usize, 2106536u32, 36864u32, 2106540u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106544u32;
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
pub fn block_0x002024b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2106544u32, 0u32, 2106548u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106552u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(220u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002024b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2183168u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2106556u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 644u32, 2106560u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2106564u32);
    emu.adi_no_count(12usize, 0usize, 12u32, 2106568u32);
    emu.apc_no_count(1usize, 2106568u32, 4096u32, 2106572u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106576u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966860u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002024d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2106664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202528));
    } else {
        emu.pc = 2106580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002024d4));
    }
}
#[inline]
pub fn block_0x002024d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 4u32, 2106584u32)?;
    emu.lw_no_count(12usize, 2usize, 8u32, 2106588u32)?;
    emu.lw_no_count(13usize, 2usize, 12u32, 2106592u32)?;
    emu.sw_no_count(11usize, 10usize, 0u32, 2106596u32)?;
    emu.sw_no_count(12usize, 10usize, 4u32, 2106600u32)?;
    emu.sw_no_count(13usize, 10usize, 8u32, 2106604u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2106608u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2106612u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2106616u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2106620u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2106624u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106628u32;
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
pub fn block_0x00202504(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 11usize, 0u32, 2106632u32)?;
    emu.lw_no_count(8usize, 10usize, 4u32, 2106636u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(8usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2106680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202538));
    } else {
        emu.pc = 2106640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202510));
    }
}
#[inline(always)]
pub fn block_0x00202510(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2106644u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2106644u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202514));
}
#[inline(always)]
pub fn block_0x00202514(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2106648u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 116u32, 2106652u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2106656u32);
    emu.apc_no_count(1usize, 2106656u32, 36864u32, 2106660u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106664u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966264u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202528(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2106668u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2106672u32);
    emu.apc_no_count(1usize, 2106672u32, 36864u32, 2106676u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106680u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966272u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202538(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 10usize, 0u32, 2106684u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2106736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202570));
    } else {
        emu.pc = 2106688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202540));
    }
}
#[inline(always)]
pub fn block_0x00202540(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2106688u32, 0u32, 2106692u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106696u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(76u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202548(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2183168u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2106700u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 644u32, 2106704u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2106708u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2106712u32);
    emu.apc_no_count(1usize, 2106712u32, 4096u32, 2106716u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106720u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966716u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202560(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2106724u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2106728u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2106740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202574));
    } else {
        emu.pc = 2106732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020256c));
    }
}
#[inline(always)]
pub fn block_0x0020256c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2106736u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2106644u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202514));
}
#[inline(always)]
pub fn block_0x00202570(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 1u32, 2106740u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2106740u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202574));
}
#[inline(always)]
pub fn block_0x00202574(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 18usize, 0u32, 2106744u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2106748u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2106496u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202480));
}
#[inline(always)]
pub fn block_0x0020257c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2106748u32, 28672u32, 2106752u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2106756u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966948u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202584(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2106760u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106764u32;
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
pub fn block_0x0020258c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106768u32;
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
pub fn block_0x00202590(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2106772u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2106776u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2106780u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2106784u32)?;
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2106788u32;
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
pub fn block_0x002025a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(9usize, 11usize, 12u32, 2106792u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2106796u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2106800u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2106804u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(9usize);
    let return_addr = 2106808u32;
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
#[inline]
pub fn block_0x002025b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2106812u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2106816u32)?;
    emu.lw_no_count(12usize, 2usize, 0u32, 2106820u32)?;
    emu.lw_no_count(13usize, 2usize, 4u32, 2106824u32)?;
    let a = 0u32.wrapping_add(3112902656u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2106828u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966129u32, 2106832u32);
    emu.xrr_no_count(11usize, 11usize, 14usize, 2106836u32);
    let a = 0u32.wrapping_add(1676365824u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2106840u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 44u32, 2106844u32);
    emu.xrr_no_count(13usize, 13usize, 14usize, 2106848u32);
    let a = 0u32.wrapping_add(1470513152u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2106852u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 376u32, 2106856u32);
    emu.xrr_no_count(10usize, 10usize, 14usize, 2106860u32);
    let a = 0u32.wrapping_add(3603652608u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2106864u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966637u32, 2106868u32);
    emu.xrr_no_count(12usize, 12usize, 14usize, 2106872u32);
    emu.orr_no_count(11usize, 13usize, 11usize, 2106876u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2106880u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2106884u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2106912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202620));
    } else {
        emu.pc = 2106888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202608));
    }
}
#[inline(always)]
pub fn block_0x00202608(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 4u32, 2106892u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2106896u32);
    emu.adr_no_count(11usize, 8usize, 11usize, 2106900u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2106904u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2107024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202690));
    } else {
        emu.pc = 2106908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020261c));
    }
}
#[inline(always)]
pub fn block_0x0020261c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2106912u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2107044u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002026a4));
}
#[inline(always)]
pub fn block_0x00202620(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2106916u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2106920u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(9usize);
    let return_addr = 2106924u32;
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
#[inline]
pub fn block_0x0020262c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2106928u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2106932u32)?;
    emu.lw_no_count(12usize, 2usize, 0u32, 2106936u32)?;
    emu.lw_no_count(13usize, 2usize, 4u32, 2106940u32)?;
    let a = 0u32.wrapping_add(2330587136u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2106944u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 428u32, 2106948u32);
    emu.xrr_no_count(11usize, 11usize, 14usize, 2106952u32);
    let a = 0u32.wrapping_add(2965131264u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2106956u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965685u32, 2106960u32);
    emu.xrr_no_count(13usize, 13usize, 14usize, 2106964u32);
    let a = 0u32.wrapping_add(4112453632u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2106968u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1473u32, 2106972u32);
    emu.xrr_no_count(10usize, 10usize, 14usize, 2106976u32);
    let a = 0u32.wrapping_add(2020298752u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2106980u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965580u32, 2106984u32);
    emu.xrr_no_count(12usize, 12usize, 14usize, 2106988u32);
    emu.orr_no_count(11usize, 13usize, 11usize, 2106992u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2106996u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2107000u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2107024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202690));
    } else {
        emu.pc = 2107004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020267c));
    }
}
#[inline(always)]
pub fn block_0x0020267c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 4u32, 2107008u32);
    emu.adi_no_count(11usize, 0usize, 8u32, 2107012u32);
    emu.adr_no_count(11usize, 8usize, 11usize, 2107016u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2107020u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2107044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002026a4));
    } else {
        emu.pc = 2107024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202690));
    }
}
#[inline(always)]
pub fn block_0x00202690(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2107028u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2107032u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2107036u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2107040u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107044u32;
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
pub fn block_0x002026a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2107048u32)?;
    emu.apc_no_count(1usize, 2107048u32, 0u32, 2107052u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107056u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1820u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002026b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2107060u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2107064u32)?;
    emu.lw_no_count(11usize, 11usize, 20u32, 2107068u32)?;
    emu.apc_no_count(1usize, 2107068u32, 0u32, 2107072u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107076u32;
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
pub fn block_0x002026c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2107076u32, 32768u32, 2107080u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107084u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966904u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002026cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2107088u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2107092u32)?;
    emu.apc_no_count(1usize, 2107092u32, 4096u32, 2107096u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107100u32;
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
#[inline(always)]
pub fn block_0x002026dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2107104u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2107108u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107112u32;
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
pub fn block_0x002026e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967088u32, 2107116u32);
    emu.sw_no_count(1usize, 2usize, 204u32, 2107120u32)?;
    emu.sw_no_count(8usize, 2usize, 200u32, 2107124u32)?;
    emu.sw_no_count(9usize, 2usize, 196u32, 2107128u32)?;
    emu.sw_no_count(18usize, 2usize, 192u32, 2107132u32)?;
    emu.sw_no_count(19usize, 2usize, 188u32, 2107136u32)?;
    emu.sw_no_count(20usize, 2usize, 184u32, 2107140u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2107144u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2107148u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 10usize, 4294965520u32, 2107152u32)?;
    emu.sw_no_count(0usize, 10usize, 4294965524u32, 2107156u32)?;
    emu.ani_no_count(11usize, 11usize, 1u32, 2107160u32);
    emu.sw_no_count(0usize, 10usize, 4294965520u32, 2107164u32)?;
    emu.add_memory_rw_events(13usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2108480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202c40));
    } else {
        emu.pc = 2107168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202720));
    }
}
#[inline(always)]
pub fn block_0x00202720(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2107172u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965528u32, 2107176u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2107180u32);
    emu.adi_no_count(12usize, 0usize, 112u32, 2107184u32);
    emu.apc_no_count(1usize, 2107184u32, 0u32, 2107188u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107192u32;
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
#[inline(never)]
pub fn block_0x00202738(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 35u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 2usize, 48u32, 2107196u32);
    emu.lbu_no_count(18usize, 2usize, 112u32, 2107200u32);
    emu.lw_no_count(10usize, 2usize, 40u32, 2107204u32)?;
    emu.lw_no_count(11usize, 2usize, 44u32, 2107208u32)?;
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2107212u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 0usize, 128u32, 2107216u32);
    emu.adi_no_count(12usize, 12usize, 4294967040u32, 2107220u32);
    emu.sli_no_count(14usize, 11usize, 9u32, 2107224u32);
    emu.sli_no_count(15usize, 10usize, 9u32, 2107228u32);
    emu.sli_no_count(16usize, 18usize, 3u32, 2107232u32);
    emu.sli_no_count(17usize, 10usize, 1u32, 2107236u32);
    emu.sli_no_count(11usize, 11usize, 1u32, 2107240u32);
    emu.orr_no_count(16usize, 15usize, 16usize, 2107244u32);
    emu.anr_no_count(17usize, 17usize, 12usize, 2107248u32);
    emu.sri_no_count(15usize, 15usize, 24u32, 2107252u32);
    emu.anr_no_count(11usize, 11usize, 12usize, 2107256u32);
    emu.orr_no_count(15usize, 17usize, 15usize, 2107260u32);
    emu.sri_no_count(17usize, 14usize, 24u32, 2107264u32);
    emu.orr_no_count(11usize, 11usize, 17usize, 2107268u32);
    emu.adi_no_count(17usize, 0usize, 63u32, 2107272u32);
    emu.sri_no_count(5usize, 10usize, 23u32, 2107276u32);
    emu.orr_no_count(10usize, 14usize, 5usize, 2107280u32);
    emu.anr_no_count(14usize, 16usize, 12usize, 2107284u32);
    emu.anr_no_count(12usize, 10usize, 12usize, 2107288u32);
    emu.sli_no_count(10usize, 18usize, 27u32, 2107292u32);
    emu.orr_no_count(15usize, 10usize, 15usize, 2107296u32);
    emu.adr_no_count(10usize, 9usize, 18usize, 2107300u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2107304u32);
    emu.sli_no_count(14usize, 14usize, 8u32, 2107308u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2107312u32);
    emu.orr_no_count(11usize, 5usize, 11usize, 2107316u32);
    emu.orr_no_count(20usize, 15usize, 14usize, 2107320u32);
    emu.orr_no_count(19usize, 11usize, 12usize, 2107324u32);
    emu.sb_no_count(13usize, 10usize, 0u32, 2107328u32);
    emu.add_memory_rw_events(34usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2107364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002027e4));
    } else {
        emu.pc = 2107332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002027c4));
    }
}
#[inline(always)]
pub fn block_0x002027c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 10usize, 1u32, 2107336u32);
    emu.xri_no_count(12usize, 18usize, 63u32, 2107340u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2107344u32);
    emu.apc_no_count(1usize, 2107344u32, 0u32, 2107348u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107352u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1572u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002027d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(10usize, 18usize, 56u32, 2107356u32);
    emu.adi_no_count(11usize, 0usize, 7u32, 2107360u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2107476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202854));
    } else {
        emu.pc = 2107364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002027e4));
    }
}
#[inline(always)]
pub fn block_0x002027e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 8u32, 2107368u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2107372u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2107376u32);
    emu.apc_no_count(1usize, 2107376u32, 4096u32, 2107380u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107384u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967052u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002027f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 120u32, 2107388u32);
    emu.adi_no_count(12usize, 0usize, 56u32, 2107392u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2107396u32);
    emu.apc_no_count(1usize, 2107396u32, 0u32, 2107400u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107404u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1520u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020280c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 20usize, 24u32, 2107408u32);
    emu.sri_no_count(11usize, 20usize, 16u32, 2107412u32);
    emu.sri_no_count(12usize, 20usize, 8u32, 2107416u32);
    emu.sri_no_count(13usize, 19usize, 24u32, 2107420u32);
    emu.sri_no_count(14usize, 19usize, 16u32, 2107424u32);
    emu.sb_no_count(20usize, 2usize, 180u32, 2107428u32);
    emu.sb_no_count(12usize, 2usize, 181u32, 2107432u32);
    emu.sb_no_count(11usize, 2usize, 182u32, 2107436u32);
    emu.sb_no_count(10usize, 2usize, 183u32, 2107440u32);
    emu.sri_no_count(10usize, 19usize, 8u32, 2107444u32);
    emu.sb_no_count(19usize, 2usize, 176u32, 2107448u32);
    emu.sb_no_count(10usize, 2usize, 177u32, 2107452u32);
    emu.sb_no_count(14usize, 2usize, 178u32, 2107456u32);
    emu.sb_no_count(13usize, 2usize, 179u32, 2107460u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2107464u32);
    emu.adi_no_count(11usize, 2usize, 120u32, 2107468u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2107472u32);
    emu.add_memory_rw_events(18usize);
    let return_addr = 2107476u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2107496u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202868));
}
#[inline(always)]
pub fn block_0x00202854(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(19usize, 2usize, 104u32, 2107480u32)?;
    emu.sw_no_count(20usize, 2usize, 108u32, 2107484u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2107488u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2107492u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2107496u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2107496u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202868));
}
#[inline(always)]
pub fn block_0x00202868(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2107496u32, 4096u32, 2107500u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107504u32;
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
pub fn block_0x00202870(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 85u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2107508u32);
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2107512u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 2usize, 8u32, 2107516u32)?;
    emu.lw_no_count(13usize, 2usize, 12u32, 2107520u32)?;
    emu.lw_no_count(15usize, 2usize, 16u32, 2107524u32)?;
    emu.lw_no_count(16usize, 2usize, 20u32, 2107528u32)?;
    emu.lw_no_count(17usize, 2usize, 24u32, 2107532u32)?;
    emu.lw_no_count(5usize, 2usize, 28u32, 2107536u32)?;
    emu.lw_no_count(6usize, 2usize, 32u32, 2107540u32)?;
    emu.lw_no_count(14usize, 2usize, 36u32, 2107544u32)?;
    emu.adi_no_count(12usize, 12usize, 4294967040u32, 2107548u32);
    emu.sri_no_count(7usize, 11usize, 8u32, 2107552u32);
    emu.sri_no_count(28usize, 11usize, 24u32, 2107556u32);
    emu.anr_no_count(29usize, 11usize, 12usize, 2107560u32);
    emu.sli_no_count(11usize, 11usize, 24u32, 2107564u32);
    emu.sri_no_count(30usize, 13usize, 8u32, 2107568u32);
    emu.sri_no_count(31usize, 13usize, 24u32, 2107572u32);
    emu.anr_no_count(9usize, 13usize, 12usize, 2107576u32);
    emu.sli_no_count(13usize, 13usize, 24u32, 2107580u32);
    emu.sri_no_count(18usize, 15usize, 8u32, 2107584u32);
    emu.sri_no_count(19usize, 15usize, 24u32, 2107588u32);
    emu.anr_no_count(20usize, 15usize, 12usize, 2107592u32);
    emu.sli_no_count(15usize, 15usize, 24u32, 2107596u32);
    emu.anr_no_count(7usize, 7usize, 12usize, 2107600u32);
    emu.orr_no_count(7usize, 7usize, 28usize, 2107604u32);
    emu.sri_no_count(28usize, 16usize, 8u32, 2107608u32);
    emu.sli_no_count(29usize, 29usize, 8u32, 2107612u32);
    emu.orr_no_count(11usize, 11usize, 29usize, 2107616u32);
    emu.sri_no_count(29usize, 16usize, 24u32, 2107620u32);
    emu.anr_no_count(30usize, 30usize, 12usize, 2107624u32);
    emu.orr_no_count(30usize, 30usize, 31usize, 2107628u32);
    emu.anr_no_count(31usize, 16usize, 12usize, 2107632u32);
    emu.sli_no_count(16usize, 16usize, 24u32, 2107636u32);
    emu.sli_no_count(9usize, 9usize, 8u32, 2107640u32);
    emu.orr_no_count(13usize, 13usize, 9usize, 2107644u32);
    emu.sri_no_count(9usize, 17usize, 8u32, 2107648u32);
    emu.anr_no_count(18usize, 18usize, 12usize, 2107652u32);
    emu.orr_no_count(18usize, 18usize, 19usize, 2107656u32);
    emu.sri_no_count(19usize, 17usize, 24u32, 2107660u32);
    emu.sli_no_count(20usize, 20usize, 8u32, 2107664u32);
    emu.orr_no_count(15usize, 15usize, 20usize, 2107668u32);
    emu.anr_no_count(20usize, 17usize, 12usize, 2107672u32);
    emu.sli_no_count(17usize, 17usize, 24u32, 2107676u32);
    emu.anr_no_count(28usize, 28usize, 12usize, 2107680u32);
    emu.orr_no_count(28usize, 28usize, 29usize, 2107684u32);
    emu.sri_no_count(29usize, 5usize, 8u32, 2107688u32);
    emu.sli_no_count(31usize, 31usize, 8u32, 2107692u32);
    emu.orr_no_count(16usize, 16usize, 31usize, 2107696u32);
    emu.sri_no_count(31usize, 5usize, 24u32, 2107700u32);
    emu.anr_no_count(9usize, 9usize, 12usize, 2107704u32);
    emu.orr_no_count(9usize, 9usize, 19usize, 2107708u32);
    emu.anr_no_count(19usize, 5usize, 12usize, 2107712u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2107716u32);
    emu.sli_no_count(20usize, 20usize, 8u32, 2107720u32);
    emu.orr_no_count(17usize, 17usize, 20usize, 2107724u32);
    emu.sri_no_count(20usize, 6usize, 8u32, 2107728u32);
    emu.anr_no_count(29usize, 29usize, 12usize, 2107732u32);
    emu.orr_no_count(29usize, 29usize, 31usize, 2107736u32);
    emu.sri_no_count(31usize, 6usize, 24u32, 2107740u32);
    emu.sli_no_count(19usize, 19usize, 8u32, 2107744u32);
    emu.orr_no_count(5usize, 5usize, 19usize, 2107748u32);
    emu.anr_no_count(19usize, 6usize, 12usize, 2107752u32);
    emu.sli_no_count(6usize, 6usize, 24u32, 2107756u32);
    emu.anr_no_count(20usize, 20usize, 12usize, 2107760u32);
    emu.orr_no_count(31usize, 20usize, 31usize, 2107764u32);
    emu.sri_no_count(20usize, 14usize, 8u32, 2107768u32);
    emu.sli_no_count(19usize, 19usize, 8u32, 2107772u32);
    emu.orr_no_count(6usize, 6usize, 19usize, 2107776u32);
    emu.sri_no_count(19usize, 14usize, 24u32, 2107780u32);
    emu.anr_no_count(20usize, 20usize, 12usize, 2107784u32);
    emu.orr_no_count(19usize, 20usize, 19usize, 2107788u32);
    emu.anr_no_count(12usize, 14usize, 12usize, 2107792u32);
    emu.sli_no_count(14usize, 14usize, 24u32, 2107796u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2107800u32);
    emu.orr_no_count(20usize, 14usize, 12usize, 2107804u32);
    emu.orr_no_count(11usize, 11usize, 7usize, 2107808u32);
    emu.orr_no_count(12usize, 13usize, 30usize, 2107812u32);
    emu.orr_no_count(13usize, 15usize, 18usize, 2107816u32);
    emu.orr_no_count(14usize, 16usize, 28usize, 2107820u32);
    emu.orr_no_count(15usize, 17usize, 9usize, 2107824u32);
    emu.orr_no_count(16usize, 5usize, 29usize, 2107828u32);
    emu.orr_no_count(17usize, 6usize, 31usize, 2107832u32);
    emu.adi_no_count(5usize, 0usize, 16u32, 2107836u32);
    emu.orr_no_count(6usize, 20usize, 19usize, 2107840u32);
    emu.add_memory_rw_events(85usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x002029c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2107848u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2107852u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2107856u32);
    emu.add_memory_rw_events(4usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x002029d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2107864u32);
    emu.adi_no_count(10usize, 0usize, 2u32, 2107868u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2107872u32);
    emu.add_memory_rw_events(4usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x002029e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2107880u32);
    emu.adi_no_count(10usize, 0usize, 3u32, 2107884u32);
    emu.adi_no_count(11usize, 14usize, 0u32, 2107888u32);
    emu.add_memory_rw_events(4usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x002029f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2107896u32);
    emu.adi_no_count(10usize, 0usize, 4u32, 2107900u32);
    emu.adi_no_count(11usize, 15usize, 0u32, 2107904u32);
    emu.add_memory_rw_events(4usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x00202a04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2107912u32);
    emu.adi_no_count(10usize, 0usize, 5u32, 2107916u32);
    emu.adi_no_count(11usize, 16usize, 0u32, 2107920u32);
    emu.add_memory_rw_events(4usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x00202a14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2107928u32);
    emu.adi_no_count(10usize, 0usize, 6u32, 2107932u32);
    emu.adi_no_count(11usize, 17usize, 0u32, 2107936u32);
    emu.add_memory_rw_events(4usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x00202a24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2107944u32);
    emu.adi_no_count(10usize, 0usize, 7u32, 2107948u32);
    emu.adi_no_count(11usize, 6usize, 0u32, 2107952u32);
    emu.add_memory_rw_events(4usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x00202a34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2107960u32;
    emu.update_insn_clock();
    emu.lw_no_count(10usize, 10usize, 4294965480u32, 2107964u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2108496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202c50));
    } else {
        emu.pc = 2107968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202a40));
    }
}
#[inline]
pub fn block_0x00202a40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2107972u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2107976u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965480u32, 2107980u32);
    let a = 0u32.wrapping_add(2164260864u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2107984u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 14usize, 4u32, 2107988u32)?;
    let a = 0u32.wrapping_add(2130706432u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2107992u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1u32, 2107996u32);
    emu.adi_no_count(12usize, 12usize, 1u32, 2108000u32);
    emu.mul_no_count(15usize, 11usize, 13usize, 2108004u32);
    emu.mulhu_no_count(16usize, 15usize, 12usize, 2108008u32);
    emu.mul_no_count(15usize, 15usize, 12usize, 2108012u32);
    emu.sltru_no_count(11usize, 11usize, 15usize, 2108016u32);
    emu.sltru_no_count(15usize, 0usize, 16usize, 2108020u32);
    emu.adr_no_count(16usize, 16usize, 11usize, 2108024u32);
    emu.orr_no_count(11usize, 11usize, 15usize, 2108028u32);
    emu.sbr_no_count(11usize, 0usize, 11usize, 2108032u32);
    emu.anr_no_count(11usize, 11usize, 12usize, 2108036u32);
    emu.sbr_no_count(11usize, 11usize, 16usize, 2108040u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2108044u32);
    emu.add_memory_rw_events(20usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline]
pub fn block_0x00202a90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 8u32, 2108052u32)?;
    emu.mul_no_count(11usize, 10usize, 13usize, 2108056u32);
    emu.mulhu_no_count(15usize, 11usize, 12usize, 2108060u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2108064u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2108068u32);
    emu.sltru_no_count(11usize, 0usize, 15usize, 2108072u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2108076u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2108080u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2108084u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2108088u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2108092u32);
    emu.sbr_no_count(11usize, 10usize, 15usize, 2108096u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2108100u32);
    emu.add_memory_rw_events(14usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline]
pub fn block_0x00202ac8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 12u32, 2108108u32)?;
    emu.mul_no_count(11usize, 10usize, 13usize, 2108112u32);
    emu.mulhu_no_count(15usize, 11usize, 12usize, 2108116u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2108120u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2108124u32);
    emu.sltru_no_count(11usize, 0usize, 15usize, 2108128u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2108132u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2108136u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2108140u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2108144u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2108148u32);
    emu.sbr_no_count(11usize, 10usize, 15usize, 2108152u32);
    emu.adi_no_count(10usize, 0usize, 2u32, 2108156u32);
    emu.add_memory_rw_events(14usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline]
pub fn block_0x00202b00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 16u32, 2108164u32)?;
    emu.mul_no_count(11usize, 10usize, 13usize, 2108168u32);
    emu.mulhu_no_count(15usize, 11usize, 12usize, 2108172u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2108176u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2108180u32);
    emu.sltru_no_count(11usize, 0usize, 15usize, 2108184u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2108188u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2108192u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2108196u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2108200u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2108204u32);
    emu.sbr_no_count(11usize, 10usize, 15usize, 2108208u32);
    emu.adi_no_count(10usize, 0usize, 3u32, 2108212u32);
    emu.add_memory_rw_events(14usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline]
pub fn block_0x00202b38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 20u32, 2108220u32)?;
    emu.mul_no_count(11usize, 10usize, 13usize, 2108224u32);
    emu.mulhu_no_count(15usize, 11usize, 12usize, 2108228u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2108232u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2108236u32);
    emu.sltru_no_count(11usize, 0usize, 15usize, 2108240u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2108244u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2108248u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2108252u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2108256u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2108260u32);
    emu.sbr_no_count(11usize, 10usize, 15usize, 2108264u32);
    emu.adi_no_count(10usize, 0usize, 4u32, 2108268u32);
    emu.add_memory_rw_events(14usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline]
pub fn block_0x00202b70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 24u32, 2108276u32)?;
    emu.mul_no_count(11usize, 10usize, 13usize, 2108280u32);
    emu.mulhu_no_count(15usize, 11usize, 12usize, 2108284u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2108288u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2108292u32);
    emu.sltru_no_count(11usize, 0usize, 15usize, 2108296u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2108300u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2108304u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2108308u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2108312u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2108316u32);
    emu.sbr_no_count(11usize, 10usize, 15usize, 2108320u32);
    emu.adi_no_count(10usize, 0usize, 5u32, 2108324u32);
    emu.add_memory_rw_events(14usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline]
pub fn block_0x00202ba8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 28u32, 2108332u32)?;
    emu.mul_no_count(11usize, 10usize, 13usize, 2108336u32);
    emu.mulhu_no_count(15usize, 11usize, 12usize, 2108340u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2108344u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2108348u32);
    emu.sltru_no_count(11usize, 0usize, 15usize, 2108352u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2108356u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2108360u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2108364u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2108368u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2108372u32);
    emu.sbr_no_count(11usize, 10usize, 15usize, 2108376u32);
    emu.adi_no_count(10usize, 0usize, 6u32, 2108380u32);
    emu.add_memory_rw_events(14usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline]
pub fn block_0x00202be0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 32u32, 2108388u32)?;
    emu.adi_no_count(5usize, 0usize, 26u32, 2108392u32);
    emu.mul_no_count(11usize, 10usize, 13usize, 2108396u32);
    emu.mulhu_no_count(13usize, 11usize, 12usize, 2108400u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2108404u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2108408u32);
    emu.sltru_no_count(11usize, 0usize, 13usize, 2108412u32);
    emu.adr_no_count(13usize, 13usize, 10usize, 2108416u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2108420u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2108424u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2108428u32);
    emu.sbr_no_count(11usize, 10usize, 13usize, 2108432u32);
    emu.adi_no_count(10usize, 0usize, 7u32, 2108436u32);
    emu.add_memory_rw_events(14usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x00202c18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 0u32, 2108444u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2108448u32);
    emu.add_memory_rw_events(3usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x00202c24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2108456u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 132u32, 2108460u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2108464u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 204u32, 2108468u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2108472u32);
    emu.apc_no_count(1usize, 2108472u32, 36864u32, 2108476u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108480u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1460u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202c40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2108484u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 172u32, 2108488u32);
    emu.apc_no_count(1usize, 2108488u32, 45056u32, 2108492u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108496u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(880u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202c50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2108500u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 188u32, 2108504u32);
    emu.apc_no_count(1usize, 2108504u32, 45056u32, 2108508u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108512u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(864u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
