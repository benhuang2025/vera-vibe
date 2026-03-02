pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2203944u32;
pub const PC_MAX: u32 = 2206244u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 107usize] = [
        block_0x0021a128,
        block_0x0021a138,
        block_0x0021a144,
        block_0x0021a160,
        block_0x0021a170,
        block_0x0021a1b4,
        block_0x0021a208,
        block_0x0021a214,
        block_0x0021a24c,
        block_0x0021a250,
        block_0x0021a254,
        block_0x0021a26c,
        block_0x0021a270,
        block_0x0021a28c,
        block_0x0021a2a0,
        block_0x0021a2a4,
        block_0x0021a2a8,
        block_0x0021a2d4,
        block_0x0021a2f4,
        block_0x0021a314,
        block_0x0021a320,
        block_0x0021a340,
        block_0x0021a370,
        block_0x0021a384,
        block_0x0021a39c,
        block_0x0021a3ac,
        block_0x0021a3b4,
        block_0x0021a3c4,
        block_0x0021a3d4,
        block_0x0021a3f4,
        block_0x0021a3fc,
        block_0x0021a42c,
        block_0x0021a43c,
        block_0x0021a440,
        block_0x0021a474,
        block_0x0021a484,
        block_0x0021a4a0,
        block_0x0021a4ac,
        block_0x0021a4f0,
        block_0x0021a500,
        block_0x0021a510,
        block_0x0021a518,
        block_0x0021a528,
        block_0x0021a540,
        block_0x0021a560,
        block_0x0021a574,
        block_0x0021a59c,
        block_0x0021a5a8,
        block_0x0021a5dc,
        block_0x0021a5e4,
        block_0x0021a5e8,
        block_0x0021a5f4,
        block_0x0021a5f8,
        block_0x0021a61c,
        block_0x0021a69c,
        block_0x0021a6a4,
        block_0x0021a6b0,
        block_0x0021a6c8,
        block_0x0021a6d8,
        block_0x0021a6e0,
        block_0x0021a6e4,
        block_0x0021a6e8,
        block_0x0021a6fc,
        block_0x0021a704,
        block_0x0021a70c,
        block_0x0021a71c,
        block_0x0021a730,
        block_0x0021a784,
        block_0x0021a790,
        block_0x0021a794,
        block_0x0021a7a4,
        block_0x0021a7ac,
        block_0x0021a7b4,
        block_0x0021a7c4,
        block_0x0021a7d8,
        block_0x0021a808,
        block_0x0021a820,
        block_0x0021a834,
        block_0x0021a844,
        block_0x0021a84c,
        block_0x0021a854,
        block_0x0021a86c,
        block_0x0021a878,
        block_0x0021a884,
        block_0x0021a8bc,
        block_0x0021a8e8,
        block_0x0021a8ec,
        block_0x0021a8f0,
        block_0x0021a900,
        block_0x0021a904,
        block_0x0021a914,
        block_0x0021a930,
        block_0x0021a938,
        block_0x0021a95c,
        block_0x0021a970,
        block_0x0021a988,
        block_0x0021a998,
        block_0x0021a9a0,
        block_0x0021a9a4,
        block_0x0021a9b8,
        block_0x0021a9c4,
        block_0x0021a9cc,
        block_0x0021a9d0,
        block_0x0021a9e0,
        block_0x0021a9fc,
        block_0x0021aa04,
        block_0x0021aa24,
    ];
    const IDX: [u16; 576usize] = [
        1u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 3u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 4u16, 0u16, 0u16, 0u16, 5u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 6u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 7u16, 0u16, 0u16, 8u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 9u16, 10u16, 11u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 12u16, 13u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 14u16, 0u16,
        0u16, 0u16, 0u16, 15u16, 16u16, 17u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 18u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 19u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 20u16, 0u16, 0u16, 21u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 22u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 23u16, 0u16, 0u16, 0u16, 0u16, 24u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 25u16, 0u16, 0u16, 0u16, 26u16, 0u16, 27u16, 0u16, 0u16, 0u16, 28u16, 0u16,
        0u16, 0u16, 29u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 30u16, 0u16, 31u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 32u16, 0u16,
        0u16, 0u16, 33u16, 34u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 35u16, 0u16, 0u16, 0u16, 36u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 37u16, 0u16, 0u16, 38u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 39u16, 0u16, 0u16, 0u16, 40u16,
        0u16, 0u16, 0u16, 41u16, 0u16, 42u16, 0u16, 0u16, 0u16, 43u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 44u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 45u16, 0u16, 0u16,
        0u16, 0u16, 46u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 47u16,
        0u16, 0u16, 48u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 49u16, 0u16, 50u16, 51u16, 0u16, 0u16, 52u16, 53u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 54u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 55u16,
        0u16, 56u16, 0u16, 0u16, 57u16, 0u16, 0u16, 0u16, 0u16, 0u16, 58u16, 0u16, 0u16,
        0u16, 59u16, 0u16, 60u16, 61u16, 62u16, 0u16, 0u16, 0u16, 0u16, 63u16, 0u16,
        64u16, 0u16, 65u16, 0u16, 0u16, 0u16, 66u16, 0u16, 0u16, 0u16, 0u16, 67u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 68u16, 0u16, 0u16, 69u16, 70u16, 0u16, 0u16,
        0u16, 71u16, 0u16, 72u16, 0u16, 73u16, 0u16, 0u16, 0u16, 74u16, 0u16, 0u16, 0u16,
        0u16, 75u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        76u16, 0u16, 0u16, 0u16, 0u16, 0u16, 77u16, 0u16, 0u16, 0u16, 0u16, 78u16, 0u16,
        0u16, 0u16, 79u16, 0u16, 80u16, 0u16, 81u16, 0u16, 0u16, 0u16, 0u16, 0u16, 82u16,
        0u16, 0u16, 83u16, 0u16, 0u16, 84u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 85u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 86u16, 87u16, 88u16, 0u16, 0u16, 0u16, 89u16, 90u16,
        0u16, 0u16, 0u16, 91u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 92u16, 0u16, 93u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 94u16, 0u16, 0u16, 0u16, 0u16,
        95u16, 0u16, 0u16, 0u16, 0u16, 0u16, 96u16, 0u16, 0u16, 0u16, 97u16, 0u16, 98u16,
        99u16, 0u16, 0u16, 0u16, 0u16, 100u16, 0u16, 0u16, 101u16, 0u16, 102u16, 103u16,
        0u16, 0u16, 0u16, 104u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 105u16, 0u16,
        106u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 107u16,
    ];
    if pc < 2203944u32 || pc > 2206244u32 {
        return None;
    }
    let word_offset = ((pc - 2203944u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x0021a128(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2203948u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2203952u32)?;
    emu.apc_no_count(1usize, 2203952u32, 4294959104u32, 2203956u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2203960u32;
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
#[inline(always)]
pub fn block_0x0021a138(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2203964u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2203968u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2203972u32;
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
pub fn block_0x0021a144(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2203976u32);
    let a = 0u32.wrapping_add(2248704u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2203980u32;
    emu.update_insn_clock();
    emu.lbu_no_count(12usize, 11usize, 4294965825u32, 2203984u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2203988u32);
    emu.sb_no_count(12usize, 2usize, 7u32, 2203992u32);
    emu.sb_no_count(10usize, 11usize, 4294965825u32, 2203996u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2204016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a170));
    } else {
        emu.pc = 2204000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a160));
    }
}
#[inline(always)]
pub fn block_0x0021a160(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2248704u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2204004u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965825u32, 2204008u32);
    emu.adi_no_count(2usize, 2usize, 32u32, 2204012u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2204016u32;
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
pub fn block_0x0021a170(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2204020u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965636u32, 2204024u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2204028u32);
    emu.sw_no_count(11usize, 2usize, 8u32, 2204032u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2204036u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2204040u32)?;
    emu.sw_no_count(0usize, 2usize, 20u32, 2204044u32)?;
    emu.sw_no_count(0usize, 2usize, 24u32, 2204048u32)?;
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2204052u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1254u32, 2204056u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2204060u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965644u32, 2204064u32);
    emu.adi_no_count(11usize, 2usize, 7u32, 2204068u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2204072u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2204076u32);
    emu.apc_no_count(1usize, 2204076u32, 0u32, 2204080u32);
    emu.add_memory_rw_events(17usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2204084u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(404u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021a1b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2204088u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2204092u32)?;
    emu.adi_no_count(11usize, 12usize, 0u32, 2204096u32);
    emu.sb_no_count(14usize, 2usize, 43u32, 2204100u32);
    emu.adi_no_count(12usize, 2usize, 43u32, 2204104u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2204108u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 532u32, 2204112u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2204116u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 396u32, 2204120u32);
    emu.adi_no_count(16usize, 0usize, 1u32, 2204124u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2204128u32)?;
    emu.sw_no_count(12usize, 2usize, 32u32, 2204132u32)?;
    emu.sw_no_count(14usize, 2usize, 36u32, 2204136u32)?;
    emu.adi_no_count(12usize, 2usize, 32u32, 2204140u32);
    emu.lw_no_count(13usize, 13usize, 36u32, 2204144u32)?;
    emu.sw_no_count(15usize, 2usize, 8u32, 2204148u32)?;
    emu.sw_no_count(16usize, 2usize, 12u32, 2204152u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2204156u32)?;
    emu.sw_no_count(16usize, 2usize, 20u32, 2204160u32)?;
    emu.adi_no_count(12usize, 2usize, 8u32, 2204164u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2204168u32;
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
pub fn block_0x0021a208(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2204172u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2204176u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2204180u32;
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
pub fn block_0x0021a214(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2204184u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2204188u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2204192u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2204196u32)?;
    emu.sw_no_count(18usize, 2usize, 0u32, 2204200u32)?;
    emu.lw_no_count(12usize, 11usize, 4u32, 2204204u32)?;
    emu.lw_no_count(8usize, 11usize, 0u32, 2204208u32)?;
    emu.lbu_no_count(18usize, 10usize, 0u32, 2204212u32);
    emu.lw_no_count(9usize, 12usize, 12u32, 2204216u32)?;
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2204220u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965512u32, 2204224u32);
    emu.adi_no_count(12usize, 0usize, 17u32, 2204228u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2204232u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(9usize);
    let return_addr = 2204236u32;
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
pub fn block_0x0021a24c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2204268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a26c));
    } else {
        emu.pc = 2204240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a250));
    }
}
#[inline(always)]
pub fn block_0x0021a250(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2204244u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2204244u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021a254));
}
#[inline(always)]
pub fn block_0x0021a254(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2204248u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2204252u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2204256u32)?;
    emu.lw_no_count(18usize, 2usize, 0u32, 2204260u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2204264u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2204268u32;
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
pub fn block_0x0021a26c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2204300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a28c));
    } else {
        emu.pc = 2204272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a270));
    }
}
#[inline(always)]
pub fn block_0x0021a270(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2204276u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2204280u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2204284u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2204288u32)?;
    emu.lw_no_count(18usize, 2usize, 0u32, 2204292u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2204296u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2204300u32;
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
pub fn block_0x0021a28c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2204304u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965529u32, 2204308u32);
    emu.adi_no_count(12usize, 0usize, 88u32, 2204312u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2204316u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(9usize);
    let return_addr = 2204320u32;
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
pub fn block_0x0021a2a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2204240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a250));
    } else {
        emu.pc = 2204324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a2a4));
    }
}
#[inline(always)]
pub fn block_0x0021a2a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2204328u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2204244u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021a254));
}
#[inline]
pub fn block_0x0021a2a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2204332u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2204336u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2204340u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2204344u32)?;
    emu.adi_no_count(8usize, 13usize, 0u32, 2204348u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2204352u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2204356u32);
    emu.adi_no_count(10usize, 0usize, 2u32, 2204360u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2204364u32);
    emu.apc_no_count(1usize, 2204364u32, 4294897664u32, 2204368u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2204372u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1176u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021a2d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2204376u32);
    emu.sb_no_count(10usize, 9usize, 0u32, 2204380u32);
    emu.sw_no_count(8usize, 9usize, 4u32, 2204384u32)?;
    emu.lw_no_count(1usize, 2usize, 12u32, 2204388u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2204392u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2204396u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2204400u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2204404u32;
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
pub fn block_0x0021a2f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2204408u32)?;
    emu.lw_no_count(13usize, 10usize, 4u32, 2204412u32)?;
    emu.adi_no_count(14usize, 11usize, 0u32, 2204416u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2204420u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2204424u32);
    emu.adi_no_count(12usize, 14usize, 0u32, 2204428u32);
    emu.apc_no_count(6usize, 2204428u32, 16384u32, 2204432u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2204436u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966300u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021a314(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2204440u32)?;
    emu.apc_no_count(6usize, 2204440u32, 16384u32, 2204444u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2204448u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966236u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021a320(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2204452u32)?;
    emu.lw_no_count(13usize, 10usize, 4u32, 2204456u32)?;
    emu.adi_no_count(14usize, 11usize, 0u32, 2204460u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2204464u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2204468u32);
    emu.adi_no_count(12usize, 14usize, 0u32, 2204472u32);
    emu.apc_no_count(6usize, 2204472u32, 16384u32, 2204476u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2204480u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x0021a340(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2204484u32);
    emu.adi_no_count(16usize, 14usize, 0u32, 2204488u32);
    emu.adi_no_count(15usize, 13usize, 0u32, 2204492u32);
    emu.sw_no_count(11usize, 2usize, 8u32, 2204496u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2204500u32)?;
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2204504u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965620u32, 2204508u32);
    emu.adi_no_count(11usize, 2usize, 8u32, 2204512u32);
    emu.adi_no_count(13usize, 2usize, 12u32, 2204516u32);
    emu.adi_no_count(14usize, 12usize, 0u32, 2204520u32);
    emu.apc_no_count(1usize, 2204520u32, 8192u32, 2204524u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2204528u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965472u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021a370(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2204532u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2204536u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2204540u32)?;
    emu.lw_no_count(6usize, 12usize, 12u32, 2204544u32)?;
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2204548u32;
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
pub fn block_0x0021a384(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2204552u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2204556u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2204560u32)?;
    emu.lw_no_count(8usize, 10usize, 0u32, 2204564u32)?;
    emu.lw_no_count(11usize, 8usize, 12u32, 2204568u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2204588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a3ac));
    } else {
        emu.pc = 2204572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a39c));
    }
}
#[inline(always)]
pub fn block_0x0021a39c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 16u32, 2204576u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2204580u32);
    emu.apc_no_count(1usize, 2204580u32, 4294893568u32, 2204584u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2204588u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(836u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021a3ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4294967295u32, 2204592u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2204612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a3c4));
    } else {
        emu.pc = 2204596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a3b4));
    }
}
#[inline(always)]
pub fn block_0x0021a3b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 4u32, 2204600u32)?;
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2204604u32);
    emu.sw_no_count(10usize, 8usize, 4u32, 2204608u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2204628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a3d4));
    } else {
        emu.pc = 2204612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a3c4));
    }
}
#[inline(always)]
pub fn block_0x0021a3c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2204616u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2204620u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2204624u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2204628u32;
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
pub fn block_0x0021a3d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 24u32, 2204632u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2204636u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2204640u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2204644u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2204648u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2204652u32);
    emu.apc_no_count(6usize, 2204652u32, 4294893568u32, 2204656u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2204660u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x0021a3f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2204660u32, 4294959104u32, 2204664u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2204668u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x0021a3fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2204672u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2204676u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2204680u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2204684u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2204688u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2204692u32)?;
    emu.adi_no_count(9usize, 13usize, 0u32, 2204696u32);
    emu.adi_no_count(19usize, 12usize, 0u32, 2204700u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2204704u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2204708u32);
    emu.apc_no_count(1usize, 2204708u32, 4294897664u32, 2204712u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2204716u32;
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
#[inline(always)]
pub fn block_0x0021a42c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 12u32, 2204720u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2204724u32);
    emu.apc_no_count(1usize, 2204724u32, 4294893568u32, 2204728u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2204732u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(664u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021a43c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2204788u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a474));
    } else {
        emu.pc = 2204736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a440));
    }
}
#[inline]
pub fn block_0x0021a440(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(19usize, 10usize, 0u32, 2204740u32)?;
    emu.sw_no_count(9usize, 10usize, 4u32, 2204744u32)?;
    emu.sb_no_count(18usize, 10usize, 8u32, 2204748u32);
    emu.adi_no_count(11usize, 0usize, 3u32, 2204752u32);
    emu.sb_no_count(11usize, 8usize, 0u32, 2204756u32);
    emu.sw_no_count(10usize, 8usize, 4u32, 2204760u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2204764u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2204768u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2204772u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2204776u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2204780u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2204784u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2204788u32;
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
pub fn block_0x0021a474(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2204792u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2204796u32);
    emu.apc_no_count(1usize, 2204796u32, 0u32, 2204800u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2204804u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1680u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021a484(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2204808u32);
    emu.adi_no_count(11usize, 10usize, 0u32, 2204812u32);
    emu.lbu_no_count(12usize, 10usize, 0u32, 2204816u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2204820u32);
    emu.sb_no_count(12usize, 2usize, 7u32, 2204824u32);
    emu.sb_no_count(10usize, 11usize, 0u32, 2204828u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2204844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a4ac));
    } else {
        emu.pc = 2204832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a4a0));
    }
}
#[inline(always)]
pub fn block_0x0021a4a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2204836u32);
    emu.adi_no_count(2usize, 2usize, 32u32, 2204840u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2204844u32;
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
pub fn block_0x0021a4ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2204848u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965636u32, 2204852u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2204856u32);
    emu.sw_no_count(11usize, 2usize, 8u32, 2204860u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2204864u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2204868u32)?;
    emu.sw_no_count(0usize, 2usize, 20u32, 2204872u32)?;
    emu.sw_no_count(0usize, 2usize, 24u32, 2204876u32)?;
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2204880u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1254u32, 2204884u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2204888u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965644u32, 2204892u32);
    emu.adi_no_count(11usize, 2usize, 7u32, 2204896u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2204900u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2204904u32);
    emu.apc_no_count(1usize, 2204904u32, 0u32, 2204908u32);
    emu.add_memory_rw_events(17usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2204912u32;
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
pub fn block_0x0021a4f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 4u32, 2204916u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2204920u32)?;
    emu.lw_no_count(6usize, 12usize, 12u32, 2204924u32)?;
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2204928u32;
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
pub fn block_0x0021a500(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2248704u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2204932u32;
    emu.update_insn_clock();
    emu.lw_no_count(12usize, 11usize, 4294965840u32, 2204936u32)?;
    emu.adi_no_count(11usize, 0usize, 2u32, 2204940u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2204968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a528));
    } else {
        emu.pc = 2204944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a510));
    }
}
#[inline(always)]
pub fn block_0x0021a510(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 12usize, 8u32, 2204948u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2205044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a574));
    } else {
        emu.pc = 2204952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a518));
    }
}
#[inline(always)]
pub fn block_0x0021a518(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 12usize, 12u32, 2204956u32)?;
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2204960u32);
    emu.apc_no_count(6usize, 2204960u32, 0u32, 2204964u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2204968u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x0021a528(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2248704u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2204972u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965840u32, 2204976u32);
    emu.lw_no_count(11usize, 12usize, 8u32, 2204980u32)?;
    emu.lw_no_count(12usize, 12usize, 12u32, 2204984u32)?;
    emu.orr_no_count(13usize, 11usize, 12usize, 2204988u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2205084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a59c));
    } else {
        emu.pc = 2204992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a540));
    }
}
#[inline(always)]
pub fn block_0x0021a540(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2248704u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2204996u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2248704u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2205000u32;
    emu.update_insn_clock();
    emu.lw_no_count(14usize, 14usize, 4294965832u32, 2205004u32)?;
    emu.lw_no_count(13usize, 13usize, 4294965828u32, 2205008u32)?;
    emu.xrr_no_count(12usize, 14usize, 12usize, 2205012u32);
    emu.xrr_no_count(11usize, 13usize, 11usize, 2205016u32);
    emu.orr_no_count(11usize, 11usize, 12usize, 2205020u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2205084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a59c));
    } else {
        emu.pc = 2205024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a560));
    }
}
#[inline(always)]
pub fn block_0x0021a560(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2205028u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967076u32, 2205032u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2205036u32);
    emu.apc_no_count(6usize, 2205036u32, 0u32, 2205040u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2205044u32;
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
#[inline]
pub fn block_0x0021a574(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 12usize, 0u32, 2205048u32)?;
    emu.lw_no_count(12usize, 12usize, 4u32, 2205052u32)?;
    let a = 0u32.wrapping_add(2248704u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2205056u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294965840u32, 2205060u32);
    emu.lw_no_count(14usize, 13usize, 12u32, 2205064u32)?;
    emu.lw_no_count(13usize, 13usize, 8u32, 2205068u32)?;
    emu.xrr_no_count(12usize, 12usize, 14usize, 2205072u32);
    emu.xrr_no_count(11usize, 11usize, 13usize, 2205076u32);
    emu.orr_no_count(11usize, 11usize, 12usize, 2205080u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2205024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a560));
    } else {
        emu.pc = 2205084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a59c));
    }
}
#[inline(always)]
pub fn block_0x0021a59c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2205088u32);
    emu.apc_no_count(6usize, 2205088u32, 0u32, 2205092u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2205096u32;
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
pub fn block_0x0021a5a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294966656u32, 2205100u32);
    emu.sw_no_count(1usize, 2usize, 636u32, 2205104u32)?;
    emu.sw_no_count(8usize, 2usize, 632u32, 2205108u32)?;
    emu.sw_no_count(9usize, 2usize, 628u32, 2205112u32)?;
    emu.sw_no_count(18usize, 2usize, 624u32, 2205116u32)?;
    emu.sw_no_count(19usize, 2usize, 620u32, 2205120u32)?;
    emu.sw_no_count(20usize, 2usize, 616u32, 2205124u32)?;
    emu.sw_no_count(21usize, 2usize, 612u32, 2205128u32)?;
    emu.sw_no_count(22usize, 2usize, 608u32, 2205132u32)?;
    emu.sw_no_count(23usize, 2usize, 604u32, 2205136u32)?;
    emu.sw_no_count(24usize, 2usize, 600u32, 2205140u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2205144u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2205160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a5e8));
    } else {
        emu.pc = 2205148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a5dc));
    }
}
#[inline(always)]
pub fn block_0x0021a5dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 11usize, 0u32, 2205152u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2205172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a5f4));
    } else {
        emu.pc = 2205156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a5e4));
    }
}
#[inline(always)]
pub fn block_0x0021a5e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2205160u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2205176u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021a5f8));
}
#[inline(always)]
pub fn block_0x0021a5e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2205164u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965660u32, 2205168u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2205176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a5f8));
    } else {
        emu.pc = 2205172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a5f4));
    }
}
#[inline(always)]
pub fn block_0x0021a5f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 9u32, 2205176u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2205176u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021a5f8));
}
#[inline]
pub fn block_0x0021a5f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 8u32, 2205180u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2205184u32)?;
    emu.adi_no_count(9usize, 2usize, 16u32, 2205188u32);
    emu.adi_no_count(10usize, 2usize, 16u32, 2205192u32);
    emu.adi_no_count(12usize, 0usize, 512u32, 2205196u32);
    emu.adi_no_count(18usize, 0usize, 512u32, 2205200u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2205204u32);
    emu.apc_no_count(1usize, 2205204u32, 4294897664u32, 2205208u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2205212u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(344u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0021a61c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 32u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(9usize, 2usize, 528u32, 2205216u32)?;
    emu.sw_no_count(18usize, 2usize, 532u32, 2205220u32)?;
    emu.sw_no_count(0usize, 2usize, 536u32, 2205224u32)?;
    emu.sw_no_count(0usize, 2usize, 540u32, 2205228u32)?;
    emu.lw_no_count(21usize, 8usize, 0u32, 2205232u32)?;
    emu.lw_no_count(19usize, 8usize, 4u32, 2205236u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2205240u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(20usize, a);
    emu.pc = 2205244u32;
    emu.update_insn_clock();
    emu.adi_no_count(20usize, 20usize, 800u32, 2205248u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(23usize, a);
    emu.pc = 2205252u32;
    emu.update_insn_clock();
    emu.adi_no_count(23usize, 23usize, 1512u32, 2205256u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(22usize, a);
    emu.pc = 2205260u32;
    emu.update_insn_clock();
    emu.adi_no_count(22usize, 22usize, 4294965716u32, 2205264u32);
    emu.adi_no_count(18usize, 0usize, 4u32, 2205268u32);
    emu.sw_no_count(0usize, 2usize, 568u32, 2205272u32)?;
    emu.adi_no_count(11usize, 2usize, 576u32, 2205276u32);
    emu.adi_no_count(9usize, 0usize, 3u32, 2205280u32);
    emu.sw_no_count(10usize, 2usize, 576u32, 2205284u32)?;
    emu.sw_no_count(20usize, 2usize, 580u32, 2205288u32)?;
    emu.sw_no_count(21usize, 2usize, 584u32, 2205292u32)?;
    emu.sw_no_count(23usize, 2usize, 588u32, 2205296u32)?;
    emu.sw_no_count(19usize, 2usize, 592u32, 2205300u32)?;
    emu.sw_no_count(20usize, 2usize, 596u32, 2205304u32)?;
    emu.sw_no_count(22usize, 2usize, 552u32, 2205308u32)?;
    emu.sw_no_count(18usize, 2usize, 556u32, 2205312u32)?;
    emu.sw_no_count(11usize, 2usize, 560u32, 2205316u32)?;
    emu.sw_no_count(9usize, 2usize, 564u32, 2205320u32)?;
    emu.adi_no_count(10usize, 2usize, 544u32, 2205324u32);
    emu.adi_no_count(11usize, 2usize, 528u32, 2205328u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2205332u32);
    emu.apc_no_count(1usize, 2205332u32, 4294959104u32, 2205336u32);
    emu.add_memory_rw_events(32usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2205340u32;
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
pub fn block_0x0021a69c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 544u32, 2205344u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2205412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a6e4));
    } else {
        emu.pc = 2205348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a6a4));
    }
}
#[inline(always)]
pub fn block_0x0021a6a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 2usize, 536u32, 2205352u32)?;
    emu.adi_no_count(10usize, 0usize, 513u32, 2205356u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2205704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a808));
    } else {
        emu.pc = 2205360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a6b0));
    }
}
#[inline(always)]
pub fn block_0x0021a6b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 12u32, 2205364u32)?;
    emu.lw_no_count(11usize, 8usize, 8u32, 2205368u32)?;
    emu.lw_no_count(14usize, 10usize, 28u32, 2205372u32)?;
    emu.adi_no_count(10usize, 2usize, 576u32, 2205376u32);
    emu.adi_no_count(12usize, 2usize, 16u32, 2205380u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2205384u32;
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
pub fn block_0x0021a6c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 576u32, 2205388u32);
    emu.lw_no_count(8usize, 2usize, 580u32, 2205392u32)?;
    emu.adi_no_count(11usize, 0usize, 4u32, 2205396u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2205588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a794));
    } else {
        emu.pc = 2205400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a6d8));
    }
}
#[inline(always)]
pub fn block_0x0021a6d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 3u32, 2205404u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2205656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a7d8));
    } else {
        emu.pc = 2205408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a6e0));
    }
}
#[inline(always)]
pub fn block_0x0021a6e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2205412u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2205588u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021a794));
}
#[inline(always)]
pub fn block_0x0021a6e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2205488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a730));
    } else {
        emu.pc = 2205416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a6e8));
    }
}
#[inline(always)]
pub fn block_0x0021a6e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(9usize, 2usize, 548u32, 2205420u32)?;
    emu.lw_no_count(24usize, 9usize, 4u32, 2205424u32)?;
    emu.lw_no_count(11usize, 24usize, 0u32, 2205428u32)?;
    emu.lw_no_count(18usize, 9usize, 0u32, 2205432u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2205444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a704));
    } else {
        emu.pc = 2205436u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a6fc));
    }
}
#[inline(always)]
pub fn block_0x0021a6fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2205440u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2205444u32;
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
pub fn block_0x0021a704(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 24usize, 4u32, 2205448u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2205468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a71c));
    } else {
        emu.pc = 2205452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a70c));
    }
}
#[inline(always)]
pub fn block_0x0021a70c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 24usize, 8u32, 2205456u32)?;
    emu.adi_no_count(10usize, 18usize, 0u32, 2205460u32);
    emu.apc_no_count(1usize, 2205460u32, 4294893568u32, 2205464u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2205468u32;
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
pub fn block_0x0021a71c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 12u32, 2205472u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2205476u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2205480u32);
    emu.apc_no_count(1usize, 2205480u32, 4294893568u32, 2205484u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2205488u32;
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
pub fn block_0x0021a730(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 8usize, 8u32, 2205492u32)?;
    emu.lw_no_count(10usize, 8usize, 12u32, 2205496u32)?;
    emu.adi_no_count(12usize, 2usize, 8u32, 2205500u32);
    emu.adi_no_count(18usize, 0usize, 4u32, 2205504u32);
    emu.sw_no_count(0usize, 2usize, 568u32, 2205508u32)?;
    emu.adi_no_count(13usize, 2usize, 576u32, 2205512u32);
    emu.lw_no_count(14usize, 10usize, 36u32, 2205516u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2205520u32)?;
    emu.sw_no_count(20usize, 2usize, 580u32, 2205524u32)?;
    emu.sw_no_count(21usize, 2usize, 584u32, 2205528u32)?;
    emu.sw_no_count(23usize, 2usize, 588u32, 2205532u32)?;
    emu.adi_no_count(9usize, 0usize, 3u32, 2205536u32);
    emu.sw_no_count(19usize, 2usize, 592u32, 2205540u32)?;
    emu.sw_no_count(20usize, 2usize, 596u32, 2205544u32)?;
    emu.sw_no_count(22usize, 2usize, 552u32, 2205548u32)?;
    emu.sw_no_count(18usize, 2usize, 556u32, 2205552u32)?;
    emu.sw_no_count(13usize, 2usize, 560u32, 2205556u32)?;
    emu.sw_no_count(9usize, 2usize, 564u32, 2205560u32)?;
    emu.adi_no_count(10usize, 2usize, 544u32, 2205564u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2205568u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2205572u32;
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
pub fn block_0x0021a784(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 544u32, 2205576u32);
    emu.lw_no_count(8usize, 2usize, 548u32, 2205580u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a < b {
        emu.pc = 2205588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a794));
    } else {
        emu.pc = 2205584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a790));
    }
}
#[inline(always)]
pub fn block_0x0021a790(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2205656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a7d8));
    } else {
        emu.pc = 2205588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a794));
    }
}
#[inline(always)]
pub fn block_0x0021a794(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 8usize, 4u32, 2205592u32)?;
    emu.lw_no_count(11usize, 18usize, 0u32, 2205596u32)?;
    emu.lw_no_count(9usize, 8usize, 0u32, 2205600u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2205612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a7ac));
    } else {
        emu.pc = 2205604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a7a4));
    }
}
#[inline(always)]
pub fn block_0x0021a7a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2205608u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2205612u32;
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
pub fn block_0x0021a7ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 18usize, 4u32, 2205616u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2205636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a7c4));
    } else {
        emu.pc = 2205620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a7b4));
    }
}
#[inline(always)]
pub fn block_0x0021a7b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 8u32, 2205624u32)?;
    emu.adi_no_count(10usize, 9usize, 0u32, 2205628u32);
    emu.apc_no_count(1usize, 2205628u32, 4294893568u32, 2205632u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2205636u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967084u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021a7c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 12u32, 2205640u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2205644u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2205648u32);
    emu.apc_no_count(1usize, 2205648u32, 4294893568u32, 2205652u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2205656u32;
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
pub fn block_0x0021a7d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 636u32, 2205660u32)?;
    emu.lw_no_count(8usize, 2usize, 632u32, 2205664u32)?;
    emu.lw_no_count(9usize, 2usize, 628u32, 2205668u32)?;
    emu.lw_no_count(18usize, 2usize, 624u32, 2205672u32)?;
    emu.lw_no_count(19usize, 2usize, 620u32, 2205676u32)?;
    emu.lw_no_count(20usize, 2usize, 616u32, 2205680u32)?;
    emu.lw_no_count(21usize, 2usize, 612u32, 2205684u32)?;
    emu.lw_no_count(22usize, 2usize, 608u32, 2205688u32)?;
    emu.lw_no_count(23usize, 2usize, 604u32, 2205692u32)?;
    emu.lw_no_count(24usize, 2usize, 600u32, 2205696u32)?;
    emu.adi_no_count(2usize, 2usize, 640u32, 2205700u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2205704u32;
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
pub fn block_0x0021a808(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2205708u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965672u32, 2205712u32);
    emu.adi_no_count(11usize, 0usize, 512u32, 2205716u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2205720u32);
    emu.apc_no_count(1usize, 2205720u32, 16384u32, 2205724u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2205728u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(616u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021a820(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2248704u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2205732u32;
    emu.update_insn_clock();
    emu.lw_no_count(12usize, 11usize, 4294965856u32, 2205736u32)?;
    emu.adi_no_count(13usize, 12usize, 1u32, 2205740u32);
    emu.sw_no_count(13usize, 11usize, 4294965856u32, 2205744u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2205772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a84c));
    } else {
        emu.pc = 2205748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a834));
    }
}
#[inline(always)]
pub fn block_0x0021a834(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2248704u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2205752u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965856u32, 2205756u32);
    emu.lbu_no_count(12usize, 11usize, 8u32, 2205760u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2205780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a854));
    } else {
        emu.pc = 2205764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a844));
    }
}
#[inline(always)]
pub fn block_0x0021a844(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2205768u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2205772u32;
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
pub fn block_0x0021a84c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2205776u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2205780u32;
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
pub fn block_0x0021a854(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 4u32, 2205784u32)?;
    emu.adi_no_count(12usize, 12usize, 1u32, 2205788u32);
    emu.sw_no_count(12usize, 11usize, 4u32, 2205792u32)?;
    emu.sb_no_count(10usize, 11usize, 8u32, 2205796u32);
    emu.adi_no_count(10usize, 0usize, 2u32, 2205800u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2205804u32;
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
pub fn block_0x0021a86c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2248704u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2205808u32;
    emu.update_insn_clock();
    emu.sb_no_count(0usize, 10usize, 4294965864u32, 2205812u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2205816u32;
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
pub fn block_0x0021a878(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2248704u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2205820u32;
    emu.update_insn_clock();
    emu.lw_no_count(10usize, 10usize, 4294965860u32, 2205824u32)?;
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2205828u32;
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
pub fn block_0x0021a884(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2205832u32);
    emu.adi_no_count(11usize, 10usize, 0u32, 2205836u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2205840u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965768u32, 2205844u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2205848u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2205852u32)?;
    emu.adi_no_count(13usize, 0usize, 4u32, 2205856u32);
    emu.sw_no_count(10usize, 2usize, 8u32, 2205860u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2205864u32)?;
    emu.sw_no_count(13usize, 2usize, 16u32, 2205868u32)?;
    emu.sw_no_count(0usize, 2usize, 20u32, 2205872u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2205876u32);
    emu.apc_no_count(1usize, 2205876u32, 4096u32, 2205880u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2205884u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(680u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021a8bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2205888u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2205892u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2205896u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2205900u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2205904u32)?;
    emu.adi_no_count(9usize, 10usize, 0u32, 2205908u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2205912u32)?;
    emu.sli_no_count(18usize, 10usize, 1u32, 2205916u32);
    emu.adi_no_count(12usize, 0usize, 8u32, 2205920u32);
    emu.adi_no_count(8usize, 11usize, 0u32, 2205924u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2205932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a8ec));
    } else {
        emu.pc = 2205928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a8e8));
    }
}
#[inline(always)]
pub fn block_0x0021a8e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 8u32, 2205932u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2205932u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021a8ec));
}
#[inline(always)]
pub fn block_0x0021a8ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2205952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a900));
    } else {
        emu.pc = 2205936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a8f0));
    }
}
#[inline(always)]
pub fn block_0x0021a8f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2205940u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2205944u32);
    emu.apc_no_count(1usize, 2205944u32, 0u32, 2205948u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2205952u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(508u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021a900(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2205972u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a914));
    } else {
        emu.pc = 2205956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a904));
    }
}
#[inline(always)]
pub fn block_0x0021a904(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 9usize, 4u32, 2205960u32)?;
    emu.sw_no_count(11usize, 2usize, 20u32, 2205964u32)?;
    emu.sw_no_count(10usize, 2usize, 28u32, 2205968u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2205972u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2205972u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021a914));
}
#[inline(always)]
pub fn block_0x0021a914(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 24u32, 2205976u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2205980u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2205984u32);
    emu.adi_no_count(13usize, 2usize, 20u32, 2205988u32);
    emu.adi_no_count(12usize, 18usize, 0u32, 2205992u32);
    emu.apc_no_count(1usize, 2205992u32, 0u32, 2205996u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2206000u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(276u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021a930(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2206004u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2206044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a95c));
    } else {
        emu.pc = 2206008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a938));
    }
}
#[inline]
pub fn block_0x0021a938(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2206012u32)?;
    emu.sw_no_count(18usize, 9usize, 0u32, 2206016u32)?;
    emu.sw_no_count(10usize, 9usize, 4u32, 2206020u32)?;
    emu.lw_no_count(1usize, 2usize, 44u32, 2206024u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2206028u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2206032u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2206036u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2206040u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2206044u32;
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
pub fn block_0x0021a95c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2206048u32)?;
    emu.lw_no_count(11usize, 2usize, 16u32, 2206052u32)?;
    emu.adi_no_count(12usize, 8usize, 0u32, 2206056u32);
    emu.apc_no_count(1usize, 2206056u32, 0u32, 2206060u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2206064u32;
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
pub fn block_0x0021a970(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2206068u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2206072u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2206076u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2206080u32)?;
    emu.adr_no_count(9usize, 11usize, 12usize, 2206084u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a < b {
        emu.pc = 2206116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a9a4));
    } else {
        emu.pc = 2206088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a988));
    }
}
#[inline(always)]
pub fn block_0x0021a988(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 10usize, 0u32, 2206092u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2206096u32)?;
    emu.sli_no_count(11usize, 10usize, 1u32, 2206100u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2206136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a9b8));
    } else {
        emu.pc = 2206104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a998));
    }
}
#[inline(always)]
pub fn block_0x0021a998(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 8u32, 2206108u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2206148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a9c4));
    } else {
        emu.pc = 2206112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a9a0));
    }
}
#[inline(always)]
pub fn block_0x0021a9a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2206156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a9cc));
    } else {
        emu.pc = 2206116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a9a4));
    }
}
#[inline(always)]
pub fn block_0x0021a9a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2206120u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2206124u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965776u32, 2206128u32);
    emu.apc_no_count(1usize, 2206128u32, 0u32, 2206132u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2206136u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(324u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021a9b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 11usize, 0u32, 2206140u32);
    emu.adi_no_count(11usize, 0usize, 8u32, 2206144u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2206112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a9a0));
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
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 8u32, 2206152u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2206116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a9a4));
    } else {
        emu.pc = 2206156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a9cc));
    }
}
#[inline(always)]
pub fn block_0x0021a9cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2206176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a9e0));
    } else {
        emu.pc = 2206160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a9d0));
    }
}
#[inline(always)]
pub fn block_0x0021a9d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 8usize, 4u32, 2206164u32)?;
    emu.sw_no_count(11usize, 2usize, 24u32, 2206168u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2206172u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2206176u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2206176u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021a9e0));
}
#[inline(always)]
pub fn block_0x0021a9e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 28u32, 2206180u32)?;
    emu.adi_no_count(10usize, 2usize, 12u32, 2206184u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2206188u32);
    emu.adi_no_count(13usize, 2usize, 24u32, 2206192u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2206196u32);
    emu.apc_no_count(1usize, 2206196u32, 0u32, 2206200u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2206204u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(72u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021a9fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2206208u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2206244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021aa24));
    } else {
        emu.pc = 2206212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021aa04));
    }
}
#[inline(always)]
pub fn block_0x0021aa04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 16u32, 2206216u32)?;
    emu.sw_no_count(9usize, 8usize, 0u32, 2206220u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2206224u32)?;
    emu.lw_no_count(1usize, 2usize, 44u32, 2206228u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2206232u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2206236u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2206240u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2206244u32;
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
pub fn block_0x0021aa24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 16u32, 2206248u32)?;
    emu.lw_no_count(11usize, 2usize, 20u32, 2206252u32)?;
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2206256u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965776u32, 2206260u32);
    emu.apc_no_count(1usize, 2206260u32, 0u32, 2206264u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2206268u32;
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
