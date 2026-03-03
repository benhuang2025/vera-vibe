pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2143136u32;
pub const PC_MAX: u32 = 2145360u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 109usize] = [
        block_0x0020b3a0,
        block_0x0020b3a4,
        block_0x0020b3b0,
        block_0x0020b3b8,
        block_0x0020b3cc,
        block_0x0020b3e0,
        block_0x0020b3e8,
        block_0x0020b3f8,
        block_0x0020b3fc,
        block_0x0020b42c,
        block_0x0020b440,
        block_0x0020b468,
        block_0x0020b480,
        block_0x0020b48c,
        block_0x0020b4b8,
        block_0x0020b4c0,
        block_0x0020b4c4,
        block_0x0020b4cc,
        block_0x0020b4d8,
        block_0x0020b4e8,
        block_0x0020b4f8,
        block_0x0020b500,
        block_0x0020b518,
        block_0x0020b534,
        block_0x0020b538,
        block_0x0020b554,
        block_0x0020b55c,
        block_0x0020b588,
        block_0x0020b5c0,
        block_0x0020b5ec,
        block_0x0020b5f4,
        block_0x0020b604,
        block_0x0020b608,
        block_0x0020b624,
        block_0x0020b64c,
        block_0x0020b658,
        block_0x0020b66c,
        block_0x0020b674,
        block_0x0020b67c,
        block_0x0020b684,
        block_0x0020b688,
        block_0x0020b690,
        block_0x0020b694,
        block_0x0020b6a4,
        block_0x0020b6a8,
        block_0x0020b6ac,
        block_0x0020b6c4,
        block_0x0020b6c8,
        block_0x0020b6d0,
        block_0x0020b6d4,
        block_0x0020b6dc,
        block_0x0020b6f0,
        block_0x0020b6f4,
        block_0x0020b718,
        block_0x0020b71c,
        block_0x0020b750,
        block_0x0020b778,
        block_0x0020b7a8,
        block_0x0020b7bc,
        block_0x0020b7e4,
        block_0x0020b7fc,
        block_0x0020b808,
        block_0x0020b834,
        block_0x0020b83c,
        block_0x0020b840,
        block_0x0020b848,
        block_0x0020b854,
        block_0x0020b864,
        block_0x0020b874,
        block_0x0020b87c,
        block_0x0020b894,
        block_0x0020b8b0,
        block_0x0020b8b4,
        block_0x0020b8d0,
        block_0x0020b8d8,
        block_0x0020b904,
        block_0x0020b93c,
        block_0x0020b968,
        block_0x0020b988,
        block_0x0020b9a0,
        block_0x0020b9a8,
        block_0x0020b9c0,
        block_0x0020b9e8,
        block_0x0020b9f4,
        block_0x0020ba14,
        block_0x0020ba2c,
        block_0x0020ba34,
        block_0x0020ba4c,
        block_0x0020ba74,
        block_0x0020ba80,
        block_0x0020baa0,
        block_0x0020bab8,
        block_0x0020bac0,
        block_0x0020bad8,
        block_0x0020bb00,
        block_0x0020bb0c,
        block_0x0020bb2c,
        block_0x0020bb44,
        block_0x0020bb4c,
        block_0x0020bb64,
        block_0x0020bb8c,
        block_0x0020bb98,
        block_0x0020bbbc,
        block_0x0020bbdc,
        block_0x0020bc18,
        block_0x0020bc1c,
        block_0x0020bc28,
        block_0x0020bc2c,
        block_0x0020bc50,
    ];
    const IDX: [u16; 557usize] = [
        1u16, 2u16, 0u16, 0u16, 3u16, 0u16, 4u16, 0u16, 0u16, 0u16, 0u16, 5u16, 0u16,
        0u16, 0u16, 0u16, 6u16, 0u16, 7u16, 0u16, 0u16, 0u16, 8u16, 9u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 10u16, 0u16, 0u16, 0u16,
        0u16, 11u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 12u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 13u16, 0u16, 0u16, 14u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 15u16, 0u16, 16u16, 17u16, 0u16, 18u16, 0u16, 0u16,
        19u16, 0u16, 0u16, 0u16, 20u16, 0u16, 0u16, 0u16, 21u16, 0u16, 22u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 23u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 24u16, 25u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 26u16, 0u16, 27u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 28u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 29u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 30u16, 0u16, 31u16, 0u16, 0u16, 0u16, 32u16, 33u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 34u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 35u16, 0u16, 0u16, 36u16, 0u16, 0u16, 0u16, 0u16, 37u16, 0u16, 38u16,
        0u16, 39u16, 0u16, 40u16, 41u16, 0u16, 42u16, 43u16, 0u16, 0u16, 0u16, 44u16,
        45u16, 46u16, 0u16, 0u16, 0u16, 0u16, 0u16, 47u16, 48u16, 0u16, 49u16, 50u16,
        0u16, 51u16, 0u16, 0u16, 0u16, 0u16, 52u16, 53u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 54u16, 55u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 56u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 57u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        58u16, 0u16, 0u16, 0u16, 0u16, 59u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 60u16, 0u16, 0u16, 0u16, 0u16, 0u16, 61u16, 0u16, 0u16, 62u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 63u16, 0u16, 64u16, 65u16,
        0u16, 66u16, 0u16, 0u16, 67u16, 0u16, 0u16, 0u16, 68u16, 0u16, 0u16, 0u16, 69u16,
        0u16, 70u16, 0u16, 0u16, 0u16, 0u16, 0u16, 71u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 72u16, 73u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 74u16, 0u16, 75u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 76u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 77u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 78u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 79u16, 0u16, 0u16, 0u16, 0u16, 0u16, 80u16, 0u16, 81u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 82u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 83u16, 0u16, 0u16, 84u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 85u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 86u16, 0u16, 87u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        88u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 89u16, 0u16, 0u16,
        90u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 91u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 92u16, 0u16, 93u16, 0u16, 0u16, 0u16, 0u16, 0u16, 94u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 95u16, 0u16, 0u16, 96u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 97u16, 0u16, 0u16, 0u16, 0u16, 0u16, 98u16, 0u16, 99u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 100u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 101u16, 0u16, 0u16, 102u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 103u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 104u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 105u16, 106u16,
        0u16, 0u16, 107u16, 108u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        109u16,
    ];
    if pc < 2143136u32 || pc > 2145360u32 {
        return None;
    }
    let word_offset = ((pc - 2143136u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x0020b3a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2143152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b3b0));
    } else {
        emu.pc = 2143140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b3a4));
    }
}
#[inline(always)]
pub fn block_0x0020b3a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 12usize, 0u32, 2143144u32);
    emu.apc_no_count(1usize, 2143144u32, 0u32, 2143148u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2143152u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966664u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020b3b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2143152u32, 0u32, 2143156u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2143160u32;
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
#[inline(always)]
pub fn block_0x0020b3b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2143164u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2143168u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2143172u32);
    emu.apc_no_count(1usize, 2143172u32, 4294930432u32, 2143176u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2143180u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(816u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020b3cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 0u32, 2143184u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2143188u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 144u32, 2143192u32);
    emu.apc_no_count(6usize, 2143192u32, 20480u32, 2143196u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2143200u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966256u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020b3e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2143204u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2143224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b3f8));
    } else {
        emu.pc = 2143208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b3e8));
    }
}
#[inline(always)]
pub fn block_0x0020b3e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 4u32, 2143212u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2143216u32);
    emu.apc_no_count(6usize, 2143216u32, 4294926336u32, 2143220u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2143224u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966600u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020b3f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2143228u32;
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
pub fn block_0x0020b3fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2143232u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2143236u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2143240u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2143244u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2143248u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2143252u32)?;
    emu.adi_no_count(8usize, 12usize, 0u32, 2143256u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2143260u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2143264u32)?;
    emu.lw_no_count(18usize, 9usize, 8u32, 2143268u32)?;
    emu.sbr_no_count(10usize, 10usize, 18usize, 2143272u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2143336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b468));
    } else {
        emu.pc = 2143276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b42c));
    }
}
#[inline(always)]
pub fn block_0x0020b42c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 4u32, 2143280u32)?;
    emu.adr_no_count(10usize, 10usize, 18usize, 2143284u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2143288u32);
    emu.apc_no_count(1usize, 2143288u32, 4294934528u32, 2143292u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2143296u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966332u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020b440(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 18usize, 8usize, 2143300u32);
    emu.sw_no_count(8usize, 9usize, 8u32, 2143304u32)?;
    emu.adi_no_count(10usize, 0usize, 0u32, 2143308u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2143312u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2143316u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2143320u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2143324u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2143328u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2143332u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2143336u32;
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
pub fn block_0x0020b468(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2143340u32);
    emu.adi_no_count(19usize, 11usize, 0u32, 2143344u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2143348u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2143352u32);
    emu.apc_no_count(1usize, 2143352u32, 0u32, 2143356u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2143360u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966692u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020b480(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 19usize, 0u32, 2143364u32);
    emu.lw_no_count(18usize, 9usize, 8u32, 2143368u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2143372u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2143276u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b42c));
}
#[inline]
pub fn block_0x0020b48c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2143376u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2143380u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2143384u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2143388u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2143392u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2143396u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2143400u32)?;
    emu.lw_no_count(8usize, 10usize, 8u32, 2143404u32)?;
    emu.adi_no_count(12usize, 0usize, 128u32, 2143408u32);
    emu.sri_no_count(19usize, 11usize, 11u32, 2143412u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2143424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b4c0));
    } else {
        emu.pc = 2143416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b4b8));
    }
}
#[inline(always)]
pub fn block_0x0020b4b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 1u32, 2143420u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2143424u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2143448u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b4d8));
}
#[inline(always)]
pub fn block_0x0020b4c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2143436u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b4cc));
    } else {
        emu.pc = 2143428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b4c4));
    }
}
#[inline(always)]
pub fn block_0x0020b4c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 2u32, 2143432u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2143436u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2143448u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b4d8));
}
#[inline(always)]
pub fn block_0x0020b4cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 11usize, 16u32, 2143440u32);
    emu.sltru_no_count(9usize, 0usize, 12usize, 2143444u32);
    emu.adi_no_count(9usize, 9usize, 3u32, 2143448u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2143448u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b4d8));
}
#[inline(always)]
pub fn block_0x0020b4d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2143452u32)?;
    emu.sbr_no_count(13usize, 12usize, 8usize, 2143456u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2143460u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2143488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b500));
    } else {
        emu.pc = 2143464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b4e8));
    }
}
#[inline(always)]
pub fn block_0x0020b4e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 4u32, 2143468u32)?;
    emu.adi_no_count(14usize, 0usize, 128u32, 2143472u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2143476u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2143540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b534));
    } else {
        emu.pc = 2143480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b4f8));
    }
}
#[inline(always)]
pub fn block_0x0020b4f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 12usize, 0u32, 2143484u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2143488u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2143680u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b5c0));
}
#[inline(always)]
pub fn block_0x0020b500(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2143492u32);
    emu.adi_no_count(20usize, 11usize, 0u32, 2143496u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2143500u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2143504u32);
    emu.apc_no_count(1usize, 2143504u32, 0u32, 2143508u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2143512u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966540u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020b518(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 20usize, 0u32, 2143516u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2143520u32);
    emu.lw_no_count(12usize, 18usize, 8u32, 2143524u32)?;
    emu.lw_no_count(13usize, 18usize, 4u32, 2143528u32)?;
    emu.adi_no_count(14usize, 0usize, 128u32, 2143532u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2143536u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a < b {
        emu.pc = 2143480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b4f8));
    } else {
        emu.pc = 2143540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b534));
    }
}
#[inline(always)]
pub fn block_0x0020b534(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2143572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b554));
    } else {
        emu.pc = 2143544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b538));
    }
}
#[inline(always)]
pub fn block_0x0020b538(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 6u32, 2143548u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2143552u32);
    emu.ori_no_count(13usize, 13usize, 192u32, 2143556u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2143560u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2143564u32);
    emu.sb_no_count(11usize, 12usize, 1u32, 2143568u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2143572u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2143680u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b5c0));
}
#[inline(always)]
pub fn block_0x0020b554(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 16u32, 2143576u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2143624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b588));
    } else {
        emu.pc = 2143580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b55c));
    }
}
#[inline]
pub fn block_0x0020b55c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 12u32, 2143584u32);
    emu.sli_no_count(14usize, 11usize, 20u32, 2143588u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2143592u32);
    emu.ori_no_count(13usize, 13usize, 224u32, 2143596u32);
    emu.sri_no_count(14usize, 14usize, 26u32, 2143600u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2143604u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2143608u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2143612u32);
    emu.sb_no_count(14usize, 12usize, 1u32, 2143616u32);
    emu.sb_no_count(11usize, 12usize, 2u32, 2143620u32);
    emu.add_memory_rw_events(11usize);
    let return_addr = 2143624u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2143680u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b5c0));
}
#[inline]
pub fn block_0x0020b588(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 18u32, 2143628u32);
    emu.sli_no_count(14usize, 11usize, 14u32, 2143632u32);
    emu.sli_no_count(15usize, 11usize, 20u32, 2143636u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2143640u32);
    emu.ori_no_count(13usize, 13usize, 240u32, 2143644u32);
    emu.sri_no_count(14usize, 14usize, 26u32, 2143648u32);
    emu.sri_no_count(15usize, 15usize, 26u32, 2143652u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2143656u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2143660u32);
    emu.adi_no_count(15usize, 15usize, 128u32, 2143664u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2143668u32);
    emu.sb_no_count(14usize, 12usize, 1u32, 2143672u32);
    emu.sb_no_count(15usize, 12usize, 2u32, 2143676u32);
    emu.sb_no_count(11usize, 12usize, 3u32, 2143680u32);
    emu.add_memory_rw_events(14usize);
    emu.pc = 2143680u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b5c0));
}
#[inline]
pub fn block_0x0020b5c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 9usize, 8usize, 2143684u32);
    emu.sw_no_count(8usize, 10usize, 8u32, 2143688u32)?;
    emu.adi_no_count(10usize, 0usize, 0u32, 2143692u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2143696u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2143700u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2143704u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2143708u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2143712u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2143716u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2143720u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2143724u32;
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
pub fn block_0x0020b5ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2143728u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2143748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b604));
    } else {
        emu.pc = 2143732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b5f4));
    }
}
#[inline(always)]
pub fn block_0x0020b5f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 4u32, 2143736u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2143740u32);
    emu.apc_no_count(6usize, 2143740u32, 4294926336u32, 2143744u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2143748u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x0020b604(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2143752u32;
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
pub fn block_0x0020b608(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2143756u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 168u32, 2143760u32);
    emu.adi_no_count(12usize, 0usize, 5u32, 2143764u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2143768u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2143772u32);
    emu.apc_no_count(6usize, 2143772u32, 24576u32, 2143776u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2143780u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965332u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020b624(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2143784u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2143788u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2143792u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2143796u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2143800u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2143804u32)?;
    emu.adi_no_count(9usize, 11usize, 0u32, 2143808u32);
    emu.lw_no_count(11usize, 11usize, 4u32, 2143812u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2143816u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2143880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b688));
    } else {
        emu.pc = 2143820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b64c));
    }
}
#[inline(always)]
pub fn block_0x0020b64c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 0u32, 2143824u32);
    emu.lw_no_count(10usize, 9usize, 0u32, 2143828u32)?;
    emu.adi_no_count(12usize, 10usize, 4u32, 2143832u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2143832u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b658));
}
#[inline(always)]
pub fn block_0x0020b658(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 12usize, 0u32, 2143836u32)?;
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2143840u32);
    emu.adr_no_count(18usize, 13usize, 18usize, 2143844u32);
    emu.adi_no_count(12usize, 12usize, 8u32, 2143848u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2143832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b658));
    } else {
        emu.pc = 2143852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b66c));
    }
}
#[inline(always)]
pub fn block_0x0020b66c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 9usize, 12u32, 2143856u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2143908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b6a4));
    } else {
        emu.pc = 2143860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b674));
    }
}
#[inline(always)]
pub fn block_0x0020b674(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 15u32, 2143864u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2143892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b694));
    } else {
        emu.pc = 2143868u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b67c));
    }
}
#[inline(always)]
pub fn block_0x0020b67c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 4u32, 2143872u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2143940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b6c4));
    } else {
        emu.pc = 2143876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b684));
    }
}
#[inline(always)]
pub fn block_0x0020b684(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2143880u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2143892u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b694));
}
#[inline(always)]
pub fn block_0x0020b688(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 12u32, 2143884u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2143940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b6c4));
    } else {
        emu.pc = 2143888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b690));
    }
}
#[inline(always)]
pub fn block_0x0020b690(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 0u32, 2143892u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2143892u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b694));
}
#[inline(always)]
pub fn block_0x0020b694(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltr_no_count(10usize, 0usize, 18usize, 2143896u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2143900u32);
    emu.anr_no_count(18usize, 10usize, 18usize, 2143904u32);
    emu.sli_no_count(18usize, 18usize, 1u32, 2143908u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2143908u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b6a4));
}
#[inline(always)]
pub fn block_0x0020b6a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2143952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b6d0));
    } else {
        emu.pc = 2143912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b6a8));
    }
}
#[inline(always)]
pub fn block_0x0020b6a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 0u32, 2143916u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2143916u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b6ac));
}
#[inline(always)]
pub fn block_0x0020b6ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2143920u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 200u32, 2143924u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2143928u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2143932u32);
    emu.apc_no_count(1usize, 2143932u32, 0u32, 2143936u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2143940u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966500u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020b6c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 0u32, 2143944u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2143944u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b6c8));
}
#[inline(always)]
pub fn block_0x0020b6c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2143948u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2143952u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2143988u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b6f4));
}
#[inline(always)]
pub fn block_0x0020b6d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2143944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b6c8));
    } else {
        emu.pc = 2143956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b6d4));
    }
}
#[inline(always)]
pub fn block_0x0020b6d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2143956u32, 4294930432u32, 2143960u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2143964u32;
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
#[inline(always)]
pub fn block_0x0020b6dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2143968u32);
    emu.adi_no_count(19usize, 0usize, 1u32, 2143972u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2143976u32);
    emu.apc_no_count(1usize, 2143976u32, 4294926336u32, 2143980u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2143984u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965812u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020b6f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2143916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b6ac));
    } else {
        emu.pc = 2143988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b6f4));
    }
}
#[inline]
pub fn block_0x0020b6f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 2usize, 12u32, 2143992u32)?;
    emu.sw_no_count(10usize, 2usize, 16u32, 2143996u32)?;
    emu.sw_no_count(0usize, 2usize, 20u32, 2144000u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2144004u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 176u32, 2144008u32);
    emu.adi_no_count(10usize, 2usize, 12u32, 2144012u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2144016u32);
    emu.apc_no_count(1usize, 2144016u32, 20480u32, 2144020u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2144024u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965432u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020b718(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2144080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b750));
    } else {
        emu.pc = 2144028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b71c));
    }
}
#[inline]
pub fn block_0x0020b71c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2144032u32)?;
    emu.lw_no_count(11usize, 2usize, 16u32, 2144036u32)?;
    emu.lw_no_count(12usize, 2usize, 20u32, 2144040u32)?;
    emu.sw_no_count(10usize, 8usize, 0u32, 2144044u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2144048u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2144052u32)?;
    emu.lw_no_count(1usize, 2usize, 44u32, 2144056u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2144060u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2144064u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2144068u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2144072u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2144076u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2144080u32;
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
pub fn block_0x0020b750(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2144084u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 232u32, 2144088u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2144092u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 216u32, 2144096u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2144100u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 320u32, 2144104u32);
    emu.adi_no_count(11usize, 0usize, 86u32, 2144108u32);
    emu.adi_no_count(12usize, 2usize, 27u32, 2144112u32);
    emu.apc_no_count(1usize, 2144112u32, 16384u32, 2144116u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2144120u32;
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
#[inline]
pub fn block_0x0020b778(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2144124u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2144128u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2144132u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2144136u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2144140u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2144144u32)?;
    emu.adi_no_count(8usize, 12usize, 0u32, 2144148u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2144152u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2144156u32)?;
    emu.lw_no_count(18usize, 9usize, 8u32, 2144160u32)?;
    emu.sbr_no_count(10usize, 10usize, 18usize, 2144164u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2144228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b7e4));
    } else {
        emu.pc = 2144168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b7a8));
    }
}
#[inline(always)]
pub fn block_0x0020b7a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 4u32, 2144172u32)?;
    emu.adr_no_count(10usize, 10usize, 18usize, 2144176u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2144180u32);
    emu.apc_no_count(1usize, 2144180u32, 4294934528u32, 2144184u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2144188u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965440u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020b7bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 18usize, 8usize, 2144192u32);
    emu.sw_no_count(8usize, 9usize, 8u32, 2144196u32)?;
    emu.adi_no_count(10usize, 0usize, 0u32, 2144200u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2144204u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2144208u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2144212u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2144216u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2144220u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2144224u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2144228u32;
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
pub fn block_0x0020b7e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2144232u32);
    emu.adi_no_count(19usize, 11usize, 0u32, 2144236u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2144240u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2144244u32);
    emu.apc_no_count(1usize, 2144244u32, 0u32, 2144248u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2144252u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965800u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020b7fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 19usize, 0u32, 2144256u32);
    emu.lw_no_count(18usize, 9usize, 8u32, 2144260u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2144264u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2144168u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b7a8));
}
#[inline]
pub fn block_0x0020b808(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2144268u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2144272u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2144276u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2144280u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2144284u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2144288u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2144292u32)?;
    emu.lw_no_count(8usize, 10usize, 8u32, 2144296u32)?;
    emu.adi_no_count(12usize, 0usize, 128u32, 2144300u32);
    emu.sri_no_count(19usize, 11usize, 11u32, 2144304u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2144316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b83c));
    } else {
        emu.pc = 2144308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b834));
    }
}
#[inline(always)]
pub fn block_0x0020b834(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 1u32, 2144312u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2144316u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2144340u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b854));
}
#[inline(always)]
pub fn block_0x0020b83c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2144328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b848));
    } else {
        emu.pc = 2144320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b840));
    }
}
#[inline(always)]
pub fn block_0x0020b840(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 2u32, 2144324u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2144328u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2144340u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b854));
}
#[inline(always)]
pub fn block_0x0020b848(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 11usize, 16u32, 2144332u32);
    emu.sltru_no_count(9usize, 0usize, 12usize, 2144336u32);
    emu.adi_no_count(9usize, 9usize, 3u32, 2144340u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2144340u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b854));
}
#[inline(always)]
pub fn block_0x0020b854(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2144344u32)?;
    emu.sbr_no_count(13usize, 12usize, 8usize, 2144348u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2144352u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2144380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b87c));
    } else {
        emu.pc = 2144356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b864));
    }
}
#[inline(always)]
pub fn block_0x0020b864(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 4u32, 2144360u32)?;
    emu.adi_no_count(14usize, 0usize, 128u32, 2144364u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2144368u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2144432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b8b0));
    } else {
        emu.pc = 2144372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b874));
    }
}
#[inline(always)]
pub fn block_0x0020b874(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 12usize, 0u32, 2144376u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2144380u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2144572u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b93c));
}
#[inline(always)]
pub fn block_0x0020b87c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2144384u32);
    emu.adi_no_count(20usize, 11usize, 0u32, 2144388u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2144392u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2144396u32);
    emu.apc_no_count(1usize, 2144396u32, 0u32, 2144400u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2144404u32;
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
pub fn block_0x0020b894(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 20usize, 0u32, 2144408u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2144412u32);
    emu.lw_no_count(12usize, 18usize, 8u32, 2144416u32)?;
    emu.lw_no_count(13usize, 18usize, 4u32, 2144420u32)?;
    emu.adi_no_count(14usize, 0usize, 128u32, 2144424u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2144428u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a < b {
        emu.pc = 2144372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b874));
    } else {
        emu.pc = 2144432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b8b0));
    }
}
#[inline(always)]
pub fn block_0x0020b8b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2144464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b8d0));
    } else {
        emu.pc = 2144436u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b8b4));
    }
}
#[inline(always)]
pub fn block_0x0020b8b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 6u32, 2144440u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2144444u32);
    emu.ori_no_count(13usize, 13usize, 192u32, 2144448u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2144452u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2144456u32);
    emu.sb_no_count(11usize, 12usize, 1u32, 2144460u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2144464u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2144572u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b93c));
}
#[inline(always)]
pub fn block_0x0020b8d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 16u32, 2144468u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2144516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b904));
    } else {
        emu.pc = 2144472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b8d8));
    }
}
#[inline]
pub fn block_0x0020b8d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 12u32, 2144476u32);
    emu.sli_no_count(14usize, 11usize, 20u32, 2144480u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2144484u32);
    emu.ori_no_count(13usize, 13usize, 224u32, 2144488u32);
    emu.sri_no_count(14usize, 14usize, 26u32, 2144492u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2144496u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2144500u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2144504u32);
    emu.sb_no_count(14usize, 12usize, 1u32, 2144508u32);
    emu.sb_no_count(11usize, 12usize, 2u32, 2144512u32);
    emu.add_memory_rw_events(11usize);
    let return_addr = 2144516u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2144572u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b93c));
}
#[inline]
pub fn block_0x0020b904(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 18u32, 2144520u32);
    emu.sli_no_count(14usize, 11usize, 14u32, 2144524u32);
    emu.sli_no_count(15usize, 11usize, 20u32, 2144528u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2144532u32);
    emu.ori_no_count(13usize, 13usize, 240u32, 2144536u32);
    emu.sri_no_count(14usize, 14usize, 26u32, 2144540u32);
    emu.sri_no_count(15usize, 15usize, 26u32, 2144544u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2144548u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2144552u32);
    emu.adi_no_count(15usize, 15usize, 128u32, 2144556u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2144560u32);
    emu.sb_no_count(14usize, 12usize, 1u32, 2144564u32);
    emu.sb_no_count(15usize, 12usize, 2u32, 2144568u32);
    emu.sb_no_count(11usize, 12usize, 3u32, 2144572u32);
    emu.add_memory_rw_events(14usize);
    emu.pc = 2144572u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b93c));
}
#[inline]
pub fn block_0x0020b93c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 9usize, 8usize, 2144576u32);
    emu.sw_no_count(8usize, 10usize, 8u32, 2144580u32)?;
    emu.adi_no_count(10usize, 0usize, 0u32, 2144584u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2144588u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2144592u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2144596u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2144600u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2144604u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2144608u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2144612u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2144616u32;
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
pub fn block_0x0020b968(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967152u32, 2144620u32);
    emu.sw_no_count(1usize, 2usize, 140u32, 2144624u32)?;
    emu.adi_no_count(16usize, 11usize, 0u32, 2144628u32);
    emu.adi_no_count(15usize, 0usize, 0u32, 2144632u32);
    emu.lbu_no_count(10usize, 10usize, 0u32, 2144636u32);
    emu.adi_no_count(11usize, 2usize, 139u32, 2144640u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2144644u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2144648u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2144672u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b9a0));
}
#[inline(always)]
pub fn block_0x0020b988(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 87u32, 2144652u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2144656u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2144660u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2144664u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2144668u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2144704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b9c0));
    } else {
        emu.pc = 2144672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b9a0));
    }
}
#[inline(always)]
pub fn block_0x0020b9a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(13usize, 10usize, 15u32, 2144676u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2144648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b988));
    } else {
        emu.pc = 2144680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b9a8));
    }
}
#[inline(always)]
pub fn block_0x0020b9a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 48u32, 2144684u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2144688u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2144692u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2144696u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2144700u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2144672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b9a0));
    } else {
        emu.pc = 2144704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b9c0));
    }
}
#[inline]
pub fn block_0x0020b9c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2144708u32);
    emu.sbr_no_count(10usize, 10usize, 15usize, 2144712u32);
    emu.adi_no_count(14usize, 10usize, 128u32, 2144716u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2144720u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 336u32, 2144724u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2144728u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2144732u32);
    emu.adi_no_count(10usize, 16usize, 0u32, 2144736u32);
    emu.apc_no_count(1usize, 2144736u32, 20480u32, 2144740u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2144744u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965356u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020b9e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 140u32, 2144748u32)?;
    emu.adi_no_count(2usize, 2usize, 144u32, 2144752u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2144756u32;
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
pub fn block_0x0020b9f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967152u32, 2144760u32);
    emu.sw_no_count(1usize, 2usize, 140u32, 2144764u32)?;
    emu.adi_no_count(16usize, 11usize, 0u32, 2144768u32);
    emu.adi_no_count(15usize, 0usize, 0u32, 2144772u32);
    emu.lbu_no_count(10usize, 10usize, 0u32, 2144776u32);
    emu.adi_no_count(11usize, 2usize, 139u32, 2144780u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2144784u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2144788u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2144812u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ba2c));
}
#[inline(always)]
pub fn block_0x0020ba14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 55u32, 2144792u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2144796u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2144800u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2144804u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2144808u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2144844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ba4c));
    } else {
        emu.pc = 2144812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ba2c));
    }
}
#[inline(always)]
pub fn block_0x0020ba2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(13usize, 10usize, 15u32, 2144816u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2144788u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ba14));
    } else {
        emu.pc = 2144820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ba34));
    }
}
#[inline(always)]
pub fn block_0x0020ba34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 48u32, 2144824u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2144828u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2144832u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2144836u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2144840u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2144812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ba2c));
    } else {
        emu.pc = 2144844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ba4c));
    }
}
#[inline]
pub fn block_0x0020ba4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2144848u32);
    emu.sbr_no_count(10usize, 10usize, 15usize, 2144852u32);
    emu.adi_no_count(14usize, 10usize, 128u32, 2144856u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2144860u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 336u32, 2144864u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2144868u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2144872u32);
    emu.adi_no_count(10usize, 16usize, 0u32, 2144876u32);
    emu.apc_no_count(1usize, 2144876u32, 16384u32, 2144880u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2144884u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(2016u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020ba74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 140u32, 2144888u32)?;
    emu.adi_no_count(2usize, 2usize, 144u32, 2144892u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2144896u32;
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
pub fn block_0x0020ba80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967152u32, 2144900u32);
    emu.sw_no_count(1usize, 2usize, 140u32, 2144904u32)?;
    emu.adi_no_count(16usize, 11usize, 0u32, 2144908u32);
    emu.adi_no_count(15usize, 0usize, 0u32, 2144912u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2144916u32)?;
    emu.adi_no_count(11usize, 2usize, 139u32, 2144920u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2144924u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2144928u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2144952u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bab8));
}
#[inline(always)]
pub fn block_0x0020baa0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 87u32, 2144932u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2144936u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2144940u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2144944u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2144948u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2144984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bad8));
    } else {
        emu.pc = 2144952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bab8));
    }
}
#[inline(always)]
pub fn block_0x0020bab8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(13usize, 10usize, 15u32, 2144956u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2144928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020baa0));
    } else {
        emu.pc = 2144960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bac0));
    }
}
#[inline(always)]
pub fn block_0x0020bac0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 48u32, 2144964u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2144968u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2144972u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2144976u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2144980u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2144952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bab8));
    } else {
        emu.pc = 2144984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bad8));
    }
}
#[inline]
pub fn block_0x0020bad8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2144988u32);
    emu.sbr_no_count(10usize, 10usize, 15usize, 2144992u32);
    emu.adi_no_count(14usize, 10usize, 128u32, 2144996u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2145000u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 336u32, 2145004u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2145008u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2145012u32);
    emu.adi_no_count(10usize, 16usize, 0u32, 2145016u32);
    emu.apc_no_count(1usize, 2145016u32, 16384u32, 2145020u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2145024u32;
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
#[inline(always)]
pub fn block_0x0020bb00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 140u32, 2145028u32)?;
    emu.adi_no_count(2usize, 2usize, 144u32, 2145032u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2145036u32;
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
pub fn block_0x0020bb0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967152u32, 2145040u32);
    emu.sw_no_count(1usize, 2usize, 140u32, 2145044u32)?;
    emu.adi_no_count(16usize, 11usize, 0u32, 2145048u32);
    emu.adi_no_count(15usize, 0usize, 0u32, 2145052u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2145056u32)?;
    emu.adi_no_count(11usize, 2usize, 139u32, 2145060u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2145064u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2145068u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2145092u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bb44));
}
#[inline(always)]
pub fn block_0x0020bb2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 55u32, 2145072u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2145076u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2145080u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2145084u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2145088u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2145124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bb64));
    } else {
        emu.pc = 2145092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bb44));
    }
}
#[inline(always)]
pub fn block_0x0020bb44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(13usize, 10usize, 15u32, 2145096u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2145068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bb2c));
    } else {
        emu.pc = 2145100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bb4c));
    }
}
#[inline(always)]
pub fn block_0x0020bb4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 48u32, 2145104u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2145108u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2145112u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2145116u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2145120u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2145092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bb44));
    } else {
        emu.pc = 2145124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bb64));
    }
}
#[inline]
pub fn block_0x0020bb64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2145128u32);
    emu.sbr_no_count(10usize, 10usize, 15usize, 2145132u32);
    emu.adi_no_count(14usize, 10usize, 128u32, 2145136u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2145140u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 336u32, 2145144u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2145148u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2145152u32);
    emu.adi_no_count(10usize, 16usize, 0u32, 2145156u32);
    emu.apc_no_count(1usize, 2145156u32, 16384u32, 2145160u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2145164u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1736u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020bb8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 140u32, 2145168u32)?;
    emu.adi_no_count(2usize, 2usize, 144u32, 2145172u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2145176u32;
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
pub fn block_0x0020bb98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 4294967295u32, 2145180u32);
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2145184u32;
    emu.update_insn_clock();
    emu.sbr_no_count(13usize, 13usize, 11usize, 2145188u32);
    emu.xrr_no_count(11usize, 11usize, 12usize, 2145192u32);
    emu.sltru_no_count(11usize, 12usize, 11usize, 2145196u32);
    emu.sltru_no_count(10usize, 13usize, 10usize, 2145200u32);
    emu.xri_no_count(10usize, 10usize, 1u32, 2145204u32);
    emu.anr_no_count(10usize, 11usize, 10usize, 2145208u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2145212u32;
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
pub fn block_0x0020bbbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2145216u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2145220u32)?;
    emu.adi_no_count(13usize, 11usize, 0u32, 2145224u32);
    emu.lbu_no_count(11usize, 10usize, 0u32, 2145228u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2145232u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2145236u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 338u32, 2145240u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2145308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bc1c));
    } else {
        emu.pc = 2145244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bbdc));
    }
}
#[inline]
pub fn block_0x0020bbdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 41u32, 2145248u32);
    emu.adi_no_count(14usize, 0usize, 100u32, 2145252u32);
    emu.mul_no_count(12usize, 11usize, 12usize, 2145256u32);
    emu.sri_no_count(12usize, 12usize, 12u32, 2145260u32);
    emu.mul_no_count(14usize, 12usize, 14usize, 2145264u32);
    emu.sbr_no_count(14usize, 11usize, 14usize, 2145268u32);
    emu.sli_no_count(14usize, 14usize, 25u32, 2145272u32);
    emu.sri_no_count(14usize, 14usize, 24u32, 2145276u32);
    emu.adr_no_count(14usize, 10usize, 14usize, 2145280u32);
    emu.lbu_no_count(15usize, 14usize, 0u32, 2145284u32);
    emu.lbu_no_count(14usize, 14usize, 1u32, 2145288u32);
    emu.sb_no_count(15usize, 2usize, 10u32, 2145292u32);
    emu.sb_no_count(14usize, 2usize, 11u32, 2145296u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2145300u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2145320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bc28));
    } else {
        emu.pc = 2145304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bc18));
    }
}
#[inline(always)]
pub fn block_0x0020bc18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2145308u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2145324u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bc2c));
}
#[inline(always)]
pub fn block_0x0020bc1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 3u32, 2145312u32);
    emu.adi_no_count(12usize, 11usize, 0u32, 2145316u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2145324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bc2c));
    } else {
        emu.pc = 2145320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bc28));
    }
}
#[inline(always)]
pub fn block_0x0020bc28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2145360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bc50));
    } else {
        emu.pc = 2145324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bc2c));
    }
}
#[inline]
pub fn block_0x0020bc2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 1u32, 2145328u32);
    emu.adi_no_count(12usize, 12usize, 1u32, 2145332u32);
    emu.ani_no_count(11usize, 12usize, 255u32, 2145336u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2145340u32);
    emu.lbu_no_count(10usize, 10usize, 0u32, 2145344u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2145348u32);
    emu.adi_no_count(11usize, 2usize, 9u32, 2145352u32);
    emu.adr_no_count(11usize, 11usize, 14usize, 2145356u32);
    emu.sb_no_count(10usize, 11usize, 0u32, 2145360u32);
    emu.add_memory_rw_events(9usize);
    emu.pc = 2145360u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bc50));
}
#[inline]
pub fn block_0x0020bc50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 3u32, 2145364u32);
    emu.adi_no_count(10usize, 2usize, 9u32, 2145368u32);
    emu.sbr_no_count(15usize, 15usize, 14usize, 2145372u32);
    emu.adr_no_count(14usize, 10usize, 14usize, 2145376u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2145380u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2145384u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2145388u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2145392u32);
    emu.apc_no_count(1usize, 2145392u32, 16384u32, 2145396u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2145400u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1500u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
