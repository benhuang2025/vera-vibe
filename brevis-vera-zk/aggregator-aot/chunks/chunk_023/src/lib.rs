pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2205976u32;
pub const PC_MAX: u32 = 2209544u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 110usize] = [
        block_0x0021a918,
        block_0x0021a944,
        block_0x0021a94c,
        block_0x0021a950,
        block_0x0021a958,
        block_0x0021a964,
        block_0x0021a974,
        block_0x0021a984,
        block_0x0021a98c,
        block_0x0021a9a4,
        block_0x0021a9c0,
        block_0x0021a9c4,
        block_0x0021a9e0,
        block_0x0021a9e8,
        block_0x0021aa14,
        block_0x0021aa4c,
        block_0x0021aa78,
        block_0x0021aa88,
        block_0x0021aaa8,
        block_0x0021aac8,
        block_0x0021aae8,
        block_0x0021ab00,
        block_0x0021ab08,
        block_0x0021ab20,
        block_0x0021ab48,
        block_0x0021ab54,
        block_0x0021ab74,
        block_0x0021ab8c,
        block_0x0021ab94,
        block_0x0021abac,
        block_0x0021abd4,
        block_0x0021abe0,
        block_0x0021ac00,
        block_0x0021ac18,
        block_0x0021ac20,
        block_0x0021ac38,
        block_0x0021ac60,
        block_0x0021ac6c,
        block_0x0021ac8c,
        block_0x0021aca4,
        block_0x0021acac,
        block_0x0021acc4,
        block_0x0021acec,
        block_0x0021acf8,
        block_0x0021ad1c,
        block_0x0021ad3c,
        block_0x0021ad78,
        block_0x0021ad7c,
        block_0x0021ad88,
        block_0x0021ad8c,
        block_0x0021adb0,
        block_0x0021add8,
        block_0x0021ade4,
        block_0x0021ae0c,
        block_0x0021ae38,
        block_0x0021aea8,
        block_0x0021aeb0,
        block_0x0021af04,
        block_0x0021af08,
        block_0x0021af0c,
        block_0x0021af2c,
        block_0x0021af54,
        block_0x0021af64,
        block_0x0021af70,
        block_0x0021af74,
        block_0x0021afa4,
        block_0x0021afd0,
        block_0x0021b040,
        block_0x0021b048,
        block_0x0021b09c,
        block_0x0021b0a0,
        block_0x0021b0a4,
        block_0x0021b0c4,
        block_0x0021b0f0,
        block_0x0021b100,
        block_0x0021b10c,
        block_0x0021b110,
        block_0x0021b12c,
        block_0x0021b158,
        block_0x0021b1cc,
        block_0x0021b1d4,
        block_0x0021b224,
        block_0x0021b228,
        block_0x0021b22c,
        block_0x0021b240,
        block_0x0021b270,
        block_0x0021b280,
        block_0x0021b284,
        block_0x0021b2c4,
        block_0x0021b2e4,
        block_0x0021b2fc,
        block_0x0021b35c,
        block_0x0021b384,
        block_0x0021b3a0,
        block_0x0021b418,
        block_0x0021b428,
        block_0x0021b488,
        block_0x0021b48c,
        block_0x0021b4a8,
        block_0x0021b4b4,
        block_0x0021b4bc,
        block_0x0021b4d8,
        block_0x0021b518,
        block_0x0021b538,
        block_0x0021b574,
        block_0x0021b5d4,
        block_0x0021b62c,
        block_0x0021b688,
        block_0x0021b6a0,
        block_0x0021b708,
    ];
    const IDX: [u16; 893usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16,
        3u16, 4u16, 0u16, 5u16, 0u16, 0u16, 6u16, 0u16, 0u16, 0u16, 7u16, 0u16, 0u16,
        0u16, 8u16, 0u16, 9u16, 0u16, 0u16, 0u16, 0u16, 0u16, 10u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 11u16, 12u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 13u16, 0u16,
        14u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 15u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 16u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 17u16, 0u16, 0u16,
        0u16, 18u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 19u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 20u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 21u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 22u16, 0u16, 23u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        24u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 25u16, 0u16, 0u16,
        26u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 27u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 28u16, 0u16, 29u16, 0u16, 0u16, 0u16, 0u16, 0u16, 30u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 31u16, 0u16, 0u16, 32u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 33u16, 0u16, 0u16, 0u16, 0u16, 0u16, 34u16, 0u16, 35u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 36u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 37u16, 0u16, 0u16, 38u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        39u16, 0u16, 0u16, 0u16, 0u16, 0u16, 40u16, 0u16, 41u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 42u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 43u16, 0u16,
        0u16, 44u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 45u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 46u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 47u16, 48u16, 0u16, 0u16, 49u16, 50u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 51u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 52u16, 0u16, 0u16, 53u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 54u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 55u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 56u16, 0u16, 57u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 58u16, 59u16, 60u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 61u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 62u16, 0u16, 0u16, 0u16,
        63u16, 0u16, 0u16, 64u16, 65u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 66u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 67u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 68u16, 0u16, 69u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        70u16, 71u16, 72u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 73u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 74u16, 0u16, 0u16, 0u16, 75u16,
        0u16, 0u16, 76u16, 77u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 78u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 79u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 80u16, 0u16,
        81u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 82u16, 83u16, 84u16, 0u16, 0u16, 0u16,
        0u16, 85u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        86u16, 0u16, 0u16, 0u16, 87u16, 88u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 89u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 90u16, 0u16, 0u16, 0u16, 0u16, 0u16, 91u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 92u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 93u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 94u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 95u16, 0u16, 0u16, 0u16, 96u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 97u16, 98u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 99u16,
        0u16, 0u16, 100u16, 0u16, 101u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 102u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 103u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 104u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 105u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 106u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 107u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 108u16, 0u16, 0u16, 0u16, 0u16, 0u16, 109u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 110u16,
    ];
    if pc < 2205976u32 || pc > 2209544u32 {
        return None;
    }
    let word_offset = ((pc - 2205976u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x0021a918(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2205980u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2205984u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2205988u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2205992u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2205996u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2206000u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2206004u32)?;
    emu.lw_no_count(8usize, 10usize, 8u32, 2206008u32)?;
    emu.adi_no_count(12usize, 0usize, 128u32, 2206012u32);
    emu.sri_no_count(19usize, 11usize, 11u32, 2206016u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2206028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a94c));
    } else {
        emu.pc = 2206020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a944));
    }
}
#[inline(always)]
pub fn block_0x0021a944(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 1u32, 2206024u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2206028u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2206052u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021a964));
}
#[inline(always)]
pub fn block_0x0021a94c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a != b {
        emu.pc = 2206040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a958));
    } else {
        emu.pc = 2206032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a950));
    }
}
#[inline(always)]
pub fn block_0x0021a950(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 2u32, 2206036u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2206040u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2206052u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021a964));
}
#[inline(always)]
pub fn block_0x0021a958(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 11usize, 16u32, 2206044u32);
    emu.sltru_no_count(9usize, 0usize, 12usize, 2206048u32);
    emu.adi_no_count(9usize, 9usize, 3u32, 2206052u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2206052u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021a964));
}
#[inline(always)]
pub fn block_0x0021a964(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2206056u32)?;
    emu.sbr_no_count(13usize, 12usize, 8usize, 2206060u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2206064u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2206092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a98c));
    } else {
        emu.pc = 2206068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a974));
    }
}
#[inline(always)]
pub fn block_0x0021a974(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 4u32, 2206072u32)?;
    emu.adi_no_count(14usize, 0usize, 128u32, 2206076u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2206080u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2206144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a9c0));
    } else {
        emu.pc = 2206084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a984));
    }
}
#[inline(always)]
pub fn block_0x0021a984(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 12usize, 0u32, 2206088u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2206092u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2206284u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021aa4c));
}
#[inline(always)]
pub fn block_0x0021a98c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2206096u32);
    emu.adi_no_count(20usize, 11usize, 0u32, 2206100u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2206104u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2206108u32);
    emu.apc_no_count(1usize, 2206108u32, 0u32, 2206112u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2206116u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965648u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021a9a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 20usize, 0u32, 2206120u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2206124u32);
    emu.lw_no_count(12usize, 18usize, 8u32, 2206128u32)?;
    emu.lw_no_count(13usize, 18usize, 4u32, 2206132u32)?;
    emu.adi_no_count(14usize, 0usize, 128u32, 2206136u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2206140u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a < b {
        emu.pc = 2206084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a984));
    } else {
        emu.pc = 2206144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a9c0));
    }
}
#[inline(always)]
pub fn block_0x0021a9c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a != b {
        emu.pc = 2206176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a9e0));
    } else {
        emu.pc = 2206148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a9c4));
    }
}
#[inline(always)]
pub fn block_0x0021a9c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 6u32, 2206152u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2206156u32);
    emu.ori_no_count(13usize, 13usize, 192u32, 2206160u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2206164u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2206168u32);
    emu.sb_no_count(11usize, 12usize, 1u32, 2206172u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2206176u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2206284u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021aa4c));
}
#[inline(always)]
pub fn block_0x0021a9e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 16u32, 2206180u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2206228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021aa14));
    } else {
        emu.pc = 2206184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a9e8));
    }
}
#[inline]
pub fn block_0x0021a9e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 12u32, 2206188u32);
    emu.sli_no_count(14usize, 11usize, 20u32, 2206192u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2206196u32);
    emu.ori_no_count(13usize, 13usize, 224u32, 2206200u32);
    emu.sri_no_count(14usize, 14usize, 26u32, 2206204u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2206208u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2206212u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2206216u32);
    emu.sb_no_count(14usize, 12usize, 1u32, 2206220u32);
    emu.sb_no_count(11usize, 12usize, 2u32, 2206224u32);
    emu.add_memory_rw_events(11usize);
    let return_addr = 2206228u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2206284u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021aa4c));
}
#[inline]
pub fn block_0x0021aa14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 18u32, 2206232u32);
    emu.sli_no_count(14usize, 11usize, 14u32, 2206236u32);
    emu.sli_no_count(15usize, 11usize, 20u32, 2206240u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2206244u32);
    emu.ori_no_count(13usize, 13usize, 240u32, 2206248u32);
    emu.sri_no_count(14usize, 14usize, 26u32, 2206252u32);
    emu.sri_no_count(15usize, 15usize, 26u32, 2206256u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2206260u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2206264u32);
    emu.adi_no_count(15usize, 15usize, 128u32, 2206268u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2206272u32);
    emu.sb_no_count(14usize, 12usize, 1u32, 2206276u32);
    emu.sb_no_count(15usize, 12usize, 2u32, 2206280u32);
    emu.sb_no_count(11usize, 12usize, 3u32, 2206284u32);
    emu.add_memory_rw_events(14usize);
    emu.pc = 2206284u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021aa4c));
}
#[inline]
pub fn block_0x0021aa4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 9usize, 8usize, 2206288u32);
    emu.sw_no_count(8usize, 10usize, 8u32, 2206292u32)?;
    emu.adi_no_count(10usize, 0usize, 0u32, 2206296u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2206300u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2206304u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2206308u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2206312u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2206316u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2206320u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2206324u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2206328u32;
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
pub fn block_0x0021aa78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 4u32, 2206332u32)?;
    emu.lw_no_count(11usize, 10usize, 8u32, 2206336u32)?;
    emu.adi_no_count(10usize, 12usize, 0u32, 2206340u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2206344u32;
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
pub fn block_0x0021aa88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 4u32, 2206348u32)?;
    emu.lw_no_count(13usize, 10usize, 8u32, 2206352u32)?;
    emu.adi_no_count(14usize, 11usize, 0u32, 2206356u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2206360u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2206364u32);
    emu.adi_no_count(12usize, 14usize, 0u32, 2206368u32);
    emu.apc_no_count(6usize, 2206368u32, 12288u32, 2206372u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2206376u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x0021aaa8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 4u32, 2206380u32)?;
    emu.lw_no_count(13usize, 10usize, 8u32, 2206384u32)?;
    emu.adi_no_count(14usize, 11usize, 0u32, 2206388u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2206392u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2206396u32);
    emu.adi_no_count(12usize, 14usize, 0u32, 2206400u32);
    emu.apc_no_count(6usize, 2206400u32, 12288u32, 2206404u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2206408u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x0021aac8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967152u32, 2206412u32);
    emu.sw_no_count(1usize, 2usize, 140u32, 2206416u32)?;
    emu.adi_no_count(16usize, 11usize, 0u32, 2206420u32);
    emu.adi_no_count(15usize, 0usize, 0u32, 2206424u32);
    emu.lbu_no_count(10usize, 10usize, 0u32, 2206428u32);
    emu.adi_no_count(11usize, 2usize, 139u32, 2206432u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2206436u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2206440u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2206464u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ab00));
}
#[inline(always)]
pub fn block_0x0021aae8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 87u32, 2206444u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2206448u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2206452u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2206456u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2206460u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2206496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ab20));
    } else {
        emu.pc = 2206464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ab00));
    }
}
#[inline(always)]
pub fn block_0x0021ab00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(13usize, 10usize, 15u32, 2206468u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2206440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021aae8));
    } else {
        emu.pc = 2206472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ab08));
    }
}
#[inline(always)]
pub fn block_0x0021ab08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 48u32, 2206476u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2206480u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2206484u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2206488u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2206492u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2206464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ab00));
    } else {
        emu.pc = 2206496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ab20));
    }
}
#[inline]
pub fn block_0x0021ab20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2206500u32);
    emu.sbr_no_count(10usize, 10usize, 15usize, 2206504u32);
    emu.adi_no_count(14usize, 10usize, 128u32, 2206508u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2206512u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1328u32, 2206516u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2206520u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2206524u32);
    emu.adi_no_count(10usize, 16usize, 0u32, 2206528u32);
    emu.apc_no_count(1usize, 2206528u32, 8192u32, 2206532u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2206536u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1052u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021ab48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 140u32, 2206540u32)?;
    emu.adi_no_count(2usize, 2usize, 144u32, 2206544u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2206548u32;
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
pub fn block_0x0021ab54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967152u32, 2206552u32);
    emu.sw_no_count(1usize, 2usize, 140u32, 2206556u32)?;
    emu.adi_no_count(16usize, 11usize, 0u32, 2206560u32);
    emu.adi_no_count(15usize, 0usize, 0u32, 2206564u32);
    emu.lbu_no_count(10usize, 10usize, 0u32, 2206568u32);
    emu.adi_no_count(11usize, 2usize, 139u32, 2206572u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2206576u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2206580u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2206604u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ab8c));
}
#[inline(always)]
pub fn block_0x0021ab74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 55u32, 2206584u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2206588u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2206592u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2206596u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2206600u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2206636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021abac));
    } else {
        emu.pc = 2206604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ab8c));
    }
}
#[inline(always)]
pub fn block_0x0021ab8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(13usize, 10usize, 15u32, 2206608u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2206580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ab74));
    } else {
        emu.pc = 2206612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ab94));
    }
}
#[inline(always)]
pub fn block_0x0021ab94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 48u32, 2206616u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2206620u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2206624u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2206628u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2206632u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2206604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ab8c));
    } else {
        emu.pc = 2206636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021abac));
    }
}
#[inline]
pub fn block_0x0021abac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2206640u32);
    emu.sbr_no_count(10usize, 10usize, 15usize, 2206644u32);
    emu.adi_no_count(14usize, 10usize, 128u32, 2206648u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2206652u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1328u32, 2206656u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2206660u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2206664u32);
    emu.adi_no_count(10usize, 16usize, 0u32, 2206668u32);
    emu.apc_no_count(1usize, 2206668u32, 8192u32, 2206672u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2206676u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(912u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021abd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 140u32, 2206680u32)?;
    emu.adi_no_count(2usize, 2usize, 144u32, 2206684u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2206688u32;
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
pub fn block_0x0021abe0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967152u32, 2206692u32);
    emu.sw_no_count(1usize, 2usize, 140u32, 2206696u32)?;
    emu.adi_no_count(16usize, 11usize, 0u32, 2206700u32);
    emu.adi_no_count(15usize, 0usize, 0u32, 2206704u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2206708u32)?;
    emu.adi_no_count(11usize, 2usize, 139u32, 2206712u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2206716u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2206720u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2206744u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ac18));
}
#[inline(always)]
pub fn block_0x0021ac00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 87u32, 2206724u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2206728u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2206732u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2206736u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2206740u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2206776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ac38));
    } else {
        emu.pc = 2206744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ac18));
    }
}
#[inline(always)]
pub fn block_0x0021ac18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(13usize, 10usize, 15u32, 2206748u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2206720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ac00));
    } else {
        emu.pc = 2206752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ac20));
    }
}
#[inline(always)]
pub fn block_0x0021ac20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 48u32, 2206756u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2206760u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2206764u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2206768u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2206772u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2206744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ac18));
    } else {
        emu.pc = 2206776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ac38));
    }
}
#[inline]
pub fn block_0x0021ac38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2206780u32);
    emu.sbr_no_count(10usize, 10usize, 15usize, 2206784u32);
    emu.adi_no_count(14usize, 10usize, 128u32, 2206788u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2206792u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1328u32, 2206796u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2206800u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2206804u32);
    emu.adi_no_count(10usize, 16usize, 0u32, 2206808u32);
    emu.apc_no_count(1usize, 2206808u32, 8192u32, 2206812u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2206816u32;
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
#[inline(always)]
pub fn block_0x0021ac60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 140u32, 2206820u32)?;
    emu.adi_no_count(2usize, 2usize, 144u32, 2206824u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2206828u32;
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
pub fn block_0x0021ac6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967152u32, 2206832u32);
    emu.sw_no_count(1usize, 2usize, 140u32, 2206836u32)?;
    emu.adi_no_count(16usize, 11usize, 0u32, 2206840u32);
    emu.adi_no_count(15usize, 0usize, 0u32, 2206844u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2206848u32)?;
    emu.adi_no_count(11usize, 2usize, 139u32, 2206852u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2206856u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2206860u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2206884u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021aca4));
}
#[inline(always)]
pub fn block_0x0021ac8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 55u32, 2206864u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2206868u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2206872u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2206876u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2206880u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2206916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021acc4));
    } else {
        emu.pc = 2206884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021aca4));
    }
}
#[inline(always)]
pub fn block_0x0021aca4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(13usize, 10usize, 15u32, 2206888u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2206860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ac8c));
    } else {
        emu.pc = 2206892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021acac));
    }
}
#[inline(always)]
pub fn block_0x0021acac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 48u32, 2206896u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2206900u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2206904u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2206908u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2206912u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2206884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021aca4));
    } else {
        emu.pc = 2206916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021acc4));
    }
}
#[inline]
pub fn block_0x0021acc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2206920u32);
    emu.sbr_no_count(10usize, 10usize, 15usize, 2206924u32);
    emu.adi_no_count(14usize, 10usize, 128u32, 2206928u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2206932u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1328u32, 2206936u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2206940u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2206944u32);
    emu.adi_no_count(10usize, 16usize, 0u32, 2206948u32);
    emu.apc_no_count(1usize, 2206948u32, 8192u32, 2206952u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2206956u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(632u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021acec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 140u32, 2206960u32)?;
    emu.adi_no_count(2usize, 2usize, 144u32, 2206964u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2206968u32;
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
pub fn block_0x0021acf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 4294967295u32, 2206972u32);
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2206976u32;
    emu.update_insn_clock();
    emu.sbr_no_count(13usize, 13usize, 11usize, 2206980u32);
    emu.xrr_no_count(11usize, 11usize, 12usize, 2206984u32);
    emu.sltru_no_count(11usize, 12usize, 11usize, 2206988u32);
    emu.sltru_no_count(10usize, 13usize, 10usize, 2206992u32);
    emu.xri_no_count(10usize, 10usize, 1u32, 2206996u32);
    emu.anr_no_count(10usize, 11usize, 10usize, 2207000u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2207004u32;
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
pub fn block_0x0021ad1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2207008u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2207012u32)?;
    emu.adi_no_count(13usize, 11usize, 0u32, 2207016u32);
    emu.lbu_no_count(11usize, 10usize, 0u32, 2207020u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2207024u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2207028u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1330u32, 2207032u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2207100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ad7c));
    } else {
        emu.pc = 2207036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ad3c));
    }
}
#[inline]
pub fn block_0x0021ad3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 41u32, 2207040u32);
    emu.adi_no_count(14usize, 0usize, 100u32, 2207044u32);
    emu.mul_no_count(12usize, 11usize, 12usize, 2207048u32);
    emu.sri_no_count(12usize, 12usize, 12u32, 2207052u32);
    emu.mul_no_count(14usize, 12usize, 14usize, 2207056u32);
    emu.sbr_no_count(14usize, 11usize, 14usize, 2207060u32);
    emu.sli_no_count(14usize, 14usize, 25u32, 2207064u32);
    emu.sri_no_count(14usize, 14usize, 24u32, 2207068u32);
    emu.adr_no_count(14usize, 10usize, 14usize, 2207072u32);
    emu.lbu_no_count(15usize, 14usize, 0u32, 2207076u32);
    emu.lbu_no_count(14usize, 14usize, 1u32, 2207080u32);
    emu.sb_no_count(15usize, 2usize, 10u32, 2207084u32);
    emu.sb_no_count(14usize, 2usize, 11u32, 2207088u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2207092u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2207112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ad88));
    } else {
        emu.pc = 2207096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ad78));
    }
}
#[inline(always)]
pub fn block_0x0021ad78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2207100u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2207116u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ad8c));
}
#[inline(always)]
pub fn block_0x0021ad7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 3u32, 2207104u32);
    emu.adi_no_count(12usize, 11usize, 0u32, 2207108u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2207116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ad8c));
    } else {
        emu.pc = 2207112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ad88));
    }
}
#[inline(always)]
pub fn block_0x0021ad88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2207152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021adb0));
    } else {
        emu.pc = 2207116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ad8c));
    }
}
#[inline]
pub fn block_0x0021ad8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 1u32, 2207120u32);
    emu.adi_no_count(12usize, 12usize, 1u32, 2207124u32);
    emu.ani_no_count(11usize, 12usize, 255u32, 2207128u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2207132u32);
    emu.lbu_no_count(10usize, 10usize, 0u32, 2207136u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2207140u32);
    emu.adi_no_count(11usize, 2usize, 9u32, 2207144u32);
    emu.adr_no_count(11usize, 11usize, 14usize, 2207148u32);
    emu.sb_no_count(10usize, 11usize, 0u32, 2207152u32);
    emu.add_memory_rw_events(9usize);
    emu.pc = 2207152u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021adb0));
}
#[inline]
pub fn block_0x0021adb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 3u32, 2207156u32);
    emu.adi_no_count(10usize, 2usize, 9u32, 2207160u32);
    emu.sbr_no_count(15usize, 15usize, 14usize, 2207164u32);
    emu.adr_no_count(14usize, 10usize, 14usize, 2207168u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2207172u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2207176u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2207180u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2207184u32);
    emu.apc_no_count(1usize, 2207184u32, 8192u32, 2207188u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2207192u32;
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
pub fn block_0x0021add8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2207196u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2207200u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2207204u32;
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
pub fn block_0x0021ade4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2207208u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2207212u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2207216u32)?;
    emu.adi_no_count(13usize, 11usize, 0u32, 2207220u32);
    emu.lw_no_count(11usize, 10usize, 0u32, 2207224u32)?;
    emu.adi_no_count(12usize, 0usize, 1000u32, 2207228u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2207232u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1330u32, 2207236u32);
    emu.adi_no_count(14usize, 0usize, 10u32, 2207240u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2207588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021af64));
    } else {
        emu.pc = 2207244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ae0c));
    }
}
#[inline]
pub fn block_0x0021ae0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 2usize, 23u32, 2207248u32);
    let a = 0u32.wrapping_add(3518435328u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2207252u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2207256u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2207260u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 0usize, 100u32, 2207264u32);
    let a = 0u32.wrapping_add(9998336u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2207268u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 12usize, 1881u32, 2207272u32);
    emu.adi_no_count(5usize, 5usize, 1808u32, 2207276u32);
    emu.adi_no_count(6usize, 6usize, 1147u32, 2207280u32);
    emu.adi_no_count(7usize, 7usize, 1663u32, 2207284u32);
    emu.adi_no_count(12usize, 11usize, 0u32, 2207288u32);
    emu.add_memory_rw_events(11usize);
    emu.pc = 2207288u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ae38));
}
#[inline(never)]
pub fn block_0x0021ae38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(28usize, 12usize, 0u32, 2207292u32);
    emu.adi_no_count(14usize, 14usize, 4294967292u32, 2207296u32);
    emu.mulhu_no_count(12usize, 12usize, 17usize, 2207300u32);
    emu.sri_no_count(12usize, 12usize, 13u32, 2207304u32);
    emu.mul_no_count(29usize, 12usize, 5usize, 2207308u32);
    emu.sbr_no_count(29usize, 28usize, 29usize, 2207312u32);
    emu.sli_no_count(30usize, 29usize, 16u32, 2207316u32);
    emu.sri_no_count(30usize, 30usize, 18u32, 2207320u32);
    emu.mul_no_count(30usize, 30usize, 6usize, 2207324u32);
    emu.sri_no_count(31usize, 30usize, 16u32, 2207328u32);
    emu.sri_no_count(30usize, 30usize, 17u32, 2207332u32);
    emu.mul_no_count(30usize, 30usize, 16usize, 2207336u32);
    emu.ani_no_count(31usize, 31usize, 2046u32, 2207340u32);
    emu.sbr_no_count(29usize, 29usize, 30usize, 2207344u32);
    emu.adr_no_count(31usize, 10usize, 31usize, 2207348u32);
    emu.sli_no_count(29usize, 29usize, 17u32, 2207352u32);
    emu.sri_no_count(29usize, 29usize, 16u32, 2207356u32);
    emu.adr_no_count(29usize, 10usize, 29usize, 2207360u32);
    emu.lbu_no_count(30usize, 31usize, 0u32, 2207364u32);
    emu.lbu_no_count(31usize, 31usize, 1u32, 2207368u32);
    emu.lbu_no_count(8usize, 29usize, 0u32, 2207372u32);
    emu.lbu_no_count(29usize, 29usize, 1u32, 2207376u32);
    emu.sb_no_count(30usize, 15usize, 4294967293u32, 2207380u32);
    emu.sb_no_count(31usize, 15usize, 4294967294u32, 2207384u32);
    emu.sb_no_count(8usize, 15usize, 4294967295u32, 2207388u32);
    emu.sb_no_count(29usize, 15usize, 0u32, 2207392u32);
    emu.adi_no_count(15usize, 15usize, 4294967292u32, 2207396u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a < b {
        emu.pc = 2207288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ae38));
    } else {
        emu.pc = 2207400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021aea8));
    }
}
#[inline(always)]
pub fn block_0x0021aea8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 9u32, 2207404u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2207492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021af04));
    } else {
        emu.pc = 2207408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021aeb0));
    }
}
#[inline]
pub fn block_0x0021aeb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(15usize, 12usize, 16u32, 2207412u32);
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2207416u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 0usize, 100u32, 2207420u32);
    emu.adi_no_count(5usize, 2usize, 14u32, 2207424u32);
    emu.sri_no_count(15usize, 15usize, 18u32, 2207428u32);
    emu.adi_no_count(16usize, 16usize, 1147u32, 2207432u32);
    emu.adr_no_count(6usize, 5usize, 14usize, 2207436u32);
    emu.mul_no_count(15usize, 15usize, 16usize, 2207440u32);
    emu.sri_no_count(15usize, 15usize, 17u32, 2207444u32);
    emu.mul_no_count(16usize, 15usize, 17usize, 2207448u32);
    emu.sbr_no_count(12usize, 12usize, 16usize, 2207452u32);
    emu.sli_no_count(12usize, 12usize, 17u32, 2207456u32);
    emu.sri_no_count(12usize, 12usize, 16u32, 2207460u32);
    emu.adr_no_count(12usize, 10usize, 12usize, 2207464u32);
    emu.lbu_no_count(16usize, 12usize, 0u32, 2207468u32);
    emu.lbu_no_count(12usize, 12usize, 1u32, 2207472u32);
    emu.adi_no_count(14usize, 14usize, 4294967294u32, 2207476u32);
    emu.adr_no_count(5usize, 5usize, 14usize, 2207480u32);
    emu.sb_no_count(16usize, 5usize, 0u32, 2207484u32);
    emu.sb_no_count(12usize, 6usize, 4294967295u32, 2207488u32);
    emu.adi_no_count(12usize, 15usize, 0u32, 2207492u32);
    emu.add_memory_rw_events(21usize);
    emu.pc = 2207492u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021af04));
}
#[inline(always)]
pub fn block_0x0021af04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2207500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021af0c));
    } else {
        emu.pc = 2207496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021af08));
    }
}
#[inline(always)]
pub fn block_0x0021af08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2207532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021af2c));
    } else {
        emu.pc = 2207500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021af0c));
    }
}
#[inline(always)]
pub fn block_0x0021af0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 1u32, 2207504u32);
    emu.ani_no_count(12usize, 12usize, 30u32, 2207508u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2207512u32);
    emu.lbu_no_count(10usize, 10usize, 1u32, 2207516u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2207520u32);
    emu.adi_no_count(11usize, 2usize, 14u32, 2207524u32);
    emu.adr_no_count(11usize, 11usize, 14usize, 2207528u32);
    emu.sb_no_count(10usize, 11usize, 0u32, 2207532u32);
    emu.add_memory_rw_events(8usize);
    emu.pc = 2207532u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021af2c));
}
#[inline]
pub fn block_0x0021af2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 10u32, 2207536u32);
    emu.adi_no_count(10usize, 2usize, 14u32, 2207540u32);
    emu.sbr_no_count(15usize, 15usize, 14usize, 2207544u32);
    emu.adr_no_count(14usize, 10usize, 14usize, 2207548u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2207552u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2207556u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2207560u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2207564u32);
    emu.apc_no_count(1usize, 2207564u32, 8192u32, 2207568u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2207572u32;
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
pub fn block_0x0021af54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2207576u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2207580u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2207584u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2207588u32;
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
pub fn block_0x0021af64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 0u32, 2207592u32);
    emu.adi_no_count(15usize, 0usize, 9u32, 2207596u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2207408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021aeb0));
    } else {
        emu.pc = 2207600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021af70));
    }
}
#[inline(always)]
pub fn block_0x0021af70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2207604u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2207492u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021af04));
}
#[inline]
pub fn block_0x0021af74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2207608u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2207612u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2207616u32)?;
    emu.adi_no_count(13usize, 11usize, 0u32, 2207620u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2207624u32)?;
    emu.sai_no_count(11usize, 10usize, 1055u32, 2207628u32);
    emu.xrr_no_count(12usize, 10usize, 11usize, 2207632u32);
    emu.sbr_no_count(12usize, 12usize, 11usize, 2207636u32);
    emu.adi_no_count(14usize, 0usize, 1000u32, 2207640u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2207644u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1330u32, 2207648u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2208000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b100));
    } else {
        emu.pc = 2207652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021afa4));
    }
}
#[inline]
pub fn block_0x0021afa4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 10u32, 2207656u32);
    emu.adi_no_count(15usize, 2usize, 23u32, 2207660u32);
    let a = 0u32.wrapping_add(3518435328u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2207664u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2207668u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2207672u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 0usize, 100u32, 2207676u32);
    let a = 0u32.wrapping_add(9998336u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2207680u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 1881u32, 2207684u32);
    emu.adi_no_count(5usize, 5usize, 1808u32, 2207688u32);
    emu.adi_no_count(6usize, 6usize, 1147u32, 2207692u32);
    emu.adi_no_count(7usize, 7usize, 1663u32, 2207696u32);
    emu.add_memory_rw_events(11usize);
    emu.pc = 2207696u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021afd0));
}
#[inline(never)]
pub fn block_0x0021afd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(28usize, 12usize, 0u32, 2207700u32);
    emu.adi_no_count(14usize, 14usize, 4294967292u32, 2207704u32);
    emu.mulhu_no_count(12usize, 12usize, 17usize, 2207708u32);
    emu.sri_no_count(12usize, 12usize, 13u32, 2207712u32);
    emu.mul_no_count(29usize, 12usize, 5usize, 2207716u32);
    emu.sbr_no_count(29usize, 28usize, 29usize, 2207720u32);
    emu.sli_no_count(30usize, 29usize, 16u32, 2207724u32);
    emu.sri_no_count(30usize, 30usize, 18u32, 2207728u32);
    emu.mul_no_count(30usize, 30usize, 6usize, 2207732u32);
    emu.sri_no_count(31usize, 30usize, 16u32, 2207736u32);
    emu.sri_no_count(30usize, 30usize, 17u32, 2207740u32);
    emu.mul_no_count(30usize, 30usize, 16usize, 2207744u32);
    emu.ani_no_count(31usize, 31usize, 2046u32, 2207748u32);
    emu.sbr_no_count(29usize, 29usize, 30usize, 2207752u32);
    emu.adr_no_count(31usize, 11usize, 31usize, 2207756u32);
    emu.sli_no_count(29usize, 29usize, 17u32, 2207760u32);
    emu.sri_no_count(29usize, 29usize, 16u32, 2207764u32);
    emu.adr_no_count(29usize, 11usize, 29usize, 2207768u32);
    emu.lbu_no_count(30usize, 31usize, 0u32, 2207772u32);
    emu.lbu_no_count(31usize, 31usize, 1u32, 2207776u32);
    emu.lbu_no_count(8usize, 29usize, 0u32, 2207780u32);
    emu.lbu_no_count(29usize, 29usize, 1u32, 2207784u32);
    emu.sb_no_count(30usize, 15usize, 4294967293u32, 2207788u32);
    emu.sb_no_count(31usize, 15usize, 4294967294u32, 2207792u32);
    emu.sb_no_count(8usize, 15usize, 4294967295u32, 2207796u32);
    emu.sb_no_count(29usize, 15usize, 0u32, 2207800u32);
    emu.adi_no_count(15usize, 15usize, 4294967292u32, 2207804u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a < b {
        emu.pc = 2207696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021afd0));
    } else {
        emu.pc = 2207808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b040));
    }
}
#[inline(always)]
pub fn block_0x0021b040(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 9u32, 2207812u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2207900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b09c));
    } else {
        emu.pc = 2207816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b048));
    }
}
#[inline]
pub fn block_0x0021b048(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(15usize, 12usize, 16u32, 2207820u32);
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2207824u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 0usize, 100u32, 2207828u32);
    emu.adi_no_count(5usize, 2usize, 14u32, 2207832u32);
    emu.sri_no_count(15usize, 15usize, 18u32, 2207836u32);
    emu.adi_no_count(16usize, 16usize, 1147u32, 2207840u32);
    emu.adr_no_count(6usize, 5usize, 14usize, 2207844u32);
    emu.mul_no_count(15usize, 15usize, 16usize, 2207848u32);
    emu.sri_no_count(15usize, 15usize, 17u32, 2207852u32);
    emu.mul_no_count(16usize, 15usize, 17usize, 2207856u32);
    emu.sbr_no_count(12usize, 12usize, 16usize, 2207860u32);
    emu.sli_no_count(12usize, 12usize, 17u32, 2207864u32);
    emu.sri_no_count(12usize, 12usize, 16u32, 2207868u32);
    emu.adr_no_count(12usize, 11usize, 12usize, 2207872u32);
    emu.lbu_no_count(16usize, 12usize, 0u32, 2207876u32);
    emu.lbu_no_count(12usize, 12usize, 1u32, 2207880u32);
    emu.adi_no_count(14usize, 14usize, 4294967294u32, 2207884u32);
    emu.adr_no_count(5usize, 5usize, 14usize, 2207888u32);
    emu.sb_no_count(16usize, 5usize, 0u32, 2207892u32);
    emu.sb_no_count(12usize, 6usize, 4294967295u32, 2207896u32);
    emu.adi_no_count(12usize, 15usize, 0u32, 2207900u32);
    emu.add_memory_rw_events(21usize);
    emu.pc = 2207900u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b09c));
}
#[inline(always)]
pub fn block_0x0021b09c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2207908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b0a4));
    } else {
        emu.pc = 2207904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b0a0));
    }
}
#[inline(always)]
pub fn block_0x0021b0a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2207940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b0c4));
    } else {
        emu.pc = 2207908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b0a4));
    }
}
#[inline(always)]
pub fn block_0x0021b0a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 1u32, 2207912u32);
    emu.ani_no_count(12usize, 12usize, 30u32, 2207916u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2207920u32);
    emu.lbu_no_count(11usize, 11usize, 1u32, 2207924u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2207928u32);
    emu.adi_no_count(12usize, 2usize, 14u32, 2207932u32);
    emu.adr_no_count(12usize, 12usize, 14usize, 2207936u32);
    emu.sb_no_count(11usize, 12usize, 0u32, 2207940u32);
    emu.add_memory_rw_events(8usize);
    emu.pc = 2207940u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b0c4));
}
#[inline]
pub fn block_0x0021b0c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 10u32, 2207944u32);
    emu.adi_no_count(11usize, 2usize, 14u32, 2207948u32);
    emu.sbr_no_count(15usize, 15usize, 14usize, 2207952u32);
    emu.adr_no_count(14usize, 11usize, 14usize, 2207956u32);
    emu.xri_no_count(11usize, 10usize, 4294967295u32, 2207960u32);
    emu.sri_no_count(11usize, 11usize, 31u32, 2207964u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2207968u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2207972u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2207976u32);
    emu.apc_no_count(1usize, 2207976u32, 8192u32, 2207980u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2207984u32;
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
#[inline(always)]
pub fn block_0x0021b0f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2207988u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2207992u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2207996u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2208000u32;
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
pub fn block_0x0021b100(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 10u32, 2208004u32);
    emu.adi_no_count(15usize, 0usize, 9u32, 2208008u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2207816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b048));
    } else {
        emu.pc = 2208012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b10c));
    }
}
#[inline(always)]
pub fn block_0x0021b10c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2208016u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2207900u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b09c));
}
#[inline(always)]
pub fn block_0x0021b110(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2208020u32);
    emu.sw_no_count(8usize, 2usize, 12u32, 2208024u32)?;
    emu.sw_no_count(9usize, 2usize, 8u32, 2208028u32)?;
    emu.adi_no_count(13usize, 0usize, 1000u32, 2208032u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2208036u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1330u32, 2208040u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2208368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b270));
    } else {
        emu.pc = 2208044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b12c));
    }
}
#[inline]
pub fn block_0x0021b12c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 11usize, 4294967294u32, 2208048u32);
    let a = 0u32.wrapping_add(3518435328u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2208052u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2208056u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2208060u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 0usize, 100u32, 2208064u32);
    let a = 0u32.wrapping_add(9998336u32);
    emu.write_reg_no_count(28usize, a);
    emu.pc = 2208068u32;
    emu.update_insn_clock();
    emu.adi_no_count(5usize, 13usize, 1881u32, 2208072u32);
    emu.adi_no_count(6usize, 15usize, 1808u32, 2208076u32);
    emu.adi_no_count(7usize, 7usize, 1147u32, 2208080u32);
    emu.adi_no_count(28usize, 28usize, 1663u32, 2208084u32);
    emu.adi_no_count(15usize, 10usize, 0u32, 2208088u32);
    emu.add_memory_rw_events(11usize);
    emu.pc = 2208088u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b158));
}
#[inline(never)]
pub fn block_0x0021b158(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 29u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(29usize, 15usize, 0u32, 2208092u32);
    emu.adi_no_count(13usize, 12usize, 4294967292u32, 2208096u32);
    emu.mulhu_no_count(15usize, 15usize, 5usize, 2208100u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2208104u32);
    emu.sri_no_count(15usize, 15usize, 13u32, 2208108u32);
    emu.mul_no_count(30usize, 15usize, 6usize, 2208112u32);
    emu.sbr_no_count(30usize, 29usize, 30usize, 2208116u32);
    emu.sli_no_count(31usize, 30usize, 16u32, 2208120u32);
    emu.sri_no_count(31usize, 31usize, 18u32, 2208124u32);
    emu.mul_no_count(31usize, 31usize, 7usize, 2208128u32);
    emu.sri_no_count(8usize, 31usize, 16u32, 2208132u32);
    emu.sri_no_count(31usize, 31usize, 17u32, 2208136u32);
    emu.mul_no_count(31usize, 31usize, 17usize, 2208140u32);
    emu.ani_no_count(8usize, 8usize, 2046u32, 2208144u32);
    emu.sbr_no_count(30usize, 30usize, 31usize, 2208148u32);
    emu.adr_no_count(8usize, 14usize, 8usize, 2208152u32);
    emu.sli_no_count(30usize, 30usize, 17u32, 2208156u32);
    emu.sri_no_count(30usize, 30usize, 16u32, 2208160u32);
    emu.adr_no_count(30usize, 14usize, 30usize, 2208164u32);
    emu.lbu_no_count(31usize, 8usize, 0u32, 2208168u32);
    emu.lbu_no_count(8usize, 8usize, 1u32, 2208172u32);
    emu.lbu_no_count(9usize, 30usize, 0u32, 2208176u32);
    emu.lbu_no_count(30usize, 30usize, 1u32, 2208180u32);
    emu.sb_no_count(31usize, 12usize, 4294967294u32, 2208184u32);
    emu.sb_no_count(8usize, 12usize, 4294967295u32, 2208188u32);
    emu.sb_no_count(9usize, 12usize, 0u32, 2208192u32);
    emu.sb_no_count(30usize, 12usize, 1u32, 2208196u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2208200u32);
    emu.add_memory_rw_events(28usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a < b {
        emu.pc = 2208088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b158));
    } else {
        emu.pc = 2208204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b1cc));
    }
}
#[inline(always)]
pub fn block_0x0021b1cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 9u32, 2208208u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a >= b {
        emu.pc = 2208292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b224));
    } else {
        emu.pc = 2208212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b1d4));
    }
}
#[inline]
pub fn block_0x0021b1d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 15usize, 16u32, 2208216u32);
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2208220u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 0usize, 100u32, 2208224u32);
    emu.adr_no_count(5usize, 11usize, 13usize, 2208228u32);
    emu.sri_no_count(12usize, 12usize, 18u32, 2208232u32);
    emu.adi_no_count(16usize, 16usize, 1147u32, 2208236u32);
    emu.mul_no_count(12usize, 12usize, 16usize, 2208240u32);
    emu.sri_no_count(12usize, 12usize, 17u32, 2208244u32);
    emu.mul_no_count(16usize, 12usize, 17usize, 2208248u32);
    emu.sbr_no_count(15usize, 15usize, 16usize, 2208252u32);
    emu.sli_no_count(15usize, 15usize, 17u32, 2208256u32);
    emu.sri_no_count(15usize, 15usize, 16u32, 2208260u32);
    emu.adr_no_count(15usize, 14usize, 15usize, 2208264u32);
    emu.lbu_no_count(16usize, 15usize, 0u32, 2208268u32);
    emu.lbu_no_count(15usize, 15usize, 1u32, 2208272u32);
    emu.adi_no_count(13usize, 13usize, 4294967294u32, 2208276u32);
    emu.adr_no_count(17usize, 11usize, 13usize, 2208280u32);
    emu.sb_no_count(16usize, 17usize, 0u32, 2208284u32);
    emu.sb_no_count(15usize, 5usize, 4294967295u32, 2208288u32);
    emu.adi_no_count(15usize, 12usize, 0u32, 2208292u32);
    emu.add_memory_rw_events(20usize);
    emu.pc = 2208292u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b224));
}
#[inline(always)]
pub fn block_0x0021b224(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2208320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b240));
    } else {
        emu.pc = 2208296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b228));
    }
}
#[inline(always)]
pub fn block_0x0021b228(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2208320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b240));
    } else {
        emu.pc = 2208300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b22c));
    }
}
#[inline(always)]
pub fn block_0x0021b22c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 13usize, 0u32, 2208304u32);
    emu.lw_no_count(8usize, 2usize, 12u32, 2208308u32)?;
    emu.lw_no_count(9usize, 2usize, 8u32, 2208312u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2208316u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2208320u32;
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
pub fn block_0x0021b240(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(15usize, 15usize, 1u32, 2208324u32);
    emu.ani_no_count(15usize, 15usize, 30u32, 2208328u32);
    emu.adr_no_count(14usize, 14usize, 15usize, 2208332u32);
    emu.lbu_no_count(10usize, 14usize, 1u32, 2208336u32);
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2208340u32);
    emu.adr_no_count(11usize, 11usize, 13usize, 2208344u32);
    emu.sb_no_count(10usize, 11usize, 0u32, 2208348u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2208352u32);
    emu.lw_no_count(8usize, 2usize, 12u32, 2208356u32)?;
    emu.lw_no_count(9usize, 2usize, 8u32, 2208360u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2208364u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2208368u32;
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
pub fn block_0x0021b270(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 10usize, 0u32, 2208372u32);
    emu.adi_no_count(13usize, 12usize, 0u32, 2208376u32);
    emu.adi_no_count(12usize, 0usize, 9u32, 2208380u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2208212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b1d4));
    } else {
        emu.pc = 2208384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b280));
    }
}
#[inline(always)]
pub fn block_0x0021b280(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2208388u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2208292u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b224));
}
#[inline]
pub fn block_0x0021b284(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2208392u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2208396u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2208400u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2208404u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2208408u32)?;
    emu.lw_no_count(14usize, 10usize, 0u32, 2208412u32)?;
    emu.lw_no_count(15usize, 10usize, 4u32, 2208416u32)?;
    emu.adi_no_count(8usize, 11usize, 0u32, 2208420u32);
    emu.adi_no_count(9usize, 2usize, 12u32, 2208424u32);
    emu.adi_no_count(12usize, 2usize, 12u32, 2208428u32);
    emu.adi_no_count(13usize, 0usize, 20u32, 2208432u32);
    emu.adi_no_count(18usize, 0usize, 20u32, 2208436u32);
    emu.adi_no_count(10usize, 14usize, 0u32, 2208440u32);
    emu.adi_no_count(11usize, 15usize, 0u32, 2208444u32);
    emu.apc_no_count(1usize, 2208444u32, 0u32, 2208448u32);
    emu.add_memory_rw_events(16usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2208452u32;
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
pub fn block_0x0021b2c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(15usize, 18usize, 10usize, 2208456u32);
    emu.adr_no_count(14usize, 9usize, 10usize, 2208460u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2208464u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2208468u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2208472u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2208476u32);
    emu.apc_no_count(1usize, 2208476u32, 8192u32, 2208480u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2208484u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966400u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021b2e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2208488u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2208492u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2208496u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2208500u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2208504u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2208508u32;
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
pub fn block_0x0021b2fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2208512u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2208516u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2208520u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2208524u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2208528u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2208532u32)?;
    emu.sw_no_count(20usize, 2usize, 40u32, 2208536u32)?;
    emu.sw_no_count(21usize, 2usize, 36u32, 2208540u32)?;
    emu.sw_no_count(22usize, 2usize, 32u32, 2208544u32)?;
    emu.sw_no_count(23usize, 2usize, 28u32, 2208548u32)?;
    emu.sw_no_count(24usize, 2usize, 24u32, 2208552u32)?;
    emu.sw_no_count(25usize, 2usize, 20u32, 2208556u32)?;
    emu.sw_no_count(26usize, 2usize, 16u32, 2208560u32)?;
    emu.sw_no_count(27usize, 2usize, 12u32, 2208564u32)?;
    emu.adi_no_count(21usize, 13usize, 0u32, 2208568u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2208572u32);
    emu.adi_no_count(18usize, 10usize, 0u32, 2208576u32);
    emu.sltiu_no_count(10usize, 10usize, 1000u32, 2208580u32);
    emu.sltiu_no_count(11usize, 11usize, 1u32, 2208584u32);
    emu.anr_no_count(10usize, 11usize, 10usize, 2208588u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(24usize, a);
    emu.pc = 2208592u32;
    emu.update_insn_clock();
    emu.adi_no_count(24usize, 24usize, 1330u32, 2208596u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2208600u32)?;
    emu.add_memory_rw_events(23usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2208908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b48c));
    } else {
        emu.pc = 2208604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b35c));
    }
}
#[inline]
pub fn block_0x0021b35c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 12usize, 4294967294u32, 2208608u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2208612u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2208616u32;
    emu.update_insn_clock();
    emu.adi_no_count(26usize, 0usize, 100u32, 2208620u32);
    let a = 0u32.wrapping_add(9998336u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2208624u32;
    emu.update_insn_clock();
    emu.adi_no_count(20usize, 10usize, 1808u32, 2208628u32);
    emu.adi_no_count(27usize, 11usize, 1147u32, 2208632u32);
    emu.adi_no_count(8usize, 12usize, 1663u32, 2208636u32);
    emu.adi_no_count(22usize, 18usize, 0u32, 2208640u32);
    emu.adi_no_count(23usize, 9usize, 0u32, 2208644u32);
    emu.add_memory_rw_events(10usize);
    emu.pc = 2208644u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b384));
}
#[inline(always)]
pub fn block_0x0021b384(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 21usize, 4294967292u32, 2208648u32);
    emu.adi_no_count(10usize, 22usize, 0u32, 2208652u32);
    emu.adi_no_count(11usize, 23usize, 0u32, 2208656u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2208660u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2208664u32);
    emu.apc_no_count(1usize, 2208664u32, 12288u32, 2208668u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2208672u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1212u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0021b3a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 30u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.mul_no_count(12usize, 10usize, 20usize, 2208676u32);
    emu.adr_no_count(21usize, 25usize, 21usize, 2208680u32);
    emu.sltru_no_count(13usize, 8usize, 22usize, 2208684u32);
    emu.sltru_no_count(14usize, 0usize, 23usize, 2208688u32);
    emu.sbr_no_count(12usize, 22usize, 12usize, 2208692u32);
    emu.orr_no_count(13usize, 13usize, 14usize, 2208696u32);
    emu.sli_no_count(14usize, 12usize, 16u32, 2208700u32);
    emu.sri_no_count(14usize, 14usize, 18u32, 2208704u32);
    emu.mul_no_count(14usize, 14usize, 27usize, 2208708u32);
    emu.sri_no_count(15usize, 14usize, 16u32, 2208712u32);
    emu.sri_no_count(14usize, 14usize, 17u32, 2208716u32);
    emu.mul_no_count(14usize, 14usize, 26usize, 2208720u32);
    emu.ani_no_count(15usize, 15usize, 2046u32, 2208724u32);
    emu.sbr_no_count(12usize, 12usize, 14usize, 2208728u32);
    emu.adr_no_count(15usize, 24usize, 15usize, 2208732u32);
    emu.sli_no_count(12usize, 12usize, 17u32, 2208736u32);
    emu.sri_no_count(12usize, 12usize, 16u32, 2208740u32);
    emu.adr_no_count(12usize, 24usize, 12usize, 2208744u32);
    emu.lbu_no_count(14usize, 15usize, 0u32, 2208748u32);
    emu.lbu_no_count(15usize, 15usize, 1u32, 2208752u32);
    emu.lbu_no_count(16usize, 12usize, 0u32, 2208756u32);
    emu.lbu_no_count(12usize, 12usize, 1u32, 2208760u32);
    emu.sb_no_count(14usize, 21usize, 4294967294u32, 2208764u32);
    emu.sb_no_count(15usize, 21usize, 4294967295u32, 2208768u32);
    emu.sb_no_count(16usize, 21usize, 0u32, 2208772u32);
    emu.sb_no_count(12usize, 21usize, 1u32, 2208776u32);
    emu.adi_no_count(21usize, 19usize, 0u32, 2208780u32);
    emu.adi_no_count(22usize, 10usize, 0u32, 2208784u32);
    emu.adi_no_count(23usize, 11usize, 0u32, 2208788u32);
    emu.add_memory_rw_events(29usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2208644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b384));
    } else {
        emu.pc = 2208792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b418));
    }
}
#[inline(always)]
pub fn block_0x0021b418(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(12usize, 10usize, 10u32, 2208796u32);
    emu.sltiu_no_count(13usize, 11usize, 1u32, 2208800u32);
    emu.anr_no_count(12usize, 13usize, 12usize, 2208804u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2208936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b4a8));
    } else {
        emu.pc = 2208808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b428));
    }
}
#[inline]
pub fn block_0x0021b428(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2208812u32);
    emu.sli_no_count(12usize, 10usize, 16u32, 2208816u32);
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2208820u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 0usize, 100u32, 2208824u32);
    emu.lw_no_count(16usize, 2usize, 8u32, 2208828u32)?;
    emu.adr_no_count(15usize, 16usize, 19usize, 2208832u32);
    emu.sri_no_count(12usize, 12usize, 18u32, 2208836u32);
    emu.adi_no_count(13usize, 13usize, 1147u32, 2208840u32);
    emu.mul_no_count(12usize, 12usize, 13usize, 2208844u32);
    emu.sri_no_count(12usize, 12usize, 17u32, 2208848u32);
    emu.mul_no_count(13usize, 12usize, 14usize, 2208852u32);
    emu.sbr_no_count(10usize, 10usize, 13usize, 2208856u32);
    emu.sli_no_count(10usize, 10usize, 17u32, 2208860u32);
    emu.sri_no_count(10usize, 10usize, 16u32, 2208864u32);
    emu.adr_no_count(10usize, 24usize, 10usize, 2208868u32);
    emu.lbu_no_count(13usize, 10usize, 0u32, 2208872u32);
    emu.lbu_no_count(10usize, 10usize, 1u32, 2208876u32);
    emu.adi_no_count(19usize, 19usize, 4294967294u32, 2208880u32);
    emu.adr_no_count(14usize, 16usize, 19usize, 2208884u32);
    emu.sb_no_count(13usize, 14usize, 0u32, 2208888u32);
    emu.sb_no_count(10usize, 15usize, 4294967295u32, 2208892u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2208896u32);
    emu.orr_no_count(12usize, 18usize, 9usize, 2208900u32);
    emu.add_memory_rw_events(23usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2208948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b4b4));
    } else {
        emu.pc = 2208904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b488));
    }
}
#[inline(always)]
pub fn block_0x0021b488(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2208908u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2208956u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b4bc));
}
#[inline(always)]
pub fn block_0x0021b48c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2208912u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2208916u32);
    emu.adi_no_count(19usize, 21usize, 0u32, 2208920u32);
    emu.sltiu_no_count(12usize, 18usize, 10u32, 2208924u32);
    emu.sltiu_no_count(13usize, 9usize, 1u32, 2208928u32);
    emu.anr_no_count(12usize, 13usize, 12usize, 2208932u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2208808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b428));
    } else {
        emu.pc = 2208936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b4a8));
    }
}
#[inline(always)]
pub fn block_0x0021b4a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 2usize, 8u32, 2208940u32)?;
    emu.orr_no_count(12usize, 18usize, 9usize, 2208944u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2208956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b4bc));
    } else {
        emu.pc = 2208948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b4b4));
    }
}
#[inline(always)]
pub fn block_0x0021b4b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.orr_no_count(11usize, 10usize, 11usize, 2208952u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2208984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b4d8));
    } else {
        emu.pc = 2208956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b4bc));
    }
}
#[inline(always)]
pub fn block_0x0021b4bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 10usize, 1u32, 2208960u32);
    emu.ani_no_count(10usize, 10usize, 30u32, 2208964u32);
    emu.adr_no_count(10usize, 24usize, 10usize, 2208968u32);
    emu.lbu_no_count(10usize, 10usize, 1u32, 2208972u32);
    emu.adi_no_count(19usize, 19usize, 4294967295u32, 2208976u32);
    emu.adr_no_count(11usize, 16usize, 19usize, 2208980u32);
    emu.sb_no_count(10usize, 11usize, 0u32, 2208984u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2208984u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b4d8));
}
#[inline]
pub fn block_0x0021b4d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 19usize, 0u32, 2208988u32);
    emu.lw_no_count(1usize, 2usize, 60u32, 2208992u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2208996u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2209000u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2209004u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2209008u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2209012u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2209016u32)?;
    emu.lw_no_count(22usize, 2usize, 32u32, 2209020u32)?;
    emu.lw_no_count(23usize, 2usize, 28u32, 2209024u32)?;
    emu.lw_no_count(24usize, 2usize, 24u32, 2209028u32)?;
    emu.lw_no_count(25usize, 2usize, 20u32, 2209032u32)?;
    emu.lw_no_count(26usize, 2usize, 16u32, 2209036u32)?;
    emu.lw_no_count(27usize, 2usize, 12u32, 2209040u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2209044u32);
    emu.add_memory_rw_events(16usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2209048u32;
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
pub fn block_0x0021b518(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2209052u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2209056u32);
    emu.sw_no_count(10usize, 2usize, 4u32, 2209060u32)?;
    emu.sw_no_count(11usize, 2usize, 8u32, 2209064u32)?;
    emu.sh_no_count(12usize, 2usize, 12u32, 2209068u32)?;
    emu.adi_no_count(10usize, 2usize, 4u32, 2209072u32);
    emu.apc_no_count(1usize, 2209072u32, 4294959104u32, 2209076u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2209080u32;
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
pub fn block_0x0021b538(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2209084u32);
    emu.sw_no_count(10usize, 2usize, 24u32, 2209088u32)?;
    emu.sw_no_count(11usize, 2usize, 28u32, 2209092u32)?;
    emu.adi_no_count(10usize, 2usize, 24u32, 2209096u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2209100u32);
    emu.sw_no_count(0usize, 2usize, 16u32, 2209104u32)?;
    emu.adi_no_count(13usize, 0usize, 4u32, 2209108u32);
    emu.sw_no_count(10usize, 2usize, 0u32, 2209112u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2209116u32)?;
    emu.sw_no_count(13usize, 2usize, 8u32, 2209120u32)?;
    emu.sw_no_count(0usize, 2usize, 12u32, 2209124u32)?;
    emu.adi_no_count(10usize, 2usize, 0u32, 2209128u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2209132u32);
    emu.apc_no_count(1usize, 2209132u32, 0u32, 2209136u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2209140u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967212u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021b574(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2209144u32);
    emu.sw_no_count(10usize, 2usize, 0u32, 2209148u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2209152u32)?;
    emu.adi_no_count(10usize, 2usize, 4u32, 2209156u32);
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2209160u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966756u32, 2209164u32);
    emu.adi_no_count(13usize, 2usize, 0u32, 2209168u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2209172u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1548u32, 2209176u32);
    emu.adi_no_count(15usize, 0usize, 2u32, 2209180u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2209184u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2209188u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2209192u32)?;
    emu.sw_no_count(13usize, 2usize, 40u32, 2209196u32)?;
    emu.sw_no_count(11usize, 2usize, 44u32, 2209200u32)?;
    emu.adi_no_count(10usize, 2usize, 32u32, 2209204u32);
    emu.sw_no_count(14usize, 2usize, 8u32, 2209208u32)?;
    emu.sw_no_count(15usize, 2usize, 12u32, 2209212u32)?;
    emu.sw_no_count(10usize, 2usize, 16u32, 2209216u32)?;
    emu.sw_no_count(15usize, 2usize, 20u32, 2209220u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2209224u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2209228u32);
    emu.apc_no_count(1usize, 2209228u32, 0u32, 2209232u32);
    emu.add_memory_rw_events(24usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2209236u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967116u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021b5d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967168u32, 2209240u32);
    emu.sw_no_count(1usize, 2usize, 124u32, 2209244u32)?;
    emu.sw_no_count(8usize, 2usize, 120u32, 2209248u32)?;
    emu.sw_no_count(9usize, 2usize, 116u32, 2209252u32)?;
    emu.adi_no_count(8usize, 16usize, 0u32, 2209256u32);
    emu.sw_no_count(11usize, 2usize, 12u32, 2209260u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2209264u32)?;
    emu.sli_no_count(10usize, 10usize, 2u32, 2209268u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2209272u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1872u32, 2209276u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2209280u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1884u32, 2209284u32);
    emu.adr_no_count(11usize, 11usize, 10usize, 2209288u32);
    emu.adr_no_count(10usize, 12usize, 10usize, 2209292u32);
    emu.lw_no_count(12usize, 15usize, 0u32, 2209296u32)?;
    emu.lw_no_count(11usize, 11usize, 0u32, 2209300u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2209304u32)?;
    emu.sw_no_count(13usize, 2usize, 20u32, 2209308u32)?;
    emu.sw_no_count(14usize, 2usize, 24u32, 2209312u32)?;
    emu.sw_no_count(11usize, 2usize, 28u32, 2209316u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2209320u32)?;
    emu.add_memory_rw_events(21usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2209416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b688));
    } else {
        emu.pc = 2209324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b62c));
    }
}
#[inline]
pub fn block_0x0021b62c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 23u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 28u32, 2209328u32);
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2209332u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967232u32, 2209336u32);
    emu.adi_no_count(12usize, 2usize, 12u32, 2209340u32);
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2209344u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294967216u32, 2209348u32);
    emu.adi_no_count(14usize, 2usize, 20u32, 2209352u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2209356u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 1608u32, 2209360u32);
    emu.adi_no_count(16usize, 0usize, 3u32, 2209364u32);
    emu.sw_no_count(0usize, 2usize, 108u32, 2209368u32)?;
    emu.sw_no_count(10usize, 2usize, 60u32, 2209372u32)?;
    emu.sw_no_count(11usize, 2usize, 64u32, 2209376u32)?;
    emu.sw_no_count(12usize, 2usize, 68u32, 2209380u32)?;
    emu.sw_no_count(13usize, 2usize, 72u32, 2209384u32)?;
    emu.sw_no_count(14usize, 2usize, 76u32, 2209388u32)?;
    emu.sw_no_count(13usize, 2usize, 80u32, 2209392u32)?;
    emu.adi_no_count(10usize, 2usize, 60u32, 2209396u32);
    emu.sw_no_count(15usize, 2usize, 92u32, 2209400u32)?;
    emu.sw_no_count(16usize, 2usize, 96u32, 2209404u32)?;
    emu.sw_no_count(10usize, 2usize, 100u32, 2209408u32)?;
    emu.sw_no_count(16usize, 2usize, 104u32, 2209412u32)?;
    emu.add_memory_rw_events(23usize);
    let return_addr = 2209416u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2209544u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b708));
}
#[inline(always)]
pub fn block_0x0021b688(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 36u32, 2209420u32);
    emu.adi_no_count(12usize, 0usize, 24u32, 2209424u32);
    emu.adi_no_count(9usize, 2usize, 36u32, 2209428u32);
    emu.adi_no_count(11usize, 15usize, 0u32, 2209432u32);
    emu.apc_no_count(1usize, 2209432u32, 4294889472u32, 2209436u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2209440u32;
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
#[inline(never)]
pub fn block_0x0021b6a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 26u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 28u32, 2209444u32);
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2209448u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967232u32, 2209452u32);
    let a = 0u32.wrapping_add(2215936u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2209456u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966460u32, 2209460u32);
    emu.adi_no_count(13usize, 2usize, 12u32, 2209464u32);
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2209468u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294967216u32, 2209472u32);
    emu.adi_no_count(15usize, 2usize, 20u32, 2209476u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2209480u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 1644u32, 2209484u32);
    emu.sw_no_count(10usize, 2usize, 60u32, 2209488u32)?;
    emu.sw_no_count(11usize, 2usize, 64u32, 2209492u32)?;
    emu.sw_no_count(9usize, 2usize, 68u32, 2209496u32)?;
    emu.sw_no_count(12usize, 2usize, 72u32, 2209500u32)?;
    emu.adi_no_count(10usize, 0usize, 4u32, 2209504u32);
    emu.sw_no_count(0usize, 2usize, 108u32, 2209508u32)?;
    emu.sw_no_count(13usize, 2usize, 76u32, 2209512u32)?;
    emu.sw_no_count(14usize, 2usize, 80u32, 2209516u32)?;
    emu.sw_no_count(15usize, 2usize, 84u32, 2209520u32)?;
    emu.sw_no_count(14usize, 2usize, 88u32, 2209524u32)?;
    emu.adi_no_count(11usize, 2usize, 60u32, 2209528u32);
    emu.sw_no_count(16usize, 2usize, 92u32, 2209532u32)?;
    emu.sw_no_count(10usize, 2usize, 96u32, 2209536u32)?;
    emu.sw_no_count(11usize, 2usize, 100u32, 2209540u32)?;
    emu.sw_no_count(10usize, 2usize, 104u32, 2209544u32)?;
    emu.add_memory_rw_events(26usize);
    emu.pc = 2209544u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b708));
}
#[inline(always)]
pub fn block_0x0021b708(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 92u32, 2209548u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2209552u32);
    emu.apc_no_count(1usize, 2209552u32, 0u32, 2209556u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2209560u32;
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
