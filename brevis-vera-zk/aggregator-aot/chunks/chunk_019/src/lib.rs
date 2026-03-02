pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2198496u32;
pub const PC_MAX: u32 = 2200988u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 109usize] = [
        block_0x00218be0,
        block_0x00218c00,
        block_0x00218c08,
        block_0x00218c5c,
        block_0x00218c6c,
        block_0x00218c74,
        block_0x00218c8c,
        block_0x00218c9c,
        block_0x00218ca4,
        block_0x00218cac,
        block_0x00218cbc,
        block_0x00218cd0,
        block_0x00218ce8,
        block_0x00218d3c,
        block_0x00218d74,
        block_0x00218d84,
        block_0x00218d8c,
        block_0x00218da0,
        block_0x00218da4,
        block_0x00218dd8,
        block_0x00218e0c,
        block_0x00218e14,
        block_0x00218e24,
        block_0x00218e28,
        block_0x00218e34,
        block_0x00218e38,
        block_0x00218e48,
        block_0x00218e4c,
        block_0x00218e58,
        block_0x00218e60,
        block_0x00218e64,
        block_0x00218e8c,
        block_0x00218e94,
        block_0x00218e9c,
        block_0x00218eac,
        block_0x00218ed4,
        block_0x00218edc,
        block_0x00218f08,
        block_0x00218f10,
        block_0x00218f14,
        block_0x00218f1c,
        block_0x00218f28,
        block_0x00218f38,
        block_0x00218f48,
        block_0x00218f50,
        block_0x00218f70,
        block_0x00218f8c,
        block_0x00218f90,
        block_0x00218fac,
        block_0x00218fb4,
        block_0x00218fe0,
        block_0x00219018,
        block_0x00219044,
        block_0x00219074,
        block_0x00219088,
        block_0x002190b0,
        block_0x002190d0,
        block_0x002190dc,
        block_0x00219118,
        block_0x00219130,
        block_0x00219164,
        block_0x00219180,
        block_0x00219188,
        block_0x002191b8,
        block_0x002191d0,
        block_0x002191e4,
        block_0x002191f4,
        block_0x00219210,
        block_0x00219218,
        block_0x00219220,
        block_0x00219240,
        block_0x0021924c,
        block_0x00219260,
        block_0x00219274,
        block_0x00219284,
        block_0x002192bc,
        block_0x002192c4,
        block_0x00219300,
        block_0x00219318,
        block_0x00219348,
        block_0x00219364,
        block_0x0021936c,
        block_0x00219398,
        block_0x002193b0,
        block_0x002193c4,
        block_0x002193d4,
        block_0x002193ec,
        block_0x002193f4,
        block_0x00219414,
        block_0x00219420,
        block_0x00219434,
        block_0x00219448,
        block_0x00219458,
        block_0x00219488,
        block_0x00219494,
        block_0x0021949c,
        block_0x002194a4,
        block_0x002194b0,
        block_0x002194b8,
        block_0x002194d8,
        block_0x00219504,
        block_0x0021951c,
        block_0x00219528,
        block_0x00219544,
        block_0x00219564,
        block_0x00219574,
        block_0x00219580,
        block_0x00219584,
        block_0x0021959c,
    ];
    const IDX: [u16; 624usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 3u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 4u16, 0u16, 0u16, 0u16, 5u16, 0u16, 6u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 7u16, 0u16, 0u16, 0u16, 8u16, 0u16, 9u16, 0u16, 10u16,
        0u16, 0u16, 0u16, 11u16, 0u16, 0u16, 0u16, 0u16, 12u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 13u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 14u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 15u16, 0u16, 0u16,
        0u16, 16u16, 0u16, 17u16, 0u16, 0u16, 0u16, 0u16, 18u16, 19u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 20u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 21u16, 0u16, 22u16, 0u16,
        0u16, 0u16, 23u16, 24u16, 0u16, 0u16, 25u16, 26u16, 0u16, 0u16, 0u16, 27u16,
        28u16, 0u16, 0u16, 29u16, 0u16, 30u16, 31u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 32u16, 0u16, 33u16, 0u16, 34u16, 0u16, 0u16, 0u16, 35u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 36u16, 0u16, 37u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 38u16, 0u16, 39u16, 40u16, 0u16,
        41u16, 0u16, 0u16, 42u16, 0u16, 0u16, 0u16, 43u16, 0u16, 0u16, 0u16, 44u16, 0u16,
        45u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 46u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 47u16, 48u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 49u16, 0u16, 50u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 51u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 52u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 53u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 54u16, 0u16, 0u16, 0u16, 0u16,
        55u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 56u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 57u16, 0u16, 0u16, 58u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 59u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 60u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 61u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 62u16, 0u16, 63u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 64u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 65u16, 0u16, 0u16, 0u16, 0u16, 66u16, 0u16, 0u16, 0u16,
        67u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 68u16, 0u16, 69u16, 0u16, 70u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 71u16, 0u16, 0u16, 72u16, 0u16, 0u16, 0u16,
        0u16, 73u16, 0u16, 0u16, 0u16, 0u16, 74u16, 0u16, 0u16, 0u16, 75u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 76u16, 0u16,
        77u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 78u16, 0u16, 0u16, 0u16, 0u16, 0u16, 79u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 80u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 81u16, 0u16, 82u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 83u16, 0u16, 0u16, 0u16, 0u16, 0u16, 84u16, 0u16, 0u16, 0u16, 0u16, 85u16,
        0u16, 0u16, 0u16, 86u16, 0u16, 0u16, 0u16, 0u16, 0u16, 87u16, 0u16, 88u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 89u16, 0u16, 0u16, 90u16, 0u16, 0u16, 0u16,
        0u16, 91u16, 0u16, 0u16, 0u16, 0u16, 92u16, 0u16, 0u16, 0u16, 93u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 94u16, 0u16, 0u16, 95u16,
        0u16, 96u16, 0u16, 97u16, 0u16, 0u16, 98u16, 0u16, 99u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 100u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 101u16, 0u16, 0u16, 0u16, 0u16, 0u16, 102u16, 0u16, 0u16, 103u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 104u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        105u16, 0u16, 0u16, 0u16, 106u16, 0u16, 0u16, 107u16, 108u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 109u16,
    ];
    if pc < 2198496u32 || pc > 2200988u32 {
        return None;
    }
    let word_offset = ((pc - 2198496u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x00218be0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2198500u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2198504u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2198508u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2198512u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2198516u32)?;
    emu.adi_no_count(8usize, 11usize, 0u32, 2198520u32);
    emu.apc_no_count(1usize, 2198520u32, 4294901760u32, 2198524u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2198528u32;
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
pub fn block_0x00218c00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 255u32, 2198532u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2198760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218ce8));
    } else {
        emu.pc = 2198536u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218c08));
    }
}
#[inline]
pub fn block_0x00218c08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(8usize, 2usize, 40u32, 2198540u32)?;
    emu.adi_no_count(10usize, 2usize, 40u32, 2198544u32);
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2198548u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1064u32, 2198552u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2198556u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 640u32, 2198560u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2198564u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2198568u32)?;
    emu.adi_no_count(14usize, 2usize, 32u32, 2198572u32);
    emu.sw_no_count(10usize, 2usize, 32u32, 2198576u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2198580u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2198584u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2198588u32)?;
    emu.sw_no_count(13usize, 2usize, 12u32, 2198592u32)?;
    emu.sw_no_count(14usize, 2usize, 16u32, 2198596u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2198600u32)?;
    emu.adi_no_count(10usize, 2usize, 0u32, 2198604u32);
    emu.adi_no_count(11usize, 2usize, 47u32, 2198608u32);
    emu.adi_no_count(12usize, 2usize, 8u32, 2198612u32);
    emu.apc_no_count(1usize, 2198612u32, 4294963200u32, 2198616u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2198620u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(880u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218c5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 0u32, 2198624u32);
    emu.lw_no_count(8usize, 2usize, 4u32, 2198628u32)?;
    emu.adi_no_count(11usize, 0usize, 4u32, 2198632u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2198668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218c8c));
    } else {
        emu.pc = 2198636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218c6c));
    }
}
#[inline(always)]
pub fn block_0x00218c6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 3u32, 2198640u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2198668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218c8c));
    } else {
        emu.pc = 2198644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218c74));
    }
}
#[inline(always)]
pub fn block_0x00218c74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2198648u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2198652u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2198656u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2198660u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2198664u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2198668u32;
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
pub fn block_0x00218c8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 8usize, 4u32, 2198672u32)?;
    emu.lw_no_count(11usize, 18usize, 0u32, 2198676u32)?;
    emu.lw_no_count(9usize, 8usize, 0u32, 2198680u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2198692u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218ca4));
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
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2198688u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2198692u32;
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
pub fn block_0x00218ca4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 18usize, 4u32, 2198696u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2198716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218cbc));
    } else {
        emu.pc = 2198700u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218cac));
    }
}
#[inline(always)]
pub fn block_0x00218cac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 8u32, 2198704u32)?;
    emu.adi_no_count(10usize, 9usize, 0u32, 2198708u32);
    emu.apc_no_count(1usize, 2198708u32, 4294901760u32, 2198712u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2198716u32;
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
pub fn block_0x00218cbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 12u32, 2198720u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2198724u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2198728u32);
    emu.apc_no_count(1usize, 2198728u32, 4294901760u32, 2198732u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2198736u32;
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
pub fn block_0x00218cd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2198740u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2198744u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2198748u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2198752u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2198756u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2198760u32;
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
pub fn block_0x00218ce8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(8usize, 2usize, 0u32, 2198764u32)?;
    emu.adi_no_count(10usize, 2usize, 0u32, 2198768u32);
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2198772u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1064u32, 2198776u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2198780u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 672u32, 2198784u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2198788u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2198792u32)?;
    emu.adi_no_count(14usize, 2usize, 32u32, 2198796u32);
    emu.sw_no_count(10usize, 2usize, 32u32, 2198800u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2198804u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2198808u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2198812u32)?;
    emu.sw_no_count(13usize, 2usize, 12u32, 2198816u32)?;
    emu.sw_no_count(14usize, 2usize, 16u32, 2198820u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2198824u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2198828u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 688u32, 2198832u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2198836u32);
    emu.apc_no_count(1usize, 2198836u32, 12288u32, 2198840u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2198844u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966824u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00218d3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 10usize, 0u32, 2198848u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2198852u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1192u32, 2198856u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2198860u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1360u32, 2198864u32);
    emu.sli_no_count(10usize, 10usize, 2u32, 2198868u32);
    emu.adr_no_count(12usize, 12usize, 10usize, 2198872u32);
    emu.adr_no_count(10usize, 13usize, 10usize, 2198876u32);
    emu.lw_no_count(12usize, 12usize, 0u32, 2198880u32)?;
    emu.lw_no_count(13usize, 10usize, 0u32, 2198884u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2198888u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2198892u32);
    emu.apc_no_count(6usize, 2198892u32, 20480u32, 2198896u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2198900u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x00218d74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2248704u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2198904u32;
    emu.update_insn_clock();
    emu.lw_no_count(13usize, 12usize, 4294965808u32, 2198908u32)?;
    emu.adi_no_count(12usize, 10usize, 0u32, 2198912u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2198924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218d8c));
    } else {
        emu.pc = 2198916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218d84));
    }
}
#[inline(always)]
pub fn block_0x00218d84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2198920u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966240u32, 2198924u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2198924u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218d8c));
}
#[inline(always)]
pub fn block_0x00218d8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2198928u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2198932u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2198936u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2198940u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2198944u32;
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
pub fn block_0x00218da0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.update_insn_clock();
    return Err(format!("Unimplemented instruction (UNIMP) at PC {:#x}", 2198944u32));
}
#[inline]
pub fn block_0x00218da4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2330587136u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2198952u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4112453632u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2198956u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2965131264u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2198960u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2020298752u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2198964u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 428u32, 2198968u32);
    emu.adi_no_count(12usize, 12usize, 1473u32, 2198972u32);
    emu.adi_no_count(13usize, 13usize, 4294965685u32, 2198976u32);
    emu.adi_no_count(14usize, 14usize, 4294965580u32, 2198980u32);
    emu.sw_no_count(14usize, 10usize, 0u32, 2198984u32)?;
    emu.sw_no_count(13usize, 10usize, 4u32, 2198988u32)?;
    emu.sw_no_count(12usize, 10usize, 8u32, 2198992u32)?;
    emu.sw_no_count(11usize, 10usize, 12u32, 2198996u32)?;
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199000u32;
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
pub fn block_0x00218dd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(3112902656u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2199004u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1470513152u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2199008u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1676365824u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2199012u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(3603652608u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2199016u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966129u32, 2199020u32);
    emu.adi_no_count(12usize, 12usize, 376u32, 2199024u32);
    emu.adi_no_count(13usize, 13usize, 44u32, 2199028u32);
    emu.adi_no_count(14usize, 14usize, 4294966637u32, 2199032u32);
    emu.sw_no_count(14usize, 10usize, 0u32, 2199036u32)?;
    emu.sw_no_count(13usize, 10usize, 4u32, 2199040u32)?;
    emu.sw_no_count(12usize, 10usize, 8u32, 2199044u32)?;
    emu.sw_no_count(11usize, 10usize, 12u32, 2199048u32)?;
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199052u32;
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
pub fn block_0x00218e0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2199056u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2199076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218e24));
    } else {
        emu.pc = 2199060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218e14));
    }
}
#[inline(always)]
pub fn block_0x00218e14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 4u32, 2199064u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2199068u32);
    emu.apc_no_count(6usize, 2199068u32, 4294901760u32, 2199072u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2199076u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965452u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218e24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199080u32;
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
pub fn block_0x00218e28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2199084u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2199088u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2199112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218e48));
    } else {
        emu.pc = 2199092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218e34));
    }
}
#[inline(always)]
pub fn block_0x00218e34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2199112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218e48));
    } else {
        emu.pc = 2199096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218e38));
    }
}
#[inline(always)]
pub fn block_0x00218e38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 4u32, 2199100u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2199104u32);
    emu.apc_no_count(6usize, 2199104u32, 4294901760u32, 2199108u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2199112u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965416u32);
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
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199116u32;
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
pub fn block_0x00218e4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 255u32, 2199120u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2199124u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2199140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218e64));
    } else {
        emu.pc = 2199128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218e58));
    }
}
#[inline(always)]
pub fn block_0x00218e58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 3u32, 2199132u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
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
        emu.pc = 2199136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218e60));
    }
}
#[inline(always)]
pub fn block_0x00218e60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199140u32;
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
pub fn block_0x00218e64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2199144u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2199148u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2199152u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2199156u32)?;
    emu.sw_no_count(18usize, 2usize, 0u32, 2199160u32)?;
    emu.lw_no_count(18usize, 11usize, 4u32, 2199164u32)?;
    emu.lw_no_count(12usize, 18usize, 0u32, 2199168u32)?;
    emu.adi_no_count(8usize, 11usize, 0u32, 2199172u32);
    emu.lw_no_count(9usize, 11usize, 0u32, 2199176u32)?;
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2199188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218e94));
    } else {
        emu.pc = 2199180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218e8c));
    }
}
#[inline(always)]
pub fn block_0x00218e8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2199184u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2199188u32;
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
pub fn block_0x00218e94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 18usize, 4u32, 2199192u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2199212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218eac));
    } else {
        emu.pc = 2199196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218e9c));
    }
}
#[inline(always)]
pub fn block_0x00218e9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 8u32, 2199200u32)?;
    emu.adi_no_count(10usize, 9usize, 0u32, 2199204u32);
    emu.apc_no_count(1usize, 2199204u32, 4294901760u32, 2199208u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199212u32;
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
pub fn block_0x00218eac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 12u32, 2199216u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2199220u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2199224u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2199228u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2199232u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2199236u32)?;
    emu.lw_no_count(18usize, 2usize, 0u32, 2199240u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2199244u32);
    emu.apc_no_count(6usize, 2199244u32, 4294901760u32, 2199248u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2199252u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x00218ed4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2199256u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199260u32;
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
pub fn block_0x00218edc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2199264u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2199268u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2199272u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2199276u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2199280u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2199284u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2199288u32)?;
    emu.lw_no_count(8usize, 10usize, 8u32, 2199292u32)?;
    emu.adi_no_count(12usize, 0usize, 128u32, 2199296u32);
    emu.sri_no_count(19usize, 11usize, 11u32, 2199300u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2199312u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218f10));
    } else {
        emu.pc = 2199304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218f08));
    }
}
#[inline(always)]
pub fn block_0x00218f08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 1u32, 2199308u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2199312u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2199336u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218f28));
}
#[inline(always)]
pub fn block_0x00218f10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2199324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218f1c));
    } else {
        emu.pc = 2199316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218f14));
    }
}
#[inline(always)]
pub fn block_0x00218f14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 2u32, 2199320u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2199324u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2199336u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218f28));
}
#[inline(always)]
pub fn block_0x00218f1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 11usize, 16u32, 2199328u32);
    emu.sltru_no_count(9usize, 0usize, 12usize, 2199332u32);
    emu.adi_no_count(9usize, 9usize, 3u32, 2199336u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2199336u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218f28));
}
#[inline(always)]
pub fn block_0x00218f28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2199340u32)?;
    emu.sbr_no_count(13usize, 12usize, 8usize, 2199344u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2199348u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2199376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218f50));
    } else {
        emu.pc = 2199352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218f38));
    }
}
#[inline(always)]
pub fn block_0x00218f38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 4u32, 2199356u32)?;
    emu.adi_no_count(14usize, 0usize, 128u32, 2199360u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2199364u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2199436u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218f8c));
    } else {
        emu.pc = 2199368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218f48));
    }
}
#[inline(always)]
pub fn block_0x00218f48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 12usize, 0u32, 2199372u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2199376u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2199576u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219018));
}
#[inline(always)]
pub fn block_0x00218f50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2199380u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2199384u32);
    emu.adi_no_count(18usize, 10usize, 0u32, 2199388u32);
    emu.adi_no_count(20usize, 11usize, 0u32, 2199392u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2199396u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2199400u32);
    emu.apc_no_count(1usize, 2199400u32, 4294963200u32, 2199404u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199408u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1368u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218f70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 20usize, 0u32, 2199412u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2199416u32);
    emu.lw_no_count(12usize, 18usize, 8u32, 2199420u32)?;
    emu.lw_no_count(13usize, 18usize, 4u32, 2199424u32)?;
    emu.adi_no_count(14usize, 0usize, 128u32, 2199428u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2199432u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a < b {
        emu.pc = 2199368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218f48));
    } else {
        emu.pc = 2199436u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218f8c));
    }
}
#[inline(always)]
pub fn block_0x00218f8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2199468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218fac));
    } else {
        emu.pc = 2199440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218f90));
    }
}
#[inline(always)]
pub fn block_0x00218f90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 6u32, 2199444u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2199448u32);
    emu.ori_no_count(13usize, 13usize, 192u32, 2199452u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2199456u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2199460u32);
    emu.sb_no_count(11usize, 12usize, 1u32, 2199464u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2199468u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2199576u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219018));
}
#[inline(always)]
pub fn block_0x00218fac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 16u32, 2199472u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2199520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218fe0));
    } else {
        emu.pc = 2199476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218fb4));
    }
}
#[inline]
pub fn block_0x00218fb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 12u32, 2199480u32);
    emu.sli_no_count(14usize, 11usize, 20u32, 2199484u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2199488u32);
    emu.ori_no_count(13usize, 13usize, 224u32, 2199492u32);
    emu.sri_no_count(14usize, 14usize, 26u32, 2199496u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2199500u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2199504u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2199508u32);
    emu.sb_no_count(14usize, 12usize, 1u32, 2199512u32);
    emu.sb_no_count(11usize, 12usize, 2u32, 2199516u32);
    emu.add_memory_rw_events(11usize);
    let return_addr = 2199520u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2199576u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219018));
}
#[inline]
pub fn block_0x00218fe0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 18u32, 2199524u32);
    emu.sli_no_count(14usize, 11usize, 14u32, 2199528u32);
    emu.sli_no_count(15usize, 11usize, 20u32, 2199532u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2199536u32);
    emu.ori_no_count(13usize, 13usize, 240u32, 2199540u32);
    emu.sri_no_count(14usize, 14usize, 26u32, 2199544u32);
    emu.sri_no_count(15usize, 15usize, 26u32, 2199548u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2199552u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2199556u32);
    emu.adi_no_count(15usize, 15usize, 128u32, 2199560u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2199564u32);
    emu.sb_no_count(14usize, 12usize, 1u32, 2199568u32);
    emu.sb_no_count(15usize, 12usize, 2u32, 2199572u32);
    emu.sb_no_count(11usize, 12usize, 3u32, 2199576u32);
    emu.add_memory_rw_events(14usize);
    emu.pc = 2199576u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219018));
}
#[inline]
pub fn block_0x00219018(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 9usize, 8usize, 2199580u32);
    emu.sw_no_count(8usize, 10usize, 8u32, 2199584u32)?;
    emu.adi_no_count(10usize, 0usize, 0u32, 2199588u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2199592u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2199596u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2199600u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2199604u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2199608u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2199612u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2199616u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199620u32;
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
pub fn block_0x00219044(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2199624u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2199628u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2199632u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2199636u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2199640u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2199644u32)?;
    emu.adi_no_count(8usize, 12usize, 0u32, 2199648u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2199652u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2199656u32)?;
    emu.lw_no_count(18usize, 9usize, 8u32, 2199660u32)?;
    emu.sbr_no_count(10usize, 10usize, 18usize, 2199664u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2199728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002190b0));
    } else {
        emu.pc = 2199668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219074));
    }
}
#[inline(always)]
pub fn block_0x00219074(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 4u32, 2199672u32)?;
    emu.adr_no_count(10usize, 10usize, 18usize, 2199676u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2199680u32);
    emu.apc_no_count(1usize, 2199680u32, 4294901760u32, 2199684u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199688u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(2020u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00219088(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 18usize, 8usize, 2199692u32);
    emu.sw_no_count(8usize, 9usize, 8u32, 2199696u32)?;
    emu.adi_no_count(10usize, 0usize, 0u32, 2199700u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2199704u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2199708u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2199712u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2199716u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2199720u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2199724u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199728u32;
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
pub fn block_0x002190b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2199732u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2199736u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2199740u32);
    emu.adi_no_count(19usize, 11usize, 0u32, 2199744u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2199748u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2199752u32);
    emu.apc_no_count(1usize, 2199752u32, 4294963200u32, 2199756u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199760u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1016u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002190d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 19usize, 0u32, 2199764u32);
    emu.lw_no_count(18usize, 9usize, 8u32, 2199768u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2199772u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2199668u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219074));
}
#[inline]
pub fn block_0x002190dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2199776u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2199780u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2199784u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2199788u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2199792u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2199796u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2199800u32)?;
    emu.adi_no_count(8usize, 13usize, 0u32, 2199804u32);
    emu.adi_no_count(19usize, 12usize, 0u32, 2199808u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2199812u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2199816u32)?;
    emu.lw_no_count(20usize, 9usize, 8u32, 2199820u32)?;
    emu.sbr_no_count(11usize, 11usize, 20usize, 2199824u32);
    emu.adi_no_count(18usize, 10usize, 0u32, 2199828u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2199908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219164));
    } else {
        emu.pc = 2199832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219118));
    }
}
#[inline(always)]
pub fn block_0x00219118(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 4u32, 2199836u32)?;
    emu.adr_no_count(10usize, 10usize, 20usize, 2199840u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2199844u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2199848u32);
    emu.apc_no_count(1usize, 2199848u32, 4294901760u32, 2199852u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199856u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1852u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00219130(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(20usize, 20usize, 8usize, 2199860u32);
    emu.sw_no_count(20usize, 9usize, 8u32, 2199864u32)?;
    emu.adi_no_count(10usize, 0usize, 4u32, 2199868u32);
    emu.sb_no_count(10usize, 18usize, 0u32, 2199872u32);
    emu.sw_no_count(8usize, 18usize, 4u32, 2199876u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2199880u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2199884u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2199888u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2199892u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2199896u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2199900u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2199904u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199908u32;
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
pub fn block_0x00219164(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2199912u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2199916u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2199920u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2199924u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2199928u32);
    emu.apc_no_count(1usize, 2199928u32, 4294963200u32, 2199932u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199936u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(840u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00219180(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(20usize, 9usize, 8u32, 2199940u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2199944u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2199832u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219118));
}
#[inline]
pub fn block_0x00219188(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2199948u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2199952u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2199956u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2199960u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2199964u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2199968u32)?;
    emu.sw_no_count(20usize, 2usize, 24u32, 2199972u32)?;
    emu.sw_no_count(21usize, 2usize, 20u32, 2199976u32)?;
    emu.sw_no_count(22usize, 2usize, 16u32, 2199980u32)?;
    emu.sw_no_count(23usize, 2usize, 12u32, 2199984u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2199988u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2200088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219218));
    } else {
        emu.pc = 2199992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002191b8));
    }
}
#[inline(always)]
pub fn block_0x002191b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 12usize, 0u32, 2199996u32);
    emu.adi_no_count(19usize, 11usize, 0u32, 2200000u32);
    emu.adi_no_count(9usize, 0usize, 0u32, 2200004u32);
    emu.sli_no_count(22usize, 13usize, 3u32, 2200008u32);
    emu.adr_no_count(22usize, 12usize, 22usize, 2200012u32);
    emu.adi_no_count(10usize, 12usize, 4u32, 2200016u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2200016u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002191d0));
}
#[inline(always)]
pub fn block_0x002191d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2200020u32)?;
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2200024u32);
    emu.adr_no_count(9usize, 11usize, 9usize, 2200028u32);
    emu.adi_no_count(10usize, 10usize, 8u32, 2200032u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2200016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002191d0));
    } else {
        emu.pc = 2200036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002191e4));
    }
}
#[inline(always)]
pub fn block_0x002191e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 19usize, 0u32, 2200040u32)?;
    emu.lw_no_count(20usize, 19usize, 8u32, 2200044u32)?;
    emu.sbr_no_count(10usize, 10usize, 20usize, 2200048u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2200140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021924c));
    } else {
        emu.pc = 2200052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002191f4));
    }
}
#[inline(always)]
pub fn block_0x002191f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2200056u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2200060u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2200064u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2200068u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2200072u32);
    emu.apc_no_count(1usize, 2200072u32, 4294963200u32, 2200076u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200080u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(696u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00219210(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(20usize, 19usize, 8u32, 2200084u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2200088u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2200140u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021924c));
}
#[inline(always)]
pub fn block_0x00219218(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2200092u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2200096u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2200196u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219284));
}
#[inline(always)]
pub fn block_0x00219220(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2200100u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2200104u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2200108u32);
    emu.adi_no_count(23usize, 11usize, 0u32, 2200112u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2200116u32);
    emu.adi_no_count(12usize, 21usize, 0u32, 2200120u32);
    emu.apc_no_count(1usize, 2200120u32, 4294963200u32, 2200124u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200128u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(648u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00219240(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 23usize, 0u32, 2200132u32);
    emu.lw_no_count(20usize, 19usize, 8u32, 2200136u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2200140u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2200160u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219260));
}
#[inline(always)]
pub fn block_0x0021924c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 19usize, 0u32, 2200144u32)?;
    emu.lw_no_count(21usize, 18usize, 4u32, 2200148u32)?;
    emu.lw_no_count(11usize, 18usize, 0u32, 2200152u32)?;
    emu.sbr_no_count(10usize, 10usize, 20usize, 2200156u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2200096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219220));
    } else {
        emu.pc = 2200160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219260));
    }
}
#[inline(always)]
pub fn block_0x00219260(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 19usize, 4u32, 2200164u32)?;
    emu.adr_no_count(10usize, 10usize, 20usize, 2200168u32);
    emu.adi_no_count(12usize, 21usize, 0u32, 2200172u32);
    emu.apc_no_count(1usize, 2200172u32, 4294901760u32, 2200176u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200180u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1528u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00219274(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(20usize, 20usize, 21usize, 2200184u32);
    emu.adi_no_count(18usize, 18usize, 8u32, 2200188u32);
    emu.sw_no_count(20usize, 19usize, 8u32, 2200192u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2200140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021924c));
    } else {
        emu.pc = 2200196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219284));
    }
}
#[inline]
pub fn block_0x00219284(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2200200u32);
    emu.sb_no_count(10usize, 8usize, 0u32, 2200204u32);
    emu.sw_no_count(9usize, 8usize, 4u32, 2200208u32)?;
    emu.lw_no_count(1usize, 2usize, 44u32, 2200212u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2200216u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2200220u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2200224u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2200228u32)?;
    emu.lw_no_count(20usize, 2usize, 24u32, 2200232u32)?;
    emu.lw_no_count(21usize, 2usize, 20u32, 2200236u32)?;
    emu.lw_no_count(22usize, 2usize, 16u32, 2200240u32)?;
    emu.lw_no_count(23usize, 2usize, 12u32, 2200244u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2200248u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200252u32;
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
pub fn block_0x002192bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2200256u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200260u32;
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
pub fn block_0x002192c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2200264u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2200268u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2200272u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2200276u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2200280u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2200284u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2200288u32)?;
    emu.adi_no_count(9usize, 13usize, 0u32, 2200292u32);
    emu.adi_no_count(19usize, 12usize, 0u32, 2200296u32);
    emu.adi_no_count(8usize, 11usize, 0u32, 2200300u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2200304u32)?;
    emu.lw_no_count(20usize, 8usize, 8u32, 2200308u32)?;
    emu.sbr_no_count(11usize, 11usize, 20usize, 2200312u32);
    emu.adi_no_count(18usize, 10usize, 0u32, 2200316u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2200392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219348));
    } else {
        emu.pc = 2200320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219300));
    }
}
#[inline(always)]
pub fn block_0x00219300(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 4u32, 2200324u32)?;
    emu.adr_no_count(10usize, 10usize, 20usize, 2200328u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2200332u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2200336u32);
    emu.apc_no_count(1usize, 2200336u32, 4294901760u32, 2200340u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200344u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1364u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00219318(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(9usize, 20usize, 9usize, 2200348u32);
    emu.adi_no_count(10usize, 0usize, 4u32, 2200352u32);
    emu.sw_no_count(9usize, 8usize, 8u32, 2200356u32)?;
    emu.sb_no_count(10usize, 18usize, 0u32, 2200360u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2200364u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2200368u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2200372u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2200376u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2200380u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2200384u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2200388u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200392u32;
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
pub fn block_0x00219348(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2200396u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2200400u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2200404u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2200408u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2200412u32);
    emu.apc_no_count(1usize, 2200412u32, 4294963200u32, 2200416u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200420u32;
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
pub fn block_0x00219364(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(20usize, 8usize, 8u32, 2200424u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2200428u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2200320u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219300));
}
#[inline]
pub fn block_0x0021936c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2200432u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2200436u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2200440u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2200444u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2200448u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2200452u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2200456u32)?;
    emu.sw_no_count(21usize, 2usize, 4u32, 2200460u32)?;
    emu.sw_no_count(22usize, 2usize, 0u32, 2200464u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2200468u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2200664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219458));
    } else {
        emu.pc = 2200472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219398));
    }
}
#[inline(always)]
pub fn block_0x00219398(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 12usize, 0u32, 2200476u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2200480u32);
    emu.adi_no_count(12usize, 0usize, 0u32, 2200484u32);
    emu.sli_no_count(21usize, 13usize, 3u32, 2200488u32);
    emu.adr_no_count(21usize, 9usize, 21usize, 2200492u32);
    emu.adi_no_count(10usize, 9usize, 4u32, 2200496u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2200496u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002193b0));
}
#[inline(always)]
pub fn block_0x002193b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2200500u32)?;
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2200504u32);
    emu.adr_no_count(12usize, 11usize, 12usize, 2200508u32);
    emu.adi_no_count(10usize, 10usize, 8u32, 2200512u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2200496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002193b0));
    } else {
        emu.pc = 2200516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002193c4));
    }
}
#[inline(always)]
pub fn block_0x002193c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 0u32, 2200520u32)?;
    emu.lw_no_count(19usize, 18usize, 8u32, 2200524u32)?;
    emu.sbr_no_count(10usize, 10usize, 19usize, 2200528u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2200608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219420));
    } else {
        emu.pc = 2200532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002193d4));
    }
}
#[inline(always)]
pub fn block_0x002193d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2200536u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2200540u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2200544u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2200548u32);
    emu.apc_no_count(1usize, 2200548u32, 4294963200u32, 2200552u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200556u32;
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
pub fn block_0x002193ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(19usize, 18usize, 8u32, 2200560u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2200564u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2200608u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219420));
}
#[inline(always)]
pub fn block_0x002193f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2200568u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2200572u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2200576u32);
    emu.adi_no_count(22usize, 11usize, 0u32, 2200580u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2200584u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2200588u32);
    emu.apc_no_count(1usize, 2200588u32, 4294963200u32, 2200592u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200596u32;
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
pub fn block_0x00219414(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 22usize, 0u32, 2200600u32);
    emu.lw_no_count(19usize, 18usize, 8u32, 2200604u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2200608u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2200628u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219434));
}
#[inline(always)]
pub fn block_0x00219420(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 0u32, 2200612u32)?;
    emu.lw_no_count(20usize, 9usize, 4u32, 2200616u32)?;
    emu.lw_no_count(11usize, 9usize, 0u32, 2200620u32)?;
    emu.sbr_no_count(10usize, 10usize, 19usize, 2200624u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2200564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002193f4));
    } else {
        emu.pc = 2200628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219434));
    }
}
#[inline(always)]
pub fn block_0x00219434(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 4u32, 2200632u32)?;
    emu.adr_no_count(10usize, 10usize, 19usize, 2200636u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2200640u32);
    emu.apc_no_count(1usize, 2200640u32, 4294901760u32, 2200644u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200648u32;
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
pub fn block_0x00219448(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(19usize, 19usize, 20usize, 2200652u32);
    emu.adi_no_count(9usize, 9usize, 8u32, 2200656u32);
    emu.sw_no_count(19usize, 18usize, 8u32, 2200660u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2200608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219420));
    } else {
        emu.pc = 2200664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219458));
    }
}
#[inline]
pub fn block_0x00219458(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2200668u32);
    emu.sb_no_count(10usize, 8usize, 0u32, 2200672u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2200676u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2200680u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2200684u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2200688u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2200692u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2200696u32)?;
    emu.lw_no_count(21usize, 2usize, 4u32, 2200700u32)?;
    emu.lw_no_count(22usize, 2usize, 0u32, 2200704u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2200708u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200712u32;
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
pub fn block_0x00219488(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 4u32, 2200716u32);
    emu.sb_no_count(11usize, 10usize, 0u32, 2200720u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200724u32;
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
pub fn block_0x00219494(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2200728u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200732u32;
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
pub fn block_0x0021949c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(5usize, 2200732u32, 4096u32, 2200736u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(5usize);
    let return_addr = 2200740u32;
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
pub fn block_0x002194a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 10usize, 0u32, 2200744u32);
    emu.lbu_no_count(10usize, 10usize, 13u32, 2200748u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2200760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002194b8));
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
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2200752u32, 4096u32, 2200756u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200760u32;
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
pub fn block_0x002194b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 8usize, 8u32, 2200764u32)?;
    emu.lw_no_count(10usize, 8usize, 0u32, 2200768u32)?;
    emu.lw_no_count(11usize, 8usize, 4u32, 2200772u32)?;
    emu.adi_no_count(13usize, 0usize, 3u32, 2200776u32);
    emu.sb_no_count(13usize, 2usize, 7u32, 2200780u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2200784u32)?;
    emu.apc_no_count(1usize, 2200784u32, 0u32, 2200788u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200792u32;
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
pub fn block_0x002194d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 12u32, 2200796u32)?;
    emu.sw_no_count(11usize, 2usize, 16u32, 2200800u32)?;
    emu.adi_no_count(11usize, 2usize, 8u32, 2200804u32);
    emu.adi_no_count(12usize, 2usize, 12u32, 2200808u32);
    let a = 0u32.wrapping_add(2248704u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2200812u32;
    emu.update_insn_clock();
    emu.lbu_no_count(13usize, 10usize, 4294965800u32, 2200816u32);
    emu.adi_no_count(14usize, 2usize, 7u32, 2200820u32);
    emu.sw_no_count(11usize, 2usize, 20u32, 2200824u32)?;
    emu.sw_no_count(12usize, 2usize, 24u32, 2200828u32)?;
    emu.sw_no_count(14usize, 2usize, 28u32, 2200832u32)?;
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2200964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219584));
    } else {
        emu.pc = 2200836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219504));
    }
}
#[inline(always)]
pub fn block_0x00219504(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2248704u32);
    emu.write_reg_no_count(18usize, a);
    emu.pc = 2200840u32;
    emu.update_insn_clock();
    emu.lw_no_count(19usize, 18usize, 4294965804u32, 2200844u32)?;
    emu.adi_no_count(9usize, 0usize, 1u32, 2200848u32);
    emu.sb_no_count(9usize, 10usize, 4294965800u32, 2200852u32);
    emu.sw_no_count(0usize, 18usize, 4294965804u32, 2200856u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2200964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219584));
    } else {
        emu.pc = 2200860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021951c));
    }
}
#[inline(always)]
pub fn block_0x0021951c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 19usize, 8u32, 2200864u32);
    emu.apc_no_count(1usize, 2200864u32, 4096u32, 2200868u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200872u32;
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
pub fn block_0x00219528(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 11usize, 0u32, 2200876u32);
    emu.adi_no_count(11usize, 11usize, 4u32, 2200880u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2200884u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1528u32, 2200888u32);
    emu.adi_no_count(10usize, 2usize, 20u32, 2200892u32);
    emu.apc_no_count(1usize, 2200892u32, 0u32, 2200896u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200900u32;
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
pub fn block_0x00219544(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(0usize, 8usize, 0u32, 2200904u32);
    let a = 0u32.wrapping_add(2248704u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2200908u32;
    emu.update_insn_clock();
    emu.lw_no_count(10usize, 18usize, 4294965804u32, 2200912u32)?;
    emu.sb_no_count(9usize, 11usize, 4294965800u32, 2200916u32);
    emu.sw_no_count(19usize, 18usize, 4294965804u32, 2200920u32)?;
    emu.sw_no_count(9usize, 2usize, 32u32, 2200924u32)?;
    emu.sw_no_count(10usize, 2usize, 36u32, 2200928u32)?;
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2200988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021959c));
    } else {
        emu.pc = 2200932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219564));
    }
}
#[inline(always)]
pub fn block_0x00219564(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2200936u32)?;
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2200940u32);
    emu.sw_no_count(11usize, 10usize, 0u32, 2200944u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2200988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021959c));
    } else {
        emu.pc = 2200948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219574));
    }
}
#[inline(always)]
pub fn block_0x00219574(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 36u32, 2200952u32);
    emu.apc_no_count(1usize, 2200952u32, 4096u32, 2200956u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200960u32;
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
pub fn block_0x00219580(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2200964u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2200988u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021959c));
}
#[inline(always)]
pub fn block_0x00219584(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2200968u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1568u32, 2200972u32);
    emu.adi_no_count(10usize, 2usize, 20u32, 2200976u32);
    emu.adi_no_count(11usize, 2usize, 43u32, 2200980u32);
    emu.apc_no_count(1usize, 2200980u32, 0u32, 2200984u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200988u32;
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
pub fn block_0x0021959c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2200988u32, 4096u32, 2200992u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2200996u32;
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
