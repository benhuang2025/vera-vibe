pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2221876u32;
pub const PC_MAX: u32 = 2223408u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 74usize] = [
        block_0x0021e734,
        block_0x0021e794,
        block_0x0021e7f4,
        block_0x0021e854,
        block_0x0021e878,
        block_0x0021e88c,
        block_0x0021e890,
        block_0x0021e894,
        block_0x0021e898,
        block_0x0021e89c,
        block_0x0021e8b0,
        block_0x0021e8c4,
        block_0x0021e8dc,
        block_0x0021e8f4,
        block_0x0021e8f8,
        block_0x0021e914,
        block_0x0021e924,
        block_0x0021e93c,
        block_0x0021e940,
        block_0x0021e94c,
        block_0x0021e950,
        block_0x0021e95c,
        block_0x0021e960,
        block_0x0021e974,
        block_0x0021e988,
        block_0x0021e9a0,
        block_0x0021e9b8,
        block_0x0021e9bc,
        block_0x0021e9fc,
        block_0x0021ea10,
        block_0x0021ea20,
        block_0x0021ea28,
        block_0x0021ea34,
        block_0x0021ea38,
        block_0x0021ea44,
        block_0x0021ea4c,
        block_0x0021ea60,
        block_0x0021ea74,
        block_0x0021ea94,
        block_0x0021ea98,
        block_0x0021eacc,
        block_0x0021eae4,
        block_0x0021eb00,
        block_0x0021eb1c,
        block_0x0021eb20,
        block_0x0021eb38,
        block_0x0021eb54,
        block_0x0021eb70,
        block_0x0021eb74,
        block_0x0021eb88,
        block_0x0021eb8c,
        block_0x0021eba4,
        block_0x0021ebb8,
        block_0x0021ebc8,
        block_0x0021ebd4,
        block_0x0021ebdc,
        block_0x0021ec00,
        block_0x0021ec14,
        block_0x0021ec6c,
        block_0x0021ec70,
        block_0x0021ec78,
        block_0x0021ec7c,
        block_0x0021ec80,
        block_0x0021ec94,
        block_0x0021ecac,
        block_0x0021ecc0,
        block_0x0021ecd0,
        block_0x0021ecdc,
        block_0x0021ece4,
        block_0x0021ed08,
        block_0x0021ed0c,
        block_0x0021ed18,
        block_0x0021ed28,
        block_0x0021ed30,
    ];
    const IDX: [u16; 384usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 3u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 4u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 5u16, 0u16, 0u16, 0u16, 0u16, 6u16, 7u16, 8u16, 9u16, 10u16,
        0u16, 0u16, 0u16, 0u16, 11u16, 0u16, 0u16, 0u16, 0u16, 12u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 13u16, 0u16, 0u16, 0u16, 0u16, 0u16, 14u16, 15u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 16u16, 0u16, 0u16, 0u16, 17u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        18u16, 19u16, 0u16, 0u16, 20u16, 21u16, 0u16, 0u16, 22u16, 23u16, 0u16, 0u16,
        0u16, 0u16, 24u16, 0u16, 0u16, 0u16, 0u16, 25u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        26u16, 0u16, 0u16, 0u16, 0u16, 0u16, 27u16, 28u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 29u16, 0u16, 0u16,
        0u16, 0u16, 30u16, 0u16, 0u16, 0u16, 31u16, 0u16, 32u16, 0u16, 0u16, 33u16,
        34u16, 0u16, 0u16, 35u16, 0u16, 36u16, 0u16, 0u16, 0u16, 0u16, 37u16, 0u16, 0u16,
        0u16, 0u16, 38u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 39u16, 40u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 41u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 42u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 43u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 44u16, 45u16, 0u16, 0u16, 0u16, 0u16, 0u16, 46u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 47u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        48u16, 49u16, 0u16, 0u16, 0u16, 0u16, 50u16, 51u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        52u16, 0u16, 0u16, 0u16, 0u16, 53u16, 0u16, 0u16, 0u16, 54u16, 0u16, 0u16, 55u16,
        0u16, 56u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 57u16, 0u16, 0u16,
        0u16, 0u16, 58u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 59u16, 60u16,
        0u16, 61u16, 62u16, 63u16, 0u16, 0u16, 0u16, 0u16, 64u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 65u16, 0u16, 0u16, 0u16, 0u16, 66u16, 0u16, 0u16, 0u16, 67u16, 0u16, 0u16,
        68u16, 0u16, 69u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 70u16, 71u16,
        0u16, 0u16, 72u16, 0u16, 0u16, 0u16, 73u16, 0u16, 74u16,
    ];
    if pc < 2221876u32 || pc > 2223408u32 {
        return None;
    }
    let word_offset = ((pc - 2221876u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x0021e734(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2221880u32);
    emu.sw_no_count(10usize, 2usize, 0u32, 2221884u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2221888u32)?;
    emu.adi_no_count(10usize, 2usize, 0u32, 2221892u32);
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2221896u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966756u32, 2221900u32);
    emu.adi_no_count(13usize, 2usize, 4u32, 2221904u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2221908u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 792u32, 2221912u32);
    emu.adi_no_count(15usize, 0usize, 2u32, 2221916u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2221920u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2221924u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2221928u32)?;
    emu.sw_no_count(13usize, 2usize, 40u32, 2221932u32)?;
    emu.sw_no_count(11usize, 2usize, 44u32, 2221936u32)?;
    emu.adi_no_count(10usize, 2usize, 32u32, 2221940u32);
    emu.sw_no_count(14usize, 2usize, 8u32, 2221944u32)?;
    emu.sw_no_count(15usize, 2usize, 12u32, 2221948u32)?;
    emu.sw_no_count(10usize, 2usize, 16u32, 2221952u32)?;
    emu.sw_no_count(15usize, 2usize, 20u32, 2221956u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2221960u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2221964u32);
    emu.apc_no_count(1usize, 2221964u32, 4294955008u32, 2221968u32);
    emu.add_memory_rw_events(24usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2221972u32;
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
#[inline]
pub fn block_0x0021e794(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2221976u32);
    emu.sw_no_count(10usize, 2usize, 0u32, 2221980u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2221984u32)?;
    emu.adi_no_count(10usize, 2usize, 0u32, 2221988u32);
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2221992u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966756u32, 2221996u32);
    emu.adi_no_count(13usize, 2usize, 4u32, 2222000u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2222004u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 808u32, 2222008u32);
    emu.adi_no_count(15usize, 0usize, 2u32, 2222012u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2222016u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2222020u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2222024u32)?;
    emu.sw_no_count(13usize, 2usize, 40u32, 2222028u32)?;
    emu.sw_no_count(11usize, 2usize, 44u32, 2222032u32)?;
    emu.adi_no_count(10usize, 2usize, 32u32, 2222036u32);
    emu.sw_no_count(14usize, 2usize, 8u32, 2222040u32)?;
    emu.sw_no_count(15usize, 2usize, 12u32, 2222044u32)?;
    emu.sw_no_count(10usize, 2usize, 16u32, 2222048u32)?;
    emu.sw_no_count(15usize, 2usize, 20u32, 2222052u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2222056u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2222060u32);
    emu.apc_no_count(1usize, 2222060u32, 4294955008u32, 2222064u32);
    emu.add_memory_rw_events(24usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2222068u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966572u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021e7f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2222072u32);
    emu.sw_no_count(10usize, 2usize, 0u32, 2222076u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2222080u32)?;
    emu.adi_no_count(10usize, 2usize, 0u32, 2222084u32);
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2222088u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966756u32, 2222092u32);
    emu.adi_no_count(13usize, 2usize, 4u32, 2222096u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2222100u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 860u32, 2222104u32);
    emu.adi_no_count(15usize, 0usize, 2u32, 2222108u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2222112u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2222116u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2222120u32)?;
    emu.sw_no_count(13usize, 2usize, 40u32, 2222124u32)?;
    emu.sw_no_count(11usize, 2usize, 44u32, 2222128u32)?;
    emu.adi_no_count(10usize, 2usize, 32u32, 2222132u32);
    emu.sw_no_count(14usize, 2usize, 8u32, 2222136u32)?;
    emu.sw_no_count(15usize, 2usize, 12u32, 2222140u32)?;
    emu.sw_no_count(10usize, 2usize, 16u32, 2222144u32)?;
    emu.sw_no_count(15usize, 2usize, 20u32, 2222148u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2222152u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2222156u32);
    emu.apc_no_count(1usize, 2222156u32, 4294955008u32, 2222160u32);
    emu.add_memory_rw_events(24usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2222164u32;
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
#[inline]
pub fn block_0x0021e854(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2222168u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2222172u32)?;
    emu.adi_no_count(14usize, 13usize, 0u32, 2222176u32);
    emu.adi_no_count(13usize, 12usize, 0u32, 2222180u32);
    emu.adi_no_count(12usize, 11usize, 0u32, 2222184u32);
    emu.adi_no_count(11usize, 10usize, 0u32, 2222188u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2222192u32);
    emu.apc_no_count(1usize, 2222192u32, 0u32, 2222196u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2222200u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(28u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021e878(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2222204u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2222208u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2222212u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2222216u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2222220u32;
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
pub fn block_0x0021e88c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2222396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e93c));
    } else {
        emu.pc = 2222224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e890));
    }
}
#[inline(always)]
pub fn block_0x0021e890(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a != b {
        emu.pc = 2222396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e93c));
    } else {
        emu.pc = 2222228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e894));
    }
}
#[inline(always)]
pub fn block_0x0021e894(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2222708u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ea74));
    } else {
        emu.pc = 2222232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e898));
    }
}
#[inline(always)]
pub fn block_0x0021e898(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a >= b {
        emu.pc = 2222740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ea94));
    } else {
        emu.pc = 2222236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e89c));
    }
}
#[inline(always)]
pub fn block_0x0021e89c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(17usize, 13usize, 16u32, 2222240u32);
    emu.sltru_no_count(15usize, 17usize, 12usize, 2222244u32);
    emu.xri_no_count(16usize, 15usize, 1u32, 2222248u32);
    emu.adi_no_count(15usize, 13usize, 0u32, 2222252u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2222880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eb20));
    } else {
        emu.pc = 2222256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e8b0));
    }
}
#[inline(always)]
pub fn block_0x0021e8b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(16usize, 16usize, 4u32, 2222260u32);
    emu.sri_no_count(5usize, 15usize, 8u32, 2222264u32);
    emu.sltru_no_count(17usize, 5usize, 12usize, 2222268u32);
    emu.xri_no_count(17usize, 17usize, 1u32, 2222272u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a >= b {
        emu.pc = 2222904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eb38));
    } else {
        emu.pc = 2222276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e8c4));
    }
}
#[inline(always)]
pub fn block_0x0021e8c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(5usize, 17usize, 3u32, 2222280u32);
    emu.sri_no_count(17usize, 15usize, 4u32, 2222284u32);
    emu.orr_no_count(16usize, 5usize, 16usize, 2222288u32);
    emu.sltru_no_count(5usize, 17usize, 12usize, 2222292u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2222296u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2222932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eb54));
    } else {
        emu.pc = 2222300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e8dc));
    }
}
#[inline(always)]
pub fn block_0x0021e8dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(5usize, 5usize, 2u32, 2222304u32);
    emu.sri_no_count(17usize, 15usize, 2u32, 2222308u32);
    emu.orr_no_count(16usize, 16usize, 5usize, 2222312u32);
    emu.sltru_no_count(5usize, 17usize, 12usize, 2222316u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2222320u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a < b {
        emu.pc = 2222328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e8f8));
    } else {
        emu.pc = 2222324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e8f4));
    }
}
#[inline(always)]
pub fn block_0x0021e8f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 17usize, 0u32, 2222328u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2222328u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e8f8));
}
#[inline(always)]
pub fn block_0x0021e8f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(5usize, 5usize, 1u32, 2222332u32);
    emu.sri_no_count(15usize, 15usize, 1u32, 2222336u32);
    emu.sltru_no_count(15usize, 15usize, 12usize, 2222340u32);
    emu.xri_no_count(15usize, 15usize, 1u32, 2222344u32);
    emu.orr_no_count(15usize, 5usize, 15usize, 2222348u32);
    emu.orr_no_count(15usize, 16usize, 15usize, 2222352u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2222964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eb74));
    } else {
        emu.pc = 2222356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e914));
    }
}
#[inline(always)]
pub fn block_0x0021e914(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 31u32, 2222360u32);
    emu.adi_no_count(5usize, 16usize, 4294967264u32, 2222364u32);
    emu.slr_no_count(17usize, 13usize, 16usize, 2222368u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2222984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eb88));
    } else {
        emu.pc = 2222372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e924));
    }
}
#[inline(always)]
pub fn block_0x0021e924(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.slr_no_count(14usize, 14usize, 16usize, 2222376u32);
    emu.xri_no_count(15usize, 16usize, 4294967295u32, 2222380u32);
    emu.sri_no_count(6usize, 13usize, 1u32, 2222384u32);
    emu.srr_no_count(15usize, 6usize, 15usize, 2222388u32);
    emu.orr_no_count(15usize, 14usize, 15usize, 2222392u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2222396u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2222988u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021eb8c));
}
#[inline(always)]
pub fn block_0x0021e93c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2222416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e950));
    } else {
        emu.pc = 2222400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e940));
    }
}
#[inline(always)]
pub fn block_0x0021e940(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 12usize, 14usize, 2222404u32);
    emu.adi_no_count(17usize, 0usize, 0u32, 2222408u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2222428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e95c));
    } else {
        emu.pc = 2222412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e94c));
    }
}
#[inline(always)]
pub fn block_0x0021e94c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2222416u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2222668u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ea4c));
}
#[inline(always)]
pub fn block_0x0021e950(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 11usize, 13usize, 2222420u32);
    emu.adi_no_count(17usize, 0usize, 0u32, 2222424u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2222668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ea4c));
    } else {
        emu.pc = 2222428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e95c));
    }
}
#[inline(always)]
pub fn block_0x0021e95c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2222668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ea4c));
    } else {
        emu.pc = 2222432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e960));
    }
}
#[inline(always)]
pub fn block_0x0021e960(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(17usize, 12usize, 16u32, 2222436u32);
    emu.sltru_no_count(15usize, 17usize, 14usize, 2222440u32);
    emu.xri_no_count(16usize, 15usize, 1u32, 2222444u32);
    emu.adi_no_count(15usize, 12usize, 0u32, 2222448u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2222796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eacc));
    } else {
        emu.pc = 2222452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e974));
    }
}
#[inline(always)]
pub fn block_0x0021e974(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(16usize, 16usize, 4u32, 2222456u32);
    emu.sri_no_count(5usize, 15usize, 8u32, 2222460u32);
    emu.sltru_no_count(17usize, 5usize, 14usize, 2222464u32);
    emu.xri_no_count(17usize, 17usize, 1u32, 2222468u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a >= b {
        emu.pc = 2222820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eae4));
    } else {
        emu.pc = 2222472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e988));
    }
}
#[inline(always)]
pub fn block_0x0021e988(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(5usize, 17usize, 3u32, 2222476u32);
    emu.sri_no_count(17usize, 15usize, 4u32, 2222480u32);
    emu.orr_no_count(16usize, 5usize, 16usize, 2222484u32);
    emu.sltru_no_count(5usize, 17usize, 14usize, 2222488u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2222492u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2222848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eb00));
    } else {
        emu.pc = 2222496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e9a0));
    }
}
#[inline(always)]
pub fn block_0x0021e9a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(5usize, 5usize, 2u32, 2222500u32);
    emu.sri_no_count(17usize, 15usize, 2u32, 2222504u32);
    emu.orr_no_count(16usize, 16usize, 5usize, 2222508u32);
    emu.sltru_no_count(5usize, 17usize, 14usize, 2222512u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2222516u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a < b {
        emu.pc = 2222524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e9bc));
    } else {
        emu.pc = 2222520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e9b8));
    }
}
#[inline(always)]
pub fn block_0x0021e9b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 17usize, 0u32, 2222524u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2222524u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e9bc));
}
#[inline]
pub fn block_0x0021e9bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 0usize, 0u32, 2222528u32);
    emu.sli_no_count(5usize, 5usize, 1u32, 2222532u32);
    emu.sri_no_count(15usize, 15usize, 1u32, 2222536u32);
    emu.sltru_no_count(15usize, 15usize, 14usize, 2222540u32);
    emu.xri_no_count(15usize, 15usize, 1u32, 2222544u32);
    emu.orr_no_count(15usize, 5usize, 15usize, 2222548u32);
    emu.sri_no_count(5usize, 13usize, 1u32, 2222552u32);
    emu.orr_no_count(16usize, 16usize, 15usize, 2222556u32);
    emu.xri_no_count(15usize, 16usize, 31u32, 2222560u32);
    emu.srr_no_count(15usize, 5usize, 15usize, 2222564u32);
    emu.slr_no_count(5usize, 14usize, 16usize, 2222568u32);
    emu.orr_no_count(15usize, 5usize, 15usize, 2222572u32);
    emu.adi_no_count(5usize, 0usize, 1u32, 2222576u32);
    emu.slr_no_count(5usize, 5usize, 16usize, 2222580u32);
    emu.slr_no_count(6usize, 13usize, 16usize, 2222584u32);
    emu.add_memory_rw_events(16usize);
    let return_addr = 2222588u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2222608u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ea10));
}
#[inline(always)]
pub fn block_0x0021e9fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(16usize, 6usize, 1u32, 2222592u32);
    emu.sli_no_count(6usize, 15usize, 31u32, 2222596u32);
    emu.sri_no_count(15usize, 15usize, 1u32, 2222600u32);
    emu.orr_no_count(6usize, 16usize, 6usize, 2222604u32);
    emu.sri_no_count(5usize, 5usize, 1u32, 2222608u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2222608u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ea10));
}
#[inline(always)]
pub fn block_0x0021ea10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 11usize, 6usize, 2222612u32);
    emu.sbr_no_count(7usize, 12usize, 15usize, 2222616u32);
    emu.sbr_no_count(16usize, 7usize, 16usize, 2222620u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2222588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e9fc));
    } else {
        emu.pc = 2222624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ea20));
    }
}
#[inline(always)]
pub fn block_0x0021ea20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 11usize, 6usize, 2222628u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2222648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ea38));
    } else {
        emu.pc = 2222632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ea28));
    }
}
#[inline(always)]
pub fn block_0x0021ea28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 16usize, 14usize, 2222636u32);
    emu.orr_no_count(17usize, 5usize, 17usize, 2222640u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2222660u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ea44));
    } else {
        emu.pc = 2222644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ea34));
    }
}
#[inline(always)]
pub fn block_0x0021ea34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2222648u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2222688u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ea60));
}
#[inline(always)]
pub fn block_0x0021ea38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 11usize, 13usize, 2222652u32);
    emu.orr_no_count(17usize, 5usize, 17usize, 2222656u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2222688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ea60));
    } else {
        emu.pc = 2222660u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ea44));
    }
}
#[inline(always)]
pub fn block_0x0021ea44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 16usize, 0u32, 2222664u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2222668u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2222588u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e9fc));
}
#[inline(always)]
pub fn block_0x0021ea4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(17usize, 10usize, 0u32, 2222672u32)?;
    emu.sw_no_count(17usize, 10usize, 4u32, 2222676u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2222680u32)?;
    emu.sw_no_count(12usize, 10usize, 12u32, 2222684u32)?;
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2222688u32;
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
pub fn block_0x0021ea60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(17usize, 10usize, 0u32, 2222692u32)?;
    emu.sw_no_count(0usize, 10usize, 4u32, 2222696u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2222700u32)?;
    emu.sw_no_count(16usize, 10usize, 12u32, 2222704u32)?;
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2222708u32;
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
pub fn block_0x0021ea74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.divu_no_count(17usize, 11usize, 13usize, 2222712u32);
    emu.mul_no_count(12usize, 17usize, 13usize, 2222716u32);
    emu.sbr_no_count(11usize, 11usize, 12usize, 2222720u32);
    emu.sw_no_count(17usize, 10usize, 0u32, 2222724u32)?;
    emu.sw_no_count(0usize, 10usize, 4u32, 2222728u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2222732u32)?;
    emu.sw_no_count(0usize, 10usize, 12u32, 2222736u32)?;
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2222740u32;
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
pub fn block_0x0021ea94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2223104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ec00));
    } else {
        emu.pc = 2222744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ea98));
    }
}
#[inline]
pub fn block_0x0021ea98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.divu_no_count(13usize, 11usize, 12usize, 2222748u32);
    emu.mul_no_count(12usize, 13usize, 12usize, 2222752u32);
    emu.sltru_no_count(15usize, 0usize, 13usize, 2222756u32);
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2222760u32);
    emu.sbr_no_count(11usize, 11usize, 12usize, 2222764u32);
    emu.adi_no_count(17usize, 13usize, 1u32, 2222768u32);
    emu.sltiu_no_count(12usize, 17usize, 1u32, 2222772u32);
    emu.adr_no_count(15usize, 15usize, 12usize, 2222776u32);
    emu.sw_no_count(17usize, 10usize, 0u32, 2222780u32)?;
    emu.sw_no_count(15usize, 10usize, 4u32, 2222784u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2222788u32)?;
    emu.sw_no_count(0usize, 10usize, 12u32, 2222792u32)?;
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2222796u32;
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
pub fn block_0x0021eacc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 17usize, 0u32, 2222800u32);
    emu.sli_no_count(16usize, 16usize, 4u32, 2222804u32);
    emu.sri_no_count(5usize, 17usize, 8u32, 2222808u32);
    emu.sltru_no_count(17usize, 5usize, 14usize, 2222812u32);
    emu.xri_no_count(17usize, 17usize, 1u32, 2222816u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a < b {
        emu.pc = 2222472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e988));
    } else {
        emu.pc = 2222820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eae4));
    }
}
#[inline(always)]
pub fn block_0x0021eae4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 5usize, 0u32, 2222824u32);
    emu.sli_no_count(5usize, 17usize, 3u32, 2222828u32);
    emu.sri_no_count(17usize, 15usize, 4u32, 2222832u32);
    emu.orr_no_count(16usize, 5usize, 16usize, 2222836u32);
    emu.sltru_no_count(5usize, 17usize, 14usize, 2222840u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2222844u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a < b {
        emu.pc = 2222496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e9a0));
    } else {
        emu.pc = 2222848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eb00));
    }
}
#[inline(always)]
pub fn block_0x0021eb00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 17usize, 0u32, 2222852u32);
    emu.sli_no_count(5usize, 5usize, 2u32, 2222856u32);
    emu.sri_no_count(17usize, 17usize, 2u32, 2222860u32);
    emu.orr_no_count(16usize, 16usize, 5usize, 2222864u32);
    emu.sltru_no_count(5usize, 17usize, 14usize, 2222868u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2222872u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2222520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e9b8));
    } else {
        emu.pc = 2222876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eb1c));
    }
}
#[inline(always)]
pub fn block_0x0021eb1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2222880u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2222524u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e9bc));
}
#[inline(always)]
pub fn block_0x0021eb20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 17usize, 0u32, 2222884u32);
    emu.sli_no_count(16usize, 16usize, 4u32, 2222888u32);
    emu.sri_no_count(5usize, 17usize, 8u32, 2222892u32);
    emu.sltru_no_count(17usize, 5usize, 12usize, 2222896u32);
    emu.xri_no_count(17usize, 17usize, 1u32, 2222900u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a < b {
        emu.pc = 2222276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e8c4));
    } else {
        emu.pc = 2222904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eb38));
    }
}
#[inline(always)]
pub fn block_0x0021eb38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 5usize, 0u32, 2222908u32);
    emu.sli_no_count(5usize, 17usize, 3u32, 2222912u32);
    emu.sri_no_count(17usize, 15usize, 4u32, 2222916u32);
    emu.orr_no_count(16usize, 5usize, 16usize, 2222920u32);
    emu.sltru_no_count(5usize, 17usize, 12usize, 2222924u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2222928u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a < b {
        emu.pc = 2222300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e8dc));
    } else {
        emu.pc = 2222932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eb54));
    }
}
#[inline(always)]
pub fn block_0x0021eb54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 17usize, 0u32, 2222936u32);
    emu.sli_no_count(5usize, 5usize, 2u32, 2222940u32);
    emu.sri_no_count(17usize, 17usize, 2u32, 2222944u32);
    emu.orr_no_count(16usize, 16usize, 5usize, 2222948u32);
    emu.sltru_no_count(5usize, 17usize, 12usize, 2222952u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2222956u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2222324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e8f4));
    } else {
        emu.pc = 2222960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eb70));
    }
}
#[inline(always)]
pub fn block_0x0021eb70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2222964u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2222328u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e8f8));
}
#[inline(always)]
pub fn block_0x0021eb74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 32u32, 2222968u32);
    emu.sbr_no_count(16usize, 16usize, 15usize, 2222972u32);
    emu.adi_no_count(5usize, 16usize, 4294967264u32, 2222976u32);
    emu.slr_no_count(17usize, 13usize, 16usize, 2222980u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2222372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e924));
    } else {
        emu.pc = 2222984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eb88));
    }
}
#[inline(always)]
pub fn block_0x0021eb88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 17usize, 0u32, 2222988u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2222988u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021eb8c));
}
#[inline(always)]
pub fn block_0x0021eb8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 0u32, 2222992u32);
    emu.sai_no_count(5usize, 5usize, 1055u32, 2222996u32);
    emu.anr_no_count(17usize, 5usize, 17usize, 2223000u32);
    emu.adi_no_count(5usize, 0usize, 1u32, 2223004u32);
    emu.slr_no_count(16usize, 5usize, 16usize, 2223008u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2223012u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2223032u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ebb8));
}
#[inline(always)]
pub fn block_0x0021eba4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(17usize, 17usize, 1u32, 2223016u32);
    emu.sli_no_count(5usize, 15usize, 31u32, 2223020u32);
    emu.sri_no_count(15usize, 15usize, 1u32, 2223024u32);
    emu.orr_no_count(17usize, 17usize, 5usize, 2223028u32);
    emu.sri_no_count(16usize, 16usize, 1u32, 2223032u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2223032u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ebb8));
}
#[inline(always)]
pub fn block_0x0021ebb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 11usize, 17usize, 2223036u32);
    emu.sbr_no_count(6usize, 12usize, 15usize, 2223040u32);
    emu.sbr_no_count(5usize, 6usize, 5usize, 2223044u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2223012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eba4));
    } else {
        emu.pc = 2223048u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ebc8));
    }
}
#[inline(always)]
pub fn block_0x0021ebc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 11usize, 17usize, 2223052u32);
    emu.orr_no_count(14usize, 16usize, 14usize, 2223056u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2223068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ebdc));
    } else {
        emu.pc = 2223060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ebd4));
    }
}
#[inline(always)]
pub fn block_0x0021ebd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 5usize, 0u32, 2223064u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2223068u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2223012u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021eba4));
}
#[inline]
pub fn block_0x0021ebdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.divu_no_count(12usize, 11usize, 13usize, 2223072u32);
    emu.mul_no_count(13usize, 12usize, 13usize, 2223076u32);
    emu.sbr_no_count(11usize, 11usize, 13usize, 2223080u32);
    emu.orr_no_count(17usize, 12usize, 14usize, 2223084u32);
    emu.sw_no_count(17usize, 10usize, 0u32, 2223088u32)?;
    emu.sw_no_count(0usize, 10usize, 4u32, 2223092u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2223096u32)?;
    emu.sw_no_count(0usize, 10usize, 12u32, 2223100u32)?;
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2223104u32;
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
pub fn block_0x0021ec00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(17usize, 13usize, 16u32, 2223108u32);
    emu.divu_no_count(15usize, 12usize, 13usize, 2223112u32);
    emu.mul_no_count(16usize, 15usize, 13usize, 2223116u32);
    emu.sbr_no_count(16usize, 12usize, 16usize, 2223120u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a != b {
        emu.pc = 2223212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ec6c));
    } else {
        emu.pc = 2223124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ec14));
    }
}
#[inline]
pub fn block_0x0021ec14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 11usize, 16u32, 2223128u32);
    emu.sli_no_count(16usize, 16usize, 16u32, 2223132u32);
    emu.sli_no_count(11usize, 11usize, 16u32, 2223136u32);
    emu.orr_no_count(14usize, 16usize, 12usize, 2223140u32);
    emu.sri_no_count(11usize, 11usize, 16u32, 2223144u32);
    emu.divu_no_count(14usize, 14usize, 13usize, 2223148u32);
    emu.mul_no_count(16usize, 14usize, 13usize, 2223152u32);
    emu.sbr_no_count(12usize, 12usize, 16usize, 2223156u32);
    emu.sli_no_count(16usize, 14usize, 16u32, 2223160u32);
    emu.sri_no_count(14usize, 14usize, 16u32, 2223164u32);
    emu.orr_no_count(15usize, 14usize, 15usize, 2223168u32);
    emu.sli_no_count(12usize, 12usize, 16u32, 2223172u32);
    emu.orr_no_count(11usize, 12usize, 11usize, 2223176u32);
    emu.divu_no_count(12usize, 11usize, 13usize, 2223180u32);
    emu.mul_no_count(13usize, 12usize, 13usize, 2223184u32);
    emu.sbr_no_count(11usize, 11usize, 13usize, 2223188u32);
    emu.orr_no_count(17usize, 16usize, 12usize, 2223192u32);
    emu.sw_no_count(17usize, 10usize, 0u32, 2223196u32)?;
    emu.sw_no_count(15usize, 10usize, 4u32, 2223200u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2223204u32)?;
    emu.sw_no_count(0usize, 10usize, 12u32, 2223208u32)?;
    emu.add_memory_rw_events(22usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2223212u32;
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
pub fn block_0x0021ec6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2223224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ec78));
    } else {
        emu.pc = 2223216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ec70));
    }
}
#[inline(always)]
pub fn block_0x0021ec70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 16usize, 14usize, 2223220u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2223224u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2223228u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ec7c));
}
#[inline(always)]
pub fn block_0x0021ec78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 11usize, 13usize, 2223228u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2223228u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ec7c));
}
#[inline(always)]
pub fn block_0x0021ec7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2223252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ec94));
    } else {
        emu.pc = 2223232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ec80));
    }
}
#[inline(always)]
pub fn block_0x0021ec80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 10usize, 0u32, 2223236u32)?;
    emu.sw_no_count(15usize, 10usize, 4u32, 2223240u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2223244u32)?;
    emu.sw_no_count(16usize, 10usize, 12u32, 2223248u32)?;
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2223252u32;
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
pub fn block_0x0021ec94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(17usize, 13usize, 1u32, 2223256u32);
    emu.sli_no_count(14usize, 14usize, 31u32, 2223260u32);
    emu.sli_no_count(5usize, 13usize, 31u32, 2223264u32);
    emu.orr_no_count(14usize, 14usize, 17usize, 2223268u32);
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2223272u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(6usize);
    let return_addr = 2223276u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2223296u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ecc0));
}
#[inline(always)]
pub fn block_0x0021ecac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(5usize, 5usize, 1u32, 2223280u32);
    emu.sli_no_count(6usize, 14usize, 31u32, 2223284u32);
    emu.sri_no_count(14usize, 14usize, 1u32, 2223288u32);
    emu.orr_no_count(5usize, 5usize, 6usize, 2223292u32);
    emu.sri_no_count(17usize, 17usize, 1u32, 2223296u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2223296u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ecc0));
}
#[inline(always)]
pub fn block_0x0021ecc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(6usize, 11usize, 5usize, 2223300u32);
    emu.sbr_no_count(7usize, 16usize, 14usize, 2223304u32);
    emu.sbr_no_count(6usize, 7usize, 6usize, 2223308u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(6usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2223276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ecac));
    } else {
        emu.pc = 2223312u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ecd0));
    }
}
#[inline(always)]
pub fn block_0x0021ecd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 11usize, 5usize, 2223316u32);
    emu.orr_no_count(12usize, 17usize, 12usize, 2223320u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2223332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ece4));
    } else {
        emu.pc = 2223324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ecdc));
    }
}
#[inline(always)]
pub fn block_0x0021ecdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 6usize, 0u32, 2223328u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2223332u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2223276u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ecac));
}
#[inline]
pub fn block_0x0021ece4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.divu_no_count(14usize, 11usize, 13usize, 2223336u32);
    emu.mul_no_count(13usize, 14usize, 13usize, 2223340u32);
    emu.sbr_no_count(11usize, 11usize, 13usize, 2223344u32);
    emu.orr_no_count(17usize, 14usize, 12usize, 2223348u32);
    emu.sw_no_count(17usize, 10usize, 0u32, 2223352u32)?;
    emu.sw_no_count(15usize, 10usize, 4u32, 2223356u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2223360u32)?;
    emu.sw_no_count(0usize, 10usize, 12u32, 2223364u32)?;
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2223368u32;
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
pub fn block_0x0021ed08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2223400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ed28));
    } else {
        emu.pc = 2223372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ed0c));
    }
}
#[inline(always)]
pub fn block_0x0021ed0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(13usize, 10usize, 0u32, 2223376u32);
    emu.lbu_no_count(14usize, 11usize, 0u32, 2223380u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2223408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ed30));
    } else {
        emu.pc = 2223384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ed18));
    }
}
#[inline(always)]
pub fn block_0x0021ed18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2223388u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2223392u32);
    emu.adi_no_count(10usize, 10usize, 1u32, 2223396u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2223372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ed0c));
    } else {
        emu.pc = 2223400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ed28));
    }
}
#[inline(always)]
pub fn block_0x0021ed28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2223404u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2223408u32;
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
pub fn block_0x0021ed30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 13usize, 14usize, 2223412u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2223416u32;
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
