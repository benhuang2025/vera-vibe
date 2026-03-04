pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2112324u32;
pub const PC_MAX: u32 = 2114712u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 107usize] = [
        block_0x00203b44,
        block_0x00203b50,
        block_0x00203b6c,
        block_0x00203ba0,
        block_0x00203bac,
        block_0x00203bc8,
        block_0x00203be4,
        block_0x00203c2c,
        block_0x00203c60,
        block_0x00203c6c,
        block_0x00203c88,
        block_0x00203ca4,
        block_0x00203cd8,
        block_0x00203ce4,
        block_0x00203d00,
        block_0x00203d48,
        block_0x00203d64,
        block_0x00203d98,
        block_0x00203da4,
        block_0x00203dc0,
        block_0x00203ddc,
        block_0x00203df8,
        block_0x00203e20,
        block_0x00203e2c,
        block_0x00203e70,
        block_0x00203e84,
        block_0x00203e90,
        block_0x00203ec4,
        block_0x00203ed0,
        block_0x00203ee4,
        block_0x00203ef8,
        block_0x00203f24,
        block_0x00203f30,
        block_0x00203f6c,
        block_0x00203f78,
        block_0x00203f98,
        block_0x00203fa0,
        block_0x00203fc0,
        block_0x00203fcc,
        block_0x00203fec,
        block_0x00203ff4,
        block_0x00204004,
        block_0x00204008,
        block_0x00204018,
        block_0x0020401c,
        block_0x0020402c,
        block_0x00204030,
        block_0x00204040,
        block_0x00204044,
        block_0x00204054,
        block_0x00204058,
        block_0x00204068,
        block_0x0020406c,
        block_0x0020407c,
        block_0x00204080,
        block_0x00204090,
        block_0x00204094,
        block_0x002040a4,
        block_0x002040a8,
        block_0x002040b8,
        block_0x002040bc,
        block_0x002040cc,
        block_0x002040d0,
        block_0x002040e0,
        block_0x002040e4,
        block_0x002040f4,
        block_0x002040f8,
        block_0x00204108,
        block_0x0020410c,
        block_0x0020411c,
        block_0x00204120,
        block_0x00204130,
        block_0x00204134,
        block_0x00204144,
        block_0x00204148,
        block_0x00204158,
        block_0x0020415c,
        block_0x0020416c,
        block_0x00204170,
        block_0x00204180,
        block_0x00204184,
        block_0x00204194,
        block_0x00204198,
        block_0x002041a8,
        block_0x002041ac,
        block_0x002041bc,
        block_0x002041c0,
        block_0x002041d0,
        block_0x002041d4,
        block_0x002041e4,
        block_0x002041e8,
        block_0x002041f8,
        block_0x002041fc,
        block_0x0020420c,
        block_0x00204210,
        block_0x00204234,
        block_0x00204250,
        block_0x00204268,
        block_0x00204270,
        block_0x002042e0,
        block_0x002042e8,
        block_0x00204300,
        block_0x00204304,
        block_0x00204328,
        block_0x00204338,
        block_0x00204404,
        block_0x00204498,
    ];
    const IDX: [u16; 598usize] = [
        1u16, 0u16, 0u16, 2u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 3u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 4u16, 0u16, 0u16,
        5u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 6u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 7u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 8u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 9u16, 0u16, 0u16, 10u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 11u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 12u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 13u16, 0u16, 0u16,
        14u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 15u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 16u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 17u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 18u16, 0u16, 0u16, 19u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 20u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 21u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 22u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 23u16, 0u16, 0u16, 24u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 25u16, 0u16, 0u16, 0u16, 0u16,
        26u16, 0u16, 0u16, 27u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 28u16, 0u16, 0u16, 29u16, 0u16, 0u16, 0u16, 0u16, 30u16, 0u16,
        0u16, 0u16, 0u16, 31u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 32u16, 0u16, 0u16, 33u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 34u16, 0u16, 0u16, 35u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 36u16, 0u16, 37u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 38u16, 0u16, 0u16, 39u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 40u16,
        0u16, 41u16, 0u16, 0u16, 0u16, 42u16, 43u16, 0u16, 0u16, 0u16, 44u16, 45u16,
        0u16, 0u16, 0u16, 46u16, 47u16, 0u16, 0u16, 0u16, 48u16, 49u16, 0u16, 0u16, 0u16,
        50u16, 51u16, 0u16, 0u16, 0u16, 52u16, 53u16, 0u16, 0u16, 0u16, 54u16, 55u16,
        0u16, 0u16, 0u16, 56u16, 57u16, 0u16, 0u16, 0u16, 58u16, 59u16, 0u16, 0u16, 0u16,
        60u16, 61u16, 0u16, 0u16, 0u16, 62u16, 63u16, 0u16, 0u16, 0u16, 64u16, 65u16,
        0u16, 0u16, 0u16, 66u16, 67u16, 0u16, 0u16, 0u16, 68u16, 69u16, 0u16, 0u16, 0u16,
        70u16, 71u16, 0u16, 0u16, 0u16, 72u16, 73u16, 0u16, 0u16, 0u16, 74u16, 75u16,
        0u16, 0u16, 0u16, 76u16, 77u16, 0u16, 0u16, 0u16, 78u16, 79u16, 0u16, 0u16, 0u16,
        80u16, 81u16, 0u16, 0u16, 0u16, 82u16, 83u16, 0u16, 0u16, 0u16, 84u16, 85u16,
        0u16, 0u16, 0u16, 86u16, 87u16, 0u16, 0u16, 0u16, 88u16, 89u16, 0u16, 0u16, 0u16,
        90u16, 91u16, 0u16, 0u16, 0u16, 92u16, 93u16, 0u16, 0u16, 0u16, 94u16, 95u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 96u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 97u16, 0u16, 0u16, 0u16, 0u16, 0u16, 98u16, 0u16, 99u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 100u16,
        0u16, 101u16, 0u16, 0u16, 0u16, 0u16, 0u16, 102u16, 103u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 104u16, 0u16, 0u16, 0u16, 105u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 106u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 107u16,
    ];
    if pc < 2112324u32 || pc > 2114712u32 {
        return None;
    }
    let word_offset = ((pc - 2112324u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x00203b44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2112328u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2112332u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112336u32;
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
pub fn block_0x00203b50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2112340u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966148u32, 2112344u32);
    emu.adi_no_count(12usize, 0usize, 8u32, 2112348u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2112352u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2112356u32);
    emu.apc_no_count(6usize, 2112356u32, 106496u32, 2112360u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2112364u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965948u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00203b6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 1u32, 2112368u32);
    emu.sw_no_count(12usize, 2usize, 24u32, 2112372u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2112376u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966412u32, 2112380u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2112384u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966422u32, 2112388u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2112392u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 4294966396u32, 2112396u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2112400u32);
    emu.adi_no_count(14usize, 0usize, 3u32, 2112404u32);
    emu.adi_no_count(15usize, 2usize, 24u32, 2112408u32);
    emu.apc_no_count(1usize, 2112408u32, 106496u32, 2112412u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112416u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965996u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203ba0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2112420u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2112424u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112428u32;
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
pub fn block_0x00203bac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2112432u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966256u32, 2112436u32);
    emu.adi_no_count(12usize, 0usize, 12u32, 2112440u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2112444u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2112448u32);
    emu.apc_no_count(6usize, 2112448u32, 106496u32, 2112452u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2112456u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965856u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203bc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2112460u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965822u32, 2112464u32);
    emu.adi_no_count(12usize, 0usize, 16u32, 2112468u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2112472u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2112476u32);
    emu.apc_no_count(6usize, 2112476u32, 106496u32, 2112480u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2112484u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965828u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00203be4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 12usize, 4u32, 2112488u32);
    emu.adi_no_count(12usize, 12usize, 8u32, 2112492u32);
    emu.sw_no_count(12usize, 2usize, 24u32, 2112496u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2112500u32;
    emu.update_insn_clock();
    emu.adi_no_count(5usize, 5usize, 4294966284u32, 2112504u32);
    emu.adi_no_count(6usize, 2usize, 24u32, 2112508u32);
    emu.adi_no_count(7usize, 0usize, 10u32, 2112512u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2112516u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966300u32, 2112520u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2112524u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966310u32, 2112528u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2112532u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 4294966268u32, 2112536u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2112540u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 4294966322u32, 2112544u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2112548u32);
    emu.adi_no_count(14usize, 0usize, 12u32, 2112552u32);
    emu.add_memory_rw_events(18usize);
    let return_addr = 2112556u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2113136u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203e70));
}
#[inline]
pub fn block_0x00203c2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 1u32, 2112560u32);
    emu.sw_no_count(12usize, 2usize, 24u32, 2112564u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2112568u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966371u32, 2112572u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2112576u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966368u32, 2112580u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2112584u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 4294966352u32, 2112588u32);
    emu.adi_no_count(12usize, 0usize, 12u32, 2112592u32);
    emu.adi_no_count(14usize, 0usize, 3u32, 2112596u32);
    emu.adi_no_count(15usize, 2usize, 24u32, 2112600u32);
    emu.apc_no_count(1usize, 2112600u32, 106496u32, 2112604u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112608u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965804u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203c60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2112612u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2112616u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112620u32;
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
pub fn block_0x00203c6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2112624u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966250u32, 2112628u32);
    emu.adi_no_count(12usize, 0usize, 6u32, 2112632u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2112636u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2112640u32);
    emu.apc_no_count(6usize, 2112640u32, 106496u32, 2112644u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2112648u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965664u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203c88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2112652u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966485u32, 2112656u32);
    emu.adi_no_count(12usize, 0usize, 14u32, 2112660u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2112664u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2112668u32);
    emu.apc_no_count(6usize, 2112668u32, 106496u32, 2112672u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2112676u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x00203ca4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 1u32, 2112680u32);
    emu.sw_no_count(12usize, 2usize, 24u32, 2112684u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2112688u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966552u32, 2112692u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2112696u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966100u32, 2112700u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2112704u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 4294966536u32, 2112708u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2112712u32);
    emu.adi_no_count(14usize, 0usize, 4u32, 2112716u32);
    emu.adi_no_count(15usize, 2usize, 24u32, 2112720u32);
    emu.apc_no_count(1usize, 2112720u32, 106496u32, 2112724u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112728u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965684u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203cd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2112732u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2112736u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112740u32;
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
pub fn block_0x00203ce4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2112744u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965838u32, 2112748u32);
    emu.adi_no_count(12usize, 0usize, 16u32, 2112752u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2112756u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2112760u32);
    emu.apc_no_count(6usize, 2112760u32, 106496u32, 2112764u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2112768u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965544u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00203d00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 12usize, 4u32, 2112772u32);
    emu.adi_no_count(12usize, 12usize, 8u32, 2112776u32);
    emu.sw_no_count(12usize, 2usize, 24u32, 2112780u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2112784u32;
    emu.update_insn_clock();
    emu.adi_no_count(5usize, 5usize, 4294966284u32, 2112788u32);
    emu.adi_no_count(6usize, 2usize, 24u32, 2112792u32);
    emu.adi_no_count(7usize, 0usize, 9u32, 2112796u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2112800u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966562u32, 2112804u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2112808u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966574u32, 2112812u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2112816u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 4294966268u32, 2112820u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2112824u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 4294966581u32, 2112828u32);
    emu.adi_no_count(12usize, 0usize, 12u32, 2112832u32);
    emu.adi_no_count(14usize, 0usize, 7u32, 2112836u32);
    emu.add_memory_rw_events(18usize);
    let return_addr = 2112840u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2113136u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203e70));
}
#[inline(always)]
pub fn block_0x00203d48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2112844u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966448u32, 2112848u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2112852u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2112856u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2112860u32);
    emu.apc_no_count(6usize, 2112860u32, 106496u32, 2112864u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2112868u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965444u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00203d64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 1u32, 2112872u32);
    emu.sw_no_count(12usize, 2usize, 24u32, 2112876u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2112880u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966244u32, 2112884u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2112888u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966368u32, 2112892u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2112896u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 4294966352u32, 2112900u32);
    emu.adi_no_count(12usize, 0usize, 6u32, 2112904u32);
    emu.adi_no_count(14usize, 0usize, 3u32, 2112908u32);
    emu.adi_no_count(15usize, 2usize, 24u32, 2112912u32);
    emu.apc_no_count(1usize, 2112912u32, 106496u32, 2112916u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112920u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965492u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203d98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2112924u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2112928u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112932u32;
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
pub fn block_0x00203da4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2112936u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965774u32, 2112940u32);
    emu.adi_no_count(12usize, 0usize, 16u32, 2112944u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2112948u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2112952u32);
    emu.apc_no_count(6usize, 2112952u32, 106496u32, 2112956u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2112960u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965352u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203dc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2112964u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966425u32, 2112968u32);
    emu.adi_no_count(12usize, 0usize, 12u32, 2112972u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2112976u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2112980u32);
    emu.apc_no_count(6usize, 2112980u32, 106496u32, 2112984u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2112988u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965324u32);
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
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2112992u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966383u32, 2112996u32);
    emu.adi_no_count(12usize, 0usize, 12u32, 2113000u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2113004u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2113008u32);
    emu.apc_no_count(6usize, 2113008u32, 106496u32, 2113012u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2113016u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965296u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00203df8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4u32, 2113020u32);
    emu.sw_no_count(12usize, 2usize, 24u32, 2113024u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2113028u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966128u32, 2113032u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2113036u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966592u32, 2113040u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2113044u32);
    emu.adi_no_count(13usize, 2usize, 24u32, 2113048u32);
    emu.apc_no_count(1usize, 2113048u32, 106496u32, 2113052u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113056u32;
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
pub fn block_0x00203e20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2113060u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2113064u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113068u32;
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
pub fn block_0x00203e2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 12usize, 1u32, 2113072u32);
    emu.adi_no_count(12usize, 12usize, 4u32, 2113076u32);
    emu.sw_no_count(12usize, 2usize, 24u32, 2113080u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2113084u32;
    emu.update_insn_clock();
    emu.adi_no_count(5usize, 5usize, 4294966352u32, 2113088u32);
    emu.adi_no_count(6usize, 2usize, 24u32, 2113092u32);
    emu.adi_no_count(7usize, 0usize, 6u32, 2113096u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2113100u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966516u32, 2113104u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2113108u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966212u32, 2113112u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2113116u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 4294966500u32, 2113120u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2113124u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 4294966529u32, 2113128u32);
    emu.adi_no_count(12usize, 0usize, 13u32, 2113132u32);
    emu.adi_no_count(14usize, 0usize, 8u32, 2113136u32);
    emu.add_memory_rw_events(17usize);
    emu.pc = 2113136u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203e70));
}
#[inline(always)]
pub fn block_0x00203e70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(7usize, 2usize, 0u32, 2113140u32)?;
    emu.sw_no_count(6usize, 2usize, 4u32, 2113144u32)?;
    emu.sw_no_count(5usize, 2usize, 8u32, 2113148u32)?;
    emu.apc_no_count(1usize, 2113148u32, 106496u32, 2113152u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113156u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965492u32);
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
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2113160u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2113164u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113168u32;
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
pub fn block_0x00203e90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 1u32, 2113172u32);
    emu.sw_no_count(12usize, 2usize, 24u32, 2113176u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2113180u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966608u32, 2113184u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2113188u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966368u32, 2113192u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2113196u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 4294966352u32, 2113200u32);
    emu.adi_no_count(12usize, 0usize, 5u32, 2113204u32);
    emu.adi_no_count(14usize, 0usize, 3u32, 2113208u32);
    emu.adi_no_count(15usize, 2usize, 24u32, 2113212u32);
    emu.apc_no_count(1usize, 2113212u32, 102400u32, 2113216u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113220u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1992u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203ec4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2113224u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2113228u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113232u32;
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
pub fn block_0x00203ed0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2113236u32);
    emu.lbu_no_count(13usize, 10usize, 0u32, 2113240u32);
    emu.adi_no_count(14usize, 0usize, 23u32, 2113244u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2113248u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2113272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203ef8));
    } else {
        emu.pc = 2113252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203ee4));
    }
}
#[inline(always)]
pub fn block_0x00203ee4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2113256u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966120u32, 2113260u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2113264u32);
    emu.apc_no_count(6usize, 2113264u32, 102400u32, 2113268u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2113272u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1840u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00203ef8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2113276u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2113280u32)?;
    emu.sw_no_count(12usize, 2usize, 8u32, 2113284u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2113288u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966104u32, 2113292u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2113296u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966352u32, 2113300u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2113304u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2113308u32);
    emu.apc_no_count(1usize, 2113308u32, 106496u32, 2113312u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113316u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965748u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203f24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2113320u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2113324u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113328u32;
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
pub fn block_0x00203f30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2113332u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2113336u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2113340u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2113344u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2113348u32)?;
    emu.adi_no_count(9usize, 11usize, 0u32, 2113352u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2113356u32);
    emu.lbu_no_count(10usize, 11usize, 0u32, 2113360u32);
    emu.sb_no_count(10usize, 2usize, 31u32, 2113364u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2113368u32);
    emu.adi_no_count(12usize, 2usize, 31u32, 2113372u32);
    emu.adi_no_count(13usize, 0usize, 1u32, 2113376u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2113380u32);
    emu.apc_no_count(1usize, 2113380u32, 0u32, 2113384u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113388u32;
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
#[inline(always)]
pub fn block_0x00203f6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 8u32, 2113392u32);
    emu.adi_no_count(18usize, 0usize, 4u32, 2113396u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2114100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204234));
    } else {
        emu.pc = 2113400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203f78));
    }
}
#[inline(always)]
pub fn block_0x00203f78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 9usize, 1u32, 2113404u32);
    emu.sb_no_count(10usize, 2usize, 31u32, 2113408u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2113412u32);
    emu.adi_no_count(12usize, 2usize, 31u32, 2113416u32);
    emu.adi_no_count(13usize, 0usize, 1u32, 2113420u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2113424u32);
    emu.apc_no_count(1usize, 2113424u32, 0u32, 2113428u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113432u32;
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
pub fn block_0x00203f98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 8u32, 2113436u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2114100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204234));
    } else {
        emu.pc = 2113440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203fa0));
    }
}
#[inline(always)]
pub fn block_0x00203fa0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 9usize, 2u32, 2113444u32);
    emu.sb_no_count(10usize, 2usize, 31u32, 2113448u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2113452u32);
    emu.adi_no_count(12usize, 2usize, 31u32, 2113456u32);
    emu.adi_no_count(13usize, 0usize, 1u32, 2113460u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2113464u32);
    emu.apc_no_count(1usize, 2113464u32, 0u32, 2113468u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113472u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965276u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203fc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 8u32, 2113476u32);
    emu.adi_no_count(18usize, 0usize, 4u32, 2113480u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2114100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204234));
    } else {
        emu.pc = 2113484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203fcc));
    }
}
#[inline(always)]
pub fn block_0x00203fcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 9usize, 3u32, 2113488u32);
    emu.sb_no_count(10usize, 2usize, 31u32, 2113492u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2113496u32);
    emu.adi_no_count(12usize, 2usize, 31u32, 2113500u32);
    emu.adi_no_count(13usize, 0usize, 1u32, 2113504u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2113508u32);
    emu.apc_no_count(1usize, 2113508u32, 4294963200u32, 2113512u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113516u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(2032u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203fec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 8u32, 2113520u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2114100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204234));
    } else {
        emu.pc = 2113524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203ff4));
    }
}
#[inline(always)]
pub fn block_0x00203ff4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 4u32, 2113528u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2113532u32);
    emu.apc_no_count(1usize, 2113532u32, 4294963200u32, 2113536u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113540u32;
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
pub fn block_0x00204004(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2114128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204250));
    } else {
        emu.pc = 2113544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204008));
    }
}
#[inline(always)]
pub fn block_0x00204008(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 5u32, 2113548u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2113552u32);
    emu.apc_no_count(1usize, 2113552u32, 4294963200u32, 2113556u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113560u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(260u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204018(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2114128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204250));
    } else {
        emu.pc = 2113564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020401c));
    }
}
#[inline(always)]
pub fn block_0x0020401c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 6u32, 2113568u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2113572u32);
    emu.apc_no_count(1usize, 2113572u32, 4294963200u32, 2113576u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113580u32;
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
pub fn block_0x0020402c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2114128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204250));
    } else {
        emu.pc = 2113584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204030));
    }
}
#[inline(always)]
pub fn block_0x00204030(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 7u32, 2113588u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2113592u32);
    emu.apc_no_count(1usize, 2113592u32, 4294963200u32, 2113596u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113600u32;
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
pub fn block_0x00204040(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2114128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204250));
    } else {
        emu.pc = 2113604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204044));
    }
}
#[inline(always)]
pub fn block_0x00204044(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 8u32, 2113608u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2113612u32);
    emu.apc_no_count(1usize, 2113612u32, 4294963200u32, 2113616u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113620u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(200u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204054(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2114128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204250));
    } else {
        emu.pc = 2113624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204058));
    }
}
#[inline(always)]
pub fn block_0x00204058(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 9u32, 2113628u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2113632u32);
    emu.apc_no_count(1usize, 2113632u32, 4294963200u32, 2113636u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113640u32;
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
pub fn block_0x00204068(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2114128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204250));
    } else {
        emu.pc = 2113644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020406c));
    }
}
#[inline(always)]
pub fn block_0x0020406c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 10u32, 2113648u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2113652u32);
    emu.apc_no_count(1usize, 2113652u32, 4294963200u32, 2113656u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113660u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(160u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020407c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2114128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204250));
    } else {
        emu.pc = 2113664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204080));
    }
}
#[inline(always)]
pub fn block_0x00204080(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 11u32, 2113668u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2113672u32);
    emu.apc_no_count(1usize, 2113672u32, 4294963200u32, 2113676u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113680u32;
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
pub fn block_0x00204090(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2114128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204250));
    } else {
        emu.pc = 2113684u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204094));
    }
}
#[inline(always)]
pub fn block_0x00204094(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 12u32, 2113688u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2113692u32);
    emu.apc_no_count(1usize, 2113692u32, 4294963200u32, 2113696u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113700u32;
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
pub fn block_0x002040a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2114128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204250));
    } else {
        emu.pc = 2113704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002040a8));
    }
}
#[inline(always)]
pub fn block_0x002040a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 13u32, 2113708u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2113712u32);
    emu.apc_no_count(1usize, 2113712u32, 4294963200u32, 2113716u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113720u32;
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
pub fn block_0x002040b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2114128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204250));
    } else {
        emu.pc = 2113724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002040bc));
    }
}
#[inline(always)]
pub fn block_0x002040bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 14u32, 2113728u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2113732u32);
    emu.apc_no_count(1usize, 2113732u32, 4294963200u32, 2113736u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113740u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(80u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002040cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2114128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204250));
    } else {
        emu.pc = 2113744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002040d0));
    }
}
#[inline(always)]
pub fn block_0x002040d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 15u32, 2113748u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2113752u32);
    emu.apc_no_count(1usize, 2113752u32, 4294963200u32, 2113756u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113760u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x002040e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2114128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204250));
    } else {
        emu.pc = 2113764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002040e4));
    }
}
#[inline(always)]
pub fn block_0x002040e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 16u32, 2113768u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2113772u32);
    emu.apc_no_count(1usize, 2113772u32, 4294963200u32, 2113776u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113780u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(40u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002040f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2114128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204250));
    } else {
        emu.pc = 2113784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002040f8));
    }
}
#[inline(always)]
pub fn block_0x002040f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 17u32, 2113788u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2113792u32);
    emu.apc_no_count(1usize, 2113792u32, 4294963200u32, 2113796u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113800u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(20u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204108(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2114128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204250));
    } else {
        emu.pc = 2113804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020410c));
    }
}
#[inline(always)]
pub fn block_0x0020410c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 18u32, 2113808u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2113812u32);
    emu.apc_no_count(1usize, 2113812u32, 4294963200u32, 2113816u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113820u32;
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
pub fn block_0x0020411c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2114128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204250));
    } else {
        emu.pc = 2113824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204120));
    }
}
#[inline(always)]
pub fn block_0x00204120(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 19u32, 2113828u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2113832u32);
    emu.apc_no_count(1usize, 2113832u32, 4294963200u32, 2113836u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113840u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967276u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204130(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2114128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204250));
    } else {
        emu.pc = 2113844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204134));
    }
}
#[inline(always)]
pub fn block_0x00204134(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 20u32, 2113848u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2113852u32);
    emu.apc_no_count(1usize, 2113852u32, 4294963200u32, 2113856u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113860u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967256u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204144(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2114128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204250));
    } else {
        emu.pc = 2113864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204148));
    }
}
#[inline(always)]
pub fn block_0x00204148(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 21u32, 2113868u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2113872u32);
    emu.apc_no_count(1usize, 2113872u32, 4294963200u32, 2113876u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113880u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967236u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204158(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2114128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204250));
    } else {
        emu.pc = 2113884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020415c));
    }
}
#[inline(always)]
pub fn block_0x0020415c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 22u32, 2113888u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2113892u32);
    emu.apc_no_count(1usize, 2113892u32, 4294963200u32, 2113896u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113900u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967216u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020416c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2114128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204250));
    } else {
        emu.pc = 2113904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204170));
    }
}
#[inline(always)]
pub fn block_0x00204170(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 23u32, 2113908u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2113912u32);
    emu.apc_no_count(1usize, 2113912u32, 4294963200u32, 2113916u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113920u32;
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
pub fn block_0x00204180(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2114128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204250));
    } else {
        emu.pc = 2113924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204184));
    }
}
#[inline(always)]
pub fn block_0x00204184(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 24u32, 2113928u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2113932u32);
    emu.apc_no_count(1usize, 2113932u32, 4294963200u32, 2113936u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113940u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967176u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204194(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2114128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204250));
    } else {
        emu.pc = 2113944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204198));
    }
}
#[inline(always)]
pub fn block_0x00204198(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 25u32, 2113948u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2113952u32);
    emu.apc_no_count(1usize, 2113952u32, 4294963200u32, 2113956u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113960u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967156u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002041a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2114128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204250));
    } else {
        emu.pc = 2113964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002041ac));
    }
}
#[inline(always)]
pub fn block_0x002041ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 26u32, 2113968u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2113972u32);
    emu.apc_no_count(1usize, 2113972u32, 4294963200u32, 2113976u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113980u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967136u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002041bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2114128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204250));
    } else {
        emu.pc = 2113984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002041c0));
    }
}
#[inline(always)]
pub fn block_0x002041c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 27u32, 2113988u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2113992u32);
    emu.apc_no_count(1usize, 2113992u32, 4294963200u32, 2113996u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114000u32;
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
#[inline(always)]
pub fn block_0x002041d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2114128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204250));
    } else {
        emu.pc = 2114004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002041d4));
    }
}
#[inline(always)]
pub fn block_0x002041d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 28u32, 2114008u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2114012u32);
    emu.apc_no_count(1usize, 2114012u32, 4294963200u32, 2114016u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114020u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967096u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002041e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2114128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204250));
    } else {
        emu.pc = 2114024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002041e8));
    }
}
#[inline(always)]
pub fn block_0x002041e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 29u32, 2114028u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2114032u32);
    emu.apc_no_count(1usize, 2114032u32, 4294963200u32, 2114036u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114040u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967076u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002041f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2114128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204250));
    } else {
        emu.pc = 2114044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002041fc));
    }
}
#[inline(always)]
pub fn block_0x002041fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 30u32, 2114048u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2114052u32);
    emu.apc_no_count(1usize, 2114052u32, 4294963200u32, 2114056u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114060u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967056u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020420c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2114128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204250));
    } else {
        emu.pc = 2114064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204210));
    }
}
#[inline]
pub fn block_0x00204210(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 31u32, 2114068u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2114072u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2114076u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2114080u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2114084u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2114088u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2114092u32);
    emu.apc_no_count(6usize, 2114092u32, 4294963200u32, 2114096u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2114100u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294967016u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204234(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2114104u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2114108u32)?;
    emu.sw_no_count(10usize, 2usize, 16u32, 2114112u32)?;
    emu.sw_no_count(11usize, 2usize, 20u32, 2114116u32)?;
    emu.adi_no_count(10usize, 2usize, 16u32, 2114120u32);
    emu.apc_no_count(1usize, 2114120u32, 20480u32, 2114124u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114128u32;
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
pub fn block_0x00204250(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2114132u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2114136u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2114140u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2114144u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2114148u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114152u32;
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
pub fn block_0x00204268(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2114152u32, 16384u32, 2114156u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2114160u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965748u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00204270(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967216u32, 2114164u32);
    emu.sw_no_count(1usize, 2usize, 76u32, 2114168u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2114172u32)?;
    emu.sw_no_count(11usize, 2usize, 16u32, 2114176u32)?;
    emu.sw_no_count(12usize, 2usize, 20u32, 2114180u32)?;
    emu.adi_no_count(10usize, 2usize, 12u32, 2114184u32);
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2114188u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966824u32, 2114192u32);
    emu.adi_no_count(12usize, 2usize, 16u32, 2114196u32);
    let a = 0u32.wrapping_add(2121728u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2114200u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294967076u32, 2114204u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2114208u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 356u32, 2114212u32);
    emu.adi_no_count(15usize, 0usize, 2u32, 2114216u32);
    emu.sw_no_count(0usize, 2usize, 40u32, 2114220u32)?;
    emu.sw_no_count(10usize, 2usize, 48u32, 2114224u32)?;
    emu.sw_no_count(11usize, 2usize, 52u32, 2114228u32)?;
    emu.sw_no_count(12usize, 2usize, 56u32, 2114232u32)?;
    emu.sw_no_count(13usize, 2usize, 60u32, 2114236u32)?;
    emu.adi_no_count(10usize, 2usize, 48u32, 2114240u32);
    emu.sw_no_count(14usize, 2usize, 24u32, 2114244u32)?;
    emu.sw_no_count(15usize, 2usize, 28u32, 2114248u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2114252u32)?;
    emu.sw_no_count(15usize, 2usize, 36u32, 2114256u32)?;
    emu.adi_no_count(10usize, 2usize, 64u32, 2114260u32);
    emu.adi_no_count(11usize, 2usize, 24u32, 2114264u32);
    emu.apc_no_count(1usize, 2114264u32, 90112u32, 2114268u32);
    emu.add_memory_rw_events(28usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114272u32;
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
pub fn block_0x002042e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2114272u32, 16384u32, 2114276u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114280u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(224u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002042e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2114284u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1524u32, 2114288u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2114292u32);
    emu.adi_no_count(12usize, 0usize, 12u32, 2114296u32);
    emu.apc_no_count(1usize, 2114296u32, 20480u32, 2114300u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114304u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966884u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204300(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2114344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204328));
    } else {
        emu.pc = 2114308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204304));
    }
}
#[inline]
pub fn block_0x00204304(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 64u32, 2114312u32)?;
    emu.lw_no_count(12usize, 2usize, 68u32, 2114316u32)?;
    emu.lw_no_count(13usize, 2usize, 72u32, 2114320u32)?;
    emu.sw_no_count(11usize, 10usize, 0u32, 2114324u32)?;
    emu.sw_no_count(12usize, 10usize, 4u32, 2114328u32)?;
    emu.sw_no_count(13usize, 10usize, 8u32, 2114332u32)?;
    emu.lw_no_count(1usize, 2usize, 76u32, 2114336u32)?;
    emu.adi_no_count(2usize, 2usize, 80u32, 2114340u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114344u32;
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
pub fn block_0x00204328(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2114348u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2114352u32);
    emu.apc_no_count(1usize, 2114352u32, 90112u32, 2114356u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114360u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(476u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00204338(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 51u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294966672u32, 2114364u32);
    emu.sw_no_count(1usize, 2usize, 620u32, 2114368u32)?;
    emu.sw_no_count(8usize, 2usize, 616u32, 2114372u32)?;
    emu.sw_no_count(9usize, 2usize, 612u32, 2114376u32)?;
    emu.sw_no_count(18usize, 2usize, 608u32, 2114380u32)?;
    emu.sw_no_count(19usize, 2usize, 604u32, 2114384u32)?;
    emu.sw_no_count(20usize, 2usize, 600u32, 2114388u32)?;
    emu.sw_no_count(21usize, 2usize, 596u32, 2114392u32)?;
    emu.sw_no_count(22usize, 2usize, 592u32, 2114396u32)?;
    emu.sw_no_count(23usize, 2usize, 588u32, 2114400u32)?;
    emu.sw_no_count(24usize, 2usize, 584u32, 2114404u32)?;
    emu.adi_no_count(9usize, 12usize, 0u32, 2114408u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2114412u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2114416u32);
    emu.lw_no_count(10usize, 11usize, 16u32, 2114420u32)?;
    emu.lw_no_count(11usize, 11usize, 20u32, 2114424u32)?;
    emu.lw_no_count(12usize, 18usize, 24u32, 2114428u32)?;
    emu.lw_no_count(13usize, 18usize, 28u32, 2114432u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2114436u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2114440u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2114444u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2114448u32)?;
    emu.lw_no_count(10usize, 18usize, 0u32, 2114452u32)?;
    emu.lw_no_count(11usize, 18usize, 4u32, 2114456u32)?;
    emu.lw_no_count(12usize, 18usize, 8u32, 2114460u32)?;
    emu.lw_no_count(13usize, 18usize, 12u32, 2114464u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2114468u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2114472u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2114476u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2114480u32)?;
    emu.lw_no_count(10usize, 9usize, 16u32, 2114484u32)?;
    emu.lw_no_count(11usize, 9usize, 20u32, 2114488u32)?;
    emu.lw_no_count(12usize, 9usize, 24u32, 2114492u32)?;
    emu.lw_no_count(13usize, 9usize, 28u32, 2114496u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2114500u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2114504u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2114508u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2114512u32)?;
    emu.lw_no_count(10usize, 9usize, 0u32, 2114516u32)?;
    emu.lw_no_count(11usize, 9usize, 4u32, 2114520u32)?;
    emu.lw_no_count(12usize, 9usize, 8u32, 2114524u32)?;
    emu.lw_no_count(13usize, 9usize, 12u32, 2114528u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2114532u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2114536u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2114540u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2114544u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2114548u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2114552u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2114556u32);
    emu.apc_no_count(1usize, 2114556u32, 24576u32, 2114560u32);
    emu.add_memory_rw_events(51usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114564u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965304u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00204404(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 48u32, 2114568u32)?;
    emu.lw_no_count(11usize, 18usize, 52u32, 2114572u32)?;
    emu.lw_no_count(12usize, 18usize, 56u32, 2114576u32)?;
    emu.lw_no_count(13usize, 18usize, 60u32, 2114580u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2114584u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2114588u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2114592u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2114596u32)?;
    emu.lw_no_count(10usize, 18usize, 32u32, 2114600u32)?;
    emu.lw_no_count(11usize, 18usize, 36u32, 2114604u32)?;
    emu.lw_no_count(12usize, 18usize, 40u32, 2114608u32)?;
    emu.lw_no_count(13usize, 18usize, 44u32, 2114612u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2114616u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2114620u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2114624u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2114628u32)?;
    emu.lw_no_count(10usize, 9usize, 48u32, 2114632u32)?;
    emu.lw_no_count(11usize, 9usize, 52u32, 2114636u32)?;
    emu.lw_no_count(12usize, 9usize, 56u32, 2114640u32)?;
    emu.lw_no_count(13usize, 9usize, 60u32, 2114644u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2114648u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2114652u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2114656u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2114660u32)?;
    emu.lw_no_count(10usize, 9usize, 32u32, 2114664u32)?;
    emu.lw_no_count(11usize, 9usize, 36u32, 2114668u32)?;
    emu.lw_no_count(12usize, 9usize, 40u32, 2114672u32)?;
    emu.lw_no_count(13usize, 9usize, 44u32, 2114676u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2114680u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2114684u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2114688u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2114692u32)?;
    emu.adi_no_count(10usize, 2usize, 40u32, 2114696u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2114700u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2114704u32);
    emu.apc_no_count(1usize, 2114704u32, 20480u32, 2114708u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114712u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1956u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00204498(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 80u32, 2114716u32)?;
    emu.lw_no_count(11usize, 18usize, 84u32, 2114720u32)?;
    emu.lw_no_count(12usize, 18usize, 88u32, 2114724u32)?;
    emu.lw_no_count(13usize, 18usize, 92u32, 2114728u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2114732u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2114736u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2114740u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2114744u32)?;
    emu.lw_no_count(10usize, 18usize, 64u32, 2114748u32)?;
    emu.lw_no_count(11usize, 18usize, 68u32, 2114752u32)?;
    emu.lw_no_count(12usize, 18usize, 72u32, 2114756u32)?;
    emu.lw_no_count(13usize, 18usize, 76u32, 2114760u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2114764u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2114768u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2114772u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2114776u32)?;
    emu.lw_no_count(10usize, 9usize, 80u32, 2114780u32)?;
    emu.lw_no_count(11usize, 9usize, 84u32, 2114784u32)?;
    emu.lw_no_count(12usize, 9usize, 88u32, 2114788u32)?;
    emu.lw_no_count(13usize, 9usize, 92u32, 2114792u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2114796u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2114800u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2114804u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2114808u32)?;
    emu.lw_no_count(10usize, 9usize, 64u32, 2114812u32)?;
    emu.lw_no_count(11usize, 9usize, 68u32, 2114816u32)?;
    emu.lw_no_count(12usize, 9usize, 72u32, 2114820u32)?;
    emu.lw_no_count(13usize, 9usize, 76u32, 2114824u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2114828u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2114832u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2114836u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2114840u32)?;
    emu.adi_no_count(10usize, 2usize, 72u32, 2114844u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2114848u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2114852u32);
    emu.apc_no_count(1usize, 2114852u32, 20480u32, 2114856u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114860u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1808u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
