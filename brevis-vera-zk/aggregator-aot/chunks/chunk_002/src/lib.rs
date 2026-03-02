pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2105760u32;
pub const PC_MAX: u32 = 2110184u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 107usize] = [
        block_0x002021a0,
        block_0x002021c4,
        block_0x002021fc,
        block_0x00202250,
        block_0x00202264,
        block_0x002022a8,
        block_0x002022bc,
        block_0x00202330,
        block_0x00202340,
        block_0x00202378,
        block_0x002023c8,
        block_0x002023d4,
        block_0x002023e4,
        block_0x002023f4,
        block_0x00202410,
        block_0x00202468,
        block_0x00202480,
        block_0x00202498,
        block_0x002024d4,
        block_0x002024e4,
        block_0x002024f0,
        block_0x00202504,
        block_0x00202508,
        block_0x00202510,
        block_0x0020251c,
        block_0x00202530,
        block_0x00202548,
        block_0x00202564,
        block_0x0020257c,
        block_0x002025a8,
        block_0x00202644,
        block_0x00202658,
        block_0x00202668,
        block_0x00202678,
        block_0x00202688,
        block_0x002026a0,
        block_0x002026b4,
        block_0x002026c8,
        block_0x002026dc,
        block_0x002026f0,
        block_0x00202708,
        block_0x00202718,
        block_0x0020271c,
        block_0x00202728,
        block_0x00202738,
        block_0x002028a8,
        block_0x002028e0,
        block_0x002028f0,
        block_0x00202928,
        block_0x0020297c,
        block_0x002029d0,
        block_0x002029e4,
        block_0x00202a28,
        block_0x00202a3c,
        block_0x00202ab0,
        block_0x00202b40,
        block_0x00202b50,
        block_0x00202b60,
        block_0x00202b78,
        block_0x00202b80,
        block_0x00202b94,
        block_0x00202ba8,
        block_0x00202bbc,
        block_0x00202bd0,
        block_0x00202be8,
        block_0x00202bf4,
        block_0x00202c10,
        block_0x00202c6c,
        block_0x00202c78,
        block_0x00202c88,
        block_0x00202c8c,
        block_0x00202cb8,
        block_0x00202ce8,
        block_0x00202d34,
        block_0x00202d40,
        block_0x00202d44,
        block_0x00202d58,
        block_0x00202d80,
        block_0x00202d88,
        block_0x00202da0,
        block_0x00202da8,
        block_0x00202db4,
        block_0x00202dbc,
        block_0x00202dd0,
        block_0x00202dd8,
        block_0x00202dfc,
        block_0x00202e14,
        block_0x00202ee8,
        block_0x00202efc,
        block_0x00202f08,
        block_0x00202f1c,
        block_0x00202f30,
        block_0x00202f78,
        block_0x00202f8c,
        block_0x00202f94,
        block_0x002031d0,
        block_0x002031dc,
        block_0x002031f8,
        block_0x00203210,
        block_0x00203228,
        block_0x00203240,
        block_0x0020327c,
        block_0x002032a4,
        block_0x002032b0,
        block_0x002032c0,
        block_0x002032dc,
        block_0x002032e8,
    ];
    const IDX: [u16; 1107usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 3u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 4u16, 0u16, 0u16, 0u16, 0u16, 5u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 6u16, 0u16, 0u16, 0u16, 0u16, 7u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 8u16, 0u16, 0u16, 0u16,
        9u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 10u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 11u16, 0u16, 0u16, 12u16, 0u16,
        0u16, 0u16, 13u16, 0u16, 0u16, 0u16, 14u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        15u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 16u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 17u16, 0u16, 0u16, 0u16, 0u16, 0u16, 18u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 19u16, 0u16, 0u16,
        0u16, 20u16, 0u16, 0u16, 21u16, 0u16, 0u16, 0u16, 0u16, 22u16, 23u16, 0u16,
        24u16, 0u16, 0u16, 25u16, 0u16, 0u16, 0u16, 0u16, 26u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 27u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 28u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 29u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 30u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 31u16,
        0u16, 0u16, 0u16, 0u16, 32u16, 0u16, 0u16, 0u16, 33u16, 0u16, 0u16, 0u16, 34u16,
        0u16, 0u16, 0u16, 35u16, 0u16, 0u16, 0u16, 0u16, 0u16, 36u16, 0u16, 0u16, 0u16,
        0u16, 37u16, 0u16, 0u16, 0u16, 0u16, 38u16, 0u16, 0u16, 0u16, 0u16, 39u16, 0u16,
        0u16, 0u16, 0u16, 40u16, 0u16, 0u16, 0u16, 0u16, 0u16, 41u16, 0u16, 0u16, 0u16,
        42u16, 43u16, 0u16, 0u16, 44u16, 0u16, 0u16, 0u16, 45u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 46u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 47u16, 0u16, 0u16,
        0u16, 48u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 49u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 50u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 51u16, 0u16, 0u16, 0u16, 0u16, 52u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 53u16, 0u16, 0u16, 0u16, 0u16, 54u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 55u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 56u16, 0u16, 0u16, 0u16, 57u16, 0u16, 0u16,
        0u16, 58u16, 0u16, 0u16, 0u16, 0u16, 0u16, 59u16, 0u16, 60u16, 0u16, 0u16, 0u16,
        0u16, 61u16, 0u16, 0u16, 0u16, 0u16, 62u16, 0u16, 0u16, 0u16, 0u16, 63u16, 0u16,
        0u16, 0u16, 0u16, 64u16, 0u16, 0u16, 0u16, 0u16, 0u16, 65u16, 0u16, 0u16, 66u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 67u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 68u16, 0u16, 0u16, 69u16, 0u16, 0u16, 0u16, 70u16, 71u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 72u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 73u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 74u16, 0u16, 0u16, 75u16, 76u16, 0u16, 0u16, 0u16, 0u16, 77u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 78u16, 0u16, 79u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 80u16, 0u16, 81u16, 0u16, 0u16, 82u16, 0u16, 83u16, 0u16, 0u16, 0u16,
        0u16, 84u16, 0u16, 85u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 86u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 87u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 88u16, 0u16, 0u16, 0u16, 0u16, 89u16, 0u16,
        0u16, 90u16, 0u16, 0u16, 0u16, 0u16, 91u16, 0u16, 0u16, 0u16, 0u16, 92u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 93u16, 0u16, 0u16, 0u16, 0u16, 94u16, 0u16, 95u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 96u16, 0u16, 0u16,
        97u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 98u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        99u16, 0u16, 0u16, 0u16, 0u16, 0u16, 100u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        101u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 102u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 103u16,
        0u16, 0u16, 104u16, 0u16, 0u16, 0u16, 105u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        106u16, 0u16, 0u16, 107u16,
    ];
    if pc < 2105760u32 || pc > 2110184u32 {
        return None;
    }
    let word_offset = ((pc - 2105760u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x002021a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294966944u32, 2105764u32);
    emu.sw_no_count(1usize, 2usize, 348u32, 2105768u32)?;
    emu.sw_no_count(8usize, 2usize, 344u32, 2105772u32)?;
    emu.sw_no_count(9usize, 2usize, 340u32, 2105776u32)?;
    emu.adi_no_count(9usize, 12usize, 0u32, 2105780u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2105784u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2105788u32);
    emu.apc_no_count(1usize, 2105788u32, 61440u32, 2105792u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2105796u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966128u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002021c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 2usize, 92u32, 2105800u32)?;
    emu.sw_no_count(0usize, 2usize, 96u32, 2105804u32)?;
    emu.sw_no_count(0usize, 2usize, 100u32, 2105808u32)?;
    emu.sw_no_count(0usize, 2usize, 104u32, 2105812u32)?;
    emu.lbu_no_count(13usize, 2usize, 40u32, 2105816u32);
    emu.sw_no_count(0usize, 2usize, 76u32, 2105820u32)?;
    emu.sw_no_count(0usize, 2usize, 80u32, 2105824u32)?;
    emu.sw_no_count(0usize, 2usize, 84u32, 2105828u32)?;
    emu.sw_no_count(0usize, 2usize, 88u32, 2105832u32)?;
    emu.adi_no_count(10usize, 2usize, 44u32, 2105836u32);
    emu.adi_no_count(11usize, 2usize, 76u32, 2105840u32);
    emu.adi_no_count(12usize, 2usize, 8u32, 2105844u32);
    emu.apc_no_count(1usize, 2105844u32, 61440u32, 2105848u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2105852u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1720u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002021fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 60u32, 2105856u32)?;
    emu.lw_no_count(11usize, 2usize, 64u32, 2105860u32)?;
    emu.lw_no_count(12usize, 2usize, 68u32, 2105864u32)?;
    emu.lw_no_count(13usize, 2usize, 72u32, 2105868u32)?;
    emu.sw_no_count(10usize, 2usize, 192u32, 2105872u32)?;
    emu.sw_no_count(11usize, 2usize, 196u32, 2105876u32)?;
    emu.sw_no_count(12usize, 2usize, 200u32, 2105880u32)?;
    emu.sw_no_count(13usize, 2usize, 204u32, 2105884u32)?;
    emu.lw_no_count(10usize, 2usize, 44u32, 2105888u32)?;
    emu.lw_no_count(11usize, 2usize, 48u32, 2105892u32)?;
    emu.lw_no_count(12usize, 2usize, 52u32, 2105896u32)?;
    emu.lw_no_count(13usize, 2usize, 56u32, 2105900u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2105904u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2105908u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2105912u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2105916u32)?;
    emu.adi_no_count(10usize, 2usize, 140u32, 2105920u32);
    emu.adi_no_count(11usize, 2usize, 176u32, 2105924u32);
    emu.adi_no_count(12usize, 2usize, 44u32, 2105928u32);
    emu.apc_no_count(1usize, 2105928u32, 32768u32, 2105932u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2105936u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1292u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202250(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 308u32, 2105940u32);
    emu.adi_no_count(11usize, 2usize, 140u32, 2105944u32);
    emu.adi_no_count(12usize, 2usize, 44u32, 2105948u32);
    emu.apc_no_count(1usize, 2105948u32, 32768u32, 2105952u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2105956u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1272u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00202264(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4294967292u32, 2105960u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2105964u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2105968u32);
    emu.adi_no_count(13usize, 0usize, 4294967295u32, 2105972u32);
    emu.sw_no_count(0usize, 2usize, 192u32, 2105976u32)?;
    emu.sw_no_count(0usize, 2usize, 196u32, 2105980u32)?;
    emu.sw_no_count(11usize, 2usize, 200u32, 2105984u32)?;
    emu.sw_no_count(10usize, 2usize, 204u32, 2105988u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2105992u32)?;
    emu.sw_no_count(13usize, 2usize, 180u32, 2105996u32)?;
    emu.sw_no_count(13usize, 2usize, 184u32, 2106000u32)?;
    emu.sw_no_count(12usize, 2usize, 188u32, 2106004u32)?;
    emu.adi_no_count(10usize, 2usize, 140u32, 2106008u32);
    emu.adi_no_count(11usize, 2usize, 176u32, 2106012u32);
    emu.adi_no_count(12usize, 2usize, 44u32, 2106016u32);
    emu.apc_no_count(1usize, 2106016u32, 32768u32, 2106020u32);
    emu.add_memory_rw_events(17usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106024u32;
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
pub fn block_0x002022a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 276u32, 2106028u32);
    emu.adi_no_count(11usize, 2usize, 308u32, 2106032u32);
    emu.adi_no_count(12usize, 2usize, 140u32, 2106036u32);
    emu.apc_no_count(1usize, 2106036u32, 32768u32, 2106040u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106044u32;
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
#[inline(never)]
pub fn block_0x002022bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 29u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(3694133248u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2106048u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(75976704u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2106052u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(3852607488u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2106056u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4146147328u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2106060u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2901409792u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2106064u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2021928960u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2106068u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(3634159616u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2106072u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1565u32, 2106076u32);
    emu.adi_no_count(11usize, 11usize, 4294965300u32, 2106080u32);
    emu.adi_no_count(12usize, 12usize, 171u32, 2106084u32);
    emu.adi_no_count(13usize, 13usize, 4294966998u32, 2106088u32);
    emu.sw_no_count(13usize, 2usize, 192u32, 2106092u32)?;
    emu.sw_no_count(12usize, 2usize, 196u32, 2106096u32)?;
    emu.sw_no_count(11usize, 2usize, 200u32, 2106100u32)?;
    emu.sw_no_count(10usize, 2usize, 204u32, 2106104u32)?;
    let a = 0u32.wrapping_add(700760064u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2106108u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 14usize, 1485u32, 2106112u32);
    emu.adi_no_count(12usize, 15usize, 144u32, 2106116u32);
    emu.adi_no_count(13usize, 16usize, 4294967138u32, 2106120u32);
    emu.adi_no_count(10usize, 10usize, 4294966751u32, 2106124u32);
    emu.sw_no_count(10usize, 2usize, 176u32, 2106128u32)?;
    emu.sw_no_count(13usize, 2usize, 180u32, 2106132u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2106136u32)?;
    emu.sw_no_count(11usize, 2usize, 188u32, 2106140u32)?;
    emu.adi_no_count(10usize, 2usize, 108u32, 2106144u32);
    emu.adi_no_count(11usize, 2usize, 276u32, 2106148u32);
    emu.adi_no_count(12usize, 2usize, 176u32, 2106152u32);
    emu.apc_no_count(1usize, 2106152u32, 32768u32, 2106156u32);
    emu.add_memory_rw_events(29usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106160u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967072u32);
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
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 140u32, 2106164u32);
    emu.adi_no_count(11usize, 2usize, 108u32, 2106168u32);
    emu.apc_no_count(1usize, 2106168u32, 61440u32, 2106172u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106176u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1964u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00202340(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 2usize, 292u32, 2106180u32)?;
    emu.sw_no_count(0usize, 2usize, 296u32, 2106184u32)?;
    emu.sw_no_count(0usize, 2usize, 300u32, 2106188u32)?;
    emu.sw_no_count(0usize, 2usize, 304u32, 2106192u32)?;
    emu.lbu_no_count(13usize, 2usize, 172u32, 2106196u32);
    emu.sw_no_count(0usize, 2usize, 276u32, 2106200u32)?;
    emu.sw_no_count(0usize, 2usize, 280u32, 2106204u32)?;
    emu.sw_no_count(0usize, 2usize, 284u32, 2106208u32)?;
    emu.sw_no_count(0usize, 2usize, 288u32, 2106212u32)?;
    emu.adi_no_count(10usize, 2usize, 244u32, 2106216u32);
    emu.adi_no_count(11usize, 2usize, 276u32, 2106220u32);
    emu.adi_no_count(12usize, 2usize, 140u32, 2106224u32);
    emu.apc_no_count(1usize, 2106224u32, 61440u32, 2106228u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106232u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1340u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00202378(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 260u32, 2106236u32)?;
    emu.lw_no_count(11usize, 2usize, 264u32, 2106240u32)?;
    emu.lw_no_count(12usize, 2usize, 268u32, 2106244u32)?;
    emu.lw_no_count(13usize, 2usize, 272u32, 2106248u32)?;
    emu.sw_no_count(10usize, 2usize, 192u32, 2106252u32)?;
    emu.sw_no_count(11usize, 2usize, 196u32, 2106256u32)?;
    emu.sw_no_count(12usize, 2usize, 200u32, 2106260u32)?;
    emu.sw_no_count(13usize, 2usize, 204u32, 2106264u32)?;
    emu.lw_no_count(10usize, 2usize, 244u32, 2106268u32)?;
    emu.lw_no_count(11usize, 2usize, 248u32, 2106272u32)?;
    emu.lw_no_count(12usize, 2usize, 252u32, 2106276u32)?;
    emu.lw_no_count(13usize, 2usize, 256u32, 2106280u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2106284u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2106288u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2106292u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2106296u32)?;
    emu.adi_no_count(10usize, 2usize, 308u32, 2106300u32);
    emu.adi_no_count(11usize, 2usize, 176u32, 2106304u32);
    emu.apc_no_count(1usize, 2106304u32, 36864u32, 2106308u32);
    emu.add_memory_rw_events(20usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106312u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(500u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002023c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 244u32, 2106316u32);
    emu.apc_no_count(1usize, 2106316u32, 61440u32, 2106320u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106324u32;
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
#[inline(always)]
pub fn block_0x002023d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xrr_no_count(10usize, 10usize, 9usize, 2106328u32);
    emu.ani_no_count(10usize, 10usize, 255u32, 2106332u32);
    emu.apc_no_count(1usize, 2106332u32, 86016u32, 2106336u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106340u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(412u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002023e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(10usize, 10usize, 4294967295u32, 2106344u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2106348u32);
    emu.apc_no_count(1usize, 2106348u32, 86016u32, 2106352u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106356u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(396u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002023f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 2usize, 208u32, 2106360u32);
    emu.ani_no_count(13usize, 10usize, 255u32, 2106364u32);
    emu.adi_no_count(11usize, 2usize, 308u32, 2106368u32);
    emu.adi_no_count(12usize, 2usize, 244u32, 2106372u32);
    emu.adi_no_count(10usize, 14usize, 0u32, 2106376u32);
    emu.apc_no_count(1usize, 2106376u32, 61440u32, 2106380u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106384u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1188u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00202410(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 44u32, 2106388u32)?;
    emu.lw_no_count(11usize, 2usize, 48u32, 2106392u32)?;
    emu.lw_no_count(12usize, 2usize, 52u32, 2106396u32)?;
    emu.lw_no_count(13usize, 2usize, 56u32, 2106400u32)?;
    emu.lw_no_count(14usize, 2usize, 60u32, 2106404u32)?;
    emu.lw_no_count(15usize, 2usize, 64u32, 2106408u32)?;
    emu.lw_no_count(16usize, 2usize, 68u32, 2106412u32)?;
    emu.lw_no_count(17usize, 2usize, 72u32, 2106416u32)?;
    emu.lbu_no_count(5usize, 2usize, 172u32, 2106420u32);
    emu.sw_no_count(10usize, 2usize, 176u32, 2106424u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2106428u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2106432u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2106436u32)?;
    emu.lbu_no_count(10usize, 2usize, 40u32, 2106440u32);
    emu.sw_no_count(14usize, 2usize, 192u32, 2106444u32)?;
    emu.sw_no_count(15usize, 2usize, 196u32, 2106448u32)?;
    emu.sw_no_count(16usize, 2usize, 200u32, 2106452u32)?;
    emu.sw_no_count(17usize, 2usize, 204u32, 2106456u32)?;
    emu.sb_no_count(0usize, 2usize, 240u32, 2106460u32);
    emu.anr_no_count(10usize, 10usize, 5usize, 2106464u32);
    emu.apc_no_count(1usize, 2106464u32, 86016u32, 2106468u32);
    emu.add_memory_rw_events(22usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106472u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(280u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202468(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2106476u32);
    emu.adi_no_count(11usize, 2usize, 176u32, 2106480u32);
    emu.adi_no_count(12usize, 0usize, 68u32, 2106484u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2106488u32);
    emu.apc_no_count(1usize, 2106488u32, 28672u32, 2106492u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106496u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1004u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202480(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(9usize, 8usize, 68u32, 2106500u32);
    emu.lw_no_count(1usize, 2usize, 348u32, 2106504u32)?;
    emu.lw_no_count(8usize, 2usize, 344u32, 2106508u32)?;
    emu.lw_no_count(9usize, 2usize, 340u32, 2106512u32)?;
    emu.adi_no_count(2usize, 2usize, 352u32, 2106516u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106520u32;
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
pub fn block_0x00202498(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294966688u32, 2106524u32);
    emu.sw_no_count(1usize, 2usize, 604u32, 2106528u32)?;
    emu.sw_no_count(8usize, 2usize, 600u32, 2106532u32)?;
    emu.sw_no_count(9usize, 2usize, 596u32, 2106536u32)?;
    emu.sw_no_count(18usize, 2usize, 592u32, 2106540u32)?;
    emu.sw_no_count(19usize, 2usize, 588u32, 2106544u32)?;
    emu.sw_no_count(20usize, 2usize, 584u32, 2106548u32)?;
    emu.sw_no_count(21usize, 2usize, 580u32, 2106552u32)?;
    emu.sw_no_count(22usize, 2usize, 576u32, 2106556u32)?;
    emu.sw_no_count(23usize, 2usize, 572u32, 2106560u32)?;
    emu.lbu_no_count(12usize, 11usize, 0u32, 2106564u32);
    emu.adi_no_count(13usize, 12usize, 4294967294u32, 2106568u32);
    emu.adi_no_count(14usize, 0usize, 4u32, 2106572u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2106576u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2107160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202718));
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
#[inline(always)]
pub fn block_0x002024d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 12usize, 6u32, 2106584u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2106588u32);
    emu.adi_no_count(9usize, 11usize, 1u32, 2106592u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2106632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202508));
    } else {
        emu.pc = 2106596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002024e4));
    }
}
#[inline(always)]
pub fn block_0x002024e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 12usize, 1u32, 2106600u32);
    emu.apc_no_count(1usize, 2106600u32, 86016u32, 2106604u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106608u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(144u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002024f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(12usize, 10usize, 255u32, 2106612u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2106616u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2106620u32);
    emu.apc_no_count(1usize, 2106620u32, 0u32, 2106624u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106628u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966436u32);
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
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2106632u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2108556u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202c8c));
}
#[inline(always)]
pub fn block_0x00202508(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 5u32, 2106636u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2107192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202738));
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
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2106644u32);
    emu.apc_no_count(1usize, 2106644u32, 86016u32, 2106648u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106652u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(100u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020251c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(12usize, 10usize, 255u32, 2106656u32);
    emu.adi_no_count(10usize, 2usize, 4u32, 2106660u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2106664u32);
    emu.apc_no_count(1usize, 2106664u32, 0u32, 2106668u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106672u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966392u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202530(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2106676u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966612u32, 2106680u32);
    emu.adi_no_count(10usize, 2usize, 276u32, 2106684u32);
    emu.adi_no_count(12usize, 0usize, 68u32, 2106688u32);
    emu.apc_no_count(1usize, 2106688u32, 28672u32, 2106692u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106696u32;
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
#[inline(always)]
pub fn block_0x00202548(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(9usize, 2usize, 72u32, 2106700u32);
    emu.adi_no_count(10usize, 2usize, 212u32, 2106704u32);
    emu.adi_no_count(11usize, 2usize, 276u32, 2106708u32);
    emu.adi_no_count(12usize, 2usize, 4u32, 2106712u32);
    emu.adi_no_count(13usize, 9usize, 0u32, 2106716u32);
    emu.apc_no_count(1usize, 2106716u32, 61440u32, 2106720u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106724u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(848u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202564(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 2usize, 308u32, 2106728u32);
    emu.adi_no_count(12usize, 2usize, 36u32, 2106732u32);
    emu.adi_no_count(10usize, 2usize, 244u32, 2106736u32);
    emu.adi_no_count(13usize, 9usize, 0u32, 2106740u32);
    emu.apc_no_count(1usize, 2106740u32, 61440u32, 2106744u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106748u32;
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
#[inline]
pub fn block_0x0020257c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 340u32, 2106752u32);
    emu.lbu_no_count(11usize, 2usize, 68u32, 2106756u32);
    emu.sbr_no_count(12usize, 0usize, 9usize, 2106760u32);
    emu.xrr_no_count(11usize, 11usize, 10usize, 2106764u32);
    emu.anr_no_count(11usize, 11usize, 12usize, 2106768u32);
    emu.xrr_no_count(19usize, 11usize, 10usize, 2106772u32);
    emu.adi_no_count(10usize, 2usize, 144u32, 2106776u32);
    emu.adi_no_count(11usize, 2usize, 212u32, 2106780u32);
    emu.adi_no_count(12usize, 0usize, 64u32, 2106784u32);
    emu.apc_no_count(1usize, 2106784u32, 28672u32, 2106788u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106792u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(708u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x002025a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 39u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(19usize, 2usize, 208u32, 2106796u32);
    emu.lw_no_count(10usize, 2usize, 228u32, 2106800u32)?;
    emu.lw_no_count(11usize, 2usize, 232u32, 2106804u32)?;
    emu.lw_no_count(12usize, 2usize, 236u32, 2106808u32)?;
    emu.lw_no_count(13usize, 2usize, 240u32, 2106812u32)?;
    emu.lw_no_count(14usize, 2usize, 212u32, 2106816u32)?;
    emu.lw_no_count(15usize, 2usize, 216u32, 2106820u32)?;
    emu.lw_no_count(16usize, 2usize, 220u32, 2106824u32)?;
    emu.lw_no_count(17usize, 2usize, 224u32, 2106828u32)?;
    emu.sw_no_count(10usize, 2usize, 360u32, 2106832u32)?;
    emu.sw_no_count(11usize, 2usize, 364u32, 2106836u32)?;
    emu.sw_no_count(12usize, 2usize, 368u32, 2106840u32)?;
    emu.sw_no_count(13usize, 2usize, 372u32, 2106844u32)?;
    emu.adi_no_count(9usize, 2usize, 176u32, 2106848u32);
    emu.sw_no_count(14usize, 2usize, 344u32, 2106852u32)?;
    emu.sw_no_count(15usize, 2usize, 348u32, 2106856u32)?;
    emu.sw_no_count(16usize, 2usize, 352u32, 2106860u32)?;
    emu.sw_no_count(17usize, 2usize, 356u32, 2106864u32)?;
    emu.lw_no_count(10usize, 2usize, 244u32, 2106868u32)?;
    emu.lw_no_count(11usize, 2usize, 248u32, 2106872u32)?;
    emu.lw_no_count(12usize, 2usize, 252u32, 2106876u32)?;
    emu.lw_no_count(13usize, 2usize, 256u32, 2106880u32)?;
    emu.sw_no_count(10usize, 2usize, 540u32, 2106884u32)?;
    emu.sw_no_count(11usize, 2usize, 544u32, 2106888u32)?;
    emu.sw_no_count(12usize, 2usize, 548u32, 2106892u32)?;
    emu.sw_no_count(13usize, 2usize, 552u32, 2106896u32)?;
    emu.lw_no_count(10usize, 2usize, 260u32, 2106900u32)?;
    emu.lw_no_count(11usize, 2usize, 264u32, 2106904u32)?;
    emu.lw_no_count(12usize, 2usize, 268u32, 2106908u32)?;
    emu.lw_no_count(13usize, 2usize, 272u32, 2106912u32)?;
    emu.sw_no_count(10usize, 2usize, 556u32, 2106916u32)?;
    emu.sw_no_count(11usize, 2usize, 560u32, 2106920u32)?;
    emu.sw_no_count(12usize, 2usize, 564u32, 2106924u32)?;
    emu.sw_no_count(13usize, 2usize, 568u32, 2106928u32)?;
    emu.adi_no_count(18usize, 2usize, 376u32, 2106932u32);
    emu.adi_no_count(11usize, 2usize, 540u32, 2106936u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2106940u32);
    emu.apc_no_count(1usize, 2106940u32, 36864u32, 2106944u32);
    emu.add_memory_rw_events(39usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106948u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967160u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202644(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(19usize, 2usize, 408u32, 2106952u32);
    emu.adi_no_count(10usize, 2usize, 476u32, 2106956u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2106960u32);
    emu.apc_no_count(1usize, 2106960u32, 61440u32, 2106964u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106968u32;
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
pub fn block_0x00202658(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 444u32, 2106972u32);
    emu.adi_no_count(11usize, 2usize, 476u32, 2106976u32);
    emu.apc_no_count(1usize, 2106976u32, 65536u32, 2106980u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106984u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966792u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202668(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 76u32, 2106988u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2106992u32);
    emu.apc_no_count(1usize, 2106992u32, 61440u32, 2106996u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107000u32;
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
#[inline(always)]
pub fn block_0x00202678(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 508u32, 2107004u32);
    emu.adi_no_count(11usize, 2usize, 76u32, 2107008u32);
    emu.apc_no_count(1usize, 2107008u32, 65536u32, 2107012u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107016u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966760u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202688(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 2usize, 444u32, 2107020u32)?;
    emu.lw_no_count(13usize, 2usize, 508u32, 2107024u32)?;
    emu.lw_no_count(11usize, 2usize, 512u32, 2107028u32)?;
    emu.lw_no_count(10usize, 2usize, 516u32, 2107032u32)?;
    emu.sltru_no_count(12usize, 13usize, 12usize, 2107036u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2108288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202b80));
    } else {
        emu.pc = 2107040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002026a0));
    }
}
#[inline(always)]
pub fn block_0x002026a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(11usize, 11usize, 12usize, 2107044u32);
    emu.sbr_no_count(11usize, 0usize, 11usize, 2107048u32);
    emu.slti_no_count(12usize, 11usize, 0u32, 2107052u32);
    emu.lw_no_count(11usize, 2usize, 520u32, 2107056u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2108308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202b94));
    } else {
        emu.pc = 2107060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002026b4));
    }
}
#[inline(always)]
pub fn block_0x002026b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 2usize, 452u32, 2107064u32)?;
    emu.sbr_no_count(10usize, 10usize, 12usize, 2107068u32);
    emu.sltru_no_count(12usize, 10usize, 13usize, 2107072u32);
    emu.lw_no_count(10usize, 2usize, 524u32, 2107076u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2108328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202ba8));
    } else {
        emu.pc = 2107080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002026c8));
    }
}
#[inline(always)]
pub fn block_0x002026c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 2usize, 456u32, 2107084u32)?;
    emu.sbr_no_count(11usize, 11usize, 12usize, 2107088u32);
    emu.sltru_no_count(12usize, 11usize, 13usize, 2107092u32);
    emu.lw_no_count(11usize, 2usize, 528u32, 2107096u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2108348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202bbc));
    } else {
        emu.pc = 2107100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002026dc));
    }
}
#[inline(always)]
pub fn block_0x002026dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 2usize, 460u32, 2107104u32)?;
    emu.sbr_no_count(10usize, 10usize, 12usize, 2107108u32);
    emu.sltru_no_count(12usize, 10usize, 13usize, 2107112u32);
    emu.lw_no_count(10usize, 2usize, 532u32, 2107116u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2108368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202bd0));
    } else {
        emu.pc = 2107120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002026f0));
    }
}
#[inline(always)]
pub fn block_0x002026f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 2usize, 464u32, 2107124u32)?;
    emu.sbr_no_count(11usize, 11usize, 12usize, 2107128u32);
    emu.sltru_no_count(13usize, 11usize, 13usize, 2107132u32);
    emu.lw_no_count(11usize, 2usize, 536u32, 2107136u32)?;
    emu.lw_no_count(12usize, 2usize, 472u32, 2107140u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2108392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202be8));
    } else {
        emu.pc = 2107144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202708));
    }
}
#[inline(always)]
pub fn block_0x00202708(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 2usize, 468u32, 2107148u32)?;
    emu.sbr_no_count(10usize, 10usize, 13usize, 2107152u32);
    emu.sltru_no_count(10usize, 10usize, 14usize, 2107156u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2107160u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2108404u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202bf4));
}
#[inline(always)]
pub fn block_0x00202718(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2108600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202cb8));
    } else {
        emu.pc = 2107164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020271c));
    }
}
#[inline(always)]
pub fn block_0x0020271c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2107168u32);
    emu.apc_no_count(1usize, 2107168u32, 86016u32, 2107172u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107176u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966872u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202728(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2107180u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2107184u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966612u32, 2107188u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2107192u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2108536u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202c78));
}
#[inline(never)]
pub fn block_0x00202738(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 92u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 11usize, 61u32, 2107196u32);
    emu.lbu_no_count(12usize, 11usize, 62u32, 2107200u32);
    emu.lbu_no_count(13usize, 11usize, 63u32, 2107204u32);
    emu.lbu_no_count(14usize, 11usize, 64u32, 2107208u32);
    emu.lbu_no_count(15usize, 11usize, 57u32, 2107212u32);
    emu.lbu_no_count(16usize, 11usize, 58u32, 2107216u32);
    emu.lbu_no_count(17usize, 11usize, 59u32, 2107220u32);
    emu.lbu_no_count(5usize, 11usize, 60u32, 2107224u32);
    emu.lbu_no_count(6usize, 11usize, 53u32, 2107228u32);
    emu.lbu_no_count(7usize, 11usize, 54u32, 2107232u32);
    emu.lbu_no_count(28usize, 11usize, 55u32, 2107236u32);
    emu.lbu_no_count(29usize, 11usize, 56u32, 2107240u32);
    emu.lbu_no_count(30usize, 11usize, 49u32, 2107244u32);
    emu.lbu_no_count(31usize, 11usize, 50u32, 2107248u32);
    emu.lbu_no_count(18usize, 11usize, 51u32, 2107252u32);
    emu.lbu_no_count(19usize, 11usize, 52u32, 2107256u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2107260u32);
    emu.sli_no_count(13usize, 13usize, 16u32, 2107264u32);
    emu.sli_no_count(14usize, 14usize, 24u32, 2107268u32);
    emu.sli_no_count(16usize, 16usize, 8u32, 2107272u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2107276u32);
    emu.orr_no_count(12usize, 14usize, 13usize, 2107280u32);
    emu.orr_no_count(13usize, 16usize, 15usize, 2107284u32);
    emu.lbu_no_count(20usize, 11usize, 45u32, 2107288u32);
    emu.lbu_no_count(21usize, 11usize, 46u32, 2107292u32);
    emu.lbu_no_count(22usize, 11usize, 47u32, 2107296u32);
    emu.lbu_no_count(23usize, 11usize, 48u32, 2107300u32);
    emu.sli_no_count(17usize, 17usize, 16u32, 2107304u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2107308u32);
    emu.sli_no_count(7usize, 7usize, 8u32, 2107312u32);
    emu.sli_no_count(28usize, 28usize, 16u32, 2107316u32);
    emu.sli_no_count(29usize, 29usize, 24u32, 2107320u32);
    emu.sli_no_count(31usize, 31usize, 8u32, 2107324u32);
    emu.orr_no_count(14usize, 5usize, 17usize, 2107328u32);
    emu.orr_no_count(15usize, 7usize, 6usize, 2107332u32);
    emu.orr_no_count(16usize, 29usize, 28usize, 2107336u32);
    emu.orr_no_count(17usize, 31usize, 30usize, 2107340u32);
    emu.lbu_no_count(5usize, 11usize, 41u32, 2107344u32);
    emu.lbu_no_count(6usize, 11usize, 42u32, 2107348u32);
    emu.lbu_no_count(7usize, 11usize, 43u32, 2107352u32);
    emu.lbu_no_count(28usize, 11usize, 44u32, 2107356u32);
    emu.sli_no_count(18usize, 18usize, 16u32, 2107360u32);
    emu.sli_no_count(19usize, 19usize, 24u32, 2107364u32);
    emu.sli_no_count(21usize, 21usize, 8u32, 2107368u32);
    emu.sli_no_count(22usize, 22usize, 16u32, 2107372u32);
    emu.sli_no_count(23usize, 23usize, 24u32, 2107376u32);
    emu.sli_no_count(6usize, 6usize, 8u32, 2107380u32);
    emu.orr_no_count(29usize, 19usize, 18usize, 2107384u32);
    emu.orr_no_count(30usize, 21usize, 20usize, 2107388u32);
    emu.orr_no_count(31usize, 23usize, 22usize, 2107392u32);
    emu.orr_no_count(5usize, 6usize, 5usize, 2107396u32);
    emu.lbu_no_count(6usize, 11usize, 37u32, 2107400u32);
    emu.lbu_no_count(18usize, 11usize, 38u32, 2107404u32);
    emu.lbu_no_count(19usize, 11usize, 39u32, 2107408u32);
    emu.lbu_no_count(20usize, 11usize, 40u32, 2107412u32);
    emu.sli_no_count(7usize, 7usize, 16u32, 2107416u32);
    emu.sli_no_count(28usize, 28usize, 24u32, 2107420u32);
    emu.sli_no_count(18usize, 18usize, 8u32, 2107424u32);
    emu.sli_no_count(19usize, 19usize, 16u32, 2107428u32);
    emu.sli_no_count(20usize, 20usize, 24u32, 2107432u32);
    emu.orr_no_count(7usize, 28usize, 7usize, 2107436u32);
    emu.orr_no_count(6usize, 18usize, 6usize, 2107440u32);
    emu.lbu_no_count(28usize, 11usize, 33u32, 2107444u32);
    emu.lbu_no_count(18usize, 11usize, 34u32, 2107448u32);
    emu.orr_no_count(19usize, 20usize, 19usize, 2107452u32);
    emu.lbu_no_count(20usize, 11usize, 35u32, 2107456u32);
    emu.lbu_no_count(11usize, 11usize, 36u32, 2107460u32);
    emu.sli_no_count(18usize, 18usize, 8u32, 2107464u32);
    emu.orr_no_count(28usize, 18usize, 28usize, 2107468u32);
    emu.sli_no_count(20usize, 20usize, 16u32, 2107472u32);
    emu.sli_no_count(11usize, 11usize, 24u32, 2107476u32);
    emu.orr_no_count(11usize, 11usize, 20usize, 2107480u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2107484u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2107488u32);
    emu.orr_no_count(12usize, 16usize, 15usize, 2107492u32);
    emu.orr_no_count(14usize, 29usize, 17usize, 2107496u32);
    emu.orr_no_count(15usize, 31usize, 30usize, 2107500u32);
    emu.orr_no_count(16usize, 7usize, 5usize, 2107504u32);
    emu.orr_no_count(17usize, 19usize, 6usize, 2107508u32);
    emu.orr_no_count(11usize, 11usize, 28usize, 2107512u32);
    emu.sw_no_count(14usize, 2usize, 20u32, 2107516u32)?;
    emu.sw_no_count(12usize, 2usize, 24u32, 2107520u32)?;
    emu.sw_no_count(13usize, 2usize, 28u32, 2107524u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2107528u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2107532u32)?;
    emu.sw_no_count(17usize, 2usize, 8u32, 2107536u32)?;
    emu.sw_no_count(16usize, 2usize, 12u32, 2107540u32)?;
    emu.sw_no_count(15usize, 2usize, 16u32, 2107544u32)?;
    emu.adi_no_count(10usize, 2usize, 276u32, 2107548u32);
    emu.adi_no_count(11usize, 2usize, 4u32, 2107552u32);
    emu.apc_no_count(1usize, 2107552u32, 57344u32, 2107556u32);
    emu.add_memory_rw_events(92usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107560u32;
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
#[inline]
pub fn block_0x002028a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 2usize, 460u32, 2107564u32)?;
    emu.sw_no_count(0usize, 2usize, 464u32, 2107568u32)?;
    emu.sw_no_count(0usize, 2usize, 468u32, 2107572u32)?;
    emu.sw_no_count(0usize, 2usize, 472u32, 2107576u32)?;
    emu.lbu_no_count(13usize, 2usize, 308u32, 2107580u32);
    emu.sw_no_count(0usize, 2usize, 444u32, 2107584u32)?;
    emu.sw_no_count(0usize, 2usize, 448u32, 2107588u32)?;
    emu.sw_no_count(0usize, 2usize, 452u32, 2107592u32)?;
    emu.sw_no_count(0usize, 2usize, 456u32, 2107596u32)?;
    emu.adi_no_count(10usize, 2usize, 412u32, 2107600u32);
    emu.adi_no_count(11usize, 2usize, 444u32, 2107604u32);
    emu.adi_no_count(12usize, 2usize, 276u32, 2107608u32);
    emu.apc_no_count(1usize, 2107608u32, 61440u32, 2107612u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107616u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967252u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002028e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 344u32, 2107620u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2107624u32);
    emu.apc_no_count(1usize, 2107624u32, 57344u32, 2107628u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107632u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1092u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002028f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 2usize, 524u32, 2107636u32)?;
    emu.sw_no_count(0usize, 2usize, 528u32, 2107640u32)?;
    emu.sw_no_count(0usize, 2usize, 532u32, 2107644u32)?;
    emu.sw_no_count(0usize, 2usize, 536u32, 2107648u32)?;
    emu.lbu_no_count(13usize, 2usize, 376u32, 2107652u32);
    emu.sw_no_count(0usize, 2usize, 508u32, 2107656u32)?;
    emu.sw_no_count(0usize, 2usize, 512u32, 2107660u32)?;
    emu.sw_no_count(0usize, 2usize, 516u32, 2107664u32)?;
    emu.sw_no_count(0usize, 2usize, 520u32, 2107668u32)?;
    emu.adi_no_count(10usize, 2usize, 476u32, 2107672u32);
    emu.adi_no_count(11usize, 2usize, 508u32, 2107676u32);
    emu.adi_no_count(12usize, 2usize, 344u32, 2107680u32);
    emu.apc_no_count(1usize, 2107680u32, 61440u32, 2107684u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107688u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967180u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00202928(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 428u32, 2107692u32)?;
    emu.lw_no_count(11usize, 2usize, 432u32, 2107696u32)?;
    emu.lw_no_count(12usize, 2usize, 436u32, 2107700u32)?;
    emu.lw_no_count(13usize, 2usize, 440u32, 2107704u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2107708u32)?;
    emu.sw_no_count(11usize, 2usize, 24u32, 2107712u32)?;
    emu.sw_no_count(12usize, 2usize, 28u32, 2107716u32)?;
    emu.sw_no_count(13usize, 2usize, 32u32, 2107720u32)?;
    emu.lw_no_count(10usize, 2usize, 412u32, 2107724u32)?;
    emu.lw_no_count(11usize, 2usize, 416u32, 2107728u32)?;
    emu.lw_no_count(12usize, 2usize, 420u32, 2107732u32)?;
    emu.lw_no_count(13usize, 2usize, 424u32, 2107736u32)?;
    emu.sw_no_count(10usize, 2usize, 4u32, 2107740u32)?;
    emu.sw_no_count(11usize, 2usize, 8u32, 2107744u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2107748u32)?;
    emu.sw_no_count(13usize, 2usize, 16u32, 2107752u32)?;
    emu.adi_no_count(10usize, 2usize, 540u32, 2107756u32);
    emu.adi_no_count(11usize, 2usize, 4u32, 2107760u32);
    emu.adi_no_count(12usize, 2usize, 412u32, 2107764u32);
    emu.apc_no_count(1usize, 2107764u32, 32768u32, 2107768u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107772u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966752u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020297c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 492u32, 2107776u32)?;
    emu.lw_no_count(11usize, 2usize, 496u32, 2107780u32)?;
    emu.lw_no_count(12usize, 2usize, 500u32, 2107784u32)?;
    emu.lw_no_count(13usize, 2usize, 504u32, 2107788u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2107792u32)?;
    emu.sw_no_count(11usize, 2usize, 24u32, 2107796u32)?;
    emu.sw_no_count(12usize, 2usize, 28u32, 2107800u32)?;
    emu.sw_no_count(13usize, 2usize, 32u32, 2107804u32)?;
    emu.lw_no_count(10usize, 2usize, 476u32, 2107808u32)?;
    emu.lw_no_count(11usize, 2usize, 480u32, 2107812u32)?;
    emu.lw_no_count(12usize, 2usize, 484u32, 2107816u32)?;
    emu.lw_no_count(13usize, 2usize, 488u32, 2107820u32)?;
    emu.sw_no_count(10usize, 2usize, 4u32, 2107824u32)?;
    emu.sw_no_count(11usize, 2usize, 8u32, 2107828u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2107832u32)?;
    emu.sw_no_count(13usize, 2usize, 16u32, 2107836u32)?;
    emu.adi_no_count(10usize, 2usize, 144u32, 2107840u32);
    emu.adi_no_count(11usize, 2usize, 4u32, 2107844u32);
    emu.adi_no_count(12usize, 2usize, 476u32, 2107848u32);
    emu.apc_no_count(1usize, 2107848u32, 32768u32, 2107852u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107856u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966668u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002029d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 76u32, 2107860u32);
    emu.adi_no_count(11usize, 2usize, 144u32, 2107864u32);
    emu.adi_no_count(12usize, 2usize, 476u32, 2107868u32);
    emu.apc_no_count(1usize, 2107868u32, 32768u32, 2107872u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107876u32;
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
pub fn block_0x002029e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4294967292u32, 2107880u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2107884u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2107888u32);
    emu.adi_no_count(13usize, 0usize, 4294967295u32, 2107892u32);
    emu.sw_no_count(0usize, 2usize, 20u32, 2107896u32)?;
    emu.sw_no_count(0usize, 2usize, 24u32, 2107900u32)?;
    emu.sw_no_count(11usize, 2usize, 28u32, 2107904u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2107908u32)?;
    emu.sw_no_count(10usize, 2usize, 4u32, 2107912u32)?;
    emu.sw_no_count(13usize, 2usize, 8u32, 2107916u32)?;
    emu.sw_no_count(13usize, 2usize, 12u32, 2107920u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2107924u32)?;
    emu.adi_no_count(10usize, 2usize, 144u32, 2107928u32);
    emu.adi_no_count(11usize, 2usize, 4u32, 2107932u32);
    emu.adi_no_count(12usize, 2usize, 476u32, 2107936u32);
    emu.apc_no_count(1usize, 2107936u32, 32768u32, 2107940u32);
    emu.add_memory_rw_events(17usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107944u32;
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
pub fn block_0x00202a28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 212u32, 2107948u32);
    emu.adi_no_count(11usize, 2usize, 76u32, 2107952u32);
    emu.adi_no_count(12usize, 2usize, 144u32, 2107956u32);
    emu.apc_no_count(1usize, 2107956u32, 32768u32, 2107960u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107964u32;
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
pub fn block_0x00202a3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 29u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(3694133248u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2107968u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(75976704u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2107972u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(3852607488u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2107976u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4146147328u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2107980u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2901409792u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2107984u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2021928960u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2107988u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(3634159616u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2107992u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1565u32, 2107996u32);
    emu.adi_no_count(11usize, 11usize, 4294965300u32, 2108000u32);
    emu.adi_no_count(12usize, 12usize, 171u32, 2108004u32);
    emu.adi_no_count(13usize, 13usize, 4294966998u32, 2108008u32);
    emu.sw_no_count(13usize, 2usize, 20u32, 2108012u32)?;
    emu.sw_no_count(12usize, 2usize, 24u32, 2108016u32)?;
    emu.sw_no_count(11usize, 2usize, 28u32, 2108020u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2108024u32)?;
    let a = 0u32.wrapping_add(700760064u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2108028u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 14usize, 1485u32, 2108032u32);
    emu.adi_no_count(12usize, 15usize, 144u32, 2108036u32);
    emu.adi_no_count(13usize, 16usize, 4294967138u32, 2108040u32);
    emu.adi_no_count(10usize, 10usize, 4294966751u32, 2108044u32);
    emu.sw_no_count(10usize, 2usize, 4u32, 2108048u32)?;
    emu.sw_no_count(13usize, 2usize, 8u32, 2108052u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2108056u32)?;
    emu.sw_no_count(11usize, 2usize, 16u32, 2108060u32)?;
    emu.adi_no_count(10usize, 2usize, 76u32, 2108064u32);
    emu.adi_no_count(11usize, 2usize, 212u32, 2108068u32);
    emu.adi_no_count(12usize, 2usize, 4u32, 2108072u32);
    emu.apc_no_count(1usize, 2108072u32, 28672u32, 2108076u32);
    emu.add_memory_rw_events(29usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108080u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1952u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00202ab0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 36u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 476u32, 2108084u32)?;
    emu.lw_no_count(11usize, 2usize, 480u32, 2108088u32)?;
    emu.lw_no_count(12usize, 2usize, 484u32, 2108092u32)?;
    emu.lw_no_count(13usize, 2usize, 488u32, 2108096u32)?;
    emu.sw_no_count(10usize, 2usize, 4u32, 2108100u32)?;
    emu.sw_no_count(11usize, 2usize, 8u32, 2108104u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2108108u32)?;
    emu.sw_no_count(13usize, 2usize, 16u32, 2108112u32)?;
    emu.lw_no_count(10usize, 2usize, 492u32, 2108116u32)?;
    emu.lw_no_count(11usize, 2usize, 496u32, 2108120u32)?;
    emu.lw_no_count(12usize, 2usize, 500u32, 2108124u32)?;
    emu.lw_no_count(13usize, 2usize, 504u32, 2108128u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2108132u32)?;
    emu.sw_no_count(11usize, 2usize, 24u32, 2108136u32)?;
    emu.sw_no_count(12usize, 2usize, 28u32, 2108140u32)?;
    emu.sw_no_count(13usize, 2usize, 32u32, 2108144u32)?;
    emu.lw_no_count(10usize, 2usize, 428u32, 2108148u32)?;
    emu.lw_no_count(11usize, 2usize, 432u32, 2108152u32)?;
    emu.lw_no_count(12usize, 2usize, 436u32, 2108156u32)?;
    emu.lw_no_count(13usize, 2usize, 440u32, 2108160u32)?;
    emu.sw_no_count(10usize, 2usize, 52u32, 2108164u32)?;
    emu.sw_no_count(11usize, 2usize, 56u32, 2108168u32)?;
    emu.sw_no_count(12usize, 2usize, 60u32, 2108172u32)?;
    emu.sw_no_count(13usize, 2usize, 64u32, 2108176u32)?;
    emu.lw_no_count(10usize, 2usize, 412u32, 2108180u32)?;
    emu.lw_no_count(11usize, 2usize, 416u32, 2108184u32)?;
    emu.lw_no_count(12usize, 2usize, 420u32, 2108188u32)?;
    emu.lw_no_count(13usize, 2usize, 424u32, 2108192u32)?;
    emu.sw_no_count(10usize, 2usize, 36u32, 2108196u32)?;
    emu.sw_no_count(11usize, 2usize, 40u32, 2108200u32)?;
    emu.sw_no_count(12usize, 2usize, 44u32, 2108204u32)?;
    emu.sw_no_count(13usize, 2usize, 48u32, 2108208u32)?;
    emu.adi_no_count(10usize, 2usize, 540u32, 2108212u32);
    emu.adi_no_count(11usize, 2usize, 76u32, 2108216u32);
    emu.apc_no_count(1usize, 2108216u32, 61440u32, 2108220u32);
    emu.add_memory_rw_events(36usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108224u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966876u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202b40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 2usize, 376u32, 2108228u32);
    emu.anr_no_count(10usize, 10usize, 11usize, 2108232u32);
    emu.apc_no_count(1usize, 2108232u32, 86016u32, 2108236u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108240u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965808u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202b50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 2usize, 308u32, 2108244u32);
    emu.anr_no_count(10usize, 10usize, 11usize, 2108248u32);
    emu.apc_no_count(1usize, 2108248u32, 86016u32, 2108252u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108256u32;
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
pub fn block_0x00202b60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2108260u32);
    emu.adi_no_count(11usize, 2usize, 4u32, 2108264u32);
    emu.adi_no_count(12usize, 0usize, 64u32, 2108268u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2108272u32);
    emu.apc_no_count(1usize, 2108272u32, 28672u32, 2108276u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108280u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966516u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202b78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(0usize, 8usize, 64u32, 2108284u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2108288u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2108552u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202c88));
}
#[inline(always)]
pub fn block_0x00202b80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 2usize, 448u32, 2108292u32)?;
    emu.sbr_no_count(11usize, 11usize, 12usize, 2108296u32);
    emu.sltru_no_count(12usize, 11usize, 13usize, 2108300u32);
    emu.lw_no_count(11usize, 2usize, 520u32, 2108304u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2107060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002026b4));
    } else {
        emu.pc = 2108308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202b94));
    }
}
#[inline(always)]
pub fn block_0x00202b94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 10usize, 12usize, 2108312u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2108316u32);
    emu.slti_no_count(12usize, 10usize, 0u32, 2108320u32);
    emu.lw_no_count(10usize, 2usize, 524u32, 2108324u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2107080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002026c8));
    } else {
        emu.pc = 2108328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202ba8));
    }
}
#[inline(always)]
pub fn block_0x00202ba8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(11usize, 11usize, 12usize, 2108332u32);
    emu.sbr_no_count(11usize, 0usize, 11usize, 2108336u32);
    emu.slti_no_count(12usize, 11usize, 0u32, 2108340u32);
    emu.lw_no_count(11usize, 2usize, 528u32, 2108344u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2107100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002026dc));
    } else {
        emu.pc = 2108348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202bbc));
    }
}
#[inline(always)]
pub fn block_0x00202bbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 10usize, 12usize, 2108352u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2108356u32);
    emu.slti_no_count(12usize, 10usize, 0u32, 2108360u32);
    emu.lw_no_count(10usize, 2usize, 532u32, 2108364u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2107120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002026f0));
    } else {
        emu.pc = 2108368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202bd0));
    }
}
#[inline(always)]
pub fn block_0x00202bd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(11usize, 11usize, 12usize, 2108372u32);
    emu.sbr_no_count(11usize, 0usize, 11usize, 2108376u32);
    emu.slti_no_count(13usize, 11usize, 0u32, 2108380u32);
    emu.lw_no_count(11usize, 2usize, 536u32, 2108384u32)?;
    emu.lw_no_count(12usize, 2usize, 472u32, 2108388u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2107144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202708));
    } else {
        emu.pc = 2108392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202be8));
    }
}
#[inline(always)]
pub fn block_0x00202be8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 10usize, 13usize, 2108396u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2108400u32);
    emu.slti_no_count(10usize, 10usize, 0u32, 2108404u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2108404u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202bf4));
}
#[inline(always)]
pub fn block_0x00202bf4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(13usize, 11usize, 12usize, 2108408u32);
    emu.sbr_no_count(11usize, 11usize, 12usize, 2108412u32);
    emu.sltru_no_count(10usize, 11usize, 10usize, 2108416u32);
    emu.sbr_no_count(11usize, 0usize, 13usize, 2108420u32);
    emu.sbr_no_count(10usize, 11usize, 10usize, 2108424u32);
    emu.apc_no_count(1usize, 2108424u32, 86016u32, 2108428u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108432u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965636u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00202c10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 23u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 212u32, 2108436u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2108440u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2108444u32)?;
    emu.lw_no_count(14usize, 2usize, 224u32, 2108448u32)?;
    emu.sw_no_count(11usize, 2usize, 76u32, 2108452u32)?;
    emu.sw_no_count(12usize, 2usize, 80u32, 2108456u32)?;
    emu.sw_no_count(13usize, 2usize, 84u32, 2108460u32)?;
    emu.sw_no_count(14usize, 2usize, 88u32, 2108464u32)?;
    emu.lw_no_count(11usize, 2usize, 228u32, 2108468u32)?;
    emu.lw_no_count(12usize, 2usize, 232u32, 2108472u32)?;
    emu.lw_no_count(13usize, 2usize, 236u32, 2108476u32)?;
    emu.lw_no_count(14usize, 2usize, 240u32, 2108480u32)?;
    emu.sw_no_count(11usize, 2usize, 92u32, 2108484u32)?;
    emu.sw_no_count(12usize, 2usize, 96u32, 2108488u32)?;
    emu.sw_no_count(13usize, 2usize, 100u32, 2108492u32)?;
    emu.sw_no_count(14usize, 2usize, 104u32, 2108496u32)?;
    emu.adi_no_count(11usize, 2usize, 108u32, 2108500u32);
    emu.ani_no_count(13usize, 10usize, 255u32, 2108504u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2108508u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2108512u32);
    emu.adi_no_count(12usize, 18usize, 0u32, 2108516u32);
    emu.apc_no_count(1usize, 2108516u32, 61440u32, 2108520u32);
    emu.add_memory_rw_events(23usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108524u32;
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
pub fn block_0x00202c6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(9usize, 2usize, 72u32, 2108528u32);
    emu.sb_no_count(19usize, 2usize, 140u32, 2108532u32);
    emu.adi_no_count(11usize, 2usize, 76u32, 2108536u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2108536u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202c78));
}
#[inline(always)]
pub fn block_0x00202c78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 68u32, 2108540u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2108544u32);
    emu.apc_no_count(1usize, 2108544u32, 28672u32, 2108548u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108552u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966244u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202c88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(9usize, 8usize, 68u32, 2108556u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2108556u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202c8c));
}
#[inline]
pub fn block_0x00202c8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 604u32, 2108560u32)?;
    emu.lw_no_count(8usize, 2usize, 600u32, 2108564u32)?;
    emu.lw_no_count(9usize, 2usize, 596u32, 2108568u32)?;
    emu.lw_no_count(18usize, 2usize, 592u32, 2108572u32)?;
    emu.lw_no_count(19usize, 2usize, 588u32, 2108576u32)?;
    emu.lw_no_count(20usize, 2usize, 584u32, 2108580u32)?;
    emu.lw_no_count(21usize, 2usize, 580u32, 2108584u32)?;
    emu.lw_no_count(22usize, 2usize, 576u32, 2108588u32)?;
    emu.lw_no_count(23usize, 2usize, 572u32, 2108592u32)?;
    emu.adi_no_count(2usize, 2usize, 608u32, 2108596u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108600u32;
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
pub fn block_0x00202cb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 7u32, 2108604u32);
    emu.sw_no_count(10usize, 2usize, 4u32, 2108608u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2108612u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965768u32, 2108616u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2108620u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294965752u32, 2108624u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2108628u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965780u32, 2108632u32);
    emu.adi_no_count(11usize, 0usize, 11u32, 2108636u32);
    emu.adi_no_count(12usize, 2usize, 4u32, 2108640u32);
    emu.apc_no_count(1usize, 2108640u32, 106496u32, 2108644u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108648u32;
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
#[inline]
pub fn block_0x00202ce8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2108652u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2108656u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2108660u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2108664u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2108668u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2108672u32)?;
    emu.sw_no_count(20usize, 2usize, 40u32, 2108676u32)?;
    emu.sw_no_count(21usize, 2usize, 36u32, 2108680u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2108684u32);
    emu.lw_no_count(9usize, 11usize, 8u32, 2108688u32)?;
    emu.lw_no_count(19usize, 11usize, 4u32, 2108692u32)?;
    emu.sw_no_count(9usize, 2usize, 24u32, 2108696u32)?;
    emu.sw_no_count(0usize, 2usize, 28u32, 2108700u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2108704u32);
    emu.adi_no_count(12usize, 2usize, 24u32, 2108708u32);
    emu.adi_no_count(13usize, 0usize, 8u32, 2108712u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2108716u32);
    emu.apc_no_count(1usize, 2108716u32, 12288u32, 2108720u32);
    emu.add_memory_rw_events(19usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108724u32;
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
#[inline(always)]
pub fn block_0x00202d34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 8u32, 2108728u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2108732u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2108860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202dbc));
    } else {
        emu.pc = 2108736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202d40));
    }
}
#[inline(always)]
pub fn block_0x00202d40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2108852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202db4));
    } else {
        emu.pc = 2108740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202d44));
    }
}
#[inline(always)]
pub fn block_0x00202d44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 19usize, 8u32, 2108744u32);
    emu.sli_no_count(10usize, 9usize, 2u32, 2108748u32);
    emu.sli_no_count(9usize, 9usize, 4u32, 2108752u32);
    emu.sbr_no_count(20usize, 9usize, 10usize, 2108756u32);
    emu.adi_no_count(21usize, 0usize, 4u32, 2108760u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2108760u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202d58));
}
#[inline]
pub fn block_0x00202d58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(9usize, 19usize, 0u32, 2108764u32)?;
    emu.lw_no_count(18usize, 19usize, 4294967292u32, 2108768u32)?;
    emu.sw_no_count(9usize, 2usize, 24u32, 2108772u32)?;
    emu.sw_no_count(0usize, 2usize, 28u32, 2108776u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2108780u32);
    emu.adi_no_count(12usize, 2usize, 24u32, 2108784u32);
    emu.adi_no_count(13usize, 0usize, 8u32, 2108788u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2108792u32);
    emu.apc_no_count(1usize, 2108792u32, 12288u32, 2108796u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108800u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(64u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202d80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 8u32, 2108804u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2108860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202dbc));
    } else {
        emu.pc = 2108808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202d88));
    }
}
#[inline(always)]
pub fn block_0x00202d88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 24u32, 2108812u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2108816u32);
    emu.adi_no_count(12usize, 18usize, 0u32, 2108820u32);
    emu.adi_no_count(13usize, 9usize, 0u32, 2108824u32);
    emu.apc_no_count(1usize, 2108824u32, 12288u32, 2108828u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108832u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(32u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202da0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 24u32, 2108836u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2108924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202dfc));
    } else {
        emu.pc = 2108840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202da8));
    }
}
#[inline(always)]
pub fn block_0x00202da8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 20usize, 4294967284u32, 2108844u32);
    emu.adi_no_count(19usize, 19usize, 12u32, 2108848u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a != b {
        emu.pc = 2108760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202d58));
    } else {
        emu.pc = 2108852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202db4));
    }
}
#[inline(always)]
pub fn block_0x00202db4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2108856u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2108860u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2108888u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202dd8));
}
#[inline(always)]
pub fn block_0x00202dbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2108864u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2108868u32)?;
    emu.sw_no_count(10usize, 2usize, 16u32, 2108872u32)?;
    emu.sw_no_count(11usize, 2usize, 20u32, 2108876u32)?;
    emu.adi_no_count(10usize, 2usize, 16u32, 2108880u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2108880u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202dd0));
}
#[inline(always)]
pub fn block_0x00202dd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2108880u32, 28672u32, 2108884u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108888u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(572u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00202dd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2108892u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2108896u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2108900u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2108904u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2108908u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2108912u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2108916u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2108920u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108924u32;
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
pub fn block_0x00202dfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 24u32, 2108928u32)?;
    emu.lw_no_count(11usize, 2usize, 28u32, 2108932u32)?;
    emu.sw_no_count(10usize, 2usize, 0u32, 2108936u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2108940u32)?;
    emu.adi_no_count(10usize, 2usize, 0u32, 2108944u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2108948u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2108880u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202dd0));
}
#[inline(never)]
pub fn block_0x00202e14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 53u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294966912u32, 2108952u32);
    emu.sw_no_count(1usize, 2usize, 380u32, 2108956u32)?;
    emu.sw_no_count(8usize, 2usize, 376u32, 2108960u32)?;
    emu.sw_no_count(9usize, 2usize, 372u32, 2108964u32)?;
    emu.sw_no_count(18usize, 2usize, 368u32, 2108968u32)?;
    emu.sw_no_count(19usize, 2usize, 364u32, 2108972u32)?;
    emu.sw_no_count(20usize, 2usize, 360u32, 2108976u32)?;
    emu.sw_no_count(21usize, 2usize, 356u32, 2108980u32)?;
    emu.sw_no_count(22usize, 2usize, 352u32, 2108984u32)?;
    emu.sw_no_count(23usize, 2usize, 348u32, 2108988u32)?;
    emu.sw_no_count(24usize, 2usize, 344u32, 2108992u32)?;
    emu.sw_no_count(25usize, 2usize, 340u32, 2108996u32)?;
    emu.sw_no_count(26usize, 2usize, 336u32, 2109000u32)?;
    emu.sw_no_count(27usize, 2usize, 332u32, 2109004u32)?;
    emu.sw_no_count(13usize, 2usize, 8u32, 2109008u32)?;
    emu.adi_no_count(19usize, 12usize, 0u32, 2109012u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2109016u32);
    emu.sw_no_count(10usize, 2usize, 4u32, 2109020u32)?;
    emu.adi_no_count(20usize, 12usize, 40u32, 2109024u32);
    emu.lbu_no_count(21usize, 12usize, 104u32, 2109028u32);
    emu.lw_no_count(10usize, 12usize, 32u32, 2109032u32)?;
    emu.lw_no_count(11usize, 12usize, 36u32, 2109036u32)?;
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2109040u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 0usize, 128u32, 2109044u32);
    emu.adi_no_count(12usize, 12usize, 4294967040u32, 2109048u32);
    emu.sli_no_count(14usize, 11usize, 9u32, 2109052u32);
    emu.sli_no_count(15usize, 10usize, 9u32, 2109056u32);
    emu.sli_no_count(16usize, 21usize, 3u32, 2109060u32);
    emu.sli_no_count(17usize, 10usize, 1u32, 2109064u32);
    emu.sli_no_count(11usize, 11usize, 1u32, 2109068u32);
    emu.orr_no_count(16usize, 15usize, 16usize, 2109072u32);
    emu.anr_no_count(17usize, 17usize, 12usize, 2109076u32);
    emu.sri_no_count(15usize, 15usize, 24u32, 2109080u32);
    emu.anr_no_count(11usize, 11usize, 12usize, 2109084u32);
    emu.orr_no_count(15usize, 17usize, 15usize, 2109088u32);
    emu.sri_no_count(17usize, 14usize, 24u32, 2109092u32);
    emu.orr_no_count(11usize, 11usize, 17usize, 2109096u32);
    emu.adi_no_count(17usize, 0usize, 63u32, 2109100u32);
    emu.sri_no_count(5usize, 10usize, 23u32, 2109104u32);
    emu.orr_no_count(10usize, 14usize, 5usize, 2109108u32);
    emu.anr_no_count(14usize, 16usize, 12usize, 2109112u32);
    emu.anr_no_count(12usize, 10usize, 12usize, 2109116u32);
    emu.sli_no_count(10usize, 21usize, 27u32, 2109120u32);
    emu.orr_no_count(15usize, 10usize, 15usize, 2109124u32);
    emu.adr_no_count(10usize, 20usize, 21usize, 2109128u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2109132u32);
    emu.sli_no_count(14usize, 14usize, 8u32, 2109136u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2109140u32);
    emu.orr_no_count(11usize, 5usize, 11usize, 2109144u32);
    emu.orr_no_count(23usize, 15usize, 14usize, 2109148u32);
    emu.orr_no_count(22usize, 11usize, 12usize, 2109152u32);
    emu.sb_no_count(13usize, 10usize, 0u32, 2109156u32);
    emu.add_memory_rw_events(52usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a == b {
        emu.pc = 2109192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202f08));
    } else {
        emu.pc = 2109160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202ee8));
    }
}
#[inline(always)]
pub fn block_0x00202ee8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 10usize, 1u32, 2109164u32);
    emu.xri_no_count(12usize, 21usize, 63u32, 2109168u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2109172u32);
    emu.apc_no_count(1usize, 2109172u32, 28672u32, 2109176u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109180u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965368u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202efc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(10usize, 21usize, 56u32, 2109184u32);
    emu.adi_no_count(11usize, 0usize, 7u32, 2109188u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2109304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202f78));
    } else {
        emu.pc = 2109192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202f08));
    }
}
#[inline(always)]
pub fn block_0x00202f08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 1u32, 2109196u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2109200u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2109204u32);
    emu.apc_no_count(1usize, 2109204u32, 61440u32, 2109208u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109212u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1760u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202f1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 236u32, 2109216u32);
    emu.adi_no_count(12usize, 0usize, 56u32, 2109220u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2109224u32);
    emu.apc_no_count(1usize, 2109224u32, 28672u32, 2109228u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109232u32;
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
#[inline]
pub fn block_0x00202f30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 23usize, 24u32, 2109236u32);
    emu.sri_no_count(11usize, 23usize, 16u32, 2109240u32);
    emu.sri_no_count(12usize, 23usize, 8u32, 2109244u32);
    emu.sri_no_count(13usize, 22usize, 24u32, 2109248u32);
    emu.sri_no_count(14usize, 22usize, 16u32, 2109252u32);
    emu.sb_no_count(23usize, 2usize, 296u32, 2109256u32);
    emu.sb_no_count(12usize, 2usize, 297u32, 2109260u32);
    emu.sb_no_count(11usize, 2usize, 298u32, 2109264u32);
    emu.sb_no_count(10usize, 2usize, 299u32, 2109268u32);
    emu.sri_no_count(10usize, 22usize, 8u32, 2109272u32);
    emu.sb_no_count(22usize, 2usize, 292u32, 2109276u32);
    emu.sb_no_count(10usize, 2usize, 293u32, 2109280u32);
    emu.sb_no_count(14usize, 2usize, 294u32, 2109284u32);
    emu.sb_no_count(13usize, 2usize, 295u32, 2109288u32);
    emu.adi_no_count(11usize, 2usize, 236u32, 2109292u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2109296u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2109300u32);
    emu.add_memory_rw_events(18usize);
    let return_addr = 2109304u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2109324u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202f8c));
}
#[inline(always)]
pub fn block_0x00202f78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(22usize, 19usize, 96u32, 2109308u32)?;
    emu.sw_no_count(23usize, 19usize, 100u32, 2109312u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2109316u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2109320u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2109324u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2109324u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202f8c));
}
#[inline(always)]
pub fn block_0x00202f8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2109324u32, 61440u32, 2109328u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109332u32;
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
#[inline(never)]
pub fn block_0x00202f94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 143u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(0usize, 19usize, 104u32, 2109336u32);
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2109340u32;
    emu.update_insn_clock();
    emu.lw_no_count(16usize, 19usize, 0u32, 2109344u32)?;
    emu.lw_no_count(17usize, 19usize, 4u32, 2109348u32)?;
    emu.lw_no_count(10usize, 19usize, 8u32, 2109352u32)?;
    emu.lw_no_count(11usize, 19usize, 12u32, 2109356u32)?;
    emu.lw_no_count(12usize, 19usize, 16u32, 2109360u32)?;
    emu.lw_no_count(13usize, 19usize, 20u32, 2109364u32)?;
    emu.lw_no_count(14usize, 19usize, 24u32, 2109368u32)?;
    emu.lw_no_count(15usize, 19usize, 28u32, 2109372u32)?;
    emu.lw_no_count(6usize, 18usize, 48u32, 2109376u32)?;
    emu.lw_no_count(7usize, 18usize, 52u32, 2109380u32)?;
    emu.lw_no_count(28usize, 18usize, 56u32, 2109384u32)?;
    emu.lw_no_count(29usize, 18usize, 60u32, 2109388u32)?;
    emu.lw_no_count(30usize, 18usize, 32u32, 2109392u32)?;
    emu.lw_no_count(31usize, 18usize, 36u32, 2109396u32)?;
    emu.lw_no_count(19usize, 18usize, 40u32, 2109400u32)?;
    emu.lw_no_count(20usize, 18usize, 44u32, 2109404u32)?;
    emu.lw_no_count(21usize, 18usize, 0u32, 2109408u32)?;
    emu.lw_no_count(22usize, 18usize, 4u32, 2109412u32)?;
    emu.lw_no_count(23usize, 18usize, 8u32, 2109416u32)?;
    emu.lw_no_count(24usize, 18usize, 12u32, 2109420u32)?;
    emu.lw_no_count(25usize, 18usize, 16u32, 2109424u32)?;
    emu.lw_no_count(26usize, 18usize, 20u32, 2109428u32)?;
    emu.lw_no_count(27usize, 18usize, 24u32, 2109432u32)?;
    emu.lw_no_count(1usize, 18usize, 28u32, 2109436u32)?;
    emu.adi_no_count(5usize, 5usize, 4294967040u32, 2109440u32);
    emu.sri_no_count(8usize, 16usize, 8u32, 2109444u32);
    emu.sri_no_count(9usize, 16usize, 24u32, 2109448u32);
    emu.sw_no_count(6usize, 2usize, 188u32, 2109452u32)?;
    emu.anr_no_count(6usize, 16usize, 5usize, 2109456u32);
    emu.sli_no_count(16usize, 16usize, 24u32, 2109460u32);
    emu.sw_no_count(7usize, 2usize, 192u32, 2109464u32)?;
    emu.sri_no_count(7usize, 17usize, 8u32, 2109468u32);
    emu.sw_no_count(28usize, 2usize, 196u32, 2109472u32)?;
    emu.sri_no_count(28usize, 17usize, 24u32, 2109476u32);
    emu.sw_no_count(29usize, 2usize, 200u32, 2109480u32)?;
    emu.anr_no_count(29usize, 17usize, 5usize, 2109484u32);
    emu.sli_no_count(17usize, 17usize, 24u32, 2109488u32);
    emu.sw_no_count(30usize, 2usize, 172u32, 2109492u32)?;
    emu.sw_no_count(31usize, 2usize, 176u32, 2109496u32)?;
    emu.sw_no_count(19usize, 2usize, 180u32, 2109500u32)?;
    emu.sw_no_count(20usize, 2usize, 184u32, 2109504u32)?;
    emu.sri_no_count(30usize, 10usize, 8u32, 2109508u32);
    emu.sw_no_count(21usize, 2usize, 140u32, 2109512u32)?;
    emu.sw_no_count(22usize, 2usize, 144u32, 2109516u32)?;
    emu.sw_no_count(23usize, 2usize, 148u32, 2109520u32)?;
    emu.sw_no_count(24usize, 2usize, 152u32, 2109524u32)?;
    emu.sri_no_count(31usize, 10usize, 24u32, 2109528u32);
    emu.sw_no_count(25usize, 2usize, 156u32, 2109532u32)?;
    emu.sw_no_count(26usize, 2usize, 160u32, 2109536u32)?;
    emu.sw_no_count(27usize, 2usize, 164u32, 2109540u32)?;
    emu.sw_no_count(1usize, 2usize, 168u32, 2109544u32)?;
    emu.anr_no_count(19usize, 10usize, 5usize, 2109548u32);
    emu.sli_no_count(10usize, 10usize, 24u32, 2109552u32);
    emu.anr_no_count(8usize, 8usize, 5usize, 2109556u32);
    emu.orr_no_count(8usize, 8usize, 9usize, 2109560u32);
    emu.sri_no_count(9usize, 11usize, 8u32, 2109564u32);
    emu.sli_no_count(6usize, 6usize, 8u32, 2109568u32);
    emu.orr_no_count(16usize, 16usize, 6usize, 2109572u32);
    emu.sri_no_count(6usize, 11usize, 24u32, 2109576u32);
    emu.anr_no_count(7usize, 7usize, 5usize, 2109580u32);
    emu.orr_no_count(7usize, 7usize, 28usize, 2109584u32);
    emu.anr_no_count(28usize, 11usize, 5usize, 2109588u32);
    emu.sli_no_count(11usize, 11usize, 24u32, 2109592u32);
    emu.sli_no_count(29usize, 29usize, 8u32, 2109596u32);
    emu.orr_no_count(17usize, 17usize, 29usize, 2109600u32);
    emu.sri_no_count(29usize, 12usize, 8u32, 2109604u32);
    emu.anr_no_count(30usize, 30usize, 5usize, 2109608u32);
    emu.orr_no_count(30usize, 30usize, 31usize, 2109612u32);
    emu.sri_no_count(31usize, 12usize, 24u32, 2109616u32);
    emu.sli_no_count(19usize, 19usize, 8u32, 2109620u32);
    emu.orr_no_count(10usize, 10usize, 19usize, 2109624u32);
    emu.anr_no_count(19usize, 12usize, 5usize, 2109628u32);
    emu.sli_no_count(12usize, 12usize, 24u32, 2109632u32);
    emu.anr_no_count(9usize, 9usize, 5usize, 2109636u32);
    emu.orr_no_count(6usize, 9usize, 6usize, 2109640u32);
    emu.sri_no_count(9usize, 13usize, 8u32, 2109644u32);
    emu.sli_no_count(28usize, 28usize, 8u32, 2109648u32);
    emu.orr_no_count(11usize, 11usize, 28usize, 2109652u32);
    emu.sri_no_count(28usize, 13usize, 24u32, 2109656u32);
    emu.anr_no_count(29usize, 29usize, 5usize, 2109660u32);
    emu.orr_no_count(29usize, 29usize, 31usize, 2109664u32);
    emu.anr_no_count(31usize, 13usize, 5usize, 2109668u32);
    emu.sli_no_count(13usize, 13usize, 24u32, 2109672u32);
    emu.sli_no_count(19usize, 19usize, 8u32, 2109676u32);
    emu.orr_no_count(12usize, 12usize, 19usize, 2109680u32);
    emu.sri_no_count(19usize, 14usize, 8u32, 2109684u32);
    emu.anr_no_count(9usize, 9usize, 5usize, 2109688u32);
    emu.orr_no_count(28usize, 9usize, 28usize, 2109692u32);
    emu.sri_no_count(9usize, 14usize, 24u32, 2109696u32);
    emu.sli_no_count(31usize, 31usize, 8u32, 2109700u32);
    emu.orr_no_count(13usize, 13usize, 31usize, 2109704u32);
    emu.anr_no_count(31usize, 14usize, 5usize, 2109708u32);
    emu.sli_no_count(14usize, 14usize, 24u32, 2109712u32);
    emu.anr_no_count(19usize, 19usize, 5usize, 2109716u32);
    emu.orr_no_count(9usize, 19usize, 9usize, 2109720u32);
    emu.sri_no_count(19usize, 15usize, 8u32, 2109724u32);
    emu.sli_no_count(31usize, 31usize, 8u32, 2109728u32);
    emu.orr_no_count(14usize, 14usize, 31usize, 2109732u32);
    emu.sri_no_count(31usize, 15usize, 24u32, 2109736u32);
    emu.anr_no_count(19usize, 19usize, 5usize, 2109740u32);
    emu.orr_no_count(31usize, 19usize, 31usize, 2109744u32);
    emu.anr_no_count(5usize, 15usize, 5usize, 2109748u32);
    emu.sli_no_count(15usize, 15usize, 24u32, 2109752u32);
    emu.sli_no_count(5usize, 5usize, 8u32, 2109756u32);
    emu.orr_no_count(15usize, 15usize, 5usize, 2109760u32);
    emu.orr_no_count(16usize, 16usize, 8usize, 2109764u32);
    emu.orr_no_count(17usize, 17usize, 7usize, 2109768u32);
    emu.orr_no_count(10usize, 10usize, 30usize, 2109772u32);
    emu.lbu_no_count(20usize, 18usize, 64u32, 2109776u32);
    emu.orr_no_count(11usize, 11usize, 6usize, 2109780u32);
    emu.adi_no_count(19usize, 2usize, 172u32, 2109784u32);
    emu.orr_no_count(12usize, 12usize, 29usize, 2109788u32);
    emu.adi_no_count(18usize, 2usize, 204u32, 2109792u32);
    emu.orr_no_count(13usize, 13usize, 28usize, 2109796u32);
    emu.adi_no_count(5usize, 0usize, 4294967294u32, 2109800u32);
    emu.orr_no_count(14usize, 14usize, 9usize, 2109804u32);
    emu.adi_no_count(6usize, 0usize, 4294967295u32, 2109808u32);
    emu.orr_no_count(15usize, 15usize, 31usize, 2109812u32);
    emu.adi_no_count(7usize, 0usize, 1u32, 2109816u32);
    emu.sw_no_count(6usize, 2usize, 220u32, 2109820u32)?;
    emu.sw_no_count(6usize, 2usize, 224u32, 2109824u32)?;
    emu.sw_no_count(5usize, 2usize, 228u32, 2109828u32)?;
    emu.sw_no_count(0usize, 2usize, 232u32, 2109832u32)?;
    emu.sw_no_count(16usize, 2usize, 12u32, 2109836u32)?;
    emu.sw_no_count(17usize, 2usize, 16u32, 2109840u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2109844u32)?;
    emu.sw_no_count(11usize, 2usize, 24u32, 2109848u32)?;
    emu.sw_no_count(12usize, 2usize, 28u32, 2109852u32)?;
    emu.sw_no_count(13usize, 2usize, 32u32, 2109856u32)?;
    emu.sw_no_count(14usize, 2usize, 36u32, 2109860u32)?;
    emu.sw_no_count(15usize, 2usize, 40u32, 2109864u32)?;
    emu.sw_no_count(7usize, 2usize, 204u32, 2109868u32)?;
    emu.sw_no_count(0usize, 2usize, 208u32, 2109872u32)?;
    emu.sw_no_count(0usize, 2usize, 212u32, 2109876u32)?;
    emu.sw_no_count(6usize, 2usize, 216u32, 2109880u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2109884u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966680u32, 2109888u32);
    emu.adi_no_count(10usize, 2usize, 236u32, 2109892u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2109896u32);
    emu.apc_no_count(1usize, 2109896u32, 24576u32, 2109900u32);
    emu.add_memory_rw_events(143usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109904u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1692u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002031d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 20usize, 0u32, 2109908u32);
    emu.apc_no_count(1usize, 2109908u32, 81920u32, 2109912u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109916u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(932u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002031dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(20usize, 10usize, 255u32, 2109920u32);
    emu.adi_no_count(10usize, 2usize, 44u32, 2109924u32);
    emu.adi_no_count(11usize, 2usize, 140u32, 2109928u32);
    emu.adi_no_count(12usize, 2usize, 236u32, 2109932u32);
    emu.adi_no_count(13usize, 20usize, 0u32, 2109936u32);
    emu.apc_no_count(1usize, 2109936u32, 57344u32, 2109940u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109944u32;
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
#[inline(always)]
pub fn block_0x002031f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 2usize, 268u32, 2109948u32);
    emu.adi_no_count(10usize, 2usize, 76u32, 2109952u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2109956u32);
    emu.adi_no_count(13usize, 20usize, 0u32, 2109960u32);
    emu.apc_no_count(1usize, 2109960u32, 57344u32, 2109964u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109968u32;
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
#[inline(always)]
pub fn block_0x00203210(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 2usize, 300u32, 2109972u32);
    emu.adi_no_count(10usize, 2usize, 108u32, 2109976u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2109980u32);
    emu.adi_no_count(13usize, 20usize, 0u32, 2109984u32);
    emu.apc_no_count(1usize, 2109984u32, 57344u32, 2109988u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109992u32;
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
#[inline(always)]
pub fn block_0x00203228(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 2usize, 44u32, 2109996u32);
    emu.adi_no_count(12usize, 2usize, 12u32, 2110000u32);
    emu.lw_no_count(10usize, 2usize, 4u32, 2110004u32)?;
    emu.lw_no_count(13usize, 2usize, 8u32, 2110008u32)?;
    emu.apc_no_count(1usize, 2110008u32, 16384u32, 2110012u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110016u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966040u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00203240(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 380u32, 2110020u32)?;
    emu.lw_no_count(8usize, 2usize, 376u32, 2110024u32)?;
    emu.lw_no_count(9usize, 2usize, 372u32, 2110028u32)?;
    emu.lw_no_count(18usize, 2usize, 368u32, 2110032u32)?;
    emu.lw_no_count(19usize, 2usize, 364u32, 2110036u32)?;
    emu.lw_no_count(20usize, 2usize, 360u32, 2110040u32)?;
    emu.lw_no_count(21usize, 2usize, 356u32, 2110044u32)?;
    emu.lw_no_count(22usize, 2usize, 352u32, 2110048u32)?;
    emu.lw_no_count(23usize, 2usize, 348u32, 2110052u32)?;
    emu.lw_no_count(24usize, 2usize, 344u32, 2110056u32)?;
    emu.lw_no_count(25usize, 2usize, 340u32, 2110060u32)?;
    emu.lw_no_count(26usize, 2usize, 336u32, 2110064u32)?;
    emu.lw_no_count(27usize, 2usize, 332u32, 2110068u32)?;
    emu.adi_no_count(2usize, 2usize, 384u32, 2110072u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110076u32;
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
pub fn block_0x0020327c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2110080u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2110084u32)?;
    emu.adi_no_count(14usize, 10usize, 0u32, 2110088u32);
    emu.sb_no_count(11usize, 2usize, 16u32, 2110092u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2110096u32);
    emu.adi_no_count(12usize, 2usize, 16u32, 2110100u32);
    emu.adi_no_count(13usize, 0usize, 1u32, 2110104u32);
    emu.adi_no_count(11usize, 14usize, 0u32, 2110108u32);
    emu.apc_no_count(1usize, 2110108u32, 12288u32, 2110112u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110116u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966044u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002032a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 8u32, 2110120u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2110124u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2110144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002032c0));
    } else {
        emu.pc = 2110128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002032b0));
    }
}
#[inline(always)]
pub fn block_0x002032b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2110132u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2110136u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2110140u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110144u32;
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
pub fn block_0x002032c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2110148u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2110152u32)?;
    emu.sw_no_count(10usize, 2usize, 16u32, 2110156u32)?;
    emu.sw_no_count(11usize, 2usize, 20u32, 2110160u32)?;
    emu.adi_no_count(10usize, 2usize, 16u32, 2110164u32);
    emu.apc_no_count(1usize, 2110164u32, 28672u32, 2110168u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110172u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966584u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002032dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2110176u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2110180u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110184u32;
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
pub fn block_0x002032e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294965264u32, 2110188u32);
    emu.sw_no_count(1usize, 2usize, 2028u32, 2110192u32)?;
    emu.sw_no_count(8usize, 2usize, 2024u32, 2110196u32)?;
    emu.sw_no_count(9usize, 2usize, 2020u32, 2110200u32)?;
    emu.sw_no_count(18usize, 2usize, 2016u32, 2110204u32)?;
    emu.sw_no_count(19usize, 2usize, 2012u32, 2110208u32)?;
    emu.sw_no_count(20usize, 2usize, 2008u32, 2110212u32)?;
    emu.sw_no_count(21usize, 2usize, 2004u32, 2110216u32)?;
    emu.sw_no_count(22usize, 2usize, 2000u32, 2110220u32)?;
    emu.sw_no_count(23usize, 2usize, 1996u32, 2110224u32)?;
    emu.sw_no_count(24usize, 2usize, 1992u32, 2110228u32)?;
    emu.sw_no_count(25usize, 2usize, 1988u32, 2110232u32)?;
    emu.sw_no_count(26usize, 2usize, 1984u32, 2110236u32)?;
    emu.sw_no_count(27usize, 2usize, 1980u32, 2110240u32)?;
    emu.adi_no_count(2usize, 2usize, 4294967152u32, 2110244u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2110248u32)?;
    emu.adi_no_count(18usize, 11usize, 0u32, 2110252u32);
    emu.sw_no_count(10usize, 2usize, 4u32, 2110256u32)?;
    emu.adi_no_count(20usize, 2usize, 108u32, 2110260u32);
    emu.adi_no_count(19usize, 2usize, 204u32, 2110264u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(21usize, a);
    emu.pc = 2110268u32;
    emu.update_insn_clock();
    emu.adi_no_count(21usize, 21usize, 4294966680u32, 2110272u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2110276u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2110280u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2110284u32);
    emu.apc_no_count(1usize, 2110284u32, 24576u32, 2110288u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110292u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1304u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
