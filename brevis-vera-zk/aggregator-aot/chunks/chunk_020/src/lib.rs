pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2198404u32;
pub const PC_MAX: u32 = 2201232u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 114usize] = [
        block_0x00218b84,
        block_0x00218b8c,
        block_0x00218bc8,
        block_0x00218be0,
        block_0x00218c10,
        block_0x00218c2c,
        block_0x00218c34,
        block_0x00218c60,
        block_0x00218c78,
        block_0x00218c8c,
        block_0x00218c9c,
        block_0x00218cb4,
        block_0x00218cbc,
        block_0x00218cdc,
        block_0x00218ce8,
        block_0x00218cfc,
        block_0x00218d10,
        block_0x00218d20,
        block_0x00218d50,
        block_0x00218d5c,
        block_0x00218d64,
        block_0x00218d6c,
        block_0x00218d78,
        block_0x00218d80,
        block_0x00218da0,
        block_0x00218dcc,
        block_0x00218de4,
        block_0x00218df0,
        block_0x00218e0c,
        block_0x00218e2c,
        block_0x00218e3c,
        block_0x00218e48,
        block_0x00218e4c,
        block_0x00218e64,
        block_0x00218e6c,
        block_0x00218e74,
        block_0x00218e88,
        block_0x00218eb0,
        block_0x00218ebc,
        block_0x00218ec4,
        block_0x00218ecc,
        block_0x00218ee0,
        block_0x00218f14,
        block_0x00218f20,
        block_0x00218f38,
        block_0x00218f4c,
        block_0x00218f54,
        block_0x00218f5c,
        block_0x00218f64,
        block_0x00218f70,
        block_0x00218f94,
        block_0x00218fb0,
        block_0x00219010,
        block_0x00219034,
        block_0x00219064,
        block_0x00219074,
        block_0x00219078,
        block_0x002190a8,
        block_0x002190b8,
        block_0x002190c4,
        block_0x00219134,
        block_0x00219168,
        block_0x00219174,
        block_0x00219180,
        block_0x002191d4,
        block_0x002191e0,
        block_0x002191f8,
        block_0x00219218,
        block_0x00219228,
        block_0x0021922c,
        block_0x00219250,
        block_0x00219260,
        block_0x0021926c,
        block_0x0021927c,
        block_0x00219294,
        block_0x002192ac,
        block_0x002192b0,
        block_0x002192b4,
        block_0x002192c0,
        block_0x002192c4,
        block_0x002192d0,
        block_0x002192fc,
        block_0x0021932c,
        block_0x00219350,
        block_0x002193a0,
        block_0x002193ac,
        block_0x002193b8,
        block_0x00219408,
        block_0x00219410,
        block_0x00219430,
        block_0x00219450,
        block_0x00219498,
        block_0x002194a4,
        block_0x002194b0,
        block_0x002194cc,
        block_0x002194d0,
        block_0x002194f0,
        block_0x002194f4,
        block_0x002194fc,
        block_0x00219508,
        block_0x00219570,
        block_0x002195bc,
        block_0x002195d4,
        block_0x002195e0,
        block_0x002195e4,
        block_0x00219618,
        block_0x00219630,
        block_0x00219634,
        block_0x00219644,
        block_0x0021964c,
        block_0x0021967c,
        block_0x00219684,
        block_0x0021968c,
        block_0x00219690,
    ];
    const IDX: [u16; 708usize] = [
        1u16, 0u16, 2u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 3u16, 0u16, 0u16, 0u16, 0u16, 0u16, 4u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 5u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 6u16, 0u16, 7u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 8u16, 0u16, 0u16, 0u16, 0u16, 0u16, 9u16, 0u16, 0u16, 0u16,
        0u16, 10u16, 0u16, 0u16, 0u16, 11u16, 0u16, 0u16, 0u16, 0u16, 0u16, 12u16, 0u16,
        13u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 14u16, 0u16, 0u16, 15u16, 0u16,
        0u16, 0u16, 0u16, 16u16, 0u16, 0u16, 0u16, 0u16, 17u16, 0u16, 0u16, 0u16, 18u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 19u16, 0u16,
        0u16, 20u16, 0u16, 21u16, 0u16, 22u16, 0u16, 0u16, 23u16, 0u16, 24u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 25u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 26u16, 0u16, 0u16, 0u16, 0u16, 0u16, 27u16, 0u16, 0u16,
        28u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 29u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 30u16, 0u16, 0u16, 0u16, 31u16, 0u16, 0u16, 32u16, 33u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 34u16, 0u16, 35u16, 0u16, 36u16, 0u16, 0u16, 0u16, 0u16, 37u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 38u16, 0u16, 0u16, 39u16,
        0u16, 40u16, 0u16, 41u16, 0u16, 0u16, 0u16, 0u16, 42u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 43u16, 0u16, 0u16, 44u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 45u16, 0u16, 0u16, 0u16, 0u16, 46u16, 0u16, 47u16, 0u16,
        48u16, 0u16, 49u16, 0u16, 0u16, 50u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 51u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 52u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 53u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 54u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 55u16, 0u16, 0u16, 0u16, 56u16, 57u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 58u16, 0u16, 0u16, 0u16, 59u16, 0u16, 0u16, 60u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 61u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 62u16, 0u16, 0u16, 63u16, 0u16, 0u16, 64u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 65u16, 0u16, 0u16, 66u16, 0u16, 0u16, 0u16, 0u16, 0u16, 67u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 68u16, 0u16, 0u16, 0u16, 69u16, 70u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 71u16, 0u16, 0u16, 0u16, 72u16, 0u16,
        0u16, 73u16, 0u16, 0u16, 0u16, 74u16, 0u16, 0u16, 0u16, 0u16, 0u16, 75u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 76u16, 77u16, 78u16, 0u16, 0u16, 79u16, 80u16, 0u16,
        0u16, 81u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 82u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 83u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 84u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 85u16, 0u16, 0u16, 86u16, 0u16, 0u16, 87u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 88u16, 0u16, 89u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 90u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 91u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 92u16, 0u16,
        0u16, 93u16, 0u16, 0u16, 94u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 95u16, 96u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 97u16, 98u16, 0u16, 99u16, 0u16, 0u16,
        100u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        101u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 102u16, 0u16, 0u16, 0u16, 0u16, 0u16, 103u16,
        0u16, 0u16, 104u16, 105u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 106u16, 0u16, 0u16, 0u16, 0u16, 0u16, 107u16, 108u16, 0u16,
        0u16, 0u16, 109u16, 0u16, 110u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 111u16, 0u16, 112u16, 0u16, 113u16, 114u16,
    ];
    if pc < 2198404u32 || pc > 2201232u32 {
        return None;
    }
    let word_offset = ((pc - 2198404u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x00218b84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2198408u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2198412u32;
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
pub fn block_0x00218b8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2198416u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2198420u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2198424u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2198428u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2198432u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2198436u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2198440u32)?;
    emu.adi_no_count(9usize, 13usize, 0u32, 2198444u32);
    emu.adi_no_count(19usize, 12usize, 0u32, 2198448u32);
    emu.adi_no_count(8usize, 11usize, 0u32, 2198452u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2198456u32)?;
    emu.lw_no_count(20usize, 8usize, 8u32, 2198460u32)?;
    emu.sbr_no_count(11usize, 11usize, 20usize, 2198464u32);
    emu.adi_no_count(18usize, 10usize, 0u32, 2198468u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2198544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218c10));
    } else {
        emu.pc = 2198472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218bc8));
    }
}
#[inline(always)]
pub fn block_0x00218bc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 4u32, 2198476u32)?;
    emu.adr_no_count(10usize, 10usize, 20usize, 2198480u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2198484u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2198488u32);
    emu.apc_no_count(1usize, 2198488u32, 4294901760u32, 2198492u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2198496u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(328u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00218be0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(9usize, 20usize, 9usize, 2198500u32);
    emu.adi_no_count(10usize, 0usize, 4u32, 2198504u32);
    emu.sw_no_count(9usize, 8usize, 8u32, 2198508u32)?;
    emu.sb_no_count(10usize, 18usize, 0u32, 2198512u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2198516u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2198520u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2198524u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2198528u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2198532u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2198536u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2198540u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2198544u32;
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
pub fn block_0x00218c10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2198548u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2198552u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2198556u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2198560u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2198564u32);
    emu.apc_no_count(1usize, 2198564u32, 4294963200u32, 2198568u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2198572u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(356u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218c2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(20usize, 8usize, 8u32, 2198576u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2198580u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2198472u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218bc8));
}
#[inline]
pub fn block_0x00218c34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2198584u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2198588u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2198592u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2198596u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2198600u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2198604u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2198608u32)?;
    emu.sw_no_count(21usize, 2usize, 4u32, 2198612u32)?;
    emu.sw_no_count(22usize, 2usize, 0u32, 2198616u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2198620u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2198816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218d20));
    } else {
        emu.pc = 2198624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218c60));
    }
}
#[inline(always)]
pub fn block_0x00218c60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 12usize, 0u32, 2198628u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2198632u32);
    emu.adi_no_count(12usize, 0usize, 0u32, 2198636u32);
    emu.sli_no_count(21usize, 13usize, 3u32, 2198640u32);
    emu.adr_no_count(21usize, 9usize, 21usize, 2198644u32);
    emu.adi_no_count(10usize, 9usize, 4u32, 2198648u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2198648u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218c78));
}
#[inline(always)]
pub fn block_0x00218c78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2198652u32)?;
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2198656u32);
    emu.adr_no_count(12usize, 11usize, 12usize, 2198660u32);
    emu.adi_no_count(10usize, 10usize, 8u32, 2198664u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2198648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218c78));
    } else {
        emu.pc = 2198668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218c8c));
    }
}
#[inline(always)]
pub fn block_0x00218c8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 0u32, 2198672u32)?;
    emu.lw_no_count(19usize, 18usize, 8u32, 2198676u32)?;
    emu.sbr_no_count(10usize, 10usize, 19usize, 2198680u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2198760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218ce8));
    } else {
        emu.pc = 2198684u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218c9c));
    }
}
#[inline(always)]
pub fn block_0x00218c9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2198688u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2198692u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2198696u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2198700u32);
    emu.apc_no_count(1usize, 2198700u32, 4294963200u32, 2198704u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2198708u32;
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
pub fn block_0x00218cb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(19usize, 18usize, 8u32, 2198712u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2198716u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2198760u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218ce8));
}
#[inline(always)]
pub fn block_0x00218cbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2198720u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2198724u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2198728u32);
    emu.adi_no_count(22usize, 11usize, 0u32, 2198732u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2198736u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2198740u32);
    emu.apc_no_count(1usize, 2198740u32, 4294963200u32, 2198744u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2198748u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(180u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218cdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 22usize, 0u32, 2198752u32);
    emu.lw_no_count(19usize, 18usize, 8u32, 2198756u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2198760u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2198780u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218cfc));
}
#[inline(always)]
pub fn block_0x00218ce8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 0u32, 2198764u32)?;
    emu.lw_no_count(20usize, 9usize, 4u32, 2198768u32)?;
    emu.lw_no_count(11usize, 9usize, 0u32, 2198772u32)?;
    emu.sbr_no_count(10usize, 10usize, 19usize, 2198776u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2198716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218cbc));
    } else {
        emu.pc = 2198780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218cfc));
    }
}
#[inline(always)]
pub fn block_0x00218cfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 4u32, 2198784u32)?;
    emu.adr_no_count(10usize, 10usize, 19usize, 2198788u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2198792u32);
    emu.apc_no_count(1usize, 2198792u32, 4294901760u32, 2198796u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2198800u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(24u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218d10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(19usize, 19usize, 20usize, 2198804u32);
    emu.adi_no_count(9usize, 9usize, 8u32, 2198808u32);
    emu.sw_no_count(19usize, 18usize, 8u32, 2198812u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2198760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218ce8));
    } else {
        emu.pc = 2198816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218d20));
    }
}
#[inline]
pub fn block_0x00218d20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2198820u32);
    emu.sb_no_count(10usize, 8usize, 0u32, 2198824u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2198828u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2198832u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2198836u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2198840u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2198844u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2198848u32)?;
    emu.lw_no_count(21usize, 2usize, 4u32, 2198852u32)?;
    emu.lw_no_count(22usize, 2usize, 0u32, 2198856u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2198860u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2198864u32;
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
pub fn block_0x00218d50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 4u32, 2198868u32);
    emu.sb_no_count(11usize, 10usize, 0u32, 2198872u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2198876u32;
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
pub fn block_0x00218d5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2198880u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2198884u32;
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
pub fn block_0x00218d64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(5usize, 2198884u32, 4096u32, 2198888u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(5usize);
    let return_addr = 2198892u32;
    emu.write_reg_no_count(5usize, return_addr);
    let target = base.wrapping_add(4294965720u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218d6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 10usize, 0u32, 2198896u32);
    emu.lbu_no_count(10usize, 10usize, 13u32, 2198900u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2198912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218d80));
    } else {
        emu.pc = 2198904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218d78));
    }
}
#[inline(always)]
pub fn block_0x00218d78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2198904u32, 4096u32, 2198908u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2198912u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(968u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218d80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 8usize, 8u32, 2198916u32)?;
    emu.lw_no_count(10usize, 8usize, 0u32, 2198920u32)?;
    emu.lw_no_count(11usize, 8usize, 4u32, 2198924u32)?;
    emu.adi_no_count(13usize, 0usize, 3u32, 2198928u32);
    emu.sb_no_count(13usize, 2usize, 7u32, 2198932u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2198936u32)?;
    emu.apc_no_count(1usize, 2198936u32, 0u32, 2198940u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2198944u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1428u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00218da0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 12u32, 2198948u32)?;
    emu.sw_no_count(11usize, 2usize, 16u32, 2198952u32)?;
    emu.adi_no_count(11usize, 2usize, 8u32, 2198956u32);
    emu.adi_no_count(12usize, 2usize, 12u32, 2198960u32);
    let a = 0u32.wrapping_add(2244608u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2198964u32;
    emu.update_insn_clock();
    emu.lbu_no_count(13usize, 10usize, 888u32, 2198968u32);
    emu.adi_no_count(14usize, 2usize, 7u32, 2198972u32);
    emu.sw_no_count(11usize, 2usize, 20u32, 2198976u32)?;
    emu.sw_no_count(12usize, 2usize, 24u32, 2198980u32)?;
    emu.sw_no_count(14usize, 2usize, 28u32, 2198984u32)?;
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2199116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218e4c));
    } else {
        emu.pc = 2198988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218dcc));
    }
}
#[inline(always)]
pub fn block_0x00218dcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2244608u32);
    emu.write_reg_no_count(18usize, a);
    emu.pc = 2198992u32;
    emu.update_insn_clock();
    emu.lw_no_count(19usize, 18usize, 892u32, 2198996u32)?;
    emu.adi_no_count(9usize, 0usize, 1u32, 2199000u32);
    emu.sb_no_count(9usize, 10usize, 888u32, 2199004u32);
    emu.sw_no_count(0usize, 18usize, 892u32, 2199008u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2199116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218e4c));
    } else {
        emu.pc = 2199012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218de4));
    }
}
#[inline(always)]
pub fn block_0x00218de4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 19usize, 8u32, 2199016u32);
    emu.apc_no_count(1usize, 2199016u32, 4096u32, 2199020u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199024u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967140u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218df0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 11usize, 0u32, 2199028u32);
    emu.adi_no_count(11usize, 11usize, 4u32, 2199032u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2199036u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967128u32, 2199040u32);
    emu.adi_no_count(10usize, 2usize, 20u32, 2199044u32);
    emu.apc_no_count(1usize, 2199044u32, 0u32, 2199048u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199052u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(104u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218e0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(0usize, 8usize, 0u32, 2199056u32);
    let a = 0u32.wrapping_add(2244608u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2199060u32;
    emu.update_insn_clock();
    emu.lw_no_count(10usize, 18usize, 892u32, 2199064u32)?;
    emu.sb_no_count(9usize, 11usize, 888u32, 2199068u32);
    emu.sw_no_count(19usize, 18usize, 892u32, 2199072u32)?;
    emu.sw_no_count(9usize, 2usize, 32u32, 2199076u32)?;
    emu.sw_no_count(10usize, 2usize, 36u32, 2199080u32)?;
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2199140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218e64));
    } else {
        emu.pc = 2199084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218e2c));
    }
}
#[inline(always)]
pub fn block_0x00218e2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2199088u32)?;
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2199092u32);
    emu.sw_no_count(11usize, 10usize, 0u32, 2199096u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2199140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218e64));
    } else {
        emu.pc = 2199100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218e3c));
    }
}
#[inline(always)]
pub fn block_0x00218e3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 36u32, 2199104u32);
    emu.apc_no_count(1usize, 2199104u32, 4096u32, 2199108u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199112u32;
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
pub fn block_0x00218e48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2199116u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2199140u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218e64));
}
#[inline(always)]
pub fn block_0x00218e4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2199120u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967168u32, 2199124u32);
    emu.adi_no_count(10usize, 2usize, 20u32, 2199128u32);
    emu.adi_no_count(11usize, 2usize, 43u32, 2199132u32);
    emu.apc_no_count(1usize, 2199132u32, 0u32, 2199136u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199140u32;
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
pub fn block_0x00218e64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2199140u32, 4096u32, 2199144u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2199148u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965436u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218e6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(5usize, 2199148u32, 4096u32, 2199152u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(5usize);
    let return_addr = 2199156u32;
    emu.write_reg_no_count(5usize, return_addr);
    let target = base.wrapping_add(4294965456u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218e74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 12usize, 0u32, 2199160u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2199164u32);
    emu.adi_no_count(19usize, 10usize, 0u32, 2199168u32);
    emu.apc_no_count(1usize, 2199168u32, 4096u32, 2199172u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199176u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966156u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00218e88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 10usize, 0u32, 2199180u32);
    emu.lw_no_count(10usize, 19usize, 0u32, 2199184u32)?;
    emu.lw_no_count(11usize, 19usize, 4u32, 2199188u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2199192u32)?;
    emu.sw_no_count(11usize, 2usize, 24u32, 2199196u32)?;
    emu.sw_no_count(9usize, 2usize, 28u32, 2199200u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2199204u32)?;
    emu.adi_no_count(10usize, 2usize, 20u32, 2199208u32);
    emu.apc_no_count(1usize, 2199208u32, 4096u32, 2199212u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199216u32;
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
pub fn block_0x00218eb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 19usize, 8u32, 2199220u32)?;
    emu.lbu_no_count(10usize, 10usize, 0u32, 2199224u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2199352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218f38));
    } else {
        emu.pc = 2199228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218ebc));
    }
}
#[inline(always)]
pub fn block_0x00218ebc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2199232u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2199328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218f20));
    } else {
        emu.pc = 2199236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218ec4));
    }
}
#[inline(always)]
pub fn block_0x00218ec4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 2u32, 2199240u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2199396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218f64));
    } else {
        emu.pc = 2199244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218ecc));
    }
}
#[inline(always)]
pub fn block_0x00218ecc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2244608u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2199248u32;
    emu.update_insn_clock();
    emu.lbu_no_count(12usize, 11usize, 912u32, 2199252u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2199256u32);
    emu.sb_no_count(10usize, 11usize, 912u32, 2199260u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2199396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218f64));
    } else {
        emu.pc = 2199264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218ee0));
    }
}
#[inline]
pub fn block_0x00218ee0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2199268u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967288u32, 2199272u32);
    emu.sw_no_count(0usize, 2usize, 36u32, 2199276u32)?;
    emu.adi_no_count(12usize, 0usize, 4u32, 2199280u32);
    emu.lw_no_count(13usize, 18usize, 36u32, 2199284u32)?;
    emu.sw_no_count(11usize, 2usize, 20u32, 2199288u32)?;
    emu.sw_no_count(10usize, 2usize, 24u32, 2199292u32)?;
    emu.sw_no_count(12usize, 2usize, 28u32, 2199296u32)?;
    emu.sw_no_count(0usize, 2usize, 32u32, 2199300u32)?;
    emu.adi_no_count(10usize, 2usize, 12u32, 2199304u32);
    emu.adi_no_count(12usize, 2usize, 20u32, 2199308u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2199312u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2199316u32;
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
pub fn block_0x00218f14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 12u32, 2199320u32);
    emu.lw_no_count(11usize, 2usize, 16u32, 2199324u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2199328u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2199388u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218f5c));
}
#[inline(always)]
pub fn block_0x00218f20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 20u32, 2199332u32);
    emu.adi_no_count(11usize, 2usize, 12u32, 2199336u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2199340u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2199344u32);
    emu.adi_no_count(13usize, 18usize, 0u32, 2199348u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2199352u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2199372u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218f4c));
}
#[inline(always)]
pub fn block_0x00218f38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 20u32, 2199356u32);
    emu.adi_no_count(11usize, 2usize, 12u32, 2199360u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2199364u32);
    emu.adi_no_count(13usize, 18usize, 0u32, 2199368u32);
    emu.adi_no_count(14usize, 0usize, 0u32, 2199372u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2199372u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218f4c));
}
#[inline(always)]
pub fn block_0x00218f4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2199372u32, 4096u32, 2199376u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199380u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966064u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218f54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 20u32, 2199384u32);
    emu.lw_no_count(11usize, 2usize, 24u32, 2199388u32)?;
    emu.add_memory_rw_events(2usize);
    emu.pc = 2199388u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218f5c));
}
#[inline(always)]
pub fn block_0x00218f5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2199388u32, 4294963200u32, 2199392u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199396u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1976u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218f64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(0usize, 8usize, 0u32, 2199400u32);
    emu.apc_no_count(6usize, 2199400u32, 0u32, 2199404u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2199408u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1976u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00218f70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2199412u32);
    emu.lw_no_count(11usize, 10usize, 0u32, 2199416u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2199420u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2199424u32)?;
    emu.sw_no_count(12usize, 2usize, 8u32, 2199428u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2199432u32)?;
    emu.adi_no_count(10usize, 2usize, 4u32, 2199436u32);
    emu.apc_no_count(1usize, 2199436u32, 0u32, 2199440u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199444u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(776u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218f94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2199448u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2199452u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2199456u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2199460u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2199464u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2199468u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2199604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219034));
    } else {
        emu.pc = 2199472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218fb0));
    }
}
#[inline]
pub fn block_0x00218fb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 12u32, 2199476u32)?;
    emu.adi_no_count(11usize, 0usize, 1u32, 2199480u32);
    emu.sw_no_count(0usize, 2usize, 20u32, 2199484u32)?;
    emu.sw_no_count(11usize, 2usize, 24u32, 2199488u32)?;
    emu.sw_no_count(0usize, 2usize, 28u32, 2199492u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2199496u32)?;
    emu.lw_no_count(11usize, 10usize, 0u32, 2199500u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2199504u32)?;
    emu.lw_no_count(13usize, 10usize, 8u32, 2199508u32)?;
    emu.lw_no_count(14usize, 10usize, 12u32, 2199512u32)?;
    emu.lw_no_count(15usize, 10usize, 16u32, 2199516u32)?;
    emu.lw_no_count(10usize, 10usize, 20u32, 2199520u32)?;
    emu.sw_no_count(11usize, 2usize, 32u32, 2199524u32)?;
    emu.sw_no_count(12usize, 2usize, 36u32, 2199528u32)?;
    emu.sw_no_count(13usize, 2usize, 40u32, 2199532u32)?;
    emu.sw_no_count(14usize, 2usize, 44u32, 2199536u32)?;
    emu.sw_no_count(15usize, 2usize, 48u32, 2199540u32)?;
    emu.sw_no_count(10usize, 2usize, 52u32, 2199544u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2199548u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 0u32, 2199552u32);
    emu.adi_no_count(10usize, 2usize, 20u32, 2199556u32);
    emu.adi_no_count(12usize, 2usize, 32u32, 2199560u32);
    emu.apc_no_count(1usize, 2199560u32, 16384u32, 2199564u32);
    emu.add_memory_rw_events(24usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199568u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966236u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00219010(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 20u32, 2199572u32)?;
    emu.lw_no_count(11usize, 2usize, 24u32, 2199576u32)?;
    emu.lw_no_count(12usize, 2usize, 28u32, 2199580u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2199584u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2199588u32)?;
    emu.sw_no_count(12usize, 2usize, 40u32, 2199592u32)?;
    emu.sw_no_count(10usize, 8usize, 0u32, 2199596u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2199600u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2199604u32)?;
    emu.add_memory_rw_events(9usize);
    emu.pc = 2199604u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219034));
}
#[inline]
pub fn block_0x00219034(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 0u32, 2199608u32)?;
    emu.lw_no_count(11usize, 8usize, 4u32, 2199612u32)?;
    emu.lw_no_count(12usize, 8usize, 8u32, 2199616u32)?;
    emu.sw_no_count(10usize, 2usize, 8u32, 2199620u32)?;
    emu.sw_no_count(11usize, 2usize, 12u32, 2199624u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2199628u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2199632u32);
    emu.sw_no_count(0usize, 8usize, 0u32, 2199636u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2199640u32)?;
    emu.sw_no_count(0usize, 8usize, 8u32, 2199644u32)?;
    emu.apc_no_count(1usize, 2199644u32, 4294897664u32, 2199648u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199652u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(868u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00219064(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 12u32, 2199656u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2199660u32);
    emu.apc_no_count(1usize, 2199660u32, 4294897664u32, 2199664u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199668u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(676u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00219074(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2199720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002190a8));
    } else {
        emu.pc = 2199672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219078));
    }
}
#[inline]
pub fn block_0x00219078(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 2usize, 8u32, 2199676u32)?;
    emu.lw_no_count(13usize, 2usize, 12u32, 2199680u32)?;
    emu.lw_no_count(14usize, 2usize, 16u32, 2199684u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2199688u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 24u32, 2199692u32);
    emu.sw_no_count(12usize, 10usize, 0u32, 2199696u32)?;
    emu.sw_no_count(13usize, 10usize, 4u32, 2199700u32)?;
    emu.sw_no_count(14usize, 10usize, 8u32, 2199704u32)?;
    emu.lw_no_count(1usize, 2usize, 60u32, 2199708u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2199712u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2199716u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199720u32;
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
pub fn block_0x002190a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2199724u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2199728u32);
    emu.apc_no_count(1usize, 2199728u32, 4096u32, 2199732u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199736u32;
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
pub fn block_0x002190b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2199740u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2199744u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2199912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219168));
    } else {
        emu.pc = 2199748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002190c4));
    }
}
#[inline(never)]
pub fn block_0x002190c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2199752u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2199756u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2199760u32)?;
    emu.lw_no_count(11usize, 10usize, 12u32, 2199764u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2199768u32);
    emu.sw_no_count(0usize, 2usize, 4u32, 2199772u32)?;
    emu.sw_no_count(12usize, 2usize, 8u32, 2199776u32)?;
    emu.sw_no_count(0usize, 2usize, 12u32, 2199780u32)?;
    emu.lw_no_count(11usize, 11usize, 0u32, 2199784u32)?;
    emu.lw_no_count(12usize, 11usize, 0u32, 2199788u32)?;
    emu.lw_no_count(13usize, 11usize, 4u32, 2199792u32)?;
    emu.lw_no_count(14usize, 11usize, 8u32, 2199796u32)?;
    emu.lw_no_count(15usize, 11usize, 12u32, 2199800u32)?;
    emu.lw_no_count(16usize, 11usize, 16u32, 2199804u32)?;
    emu.lw_no_count(11usize, 11usize, 20u32, 2199808u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2199812u32)?;
    emu.sw_no_count(13usize, 2usize, 20u32, 2199816u32)?;
    emu.sw_no_count(14usize, 2usize, 24u32, 2199820u32)?;
    emu.sw_no_count(15usize, 2usize, 28u32, 2199824u32)?;
    emu.sw_no_count(16usize, 2usize, 32u32, 2199828u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2199832u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2199836u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 0u32, 2199840u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2199844u32);
    emu.adi_no_count(10usize, 2usize, 4u32, 2199848u32);
    emu.adi_no_count(12usize, 2usize, 16u32, 2199852u32);
    emu.apc_no_count(1usize, 2199852u32, 16384u32, 2199856u32);
    emu.add_memory_rw_events(28usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199860u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965944u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00219134(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 0u32, 2199864u32);
    emu.lw_no_count(11usize, 2usize, 4u32, 2199868u32)?;
    emu.lw_no_count(12usize, 2usize, 8u32, 2199872u32)?;
    emu.lw_no_count(13usize, 2usize, 12u32, 2199876u32)?;
    emu.sw_no_count(11usize, 2usize, 16u32, 2199880u32)?;
    emu.sw_no_count(12usize, 2usize, 20u32, 2199884u32)?;
    emu.sw_no_count(13usize, 2usize, 24u32, 2199888u32)?;
    emu.sw_no_count(11usize, 8usize, 0u32, 2199892u32)?;
    emu.sw_no_count(12usize, 8usize, 4u32, 2199896u32)?;
    emu.sw_no_count(13usize, 8usize, 8u32, 2199900u32)?;
    emu.lw_no_count(1usize, 2usize, 44u32, 2199904u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2199908u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2199912u32);
    emu.add_memory_rw_events(13usize);
    emu.pc = 2199912u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219168));
}
#[inline(always)]
pub fn block_0x00219168(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2199916u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 24u32, 2199920u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199924u32;
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
pub fn block_0x00219174(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2199928u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2199932u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2200032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002191e0));
    } else {
        emu.pc = 2199936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219180));
    }
}
#[inline]
pub fn block_0x00219180(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2199940u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2199944u32)?;
    emu.lw_no_count(10usize, 10usize, 12u32, 2199948u32)?;
    emu.lw_no_count(12usize, 10usize, 0u32, 2199952u32)?;
    emu.lw_no_count(10usize, 11usize, 0u32, 2199956u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2199960u32)?;
    emu.lw_no_count(13usize, 12usize, 0u32, 2199964u32)?;
    emu.lw_no_count(14usize, 12usize, 4u32, 2199968u32)?;
    emu.lw_no_count(15usize, 12usize, 8u32, 2199972u32)?;
    emu.lw_no_count(16usize, 12usize, 12u32, 2199976u32)?;
    emu.lw_no_count(17usize, 12usize, 16u32, 2199980u32)?;
    emu.lw_no_count(12usize, 12usize, 20u32, 2199984u32)?;
    emu.sw_no_count(13usize, 2usize, 4u32, 2199988u32)?;
    emu.sw_no_count(14usize, 2usize, 8u32, 2199992u32)?;
    emu.sw_no_count(15usize, 2usize, 12u32, 2199996u32)?;
    emu.sw_no_count(16usize, 2usize, 16u32, 2200000u32)?;
    emu.sw_no_count(17usize, 2usize, 20u32, 2200004u32)?;
    emu.sw_no_count(12usize, 2usize, 24u32, 2200008u32)?;
    emu.adi_no_count(12usize, 2usize, 4u32, 2200012u32);
    emu.apc_no_count(1usize, 2200012u32, 16384u32, 2200016u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200020u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965784u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002191d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2200024u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2200028u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200032u32;
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
pub fn block_0x002191e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 4u32, 2200036u32)?;
    emu.lw_no_count(12usize, 10usize, 8u32, 2200040u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2200044u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2200048u32);
    emu.apc_no_count(6usize, 2200048u32, 16384u32, 2200052u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2200056u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(760u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002191f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2200060u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2200064u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2200068u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2200072u32)?;
    emu.lw_no_count(8usize, 10usize, 0u32, 2200076u32)?;
    emu.lw_no_count(9usize, 10usize, 4u32, 2200080u32)?;
    emu.apc_no_count(1usize, 2200080u32, 4294897664u32, 2200084u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200088u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(432u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00219218(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 8u32, 2200092u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2200096u32);
    emu.apc_no_count(1usize, 2200096u32, 4294897664u32, 2200100u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200104u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(240u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00219228(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2200144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219250));
    } else {
        emu.pc = 2200108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021922c));
    }
}
#[inline]
pub fn block_0x0021922c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2200112u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 40u32, 2200116u32);
    emu.sw_no_count(8usize, 10usize, 0u32, 2200120u32)?;
    emu.sw_no_count(9usize, 10usize, 4u32, 2200124u32)?;
    emu.lw_no_count(1usize, 2usize, 12u32, 2200128u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2200132u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2200136u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2200140u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200144u32;
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
pub fn block_0x00219250(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2200148u32);
    emu.adi_no_count(11usize, 0usize, 8u32, 2200152u32);
    emu.apc_no_count(1usize, 2200152u32, 4096u32, 2200156u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200160u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(380u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00219260(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2200164u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 40u32, 2200168u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200172u32;
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
pub fn block_0x0021926c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2200176u32)?;
    emu.lw_no_count(11usize, 10usize, 4u32, 2200180u32)?;
    emu.adi_no_count(10usize, 12usize, 0u32, 2200184u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200188u32;
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
pub fn block_0x0021927c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 0u32, 2200192u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2200196u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2200200u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2200204u32);
    emu.apc_no_count(6usize, 2200204u32, 16384u32, 2200208u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2200212u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(604u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00219294(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2200216u32);
    emu.lw_no_count(11usize, 10usize, 0u32, 2200220u32)?;
    emu.lw_no_count(13usize, 11usize, 4u32, 2200224u32)?;
    emu.lw_no_count(12usize, 11usize, 12u32, 2200228u32)?;
    emu.adi_no_count(14usize, 0usize, 1u32, 2200232u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2200256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002192c0));
    } else {
        emu.pc = 2200236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002192ac));
    }
}
#[inline(always)]
pub fn block_0x002192ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2200316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002192fc));
    } else {
        emu.pc = 2200240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002192b0));
    }
}
#[inline(always)]
pub fn block_0x002192b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2200316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002192fc));
    } else {
        emu.pc = 2200244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002192b4));
    }
}
#[inline(always)]
pub fn block_0x002192b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2200248u32);
    emu.adi_no_count(15usize, 0usize, 1u32, 2200252u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2200256u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2200272u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002192d0));
}
#[inline(always)]
pub fn block_0x002192c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2200316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002192fc));
    } else {
        emu.pc = 2200260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002192c4));
    }
}
#[inline(always)]
pub fn block_0x002192c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 11usize, 0u32, 2200264u32)?;
    emu.lw_no_count(15usize, 11usize, 0u32, 2200268u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2200272u32)?;
    emu.add_memory_rw_events(3usize);
    emu.pc = 2200272u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002192d0));
}
#[inline]
pub fn block_0x002192d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 10usize, 8u32, 2200276u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2200280u32)?;
    emu.lbu_no_count(13usize, 14usize, 8u32, 2200284u32);
    emu.lbu_no_count(14usize, 14usize, 9u32, 2200288u32);
    emu.sw_no_count(15usize, 2usize, 0u32, 2200292u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2200296u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2200300u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 56u32, 2200304u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2200308u32);
    emu.apc_no_count(1usize, 2200308u32, 0u32, 2200312u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200316u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(348u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002192fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 8u32, 2200320u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2200324u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2200328u32)?;
    emu.lbu_no_count(13usize, 11usize, 8u32, 2200332u32);
    emu.lbu_no_count(14usize, 11usize, 9u32, 2200336u32);
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2200340u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 2usize, 0u32, 2200344u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2200348u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 84u32, 2200352u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2200356u32);
    emu.apc_no_count(1usize, 2200356u32, 0u32, 2200360u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200364u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(300u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021932c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2200368u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2200372u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2200376u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2200380u32)?;
    emu.lw_no_count(9usize, 11usize, 12u32, 2200384u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2200388u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2200392u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2200396u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(9usize);
    let return_addr = 2200400u32;
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
pub fn block_0x00219350(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2200404u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2200408u32)?;
    emu.lw_no_count(12usize, 2usize, 0u32, 2200412u32)?;
    emu.lw_no_count(13usize, 2usize, 4u32, 2200416u32)?;
    let a = 0u32.wrapping_add(3112902656u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2200420u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966129u32, 2200424u32);
    emu.xrr_no_count(11usize, 11usize, 14usize, 2200428u32);
    let a = 0u32.wrapping_add(1676365824u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2200432u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 44u32, 2200436u32);
    emu.xrr_no_count(13usize, 13usize, 14usize, 2200440u32);
    let a = 0u32.wrapping_add(1470513152u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2200444u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 376u32, 2200448u32);
    emu.xrr_no_count(10usize, 10usize, 14usize, 2200452u32);
    let a = 0u32.wrapping_add(3603652608u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2200456u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966637u32, 2200460u32);
    emu.xrr_no_count(12usize, 12usize, 14usize, 2200464u32);
    emu.orr_no_count(11usize, 13usize, 11usize, 2200468u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2200472u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2200476u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2200492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002193ac));
    } else {
        emu.pc = 2200480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002193a0));
    }
}
#[inline(always)]
pub fn block_0x002193a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 4u32, 2200484u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2200488u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2200492u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2200592u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219410));
}
#[inline(always)]
pub fn block_0x002193ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2200496u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2200500u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(9usize);
    let return_addr = 2200504u32;
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
pub fn block_0x002193b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2200508u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2200512u32)?;
    emu.lw_no_count(12usize, 2usize, 0u32, 2200516u32)?;
    emu.lw_no_count(13usize, 2usize, 4u32, 2200520u32)?;
    let a = 0u32.wrapping_add(2330587136u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2200524u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 428u32, 2200528u32);
    emu.xrr_no_count(11usize, 11usize, 14usize, 2200532u32);
    let a = 0u32.wrapping_add(2965131264u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2200536u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965685u32, 2200540u32);
    emu.xrr_no_count(13usize, 13usize, 14usize, 2200544u32);
    let a = 0u32.wrapping_add(4112453632u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2200548u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1473u32, 2200552u32);
    emu.xrr_no_count(10usize, 10usize, 14usize, 2200556u32);
    let a = 0u32.wrapping_add(2020298752u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2200560u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965580u32, 2200564u32);
    emu.xrr_no_count(12usize, 12usize, 14usize, 2200568u32);
    emu.orr_no_count(11usize, 13usize, 11usize, 2200572u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2200576u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2200580u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2200624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219430));
    } else {
        emu.pc = 2200584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219408));
    }
}
#[inline(always)]
pub fn block_0x00219408(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 4u32, 2200588u32);
    emu.adi_no_count(11usize, 0usize, 8u32, 2200592u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2200592u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219410));
}
#[inline(always)]
pub fn block_0x00219410(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2200596u32)?;
    emu.adr_no_count(11usize, 8usize, 11usize, 2200600u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2200604u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2200608u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2200612u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2200616u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2200620u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200624u32;
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
pub fn block_0x00219430(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2200628u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 112u32, 2200632u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2200636u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2200640u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2200644u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2200648u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2200652u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200656u32;
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
pub fn block_0x00219450(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967200u32, 2200660u32);
    emu.sw_no_count(1usize, 2usize, 92u32, 2200664u32)?;
    emu.sw_no_count(8usize, 2usize, 88u32, 2200668u32)?;
    emu.sw_no_count(9usize, 2usize, 84u32, 2200672u32)?;
    emu.sw_no_count(18usize, 2usize, 80u32, 2200676u32)?;
    emu.sw_no_count(19usize, 2usize, 76u32, 2200680u32)?;
    emu.sw_no_count(20usize, 2usize, 72u32, 2200684u32)?;
    emu.adi_no_count(19usize, 14usize, 0u32, 2200688u32);
    emu.adi_no_count(18usize, 13usize, 0u32, 2200692u32);
    emu.adi_no_count(20usize, 12usize, 0u32, 2200696u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2200700u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2200704u32);
    emu.sw_no_count(10usize, 2usize, 4u32, 2200708u32)?;
    emu.sw_no_count(11usize, 2usize, 8u32, 2200712u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2200716u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2200720u32);
    emu.apc_no_count(1usize, 2200720u32, 4096u32, 2200724u32);
    emu.add_memory_rw_events(18usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200728u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966360u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00219498(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 255u32, 2200732u32);
    emu.adi_no_count(11usize, 0usize, 2u32, 2200736u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2200820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002194f4));
    } else {
        emu.pc = 2200740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002194a4));
    }
}
#[inline(always)]
pub fn block_0x002194a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2244608u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2200744u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 10usize, 900u32, 2200748u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2201232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219690));
    } else {
        emu.pc = 2200752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002194b0));
    }
}
#[inline(always)]
pub fn block_0x002194b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 900u32, 2200756u32);
    emu.lw_no_count(12usize, 12usize, 4u32, 2200760u32)?;
    emu.adi_no_count(11usize, 11usize, 1u32, 2200764u32);
    emu.sw_no_count(11usize, 10usize, 900u32, 2200768u32)?;
    emu.lw_no_count(11usize, 9usize, 20u32, 2200772u32)?;
    emu.adi_no_count(10usize, 8usize, 0u32, 2200776u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2201056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002195e0));
    } else {
        emu.pc = 2200780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002194cc));
    }
}
#[inline(always)]
pub fn block_0x002194cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2200784u32;
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
pub fn block_0x002194d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 48u32, 2200788u32)?;
    emu.sw_no_count(11usize, 2usize, 52u32, 2200792u32)?;
    emu.sw_no_count(20usize, 2usize, 56u32, 2200796u32)?;
    emu.sb_no_count(18usize, 2usize, 60u32, 2200800u32);
    emu.sb_no_count(19usize, 2usize, 61u32, 2200804u32);
    emu.adi_no_count(10usize, 2usize, 48u32, 2200808u32);
    emu.apc_no_count(1usize, 2200808u32, 0u32, 2200812u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200816u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965372u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002194f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2200820u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2201112u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219618));
}
#[inline(always)]
pub fn block_0x002194f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 1u32, 2200824u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2200944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219570));
    } else {
        emu.pc = 2200828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002194fc));
    }
}
#[inline(always)]
pub fn block_0x002194fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 9usize, 24u32, 2200832u32)?;
    emu.adi_no_count(10usize, 8usize, 0u32, 2200836u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2200840u32;
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
#[inline(never)]
pub fn block_0x00219508(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 26u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(12usize, 10usize, 1u32, 2200844u32);
    emu.adi_no_count(13usize, 2usize, 12u32, 2200848u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2200852u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966960u32, 2200856u32);
    emu.adi_no_count(15usize, 2usize, 16u32, 2200860u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2200864u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 4294966248u32, 2200868u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2200872u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 240u32, 2200876u32);
    emu.sw_no_count(13usize, 2usize, 32u32, 2200880u32)?;
    emu.sw_no_count(14usize, 2usize, 36u32, 2200884u32)?;
    emu.sw_no_count(15usize, 2usize, 40u32, 2200888u32)?;
    emu.sw_no_count(16usize, 2usize, 44u32, 2200892u32)?;
    emu.adi_no_count(13usize, 0usize, 3u32, 2200896u32);
    emu.sw_no_count(0usize, 2usize, 64u32, 2200900u32)?;
    emu.adr_no_count(10usize, 10usize, 12usize, 2200904u32);
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2200908u32);
    emu.anr_no_count(11usize, 12usize, 11usize, 2200912u32);
    emu.adi_no_count(12usize, 2usize, 32u32, 2200916u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2200920u32)?;
    emu.sw_no_count(11usize, 2usize, 20u32, 2200924u32)?;
    emu.adi_no_count(10usize, 0usize, 2u32, 2200928u32);
    emu.sw_no_count(17usize, 2usize, 48u32, 2200932u32)?;
    emu.sw_no_count(13usize, 2usize, 52u32, 2200936u32)?;
    emu.sw_no_count(12usize, 2usize, 56u32, 2200940u32)?;
    emu.add_memory_rw_events(26usize);
    let return_addr = 2200944u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2201020u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002195bc));
}
#[inline]
pub fn block_0x00219570(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2200948u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2200952u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966960u32, 2200956u32);
    emu.adi_no_count(12usize, 2usize, 4u32, 2200960u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2200964u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966712u32, 2200968u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2200972u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 152u32, 2200976u32);
    emu.adi_no_count(15usize, 0usize, 3u32, 2200980u32);
    emu.sw_no_count(0usize, 2usize, 64u32, 2200984u32)?;
    emu.adi_no_count(16usize, 2usize, 32u32, 2200988u32);
    emu.sw_no_count(10usize, 2usize, 32u32, 2200992u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2200996u32)?;
    emu.sw_no_count(12usize, 2usize, 40u32, 2201000u32)?;
    emu.sw_no_count(13usize, 2usize, 44u32, 2201004u32)?;
    emu.adi_no_count(10usize, 0usize, 2u32, 2201008u32);
    emu.sw_no_count(14usize, 2usize, 48u32, 2201012u32)?;
    emu.sw_no_count(15usize, 2usize, 52u32, 2201016u32)?;
    emu.sw_no_count(16usize, 2usize, 56u32, 2201020u32)?;
    emu.add_memory_rw_events(19usize);
    emu.pc = 2201020u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002195bc));
}
#[inline(always)]
pub fn block_0x002195bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 60u32, 2201024u32)?;
    emu.adi_no_count(10usize, 2usize, 24u32, 2201028u32);
    emu.adi_no_count(11usize, 2usize, 24u32, 2201032u32);
    emu.adi_no_count(12usize, 2usize, 48u32, 2201036u32);
    emu.apc_no_count(1usize, 2201036u32, 0u32, 2201040u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2201044u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1060u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002195d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 24u32, 2201048u32);
    emu.lw_no_count(11usize, 2usize, 28u32, 2201052u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2201056u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2201220u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219684));
}
#[inline(always)]
pub fn block_0x002195e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2201060u32;
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
pub fn block_0x002195e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2244608u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2201064u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 900u32, 2201068u32);
    emu.lw_no_count(13usize, 12usize, 8u32, 2201072u32)?;
    emu.lw_no_count(12usize, 12usize, 4u32, 2201076u32)?;
    emu.lw_no_count(13usize, 13usize, 20u32, 2201080u32)?;
    emu.sw_no_count(10usize, 2usize, 48u32, 2201084u32)?;
    emu.sw_no_count(11usize, 2usize, 52u32, 2201088u32)?;
    emu.sw_no_count(20usize, 2usize, 56u32, 2201092u32)?;
    emu.sb_no_count(18usize, 2usize, 60u32, 2201096u32);
    emu.sb_no_count(19usize, 2usize, 61u32, 2201100u32);
    emu.adi_no_count(11usize, 2usize, 48u32, 2201104u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2201108u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2201112u32;
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
pub fn block_0x00219618(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2244608u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2201116u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 10usize, 900u32, 2201120u32)?;
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2201124u32);
    emu.sw_no_count(11usize, 10usize, 900u32, 2201128u32)?;
    emu.apc_no_count(1usize, 2201128u32, 4096u32, 2201132u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2201136u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966028u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00219630(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2201156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219644));
    } else {
        emu.pc = 2201140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219634));
    }
}
#[inline(always)]
pub fn block_0x00219634(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 0u32, 2201144u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2201148u32);
    emu.apc_no_count(1usize, 2201148u32, 0u32, 2201152u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2201156u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(96u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00219644(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2201160u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 312u32, 2201164u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2201164u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021964c));
}
#[inline]
pub fn block_0x0021964c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2201168u32);
    emu.sw_no_count(0usize, 2usize, 64u32, 2201172u32)?;
    emu.adi_no_count(12usize, 0usize, 4u32, 2201176u32);
    emu.sw_no_count(10usize, 2usize, 48u32, 2201180u32)?;
    emu.sw_no_count(11usize, 2usize, 52u32, 2201184u32)?;
    emu.sw_no_count(12usize, 2usize, 56u32, 2201188u32)?;
    emu.sw_no_count(0usize, 2usize, 60u32, 2201192u32)?;
    emu.adi_no_count(10usize, 2usize, 32u32, 2201196u32);
    emu.adi_no_count(11usize, 2usize, 24u32, 2201200u32);
    emu.adi_no_count(12usize, 2usize, 48u32, 2201204u32);
    emu.apc_no_count(1usize, 2201204u32, 0u32, 2201208u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2201212u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(892u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021967c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 32u32, 2201216u32);
    emu.lw_no_count(11usize, 2usize, 36u32, 2201220u32)?;
    emu.add_memory_rw_events(2usize);
    emu.pc = 2201220u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219684));
}
#[inline(always)]
pub fn block_0x00219684(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2201220u32, 4294963200u32, 2201224u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2201228u32;
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
pub fn block_0x0021968c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.update_insn_clock();
    return Err(format!("Unimplemented instruction (UNIMP) at PC {:#x}", 2201228u32));
}
#[inline(always)]
pub fn block_0x00219690(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2201236u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 460u32, 2201240u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2201244u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2201164u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021964c));
}
