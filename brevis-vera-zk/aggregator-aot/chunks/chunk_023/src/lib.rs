pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2208280u32;
pub const PC_MAX: u32 = 2211884u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 122usize] = [
        block_0x0021b218,
        block_0x0021b224,
        block_0x0021b244,
        block_0x0021b25c,
        block_0x0021b264,
        block_0x0021b27c,
        block_0x0021b2a4,
        block_0x0021b2b0,
        block_0x0021b2d0,
        block_0x0021b2e8,
        block_0x0021b2f0,
        block_0x0021b308,
        block_0x0021b330,
        block_0x0021b33c,
        block_0x0021b360,
        block_0x0021b380,
        block_0x0021b3bc,
        block_0x0021b3c0,
        block_0x0021b3cc,
        block_0x0021b3d0,
        block_0x0021b3f4,
        block_0x0021b41c,
        block_0x0021b428,
        block_0x0021b450,
        block_0x0021b47c,
        block_0x0021b4ec,
        block_0x0021b4f4,
        block_0x0021b548,
        block_0x0021b54c,
        block_0x0021b550,
        block_0x0021b570,
        block_0x0021b598,
        block_0x0021b5a8,
        block_0x0021b5b4,
        block_0x0021b5b8,
        block_0x0021b5e8,
        block_0x0021b614,
        block_0x0021b684,
        block_0x0021b68c,
        block_0x0021b6e0,
        block_0x0021b6e4,
        block_0x0021b6e8,
        block_0x0021b708,
        block_0x0021b734,
        block_0x0021b744,
        block_0x0021b750,
        block_0x0021b754,
        block_0x0021b770,
        block_0x0021b79c,
        block_0x0021b810,
        block_0x0021b818,
        block_0x0021b868,
        block_0x0021b86c,
        block_0x0021b870,
        block_0x0021b884,
        block_0x0021b8b4,
        block_0x0021b8c4,
        block_0x0021b8c8,
        block_0x0021b908,
        block_0x0021b928,
        block_0x0021b940,
        block_0x0021b9a0,
        block_0x0021b9c8,
        block_0x0021b9e4,
        block_0x0021ba5c,
        block_0x0021ba6c,
        block_0x0021bacc,
        block_0x0021bad0,
        block_0x0021baec,
        block_0x0021baf8,
        block_0x0021bb00,
        block_0x0021bb1c,
        block_0x0021bb5c,
        block_0x0021bb7c,
        block_0x0021bbb8,
        block_0x0021bc18,
        block_0x0021bc48,
        block_0x0021bca0,
        block_0x0021bcfc,
        block_0x0021bd14,
        block_0x0021bd7c,
        block_0x0021bd8c,
        block_0x0021bd94,
        block_0x0021bda8,
        block_0x0021bdbc,
        block_0x0021bdc4,
        block_0x0021bdd0,
        block_0x0021bdd8,
        block_0x0021bde8,
        block_0x0021bdf0,
        block_0x0021bdf8,
        block_0x0021be00,
        block_0x0021be0c,
        block_0x0021be10,
        block_0x0021be14,
        block_0x0021be18,
        block_0x0021be1c,
        block_0x0021be2c,
        block_0x0021be30,
        block_0x0021be38,
        block_0x0021be3c,
        block_0x0021be40,
        block_0x0021be4c,
        block_0x0021be54,
        block_0x0021be58,
        block_0x0021be64,
        block_0x0021be74,
        block_0x0021be80,
        block_0x0021be94,
        block_0x0021beac,
        block_0x0021bed0,
        block_0x0021bed4,
        block_0x0021bedc,
        block_0x0021bee8,
        block_0x0021bef0,
        block_0x0021bf04,
        block_0x0021bf0c,
        block_0x0021bf14,
        block_0x0021bf20,
        block_0x0021bfbc,
        block_0x0021bfc0,
        block_0x0021c02c,
    ];
    const IDX: [u16; 902usize] = [
        1u16, 0u16, 0u16, 2u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 3u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 4u16, 0u16, 5u16, 0u16, 0u16, 0u16, 0u16, 0u16, 6u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 7u16, 0u16, 0u16, 8u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 9u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        10u16, 0u16, 11u16, 0u16, 0u16, 0u16, 0u16, 0u16, 12u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 13u16, 0u16, 0u16, 14u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 15u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 16u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 17u16, 18u16, 0u16, 0u16, 19u16, 20u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 21u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 22u16,
        0u16, 0u16, 23u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 24u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 25u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 26u16,
        0u16, 27u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 28u16, 29u16, 30u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 31u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 32u16, 0u16, 0u16, 0u16, 33u16, 0u16, 0u16, 34u16, 35u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 36u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 37u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 38u16, 0u16, 39u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 40u16, 41u16, 42u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 43u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 44u16, 0u16, 0u16, 0u16, 45u16, 0u16, 0u16, 46u16, 47u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 48u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 49u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 50u16, 0u16, 51u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 52u16, 53u16, 54u16, 0u16, 0u16, 0u16, 0u16, 55u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 56u16, 0u16, 0u16, 0u16, 57u16, 58u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 59u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 60u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 61u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 62u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 63u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 64u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 65u16, 0u16, 0u16, 0u16,
        66u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 67u16, 68u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 69u16, 0u16, 0u16, 70u16, 0u16, 71u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 72u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 73u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 74u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 75u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 76u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 77u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 78u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 79u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 80u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 81u16, 0u16, 0u16, 0u16, 82u16, 0u16, 83u16, 0u16, 0u16, 0u16, 0u16, 84u16,
        0u16, 0u16, 0u16, 0u16, 85u16, 0u16, 86u16, 0u16, 0u16, 87u16, 0u16, 88u16, 0u16,
        0u16, 0u16, 89u16, 0u16, 90u16, 0u16, 91u16, 0u16, 92u16, 0u16, 0u16, 93u16,
        94u16, 95u16, 96u16, 97u16, 0u16, 0u16, 0u16, 98u16, 99u16, 0u16, 100u16, 101u16,
        102u16, 0u16, 0u16, 103u16, 0u16, 104u16, 105u16, 0u16, 0u16, 106u16, 0u16, 0u16,
        0u16, 107u16, 0u16, 0u16, 108u16, 0u16, 0u16, 0u16, 0u16, 109u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 110u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 111u16,
        112u16, 0u16, 113u16, 0u16, 0u16, 114u16, 0u16, 115u16, 0u16, 0u16, 0u16, 0u16,
        116u16, 0u16, 117u16, 0u16, 118u16, 0u16, 0u16, 119u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 120u16, 121u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 122u16,
    ];
    if pc < 2208280u32 || pc > 2211884u32 {
        return None;
    }
    let word_offset = ((pc - 2208280u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x0021b218(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 140u32, 2208284u32)?;
    emu.adi_no_count(2usize, 2usize, 144u32, 2208288u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2208292u32;
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
pub fn block_0x0021b224(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967152u32, 2208296u32);
    emu.sw_no_count(1usize, 2usize, 140u32, 2208300u32)?;
    emu.adi_no_count(16usize, 11usize, 0u32, 2208304u32);
    emu.adi_no_count(15usize, 0usize, 0u32, 2208308u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2208312u32)?;
    emu.adi_no_count(11usize, 2usize, 139u32, 2208316u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2208320u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2208324u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2208348u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b25c));
}
#[inline(always)]
pub fn block_0x0021b244(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 87u32, 2208328u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2208332u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2208336u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2208340u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2208344u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2208380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b27c));
    } else {
        emu.pc = 2208348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b25c));
    }
}
#[inline(always)]
pub fn block_0x0021b25c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(13usize, 10usize, 15u32, 2208352u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2208324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b244));
    } else {
        emu.pc = 2208356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b264));
    }
}
#[inline(always)]
pub fn block_0x0021b264(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 48u32, 2208360u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2208364u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2208368u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2208372u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2208376u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2208348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b25c));
    } else {
        emu.pc = 2208380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b27c));
    }
}
#[inline]
pub fn block_0x0021b27c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2208384u32);
    emu.sbr_no_count(10usize, 10usize, 15usize, 2208388u32);
    emu.adi_no_count(14usize, 10usize, 128u32, 2208392u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2208396u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965984u32, 2208400u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2208404u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2208408u32);
    emu.adi_no_count(10usize, 16usize, 0u32, 2208412u32);
    emu.apc_no_count(1usize, 2208412u32, 8192u32, 2208416u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2208420u32;
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
pub fn block_0x0021b2a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 140u32, 2208424u32)?;
    emu.adi_no_count(2usize, 2usize, 144u32, 2208428u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2208432u32;
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
pub fn block_0x0021b2b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967152u32, 2208436u32);
    emu.sw_no_count(1usize, 2usize, 140u32, 2208440u32)?;
    emu.adi_no_count(16usize, 11usize, 0u32, 2208444u32);
    emu.adi_no_count(15usize, 0usize, 0u32, 2208448u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2208452u32)?;
    emu.adi_no_count(11usize, 2usize, 139u32, 2208456u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2208460u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2208464u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2208488u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b2e8));
}
#[inline(always)]
pub fn block_0x0021b2d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 55u32, 2208468u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2208472u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2208476u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2208480u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2208484u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2208520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b308));
    } else {
        emu.pc = 2208488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b2e8));
    }
}
#[inline(always)]
pub fn block_0x0021b2e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(13usize, 10usize, 15u32, 2208492u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2208464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b2d0));
    } else {
        emu.pc = 2208496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b2f0));
    }
}
#[inline(always)]
pub fn block_0x0021b2f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 48u32, 2208500u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2208504u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2208508u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2208512u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2208516u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2208488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b2e8));
    } else {
        emu.pc = 2208520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b308));
    }
}
#[inline]
pub fn block_0x0021b308(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2208524u32);
    emu.sbr_no_count(10usize, 10usize, 15usize, 2208528u32);
    emu.adi_no_count(14usize, 10usize, 128u32, 2208532u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2208536u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965984u32, 2208540u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2208544u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2208548u32);
    emu.adi_no_count(10usize, 16usize, 0u32, 2208552u32);
    emu.apc_no_count(1usize, 2208552u32, 8192u32, 2208556u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2208560u32;
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
pub fn block_0x0021b330(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 140u32, 2208564u32)?;
    emu.adi_no_count(2usize, 2usize, 144u32, 2208568u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2208572u32;
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
pub fn block_0x0021b33c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 4294967295u32, 2208576u32);
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2208580u32;
    emu.update_insn_clock();
    emu.sbr_no_count(13usize, 13usize, 11usize, 2208584u32);
    emu.xrr_no_count(11usize, 11usize, 12usize, 2208588u32);
    emu.sltru_no_count(11usize, 12usize, 11usize, 2208592u32);
    emu.sltru_no_count(10usize, 13usize, 10usize, 2208596u32);
    emu.xri_no_count(10usize, 10usize, 1u32, 2208600u32);
    emu.anr_no_count(10usize, 11usize, 10usize, 2208604u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2208608u32;
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
pub fn block_0x0021b360(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2208612u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2208616u32)?;
    emu.adi_no_count(13usize, 11usize, 0u32, 2208620u32);
    emu.lbu_no_count(11usize, 10usize, 0u32, 2208624u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2208628u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2208632u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965986u32, 2208636u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2208704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b3c0));
    } else {
        emu.pc = 2208640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b380));
    }
}
#[inline]
pub fn block_0x0021b380(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 41u32, 2208644u32);
    emu.adi_no_count(14usize, 0usize, 100u32, 2208648u32);
    emu.mul_no_count(12usize, 11usize, 12usize, 2208652u32);
    emu.sri_no_count(12usize, 12usize, 12u32, 2208656u32);
    emu.mul_no_count(14usize, 12usize, 14usize, 2208660u32);
    emu.sbr_no_count(14usize, 11usize, 14usize, 2208664u32);
    emu.sli_no_count(14usize, 14usize, 25u32, 2208668u32);
    emu.sri_no_count(14usize, 14usize, 24u32, 2208672u32);
    emu.adr_no_count(14usize, 10usize, 14usize, 2208676u32);
    emu.lbu_no_count(15usize, 14usize, 0u32, 2208680u32);
    emu.lbu_no_count(14usize, 14usize, 1u32, 2208684u32);
    emu.sb_no_count(15usize, 2usize, 10u32, 2208688u32);
    emu.sb_no_count(14usize, 2usize, 11u32, 2208692u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2208696u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2208716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b3cc));
    } else {
        emu.pc = 2208700u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b3bc));
    }
}
#[inline(always)]
pub fn block_0x0021b3bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2208704u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2208720u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b3d0));
}
#[inline(always)]
pub fn block_0x0021b3c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 3u32, 2208708u32);
    emu.adi_no_count(12usize, 11usize, 0u32, 2208712u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2208720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b3d0));
    } else {
        emu.pc = 2208716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b3cc));
    }
}
#[inline(always)]
pub fn block_0x0021b3cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2208756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b3f4));
    } else {
        emu.pc = 2208720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b3d0));
    }
}
#[inline]
pub fn block_0x0021b3d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 1u32, 2208724u32);
    emu.adi_no_count(12usize, 12usize, 1u32, 2208728u32);
    emu.ani_no_count(11usize, 12usize, 255u32, 2208732u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2208736u32);
    emu.lbu_no_count(10usize, 10usize, 0u32, 2208740u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2208744u32);
    emu.adi_no_count(11usize, 2usize, 9u32, 2208748u32);
    emu.adr_no_count(11usize, 11usize, 14usize, 2208752u32);
    emu.sb_no_count(10usize, 11usize, 0u32, 2208756u32);
    emu.add_memory_rw_events(9usize);
    emu.pc = 2208756u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b3f4));
}
#[inline]
pub fn block_0x0021b3f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 3u32, 2208760u32);
    emu.adi_no_count(10usize, 2usize, 9u32, 2208764u32);
    emu.sbr_no_count(15usize, 15usize, 14usize, 2208768u32);
    emu.adr_no_count(14usize, 10usize, 14usize, 2208772u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2208776u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2208780u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2208784u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2208788u32);
    emu.apc_no_count(1usize, 2208788u32, 8192u32, 2208792u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2208796u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967244u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021b41c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2208800u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2208804u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2208808u32;
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
pub fn block_0x0021b428(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2208812u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2208816u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2208820u32)?;
    emu.adi_no_count(13usize, 11usize, 0u32, 2208824u32);
    emu.lw_no_count(11usize, 10usize, 0u32, 2208828u32)?;
    emu.adi_no_count(12usize, 0usize, 1000u32, 2208832u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2208836u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965986u32, 2208840u32);
    emu.adi_no_count(14usize, 0usize, 10u32, 2208844u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2209192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b5a8));
    } else {
        emu.pc = 2208848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b450));
    }
}
#[inline]
pub fn block_0x0021b450(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 2usize, 23u32, 2208852u32);
    let a = 0u32.wrapping_add(3518435328u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2208856u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2208860u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2208864u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 0usize, 100u32, 2208868u32);
    let a = 0u32.wrapping_add(9998336u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2208872u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 12usize, 1881u32, 2208876u32);
    emu.adi_no_count(5usize, 5usize, 1808u32, 2208880u32);
    emu.adi_no_count(6usize, 6usize, 1147u32, 2208884u32);
    emu.adi_no_count(7usize, 7usize, 1663u32, 2208888u32);
    emu.adi_no_count(12usize, 11usize, 0u32, 2208892u32);
    emu.add_memory_rw_events(11usize);
    emu.pc = 2208892u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b47c));
}
#[inline(never)]
pub fn block_0x0021b47c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(28usize, 12usize, 0u32, 2208896u32);
    emu.adi_no_count(14usize, 14usize, 4294967292u32, 2208900u32);
    emu.mulhu_no_count(12usize, 12usize, 17usize, 2208904u32);
    emu.sri_no_count(12usize, 12usize, 13u32, 2208908u32);
    emu.mul_no_count(29usize, 12usize, 5usize, 2208912u32);
    emu.sbr_no_count(29usize, 28usize, 29usize, 2208916u32);
    emu.sli_no_count(30usize, 29usize, 16u32, 2208920u32);
    emu.sri_no_count(30usize, 30usize, 18u32, 2208924u32);
    emu.mul_no_count(30usize, 30usize, 6usize, 2208928u32);
    emu.sri_no_count(31usize, 30usize, 16u32, 2208932u32);
    emu.sri_no_count(30usize, 30usize, 17u32, 2208936u32);
    emu.mul_no_count(30usize, 30usize, 16usize, 2208940u32);
    emu.ani_no_count(31usize, 31usize, 2046u32, 2208944u32);
    emu.sbr_no_count(29usize, 29usize, 30usize, 2208948u32);
    emu.adr_no_count(31usize, 10usize, 31usize, 2208952u32);
    emu.sli_no_count(29usize, 29usize, 17u32, 2208956u32);
    emu.sri_no_count(29usize, 29usize, 16u32, 2208960u32);
    emu.adr_no_count(29usize, 10usize, 29usize, 2208964u32);
    emu.lbu_no_count(30usize, 31usize, 0u32, 2208968u32);
    emu.lbu_no_count(31usize, 31usize, 1u32, 2208972u32);
    emu.lbu_no_count(8usize, 29usize, 0u32, 2208976u32);
    emu.lbu_no_count(29usize, 29usize, 1u32, 2208980u32);
    emu.sb_no_count(30usize, 15usize, 4294967293u32, 2208984u32);
    emu.sb_no_count(31usize, 15usize, 4294967294u32, 2208988u32);
    emu.sb_no_count(8usize, 15usize, 4294967295u32, 2208992u32);
    emu.sb_no_count(29usize, 15usize, 0u32, 2208996u32);
    emu.adi_no_count(15usize, 15usize, 4294967292u32, 2209000u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a < b {
        emu.pc = 2208892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b47c));
    } else {
        emu.pc = 2209004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b4ec));
    }
}
#[inline(always)]
pub fn block_0x0021b4ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 9u32, 2209008u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2209096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b548));
    } else {
        emu.pc = 2209012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b4f4));
    }
}
#[inline]
pub fn block_0x0021b4f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(15usize, 12usize, 16u32, 2209016u32);
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2209020u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 0usize, 100u32, 2209024u32);
    emu.adi_no_count(5usize, 2usize, 14u32, 2209028u32);
    emu.sri_no_count(15usize, 15usize, 18u32, 2209032u32);
    emu.adi_no_count(16usize, 16usize, 1147u32, 2209036u32);
    emu.adr_no_count(6usize, 5usize, 14usize, 2209040u32);
    emu.mul_no_count(15usize, 15usize, 16usize, 2209044u32);
    emu.sri_no_count(15usize, 15usize, 17u32, 2209048u32);
    emu.mul_no_count(16usize, 15usize, 17usize, 2209052u32);
    emu.sbr_no_count(12usize, 12usize, 16usize, 2209056u32);
    emu.sli_no_count(12usize, 12usize, 17u32, 2209060u32);
    emu.sri_no_count(12usize, 12usize, 16u32, 2209064u32);
    emu.adr_no_count(12usize, 10usize, 12usize, 2209068u32);
    emu.lbu_no_count(16usize, 12usize, 0u32, 2209072u32);
    emu.lbu_no_count(12usize, 12usize, 1u32, 2209076u32);
    emu.adi_no_count(14usize, 14usize, 4294967294u32, 2209080u32);
    emu.adr_no_count(5usize, 5usize, 14usize, 2209084u32);
    emu.sb_no_count(16usize, 5usize, 0u32, 2209088u32);
    emu.sb_no_count(12usize, 6usize, 4294967295u32, 2209092u32);
    emu.adi_no_count(12usize, 15usize, 0u32, 2209096u32);
    emu.add_memory_rw_events(21usize);
    emu.pc = 2209096u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b548));
}
#[inline(always)]
pub fn block_0x0021b548(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2209104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b550));
    } else {
        emu.pc = 2209100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b54c));
    }
}
#[inline(always)]
pub fn block_0x0021b54c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2209136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b570));
    } else {
        emu.pc = 2209104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b550));
    }
}
#[inline(always)]
pub fn block_0x0021b550(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 1u32, 2209108u32);
    emu.ani_no_count(12usize, 12usize, 30u32, 2209112u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2209116u32);
    emu.lbu_no_count(10usize, 10usize, 1u32, 2209120u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2209124u32);
    emu.adi_no_count(11usize, 2usize, 14u32, 2209128u32);
    emu.adr_no_count(11usize, 11usize, 14usize, 2209132u32);
    emu.sb_no_count(10usize, 11usize, 0u32, 2209136u32);
    emu.add_memory_rw_events(8usize);
    emu.pc = 2209136u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b570));
}
#[inline]
pub fn block_0x0021b570(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 10u32, 2209140u32);
    emu.adi_no_count(10usize, 2usize, 14u32, 2209144u32);
    emu.sbr_no_count(15usize, 15usize, 14usize, 2209148u32);
    emu.adr_no_count(14usize, 10usize, 14usize, 2209152u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2209156u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2209160u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2209164u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2209168u32);
    emu.apc_no_count(1usize, 2209168u32, 8192u32, 2209172u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2209176u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966864u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021b598(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2209180u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2209184u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2209188u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2209192u32;
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
pub fn block_0x0021b5a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 0u32, 2209196u32);
    emu.adi_no_count(15usize, 0usize, 9u32, 2209200u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2209012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b4f4));
    } else {
        emu.pc = 2209204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b5b4));
    }
}
#[inline(always)]
pub fn block_0x0021b5b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2209208u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2209096u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b548));
}
#[inline]
pub fn block_0x0021b5b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2209212u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2209216u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2209220u32)?;
    emu.adi_no_count(13usize, 11usize, 0u32, 2209224u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2209228u32)?;
    emu.sai_no_count(11usize, 10usize, 1055u32, 2209232u32);
    emu.xrr_no_count(12usize, 10usize, 11usize, 2209236u32);
    emu.sbr_no_count(12usize, 12usize, 11usize, 2209240u32);
    emu.adi_no_count(14usize, 0usize, 1000u32, 2209244u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2209248u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965986u32, 2209252u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2209604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b744));
    } else {
        emu.pc = 2209256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b5e8));
    }
}
#[inline]
pub fn block_0x0021b5e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 10u32, 2209260u32);
    emu.adi_no_count(15usize, 2usize, 23u32, 2209264u32);
    let a = 0u32.wrapping_add(3518435328u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2209268u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2209272u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2209276u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 0usize, 100u32, 2209280u32);
    let a = 0u32.wrapping_add(9998336u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2209284u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 1881u32, 2209288u32);
    emu.adi_no_count(5usize, 5usize, 1808u32, 2209292u32);
    emu.adi_no_count(6usize, 6usize, 1147u32, 2209296u32);
    emu.adi_no_count(7usize, 7usize, 1663u32, 2209300u32);
    emu.add_memory_rw_events(11usize);
    emu.pc = 2209300u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b614));
}
#[inline(never)]
pub fn block_0x0021b614(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(28usize, 12usize, 0u32, 2209304u32);
    emu.adi_no_count(14usize, 14usize, 4294967292u32, 2209308u32);
    emu.mulhu_no_count(12usize, 12usize, 17usize, 2209312u32);
    emu.sri_no_count(12usize, 12usize, 13u32, 2209316u32);
    emu.mul_no_count(29usize, 12usize, 5usize, 2209320u32);
    emu.sbr_no_count(29usize, 28usize, 29usize, 2209324u32);
    emu.sli_no_count(30usize, 29usize, 16u32, 2209328u32);
    emu.sri_no_count(30usize, 30usize, 18u32, 2209332u32);
    emu.mul_no_count(30usize, 30usize, 6usize, 2209336u32);
    emu.sri_no_count(31usize, 30usize, 16u32, 2209340u32);
    emu.sri_no_count(30usize, 30usize, 17u32, 2209344u32);
    emu.mul_no_count(30usize, 30usize, 16usize, 2209348u32);
    emu.ani_no_count(31usize, 31usize, 2046u32, 2209352u32);
    emu.sbr_no_count(29usize, 29usize, 30usize, 2209356u32);
    emu.adr_no_count(31usize, 11usize, 31usize, 2209360u32);
    emu.sli_no_count(29usize, 29usize, 17u32, 2209364u32);
    emu.sri_no_count(29usize, 29usize, 16u32, 2209368u32);
    emu.adr_no_count(29usize, 11usize, 29usize, 2209372u32);
    emu.lbu_no_count(30usize, 31usize, 0u32, 2209376u32);
    emu.lbu_no_count(31usize, 31usize, 1u32, 2209380u32);
    emu.lbu_no_count(8usize, 29usize, 0u32, 2209384u32);
    emu.lbu_no_count(29usize, 29usize, 1u32, 2209388u32);
    emu.sb_no_count(30usize, 15usize, 4294967293u32, 2209392u32);
    emu.sb_no_count(31usize, 15usize, 4294967294u32, 2209396u32);
    emu.sb_no_count(8usize, 15usize, 4294967295u32, 2209400u32);
    emu.sb_no_count(29usize, 15usize, 0u32, 2209404u32);
    emu.adi_no_count(15usize, 15usize, 4294967292u32, 2209408u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a < b {
        emu.pc = 2209300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b614));
    } else {
        emu.pc = 2209412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b684));
    }
}
#[inline(always)]
pub fn block_0x0021b684(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 9u32, 2209416u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2209504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b6e0));
    } else {
        emu.pc = 2209420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b68c));
    }
}
#[inline]
pub fn block_0x0021b68c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(15usize, 12usize, 16u32, 2209424u32);
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2209428u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 0usize, 100u32, 2209432u32);
    emu.adi_no_count(5usize, 2usize, 14u32, 2209436u32);
    emu.sri_no_count(15usize, 15usize, 18u32, 2209440u32);
    emu.adi_no_count(16usize, 16usize, 1147u32, 2209444u32);
    emu.adr_no_count(6usize, 5usize, 14usize, 2209448u32);
    emu.mul_no_count(15usize, 15usize, 16usize, 2209452u32);
    emu.sri_no_count(15usize, 15usize, 17u32, 2209456u32);
    emu.mul_no_count(16usize, 15usize, 17usize, 2209460u32);
    emu.sbr_no_count(12usize, 12usize, 16usize, 2209464u32);
    emu.sli_no_count(12usize, 12usize, 17u32, 2209468u32);
    emu.sri_no_count(12usize, 12usize, 16u32, 2209472u32);
    emu.adr_no_count(12usize, 11usize, 12usize, 2209476u32);
    emu.lbu_no_count(16usize, 12usize, 0u32, 2209480u32);
    emu.lbu_no_count(12usize, 12usize, 1u32, 2209484u32);
    emu.adi_no_count(14usize, 14usize, 4294967294u32, 2209488u32);
    emu.adr_no_count(5usize, 5usize, 14usize, 2209492u32);
    emu.sb_no_count(16usize, 5usize, 0u32, 2209496u32);
    emu.sb_no_count(12usize, 6usize, 4294967295u32, 2209500u32);
    emu.adi_no_count(12usize, 15usize, 0u32, 2209504u32);
    emu.add_memory_rw_events(21usize);
    emu.pc = 2209504u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b6e0));
}
#[inline(always)]
pub fn block_0x0021b6e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2209512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b6e8));
    } else {
        emu.pc = 2209508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b6e4));
    }
}
#[inline(always)]
pub fn block_0x0021b6e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2209544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b708));
    } else {
        emu.pc = 2209512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b6e8));
    }
}
#[inline(always)]
pub fn block_0x0021b6e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 1u32, 2209516u32);
    emu.ani_no_count(12usize, 12usize, 30u32, 2209520u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2209524u32);
    emu.lbu_no_count(11usize, 11usize, 1u32, 2209528u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2209532u32);
    emu.adi_no_count(12usize, 2usize, 14u32, 2209536u32);
    emu.adr_no_count(12usize, 12usize, 14usize, 2209540u32);
    emu.sb_no_count(11usize, 12usize, 0u32, 2209544u32);
    emu.add_memory_rw_events(8usize);
    emu.pc = 2209544u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b708));
}
#[inline]
pub fn block_0x0021b708(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 10u32, 2209548u32);
    emu.adi_no_count(11usize, 2usize, 14u32, 2209552u32);
    emu.sbr_no_count(15usize, 15usize, 14usize, 2209556u32);
    emu.adr_no_count(14usize, 11usize, 14usize, 2209560u32);
    emu.xri_no_count(11usize, 10usize, 4294967295u32, 2209564u32);
    emu.sri_no_count(11usize, 11usize, 31u32, 2209568u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2209572u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2209576u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2209580u32);
    emu.apc_no_count(1usize, 2209580u32, 8192u32, 2209584u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2209588u32;
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
pub fn block_0x0021b734(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2209592u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2209596u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2209600u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2209604u32;
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
pub fn block_0x0021b744(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 10u32, 2209608u32);
    emu.adi_no_count(15usize, 0usize, 9u32, 2209612u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2209420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b68c));
    } else {
        emu.pc = 2209616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b750));
    }
}
#[inline(always)]
pub fn block_0x0021b750(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2209620u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2209504u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b6e0));
}
#[inline(always)]
pub fn block_0x0021b754(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2209624u32);
    emu.sw_no_count(8usize, 2usize, 12u32, 2209628u32)?;
    emu.sw_no_count(9usize, 2usize, 8u32, 2209632u32)?;
    emu.adi_no_count(13usize, 0usize, 1000u32, 2209636u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2209640u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965986u32, 2209644u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2209972u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b8b4));
    } else {
        emu.pc = 2209648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b770));
    }
}
#[inline]
pub fn block_0x0021b770(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 11usize, 4294967294u32, 2209652u32);
    let a = 0u32.wrapping_add(3518435328u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2209656u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2209660u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2209664u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 0usize, 100u32, 2209668u32);
    let a = 0u32.wrapping_add(9998336u32);
    emu.write_reg_no_count(28usize, a);
    emu.pc = 2209672u32;
    emu.update_insn_clock();
    emu.adi_no_count(5usize, 13usize, 1881u32, 2209676u32);
    emu.adi_no_count(6usize, 15usize, 1808u32, 2209680u32);
    emu.adi_no_count(7usize, 7usize, 1147u32, 2209684u32);
    emu.adi_no_count(28usize, 28usize, 1663u32, 2209688u32);
    emu.adi_no_count(15usize, 10usize, 0u32, 2209692u32);
    emu.add_memory_rw_events(11usize);
    emu.pc = 2209692u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b79c));
}
#[inline(never)]
pub fn block_0x0021b79c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 29u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(29usize, 15usize, 0u32, 2209696u32);
    emu.adi_no_count(13usize, 12usize, 4294967292u32, 2209700u32);
    emu.mulhu_no_count(15usize, 15usize, 5usize, 2209704u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2209708u32);
    emu.sri_no_count(15usize, 15usize, 13u32, 2209712u32);
    emu.mul_no_count(30usize, 15usize, 6usize, 2209716u32);
    emu.sbr_no_count(30usize, 29usize, 30usize, 2209720u32);
    emu.sli_no_count(31usize, 30usize, 16u32, 2209724u32);
    emu.sri_no_count(31usize, 31usize, 18u32, 2209728u32);
    emu.mul_no_count(31usize, 31usize, 7usize, 2209732u32);
    emu.sri_no_count(8usize, 31usize, 16u32, 2209736u32);
    emu.sri_no_count(31usize, 31usize, 17u32, 2209740u32);
    emu.mul_no_count(31usize, 31usize, 17usize, 2209744u32);
    emu.ani_no_count(8usize, 8usize, 2046u32, 2209748u32);
    emu.sbr_no_count(30usize, 30usize, 31usize, 2209752u32);
    emu.adr_no_count(8usize, 14usize, 8usize, 2209756u32);
    emu.sli_no_count(30usize, 30usize, 17u32, 2209760u32);
    emu.sri_no_count(30usize, 30usize, 16u32, 2209764u32);
    emu.adr_no_count(30usize, 14usize, 30usize, 2209768u32);
    emu.lbu_no_count(31usize, 8usize, 0u32, 2209772u32);
    emu.lbu_no_count(8usize, 8usize, 1u32, 2209776u32);
    emu.lbu_no_count(9usize, 30usize, 0u32, 2209780u32);
    emu.lbu_no_count(30usize, 30usize, 1u32, 2209784u32);
    emu.sb_no_count(31usize, 12usize, 4294967294u32, 2209788u32);
    emu.sb_no_count(8usize, 12usize, 4294967295u32, 2209792u32);
    emu.sb_no_count(9usize, 12usize, 0u32, 2209796u32);
    emu.sb_no_count(30usize, 12usize, 1u32, 2209800u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2209804u32);
    emu.add_memory_rw_events(28usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a < b {
        emu.pc = 2209692u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b79c));
    } else {
        emu.pc = 2209808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b810));
    }
}
#[inline(always)]
pub fn block_0x0021b810(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 9u32, 2209812u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a >= b {
        emu.pc = 2209896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b868));
    } else {
        emu.pc = 2209816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b818));
    }
}
#[inline]
pub fn block_0x0021b818(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 15usize, 16u32, 2209820u32);
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2209824u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 0usize, 100u32, 2209828u32);
    emu.adr_no_count(5usize, 11usize, 13usize, 2209832u32);
    emu.sri_no_count(12usize, 12usize, 18u32, 2209836u32);
    emu.adi_no_count(16usize, 16usize, 1147u32, 2209840u32);
    emu.mul_no_count(12usize, 12usize, 16usize, 2209844u32);
    emu.sri_no_count(12usize, 12usize, 17u32, 2209848u32);
    emu.mul_no_count(16usize, 12usize, 17usize, 2209852u32);
    emu.sbr_no_count(15usize, 15usize, 16usize, 2209856u32);
    emu.sli_no_count(15usize, 15usize, 17u32, 2209860u32);
    emu.sri_no_count(15usize, 15usize, 16u32, 2209864u32);
    emu.adr_no_count(15usize, 14usize, 15usize, 2209868u32);
    emu.lbu_no_count(16usize, 15usize, 0u32, 2209872u32);
    emu.lbu_no_count(15usize, 15usize, 1u32, 2209876u32);
    emu.adi_no_count(13usize, 13usize, 4294967294u32, 2209880u32);
    emu.adr_no_count(17usize, 11usize, 13usize, 2209884u32);
    emu.sb_no_count(16usize, 17usize, 0u32, 2209888u32);
    emu.sb_no_count(15usize, 5usize, 4294967295u32, 2209892u32);
    emu.adi_no_count(15usize, 12usize, 0u32, 2209896u32);
    emu.add_memory_rw_events(20usize);
    emu.pc = 2209896u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b868));
}
#[inline(always)]
pub fn block_0x0021b868(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2209924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b884));
    } else {
        emu.pc = 2209900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b86c));
    }
}
#[inline(always)]
pub fn block_0x0021b86c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2209924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b884));
    } else {
        emu.pc = 2209904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b870));
    }
}
#[inline(always)]
pub fn block_0x0021b870(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 13usize, 0u32, 2209908u32);
    emu.lw_no_count(8usize, 2usize, 12u32, 2209912u32)?;
    emu.lw_no_count(9usize, 2usize, 8u32, 2209916u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2209920u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2209924u32;
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
pub fn block_0x0021b884(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(15usize, 15usize, 1u32, 2209928u32);
    emu.ani_no_count(15usize, 15usize, 30u32, 2209932u32);
    emu.adr_no_count(14usize, 14usize, 15usize, 2209936u32);
    emu.lbu_no_count(10usize, 14usize, 1u32, 2209940u32);
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2209944u32);
    emu.adr_no_count(11usize, 11usize, 13usize, 2209948u32);
    emu.sb_no_count(10usize, 11usize, 0u32, 2209952u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2209956u32);
    emu.lw_no_count(8usize, 2usize, 12u32, 2209960u32)?;
    emu.lw_no_count(9usize, 2usize, 8u32, 2209964u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2209968u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2209972u32;
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
pub fn block_0x0021b8b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 10usize, 0u32, 2209976u32);
    emu.adi_no_count(13usize, 12usize, 0u32, 2209980u32);
    emu.adi_no_count(12usize, 0usize, 9u32, 2209984u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2209816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b818));
    } else {
        emu.pc = 2209988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b8c4));
    }
}
#[inline(always)]
pub fn block_0x0021b8c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2209992u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2209896u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b868));
}
#[inline]
pub fn block_0x0021b8c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2209996u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2210000u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2210004u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2210008u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2210012u32)?;
    emu.lw_no_count(14usize, 10usize, 0u32, 2210016u32)?;
    emu.lw_no_count(15usize, 10usize, 4u32, 2210020u32)?;
    emu.adi_no_count(8usize, 11usize, 0u32, 2210024u32);
    emu.adi_no_count(9usize, 2usize, 12u32, 2210028u32);
    emu.adi_no_count(12usize, 2usize, 12u32, 2210032u32);
    emu.adi_no_count(13usize, 0usize, 20u32, 2210036u32);
    emu.adi_no_count(18usize, 0usize, 20u32, 2210040u32);
    emu.adi_no_count(10usize, 14usize, 0u32, 2210044u32);
    emu.adi_no_count(11usize, 15usize, 0u32, 2210048u32);
    emu.apc_no_count(1usize, 2210048u32, 0u32, 2210052u32);
    emu.add_memory_rw_events(16usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2210056u32;
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
pub fn block_0x0021b908(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(15usize, 18usize, 10usize, 2210060u32);
    emu.adr_no_count(14usize, 9usize, 10usize, 2210064u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2210068u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2210072u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2210076u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2210080u32);
    emu.apc_no_count(1usize, 2210080u32, 8192u32, 2210084u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2210088u32;
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
pub fn block_0x0021b928(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2210092u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2210096u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2210100u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2210104u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2210108u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2210112u32;
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
pub fn block_0x0021b940(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2210116u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2210120u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2210124u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2210128u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2210132u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2210136u32)?;
    emu.sw_no_count(20usize, 2usize, 40u32, 2210140u32)?;
    emu.sw_no_count(21usize, 2usize, 36u32, 2210144u32)?;
    emu.sw_no_count(22usize, 2usize, 32u32, 2210148u32)?;
    emu.sw_no_count(23usize, 2usize, 28u32, 2210152u32)?;
    emu.sw_no_count(24usize, 2usize, 24u32, 2210156u32)?;
    emu.sw_no_count(25usize, 2usize, 20u32, 2210160u32)?;
    emu.sw_no_count(26usize, 2usize, 16u32, 2210164u32)?;
    emu.sw_no_count(27usize, 2usize, 12u32, 2210168u32)?;
    emu.adi_no_count(21usize, 13usize, 0u32, 2210172u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2210176u32);
    emu.adi_no_count(18usize, 10usize, 0u32, 2210180u32);
    emu.sltiu_no_count(10usize, 10usize, 1000u32, 2210184u32);
    emu.sltiu_no_count(11usize, 11usize, 1u32, 2210188u32);
    emu.anr_no_count(10usize, 11usize, 10usize, 2210192u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(24usize, a);
    emu.pc = 2210196u32;
    emu.update_insn_clock();
    emu.adi_no_count(24usize, 24usize, 4294965986u32, 2210200u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2210204u32)?;
    emu.add_memory_rw_events(23usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2210512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bad0));
    } else {
        emu.pc = 2210208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b9a0));
    }
}
#[inline]
pub fn block_0x0021b9a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 12usize, 4294967294u32, 2210212u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2210216u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2210220u32;
    emu.update_insn_clock();
    emu.adi_no_count(26usize, 0usize, 100u32, 2210224u32);
    let a = 0u32.wrapping_add(9998336u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2210228u32;
    emu.update_insn_clock();
    emu.adi_no_count(20usize, 10usize, 1808u32, 2210232u32);
    emu.adi_no_count(27usize, 11usize, 1147u32, 2210236u32);
    emu.adi_no_count(8usize, 12usize, 1663u32, 2210240u32);
    emu.adi_no_count(22usize, 18usize, 0u32, 2210244u32);
    emu.adi_no_count(23usize, 9usize, 0u32, 2210248u32);
    emu.add_memory_rw_events(10usize);
    emu.pc = 2210248u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b9c8));
}
#[inline(always)]
pub fn block_0x0021b9c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 21usize, 4294967292u32, 2210252u32);
    emu.adi_no_count(10usize, 22usize, 0u32, 2210256u32);
    emu.adi_no_count(11usize, 23usize, 0u32, 2210260u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2210264u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2210268u32);
    emu.apc_no_count(1usize, 2210268u32, 12288u32, 2210272u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2210276u32;
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
#[inline(never)]
pub fn block_0x0021b9e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 30u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.mul_no_count(12usize, 10usize, 20usize, 2210280u32);
    emu.adr_no_count(21usize, 25usize, 21usize, 2210284u32);
    emu.sltru_no_count(13usize, 8usize, 22usize, 2210288u32);
    emu.sltru_no_count(14usize, 0usize, 23usize, 2210292u32);
    emu.sbr_no_count(12usize, 22usize, 12usize, 2210296u32);
    emu.orr_no_count(13usize, 13usize, 14usize, 2210300u32);
    emu.sli_no_count(14usize, 12usize, 16u32, 2210304u32);
    emu.sri_no_count(14usize, 14usize, 18u32, 2210308u32);
    emu.mul_no_count(14usize, 14usize, 27usize, 2210312u32);
    emu.sri_no_count(15usize, 14usize, 16u32, 2210316u32);
    emu.sri_no_count(14usize, 14usize, 17u32, 2210320u32);
    emu.mul_no_count(14usize, 14usize, 26usize, 2210324u32);
    emu.ani_no_count(15usize, 15usize, 2046u32, 2210328u32);
    emu.sbr_no_count(12usize, 12usize, 14usize, 2210332u32);
    emu.adr_no_count(15usize, 24usize, 15usize, 2210336u32);
    emu.sli_no_count(12usize, 12usize, 17u32, 2210340u32);
    emu.sri_no_count(12usize, 12usize, 16u32, 2210344u32);
    emu.adr_no_count(12usize, 24usize, 12usize, 2210348u32);
    emu.lbu_no_count(14usize, 15usize, 0u32, 2210352u32);
    emu.lbu_no_count(15usize, 15usize, 1u32, 2210356u32);
    emu.lbu_no_count(16usize, 12usize, 0u32, 2210360u32);
    emu.lbu_no_count(12usize, 12usize, 1u32, 2210364u32);
    emu.sb_no_count(14usize, 21usize, 4294967294u32, 2210368u32);
    emu.sb_no_count(15usize, 21usize, 4294967295u32, 2210372u32);
    emu.sb_no_count(16usize, 21usize, 0u32, 2210376u32);
    emu.sb_no_count(12usize, 21usize, 1u32, 2210380u32);
    emu.adi_no_count(21usize, 19usize, 0u32, 2210384u32);
    emu.adi_no_count(22usize, 10usize, 0u32, 2210388u32);
    emu.adi_no_count(23usize, 11usize, 0u32, 2210392u32);
    emu.add_memory_rw_events(29usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2210248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b9c8));
    } else {
        emu.pc = 2210396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ba5c));
    }
}
#[inline(always)]
pub fn block_0x0021ba5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(12usize, 10usize, 10u32, 2210400u32);
    emu.sltiu_no_count(13usize, 11usize, 1u32, 2210404u32);
    emu.anr_no_count(12usize, 13usize, 12usize, 2210408u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2210540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021baec));
    } else {
        emu.pc = 2210412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ba6c));
    }
}
#[inline]
pub fn block_0x0021ba6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2210416u32);
    emu.sli_no_count(12usize, 10usize, 16u32, 2210420u32);
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2210424u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 0usize, 100u32, 2210428u32);
    emu.lw_no_count(16usize, 2usize, 8u32, 2210432u32)?;
    emu.adr_no_count(15usize, 16usize, 19usize, 2210436u32);
    emu.sri_no_count(12usize, 12usize, 18u32, 2210440u32);
    emu.adi_no_count(13usize, 13usize, 1147u32, 2210444u32);
    emu.mul_no_count(12usize, 12usize, 13usize, 2210448u32);
    emu.sri_no_count(12usize, 12usize, 17u32, 2210452u32);
    emu.mul_no_count(13usize, 12usize, 14usize, 2210456u32);
    emu.sbr_no_count(10usize, 10usize, 13usize, 2210460u32);
    emu.sli_no_count(10usize, 10usize, 17u32, 2210464u32);
    emu.sri_no_count(10usize, 10usize, 16u32, 2210468u32);
    emu.adr_no_count(10usize, 24usize, 10usize, 2210472u32);
    emu.lbu_no_count(13usize, 10usize, 0u32, 2210476u32);
    emu.lbu_no_count(10usize, 10usize, 1u32, 2210480u32);
    emu.adi_no_count(19usize, 19usize, 4294967294u32, 2210484u32);
    emu.adr_no_count(14usize, 16usize, 19usize, 2210488u32);
    emu.sb_no_count(13usize, 14usize, 0u32, 2210492u32);
    emu.sb_no_count(10usize, 15usize, 4294967295u32, 2210496u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2210500u32);
    emu.orr_no_count(12usize, 18usize, 9usize, 2210504u32);
    emu.add_memory_rw_events(23usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2210552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021baf8));
    } else {
        emu.pc = 2210508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bacc));
    }
}
#[inline(always)]
pub fn block_0x0021bacc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2210512u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2210560u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bb00));
}
#[inline(always)]
pub fn block_0x0021bad0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2210516u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2210520u32);
    emu.adi_no_count(19usize, 21usize, 0u32, 2210524u32);
    emu.sltiu_no_count(12usize, 18usize, 10u32, 2210528u32);
    emu.sltiu_no_count(13usize, 9usize, 1u32, 2210532u32);
    emu.anr_no_count(12usize, 13usize, 12usize, 2210536u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2210412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ba6c));
    } else {
        emu.pc = 2210540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021baec));
    }
}
#[inline(always)]
pub fn block_0x0021baec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 2usize, 8u32, 2210544u32)?;
    emu.orr_no_count(12usize, 18usize, 9usize, 2210548u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2210560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bb00));
    } else {
        emu.pc = 2210552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021baf8));
    }
}
#[inline(always)]
pub fn block_0x0021baf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.orr_no_count(11usize, 10usize, 11usize, 2210556u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2210588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bb1c));
    } else {
        emu.pc = 2210560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bb00));
    }
}
#[inline(always)]
pub fn block_0x0021bb00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 10usize, 1u32, 2210564u32);
    emu.ani_no_count(10usize, 10usize, 30u32, 2210568u32);
    emu.adr_no_count(10usize, 24usize, 10usize, 2210572u32);
    emu.lbu_no_count(10usize, 10usize, 1u32, 2210576u32);
    emu.adi_no_count(19usize, 19usize, 4294967295u32, 2210580u32);
    emu.adr_no_count(11usize, 16usize, 19usize, 2210584u32);
    emu.sb_no_count(10usize, 11usize, 0u32, 2210588u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2210588u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bb1c));
}
#[inline]
pub fn block_0x0021bb1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 19usize, 0u32, 2210592u32);
    emu.lw_no_count(1usize, 2usize, 60u32, 2210596u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2210600u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2210604u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2210608u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2210612u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2210616u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2210620u32)?;
    emu.lw_no_count(22usize, 2usize, 32u32, 2210624u32)?;
    emu.lw_no_count(23usize, 2usize, 28u32, 2210628u32)?;
    emu.lw_no_count(24usize, 2usize, 24u32, 2210632u32)?;
    emu.lw_no_count(25usize, 2usize, 20u32, 2210636u32)?;
    emu.lw_no_count(26usize, 2usize, 16u32, 2210640u32)?;
    emu.lw_no_count(27usize, 2usize, 12u32, 2210644u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2210648u32);
    emu.add_memory_rw_events(16usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2210652u32;
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
pub fn block_0x0021bb5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2210656u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2210660u32);
    emu.sw_no_count(10usize, 2usize, 4u32, 2210664u32)?;
    emu.sw_no_count(11usize, 2usize, 8u32, 2210668u32)?;
    emu.sh_no_count(12usize, 2usize, 12u32, 2210672u32)?;
    emu.adi_no_count(10usize, 2usize, 4u32, 2210676u32);
    emu.apc_no_count(1usize, 2210676u32, 4294959104u32, 2210680u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2210684u32;
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
pub fn block_0x0021bb7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2210688u32);
    emu.sw_no_count(10usize, 2usize, 24u32, 2210692u32)?;
    emu.sw_no_count(11usize, 2usize, 28u32, 2210696u32)?;
    emu.adi_no_count(10usize, 2usize, 24u32, 2210700u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2210704u32);
    emu.sw_no_count(0usize, 2usize, 16u32, 2210708u32)?;
    emu.adi_no_count(13usize, 0usize, 4u32, 2210712u32);
    emu.sw_no_count(10usize, 2usize, 0u32, 2210716u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2210720u32)?;
    emu.sw_no_count(13usize, 2usize, 8u32, 2210724u32)?;
    emu.sw_no_count(0usize, 2usize, 12u32, 2210728u32)?;
    emu.adi_no_count(10usize, 2usize, 0u32, 2210732u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2210736u32);
    emu.apc_no_count(1usize, 2210736u32, 0u32, 2210740u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2210744u32;
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
pub fn block_0x0021bbb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2210748u32);
    emu.sw_no_count(10usize, 2usize, 0u32, 2210752u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2210756u32)?;
    emu.adi_no_count(10usize, 2usize, 4u32, 2210760u32);
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2210764u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1064u32, 2210768u32);
    emu.adi_no_count(13usize, 2usize, 0u32, 2210772u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2210776u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966204u32, 2210780u32);
    emu.adi_no_count(15usize, 0usize, 2u32, 2210784u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2210788u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2210792u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2210796u32)?;
    emu.sw_no_count(13usize, 2usize, 40u32, 2210800u32)?;
    emu.sw_no_count(11usize, 2usize, 44u32, 2210804u32)?;
    emu.adi_no_count(10usize, 2usize, 32u32, 2210808u32);
    emu.sw_no_count(14usize, 2usize, 8u32, 2210812u32)?;
    emu.sw_no_count(15usize, 2usize, 12u32, 2210816u32)?;
    emu.sw_no_count(10usize, 2usize, 16u32, 2210820u32)?;
    emu.sw_no_count(15usize, 2usize, 20u32, 2210824u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2210828u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2210832u32);
    emu.apc_no_count(1usize, 2210832u32, 0u32, 2210836u32);
    emu.add_memory_rw_events(24usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2210840u32;
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
pub fn block_0x0021bc18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2210844u32);
    emu.adi_no_count(16usize, 14usize, 0u32, 2210848u32);
    emu.adi_no_count(15usize, 13usize, 0u32, 2210852u32);
    emu.sw_no_count(11usize, 2usize, 8u32, 2210856u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2210860u32)?;
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2210864u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966220u32, 2210868u32);
    emu.adi_no_count(11usize, 2usize, 8u32, 2210872u32);
    emu.adi_no_count(13usize, 2usize, 12u32, 2210876u32);
    emu.adi_no_count(14usize, 12usize, 0u32, 2210880u32);
    emu.apc_no_count(1usize, 2210880u32, 0u32, 2210884u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2210888u32;
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
#[inline]
pub fn block_0x0021bc48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967168u32, 2210892u32);
    emu.sw_no_count(1usize, 2usize, 124u32, 2210896u32)?;
    emu.sw_no_count(8usize, 2usize, 120u32, 2210900u32)?;
    emu.sw_no_count(9usize, 2usize, 116u32, 2210904u32)?;
    emu.adi_no_count(8usize, 16usize, 0u32, 2210908u32);
    emu.sw_no_count(11usize, 2usize, 12u32, 2210912u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2210916u32)?;
    emu.sli_no_count(10usize, 10usize, 2u32, 2210920u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2210924u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966544u32, 2210928u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2210932u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966556u32, 2210936u32);
    emu.adr_no_count(11usize, 11usize, 10usize, 2210940u32);
    emu.adr_no_count(10usize, 12usize, 10usize, 2210944u32);
    emu.lw_no_count(12usize, 15usize, 0u32, 2210948u32)?;
    emu.lw_no_count(11usize, 11usize, 0u32, 2210952u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2210956u32)?;
    emu.sw_no_count(13usize, 2usize, 20u32, 2210960u32)?;
    emu.sw_no_count(14usize, 2usize, 24u32, 2210964u32)?;
    emu.sw_no_count(11usize, 2usize, 28u32, 2210968u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2210972u32)?;
    emu.add_memory_rw_events(21usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2211068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bcfc));
    } else {
        emu.pc = 2210976u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bca0));
    }
}
#[inline]
pub fn block_0x0021bca0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 23u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 28u32, 2210980u32);
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2210984u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1328u32, 2210988u32);
    emu.adi_no_count(12usize, 2usize, 12u32, 2210992u32);
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2210996u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1312u32, 2211000u32);
    emu.adi_no_count(14usize, 2usize, 20u32, 2211004u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2211008u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 4294966280u32, 2211012u32);
    emu.adi_no_count(16usize, 0usize, 3u32, 2211016u32);
    emu.sw_no_count(0usize, 2usize, 108u32, 2211020u32)?;
    emu.sw_no_count(10usize, 2usize, 60u32, 2211024u32)?;
    emu.sw_no_count(11usize, 2usize, 64u32, 2211028u32)?;
    emu.sw_no_count(12usize, 2usize, 68u32, 2211032u32)?;
    emu.sw_no_count(13usize, 2usize, 72u32, 2211036u32)?;
    emu.sw_no_count(14usize, 2usize, 76u32, 2211040u32)?;
    emu.sw_no_count(13usize, 2usize, 80u32, 2211044u32)?;
    emu.adi_no_count(10usize, 2usize, 60u32, 2211048u32);
    emu.sw_no_count(15usize, 2usize, 92u32, 2211052u32)?;
    emu.sw_no_count(16usize, 2usize, 96u32, 2211056u32)?;
    emu.sw_no_count(10usize, 2usize, 100u32, 2211060u32)?;
    emu.sw_no_count(16usize, 2usize, 104u32, 2211064u32)?;
    emu.add_memory_rw_events(23usize);
    let return_addr = 2211068u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2211196u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bd7c));
}
#[inline(always)]
pub fn block_0x0021bcfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 36u32, 2211072u32);
    emu.adi_no_count(12usize, 0usize, 24u32, 2211076u32);
    emu.adi_no_count(9usize, 2usize, 36u32, 2211080u32);
    emu.adi_no_count(11usize, 15usize, 0u32, 2211084u32);
    emu.apc_no_count(1usize, 2211084u32, 4294893568u32, 2211088u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2211092u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966104u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0021bd14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 26u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 28u32, 2211096u32);
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2211100u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1328u32, 2211104u32);
    let a = 0u32.wrapping_add(2215936u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2211108u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 320u32, 2211112u32);
    emu.adi_no_count(13usize, 2usize, 12u32, 2211116u32);
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2211120u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1312u32, 2211124u32);
    emu.adi_no_count(15usize, 2usize, 20u32, 2211128u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2211132u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 4294966316u32, 2211136u32);
    emu.sw_no_count(10usize, 2usize, 60u32, 2211140u32)?;
    emu.sw_no_count(11usize, 2usize, 64u32, 2211144u32)?;
    emu.sw_no_count(9usize, 2usize, 68u32, 2211148u32)?;
    emu.sw_no_count(12usize, 2usize, 72u32, 2211152u32)?;
    emu.adi_no_count(10usize, 0usize, 4u32, 2211156u32);
    emu.sw_no_count(0usize, 2usize, 108u32, 2211160u32)?;
    emu.sw_no_count(13usize, 2usize, 76u32, 2211164u32)?;
    emu.sw_no_count(14usize, 2usize, 80u32, 2211168u32)?;
    emu.sw_no_count(15usize, 2usize, 84u32, 2211172u32)?;
    emu.sw_no_count(14usize, 2usize, 88u32, 2211176u32)?;
    emu.adi_no_count(11usize, 2usize, 60u32, 2211180u32);
    emu.sw_no_count(16usize, 2usize, 92u32, 2211184u32)?;
    emu.sw_no_count(10usize, 2usize, 96u32, 2211188u32)?;
    emu.sw_no_count(11usize, 2usize, 100u32, 2211192u32)?;
    emu.sw_no_count(10usize, 2usize, 104u32, 2211196u32)?;
    emu.add_memory_rw_events(26usize);
    emu.pc = 2211196u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bd7c));
}
#[inline(always)]
pub fn block_0x0021bd7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 92u32, 2211200u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2211204u32);
    emu.apc_no_count(1usize, 2211204u32, 0u32, 2211208u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2211212u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966744u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021bd8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2211212u32, 0u32, 2211216u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2211220u32;
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
pub fn block_0x0021bd94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967184u32, 2211224u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2211228u32)?;
    emu.adi_no_count(15usize, 0usize, 257u32, 2211232u32);
    emu.sw_no_count(13usize, 2usize, 12u32, 2211236u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2211260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bdbc));
    } else {
        emu.pc = 2211240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bda8));
    }
}
#[inline(always)]
pub fn block_0x0021bda8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 0u32, 2211244u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2211248u32)?;
    emu.sw_no_count(11usize, 2usize, 20u32, 2211252u32)?;
    emu.adi_no_count(15usize, 0usize, 1u32, 2211256u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2211260u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2211328u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021be00));
}
#[inline(always)]
pub fn block_0x0021bdbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 256u32, 2211264u32);
    emu.adi_no_count(16usize, 0usize, 4294967231u32, 2211268u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2211268u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bdc4));
}
#[inline(always)]
pub fn block_0x0021bdc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(17usize, 10usize, 15usize, 2211272u32);
    emu.lb_no_count(17usize, 17usize, 0u32, 2211276u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(16usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2211288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bdd8));
    } else {
        emu.pc = 2211280u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bdd0));
    }
}
#[inline(always)]
pub fn block_0x0021bdd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 15usize, 4294967295u32, 2211284u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2211268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bdc4));
    } else {
        emu.pc = 2211288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bdd8));
    }
}
#[inline(always)]
pub fn block_0x0021bdd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 16u32, 2211292u32)?;
    emu.sw_no_count(15usize, 2usize, 20u32, 2211296u32)?;
    emu.sltru_no_count(16usize, 15usize, 11usize, 2211300u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2211312u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bdf0));
    } else {
        emu.pc = 2211304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bde8));
    }
}
#[inline(always)]
pub fn block_0x0021bde8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 1u32, 2211308u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2211312u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2211320u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bdf8));
}
#[inline(always)]
pub fn block_0x0021bdf0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2211316u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 4294966348u32, 2211320u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2211320u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bdf8));
}
#[inline(always)]
pub fn block_0x0021bdf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(16usize, 0usize, 16usize, 2211324u32);
    emu.ani_no_count(16usize, 16usize, 5u32, 2211328u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2211328u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021be00));
}
#[inline(always)]
pub fn block_0x0021be00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(15usize, 2usize, 24u32, 2211332u32)?;
    emu.sw_no_count(16usize, 2usize, 28u32, 2211336u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2211776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bfc0));
    } else {
        emu.pc = 2211340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021be0c));
    }
}
#[inline(always)]
pub fn block_0x0021be0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2211772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bfbc));
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
        emu.pc = 2211884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c02c));
    } else {
        emu.pc = 2211348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021be14));
    }
}
#[inline(always)]
pub fn block_0x0021be14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2211376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021be30));
    } else {
        emu.pc = 2211352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021be18));
    }
}
#[inline(always)]
pub fn block_0x0021be18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2211376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021be30));
    } else {
        emu.pc = 2211356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021be1c));
    }
}
#[inline(always)]
pub fn block_0x0021be1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 10usize, 12usize, 2211360u32);
    emu.lb_no_count(15usize, 15usize, 0u32, 2211364u32);
    emu.adi_no_count(16usize, 0usize, 4294967231u32, 2211368u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(16usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2211376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021be30));
    } else {
        emu.pc = 2211372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021be2c));
    }
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
    emu.adi_no_count(13usize, 12usize, 0u32, 2211376u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2211376u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021be30));
}
#[inline(always)]
pub fn block_0x0021be30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(13usize, 2usize, 32u32, 2211380u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2211416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021be58));
    } else {
        emu.pc = 2211384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021be38));
    }
}
#[inline(always)]
pub fn block_0x0021be38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2211412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021be54));
    } else {
        emu.pc = 2211388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021be3c));
    }
}
#[inline(always)]
pub fn block_0x0021be3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 4294967231u32, 2211392u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2211392u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021be40));
}
#[inline(always)]
pub fn block_0x0021be40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 10usize, 13usize, 2211396u32);
    emu.lb_no_count(15usize, 15usize, 0u32, 2211400u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2211412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021be54));
    } else {
        emu.pc = 2211404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021be4c));
    }
}
#[inline(always)]
pub fn block_0x0021be4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2211408u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2211392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021be40));
    } else {
        emu.pc = 2211412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021be54));
    }
}
#[inline(always)]
pub fn block_0x0021be54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2211428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021be64));
    } else {
        emu.pc = 2211416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021be58));
    }
}
#[inline(always)]
pub fn block_0x0021be58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 14usize, 0u32, 2211420u32);
    emu.apc_no_count(1usize, 2211420u32, 4096u32, 2211424u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2211428u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966588u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021be64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 10usize, 13usize, 2211432u32);
    emu.lb_no_count(12usize, 10usize, 0u32, 2211436u32);
    emu.ani_no_count(11usize, 12usize, 255u32, 2211440u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2211456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021be80));
    } else {
        emu.pc = 2211444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021be74));
    }
}
#[inline(always)]
pub fn block_0x0021be74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 2usize, 36u32, 2211448u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2211452u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2211456u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2211616u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bf20));
}
#[inline(always)]
pub fn block_0x0021be80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(15usize, 10usize, 1u32, 2211460u32);
    emu.ani_no_count(12usize, 11usize, 31u32, 2211464u32);
    emu.adi_no_count(16usize, 0usize, 223u32, 2211468u32);
    emu.ani_no_count(15usize, 15usize, 63u32, 2211472u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a >= b {
        emu.pc = 2211540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bed4));
    } else {
        emu.pc = 2211476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021be94));
    }
}
#[inline(always)]
pub fn block_0x0021be94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(16usize, 10usize, 2u32, 2211480u32);
    emu.sli_no_count(15usize, 15usize, 6u32, 2211484u32);
    emu.ani_no_count(16usize, 16usize, 63u32, 2211488u32);
    emu.adi_no_count(17usize, 0usize, 240u32, 2211492u32);
    emu.orr_no_count(15usize, 15usize, 16usize, 2211496u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2211568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bef0));
    } else {
        emu.pc = 2211500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021beac));
    }
}
#[inline]
pub fn block_0x0021beac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 10usize, 3u32, 2211504u32);
    emu.sli_no_count(12usize, 12usize, 29u32, 2211508u32);
    emu.sli_no_count(15usize, 15usize, 6u32, 2211512u32);
    emu.sri_no_count(12usize, 12usize, 11u32, 2211516u32);
    emu.ani_no_count(10usize, 10usize, 63u32, 2211520u32);
    emu.orr_no_count(10usize, 15usize, 10usize, 2211524u32);
    emu.orr_no_count(10usize, 10usize, 12usize, 2211528u32);
    let a = 0u32.wrapping_add(1114112u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2211532u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2211416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021be58));
    } else {
        emu.pc = 2211536u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bed0));
    }
}
#[inline(always)]
pub fn block_0x0021bed0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2211540u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2211548u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bedc));
}
#[inline(always)]
pub fn block_0x0021bed4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 12usize, 6u32, 2211544u32);
    emu.orr_no_count(10usize, 10usize, 15usize, 2211548u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2211548u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bedc));
}
#[inline(always)]
pub fn block_0x0021bedc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 128u32, 2211552u32);
    emu.sw_no_count(10usize, 2usize, 36u32, 2211556u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2211588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bf04));
    } else {
        emu.pc = 2211560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bee8));
    }
}
#[inline(always)]
pub fn block_0x0021bee8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2211564u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2211568u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2211616u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bf20));
}
#[inline(always)]
pub fn block_0x0021bef0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 12usize, 12u32, 2211572u32);
    emu.orr_no_count(10usize, 15usize, 10usize, 2211576u32);
    emu.adi_no_count(11usize, 0usize, 128u32, 2211580u32);
    emu.sw_no_count(10usize, 2usize, 36u32, 2211584u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2211560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bee8));
    } else {
        emu.pc = 2211588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bf04));
    }
}
#[inline(always)]
pub fn block_0x0021bf04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 10usize, 11u32, 2211592u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2211604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bf14));
    } else {
        emu.pc = 2211596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bf0c));
    }
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
    emu.adi_no_count(10usize, 0usize, 2u32, 2211600u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2211604u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2211616u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bf20));
}
#[inline(always)]
pub fn block_0x0021bf14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 10usize, 16u32, 2211608u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2211612u32);
    emu.adi_no_count(10usize, 10usize, 3u32, 2211616u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2211616u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bf20));
}
#[inline(never)]
pub fn block_0x0021bf20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 39u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 10usize, 13usize, 2211620u32);
    emu.adi_no_count(11usize, 2usize, 32u32, 2211624u32);
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2211628u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1064u32, 2211632u32);
    emu.adi_no_count(15usize, 2usize, 36u32, 2211636u32);
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2211640u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 800u32, 2211644u32);
    emu.adi_no_count(17usize, 2usize, 40u32, 2211648u32);
    let a = 0u32.wrapping_add(2215936u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2211652u32;
    emu.update_insn_clock();
    emu.adi_no_count(5usize, 5usize, 4294965492u32, 2211656u32);
    emu.adi_no_count(6usize, 2usize, 16u32, 2211660u32);
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2211664u32;
    emu.update_insn_clock();
    emu.adi_no_count(7usize, 7usize, 1328u32, 2211668u32);
    emu.adi_no_count(28usize, 2usize, 24u32, 2211672u32);
    emu.sw_no_count(13usize, 2usize, 40u32, 2211676u32)?;
    emu.sw_no_count(10usize, 2usize, 44u32, 2211680u32)?;
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2211684u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966456u32, 2211688u32);
    emu.sw_no_count(11usize, 2usize, 72u32, 2211692u32)?;
    emu.sw_no_count(12usize, 2usize, 76u32, 2211696u32)?;
    emu.sw_no_count(15usize, 2usize, 80u32, 2211700u32)?;
    emu.sw_no_count(16usize, 2usize, 84u32, 2211704u32)?;
    emu.adi_no_count(11usize, 0usize, 5u32, 2211708u32);
    emu.sw_no_count(0usize, 2usize, 64u32, 2211712u32)?;
    emu.sw_no_count(17usize, 2usize, 88u32, 2211716u32)?;
    emu.sw_no_count(5usize, 2usize, 92u32, 2211720u32)?;
    emu.sw_no_count(6usize, 2usize, 96u32, 2211724u32)?;
    emu.sw_no_count(7usize, 2usize, 100u32, 2211728u32)?;
    emu.sw_no_count(28usize, 2usize, 104u32, 2211732u32)?;
    emu.sw_no_count(7usize, 2usize, 108u32, 2211736u32)?;
    emu.adi_no_count(12usize, 2usize, 72u32, 2211740u32);
    emu.sw_no_count(10usize, 2usize, 48u32, 2211744u32)?;
    emu.sw_no_count(11usize, 2usize, 52u32, 2211748u32)?;
    emu.sw_no_count(12usize, 2usize, 56u32, 2211752u32)?;
    emu.sw_no_count(11usize, 2usize, 60u32, 2211756u32)?;
    emu.adi_no_count(10usize, 2usize, 48u32, 2211760u32);
    emu.adi_no_count(11usize, 14usize, 0u32, 2211764u32);
    emu.apc_no_count(1usize, 2211764u32, 0u32, 2211768u32);
    emu.add_memory_rw_events(39usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2211772u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966184u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021bfbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 13usize, 0u32, 2211776u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2211776u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bfc0));
}
#[inline(never)]
pub fn block_0x0021bfc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(12usize, 2usize, 40u32, 2211780u32)?;
    emu.adi_no_count(10usize, 2usize, 40u32, 2211784u32);
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2211788u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1064u32, 2211792u32);
    emu.adi_no_count(12usize, 2usize, 16u32, 2211796u32);
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2211800u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1328u32, 2211804u32);
    emu.adi_no_count(15usize, 2usize, 24u32, 2211808u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2211812u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 4294966520u32, 2211816u32);
    emu.adi_no_count(17usize, 0usize, 3u32, 2211820u32);
    emu.sw_no_count(0usize, 2usize, 64u32, 2211824u32)?;
    emu.sw_no_count(10usize, 2usize, 72u32, 2211828u32)?;
    emu.sw_no_count(11usize, 2usize, 76u32, 2211832u32)?;
    emu.sw_no_count(12usize, 2usize, 80u32, 2211836u32)?;
    emu.sw_no_count(13usize, 2usize, 84u32, 2211840u32)?;
    emu.sw_no_count(15usize, 2usize, 88u32, 2211844u32)?;
    emu.sw_no_count(13usize, 2usize, 92u32, 2211848u32)?;
    emu.adi_no_count(10usize, 2usize, 72u32, 2211852u32);
    emu.sw_no_count(16usize, 2usize, 48u32, 2211856u32)?;
    emu.sw_no_count(17usize, 2usize, 52u32, 2211860u32)?;
    emu.sw_no_count(10usize, 2usize, 56u32, 2211864u32)?;
    emu.sw_no_count(17usize, 2usize, 60u32, 2211868u32)?;
    emu.adi_no_count(10usize, 2usize, 48u32, 2211872u32);
    emu.adi_no_count(11usize, 14usize, 0u32, 2211876u32);
    emu.apc_no_count(1usize, 2211876u32, 0u32, 2211880u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2211884u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966072u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0021c02c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 29u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 8u32, 2211888u32);
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2211892u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1064u32, 2211896u32);
    emu.adi_no_count(12usize, 2usize, 12u32, 2211900u32);
    emu.adi_no_count(13usize, 2usize, 16u32, 2211904u32);
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2211908u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 1328u32, 2211912u32);
    emu.adi_no_count(16usize, 2usize, 24u32, 2211916u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2211920u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 4294966368u32, 2211924u32);
    emu.sw_no_count(10usize, 2usize, 72u32, 2211928u32)?;
    emu.sw_no_count(11usize, 2usize, 76u32, 2211932u32)?;
    emu.sw_no_count(12usize, 2usize, 80u32, 2211936u32)?;
    emu.sw_no_count(11usize, 2usize, 84u32, 2211940u32)?;
    emu.adi_no_count(10usize, 0usize, 4u32, 2211944u32);
    emu.sw_no_count(0usize, 2usize, 64u32, 2211948u32)?;
    emu.sw_no_count(13usize, 2usize, 88u32, 2211952u32)?;
    emu.sw_no_count(15usize, 2usize, 92u32, 2211956u32)?;
    emu.sw_no_count(16usize, 2usize, 96u32, 2211960u32)?;
    emu.sw_no_count(15usize, 2usize, 100u32, 2211964u32)?;
    emu.adi_no_count(11usize, 2usize, 72u32, 2211968u32);
    emu.sw_no_count(17usize, 2usize, 48u32, 2211972u32)?;
    emu.sw_no_count(10usize, 2usize, 52u32, 2211976u32)?;
    emu.sw_no_count(11usize, 2usize, 56u32, 2211980u32)?;
    emu.sw_no_count(10usize, 2usize, 60u32, 2211984u32)?;
    emu.adi_no_count(10usize, 2usize, 48u32, 2211988u32);
    emu.adi_no_count(11usize, 14usize, 0u32, 2211992u32);
    emu.apc_no_count(1usize, 2211992u32, 0u32, 2211996u32);
    emu.add_memory_rw_events(29usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2212000u32;
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
