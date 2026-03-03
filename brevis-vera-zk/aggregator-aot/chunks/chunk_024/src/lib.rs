pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2209560u32;
pub const PC_MAX: u32 = 2211716u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 117usize] = [
        block_0x0021b718,
        block_0x0021b720,
        block_0x0021b734,
        block_0x0021b748,
        block_0x0021b750,
        block_0x0021b75c,
        block_0x0021b764,
        block_0x0021b774,
        block_0x0021b77c,
        block_0x0021b784,
        block_0x0021b78c,
        block_0x0021b798,
        block_0x0021b79c,
        block_0x0021b7a0,
        block_0x0021b7a4,
        block_0x0021b7a8,
        block_0x0021b7b8,
        block_0x0021b7bc,
        block_0x0021b7c4,
        block_0x0021b7c8,
        block_0x0021b7cc,
        block_0x0021b7d8,
        block_0x0021b7e0,
        block_0x0021b7e4,
        block_0x0021b7f0,
        block_0x0021b800,
        block_0x0021b80c,
        block_0x0021b820,
        block_0x0021b838,
        block_0x0021b85c,
        block_0x0021b860,
        block_0x0021b868,
        block_0x0021b874,
        block_0x0021b87c,
        block_0x0021b890,
        block_0x0021b898,
        block_0x0021b8a0,
        block_0x0021b8ac,
        block_0x0021b948,
        block_0x0021b94c,
        block_0x0021b9b8,
        block_0x0021ba2c,
        block_0x0021bac4,
        block_0x0021bad8,
        block_0x0021baf4,
        block_0x0021bafc,
        block_0x0021bb04,
        block_0x0021bb08,
        block_0x0021bb10,
        block_0x0021bb18,
        block_0x0021bb28,
        block_0x0021bb2c,
        block_0x0021bb38,
        block_0x0021bb40,
        block_0x0021bb4c,
        block_0x0021bb50,
        block_0x0021bb5c,
        block_0x0021bb64,
        block_0x0021bb6c,
        block_0x0021bba0,
        block_0x0021bba8,
        block_0x0021bbac,
        block_0x0021bbb4,
        block_0x0021bbc0,
        block_0x0021bbc8,
        block_0x0021bbd0,
        block_0x0021bbd4,
        block_0x0021bbe4,
        block_0x0021bbec,
        block_0x0021bbf8,
        block_0x0021bbfc,
        block_0x0021bc00,
        block_0x0021bc0c,
        block_0x0021bc10,
        block_0x0021bc20,
        block_0x0021bc30,
        block_0x0021bc44,
        block_0x0021bc48,
        block_0x0021bc4c,
        block_0x0021bc50,
        block_0x0021bc68,
        block_0x0021bc84,
        block_0x0021bc88,
        block_0x0021bc8c,
        block_0x0021bc94,
        block_0x0021bc9c,
        block_0x0021bca0,
        block_0x0021bcdc,
        block_0x0021bd08,
        block_0x0021bd28,
        block_0x0021bd30,
        block_0x0021bd50,
        block_0x0021bd80,
        block_0x0021bdbc,
        block_0x0021bdf4,
        block_0x0021be10,
        block_0x0021be20,
        block_0x0021be2c,
        block_0x0021be30,
        block_0x0021be58,
        block_0x0021be68,
        block_0x0021beb8,
        block_0x0021bebc,
        block_0x0021bed4,
        block_0x0021bed8,
        block_0x0021bee8,
        block_0x0021beec,
        block_0x0021bf08,
        block_0x0021bf0c,
        block_0x0021bf14,
        block_0x0021bf28,
        block_0x0021bf30,
        block_0x0021bf48,
        block_0x0021bf50,
        block_0x0021bf6c,
        block_0x0021bf74,
        block_0x0021bf84,
    ];
    const IDX: [u16; 540usize] = [
        1u16, 0u16, 2u16, 0u16, 0u16, 0u16, 0u16, 3u16, 0u16, 0u16, 0u16, 0u16, 4u16,
        0u16, 5u16, 0u16, 0u16, 6u16, 0u16, 7u16, 0u16, 0u16, 0u16, 8u16, 0u16, 9u16,
        0u16, 10u16, 0u16, 11u16, 0u16, 0u16, 12u16, 13u16, 14u16, 15u16, 16u16, 0u16,
        0u16, 0u16, 17u16, 18u16, 0u16, 19u16, 20u16, 21u16, 0u16, 0u16, 22u16, 0u16,
        23u16, 24u16, 0u16, 0u16, 25u16, 0u16, 0u16, 0u16, 26u16, 0u16, 0u16, 27u16,
        0u16, 0u16, 0u16, 0u16, 28u16, 0u16, 0u16, 0u16, 0u16, 0u16, 29u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 30u16, 31u16, 0u16, 32u16, 0u16, 0u16, 33u16,
        0u16, 34u16, 0u16, 0u16, 0u16, 0u16, 35u16, 0u16, 36u16, 0u16, 37u16, 0u16, 0u16,
        38u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        39u16, 40u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 41u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 42u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 43u16, 0u16, 0u16, 0u16, 0u16, 44u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 45u16, 0u16, 46u16, 0u16, 47u16, 48u16, 0u16, 49u16, 0u16,
        50u16, 0u16, 0u16, 0u16, 51u16, 52u16, 0u16, 0u16, 53u16, 0u16, 54u16, 0u16,
        0u16, 55u16, 56u16, 0u16, 0u16, 57u16, 0u16, 58u16, 0u16, 59u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 60u16, 0u16, 61u16,
        62u16, 0u16, 63u16, 0u16, 0u16, 64u16, 0u16, 65u16, 0u16, 66u16, 67u16, 0u16,
        0u16, 0u16, 68u16, 0u16, 69u16, 0u16, 0u16, 70u16, 71u16, 72u16, 0u16, 0u16,
        73u16, 74u16, 0u16, 0u16, 0u16, 75u16, 0u16, 0u16, 0u16, 76u16, 0u16, 0u16, 0u16,
        0u16, 77u16, 78u16, 79u16, 80u16, 0u16, 0u16, 0u16, 0u16, 0u16, 81u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 82u16, 83u16, 84u16, 0u16, 85u16, 0u16, 86u16,
        87u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 88u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        89u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 90u16, 0u16, 91u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 92u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 93u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 94u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 95u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 96u16, 0u16, 0u16, 0u16, 97u16, 0u16, 0u16, 98u16, 99u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 100u16, 0u16, 0u16, 0u16, 101u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 102u16, 103u16, 0u16, 0u16, 0u16, 0u16, 0u16, 104u16,
        105u16, 0u16, 0u16, 0u16, 106u16, 107u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        108u16, 109u16, 0u16, 110u16, 0u16, 0u16, 0u16, 0u16, 111u16, 0u16, 112u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 113u16, 0u16, 114u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        115u16, 0u16, 116u16, 0u16, 0u16, 0u16, 117u16,
    ];
    if pc < 2209560u32 || pc > 2211716u32 {
        return None;
    }
    let word_offset = ((pc - 2209560u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x0021b718(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2209560u32, 0u32, 2209564u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2209568u32;
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
pub fn block_0x0021b720(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967184u32, 2209572u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2209576u32)?;
    emu.adi_no_count(15usize, 0usize, 257u32, 2209580u32);
    emu.sw_no_count(13usize, 2usize, 12u32, 2209584u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2209608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b748));
    } else {
        emu.pc = 2209588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b734));
    }
}
#[inline(always)]
pub fn block_0x0021b734(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 0u32, 2209592u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2209596u32)?;
    emu.sw_no_count(11usize, 2usize, 20u32, 2209600u32)?;
    emu.adi_no_count(15usize, 0usize, 1u32, 2209604u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2209608u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2209676u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b78c));
}
#[inline(always)]
pub fn block_0x0021b748(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 256u32, 2209612u32);
    emu.adi_no_count(16usize, 0usize, 4294967231u32, 2209616u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2209616u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b750));
}
#[inline(always)]
pub fn block_0x0021b750(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(17usize, 10usize, 15usize, 2209620u32);
    emu.lb_no_count(17usize, 17usize, 0u32, 2209624u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(16usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2209636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b764));
    } else {
        emu.pc = 2209628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b75c));
    }
}
#[inline(always)]
pub fn block_0x0021b75c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 15usize, 4294967295u32, 2209632u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2209616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b750));
    } else {
        emu.pc = 2209636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b764));
    }
}
#[inline(always)]
pub fn block_0x0021b764(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 16u32, 2209640u32)?;
    emu.sw_no_count(15usize, 2usize, 20u32, 2209644u32)?;
    emu.sltru_no_count(16usize, 15usize, 11usize, 2209648u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2209660u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b77c));
    } else {
        emu.pc = 2209652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b774));
    }
}
#[inline(always)]
pub fn block_0x0021b774(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 1u32, 2209656u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2209660u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2209668u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b784));
}
#[inline(always)]
pub fn block_0x0021b77c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2209664u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 1676u32, 2209668u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2209668u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b784));
}
#[inline(always)]
pub fn block_0x0021b784(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(16usize, 0usize, 16usize, 2209672u32);
    emu.ani_no_count(16usize, 16usize, 5u32, 2209676u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2209676u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b78c));
}
#[inline(always)]
pub fn block_0x0021b78c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(15usize, 2usize, 24u32, 2209680u32)?;
    emu.sw_no_count(16usize, 2usize, 28u32, 2209684u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2210124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b94c));
    } else {
        emu.pc = 2209688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b798));
    }
}
#[inline(always)]
pub fn block_0x0021b798(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2210120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b948));
    } else {
        emu.pc = 2209692u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b79c));
    }
}
#[inline(always)]
pub fn block_0x0021b79c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2210232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b9b8));
    } else {
        emu.pc = 2209696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b7a0));
    }
}
#[inline(always)]
pub fn block_0x0021b7a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2209724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b7bc));
    } else {
        emu.pc = 2209700u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b7a4));
    }
}
#[inline(always)]
pub fn block_0x0021b7a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a >= b {
        emu.pc = 2209724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b7bc));
    } else {
        emu.pc = 2209704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b7a8));
    }
}
#[inline(always)]
pub fn block_0x0021b7a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 10usize, 12usize, 2209708u32);
    emu.lb_no_count(15usize, 15usize, 0u32, 2209712u32);
    emu.adi_no_count(16usize, 0usize, 4294967231u32, 2209716u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(16usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2209724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b7bc));
    } else {
        emu.pc = 2209720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b7b8));
    }
}
#[inline(always)]
pub fn block_0x0021b7b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 12usize, 0u32, 2209724u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2209724u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b7bc));
}
#[inline(always)]
pub fn block_0x0021b7bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(13usize, 2usize, 32u32, 2209728u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2209764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b7e4));
    } else {
        emu.pc = 2209732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b7c4));
    }
}
#[inline(always)]
pub fn block_0x0021b7c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2209760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b7e0));
    } else {
        emu.pc = 2209736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b7c8));
    }
}
#[inline(always)]
pub fn block_0x0021b7c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 4294967231u32, 2209740u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2209740u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b7cc));
}
#[inline(always)]
pub fn block_0x0021b7cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 10usize, 13usize, 2209744u32);
    emu.lb_no_count(15usize, 15usize, 0u32, 2209748u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2209760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b7e0));
    } else {
        emu.pc = 2209752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b7d8));
    }
}
#[inline(always)]
pub fn block_0x0021b7d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2209756u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2209740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b7cc));
    } else {
        emu.pc = 2209760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b7e0));
    }
}
#[inline(always)]
pub fn block_0x0021b7e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2209776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b7f0));
    } else {
        emu.pc = 2209764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b7e4));
    }
}
#[inline(always)]
pub fn block_0x0021b7e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 14usize, 0u32, 2209768u32);
    emu.apc_no_count(1usize, 2209768u32, 4096u32, 2209772u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2209776u32;
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
pub fn block_0x0021b7f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 10usize, 13usize, 2209780u32);
    emu.lb_no_count(12usize, 10usize, 0u32, 2209784u32);
    emu.ani_no_count(11usize, 12usize, 255u32, 2209788u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2209804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b80c));
    } else {
        emu.pc = 2209792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b800));
    }
}
#[inline(always)]
pub fn block_0x0021b800(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 2usize, 36u32, 2209796u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2209800u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2209804u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2209964u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b8ac));
}
#[inline(always)]
pub fn block_0x0021b80c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(15usize, 10usize, 1u32, 2209808u32);
    emu.ani_no_count(12usize, 11usize, 31u32, 2209812u32);
    emu.adi_no_count(16usize, 0usize, 223u32, 2209816u32);
    emu.ani_no_count(15usize, 15usize, 63u32, 2209820u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a >= b {
        emu.pc = 2209888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b860));
    } else {
        emu.pc = 2209824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b820));
    }
}
#[inline(always)]
pub fn block_0x0021b820(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(16usize, 10usize, 2u32, 2209828u32);
    emu.sli_no_count(15usize, 15usize, 6u32, 2209832u32);
    emu.ani_no_count(16usize, 16usize, 63u32, 2209836u32);
    emu.adi_no_count(17usize, 0usize, 240u32, 2209840u32);
    emu.orr_no_count(15usize, 15usize, 16usize, 2209844u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2209916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b87c));
    } else {
        emu.pc = 2209848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b838));
    }
}
#[inline]
pub fn block_0x0021b838(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 10usize, 3u32, 2209852u32);
    emu.sli_no_count(12usize, 12usize, 29u32, 2209856u32);
    emu.sli_no_count(15usize, 15usize, 6u32, 2209860u32);
    emu.sri_no_count(12usize, 12usize, 11u32, 2209864u32);
    emu.ani_no_count(10usize, 10usize, 63u32, 2209868u32);
    emu.orr_no_count(10usize, 15usize, 10usize, 2209872u32);
    emu.orr_no_count(10usize, 10usize, 12usize, 2209876u32);
    let a = 0u32.wrapping_add(1114112u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2209880u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2209764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b7e4));
    } else {
        emu.pc = 2209884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b85c));
    }
}
#[inline(always)]
pub fn block_0x0021b85c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2209888u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2209896u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b868));
}
#[inline(always)]
pub fn block_0x0021b860(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 12usize, 6u32, 2209892u32);
    emu.orr_no_count(10usize, 10usize, 15usize, 2209896u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2209896u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b868));
}
#[inline(always)]
pub fn block_0x0021b868(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 128u32, 2209900u32);
    emu.sw_no_count(10usize, 2usize, 36u32, 2209904u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2209936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b890));
    } else {
        emu.pc = 2209908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b874));
    }
}
#[inline(always)]
pub fn block_0x0021b874(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2209912u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2209916u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2209964u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b8ac));
}
#[inline(always)]
pub fn block_0x0021b87c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 12usize, 12u32, 2209920u32);
    emu.orr_no_count(10usize, 15usize, 10usize, 2209924u32);
    emu.adi_no_count(11usize, 0usize, 128u32, 2209928u32);
    emu.sw_no_count(10usize, 2usize, 36u32, 2209932u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2209908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b874));
    } else {
        emu.pc = 2209936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b890));
    }
}
#[inline(always)]
pub fn block_0x0021b890(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 10usize, 11u32, 2209940u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2209952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b8a0));
    } else {
        emu.pc = 2209944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b898));
    }
}
#[inline(always)]
pub fn block_0x0021b898(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2209948u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2209952u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2209964u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b8ac));
}
#[inline(always)]
pub fn block_0x0021b8a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 10usize, 16u32, 2209956u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2209960u32);
    emu.adi_no_count(10usize, 10usize, 3u32, 2209964u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2209964u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b8ac));
}
#[inline(never)]
pub fn block_0x0021b8ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 39u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 10usize, 13usize, 2209968u32);
    emu.adi_no_count(11usize, 2usize, 32u32, 2209972u32);
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2209976u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966756u32, 2209980u32);
    emu.adi_no_count(15usize, 2usize, 36u32, 2209984u32);
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2209988u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 4294967028u32, 2209992u32);
    emu.adi_no_count(17usize, 2usize, 40u32, 2209996u32);
    let a = 0u32.wrapping_add(2211840u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2210000u32;
    emu.update_insn_clock();
    emu.adi_no_count(5usize, 5usize, 1136u32, 2210004u32);
    emu.adi_no_count(6usize, 2usize, 16u32, 2210008u32);
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2210012u32;
    emu.update_insn_clock();
    emu.adi_no_count(7usize, 7usize, 4294967232u32, 2210016u32);
    emu.adi_no_count(28usize, 2usize, 24u32, 2210020u32);
    emu.sw_no_count(13usize, 2usize, 40u32, 2210024u32)?;
    emu.sw_no_count(10usize, 2usize, 44u32, 2210028u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2210032u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1784u32, 2210036u32);
    emu.sw_no_count(11usize, 2usize, 72u32, 2210040u32)?;
    emu.sw_no_count(12usize, 2usize, 76u32, 2210044u32)?;
    emu.sw_no_count(15usize, 2usize, 80u32, 2210048u32)?;
    emu.sw_no_count(16usize, 2usize, 84u32, 2210052u32)?;
    emu.adi_no_count(11usize, 0usize, 5u32, 2210056u32);
    emu.sw_no_count(0usize, 2usize, 64u32, 2210060u32)?;
    emu.sw_no_count(17usize, 2usize, 88u32, 2210064u32)?;
    emu.sw_no_count(5usize, 2usize, 92u32, 2210068u32)?;
    emu.sw_no_count(6usize, 2usize, 96u32, 2210072u32)?;
    emu.sw_no_count(7usize, 2usize, 100u32, 2210076u32)?;
    emu.sw_no_count(28usize, 2usize, 104u32, 2210080u32)?;
    emu.sw_no_count(7usize, 2usize, 108u32, 2210084u32)?;
    emu.adi_no_count(12usize, 2usize, 72u32, 2210088u32);
    emu.sw_no_count(10usize, 2usize, 48u32, 2210092u32)?;
    emu.sw_no_count(11usize, 2usize, 52u32, 2210096u32)?;
    emu.sw_no_count(12usize, 2usize, 56u32, 2210100u32)?;
    emu.sw_no_count(11usize, 2usize, 60u32, 2210104u32)?;
    emu.adi_no_count(10usize, 2usize, 48u32, 2210108u32);
    emu.adi_no_count(11usize, 14usize, 0u32, 2210112u32);
    emu.apc_no_count(1usize, 2210112u32, 0u32, 2210116u32);
    emu.add_memory_rw_events(39usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2210120u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966232u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021b948(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 13usize, 0u32, 2210124u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2210124u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b94c));
}
#[inline(never)]
pub fn block_0x0021b94c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(12usize, 2usize, 40u32, 2210128u32)?;
    emu.adi_no_count(10usize, 2usize, 40u32, 2210132u32);
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2210136u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966756u32, 2210140u32);
    emu.adi_no_count(12usize, 2usize, 16u32, 2210144u32);
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2210148u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294967232u32, 2210152u32);
    emu.adi_no_count(15usize, 2usize, 24u32, 2210156u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2210160u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 1848u32, 2210164u32);
    emu.adi_no_count(17usize, 0usize, 3u32, 2210168u32);
    emu.sw_no_count(0usize, 2usize, 64u32, 2210172u32)?;
    emu.sw_no_count(10usize, 2usize, 72u32, 2210176u32)?;
    emu.sw_no_count(11usize, 2usize, 76u32, 2210180u32)?;
    emu.sw_no_count(12usize, 2usize, 80u32, 2210184u32)?;
    emu.sw_no_count(13usize, 2usize, 84u32, 2210188u32)?;
    emu.sw_no_count(15usize, 2usize, 88u32, 2210192u32)?;
    emu.sw_no_count(13usize, 2usize, 92u32, 2210196u32)?;
    emu.adi_no_count(10usize, 2usize, 72u32, 2210200u32);
    emu.sw_no_count(16usize, 2usize, 48u32, 2210204u32)?;
    emu.sw_no_count(17usize, 2usize, 52u32, 2210208u32)?;
    emu.sw_no_count(10usize, 2usize, 56u32, 2210212u32)?;
    emu.sw_no_count(17usize, 2usize, 60u32, 2210216u32)?;
    emu.adi_no_count(10usize, 2usize, 48u32, 2210220u32);
    emu.adi_no_count(11usize, 14usize, 0u32, 2210224u32);
    emu.apc_no_count(1usize, 2210224u32, 0u32, 2210228u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2210232u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966120u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0021b9b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 29u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 8u32, 2210236u32);
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2210240u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966756u32, 2210244u32);
    emu.adi_no_count(12usize, 2usize, 12u32, 2210248u32);
    emu.adi_no_count(13usize, 2usize, 16u32, 2210252u32);
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2210256u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 4294967232u32, 2210260u32);
    emu.adi_no_count(16usize, 2usize, 24u32, 2210264u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2210268u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 1696u32, 2210272u32);
    emu.sw_no_count(10usize, 2usize, 72u32, 2210276u32)?;
    emu.sw_no_count(11usize, 2usize, 76u32, 2210280u32)?;
    emu.sw_no_count(12usize, 2usize, 80u32, 2210284u32)?;
    emu.sw_no_count(11usize, 2usize, 84u32, 2210288u32)?;
    emu.adi_no_count(10usize, 0usize, 4u32, 2210292u32);
    emu.sw_no_count(0usize, 2usize, 64u32, 2210296u32)?;
    emu.sw_no_count(13usize, 2usize, 88u32, 2210300u32)?;
    emu.sw_no_count(15usize, 2usize, 92u32, 2210304u32)?;
    emu.sw_no_count(16usize, 2usize, 96u32, 2210308u32)?;
    emu.sw_no_count(15usize, 2usize, 100u32, 2210312u32)?;
    emu.adi_no_count(11usize, 2usize, 72u32, 2210316u32);
    emu.sw_no_count(17usize, 2usize, 48u32, 2210320u32)?;
    emu.sw_no_count(10usize, 2usize, 52u32, 2210324u32)?;
    emu.sw_no_count(11usize, 2usize, 56u32, 2210328u32)?;
    emu.sw_no_count(10usize, 2usize, 60u32, 2210332u32)?;
    emu.adi_no_count(10usize, 2usize, 48u32, 2210336u32);
    emu.adi_no_count(11usize, 14usize, 0u32, 2210340u32);
    emu.apc_no_count(1usize, 2210340u32, 0u32, 2210344u32);
    emu.add_memory_rw_events(29usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2210348u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966004u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0021ba2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 38u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967216u32, 2210352u32);
    emu.sw_no_count(1usize, 2usize, 76u32, 2210356u32)?;
    emu.sw_no_count(8usize, 2usize, 72u32, 2210360u32)?;
    emu.sw_no_count(9usize, 2usize, 68u32, 2210364u32)?;
    emu.sw_no_count(18usize, 2usize, 64u32, 2210368u32)?;
    emu.sw_no_count(19usize, 2usize, 60u32, 2210372u32)?;
    emu.sw_no_count(20usize, 2usize, 56u32, 2210376u32)?;
    emu.sw_no_count(21usize, 2usize, 52u32, 2210380u32)?;
    emu.sw_no_count(22usize, 2usize, 48u32, 2210384u32)?;
    emu.sw_no_count(23usize, 2usize, 44u32, 2210388u32)?;
    emu.sw_no_count(24usize, 2usize, 40u32, 2210392u32)?;
    emu.sw_no_count(25usize, 2usize, 36u32, 2210396u32)?;
    emu.sw_no_count(26usize, 2usize, 32u32, 2210400u32)?;
    emu.sw_no_count(27usize, 2usize, 28u32, 2210404u32)?;
    emu.adi_no_count(8usize, 12usize, 0u32, 2210408u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2210412u32);
    emu.adi_no_count(20usize, 0usize, 0u32, 2210416u32);
    emu.adi_no_count(21usize, 0usize, 0u32, 2210420u32);
    emu.adi_no_count(25usize, 0usize, 0u32, 2210424u32);
    let a = 0u32.wrapping_add(168431616u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2210428u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2210432u32;
    emu.update_insn_clock();
    emu.lw_no_count(13usize, 10usize, 0u32, 2210436u32)?;
    emu.sw_no_count(13usize, 2usize, 24u32, 2210440u32)?;
    emu.lw_no_count(13usize, 10usize, 4u32, 2210444u32)?;
    emu.sw_no_count(13usize, 2usize, 20u32, 2210448u32)?;
    emu.lw_no_count(22usize, 10usize, 8u32, 2210452u32)?;
    emu.adi_no_count(10usize, 9usize, 4294967295u32, 2210456u32);
    emu.sw_no_count(10usize, 2usize, 12u32, 2210460u32)?;
    emu.adi_no_count(10usize, 9usize, 4u32, 2210464u32);
    emu.sw_no_count(10usize, 2usize, 8u32, 2210468u32)?;
    emu.sbr_no_count(10usize, 0usize, 8usize, 2210472u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2210476u32)?;
    emu.adi_no_count(27usize, 11usize, 4294965770u32, 2210480u32);
    emu.adi_no_count(19usize, 12usize, 256u32, 2210484u32);
    emu.adi_no_count(24usize, 0usize, 10u32, 2210488u32);
    let a = 0u32.wrapping_add(2155905024u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2210492u32;
    emu.update_insn_clock();
    emu.adi_no_count(23usize, 10usize, 128u32, 2210496u32);
    emu.add_memory_rw_events(38usize);
    let return_addr = 2210500u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2210556u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bafc));
}
#[inline(always)]
pub fn block_0x0021bac4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2210504u32)?;
    emu.adr_no_count(10usize, 10usize, 26usize, 2210508u32);
    emu.lbu_no_count(10usize, 10usize, 0u32, 2210512u32);
    emu.adi_no_count(10usize, 10usize, 4294967286u32, 2210516u32);
    emu.sltiu_no_count(10usize, 10usize, 1u32, 2210520u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2210520u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bad8));
}
#[inline(always)]
pub fn block_0x0021bad8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(10usize, 22usize, 0u32, 2210524u32);
    emu.lw_no_count(10usize, 2usize, 20u32, 2210528u32)?;
    emu.lw_no_count(13usize, 10usize, 12u32, 2210532u32)?;
    emu.sbr_no_count(12usize, 26usize, 20usize, 2210536u32);
    emu.adr_no_count(11usize, 9usize, 20usize, 2210540u32);
    emu.lw_no_count(10usize, 2usize, 24u32, 2210544u32)?;
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2210548u32;
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
pub fn block_0x0021baf4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 18usize, 0u32, 2210552u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2210972u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bc9c));
    } else {
        emu.pc = 2210556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bafc));
    }
}
#[inline(always)]
pub fn block_0x0021bafc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 25usize, 1u32, 2210560u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2210964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bc94));
    } else {
        emu.pc = 2210564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bb04));
    }
}
#[inline(always)]
pub fn block_0x0021bb04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a >= b {
        emu.pc = 2210584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bb18));
    } else {
        emu.pc = 2210568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bb08));
    }
}
#[inline(always)]
pub fn block_0x0021bb08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 21usize, 0u32, 2210572u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2210576u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2210892u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bc4c));
}
#[inline(always)]
pub fn block_0x0021bb10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(21usize, 26usize, 0u32, 2210580u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a < b {
        emu.pc = 2210892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bc4c));
    } else {
        emu.pc = 2210584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bb18));
    }
}
#[inline(always)]
pub fn block_0x0021bb18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 8usize, 21usize, 2210588u32);
    emu.adr_no_count(10usize, 9usize, 21usize, 2210592u32);
    emu.adi_no_count(12usize, 0usize, 7u32, 2210596u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2210640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bb50));
    } else {
        emu.pc = 2210600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bb28));
    }
}
#[inline(always)]
pub fn block_0x0021bb28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2210888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bc48));
    } else {
        emu.pc = 2210604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bb2c));
    }
}
#[inline(always)]
pub fn block_0x0021bb2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2210608u32);
    emu.lw_no_count(12usize, 2usize, 16u32, 2210612u32)?;
    emu.adr_no_count(12usize, 12usize, 21usize, 2210616u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2210616u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bb38));
}
#[inline(always)]
pub fn block_0x0021bb38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(13usize, 10usize, 0u32, 2210620u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2210812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bbfc));
    } else {
        emu.pc = 2210624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bb40));
    }
}
#[inline(always)]
pub fn block_0x0021bb40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2210628u32);
    emu.adi_no_count(10usize, 10usize, 1u32, 2210632u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2210616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bb38));
    } else {
        emu.pc = 2210636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bb4c));
    }
}
#[inline(always)]
pub fn block_0x0021bb4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2210640u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2210888u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bc48));
}
#[inline(always)]
pub fn block_0x0021bb50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 3u32, 2210644u32);
    emu.ani_no_count(12usize, 12usize, 4294967292u32, 2210648u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2210732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bbac));
    } else {
        emu.pc = 2210652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bb5c));
    }
}
#[inline(always)]
pub fn block_0x0021bb5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2210656u32);
    emu.adi_no_count(13usize, 11usize, 4294967288u32, 2210660u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2210660u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bb64));
}
#[inline(always)]
pub fn block_0x0021bb64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 2usize, 8u32, 2210664u32)?;
    emu.adr_no_count(14usize, 14usize, 21usize, 2210668u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2210668u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bb6c));
}
#[inline]
pub fn block_0x0021bb6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 10usize, 12usize, 2210672u32);
    emu.adr_no_count(16usize, 14usize, 12usize, 2210676u32);
    emu.lw_no_count(15usize, 15usize, 0u32, 2210680u32)?;
    emu.lw_no_count(16usize, 16usize, 0u32, 2210684u32)?;
    emu.xrr_no_count(17usize, 15usize, 27usize, 2210688u32);
    emu.xrr_no_count(16usize, 16usize, 27usize, 2210692u32);
    emu.sbr_no_count(17usize, 19usize, 17usize, 2210696u32);
    emu.orr_no_count(15usize, 17usize, 15usize, 2210700u32);
    emu.sbr_no_count(17usize, 19usize, 16usize, 2210704u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2210708u32);
    emu.anr_no_count(15usize, 15usize, 16usize, 2210712u32);
    emu.anr_no_count(15usize, 15usize, 23usize, 2210716u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2210768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bbd0));
    } else {
        emu.pc = 2210720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bba0));
    }
}
#[inline(always)]
pub fn block_0x0021bba0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 8u32, 2210724u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2210668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bb6c));
    } else {
        emu.pc = 2210728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bba8));
    }
}
#[inline(always)]
pub fn block_0x0021bba8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2210732u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2210768u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bbd0));
}
#[inline(always)]
pub fn block_0x0021bbac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 0u32, 2210736u32);
    emu.sbr_no_count(12usize, 12usize, 10usize, 2210740u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2210740u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bbb4));
}
#[inline(always)]
pub fn block_0x0021bbb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(14usize, 10usize, 13usize, 2210744u32);
    emu.lbu_no_count(14usize, 14usize, 0u32, 2210748u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2210816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bc00));
    } else {
        emu.pc = 2210752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bbc0));
    }
}
#[inline(always)]
pub fn block_0x0021bbc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 1u32, 2210756u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2210740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bbb4));
    } else {
        emu.pc = 2210760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bbc8));
    }
}
#[inline(always)]
pub fn block_0x0021bbc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 11usize, 4294967288u32, 2210764u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2210660u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bb64));
    } else {
        emu.pc = 2210768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bbd0));
    }
}
#[inline(always)]
pub fn block_0x0021bbd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2210888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bc48));
    } else {
        emu.pc = 2210772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bbd4));
    }
}
#[inline(always)]
pub fn block_0x0021bbd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 10usize, 12usize, 2210776u32);
    emu.sbr_no_count(10usize, 0usize, 12usize, 2210780u32);
    emu.lw_no_count(12usize, 2usize, 16u32, 2210784u32)?;
    emu.adr_no_count(12usize, 12usize, 21usize, 2210788u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2210788u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bbe4));
}
#[inline(always)]
pub fn block_0x0021bbe4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(13usize, 11usize, 0u32, 2210792u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2210832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bc10));
    } else {
        emu.pc = 2210796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bbec));
    }
}
#[inline(always)]
pub fn block_0x0021bbec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2210800u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2210804u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2210788u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bbe4));
    } else {
        emu.pc = 2210808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bbf8));
    }
}
#[inline(always)]
pub fn block_0x0021bbf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2210812u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2210888u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bc48));
}
#[inline(always)]
pub fn block_0x0021bbfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(13usize, 0usize, 11usize, 2210816u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2210816u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bc00));
}
#[inline(always)]
pub fn block_0x0021bc00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 21usize, 13usize, 2210820u32);
    emu.adi_no_count(26usize, 10usize, 1u32, 2210824u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2210576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bb10));
    } else {
        emu.pc = 2210828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bc0c));
    }
}
#[inline(always)]
pub fn block_0x0021bc0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2210832u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2210848u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bc20));
}
#[inline(always)]
pub fn block_0x0021bc10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(13usize, 0usize, 10usize, 2210836u32);
    emu.adr_no_count(10usize, 21usize, 13usize, 2210840u32);
    emu.adi_no_count(26usize, 10usize, 1u32, 2210844u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2210576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bb10));
    } else {
        emu.pc = 2210848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bc20));
    }
}
#[inline(always)]
pub fn block_0x0021bc20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(21usize, 9usize, 21usize, 2210852u32);
    emu.adr_no_count(13usize, 21usize, 13usize, 2210856u32);
    emu.lbu_no_count(10usize, 13usize, 0u32, 2210860u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2210576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bb10));
    } else {
        emu.pc = 2210864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bc30));
    }
}
#[inline(always)]
pub fn block_0x0021bc30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 0usize, 0u32, 2210868u32);
    emu.adi_no_count(18usize, 26usize, 0u32, 2210872u32);
    emu.adi_no_count(21usize, 26usize, 0u32, 2210876u32);
    emu.lbu_no_count(10usize, 22usize, 0u32, 2210880u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2210952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bc88));
    } else {
        emu.pc = 2210884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bc44));
    }
}
#[inline(always)]
pub fn block_0x0021bc44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2210888u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2210920u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bc68));
}
#[inline(always)]
pub fn block_0x0021bc48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 8usize, 0u32, 2210892u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2210892u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bc4c));
}
#[inline(always)]
pub fn block_0x0021bc4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2210964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bc94));
    } else {
        emu.pc = 2210896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bc50));
    }
}
#[inline(always)]
pub fn block_0x0021bc50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 0usize, 1u32, 2210900u32);
    emu.adi_no_count(18usize, 20usize, 0u32, 2210904u32);
    emu.adi_no_count(21usize, 26usize, 0u32, 2210908u32);
    emu.adi_no_count(26usize, 8usize, 0u32, 2210912u32);
    emu.lbu_no_count(10usize, 22usize, 0u32, 2210916u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2210952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bc88));
    } else {
        emu.pc = 2210920u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bc68));
    }
}
#[inline(always)]
pub fn block_0x0021bc68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 20u32, 2210924u32)?;
    emu.lw_no_count(13usize, 10usize, 12u32, 2210928u32)?;
    emu.adi_no_count(12usize, 0usize, 4u32, 2210932u32);
    emu.lw_no_count(10usize, 2usize, 24u32, 2210936u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2210940u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966052u32, 2210944u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2210948u32;
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
pub fn block_0x0021bc84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2210972u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bc9c));
    } else {
        emu.pc = 2210952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bc88));
    }
}
#[inline(always)]
pub fn block_0x0021bc88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a != b {
        emu.pc = 2210500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bac4));
    } else {
        emu.pc = 2210956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bc8c));
    }
}
#[inline(always)]
pub fn block_0x0021bc8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2210960u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2210964u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2210520u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bad8));
}
#[inline(always)]
pub fn block_0x0021bc94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2210968u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2210972u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2210976u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bca0));
}
#[inline(always)]
pub fn block_0x0021bc9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2210976u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2210976u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bca0));
}
#[inline]
pub fn block_0x0021bca0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 76u32, 2210980u32)?;
    emu.lw_no_count(8usize, 2usize, 72u32, 2210984u32)?;
    emu.lw_no_count(9usize, 2usize, 68u32, 2210988u32)?;
    emu.lw_no_count(18usize, 2usize, 64u32, 2210992u32)?;
    emu.lw_no_count(19usize, 2usize, 60u32, 2210996u32)?;
    emu.lw_no_count(20usize, 2usize, 56u32, 2211000u32)?;
    emu.lw_no_count(21usize, 2usize, 52u32, 2211004u32)?;
    emu.lw_no_count(22usize, 2usize, 48u32, 2211008u32)?;
    emu.lw_no_count(23usize, 2usize, 44u32, 2211012u32)?;
    emu.lw_no_count(24usize, 2usize, 40u32, 2211016u32)?;
    emu.lw_no_count(25usize, 2usize, 36u32, 2211020u32)?;
    emu.lw_no_count(26usize, 2usize, 32u32, 2211024u32)?;
    emu.lw_no_count(27usize, 2usize, 28u32, 2211028u32)?;
    emu.adi_no_count(2usize, 2usize, 80u32, 2211032u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2211036u32;
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
pub fn block_0x0021bcdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2211040u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2211044u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2211048u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2211052u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2211056u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2211060u32)?;
    emu.lw_no_count(9usize, 10usize, 8u32, 2211064u32)?;
    emu.lbu_no_count(12usize, 9usize, 0u32, 2211068u32);
    emu.lw_no_count(8usize, 10usize, 0u32, 2211072u32)?;
    emu.lw_no_count(18usize, 10usize, 4u32, 2211076u32)?;
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2211152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bd50));
    } else {
        emu.pc = 2211080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bd08));
    }
}
#[inline(always)]
pub fn block_0x0021bd08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 18usize, 12u32, 2211084u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2211088u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966052u32, 2211092u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2211096u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2211100u32);
    emu.adi_no_count(19usize, 11usize, 0u32, 2211104u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2211108u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2211112u32;
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
pub fn block_0x0021bd28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 19usize, 0u32, 2211116u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2211152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bd50));
    } else {
        emu.pc = 2211120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bd30));
    }
}
#[inline(always)]
pub fn block_0x0021bd30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2211124u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2211128u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2211132u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2211136u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2211140u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2211144u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2211148u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2211152u32;
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
pub fn block_0x0021bd50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 11usize, 4294967286u32, 2211156u32);
    emu.sltiu_no_count(10usize, 10usize, 1u32, 2211160u32);
    emu.sb_no_count(10usize, 9usize, 0u32, 2211164u32);
    emu.lw_no_count(6usize, 18usize, 16u32, 2211168u32)?;
    emu.adi_no_count(10usize, 8usize, 0u32, 2211172u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2211176u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2211180u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2211184u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2211188u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2211192u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2211196u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2211200u32;
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
pub fn block_0x0021bd80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967216u32, 2211204u32);
    emu.sw_no_count(1usize, 2usize, 76u32, 2211208u32)?;
    emu.sw_no_count(8usize, 2usize, 72u32, 2211212u32)?;
    emu.sw_no_count(9usize, 2usize, 68u32, 2211216u32)?;
    emu.sw_no_count(18usize, 2usize, 64u32, 2211220u32)?;
    emu.sw_no_count(19usize, 2usize, 60u32, 2211224u32)?;
    emu.sw_no_count(20usize, 2usize, 56u32, 2211228u32)?;
    emu.sw_no_count(21usize, 2usize, 52u32, 2211232u32)?;
    emu.sw_no_count(22usize, 2usize, 48u32, 2211236u32)?;
    emu.sw_no_count(23usize, 2usize, 44u32, 2211240u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2211244u32);
    emu.lbu_no_count(10usize, 10usize, 4u32, 2211248u32);
    emu.adi_no_count(21usize, 0usize, 1u32, 2211252u32);
    emu.adi_no_count(20usize, 0usize, 1u32, 2211256u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2211316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bdf4));
    } else {
        emu.pc = 2211260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bdbc));
    }
}
#[inline]
pub fn block_0x0021bdbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(20usize, 8usize, 4u32, 2211264u32);
    emu.sb_no_count(21usize, 8usize, 5u32, 2211268u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2211272u32);
    emu.lw_no_count(1usize, 2usize, 76u32, 2211276u32)?;
    emu.lw_no_count(8usize, 2usize, 72u32, 2211280u32)?;
    emu.lw_no_count(9usize, 2usize, 68u32, 2211284u32)?;
    emu.lw_no_count(18usize, 2usize, 64u32, 2211288u32)?;
    emu.lw_no_count(19usize, 2usize, 60u32, 2211292u32)?;
    emu.lw_no_count(20usize, 2usize, 56u32, 2211296u32)?;
    emu.lw_no_count(21usize, 2usize, 52u32, 2211300u32)?;
    emu.lw_no_count(22usize, 2usize, 48u32, 2211304u32)?;
    emu.lw_no_count(23usize, 2usize, 44u32, 2211308u32)?;
    emu.adi_no_count(2usize, 2usize, 80u32, 2211312u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2211316u32;
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
pub fn block_0x0021bdf4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 14usize, 0u32, 2211320u32);
    emu.adi_no_count(9usize, 13usize, 0u32, 2211324u32);
    emu.lw_no_count(19usize, 8usize, 0u32, 2211328u32)?;
    emu.lbu_no_count(10usize, 8usize, 5u32, 2211332u32);
    emu.lbu_no_count(13usize, 19usize, 10u32, 2211336u32);
    emu.ani_no_count(13usize, 13usize, 128u32, 2211340u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2211372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021be2c));
    } else {
        emu.pc = 2211344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021be10));
    }
}
#[inline(always)]
pub fn block_0x0021be10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(13usize, 10usize, 3u32, 2211348u32);
    emu.adi_no_count(22usize, 11usize, 0u32, 2211352u32);
    emu.adi_no_count(23usize, 12usize, 0u32, 2211356u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2211596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bf0c));
    } else {
        emu.pc = 2211360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021be20));
    }
}
#[inline(always)]
pub fn block_0x0021be20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2211364u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1920u32, 2211368u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2211372u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2211604u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bf14));
}
#[inline(always)]
pub fn block_0x0021be2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2211432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021be68));
    } else {
        emu.pc = 2211376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021be30));
    }
}
#[inline]
pub fn block_0x0021be30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 19usize, 4u32, 2211380u32)?;
    emu.lw_no_count(10usize, 19usize, 0u32, 2211384u32)?;
    emu.lw_no_count(14usize, 13usize, 12u32, 2211388u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2211392u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1927u32, 2211396u32);
    emu.adi_no_count(20usize, 12usize, 0u32, 2211400u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2211404u32);
    emu.adi_no_count(22usize, 11usize, 0u32, 2211408u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2211412u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2211416u32;
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
pub fn block_0x0021be58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 22usize, 0u32, 2211420u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2211424u32);
    emu.adi_no_count(20usize, 0usize, 1u32, 2211428u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2211260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bdbc));
    } else {
        emu.pc = 2211432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021be68));
    }
}
#[inline]
pub fn block_0x0021be68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 1u32, 2211436u32);
    emu.adi_no_count(10usize, 2usize, 27u32, 2211440u32);
    emu.lw_no_count(13usize, 19usize, 0u32, 2211444u32)?;
    emu.lw_no_count(14usize, 19usize, 4u32, 2211448u32)?;
    emu.lw_no_count(15usize, 19usize, 8u32, 2211452u32)?;
    emu.lw_no_count(16usize, 19usize, 12u32, 2211456u32)?;
    emu.adi_no_count(17usize, 2usize, 12u32, 2211460u32);
    emu.sw_no_count(13usize, 2usize, 12u32, 2211464u32)?;
    emu.sw_no_count(14usize, 2usize, 16u32, 2211468u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2211472u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2211476u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1896u32, 2211480u32);
    emu.sb_no_count(20usize, 2usize, 27u32, 2211484u32);
    emu.sw_no_count(17usize, 2usize, 28u32, 2211488u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2211492u32)?;
    emu.sw_no_count(15usize, 2usize, 36u32, 2211496u32)?;
    emu.sw_no_count(16usize, 2usize, 40u32, 2211500u32)?;
    emu.adi_no_count(10usize, 2usize, 12u32, 2211504u32);
    emu.apc_no_count(1usize, 2211504u32, 0u32, 2211508u32);
    emu.add_memory_rw_events(20usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2211512u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966140u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021beb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2211260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bdbc));
    } else {
        emu.pc = 2211516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bebc));
    }
}
#[inline(always)]
pub fn block_0x0021bebc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2211520u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1925u32, 2211524u32);
    emu.adi_no_count(10usize, 2usize, 12u32, 2211528u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2211532u32);
    emu.apc_no_count(1usize, 2211532u32, 0u32, 2211536u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2211540u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966112u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021bed4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2211260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bdbc));
    } else {
        emu.pc = 2211544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bed8));
    }
}
#[inline(always)]
pub fn block_0x0021bed8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 12u32, 2211548u32)?;
    emu.adi_no_count(11usize, 2usize, 28u32, 2211552u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2211556u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2211560u32;
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
pub fn block_0x0021bee8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2211260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bdbc));
    } else {
        emu.pc = 2211564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021beec));
    }
}
#[inline(always)]
pub fn block_0x0021beec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 32u32, 2211568u32)?;
    emu.lw_no_count(10usize, 2usize, 28u32, 2211572u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2211576u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2211580u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1930u32, 2211584u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2211588u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2211592u32;
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
pub fn block_0x0021bf08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2211596u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2211716u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bf84));
}
#[inline(always)]
pub fn block_0x0021bf0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2211600u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1923u32, 2211604u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2211604u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bf14));
}
#[inline(always)]
pub fn block_0x0021bf14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 19usize, 4u32, 2211608u32)?;
    emu.lw_no_count(10usize, 19usize, 0u32, 2211612u32)?;
    emu.lw_no_count(14usize, 12usize, 12u32, 2211616u32)?;
    emu.adi_no_count(12usize, 13usize, 0u32, 2211620u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2211624u32;
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
pub fn block_0x0021bf28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 1u32, 2211628u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2211260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bdbc));
    } else {
        emu.pc = 2211632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bf30));
    }
}
#[inline(always)]
pub fn block_0x0021bf30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 23usize, 0u32, 2211636u32);
    emu.adi_no_count(11usize, 22usize, 0u32, 2211640u32);
    emu.lw_no_count(13usize, 19usize, 4u32, 2211644u32)?;
    emu.lw_no_count(10usize, 19usize, 0u32, 2211648u32)?;
    emu.lw_no_count(13usize, 13usize, 12u32, 2211652u32)?;
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2211656u32;
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
pub fn block_0x0021bf48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 1u32, 2211660u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2211260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bdbc));
    } else {
        emu.pc = 2211664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bf50));
    }
}
#[inline(always)]
pub fn block_0x0021bf50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 19usize, 4u32, 2211668u32)?;
    emu.lw_no_count(10usize, 19usize, 0u32, 2211672u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2211676u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2211680u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1925u32, 2211684u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2211688u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2211692u32;
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
pub fn block_0x0021bf6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 1u32, 2211696u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2211260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bdbc));
    } else {
        emu.pc = 2211700u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bf74));
    }
}
#[inline(always)]
pub fn block_0x0021bf74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 12u32, 2211704u32)?;
    emu.adi_no_count(10usize, 9usize, 0u32, 2211708u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2211712u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2211716u32;
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
pub fn block_0x0021bf84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 10usize, 0u32, 2211720u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2211724u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2211260u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bdbc));
}
