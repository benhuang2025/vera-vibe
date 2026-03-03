pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2176652u32;
pub const PC_MAX: u32 = 2178168u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 106usize] = [
        block_0x0021368c,
        block_0x00213694,
        block_0x00213698,
        block_0x0021369c,
        block_0x002136b0,
        block_0x002136b4,
        block_0x002136cc,
        block_0x002136d0,
        block_0x002136e0,
        block_0x00213714,
        block_0x00213718,
        block_0x0021372c,
        block_0x00213730,
        block_0x00213734,
        block_0x00213740,
        block_0x00213744,
        block_0x00213748,
        block_0x0021375c,
        block_0x00213760,
        block_0x00213778,
        block_0x0021377c,
        block_0x00213780,
        block_0x00213790,
        block_0x002137c4,
        block_0x002137c8,
        block_0x002137d8,
        block_0x002137dc,
        block_0x002137e0,
        block_0x002137ec,
        block_0x002137f0,
        block_0x002137f4,
        block_0x00213804,
        block_0x00213808,
        block_0x00213820,
        block_0x00213824,
        block_0x00213828,
        block_0x00213838,
        block_0x0021386c,
        block_0x00213870,
        block_0x00213880,
        block_0x00213884,
        block_0x00213888,
        block_0x00213894,
        block_0x00213898,
        block_0x0021389c,
        block_0x002138ac,
        block_0x002138b0,
        block_0x002138c8,
        block_0x002138cc,
        block_0x002138d0,
        block_0x002138e0,
        block_0x00213914,
        block_0x00213918,
        block_0x00213924,
        block_0x00213928,
        block_0x0021392c,
        block_0x00213930,
        block_0x00213944,
        block_0x0021394c,
        block_0x00213960,
        block_0x00213988,
        block_0x0021398c,
        block_0x00213994,
        block_0x0021399c,
        block_0x002139ac,
        block_0x002139b4,
        block_0x002139b8,
        block_0x002139bc,
        block_0x002139d4,
        block_0x002139e0,
        block_0x002139e4,
        block_0x002139f4,
        block_0x00213a08,
        block_0x00213a14,
        block_0x00213a18,
        block_0x00213a20,
        block_0x00213a30,
        block_0x00213a34,
        block_0x00213a40,
        block_0x00213a44,
        block_0x00213a50,
        block_0x00213a54,
        block_0x00213a5c,
        block_0x00213a60,
        block_0x00213a64,
        block_0x00213aa8,
        block_0x00213abc,
        block_0x00213ad4,
        block_0x00213aec,
        block_0x00213b04,
        block_0x00213b20,
        block_0x00213b3c,
        block_0x00213b58,
        block_0x00213b74,
        block_0x00213b90,
        block_0x00213bac,
        block_0x00213bc4,
        block_0x00213be0,
        block_0x00213bf8,
        block_0x00213c10,
        block_0x00213c24,
        block_0x00213c3c,
        block_0x00213c54,
        block_0x00213c68,
        block_0x00213c70,
        block_0x00213c78,
    ];
    const IDX: [u16; 380usize] = [
        1u16, 0u16, 2u16, 3u16, 4u16, 0u16, 0u16, 0u16, 0u16, 5u16, 6u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 7u16, 8u16, 0u16, 0u16, 0u16, 9u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 10u16, 11u16, 0u16, 0u16, 0u16,
        0u16, 12u16, 13u16, 14u16, 0u16, 0u16, 15u16, 16u16, 17u16, 0u16, 0u16, 0u16,
        0u16, 18u16, 19u16, 0u16, 0u16, 0u16, 0u16, 0u16, 20u16, 21u16, 22u16, 0u16,
        0u16, 0u16, 23u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 24u16, 25u16, 0u16, 0u16, 0u16, 26u16, 27u16, 28u16, 0u16, 0u16,
        29u16, 30u16, 31u16, 0u16, 0u16, 0u16, 32u16, 33u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 34u16, 35u16, 36u16, 0u16, 0u16, 0u16, 37u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 38u16, 39u16, 0u16, 0u16, 0u16, 40u16,
        41u16, 42u16, 0u16, 0u16, 43u16, 44u16, 45u16, 0u16, 0u16, 0u16, 46u16, 47u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 48u16, 49u16, 50u16, 0u16, 0u16, 0u16, 51u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 52u16, 53u16,
        0u16, 0u16, 54u16, 55u16, 56u16, 57u16, 0u16, 0u16, 0u16, 0u16, 58u16, 0u16,
        59u16, 0u16, 0u16, 0u16, 0u16, 60u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 61u16, 62u16, 0u16, 63u16, 0u16, 64u16, 0u16, 0u16, 0u16, 65u16,
        0u16, 66u16, 67u16, 68u16, 0u16, 0u16, 0u16, 0u16, 0u16, 69u16, 0u16, 0u16,
        70u16, 71u16, 0u16, 0u16, 0u16, 72u16, 0u16, 0u16, 0u16, 0u16, 73u16, 0u16, 0u16,
        74u16, 75u16, 0u16, 76u16, 0u16, 0u16, 0u16, 77u16, 78u16, 0u16, 0u16, 79u16,
        80u16, 0u16, 0u16, 81u16, 82u16, 0u16, 83u16, 84u16, 85u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        86u16, 0u16, 0u16, 0u16, 0u16, 87u16, 0u16, 0u16, 0u16, 0u16, 0u16, 88u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 89u16, 0u16, 0u16, 0u16, 0u16, 0u16, 90u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 91u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 92u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 93u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 94u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 95u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        96u16, 0u16, 0u16, 0u16, 0u16, 0u16, 97u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        98u16, 0u16, 0u16, 0u16, 0u16, 0u16, 99u16, 0u16, 0u16, 0u16, 0u16, 0u16, 100u16,
        0u16, 0u16, 0u16, 0u16, 101u16, 0u16, 0u16, 0u16, 0u16, 0u16, 102u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 103u16, 0u16, 0u16, 0u16, 0u16, 104u16, 0u16, 105u16, 0u16,
        106u16,
    ];
    if pc < 2176652u32 || pc > 2178168u32 {
        return None;
    }
    let word_offset = ((pc - 2176652u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x0021368c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 16usize, 0u32, 2176656u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2176664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213698));
    } else {
        emu.pc = 2176660u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213694));
    }
}
#[inline(always)]
pub fn block_0x00213694(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 15usize, 0u32, 2176664u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2176664u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213698));
}
#[inline(always)]
pub fn block_0x00213698(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2177724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213abc));
    } else {
        emu.pc = 2176668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021369c));
    }
}
#[inline(always)]
pub fn block_0x0021369c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(27usize, 13usize, 2u32, 2176672u32);
    emu.sbr_no_count(14usize, 0usize, 27usize, 2176676u32);
    emu.adi_no_count(28usize, 1usize, 0u32, 2176680u32);
    emu.adr_no_count(8usize, 1usize, 27usize, 2176684u32);
    emu.adr_no_count(1usize, 25usize, 27usize, 2176688u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2176688u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002136b0));
}
#[inline(always)]
pub fn block_0x002136b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2176816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213730));
    } else {
        emu.pc = 2176692u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002136b4));
    }
}
#[inline(always)]
pub fn block_0x002136b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(7usize, 1usize, 0u32, 2176696u32)?;
    emu.lw_no_count(23usize, 8usize, 0u32, 2176700u32)?;
    emu.adi_no_count(14usize, 14usize, 4u32, 2176704u32);
    emu.adi_no_count(8usize, 8usize, 4294967292u32, 2176708u32);
    emu.adi_no_count(1usize, 1usize, 4294967292u32, 2176712u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2176688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002136b0));
    } else {
        emu.pc = 2176716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002136cc));
    }
}
#[inline(always)]
pub fn block_0x002136cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a < b {
        emu.pc = 2176820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213734));
    } else {
        emu.pc = 2176720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002136d0));
    }
}
#[inline(always)]
pub fn block_0x002136d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 2usize, 664u32, 2176724u32);
    emu.adi_no_count(8usize, 0usize, 1u32, 2176728u32);
    emu.adr_no_count(15usize, 14usize, 27usize, 2176732u32);
    emu.adi_no_count(27usize, 2usize, 8u32, 2176736u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2176736u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002136e0));
}
#[inline]
pub fn block_0x002136e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(6usize, 14usize, 0u32, 2176740u32)?;
    emu.lw_no_count(7usize, 27usize, 0u32, 2176744u32)?;
    emu.ani_no_count(8usize, 8usize, 1u32, 2176748u32);
    emu.adi_no_count(14usize, 14usize, 4u32, 2176752u32);
    emu.xri_no_count(6usize, 6usize, 4294967295u32, 2176756u32);
    emu.adr_no_count(6usize, 7usize, 6usize, 2176760u32);
    emu.sltru_no_count(7usize, 6usize, 7usize, 2176764u32);
    emu.adr_no_count(8usize, 6usize, 8usize, 2176768u32);
    emu.sltru_no_count(6usize, 8usize, 6usize, 2176772u32);
    emu.sw_no_count(8usize, 27usize, 0u32, 2176776u32)?;
    emu.orr_no_count(8usize, 7usize, 6usize, 2176780u32);
    emu.adi_no_count(27usize, 27usize, 4u32, 2176784u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a != b {
        emu.pc = 2176736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002136e0));
    } else {
        emu.pc = 2176788u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213714));
    }
}
#[inline(always)]
pub fn block_0x00213714(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2177988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213bc4));
    } else {
        emu.pc = 2176792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213718));
    }
}
#[inline(always)]
pub fn block_0x00213718(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(13usize, 2usize, 168u32, 2176796u32)?;
    emu.adi_no_count(27usize, 0usize, 8u32, 2176800u32);
    emu.adi_no_count(15usize, 13usize, 0u32, 2176804u32);
    emu.adi_no_count(13usize, 17usize, 0u32, 2176808u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2176836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213744));
    } else {
        emu.pc = 2176812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021372c));
    }
}
#[inline(always)]
pub fn block_0x0021372c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2176816u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2176832u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213740));
}
#[inline(always)]
pub fn block_0x00213730(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2176720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002136d0));
    } else {
        emu.pc = 2176820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213734));
    }
}
#[inline(always)]
pub fn block_0x00213734(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(27usize, 0usize, 0u32, 2176824u32);
    emu.adi_no_count(13usize, 17usize, 0u32, 2176828u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2176836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213744));
    } else {
        emu.pc = 2176832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213740));
    }
}
#[inline(always)]
pub fn block_0x00213740(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 15usize, 0u32, 2176836u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2176836u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213744));
}
#[inline(always)]
pub fn block_0x00213744(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2177724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213abc));
    } else {
        emu.pc = 2176840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213748));
    }
}
#[inline(always)]
pub fn block_0x00213748(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(1usize, 13usize, 2u32, 2176844u32);
    emu.sbr_no_count(14usize, 0usize, 1usize, 2176848u32);
    emu.adi_no_count(8usize, 2usize, 496u32, 2176852u32);
    emu.adr_no_count(8usize, 8usize, 1usize, 2176856u32);
    emu.adr_no_count(7usize, 25usize, 1usize, 2176860u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2176860u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021375c));
}
#[inline(always)]
pub fn block_0x0021375c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2176988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002137dc));
    } else {
        emu.pc = 2176864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213760));
    }
}
#[inline(always)]
pub fn block_0x00213760(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(23usize, 7usize, 0u32, 2176868u32)?;
    emu.lw_no_count(6usize, 8usize, 0u32, 2176872u32)?;
    emu.adi_no_count(14usize, 14usize, 4u32, 2176876u32);
    emu.adi_no_count(8usize, 8usize, 4294967292u32, 2176880u32);
    emu.adi_no_count(7usize, 7usize, 4294967292u32, 2176884u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a == b {
        emu.pc = 2176860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021375c));
    } else {
        emu.pc = 2176888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213778));
    }
}
#[inline(always)]
pub fn block_0x00213778(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a < b {
        emu.pc = 2176992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002137e0));
    } else {
        emu.pc = 2176892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021377c));
    }
}
#[inline(always)]
pub fn block_0x0021377c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2176968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002137c8));
    } else {
        emu.pc = 2176896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213780));
    }
}
#[inline(always)]
pub fn block_0x00213780(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 2usize, 500u32, 2176900u32);
    emu.adi_no_count(8usize, 0usize, 1u32, 2176904u32);
    emu.adr_no_count(1usize, 14usize, 1usize, 2176908u32);
    emu.adi_no_count(15usize, 2usize, 8u32, 2176912u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2176912u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213790));
}
#[inline]
pub fn block_0x00213790(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(6usize, 14usize, 0u32, 2176916u32)?;
    emu.lw_no_count(7usize, 15usize, 0u32, 2176920u32)?;
    emu.ani_no_count(8usize, 8usize, 1u32, 2176924u32);
    emu.adi_no_count(14usize, 14usize, 4u32, 2176928u32);
    emu.xri_no_count(6usize, 6usize, 4294967295u32, 2176932u32);
    emu.adr_no_count(6usize, 7usize, 6usize, 2176936u32);
    emu.sltru_no_count(7usize, 6usize, 7usize, 2176940u32);
    emu.adr_no_count(8usize, 6usize, 8usize, 2176944u32);
    emu.sltru_no_count(6usize, 8usize, 6usize, 2176948u32);
    emu.sw_no_count(8usize, 15usize, 0u32, 2176952u32)?;
    emu.orr_no_count(8usize, 7usize, 6usize, 2176956u32);
    emu.adi_no_count(15usize, 15usize, 4u32, 2176960u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(1usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a != b {
        emu.pc = 2176912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213790));
    } else {
        emu.pc = 2176964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002137c4));
    }
}
#[inline(always)]
pub fn block_0x002137c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2177988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213bc4));
    } else {
        emu.pc = 2176968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002137c8));
    }
}
#[inline(always)]
pub fn block_0x002137c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(13usize, 2usize, 168u32, 2176972u32)?;
    emu.ori_no_count(27usize, 27usize, 4u32, 2176976u32);
    emu.adi_no_count(14usize, 5usize, 0u32, 2176980u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2177008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002137f0));
    } else {
        emu.pc = 2176984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002137d8));
    }
}
#[inline(always)]
pub fn block_0x002137d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2176988u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2177004u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002137ec));
}
#[inline(always)]
pub fn block_0x002137dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2176892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021377c));
    } else {
        emu.pc = 2176992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002137e0));
    }
}
#[inline(always)]
pub fn block_0x002137e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 15usize, 0u32, 2176996u32);
    emu.adi_no_count(14usize, 5usize, 0u32, 2177000u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2177008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002137f0));
    } else {
        emu.pc = 2177004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002137ec));
    }
}
#[inline(always)]
pub fn block_0x002137ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 13usize, 0u32, 2177008u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2177008u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002137f0));
}
#[inline(always)]
pub fn block_0x002137f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a >= b {
        emu.pc = 2177748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213ad4));
    } else {
        emu.pc = 2177012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002137f4));
    }
}
#[inline(always)]
pub fn block_0x002137f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(15usize, 14usize, 2u32, 2177016u32);
    emu.sbr_no_count(8usize, 0usize, 15usize, 2177020u32);
    emu.adr_no_count(1usize, 29usize, 15usize, 2177024u32);
    emu.adr_no_count(7usize, 25usize, 15usize, 2177028u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2177028u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213804));
}
#[inline(always)]
pub fn block_0x00213804(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2177156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213884));
    } else {
        emu.pc = 2177032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213808));
    }
}
#[inline(always)]
pub fn block_0x00213808(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(23usize, 7usize, 0u32, 2177036u32)?;
    emu.lw_no_count(6usize, 1usize, 0u32, 2177040u32)?;
    emu.adi_no_count(8usize, 8usize, 4u32, 2177044u32);
    emu.adi_no_count(1usize, 1usize, 4294967292u32, 2177048u32);
    emu.adi_no_count(7usize, 7usize, 4294967292u32, 2177052u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a == b {
        emu.pc = 2177028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213804));
    } else {
        emu.pc = 2177056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213820));
    }
}
#[inline(always)]
pub fn block_0x00213820(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a < b {
        emu.pc = 2177160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213888));
    } else {
        emu.pc = 2177060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213824));
    }
}
#[inline(always)]
pub fn block_0x00213824(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2177136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213870));
    } else {
        emu.pc = 2177064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213828));
    }
}
#[inline(always)]
pub fn block_0x00213828(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 2usize, 336u32, 2177068u32);
    emu.adi_no_count(8usize, 0usize, 1u32, 2177072u32);
    emu.adr_no_count(15usize, 13usize, 15usize, 2177076u32);
    emu.adi_no_count(1usize, 2usize, 8u32, 2177080u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2177080u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213838));
}
#[inline]
pub fn block_0x00213838(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(6usize, 13usize, 0u32, 2177084u32)?;
    emu.lw_no_count(7usize, 1usize, 0u32, 2177088u32)?;
    emu.ani_no_count(8usize, 8usize, 1u32, 2177092u32);
    emu.adi_no_count(13usize, 13usize, 4u32, 2177096u32);
    emu.xri_no_count(6usize, 6usize, 4294967295u32, 2177100u32);
    emu.adr_no_count(6usize, 7usize, 6usize, 2177104u32);
    emu.sltru_no_count(7usize, 6usize, 7usize, 2177108u32);
    emu.adr_no_count(8usize, 6usize, 8usize, 2177112u32);
    emu.sltru_no_count(6usize, 8usize, 6usize, 2177116u32);
    emu.sw_no_count(8usize, 1usize, 0u32, 2177120u32)?;
    emu.orr_no_count(8usize, 7usize, 6usize, 2177124u32);
    emu.adi_no_count(1usize, 1usize, 4u32, 2177128u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2177080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213838));
    } else {
        emu.pc = 2177132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021386c));
    }
}
#[inline(always)]
pub fn block_0x0021386c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2177988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213bc4));
    } else {
        emu.pc = 2177136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213870));
    }
}
#[inline(always)]
pub fn block_0x00213870(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(14usize, 2usize, 168u32, 2177140u32)?;
    emu.adi_no_count(27usize, 27usize, 2u32, 2177144u32);
    emu.adi_no_count(15usize, 10usize, 0u32, 2177148u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2177176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213898));
    } else {
        emu.pc = 2177152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213880));
    }
}
#[inline(always)]
pub fn block_0x00213880(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2177156u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2177172u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213894));
}
#[inline(always)]
pub fn block_0x00213884(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2177060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213824));
    } else {
        emu.pc = 2177160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213888));
    }
}
#[inline(always)]
pub fn block_0x00213888(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 13usize, 0u32, 2177164u32);
    emu.adi_no_count(15usize, 10usize, 0u32, 2177168u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2177176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213898));
    } else {
        emu.pc = 2177172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213894));
    }
}
#[inline(always)]
pub fn block_0x00213894(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 14usize, 0u32, 2177176u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2177176u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213898));
}
#[inline(always)]
pub fn block_0x00213898(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2177772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213aec));
    } else {
        emu.pc = 2177180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021389c));
    }
}
#[inline(always)]
pub fn block_0x0021389c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(1usize, 15usize, 2u32, 2177184u32);
    emu.sbr_no_count(13usize, 0usize, 1usize, 2177188u32);
    emu.adr_no_count(8usize, 30usize, 1usize, 2177192u32);
    emu.adr_no_count(7usize, 25usize, 1usize, 2177196u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2177196u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002138ac));
}
#[inline(always)]
pub fn block_0x002138ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2177316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213924));
    } else {
        emu.pc = 2177200u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002138b0));
    }
}
#[inline(always)]
pub fn block_0x002138b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(23usize, 7usize, 0u32, 2177204u32)?;
    emu.lw_no_count(6usize, 8usize, 0u32, 2177208u32)?;
    emu.adi_no_count(13usize, 13usize, 4u32, 2177212u32);
    emu.adi_no_count(8usize, 8usize, 4294967292u32, 2177216u32);
    emu.adi_no_count(7usize, 7usize, 4294967292u32, 2177220u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a == b {
        emu.pc = 2177196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002138ac));
    } else {
        emu.pc = 2177224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002138c8));
    }
}
#[inline(always)]
pub fn block_0x002138c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a < b {
        emu.pc = 2177320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213928));
    } else {
        emu.pc = 2177228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002138cc));
    }
}
#[inline(always)]
pub fn block_0x002138cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2177304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213918));
    } else {
        emu.pc = 2177232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002138d0));
    }
}
#[inline(always)]
pub fn block_0x002138d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 2usize, 172u32, 2177236u32);
    emu.adi_no_count(8usize, 0usize, 1u32, 2177240u32);
    emu.adr_no_count(1usize, 13usize, 1usize, 2177244u32);
    emu.adi_no_count(14usize, 2usize, 8u32, 2177248u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2177248u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002138e0));
}
#[inline]
pub fn block_0x002138e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(6usize, 13usize, 0u32, 2177252u32)?;
    emu.lw_no_count(7usize, 14usize, 0u32, 2177256u32)?;
    emu.ani_no_count(8usize, 8usize, 1u32, 2177260u32);
    emu.adi_no_count(13usize, 13usize, 4u32, 2177264u32);
    emu.xri_no_count(6usize, 6usize, 4294967295u32, 2177268u32);
    emu.adr_no_count(6usize, 7usize, 6usize, 2177272u32);
    emu.sltru_no_count(7usize, 6usize, 7usize, 2177276u32);
    emu.adr_no_count(8usize, 6usize, 8usize, 2177280u32);
    emu.sltru_no_count(6usize, 8usize, 6usize, 2177284u32);
    emu.sw_no_count(8usize, 14usize, 0u32, 2177288u32)?;
    emu.orr_no_count(8usize, 7usize, 6usize, 2177292u32);
    emu.adi_no_count(14usize, 14usize, 4u32, 2177296u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(1usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2177248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002138e0));
    } else {
        emu.pc = 2177300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213914));
    }
}
#[inline(always)]
pub fn block_0x00213914(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2177988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213bc4));
    } else {
        emu.pc = 2177304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213918));
    }
}
#[inline(always)]
pub fn block_0x00213918(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(15usize, 2usize, 168u32, 2177308u32)?;
    emu.adi_no_count(27usize, 27usize, 1u32, 2177312u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2177316u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2177324u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021392c));
}
#[inline(always)]
pub fn block_0x00213924(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2177228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002138cc));
    } else {
        emu.pc = 2177320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213928));
    }
}
#[inline(always)]
pub fn block_0x00213928(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 14usize, 0u32, 2177324u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2177324u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021392c));
}
#[inline(always)]
pub fn block_0x0021392c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a == b {
        emu.pc = 2178108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213c3c));
    } else {
        emu.pc = 2177328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213930));
    }
}
#[inline(always)]
pub fn block_0x00213930(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 27usize, 48u32, 2177332u32);
    emu.lw_no_count(14usize, 2usize, 4u32, 2177336u32)?;
    emu.adr_no_count(14usize, 14usize, 11usize, 2177340u32);
    emu.sb_no_count(13usize, 14usize, 0u32, 2177344u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2177772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213aec));
    } else {
        emu.pc = 2177348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213944));
    }
}
#[inline(always)]
pub fn block_0x00213944(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(1usize, 28usize, 0u32, 2177352u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2177436u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021399c));
    } else {
        emu.pc = 2177356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021394c));
    }
}
#[inline(always)]
pub fn block_0x0021394c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 0u32, 2177360u32);
    emu.sli_no_count(8usize, 15usize, 2u32, 2177364u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2177368u32);
    emu.adr_no_count(13usize, 13usize, 8usize, 2177372u32);
    emu.adi_no_count(7usize, 2usize, 8u32, 2177376u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2177376u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213960));
}
#[inline]
pub fn block_0x00213960(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(6usize, 7usize, 0u32, 2177380u32)?;
    emu.mulhu_no_count(23usize, 6usize, 24usize, 2177384u32);
    emu.mul_no_count(6usize, 6usize, 24usize, 2177388u32);
    emu.adr_no_count(14usize, 6usize, 14usize, 2177392u32);
    emu.sw_no_count(14usize, 7usize, 0u32, 2177396u32)?;
    emu.adi_no_count(7usize, 7usize, 4u32, 2177400u32);
    emu.sltru_no_count(14usize, 14usize, 6usize, 2177404u32);
    emu.adi_no_count(8usize, 8usize, 4294967292u32, 2177408u32);
    emu.adr_no_count(14usize, 23usize, 14usize, 2177412u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a != b {
        emu.pc = 2177376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213960));
    } else {
        emu.pc = 2177416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213988));
    }
}
#[inline(always)]
pub fn block_0x00213988(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2177436u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021399c));
    } else {
        emu.pc = 2177420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021398c));
    }
}
#[inline(always)]
pub fn block_0x0021398c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(6usize, 0usize, 40u32, 2177424u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2178084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213c24));
    } else {
        emu.pc = 2177428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213994));
    }
}
#[inline(always)]
pub fn block_0x00213994(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(14usize, 13usize, 0u32, 2177432u32)?;
    emu.adi_no_count(15usize, 15usize, 1u32, 2177436u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2177436u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021399c));
}
#[inline(always)]
pub fn block_0x0021399c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(15usize, 2usize, 168u32, 2177440u32)?;
    emu.adi_no_count(27usize, 26usize, 1u32, 2177444u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2177448u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a != b {
        emu.pc = 2176616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2176616u32));
    } else {
        emu.pc = 2177452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002139ac));
    }
}
#[inline(always)]
pub fn block_0x002139ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2177456u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2177460u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2176100u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(2176100u32));
}
#[inline(always)]
pub fn block_0x002139b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a < b {
        emu.pc = 2178016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213be0));
    } else {
        emu.pc = 2177464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002139b8));
    }
}
#[inline(always)]
pub fn block_0x002139b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2177492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002139d4));
    } else {
        emu.pc = 2177468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002139bc));
    }
}
#[inline(always)]
pub fn block_0x002139bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 4u32, 2177472u32)?;
    emu.adr_no_count(10usize, 10usize, 11usize, 2177476u32);
    emu.sbr_no_count(12usize, 20usize, 11usize, 2177480u32);
    emu.adi_no_count(11usize, 0usize, 48u32, 2177484u32);
    emu.apc_no_count(1usize, 2177484u32, 4294897664u32, 2177488u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2177492u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1456u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002139d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 4u32, 2177496u32)?;
    emu.sw_no_count(10usize, 9usize, 0u32, 2177500u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2177504u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2177636u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213a64));
}
#[inline(always)]
pub fn block_0x002139e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2177588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213a34));
    } else {
        emu.pc = 2177508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002139e4));
    }
}
#[inline(always)]
pub fn block_0x002139e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 49u32, 2177512u32);
    emu.adi_no_count(12usize, 20usize, 4294967295u32, 2177516u32);
    emu.sb_no_count(10usize, 14usize, 0u32, 2177520u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2177604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213a44));
    } else {
        emu.pc = 2177524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002139f4));
    }
}
#[inline(always)]
pub fn block_0x002139f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 14usize, 1u32, 2177528u32);
    emu.adi_no_count(11usize, 0usize, 48u32, 2177532u32);
    emu.adi_no_count(8usize, 0usize, 48u32, 2177536u32);
    emu.apc_no_count(1usize, 2177536u32, 4294897664u32, 2177540u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2177544u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1404u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00213a08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 2usize, 4u32, 2177548u32)?;
    emu.adi_no_count(22usize, 22usize, 1u32, 2177552u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(21usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2177616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213a50));
    } else {
        emu.pc = 2177556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213a14));
    }
}
#[inline(always)]
pub fn block_0x00213a14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2177560u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2177628u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213a5c));
}
#[inline(always)]
pub fn block_0x00213a18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 20usize, 4294967295u32, 2177564u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2178132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213c54));
    } else {
        emu.pc = 2177568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213a20));
    }
}
#[inline(always)]
pub fn block_0x00213a20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 14usize, 10usize, 2177572u32);
    emu.lbu_no_count(10usize, 10usize, 0u32, 2177576u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2177580u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2176332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2176332u32));
    } else {
        emu.pc = 2177584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213a30));
    }
}
#[inline(always)]
pub fn block_0x00213a30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2177588u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2177628u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213a5c));
}
#[inline(always)]
pub fn block_0x00213a34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 0usize, 49u32, 2177592u32);
    emu.adi_no_count(22usize, 22usize, 1u32, 2177596u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(21usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2177616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213a50));
    } else {
        emu.pc = 2177600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213a40));
    }
}
#[inline(always)]
pub fn block_0x00213a40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2177604u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2177628u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213a5c));
}
#[inline(always)]
pub fn block_0x00213a44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 0usize, 48u32, 2177608u32);
    emu.adi_no_count(22usize, 22usize, 1u32, 2177612u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(21usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2177628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213a5c));
    } else {
        emu.pc = 2177616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213a50));
    }
}
#[inline(always)]
pub fn block_0x00213a50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a >= b {
        emu.pc = 2177628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213a5c));
    } else {
        emu.pc = 2177620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213a54));
    }
}
#[inline(always)]
pub fn block_0x00213a54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(8usize, 23usize, 0u32, 2177624u32);
    emu.adi_no_count(20usize, 20usize, 1u32, 2177628u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2177628u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213a5c));
}
#[inline(always)]
pub fn block_0x00213a5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a < b {
        emu.pc = 2177964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213bac));
    } else {
        emu.pc = 2177632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213a60));
    }
}
#[inline(always)]
pub fn block_0x00213a60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(14usize, 9usize, 0u32, 2177636u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2177636u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213a64));
}
#[inline]
pub fn block_0x00213a64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(20usize, 9usize, 4u32, 2177640u32)?;
    emu.sh_no_count(22usize, 9usize, 8u32, 2177644u32)?;
    emu.lw_no_count(1usize, 2usize, 876u32, 2177648u32)?;
    emu.lw_no_count(8usize, 2usize, 872u32, 2177652u32)?;
    emu.lw_no_count(9usize, 2usize, 868u32, 2177656u32)?;
    emu.lw_no_count(18usize, 2usize, 864u32, 2177660u32)?;
    emu.lw_no_count(19usize, 2usize, 860u32, 2177664u32)?;
    emu.lw_no_count(20usize, 2usize, 856u32, 2177668u32)?;
    emu.lw_no_count(21usize, 2usize, 852u32, 2177672u32)?;
    emu.lw_no_count(22usize, 2usize, 848u32, 2177676u32)?;
    emu.lw_no_count(23usize, 2usize, 844u32, 2177680u32)?;
    emu.lw_no_count(24usize, 2usize, 840u32, 2177684u32)?;
    emu.lw_no_count(25usize, 2usize, 836u32, 2177688u32)?;
    emu.lw_no_count(26usize, 2usize, 832u32, 2177692u32)?;
    emu.lw_no_count(27usize, 2usize, 828u32, 2177696u32)?;
    emu.adi_no_count(2usize, 2usize, 880u32, 2177700u32);
    emu.add_memory_rw_events(17usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2177704u32;
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
pub fn block_0x00213aa8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2177708u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 920u32, 2177712u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2177716u32);
    emu.apc_no_count(1usize, 2177716u32, 0u32, 2177720u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2177724u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(444u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00213abc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2177728u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 920u32, 2177732u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2177736u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2177740u32);
    emu.apc_no_count(1usize, 2177740u32, 0u32, 2177744u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2177748u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(420u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00213ad4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2177752u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 920u32, 2177756u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2177760u32);
    emu.adi_no_count(10usize, 14usize, 0u32, 2177764u32);
    emu.apc_no_count(1usize, 2177764u32, 0u32, 2177768u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2177772u32;
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
pub fn block_0x00213aec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2177776u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 920u32, 2177780u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2177784u32);
    emu.adi_no_count(10usize, 15usize, 0u32, 2177788u32);
    emu.apc_no_count(1usize, 2177788u32, 0u32, 2177792u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2177796u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(372u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00213b04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2177800u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965480u32, 2177804u32);
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2177808u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965872u32, 2177812u32);
    emu.adi_no_count(11usize, 0usize, 28u32, 2177816u32);
    emu.apc_no_count(1usize, 2177816u32, 4294938624u32, 2177820u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2177824u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965596u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00213b20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2177828u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965524u32, 2177832u32);
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2177836u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965888u32, 2177840u32);
    emu.adi_no_count(11usize, 0usize, 29u32, 2177844u32);
    emu.apc_no_count(1usize, 2177844u32, 4294938624u32, 2177848u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2177852u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965568u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00213b3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2177856u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965572u32, 2177860u32);
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2177864u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965904u32, 2177868u32);
    emu.adi_no_count(11usize, 0usize, 28u32, 2177872u32);
    emu.apc_no_count(1usize, 2177872u32, 4294938624u32, 2177876u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2177880u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965540u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00213b58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2177884u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965800u32, 2177888u32);
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2177892u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966016u32, 2177896u32);
    emu.adi_no_count(11usize, 0usize, 54u32, 2177900u32);
    emu.apc_no_count(1usize, 2177900u32, 4294938624u32, 2177904u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2177908u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965512u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00213b74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2177912u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965728u32, 2177916u32);
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2177920u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966000u32, 2177924u32);
    emu.adi_no_count(11usize, 0usize, 55u32, 2177928u32);
    emu.apc_no_count(1usize, 2177928u32, 4294938624u32, 2177932u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2177936u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965484u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00213b90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2177940u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 991u32, 2177944u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2177948u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 920u32, 2177952u32);
    emu.adi_no_count(11usize, 0usize, 27u32, 2177956u32);
    emu.apc_no_count(1usize, 2177956u32, 4294938624u32, 2177960u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2177964u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x00213bac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2177968u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965952u32, 2177972u32);
    emu.adi_no_count(10usize, 20usize, 0u32, 2177976u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2177980u32);
    emu.apc_no_count(1usize, 2177980u32, 0u32, 2177984u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2177988u32;
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
pub fn block_0x00213bc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2177992u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 936u32, 2177996u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2178000u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 920u32, 2178004u32);
    emu.adi_no_count(11usize, 0usize, 26u32, 2178008u32);
    emu.apc_no_count(1usize, 2178008u32, 4294938624u32, 2178012u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2178016u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965404u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00213be0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2178020u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965984u32, 2178024u32);
    emu.adi_no_count(10usize, 20usize, 0u32, 2178028u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2178032u32);
    emu.apc_no_count(1usize, 2178032u32, 0u32, 2178036u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2178040u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(128u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00213bf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2178044u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965936u32, 2178048u32);
    emu.adi_no_count(10usize, 20usize, 0u32, 2178052u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2178056u32);
    emu.apc_no_count(1usize, 2178056u32, 0u32, 2178060u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2178064u32;
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
pub fn block_0x00213c10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2178068u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1068u32, 2178072u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2178076u32);
    emu.apc_no_count(1usize, 2178076u32, 0u32, 2178080u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2178084u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(76u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00213c24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2178088u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 920u32, 2178092u32);
    emu.adi_no_count(10usize, 0usize, 40u32, 2178096u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2178100u32);
    emu.apc_no_count(1usize, 2178100u32, 4294938624u32, 2178104u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2178108u32;
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
pub fn block_0x00213c3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2178112u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965968u32, 2178116u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2178120u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2178124u32);
    emu.apc_no_count(1usize, 2178124u32, 4294938624u32, 2178128u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2178132u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965348u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00213c54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2178136u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965920u32, 2178140u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2178144u32);
    emu.apc_no_count(1usize, 2178144u32, 4294938624u32, 2178148u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2178152u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965328u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00213c68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2178152u32, 0u32, 2178156u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2178160u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(556u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00213c70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2178160u32, 0u32, 2178164u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2178168u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(644u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00213c78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2178168u32, 0u32, 2178172u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2178176u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(732u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
