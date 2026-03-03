pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2130600u32;
pub const PC_MAX: u32 = 2132956u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 110usize] = [
        block_0x002082a8,
        block_0x002082cc,
        block_0x002082e8,
        block_0x0020831c,
        block_0x00208328,
        block_0x00208330,
        block_0x00208350,
        block_0x00208358,
        block_0x00208388,
        block_0x002083c4,
        block_0x002083ec,
        block_0x002083f0,
        block_0x00208414,
        block_0x00208418,
        block_0x00208428,
        block_0x00208440,
        block_0x00208454,
        block_0x0020845c,
        block_0x0020846c,
        block_0x00208474,
        block_0x00208480,
        block_0x002084a8,
        block_0x002084b8,
        block_0x002084c0,
        block_0x002084c8,
        block_0x002084d8,
        block_0x002084ec,
        block_0x002084f0,
        block_0x00208504,
        block_0x00208510,
        block_0x00208518,
        block_0x00208538,
        block_0x00208540,
        block_0x00208570,
        block_0x002085ac,
        block_0x002085bc,
        block_0x002085cc,
        block_0x002085ec,
        block_0x002085f8,
        block_0x00208600,
        block_0x00208620,
        block_0x00208628,
        block_0x00208658,
        block_0x00208694,
        block_0x002086a8,
        block_0x002086c0,
        block_0x002086e4,
        block_0x00208700,
        block_0x00208708,
        block_0x0020871c,
        block_0x00208730,
        block_0x00208744,
        block_0x00208768,
        block_0x00208770,
        block_0x00208788,
        block_0x00208798,
        block_0x002087a0,
        block_0x002087a8,
        block_0x002087b8,
        block_0x002087e0,
        block_0x00208818,
        block_0x0020881c,
        block_0x00208824,
        block_0x00208834,
        block_0x0020884c,
        block_0x0020885c,
        block_0x00208864,
        block_0x00208874,
        block_0x0020887c,
        block_0x00208884,
        block_0x00208894,
        block_0x002088a8,
        block_0x002088c0,
        block_0x002088f8,
        block_0x00208930,
        block_0x00208934,
        block_0x0020893c,
        block_0x0020894c,
        block_0x00208964,
        block_0x00208974,
        block_0x0020897c,
        block_0x0020898c,
        block_0x00208994,
        block_0x0020899c,
        block_0x002089ac,
        block_0x002089c0,
        block_0x002089d8,
        block_0x00208a10,
        block_0x00208a48,
        block_0x00208a4c,
        block_0x00208a54,
        block_0x00208a64,
        block_0x00208a7c,
        block_0x00208a8c,
        block_0x00208a94,
        block_0x00208aa4,
        block_0x00208aac,
        block_0x00208ab4,
        block_0x00208ac4,
        block_0x00208ad8,
        block_0x00208af0,
        block_0x00208b28,
        block_0x00208b2c,
        block_0x00208b40,
        block_0x00208b48,
        block_0x00208b50,
        block_0x00208b80,
        block_0x00208b94,
        block_0x00208bbc,
        block_0x00208bdc,
    ];
    const IDX: [u16; 590usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 3u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 4u16, 0u16, 0u16, 5u16, 0u16, 6u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 7u16, 0u16, 8u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 9u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 10u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 11u16, 12u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        13u16, 14u16, 0u16, 0u16, 0u16, 15u16, 0u16, 0u16, 0u16, 0u16, 0u16, 16u16, 0u16,
        0u16, 0u16, 0u16, 17u16, 0u16, 18u16, 0u16, 0u16, 0u16, 19u16, 0u16, 20u16, 0u16,
        0u16, 21u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 22u16, 0u16,
        0u16, 0u16, 23u16, 0u16, 24u16, 0u16, 25u16, 0u16, 0u16, 0u16, 26u16, 0u16, 0u16,
        0u16, 0u16, 27u16, 28u16, 0u16, 0u16, 0u16, 0u16, 29u16, 0u16, 0u16, 30u16, 0u16,
        31u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 32u16, 0u16, 33u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 34u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 35u16, 0u16,
        0u16, 0u16, 36u16, 0u16, 0u16, 0u16, 37u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 38u16, 0u16, 0u16, 39u16, 0u16, 40u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 41u16, 0u16, 42u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 43u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 44u16, 0u16, 0u16, 0u16, 0u16, 45u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 46u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 47u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 48u16, 0u16, 49u16, 0u16, 0u16, 0u16, 0u16, 50u16,
        0u16, 0u16, 0u16, 0u16, 51u16, 0u16, 0u16, 0u16, 0u16, 52u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 53u16, 0u16, 54u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        55u16, 0u16, 0u16, 0u16, 56u16, 0u16, 57u16, 0u16, 58u16, 0u16, 0u16, 0u16,
        59u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 60u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 61u16, 62u16,
        0u16, 63u16, 0u16, 0u16, 0u16, 64u16, 0u16, 0u16, 0u16, 0u16, 0u16, 65u16, 0u16,
        0u16, 0u16, 66u16, 0u16, 67u16, 0u16, 0u16, 0u16, 68u16, 0u16, 69u16, 0u16,
        70u16, 0u16, 0u16, 0u16, 71u16, 0u16, 0u16, 0u16, 0u16, 72u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 73u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 74u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 75u16, 76u16, 0u16, 77u16, 0u16, 0u16, 0u16, 78u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 79u16, 0u16, 0u16, 0u16, 80u16, 0u16, 81u16, 0u16, 0u16,
        0u16, 82u16, 0u16, 83u16, 0u16, 84u16, 0u16, 0u16, 0u16, 85u16, 0u16, 0u16, 0u16,
        0u16, 86u16, 0u16, 0u16, 0u16, 0u16, 0u16, 87u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 88u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 89u16, 90u16, 0u16, 91u16,
        0u16, 0u16, 0u16, 92u16, 0u16, 0u16, 0u16, 0u16, 0u16, 93u16, 0u16, 0u16, 0u16,
        94u16, 0u16, 95u16, 0u16, 0u16, 0u16, 96u16, 0u16, 97u16, 0u16, 98u16, 0u16,
        0u16, 0u16, 99u16, 0u16, 0u16, 0u16, 0u16, 100u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        101u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 102u16, 103u16, 0u16, 0u16, 0u16, 0u16, 104u16, 0u16, 105u16, 0u16, 106u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 107u16, 0u16,
        0u16, 0u16, 0u16, 108u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        109u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 110u16,
    ];
    if pc < 2130600u32 || pc > 2132956u32 {
        return None;
    }
    let word_offset = ((pc - 2130600u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x002082a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(13usize, 10usize, 4u32, 2130604u32);
    emu.lw_no_count(12usize, 10usize, 0u32, 2130608u32)?;
    emu.adi_no_count(14usize, 11usize, 4294967250u32, 2130612u32);
    emu.sltiu_no_count(14usize, 14usize, 1u32, 2130616u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2130620u32);
    emu.sb_no_count(13usize, 10usize, 4u32, 2130624u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2130628u32);
    emu.apc_no_count(6usize, 2130628u32, 36864u32, 2130632u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2130636u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294967124u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002082cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2130640u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1739u32, 2130644u32);
    emu.adi_no_count(12usize, 0usize, 5u32, 2130648u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2130652u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2130656u32);
    emu.apc_no_count(6usize, 2130656u32, 36864u32, 2130660u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2130664u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966160u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002082e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2130668u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2130672u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2130676u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2130680u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2130684u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2130688u32)?;
    emu.sw_no_count(20usize, 2usize, 24u32, 2130692u32)?;
    emu.sw_no_count(21usize, 2usize, 20u32, 2130696u32)?;
    emu.sw_no_count(22usize, 2usize, 16u32, 2130700u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2130704u32);
    emu.adi_no_count(10usize, 0usize, 128u32, 2130708u32);
    emu.sw_no_count(0usize, 2usize, 12u32, 2130712u32)?;
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2130728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208328));
    } else {
        emu.pc = 2130716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020831c));
    }
}
#[inline(always)]
pub fn block_0x0020831c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 2usize, 12u32, 2130720u32);
    emu.adi_no_count(18usize, 0usize, 1u32, 2130724u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2130728u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2130884u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002083c4));
}
#[inline(always)]
pub fn block_0x00208328(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 11u32, 2130732u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2130768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208350));
    } else {
        emu.pc = 2130736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208330));
    }
}
#[inline(always)]
pub fn block_0x00208330(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 6u32, 2130740u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2130744u32);
    emu.ori_no_count(10usize, 10usize, 192u32, 2130748u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2130752u32);
    emu.sb_no_count(10usize, 2usize, 12u32, 2130756u32);
    emu.sb_no_count(11usize, 2usize, 13u32, 2130760u32);
    emu.adi_no_count(18usize, 0usize, 2u32, 2130764u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2130768u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2130884u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002083c4));
}
#[inline(always)]
pub fn block_0x00208350(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 16u32, 2130772u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2130824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208388));
    } else {
        emu.pc = 2130776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208358));
    }
}
#[inline]
pub fn block_0x00208358(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 12u32, 2130780u32);
    emu.sli_no_count(12usize, 11usize, 20u32, 2130784u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2130788u32);
    emu.ori_no_count(10usize, 10usize, 224u32, 2130792u32);
    emu.sri_no_count(12usize, 12usize, 26u32, 2130796u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2130800u32);
    emu.adi_no_count(12usize, 12usize, 128u32, 2130804u32);
    emu.sb_no_count(10usize, 2usize, 12u32, 2130808u32);
    emu.sb_no_count(12usize, 2usize, 13u32, 2130812u32);
    emu.sb_no_count(11usize, 2usize, 14u32, 2130816u32);
    emu.adi_no_count(18usize, 0usize, 3u32, 2130820u32);
    emu.add_memory_rw_events(12usize);
    let return_addr = 2130824u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2130884u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002083c4));
}
#[inline]
pub fn block_0x00208388(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 18u32, 2130828u32);
    emu.sli_no_count(12usize, 11usize, 14u32, 2130832u32);
    emu.sli_no_count(13usize, 11usize, 20u32, 2130836u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2130840u32);
    emu.ori_no_count(10usize, 10usize, 240u32, 2130844u32);
    emu.sri_no_count(12usize, 12usize, 26u32, 2130848u32);
    emu.sri_no_count(13usize, 13usize, 26u32, 2130852u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2130856u32);
    emu.adi_no_count(12usize, 12usize, 128u32, 2130860u32);
    emu.adi_no_count(13usize, 13usize, 128u32, 2130864u32);
    emu.sb_no_count(10usize, 2usize, 12u32, 2130868u32);
    emu.sb_no_count(12usize, 2usize, 13u32, 2130872u32);
    emu.sb_no_count(13usize, 2usize, 14u32, 2130876u32);
    emu.sb_no_count(11usize, 2usize, 15u32, 2130880u32);
    emu.adi_no_count(18usize, 0usize, 4u32, 2130884u32);
    emu.add_memory_rw_events(15usize);
    emu.pc = 2130884u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002083c4));
}
#[inline]
pub fn block_0x002083c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(19usize, 8usize, 8u32, 2130888u32)?;
    emu.lw_no_count(10usize, 19usize, 4u32, 2130892u32)?;
    emu.lw_no_count(21usize, 19usize, 8u32, 2130896u32)?;
    emu.lw_no_count(20usize, 19usize, 12u32, 2130900u32)?;
    emu.lw_no_count(11usize, 19usize, 0u32, 2130904u32)?;
    emu.sltru_no_count(12usize, 21usize, 10usize, 2130908u32);
    emu.sltiu_no_count(13usize, 20usize, 1u32, 2130912u32);
    emu.anr_no_count(14usize, 13usize, 12usize, 2130916u32);
    emu.adi_no_count(12usize, 21usize, 0u32, 2130920u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a != b {
        emu.pc = 2130928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002083f0));
    } else {
        emu.pc = 2130924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002083ec));
    }
}
#[inline(always)]
pub fn block_0x002083ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2130928u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2130928u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002083f0));
}
#[inline]
pub fn block_0x002083f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2130932u32);
    emu.orr_no_count(13usize, 13usize, 21usize, 2130936u32);
    emu.sbr_no_count(13usize, 10usize, 13usize, 2130940u32);
    emu.sltru_no_count(10usize, 10usize, 13usize, 2130944u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2130948u32);
    emu.anr_no_count(22usize, 10usize, 13usize, 2130952u32);
    emu.adr_no_count(10usize, 11usize, 12usize, 2130956u32);
    emu.adi_no_count(9usize, 22usize, 0u32, 2130960u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a < b {
        emu.pc = 2130968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208418));
    } else {
        emu.pc = 2130964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208414));
    }
}
#[inline(always)]
pub fn block_0x00208414(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 18usize, 0u32, 2130968u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2130968u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208418));
}
#[inline(always)]
pub fn block_0x00208418(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 2usize, 12u32, 2130972u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2130976u32);
    emu.apc_no_count(1usize, 2130976u32, 4294946816u32, 2130980u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130984u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966356u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208428(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(9usize, 21usize, 9usize, 2130988u32);
    emu.sltru_no_count(10usize, 9usize, 21usize, 2130992u32);
    emu.adr_no_count(10usize, 20usize, 10usize, 2130996u32);
    emu.sw_no_count(9usize, 19usize, 8u32, 2131000u32)?;
    emu.sw_no_count(10usize, 19usize, 12u32, 2131004u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a >= b {
        emu.pc = 2131028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208454));
    } else {
        emu.pc = 2131008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208440));
    }
}
#[inline(always)]
pub fn block_0x00208440(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2131012u32;
    emu.update_insn_clock();
    emu.lw_no_count(19usize, 10usize, 1856u32, 2131016u32)?;
    emu.ani_no_count(12usize, 19usize, 255u32, 2131020u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2131024u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2131036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020845c));
    } else {
        emu.pc = 2131028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208454));
    }
}
#[inline(always)]
pub fn block_0x00208454(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2131032u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2131036u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2131072u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208480));
}
#[inline(always)]
pub fn block_0x0020845c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(20usize, 10usize, 1860u32, 2131040u32)?;
    emu.lbu_no_count(10usize, 8usize, 0u32, 2131044u32);
    emu.lw_no_count(9usize, 8usize, 4u32, 2131048u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2131112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002084a8));
    } else {
        emu.pc = 2131052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020846c));
    }
}
#[inline(always)]
pub fn block_0x0020846c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 3u32, 2131056u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2131112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002084a8));
    } else {
        emu.pc = 2131060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208474));
    }
}
#[inline(always)]
pub fn block_0x00208474(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(19usize, 8usize, 0u32, 2131064u32)?;
    emu.sw_no_count(20usize, 8usize, 4u32, 2131068u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2131072u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2131072u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208480));
}
#[inline]
pub fn block_0x00208480(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2131076u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2131080u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2131084u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2131088u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2131092u32)?;
    emu.lw_no_count(20usize, 2usize, 24u32, 2131096u32)?;
    emu.lw_no_count(21usize, 2usize, 20u32, 2131100u32)?;
    emu.lw_no_count(22usize, 2usize, 16u32, 2131104u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2131108u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131112u32;
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
pub fn block_0x002084a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(21usize, 9usize, 4u32, 2131116u32)?;
    emu.lw_no_count(11usize, 21usize, 0u32, 2131120u32)?;
    emu.lw_no_count(18usize, 9usize, 0u32, 2131124u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2131136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002084c0));
    } else {
        emu.pc = 2131128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002084b8));
    }
}
#[inline(always)]
pub fn block_0x002084b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2131132u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2131136u32;
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
pub fn block_0x002084c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 21usize, 4u32, 2131140u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2131160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002084d8));
    } else {
        emu.pc = 2131144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002084c8));
    }
}
#[inline(always)]
pub fn block_0x002084c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 21usize, 8u32, 2131148u32)?;
    emu.adi_no_count(10usize, 18usize, 0u32, 2131152u32);
    emu.apc_no_count(1usize, 2131152u32, 4294938624u32, 2131156u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131160u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966376u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002084d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 12u32, 2131164u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2131168u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2131172u32);
    emu.apc_no_count(1usize, 2131172u32, 4294938624u32, 2131176u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131180u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966356u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002084ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2131184u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2131060u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208474));
}
#[inline(always)]
pub fn block_0x002084f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2131188u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2131192u32)?;
    emu.adi_no_count(10usize, 0usize, 128u32, 2131196u32);
    emu.sw_no_count(0usize, 2usize, 8u32, 2131200u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2131216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208510));
    } else {
        emu.pc = 2131204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208504));
    }
}
#[inline(always)]
pub fn block_0x00208504(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 2usize, 8u32, 2131208u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2131212u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2131216u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2131372u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002085ac));
}
#[inline(always)]
pub fn block_0x00208510(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 11u32, 2131220u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2131256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208538));
    } else {
        emu.pc = 2131224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208518));
    }
}
#[inline(always)]
pub fn block_0x00208518(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 6u32, 2131228u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2131232u32);
    emu.ori_no_count(10usize, 10usize, 192u32, 2131236u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2131240u32);
    emu.sb_no_count(10usize, 2usize, 8u32, 2131244u32);
    emu.sb_no_count(11usize, 2usize, 9u32, 2131248u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2131252u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2131256u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2131372u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002085ac));
}
#[inline(always)]
pub fn block_0x00208538(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 16u32, 2131260u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2131312u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208570));
    } else {
        emu.pc = 2131264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208540));
    }
}
#[inline]
pub fn block_0x00208540(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 12u32, 2131268u32);
    emu.sli_no_count(12usize, 11usize, 20u32, 2131272u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2131276u32);
    emu.ori_no_count(10usize, 10usize, 224u32, 2131280u32);
    emu.sri_no_count(12usize, 12usize, 26u32, 2131284u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2131288u32);
    emu.adi_no_count(12usize, 12usize, 128u32, 2131292u32);
    emu.sb_no_count(10usize, 2usize, 8u32, 2131296u32);
    emu.sb_no_count(12usize, 2usize, 9u32, 2131300u32);
    emu.sb_no_count(11usize, 2usize, 10u32, 2131304u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2131308u32);
    emu.add_memory_rw_events(12usize);
    let return_addr = 2131312u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2131372u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002085ac));
}
#[inline]
pub fn block_0x00208570(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 18u32, 2131316u32);
    emu.sli_no_count(12usize, 11usize, 14u32, 2131320u32);
    emu.sli_no_count(13usize, 11usize, 20u32, 2131324u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2131328u32);
    emu.ori_no_count(10usize, 10usize, 240u32, 2131332u32);
    emu.sri_no_count(12usize, 12usize, 26u32, 2131336u32);
    emu.sri_no_count(13usize, 13usize, 26u32, 2131340u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2131344u32);
    emu.adi_no_count(12usize, 12usize, 128u32, 2131348u32);
    emu.adi_no_count(13usize, 13usize, 128u32, 2131352u32);
    emu.sb_no_count(10usize, 2usize, 8u32, 2131356u32);
    emu.sb_no_count(12usize, 2usize, 9u32, 2131360u32);
    emu.sb_no_count(13usize, 2usize, 10u32, 2131364u32);
    emu.sb_no_count(11usize, 2usize, 11u32, 2131368u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2131372u32);
    emu.add_memory_rw_events(15usize);
    emu.pc = 2131372u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002085ac));
}
#[inline(always)]
pub fn block_0x002085ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2131376u32);
    emu.adi_no_count(11usize, 2usize, 8u32, 2131380u32);
    emu.apc_no_count(1usize, 2131380u32, 4294946816u32, 2131384u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131388u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965680u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002085bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2131392u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2131396u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2131400u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131404u32;
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
pub fn block_0x002085cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2131408u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2131412u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2131416u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2131420u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2131424u32)?;
    emu.adi_no_count(12usize, 0usize, 128u32, 2131428u32);
    emu.sw_no_count(0usize, 2usize, 12u32, 2131432u32)?;
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2131448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002085f8));
    } else {
        emu.pc = 2131436u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002085ec));
    }
}
#[inline(always)]
pub fn block_0x002085ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 2usize, 12u32, 2131440u32);
    emu.adi_no_count(8usize, 0usize, 1u32, 2131444u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2131448u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2131604u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208694));
}
#[inline(always)]
pub fn block_0x002085f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 11usize, 11u32, 2131452u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2131488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208620));
    } else {
        emu.pc = 2131456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208600));
    }
}
#[inline(always)]
pub fn block_0x00208600(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 11usize, 6u32, 2131460u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2131464u32);
    emu.ori_no_count(12usize, 12usize, 192u32, 2131468u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2131472u32);
    emu.sb_no_count(12usize, 2usize, 12u32, 2131476u32);
    emu.sb_no_count(11usize, 2usize, 13u32, 2131480u32);
    emu.adi_no_count(8usize, 0usize, 2u32, 2131484u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2131488u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2131604u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208694));
}
#[inline(always)]
pub fn block_0x00208620(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 11usize, 16u32, 2131492u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2131544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208658));
    } else {
        emu.pc = 2131496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208628));
    }
}
#[inline]
pub fn block_0x00208628(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 11usize, 12u32, 2131500u32);
    emu.sli_no_count(13usize, 11usize, 20u32, 2131504u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2131508u32);
    emu.ori_no_count(12usize, 12usize, 224u32, 2131512u32);
    emu.sri_no_count(13usize, 13usize, 26u32, 2131516u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2131520u32);
    emu.adi_no_count(13usize, 13usize, 128u32, 2131524u32);
    emu.sb_no_count(12usize, 2usize, 12u32, 2131528u32);
    emu.sb_no_count(13usize, 2usize, 13u32, 2131532u32);
    emu.sb_no_count(11usize, 2usize, 14u32, 2131536u32);
    emu.adi_no_count(8usize, 0usize, 3u32, 2131540u32);
    emu.add_memory_rw_events(12usize);
    let return_addr = 2131544u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2131604u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208694));
}
#[inline]
pub fn block_0x00208658(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 11usize, 18u32, 2131548u32);
    emu.sli_no_count(13usize, 11usize, 14u32, 2131552u32);
    emu.sli_no_count(14usize, 11usize, 20u32, 2131556u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2131560u32);
    emu.ori_no_count(12usize, 12usize, 240u32, 2131564u32);
    emu.sri_no_count(13usize, 13usize, 26u32, 2131568u32);
    emu.sri_no_count(14usize, 14usize, 26u32, 2131572u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2131576u32);
    emu.adi_no_count(13usize, 13usize, 128u32, 2131580u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2131584u32);
    emu.sb_no_count(12usize, 2usize, 12u32, 2131588u32);
    emu.sb_no_count(13usize, 2usize, 13u32, 2131592u32);
    emu.sb_no_count(14usize, 2usize, 14u32, 2131596u32);
    emu.sb_no_count(11usize, 2usize, 15u32, 2131600u32);
    emu.adi_no_count(8usize, 0usize, 4u32, 2131604u32);
    emu.add_memory_rw_events(15usize);
    emu.pc = 2131604u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208694));
}
#[inline(always)]
pub fn block_0x00208694(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(9usize, 10usize, 8u32, 2131608u32)?;
    emu.lw_no_count(10usize, 9usize, 0u32, 2131612u32)?;
    emu.lw_no_count(18usize, 9usize, 8u32, 2131616u32)?;
    emu.sbr_no_count(10usize, 10usize, 18usize, 2131620u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2131684u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002086e4));
    } else {
        emu.pc = 2131624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002086a8));
    }
}
#[inline(always)]
pub fn block_0x002086a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 4u32, 2131628u32)?;
    emu.adr_no_count(10usize, 10usize, 18usize, 2131632u32);
    emu.adi_no_count(11usize, 2usize, 12u32, 2131636u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2131640u32);
    emu.apc_no_count(1usize, 2131640u32, 4294946816u32, 2131644u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131648u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965692u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002086c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 18usize, 8usize, 2131652u32);
    emu.sw_no_count(8usize, 9usize, 8u32, 2131656u32)?;
    emu.adi_no_count(10usize, 0usize, 0u32, 2131660u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2131664u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2131668u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2131672u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2131676u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2131680u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131684u32;
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
pub fn block_0x002086e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2131688u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2131692u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2131696u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2131700u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2131704u32);
    emu.apc_no_count(1usize, 2131704u32, 0u32, 2131708u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131712u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1788u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208700(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 9usize, 8u32, 2131716u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2131720u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2131624u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002086a8));
}
#[inline(always)]
pub fn block_0x00208708(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 0u32, 2131724u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2131728u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1744u32, 2131732u32);
    emu.apc_no_count(6usize, 2131732u32, 32768u32, 2131736u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2131740u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965428u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020871c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 0u32, 2131744u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2131748u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1792u32, 2131752u32);
    emu.apc_no_count(6usize, 2131752u32, 32768u32, 2131756u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2131760u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965408u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208730(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 0u32, 2131764u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2131768u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1768u32, 2131772u32);
    emu.apc_no_count(6usize, 2131772u32, 32768u32, 2131776u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2131780u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965388u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00208744(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2131784u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2131788u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2131792u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2131796u32)?;
    emu.sw_no_count(18usize, 2usize, 0u32, 2131800u32)?;
    emu.lbu_no_count(11usize, 10usize, 0u32, 2131804u32);
    emu.lw_no_count(8usize, 10usize, 4u32, 2131808u32)?;
    emu.adi_no_count(10usize, 0usize, 4u32, 2131812u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2131848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208788));
    } else {
        emu.pc = 2131816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208768));
    }
}
#[inline(always)]
pub fn block_0x00208768(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 3u32, 2131820u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2131848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208788));
    } else {
        emu.pc = 2131824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208770));
    }
}
#[inline(always)]
pub fn block_0x00208770(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2131828u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2131832u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2131836u32)?;
    emu.lw_no_count(18usize, 2usize, 0u32, 2131840u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2131844u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131848u32;
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
pub fn block_0x00208788(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 8usize, 4u32, 2131852u32)?;
    emu.lw_no_count(11usize, 18usize, 0u32, 2131856u32)?;
    emu.lw_no_count(9usize, 8usize, 0u32, 2131860u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2131872u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002087a0));
    } else {
        emu.pc = 2131864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208798));
    }
}
#[inline(always)]
pub fn block_0x00208798(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2131868u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2131872u32;
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
pub fn block_0x002087a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 18usize, 4u32, 2131876u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2131896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002087b8));
    } else {
        emu.pc = 2131880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002087a8));
    }
}
#[inline(always)]
pub fn block_0x002087a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 8u32, 2131884u32)?;
    emu.adi_no_count(10usize, 9usize, 0u32, 2131888u32);
    emu.apc_no_count(1usize, 2131888u32, 4294938624u32, 2131892u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131896u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965640u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002087b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 12u32, 2131900u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2131904u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2131908u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2131912u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2131916u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2131920u32)?;
    emu.lw_no_count(18usize, 2usize, 0u32, 2131924u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2131928u32);
    emu.apc_no_count(6usize, 2131928u32, 4294938624u32, 2131932u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2131936u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965600u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002087e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2131940u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2131944u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2131948u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2131952u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2131956u32)?;
    emu.adi_no_count(9usize, 10usize, 0u32, 2131960u32);
    emu.adi_no_count(18usize, 0usize, 4u32, 2131964u32);
    emu.sb_no_count(18usize, 2usize, 8u32, 2131968u32);
    emu.sw_no_count(11usize, 2usize, 16u32, 2131972u32)?;
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2131976u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1768u32, 2131980u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2131984u32);
    emu.apc_no_count(1usize, 2131984u32, 28672u32, 2131988u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131992u32;
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
pub fn block_0x00208818(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2132044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020884c));
    } else {
        emu.pc = 2131996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020881c));
    }
}
#[inline(always)]
pub fn block_0x0020881c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 8u32, 2132000u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2132160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002088c0));
    } else {
        emu.pc = 2132004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208824));
    }
}
#[inline(always)]
pub fn block_0x00208824(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2132008u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2132012u32)?;
    emu.sw_no_count(10usize, 9usize, 0u32, 2132016u32)?;
    emu.sw_no_count(11usize, 9usize, 4u32, 2132020u32)?;
    emu.add_memory_rw_events(4usize);
    emu.pc = 2132020u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208834));
}
#[inline(always)]
pub fn block_0x00208834(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2132024u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2132028u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2132032u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2132036u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2132040u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2132044u32;
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
pub fn block_0x0020884c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 8u32, 2132048u32);
    emu.lw_no_count(8usize, 2usize, 12u32, 2132052u32)?;
    emu.sb_no_count(18usize, 9usize, 0u32, 2132056u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a < b {
        emu.pc = 2132068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208864));
    } else {
        emu.pc = 2132060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020885c));
    }
}
#[inline(always)]
pub fn block_0x0020885c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 3u32, 2132064u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2132020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208834));
    } else {
        emu.pc = 2132068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208864));
    }
}
#[inline(always)]
pub fn block_0x00208864(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 8usize, 4u32, 2132072u32)?;
    emu.lw_no_count(11usize, 18usize, 0u32, 2132076u32)?;
    emu.lw_no_count(9usize, 8usize, 0u32, 2132080u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2132092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020887c));
    } else {
        emu.pc = 2132084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208874));
    }
}
#[inline(always)]
pub fn block_0x00208874(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2132088u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2132092u32;
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
pub fn block_0x0020887c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 18usize, 4u32, 2132096u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2132116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208894));
    } else {
        emu.pc = 2132100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208884));
    }
}
#[inline(always)]
pub fn block_0x00208884(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 8u32, 2132104u32)?;
    emu.adi_no_count(10usize, 9usize, 0u32, 2132108u32);
    emu.apc_no_count(1usize, 2132108u32, 4294938624u32, 2132112u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2132116u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965420u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208894(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 12u32, 2132120u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2132124u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2132128u32);
    emu.apc_no_count(1usize, 2132128u32, 4294938624u32, 2132132u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2132136u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965400u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002088a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2132140u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2132144u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2132148u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2132152u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2132156u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2132160u32;
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
pub fn block_0x002088c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2132164u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1952u32, 2132168u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2132172u32);
    emu.sw_no_count(0usize, 2usize, 40u32, 2132176u32)?;
    emu.adi_no_count(12usize, 0usize, 4u32, 2132180u32);
    emu.sw_no_count(10usize, 2usize, 24u32, 2132184u32)?;
    emu.sw_no_count(11usize, 2usize, 28u32, 2132188u32)?;
    emu.sw_no_count(12usize, 2usize, 32u32, 2132192u32)?;
    emu.sw_no_count(0usize, 2usize, 36u32, 2132196u32)?;
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2132200u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1960u32, 2132204u32);
    emu.adi_no_count(10usize, 2usize, 24u32, 2132208u32);
    emu.apc_no_count(1usize, 2132208u32, 16384u32, 2132212u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2132216u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966116u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002088f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2132220u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2132224u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2132228u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2132232u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2132236u32)?;
    emu.adi_no_count(9usize, 10usize, 0u32, 2132240u32);
    emu.adi_no_count(18usize, 0usize, 4u32, 2132244u32);
    emu.sb_no_count(18usize, 2usize, 8u32, 2132248u32);
    emu.sw_no_count(11usize, 2usize, 16u32, 2132252u32)?;
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2132256u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1744u32, 2132260u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2132264u32);
    emu.apc_no_count(1usize, 2132264u32, 28672u32, 2132268u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2132272u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1696u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208930(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2132324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208964));
    } else {
        emu.pc = 2132276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208934));
    }
}
#[inline(always)]
pub fn block_0x00208934(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 8u32, 2132280u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2132440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002089d8));
    } else {
        emu.pc = 2132284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020893c));
    }
}
#[inline(always)]
pub fn block_0x0020893c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2132288u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2132292u32)?;
    emu.sw_no_count(10usize, 9usize, 0u32, 2132296u32)?;
    emu.sw_no_count(11usize, 9usize, 4u32, 2132300u32)?;
    emu.add_memory_rw_events(4usize);
    emu.pc = 2132300u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020894c));
}
#[inline(always)]
pub fn block_0x0020894c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2132304u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2132308u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2132312u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2132316u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2132320u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2132324u32;
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
pub fn block_0x00208964(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 8u32, 2132328u32);
    emu.lw_no_count(8usize, 2usize, 12u32, 2132332u32)?;
    emu.sb_no_count(18usize, 9usize, 0u32, 2132336u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a < b {
        emu.pc = 2132348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020897c));
    } else {
        emu.pc = 2132340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208974));
    }
}
#[inline(always)]
pub fn block_0x00208974(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 3u32, 2132344u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2132300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020894c));
    } else {
        emu.pc = 2132348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020897c));
    }
}
#[inline(always)]
pub fn block_0x0020897c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 8usize, 4u32, 2132352u32)?;
    emu.lw_no_count(11usize, 18usize, 0u32, 2132356u32)?;
    emu.lw_no_count(9usize, 8usize, 0u32, 2132360u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2132372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208994));
    } else {
        emu.pc = 2132364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020898c));
    }
}
#[inline(always)]
pub fn block_0x0020898c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2132368u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2132372u32;
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
pub fn block_0x00208994(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 18usize, 4u32, 2132376u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2132396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002089ac));
    } else {
        emu.pc = 2132380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020899c));
    }
}
#[inline(always)]
pub fn block_0x0020899c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 8u32, 2132384u32)?;
    emu.adi_no_count(10usize, 9usize, 0u32, 2132388u32);
    emu.apc_no_count(1usize, 2132388u32, 4294934528u32, 2132392u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2132396u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1940u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002089ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 12u32, 2132400u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2132404u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2132408u32);
    emu.apc_no_count(1usize, 2132408u32, 4294934528u32, 2132412u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2132416u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1920u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002089c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2132420u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2132424u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2132428u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2132432u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2132436u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2132440u32;
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
pub fn block_0x002089d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2132444u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1952u32, 2132448u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2132452u32);
    emu.sw_no_count(0usize, 2usize, 40u32, 2132456u32)?;
    emu.adi_no_count(12usize, 0usize, 4u32, 2132460u32);
    emu.sw_no_count(10usize, 2usize, 24u32, 2132464u32)?;
    emu.sw_no_count(11usize, 2usize, 28u32, 2132468u32)?;
    emu.sw_no_count(12usize, 2usize, 32u32, 2132472u32)?;
    emu.sw_no_count(0usize, 2usize, 36u32, 2132476u32)?;
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2132480u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1960u32, 2132484u32);
    emu.adi_no_count(10usize, 2usize, 24u32, 2132488u32);
    emu.apc_no_count(1usize, 2132488u32, 16384u32, 2132492u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2132496u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965836u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00208a10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2132500u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2132504u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2132508u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2132512u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2132516u32)?;
    emu.adi_no_count(9usize, 10usize, 0u32, 2132520u32);
    emu.adi_no_count(18usize, 0usize, 4u32, 2132524u32);
    emu.sb_no_count(18usize, 2usize, 8u32, 2132528u32);
    emu.sw_no_count(11usize, 2usize, 16u32, 2132532u32)?;
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2132536u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1792u32, 2132540u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2132544u32);
    emu.apc_no_count(1usize, 2132544u32, 28672u32, 2132548u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2132552u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1416u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208a48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2132604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208a7c));
    } else {
        emu.pc = 2132556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208a4c));
    }
}
#[inline(always)]
pub fn block_0x00208a4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 8u32, 2132560u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2132720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208af0));
    } else {
        emu.pc = 2132564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208a54));
    }
}
#[inline(always)]
pub fn block_0x00208a54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2132568u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2132572u32)?;
    emu.sw_no_count(10usize, 9usize, 0u32, 2132576u32)?;
    emu.sw_no_count(11usize, 9usize, 4u32, 2132580u32)?;
    emu.add_memory_rw_events(4usize);
    emu.pc = 2132580u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208a64));
}
#[inline(always)]
pub fn block_0x00208a64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2132584u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2132588u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2132592u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2132596u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2132600u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2132604u32;
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
pub fn block_0x00208a7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 8u32, 2132608u32);
    emu.lw_no_count(8usize, 2usize, 12u32, 2132612u32)?;
    emu.sb_no_count(18usize, 9usize, 0u32, 2132616u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a < b {
        emu.pc = 2132628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208a94));
    } else {
        emu.pc = 2132620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208a8c));
    }
}
#[inline(always)]
pub fn block_0x00208a8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 3u32, 2132624u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2132580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208a64));
    } else {
        emu.pc = 2132628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208a94));
    }
}
#[inline(always)]
pub fn block_0x00208a94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 8usize, 4u32, 2132632u32)?;
    emu.lw_no_count(11usize, 18usize, 0u32, 2132636u32)?;
    emu.lw_no_count(9usize, 8usize, 0u32, 2132640u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2132652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208aac));
    } else {
        emu.pc = 2132644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208aa4));
    }
}
#[inline(always)]
pub fn block_0x00208aa4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2132648u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2132652u32;
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
pub fn block_0x00208aac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 18usize, 4u32, 2132656u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2132676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208ac4));
    } else {
        emu.pc = 2132660u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208ab4));
    }
}
#[inline(always)]
pub fn block_0x00208ab4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 8u32, 2132664u32)?;
    emu.adi_no_count(10usize, 9usize, 0u32, 2132668u32);
    emu.apc_no_count(1usize, 2132668u32, 4294934528u32, 2132672u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2132676u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1660u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208ac4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 12u32, 2132680u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2132684u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2132688u32);
    emu.apc_no_count(1usize, 2132688u32, 4294934528u32, 2132692u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2132696u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1640u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208ad8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2132700u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2132704u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2132708u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2132712u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2132716u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2132720u32;
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
pub fn block_0x00208af0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2132724u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1952u32, 2132728u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2132732u32);
    emu.sw_no_count(0usize, 2usize, 40u32, 2132736u32)?;
    emu.adi_no_count(12usize, 0usize, 4u32, 2132740u32);
    emu.sw_no_count(10usize, 2usize, 24u32, 2132744u32)?;
    emu.sw_no_count(11usize, 2usize, 28u32, 2132748u32)?;
    emu.sw_no_count(12usize, 2usize, 32u32, 2132752u32)?;
    emu.sw_no_count(0usize, 2usize, 36u32, 2132756u32)?;
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2132760u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1960u32, 2132764u32);
    emu.adi_no_count(10usize, 2usize, 24u32, 2132768u32);
    emu.apc_no_count(1usize, 2132768u32, 16384u32, 2132772u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2132776u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965556u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208b28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2132808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208b48));
    } else {
        emu.pc = 2132780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208b2c));
    }
}
#[inline(always)]
pub fn block_0x00208b2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2132784u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2132788u32)?;
    emu.adi_no_count(10usize, 0usize, 2u32, 2132792u32);
    emu.apc_no_count(1usize, 2132792u32, 4294942720u32, 2132796u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2132800u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1068u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208b40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2132804u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2132808u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2132808u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208b48));
}
#[inline(always)]
pub fn block_0x00208b48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2132812u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2132816u32;
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
pub fn block_0x00208b50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2132820u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2132824u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2132828u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2132832u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2132836u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2132840u32)?;
    emu.adi_no_count(8usize, 12usize, 0u32, 2132844u32);
    emu.lw_no_count(9usize, 10usize, 8u32, 2132848u32)?;
    emu.lw_no_count(10usize, 9usize, 0u32, 2132852u32)?;
    emu.lw_no_count(18usize, 9usize, 8u32, 2132856u32)?;
    emu.sbr_no_count(10usize, 10usize, 18usize, 2132860u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2132924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208bbc));
    } else {
        emu.pc = 2132864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208b80));
    }
}
#[inline(always)]
pub fn block_0x00208b80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 4u32, 2132868u32)?;
    emu.adr_no_count(10usize, 10usize, 18usize, 2132872u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2132876u32);
    emu.apc_no_count(1usize, 2132876u32, 4294942720u32, 2132880u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2132884u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1256u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00208b94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 18usize, 8usize, 2132888u32);
    emu.sw_no_count(8usize, 9usize, 8u32, 2132892u32)?;
    emu.adi_no_count(10usize, 0usize, 0u32, 2132896u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2132900u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2132904u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2132908u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2132912u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2132916u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2132920u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2132924u32;
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
pub fn block_0x00208bbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2132928u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2132932u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2132936u32);
    emu.adi_no_count(19usize, 11usize, 0u32, 2132940u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2132944u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2132948u32);
    emu.apc_no_count(1usize, 2132948u32, 0u32, 2132952u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2132956u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(544u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208bdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 19usize, 0u32, 2132960u32);
    emu.lw_no_count(18usize, 9usize, 8u32, 2132964u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2132968u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2132864u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208b80));
}
