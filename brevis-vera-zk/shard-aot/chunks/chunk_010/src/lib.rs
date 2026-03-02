pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2142488u32;
pub const PC_MAX: u32 = 2144712u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 109usize] = [
        block_0x0020b118,
        block_0x0020b11c,
        block_0x0020b128,
        block_0x0020b130,
        block_0x0020b144,
        block_0x0020b158,
        block_0x0020b160,
        block_0x0020b170,
        block_0x0020b174,
        block_0x0020b1a4,
        block_0x0020b1b8,
        block_0x0020b1e0,
        block_0x0020b1f8,
        block_0x0020b204,
        block_0x0020b230,
        block_0x0020b238,
        block_0x0020b23c,
        block_0x0020b244,
        block_0x0020b250,
        block_0x0020b260,
        block_0x0020b270,
        block_0x0020b278,
        block_0x0020b290,
        block_0x0020b2ac,
        block_0x0020b2b0,
        block_0x0020b2cc,
        block_0x0020b2d4,
        block_0x0020b300,
        block_0x0020b338,
        block_0x0020b364,
        block_0x0020b36c,
        block_0x0020b37c,
        block_0x0020b380,
        block_0x0020b39c,
        block_0x0020b3c4,
        block_0x0020b3d0,
        block_0x0020b3e4,
        block_0x0020b3ec,
        block_0x0020b3f4,
        block_0x0020b3fc,
        block_0x0020b400,
        block_0x0020b408,
        block_0x0020b40c,
        block_0x0020b41c,
        block_0x0020b420,
        block_0x0020b424,
        block_0x0020b43c,
        block_0x0020b440,
        block_0x0020b448,
        block_0x0020b44c,
        block_0x0020b454,
        block_0x0020b468,
        block_0x0020b46c,
        block_0x0020b490,
        block_0x0020b494,
        block_0x0020b4c8,
        block_0x0020b4f0,
        block_0x0020b520,
        block_0x0020b534,
        block_0x0020b55c,
        block_0x0020b574,
        block_0x0020b580,
        block_0x0020b5ac,
        block_0x0020b5b4,
        block_0x0020b5b8,
        block_0x0020b5c0,
        block_0x0020b5cc,
        block_0x0020b5dc,
        block_0x0020b5ec,
        block_0x0020b5f4,
        block_0x0020b60c,
        block_0x0020b628,
        block_0x0020b62c,
        block_0x0020b648,
        block_0x0020b650,
        block_0x0020b67c,
        block_0x0020b6b4,
        block_0x0020b6e0,
        block_0x0020b700,
        block_0x0020b718,
        block_0x0020b720,
        block_0x0020b738,
        block_0x0020b760,
        block_0x0020b76c,
        block_0x0020b78c,
        block_0x0020b7a4,
        block_0x0020b7ac,
        block_0x0020b7c4,
        block_0x0020b7ec,
        block_0x0020b7f8,
        block_0x0020b818,
        block_0x0020b830,
        block_0x0020b838,
        block_0x0020b850,
        block_0x0020b878,
        block_0x0020b884,
        block_0x0020b8a4,
        block_0x0020b8bc,
        block_0x0020b8c4,
        block_0x0020b8dc,
        block_0x0020b904,
        block_0x0020b910,
        block_0x0020b934,
        block_0x0020b954,
        block_0x0020b990,
        block_0x0020b994,
        block_0x0020b9a0,
        block_0x0020b9a4,
        block_0x0020b9c8,
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
    if pc < 2142488u32 || pc > 2144712u32 {
        return None;
    }
    let word_offset = ((pc - 2142488u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x0020b118(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2142504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b128));
    } else {
        emu.pc = 2142492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b11c));
    }
}
#[inline(always)]
pub fn block_0x0020b11c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 12usize, 0u32, 2142496u32);
    emu.apc_no_count(1usize, 2142496u32, 0u32, 2142500u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2142504u32;
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
pub fn block_0x0020b128(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2142504u32, 0u32, 2142508u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2142512u32;
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
pub fn block_0x0020b130(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2142516u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2142520u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2142524u32);
    emu.apc_no_count(1usize, 2142524u32, 4294930432u32, 2142528u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2142532u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1088u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020b144(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 0u32, 2142536u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2142540u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966640u32, 2142544u32);
    emu.apc_no_count(6usize, 2142544u32, 20480u32, 2142548u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2142552u32;
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
pub fn block_0x0020b158(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2142556u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2142576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b170));
    } else {
        emu.pc = 2142560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b160));
    }
}
#[inline(always)]
pub fn block_0x0020b160(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 4u32, 2142564u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2142568u32);
    emu.apc_no_count(6usize, 2142568u32, 4294926336u32, 2142572u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2142576u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x0020b170(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2142580u32;
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
pub fn block_0x0020b174(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2142584u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2142588u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2142592u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2142596u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2142600u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2142604u32)?;
    emu.adi_no_count(8usize, 12usize, 0u32, 2142608u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2142612u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2142616u32)?;
    emu.lw_no_count(18usize, 9usize, 8u32, 2142620u32)?;
    emu.sbr_no_count(10usize, 10usize, 18usize, 2142624u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2142688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b1e0));
    } else {
        emu.pc = 2142628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b1a4));
    }
}
#[inline(always)]
pub fn block_0x0020b1a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 4u32, 2142632u32)?;
    emu.adr_no_count(10usize, 10usize, 18usize, 2142636u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2142640u32);
    emu.apc_no_count(1usize, 2142640u32, 4294934528u32, 2142644u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2142648u32;
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
#[inline]
pub fn block_0x0020b1b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 18usize, 8usize, 2142652u32);
    emu.sw_no_count(8usize, 9usize, 8u32, 2142656u32)?;
    emu.adi_no_count(10usize, 0usize, 0u32, 2142660u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2142664u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2142668u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2142672u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2142676u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2142680u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2142684u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2142688u32;
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
pub fn block_0x0020b1e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2142692u32);
    emu.adi_no_count(19usize, 11usize, 0u32, 2142696u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2142700u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2142704u32);
    emu.apc_no_count(1usize, 2142704u32, 0u32, 2142708u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2142712u32;
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
pub fn block_0x0020b1f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 19usize, 0u32, 2142716u32);
    emu.lw_no_count(18usize, 9usize, 8u32, 2142720u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2142724u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2142628u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b1a4));
}
#[inline]
pub fn block_0x0020b204(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2142728u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2142732u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2142736u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2142740u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2142744u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2142748u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2142752u32)?;
    emu.lw_no_count(8usize, 10usize, 8u32, 2142756u32)?;
    emu.adi_no_count(12usize, 0usize, 128u32, 2142760u32);
    emu.sri_no_count(19usize, 11usize, 11u32, 2142764u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2142776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b238));
    } else {
        emu.pc = 2142768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b230));
    }
}
#[inline(always)]
pub fn block_0x0020b230(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 1u32, 2142772u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2142776u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2142800u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b250));
}
#[inline(always)]
pub fn block_0x0020b238(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2142788u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b244));
    } else {
        emu.pc = 2142780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b23c));
    }
}
#[inline(always)]
pub fn block_0x0020b23c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 2u32, 2142784u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2142788u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2142800u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b250));
}
#[inline(always)]
pub fn block_0x0020b244(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 11usize, 16u32, 2142792u32);
    emu.sltru_no_count(9usize, 0usize, 12usize, 2142796u32);
    emu.adi_no_count(9usize, 9usize, 3u32, 2142800u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2142800u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b250));
}
#[inline(always)]
pub fn block_0x0020b250(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2142804u32)?;
    emu.sbr_no_count(13usize, 12usize, 8usize, 2142808u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2142812u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2142840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b278));
    } else {
        emu.pc = 2142816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b260));
    }
}
#[inline(always)]
pub fn block_0x0020b260(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 4u32, 2142820u32)?;
    emu.adi_no_count(14usize, 0usize, 128u32, 2142824u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2142828u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2142892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b2ac));
    } else {
        emu.pc = 2142832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b270));
    }
}
#[inline(always)]
pub fn block_0x0020b270(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 12usize, 0u32, 2142836u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2142840u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2143032u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b338));
}
#[inline(always)]
pub fn block_0x0020b278(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2142844u32);
    emu.adi_no_count(20usize, 11usize, 0u32, 2142848u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2142852u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2142856u32);
    emu.apc_no_count(1usize, 2142856u32, 0u32, 2142860u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2142864u32;
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
pub fn block_0x0020b290(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 20usize, 0u32, 2142868u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2142872u32);
    emu.lw_no_count(12usize, 18usize, 8u32, 2142876u32)?;
    emu.lw_no_count(13usize, 18usize, 4u32, 2142880u32)?;
    emu.adi_no_count(14usize, 0usize, 128u32, 2142884u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2142888u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a < b {
        emu.pc = 2142832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b270));
    } else {
        emu.pc = 2142892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b2ac));
    }
}
#[inline(always)]
pub fn block_0x0020b2ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2142924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b2cc));
    } else {
        emu.pc = 2142896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b2b0));
    }
}
#[inline(always)]
pub fn block_0x0020b2b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 6u32, 2142900u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2142904u32);
    emu.ori_no_count(13usize, 13usize, 192u32, 2142908u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2142912u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2142916u32);
    emu.sb_no_count(11usize, 12usize, 1u32, 2142920u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2142924u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2143032u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b338));
}
#[inline(always)]
pub fn block_0x0020b2cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 16u32, 2142928u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2142976u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b300));
    } else {
        emu.pc = 2142932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b2d4));
    }
}
#[inline]
pub fn block_0x0020b2d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 12u32, 2142936u32);
    emu.sli_no_count(14usize, 11usize, 20u32, 2142940u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2142944u32);
    emu.ori_no_count(13usize, 13usize, 224u32, 2142948u32);
    emu.sri_no_count(14usize, 14usize, 26u32, 2142952u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2142956u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2142960u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2142964u32);
    emu.sb_no_count(14usize, 12usize, 1u32, 2142968u32);
    emu.sb_no_count(11usize, 12usize, 2u32, 2142972u32);
    emu.add_memory_rw_events(11usize);
    let return_addr = 2142976u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2143032u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b338));
}
#[inline]
pub fn block_0x0020b300(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 18u32, 2142980u32);
    emu.sli_no_count(14usize, 11usize, 14u32, 2142984u32);
    emu.sli_no_count(15usize, 11usize, 20u32, 2142988u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2142992u32);
    emu.ori_no_count(13usize, 13usize, 240u32, 2142996u32);
    emu.sri_no_count(14usize, 14usize, 26u32, 2143000u32);
    emu.sri_no_count(15usize, 15usize, 26u32, 2143004u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2143008u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2143012u32);
    emu.adi_no_count(15usize, 15usize, 128u32, 2143016u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2143020u32);
    emu.sb_no_count(14usize, 12usize, 1u32, 2143024u32);
    emu.sb_no_count(15usize, 12usize, 2u32, 2143028u32);
    emu.sb_no_count(11usize, 12usize, 3u32, 2143032u32);
    emu.add_memory_rw_events(14usize);
    emu.pc = 2143032u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b338));
}
#[inline]
pub fn block_0x0020b338(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 9usize, 8usize, 2143036u32);
    emu.sw_no_count(8usize, 10usize, 8u32, 2143040u32)?;
    emu.adi_no_count(10usize, 0usize, 0u32, 2143044u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2143048u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2143052u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2143056u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2143060u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2143064u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2143068u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2143072u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2143076u32;
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
pub fn block_0x0020b364(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2143080u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2143100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b37c));
    } else {
        emu.pc = 2143084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b36c));
    }
}
#[inline(always)]
pub fn block_0x0020b36c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 4u32, 2143088u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2143092u32);
    emu.apc_no_count(6usize, 2143092u32, 4294926336u32, 2143096u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2143100u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(936u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020b37c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2143104u32;
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
pub fn block_0x0020b380(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2143108u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966664u32, 2143112u32);
    emu.adi_no_count(12usize, 0usize, 5u32, 2143116u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2143120u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2143124u32);
    emu.apc_no_count(6usize, 2143124u32, 24576u32, 2143128u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2143132u32;
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
pub fn block_0x0020b39c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2143136u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2143140u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2143144u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2143148u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2143152u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2143156u32)?;
    emu.adi_no_count(9usize, 11usize, 0u32, 2143160u32);
    emu.lw_no_count(11usize, 11usize, 4u32, 2143164u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2143168u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2143232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b400));
    } else {
        emu.pc = 2143172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b3c4));
    }
}
#[inline(always)]
pub fn block_0x0020b3c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 0u32, 2143176u32);
    emu.lw_no_count(10usize, 9usize, 0u32, 2143180u32)?;
    emu.adi_no_count(12usize, 10usize, 4u32, 2143184u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2143184u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b3d0));
}
#[inline(always)]
pub fn block_0x0020b3d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 12usize, 0u32, 2143188u32)?;
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2143192u32);
    emu.adr_no_count(18usize, 13usize, 18usize, 2143196u32);
    emu.adi_no_count(12usize, 12usize, 8u32, 2143200u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2143184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b3d0));
    } else {
        emu.pc = 2143204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b3e4));
    }
}
#[inline(always)]
pub fn block_0x0020b3e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 9usize, 12u32, 2143208u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2143260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b41c));
    } else {
        emu.pc = 2143212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b3ec));
    }
}
#[inline(always)]
pub fn block_0x0020b3ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 15u32, 2143216u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2143244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b40c));
    } else {
        emu.pc = 2143220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b3f4));
    }
}
#[inline(always)]
pub fn block_0x0020b3f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 4u32, 2143224u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2143292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b43c));
    } else {
        emu.pc = 2143228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b3fc));
    }
}
#[inline(always)]
pub fn block_0x0020b3fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2143232u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2143244u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b40c));
}
#[inline(always)]
pub fn block_0x0020b400(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 12u32, 2143236u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2143292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b43c));
    } else {
        emu.pc = 2143240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b408));
    }
}
#[inline(always)]
pub fn block_0x0020b408(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 0u32, 2143244u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2143244u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b40c));
}
#[inline(always)]
pub fn block_0x0020b40c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltr_no_count(10usize, 0usize, 18usize, 2143248u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2143252u32);
    emu.anr_no_count(18usize, 10usize, 18usize, 2143256u32);
    emu.sli_no_count(18usize, 18usize, 1u32, 2143260u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2143260u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b41c));
}
#[inline(always)]
pub fn block_0x0020b41c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2143304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b448));
    } else {
        emu.pc = 2143264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b420));
    }
}
#[inline(always)]
pub fn block_0x0020b420(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 0u32, 2143268u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2143268u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b424));
}
#[inline(always)]
pub fn block_0x0020b424(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2143272u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966696u32, 2143276u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2143280u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2143284u32);
    emu.apc_no_count(1usize, 2143284u32, 0u32, 2143288u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2143292u32;
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
pub fn block_0x0020b43c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 0u32, 2143296u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2143296u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b440));
}
#[inline(always)]
pub fn block_0x0020b440(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2143300u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2143304u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2143340u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b46c));
}
#[inline(always)]
pub fn block_0x0020b448(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2143296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b440));
    } else {
        emu.pc = 2143308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b44c));
    }
}
#[inline(always)]
pub fn block_0x0020b44c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2143308u32, 4294930432u32, 2143312u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2143316u32;
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
pub fn block_0x0020b454(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2143320u32);
    emu.adi_no_count(19usize, 0usize, 1u32, 2143324u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2143328u32);
    emu.apc_no_count(1usize, 2143328u32, 4294926336u32, 2143332u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2143336u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(672u32);
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
        emu.pc = 2143268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b424));
    } else {
        emu.pc = 2143340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b46c));
    }
}
#[inline]
pub fn block_0x0020b46c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 2usize, 12u32, 2143344u32)?;
    emu.sw_no_count(10usize, 2usize, 16u32, 2143348u32)?;
    emu.sw_no_count(0usize, 2usize, 20u32, 2143352u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2143356u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966672u32, 2143360u32);
    emu.adi_no_count(10usize, 2usize, 12u32, 2143364u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2143368u32);
    emu.apc_no_count(1usize, 2143368u32, 20480u32, 2143372u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2143376u32;
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
pub fn block_0x0020b490(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2143432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b4c8));
    } else {
        emu.pc = 2143380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b494));
    }
}
#[inline]
pub fn block_0x0020b494(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2143384u32)?;
    emu.lw_no_count(11usize, 2usize, 16u32, 2143388u32)?;
    emu.lw_no_count(12usize, 2usize, 20u32, 2143392u32)?;
    emu.sw_no_count(10usize, 8usize, 0u32, 2143396u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2143400u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2143404u32)?;
    emu.lw_no_count(1usize, 2usize, 44u32, 2143408u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2143412u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2143416u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2143420u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2143424u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2143428u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2143432u32;
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
pub fn block_0x0020b4c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2143436u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966728u32, 2143440u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2143444u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966712u32, 2143448u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2143452u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966816u32, 2143456u32);
    emu.adi_no_count(11usize, 0usize, 86u32, 2143460u32);
    emu.adi_no_count(12usize, 2usize, 27u32, 2143464u32);
    emu.apc_no_count(1usize, 2143464u32, 16384u32, 2143468u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2143472u32;
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
pub fn block_0x0020b4f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2143476u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2143480u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2143484u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2143488u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2143492u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2143496u32)?;
    emu.adi_no_count(8usize, 12usize, 0u32, 2143500u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2143504u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2143508u32)?;
    emu.lw_no_count(18usize, 9usize, 8u32, 2143512u32)?;
    emu.sbr_no_count(10usize, 10usize, 18usize, 2143516u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2143580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b55c));
    } else {
        emu.pc = 2143520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b520));
    }
}
#[inline(always)]
pub fn block_0x0020b520(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 4u32, 2143524u32)?;
    emu.adr_no_count(10usize, 10usize, 18usize, 2143528u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2143532u32);
    emu.apc_no_count(1usize, 2143532u32, 4294934528u32, 2143536u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2143540u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965696u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020b534(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 18usize, 8usize, 2143544u32);
    emu.sw_no_count(8usize, 9usize, 8u32, 2143548u32)?;
    emu.adi_no_count(10usize, 0usize, 0u32, 2143552u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2143556u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2143560u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2143564u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2143568u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2143572u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2143576u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2143580u32;
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
pub fn block_0x0020b55c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2143584u32);
    emu.adi_no_count(19usize, 11usize, 0u32, 2143588u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2143592u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2143596u32);
    emu.apc_no_count(1usize, 2143596u32, 0u32, 2143600u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2143604u32;
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
pub fn block_0x0020b574(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 19usize, 0u32, 2143608u32);
    emu.lw_no_count(18usize, 9usize, 8u32, 2143612u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2143616u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2143520u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b520));
}
#[inline]
pub fn block_0x0020b580(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2143620u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2143624u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2143628u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2143632u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2143636u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2143640u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2143644u32)?;
    emu.lw_no_count(8usize, 10usize, 8u32, 2143648u32)?;
    emu.adi_no_count(12usize, 0usize, 128u32, 2143652u32);
    emu.sri_no_count(19usize, 11usize, 11u32, 2143656u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2143668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b5b4));
    } else {
        emu.pc = 2143660u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b5ac));
    }
}
#[inline(always)]
pub fn block_0x0020b5ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 1u32, 2143664u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2143668u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2143692u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b5cc));
}
#[inline(always)]
pub fn block_0x0020b5b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2143680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b5c0));
    } else {
        emu.pc = 2143672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b5b8));
    }
}
#[inline(always)]
pub fn block_0x0020b5b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 2u32, 2143676u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2143680u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2143692u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b5cc));
}
#[inline(always)]
pub fn block_0x0020b5c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 11usize, 16u32, 2143684u32);
    emu.sltru_no_count(9usize, 0usize, 12usize, 2143688u32);
    emu.adi_no_count(9usize, 9usize, 3u32, 2143692u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2143692u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b5cc));
}
#[inline(always)]
pub fn block_0x0020b5cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2143696u32)?;
    emu.sbr_no_count(13usize, 12usize, 8usize, 2143700u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2143704u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2143732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b5f4));
    } else {
        emu.pc = 2143708u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b5dc));
    }
}
#[inline(always)]
pub fn block_0x0020b5dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 4u32, 2143712u32)?;
    emu.adi_no_count(14usize, 0usize, 128u32, 2143716u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2143720u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2143784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b628));
    } else {
        emu.pc = 2143724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b5ec));
    }
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
    emu.sb_no_count(11usize, 12usize, 0u32, 2143728u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2143732u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2143924u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b6b4));
}
#[inline(always)]
pub fn block_0x0020b5f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2143736u32);
    emu.adi_no_count(20usize, 11usize, 0u32, 2143740u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2143744u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2143748u32);
    emu.apc_no_count(1usize, 2143748u32, 0u32, 2143752u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2143756u32;
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
pub fn block_0x0020b60c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 20usize, 0u32, 2143760u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2143764u32);
    emu.lw_no_count(12usize, 18usize, 8u32, 2143768u32)?;
    emu.lw_no_count(13usize, 18usize, 4u32, 2143772u32)?;
    emu.adi_no_count(14usize, 0usize, 128u32, 2143776u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2143780u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a < b {
        emu.pc = 2143724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b5ec));
    } else {
        emu.pc = 2143784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b628));
    }
}
#[inline(always)]
pub fn block_0x0020b628(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2143816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b648));
    } else {
        emu.pc = 2143788u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b62c));
    }
}
#[inline(always)]
pub fn block_0x0020b62c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 6u32, 2143792u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2143796u32);
    emu.ori_no_count(13usize, 13usize, 192u32, 2143800u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2143804u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2143808u32);
    emu.sb_no_count(11usize, 12usize, 1u32, 2143812u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2143816u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2143924u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b6b4));
}
#[inline(always)]
pub fn block_0x0020b648(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 16u32, 2143820u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2143868u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b67c));
    } else {
        emu.pc = 2143824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b650));
    }
}
#[inline]
pub fn block_0x0020b650(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 12u32, 2143828u32);
    emu.sli_no_count(14usize, 11usize, 20u32, 2143832u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2143836u32);
    emu.ori_no_count(13usize, 13usize, 224u32, 2143840u32);
    emu.sri_no_count(14usize, 14usize, 26u32, 2143844u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2143848u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2143852u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2143856u32);
    emu.sb_no_count(14usize, 12usize, 1u32, 2143860u32);
    emu.sb_no_count(11usize, 12usize, 2u32, 2143864u32);
    emu.add_memory_rw_events(11usize);
    let return_addr = 2143868u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2143924u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b6b4));
}
#[inline]
pub fn block_0x0020b67c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 18u32, 2143872u32);
    emu.sli_no_count(14usize, 11usize, 14u32, 2143876u32);
    emu.sli_no_count(15usize, 11usize, 20u32, 2143880u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2143884u32);
    emu.ori_no_count(13usize, 13usize, 240u32, 2143888u32);
    emu.sri_no_count(14usize, 14usize, 26u32, 2143892u32);
    emu.sri_no_count(15usize, 15usize, 26u32, 2143896u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2143900u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2143904u32);
    emu.adi_no_count(15usize, 15usize, 128u32, 2143908u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2143912u32);
    emu.sb_no_count(14usize, 12usize, 1u32, 2143916u32);
    emu.sb_no_count(15usize, 12usize, 2u32, 2143920u32);
    emu.sb_no_count(11usize, 12usize, 3u32, 2143924u32);
    emu.add_memory_rw_events(14usize);
    emu.pc = 2143924u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b6b4));
}
#[inline]
pub fn block_0x0020b6b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 9usize, 8usize, 2143928u32);
    emu.sw_no_count(8usize, 10usize, 8u32, 2143932u32)?;
    emu.adi_no_count(10usize, 0usize, 0u32, 2143936u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2143940u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2143944u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2143948u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2143952u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2143956u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2143960u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2143964u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2143968u32;
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
pub fn block_0x0020b6e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967152u32, 2143972u32);
    emu.sw_no_count(1usize, 2usize, 140u32, 2143976u32)?;
    emu.adi_no_count(16usize, 11usize, 0u32, 2143980u32);
    emu.adi_no_count(15usize, 0usize, 0u32, 2143984u32);
    emu.lbu_no_count(10usize, 10usize, 0u32, 2143988u32);
    emu.adi_no_count(11usize, 2usize, 139u32, 2143992u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2143996u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2144000u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2144024u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b718));
}
#[inline(always)]
pub fn block_0x0020b700(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 87u32, 2144004u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2144008u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2144012u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2144016u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2144020u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2144056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b738));
    } else {
        emu.pc = 2144024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b718));
    }
}
#[inline(always)]
pub fn block_0x0020b718(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(13usize, 10usize, 15u32, 2144028u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2144000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b700));
    } else {
        emu.pc = 2144032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b720));
    }
}
#[inline(always)]
pub fn block_0x0020b720(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 48u32, 2144036u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2144040u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2144044u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2144048u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2144052u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2144024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b718));
    } else {
        emu.pc = 2144056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b738));
    }
}
#[inline]
pub fn block_0x0020b738(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2144060u32);
    emu.sbr_no_count(10usize, 10usize, 15usize, 2144064u32);
    emu.adi_no_count(14usize, 10usize, 128u32, 2144068u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2144072u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966832u32, 2144076u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2144080u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2144084u32);
    emu.adi_no_count(10usize, 16usize, 0u32, 2144088u32);
    emu.apc_no_count(1usize, 2144088u32, 20480u32, 2144092u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2144096u32;
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
pub fn block_0x0020b760(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 140u32, 2144100u32)?;
    emu.adi_no_count(2usize, 2usize, 144u32, 2144104u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2144108u32;
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
pub fn block_0x0020b76c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967152u32, 2144112u32);
    emu.sw_no_count(1usize, 2usize, 140u32, 2144116u32)?;
    emu.adi_no_count(16usize, 11usize, 0u32, 2144120u32);
    emu.adi_no_count(15usize, 0usize, 0u32, 2144124u32);
    emu.lbu_no_count(10usize, 10usize, 0u32, 2144128u32);
    emu.adi_no_count(11usize, 2usize, 139u32, 2144132u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2144136u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2144140u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2144164u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b7a4));
}
#[inline(always)]
pub fn block_0x0020b78c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 55u32, 2144144u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2144148u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2144152u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2144156u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2144160u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2144196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b7c4));
    } else {
        emu.pc = 2144164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b7a4));
    }
}
#[inline(always)]
pub fn block_0x0020b7a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(13usize, 10usize, 15u32, 2144168u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2144140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b78c));
    } else {
        emu.pc = 2144172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b7ac));
    }
}
#[inline(always)]
pub fn block_0x0020b7ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 48u32, 2144176u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2144180u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2144184u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2144188u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2144192u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2144164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b7a4));
    } else {
        emu.pc = 2144196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b7c4));
    }
}
#[inline]
pub fn block_0x0020b7c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2144200u32);
    emu.sbr_no_count(10usize, 10usize, 15usize, 2144204u32);
    emu.adi_no_count(14usize, 10usize, 128u32, 2144208u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2144212u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966832u32, 2144216u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2144220u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2144224u32);
    emu.adi_no_count(10usize, 16usize, 0u32, 2144228u32);
    emu.apc_no_count(1usize, 2144228u32, 16384u32, 2144232u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2144236u32;
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
pub fn block_0x0020b7ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 140u32, 2144240u32)?;
    emu.adi_no_count(2usize, 2usize, 144u32, 2144244u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2144248u32;
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
pub fn block_0x0020b7f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967152u32, 2144252u32);
    emu.sw_no_count(1usize, 2usize, 140u32, 2144256u32)?;
    emu.adi_no_count(16usize, 11usize, 0u32, 2144260u32);
    emu.adi_no_count(15usize, 0usize, 0u32, 2144264u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2144268u32)?;
    emu.adi_no_count(11usize, 2usize, 139u32, 2144272u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2144276u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2144280u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2144304u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b830));
}
#[inline(always)]
pub fn block_0x0020b818(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 87u32, 2144284u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2144288u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2144292u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2144296u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2144300u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2144336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b850));
    } else {
        emu.pc = 2144304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b830));
    }
}
#[inline(always)]
pub fn block_0x0020b830(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(13usize, 10usize, 15u32, 2144308u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2144280u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b818));
    } else {
        emu.pc = 2144312u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b838));
    }
}
#[inline(always)]
pub fn block_0x0020b838(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 48u32, 2144316u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2144320u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2144324u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2144328u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2144332u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2144304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b830));
    } else {
        emu.pc = 2144336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b850));
    }
}
#[inline]
pub fn block_0x0020b850(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2144340u32);
    emu.sbr_no_count(10usize, 10usize, 15usize, 2144344u32);
    emu.adi_no_count(14usize, 10usize, 128u32, 2144348u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2144352u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966832u32, 2144356u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2144360u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2144364u32);
    emu.adi_no_count(10usize, 16usize, 0u32, 2144368u32);
    emu.apc_no_count(1usize, 2144368u32, 16384u32, 2144372u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2144376u32;
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
pub fn block_0x0020b878(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 140u32, 2144380u32)?;
    emu.adi_no_count(2usize, 2usize, 144u32, 2144384u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2144388u32;
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
pub fn block_0x0020b884(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967152u32, 2144392u32);
    emu.sw_no_count(1usize, 2usize, 140u32, 2144396u32)?;
    emu.adi_no_count(16usize, 11usize, 0u32, 2144400u32);
    emu.adi_no_count(15usize, 0usize, 0u32, 2144404u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2144408u32)?;
    emu.adi_no_count(11usize, 2usize, 139u32, 2144412u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2144416u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2144420u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2144444u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b8bc));
}
#[inline(always)]
pub fn block_0x0020b8a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 55u32, 2144424u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2144428u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2144432u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2144436u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2144440u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2144476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b8dc));
    } else {
        emu.pc = 2144444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b8bc));
    }
}
#[inline(always)]
pub fn block_0x0020b8bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(13usize, 10usize, 15u32, 2144448u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2144420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b8a4));
    } else {
        emu.pc = 2144452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b8c4));
    }
}
#[inline(always)]
pub fn block_0x0020b8c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 48u32, 2144456u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2144460u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2144464u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2144468u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2144472u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2144444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b8bc));
    } else {
        emu.pc = 2144476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b8dc));
    }
}
#[inline]
pub fn block_0x0020b8dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2144480u32);
    emu.sbr_no_count(10usize, 10usize, 15usize, 2144484u32);
    emu.adi_no_count(14usize, 10usize, 128u32, 2144488u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2144492u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966832u32, 2144496u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2144500u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2144504u32);
    emu.adi_no_count(10usize, 16usize, 0u32, 2144508u32);
    emu.apc_no_count(1usize, 2144508u32, 16384u32, 2144512u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2144516u32;
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
pub fn block_0x0020b904(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 140u32, 2144520u32)?;
    emu.adi_no_count(2usize, 2usize, 144u32, 2144524u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2144528u32;
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
pub fn block_0x0020b910(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 4294967295u32, 2144532u32);
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2144536u32;
    emu.update_insn_clock();
    emu.sbr_no_count(13usize, 13usize, 11usize, 2144540u32);
    emu.xrr_no_count(11usize, 11usize, 12usize, 2144544u32);
    emu.sltru_no_count(11usize, 12usize, 11usize, 2144548u32);
    emu.sltru_no_count(10usize, 13usize, 10usize, 2144552u32);
    emu.xri_no_count(10usize, 10usize, 1u32, 2144556u32);
    emu.anr_no_count(10usize, 11usize, 10usize, 2144560u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2144564u32;
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
pub fn block_0x0020b934(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2144568u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2144572u32)?;
    emu.adi_no_count(13usize, 11usize, 0u32, 2144576u32);
    emu.lbu_no_count(11usize, 10usize, 0u32, 2144580u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2144584u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2144588u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966834u32, 2144592u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2144660u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b994));
    } else {
        emu.pc = 2144596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b954));
    }
}
#[inline]
pub fn block_0x0020b954(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 41u32, 2144600u32);
    emu.adi_no_count(14usize, 0usize, 100u32, 2144604u32);
    emu.mul_no_count(12usize, 11usize, 12usize, 2144608u32);
    emu.sri_no_count(12usize, 12usize, 12u32, 2144612u32);
    emu.mul_no_count(14usize, 12usize, 14usize, 2144616u32);
    emu.sbr_no_count(14usize, 11usize, 14usize, 2144620u32);
    emu.sli_no_count(14usize, 14usize, 25u32, 2144624u32);
    emu.sri_no_count(14usize, 14usize, 24u32, 2144628u32);
    emu.adr_no_count(14usize, 10usize, 14usize, 2144632u32);
    emu.lbu_no_count(15usize, 14usize, 0u32, 2144636u32);
    emu.lbu_no_count(14usize, 14usize, 1u32, 2144640u32);
    emu.sb_no_count(15usize, 2usize, 10u32, 2144644u32);
    emu.sb_no_count(14usize, 2usize, 11u32, 2144648u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2144652u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2144672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b9a0));
    } else {
        emu.pc = 2144656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b990));
    }
}
#[inline(always)]
pub fn block_0x0020b990(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2144660u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2144676u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b9a4));
}
#[inline(always)]
pub fn block_0x0020b994(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 3u32, 2144664u32);
    emu.adi_no_count(12usize, 11usize, 0u32, 2144668u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2144676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b9a4));
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
        emu.pc = 2144712u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b9c8));
    } else {
        emu.pc = 2144676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b9a4));
    }
}
#[inline]
pub fn block_0x0020b9a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 1u32, 2144680u32);
    emu.adi_no_count(12usize, 12usize, 1u32, 2144684u32);
    emu.ani_no_count(11usize, 12usize, 255u32, 2144688u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2144692u32);
    emu.lbu_no_count(10usize, 10usize, 0u32, 2144696u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2144700u32);
    emu.adi_no_count(11usize, 2usize, 9u32, 2144704u32);
    emu.adr_no_count(11usize, 11usize, 14usize, 2144708u32);
    emu.sb_no_count(10usize, 11usize, 0u32, 2144712u32);
    emu.add_memory_rw_events(9usize);
    emu.pc = 2144712u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b9c8));
}
#[inline]
pub fn block_0x0020b9c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 3u32, 2144716u32);
    emu.adi_no_count(10usize, 2usize, 9u32, 2144720u32);
    emu.sbr_no_count(15usize, 15usize, 14usize, 2144724u32);
    emu.adr_no_count(14usize, 10usize, 14usize, 2144728u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2144732u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2144736u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2144740u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2144744u32);
    emu.apc_no_count(1usize, 2144744u32, 16384u32, 2144748u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2144752u32;
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
