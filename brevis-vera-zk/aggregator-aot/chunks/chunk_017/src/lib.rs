pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2191040u32;
pub const PC_MAX: u32 = 2193492u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 107usize] = [
        block_0x00216ec0,
        block_0x00216f1c,
        block_0x00216f28,
        block_0x00216f7c,
        block_0x00216f84,
        block_0x00216f88,
        block_0x00216f90,
        block_0x00216fa4,
        block_0x00216fac,
        block_0x00216fb0,
        block_0x00216fd4,
        block_0x00216fdc,
        block_0x00217004,
        block_0x00217014,
        block_0x0021701c,
        block_0x00217024,
        block_0x00217068,
        block_0x0021706c,
        block_0x00217074,
        block_0x00217078,
        block_0x0021708c,
        block_0x00217090,
        block_0x002170cc,
        block_0x00217104,
        block_0x00217130,
        block_0x00217164,
        block_0x00217170,
        block_0x0021718c,
        block_0x002171a8,
        block_0x002171dc,
        block_0x002171e8,
        block_0x00217204,
        block_0x00217220,
        block_0x0021723c,
        block_0x00217258,
        block_0x00217270,
        block_0x0021727c,
        block_0x002172b0,
        block_0x002172bc,
        block_0x002172c4,
        block_0x002172e4,
        block_0x002172ec,
        block_0x0021731c,
        block_0x00217358,
        block_0x00217380,
        block_0x00217384,
        block_0x002173a8,
        block_0x002173ac,
        block_0x002173bc,
        block_0x002173d4,
        block_0x002173e8,
        block_0x002173f0,
        block_0x00217400,
        block_0x00217408,
        block_0x00217414,
        block_0x0021743c,
        block_0x0021744c,
        block_0x00217454,
        block_0x0021745c,
        block_0x0021746c,
        block_0x00217480,
        block_0x00217484,
        block_0x00217498,
        block_0x002174a4,
        block_0x002174ac,
        block_0x002174cc,
        block_0x002174d4,
        block_0x00217504,
        block_0x00217540,
        block_0x00217550,
        block_0x00217560,
        block_0x00217580,
        block_0x0021758c,
        block_0x00217594,
        block_0x002175b4,
        block_0x002175bc,
        block_0x002175ec,
        block_0x00217628,
        block_0x0021763c,
        block_0x00217654,
        block_0x00217678,
        block_0x00217694,
        block_0x0021769c,
        block_0x002176b0,
        block_0x002176c4,
        block_0x002176d8,
        block_0x002176fc,
        block_0x00217704,
        block_0x0021771c,
        block_0x0021772c,
        block_0x00217734,
        block_0x0021773c,
        block_0x0021774c,
        block_0x00217774,
        block_0x002177ac,
        block_0x002177b0,
        block_0x002177b8,
        block_0x002177c8,
        block_0x002177e0,
        block_0x002177f0,
        block_0x002177f8,
        block_0x00217808,
        block_0x00217810,
        block_0x00217818,
        block_0x00217828,
        block_0x0021783c,
        block_0x00217854,
    ];
    const IDX: [u16; 614usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16,
        3u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 4u16, 0u16, 5u16, 6u16, 0u16,
        7u16, 0u16, 0u16, 0u16, 0u16, 8u16, 0u16, 9u16, 10u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 11u16, 0u16, 12u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 13u16, 0u16, 0u16, 0u16, 14u16, 0u16, 15u16, 0u16, 16u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 17u16, 18u16, 0u16, 19u16, 20u16, 0u16, 0u16, 0u16, 0u16, 21u16,
        22u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 23u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 24u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 25u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 26u16, 0u16, 0u16, 27u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 28u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 29u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 30u16, 0u16, 0u16, 31u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 32u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 33u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 34u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 35u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 36u16, 0u16, 0u16, 37u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 38u16, 0u16, 0u16, 39u16, 0u16, 40u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 41u16, 0u16, 42u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 43u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 44u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 45u16, 46u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 47u16, 48u16, 0u16, 0u16, 0u16, 49u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 50u16, 0u16, 0u16, 0u16, 0u16, 51u16, 0u16, 52u16, 0u16, 0u16, 0u16, 53u16,
        0u16, 54u16, 0u16, 0u16, 55u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 56u16, 0u16, 0u16, 0u16, 57u16, 0u16, 58u16, 0u16, 59u16, 0u16, 0u16, 0u16,
        60u16, 0u16, 0u16, 0u16, 0u16, 61u16, 62u16, 0u16, 0u16, 0u16, 0u16, 63u16, 0u16,
        0u16, 64u16, 0u16, 65u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 66u16, 0u16,
        67u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 68u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 69u16, 0u16, 0u16, 0u16, 70u16, 0u16, 0u16, 0u16, 71u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 72u16, 0u16, 0u16, 73u16, 0u16, 74u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 75u16, 0u16, 76u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 77u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 78u16, 0u16, 0u16, 0u16, 0u16, 79u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 80u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 81u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 82u16, 0u16, 83u16, 0u16, 0u16,
        0u16, 0u16, 84u16, 0u16, 0u16, 0u16, 0u16, 85u16, 0u16, 0u16, 0u16, 0u16, 86u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 87u16, 0u16, 88u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 89u16, 0u16, 0u16, 0u16, 90u16, 0u16, 91u16, 0u16, 92u16, 0u16,
        0u16, 0u16, 93u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 94u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        95u16, 96u16, 0u16, 97u16, 0u16, 0u16, 0u16, 98u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        99u16, 0u16, 0u16, 0u16, 100u16, 0u16, 101u16, 0u16, 0u16, 0u16, 102u16, 0u16,
        103u16, 0u16, 104u16, 0u16, 0u16, 0u16, 105u16, 0u16, 0u16, 0u16, 0u16, 106u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 107u16,
    ];
    if pc < 2191040u32 || pc > 2193492u32 {
        return None;
    }
    let word_offset = ((pc - 2191040u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x00216ec0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 23u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2191044u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2191048u32)?;
    emu.sw_no_count(10usize, 2usize, 8u32, 2191052u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2191056u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2191060u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966404u32, 2191064u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2191068u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294965616u32, 2191072u32);
    emu.adi_no_count(14usize, 0usize, 2u32, 2191076u32);
    emu.adi_no_count(15usize, 2usize, 36u32, 2191080u32);
    emu.adi_no_count(16usize, 0usize, 1u32, 2191084u32);
    emu.sw_no_count(10usize, 2usize, 36u32, 2191088u32)?;
    emu.sw_no_count(12usize, 2usize, 40u32, 2191092u32)?;
    emu.lw_no_count(10usize, 11usize, 0u32, 2191096u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2191100u32)?;
    emu.sw_no_count(13usize, 2usize, 12u32, 2191104u32)?;
    emu.sw_no_count(14usize, 2usize, 16u32, 2191108u32)?;
    emu.sw_no_count(15usize, 2usize, 20u32, 2191112u32)?;
    emu.sw_no_count(16usize, 2usize, 24u32, 2191116u32)?;
    emu.sw_no_count(0usize, 2usize, 28u32, 2191120u32)?;
    emu.adi_no_count(12usize, 2usize, 12u32, 2191124u32);
    emu.apc_no_count(1usize, 2191124u32, 24576u32, 2191128u32);
    emu.add_memory_rw_events(23usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2191132u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966480u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00216f1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2191136u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2191140u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2191144u32;
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
pub fn block_0x00216f28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967184u32, 2191148u32);
    emu.sw_no_count(1usize, 2usize, 108u32, 2191152u32)?;
    emu.sw_no_count(8usize, 2usize, 104u32, 2191156u32)?;
    emu.sw_no_count(9usize, 2usize, 100u32, 2191160u32)?;
    emu.sw_no_count(18usize, 2usize, 96u32, 2191164u32)?;
    emu.sw_no_count(19usize, 2usize, 92u32, 2191168u32)?;
    emu.sw_no_count(20usize, 2usize, 88u32, 2191172u32)?;
    emu.sw_no_count(21usize, 2usize, 84u32, 2191176u32)?;
    emu.sw_no_count(22usize, 2usize, 80u32, 2191180u32)?;
    emu.sw_no_count(23usize, 2usize, 76u32, 2191184u32)?;
    emu.sw_no_count(24usize, 2usize, 72u32, 2191188u32)?;
    emu.sw_no_count(25usize, 2usize, 68u32, 2191192u32)?;
    emu.sw_no_count(26usize, 2usize, 64u32, 2191196u32)?;
    emu.adi_no_count(9usize, 11usize, 0u32, 2191200u32);
    emu.adi_no_count(18usize, 10usize, 0u32, 2191204u32);
    emu.sw_no_count(0usize, 2usize, 12u32, 2191208u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2191212u32)?;
    emu.adi_no_count(10usize, 2usize, 32u32, 2191216u32);
    emu.adi_no_count(11usize, 2usize, 12u32, 2191220u32);
    emu.apc_no_count(1usize, 2191220u32, 0u32, 2191224u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2191228u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966556u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00216f7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 32u32, 2191232u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2191564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002170cc));
    } else {
        emu.pc = 2191236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216f84));
    }
}
#[inline(always)]
pub fn block_0x00216f84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 0u32, 2191240u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2191240u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00216f88));
}
#[inline(always)]
pub fn block_0x00216f88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 36u32, 2191244u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2191280u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216fb0));
    } else {
        emu.pc = 2191248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216f90));
    }
}
#[inline(always)]
pub fn block_0x00216f90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 20usize, 1u32, 2191252u32);
    emu.adi_no_count(10usize, 2usize, 32u32, 2191256u32);
    emu.adi_no_count(11usize, 2usize, 12u32, 2191260u32);
    emu.apc_no_count(1usize, 2191260u32, 0u32, 2191264u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2191268u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x00216fa4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 32u32, 2191272u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2191240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216f88));
    } else {
        emu.pc = 2191276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216fac));
    }
}
#[inline(always)]
pub fn block_0x00216fac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2191280u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2191564u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002170cc));
}
#[inline]
pub fn block_0x00216fb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(8usize, 9usize, 0u32, 2191284u32)?;
    emu.lw_no_count(9usize, 9usize, 4u32, 2191288u32)?;
    emu.sw_no_count(0usize, 2usize, 12u32, 2191292u32)?;
    emu.sw_no_count(18usize, 2usize, 20u32, 2191296u32)?;
    emu.sw_no_count(0usize, 2usize, 24u32, 2191300u32)?;
    emu.adi_no_count(10usize, 2usize, 32u32, 2191304u32);
    emu.adi_no_count(11usize, 2usize, 12u32, 2191308u32);
    emu.apc_no_count(1usize, 2191308u32, 0u32, 2191312u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2191316u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966468u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00216fd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 32u32, 2191320u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2191564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002170cc));
    } else {
        emu.pc = 2191324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216fdc));
    }
}
#[inline]
pub fn block_0x00216fdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(21usize, 2usize, 28u32, 2191328u32);
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(22usize, a);
    emu.pc = 2191332u32;
    emu.update_insn_clock();
    emu.adi_no_count(22usize, 22usize, 4294966512u32, 2191336u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(23usize, a);
    emu.pc = 2191340u32;
    emu.update_insn_clock();
    emu.adi_no_count(23usize, 23usize, 4294965860u32, 2191344u32);
    emu.adi_no_count(24usize, 0usize, 1u32, 2191348u32);
    emu.adi_no_count(25usize, 2usize, 56u32, 2191352u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(18usize, a);
    emu.pc = 2191356u32;
    emu.update_insn_clock();
    emu.adi_no_count(18usize, 18usize, 4294965632u32, 2191360u32);
    emu.add_memory_rw_events(10usize);
    let return_addr = 2191364u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2191388u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021701c));
}
#[inline(always)]
pub fn block_0x00217004(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 32u32, 2191368u32);
    emu.adi_no_count(11usize, 2usize, 12u32, 2191372u32);
    emu.apc_no_count(1usize, 2191372u32, 0u32, 2191376u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2191380u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966404u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00217014(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 32u32, 2191384u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2191564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002170cc));
    } else {
        emu.pc = 2191388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021701c));
    }
}
#[inline(always)]
pub fn block_0x0021701c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(19usize, 2usize, 36u32, 2191392u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2191504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217090));
    } else {
        emu.pc = 2191396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217024));
    }
}
#[inline]
pub fn block_0x00217024(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 40u32, 2191400u32)?;
    emu.lw_no_count(26usize, 2usize, 24u32, 2191404u32)?;
    emu.sw_no_count(21usize, 2usize, 56u32, 2191408u32)?;
    emu.sw_no_count(22usize, 2usize, 60u32, 2191412u32)?;
    emu.sw_no_count(0usize, 2usize, 48u32, 2191416u32)?;
    emu.adi_no_count(26usize, 26usize, 1u32, 2191420u32);
    emu.sw_no_count(10usize, 2usize, 28u32, 2191424u32)?;
    emu.sw_no_count(26usize, 2usize, 24u32, 2191428u32)?;
    emu.sw_no_count(23usize, 2usize, 32u32, 2191432u32)?;
    emu.sw_no_count(24usize, 2usize, 36u32, 2191436u32)?;
    emu.sw_no_count(25usize, 2usize, 40u32, 2191440u32)?;
    emu.sw_no_count(24usize, 2usize, 44u32, 2191444u32)?;
    emu.adi_no_count(12usize, 2usize, 32u32, 2191448u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2191452u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2191456u32);
    emu.apc_no_count(1usize, 2191456u32, 24576u32, 2191460u32);
    emu.add_memory_rw_events(17usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2191464u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966148u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00217068(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2191504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217090));
    } else {
        emu.pc = 2191468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021706c));
    }
}
#[inline(always)]
pub fn block_0x0021706c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(10usize, 26usize, 1u32, 2191472u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2191364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217004));
    } else {
        emu.pc = 2191476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217074));
    }
}
#[inline(always)]
pub fn block_0x00217074(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a >= b {
        emu.pc = 2191364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217004));
    } else {
        emu.pc = 2191480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217078));
    }
}
#[inline(always)]
pub fn block_0x00217078(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 9usize, 12u32, 2191484u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2191488u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2191492u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2191496u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2191500u32;
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
pub fn block_0x0021708c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2191364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217004));
    } else {
        emu.pc = 2191504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217090));
    }
}
#[inline]
pub fn block_0x00217090(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 19usize, 0u32, 2191508u32);
    emu.lw_no_count(1usize, 2usize, 108u32, 2191512u32)?;
    emu.lw_no_count(8usize, 2usize, 104u32, 2191516u32)?;
    emu.lw_no_count(9usize, 2usize, 100u32, 2191520u32)?;
    emu.lw_no_count(18usize, 2usize, 96u32, 2191524u32)?;
    emu.lw_no_count(19usize, 2usize, 92u32, 2191528u32)?;
    emu.lw_no_count(20usize, 2usize, 88u32, 2191532u32)?;
    emu.lw_no_count(21usize, 2usize, 84u32, 2191536u32)?;
    emu.lw_no_count(22usize, 2usize, 80u32, 2191540u32)?;
    emu.lw_no_count(23usize, 2usize, 76u32, 2191544u32)?;
    emu.lw_no_count(24usize, 2usize, 72u32, 2191548u32)?;
    emu.lw_no_count(25usize, 2usize, 68u32, 2191552u32)?;
    emu.lw_no_count(26usize, 2usize, 64u32, 2191556u32)?;
    emu.adi_no_count(2usize, 2usize, 112u32, 2191560u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2191564u32;
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
pub fn block_0x002170cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 36u32, 2191568u32)?;
    emu.lw_no_count(11usize, 2usize, 40u32, 2191572u32)?;
    emu.sw_no_count(10usize, 2usize, 56u32, 2191576u32)?;
    emu.sw_no_count(11usize, 2usize, 60u32, 2191580u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2191584u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965548u32, 2191588u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2191592u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294965532u32, 2191596u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2191600u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965564u32, 2191604u32);
    emu.adi_no_count(11usize, 0usize, 13u32, 2191608u32);
    emu.adi_no_count(12usize, 2usize, 56u32, 2191612u32);
    emu.apc_no_count(1usize, 2191612u32, 20480u32, 2191616u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2191620u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1692u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00217104(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2191624u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2191628u32)?;
    emu.adi_no_count(12usize, 10usize, 0u32, 2191632u32);
    emu.lbu_no_count(10usize, 10usize, 0u32, 2191636u32);
    emu.sli_no_count(10usize, 10usize, 2u32, 2191640u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2191644u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294965468u32, 2191648u32);
    emu.adr_no_count(10usize, 10usize, 13usize, 2191652u32);
    emu.lw_no_count(13usize, 10usize, 0u32, 2191656u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2191660u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2191664u32;
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
pub fn block_0x00217130(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4u32, 2191668u32);
    emu.sw_no_count(12usize, 2usize, 4u32, 2191672u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2191676u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965652u32, 2191680u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2191684u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294965662u32, 2191688u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2191692u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 4294965636u32, 2191696u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2191700u32);
    emu.adi_no_count(14usize, 0usize, 3u32, 2191704u32);
    emu.adi_no_count(15usize, 2usize, 4u32, 2191708u32);
    emu.apc_no_count(1usize, 2191708u32, 24576u32, 2191712u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2191716u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1008u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00217164(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2191720u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2191724u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2191728u32;
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
pub fn block_0x00217170(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2191732u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965719u32, 2191736u32);
    emu.adi_no_count(12usize, 0usize, 5u32, 2191740u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2191744u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2191748u32);
    emu.apc_no_count(6usize, 2191748u32, 24576u32, 2191752u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2191756u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x0021718c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2191760u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965674u32, 2191764u32);
    emu.adi_no_count(12usize, 0usize, 7u32, 2191768u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2191772u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2191776u32);
    emu.apc_no_count(6usize, 2191776u32, 24576u32, 2191780u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2191784u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(840u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002171a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 1u32, 2191788u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2191792u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2191796u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965700u32, 2191800u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2191804u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294965713u32, 2191808u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2191812u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 4294965684u32, 2191816u32);
    emu.adi_no_count(12usize, 0usize, 13u32, 2191820u32);
    emu.adi_no_count(14usize, 0usize, 6u32, 2191824u32);
    emu.adi_no_count(15usize, 2usize, 8u32, 2191828u32);
    emu.apc_no_count(1usize, 2191828u32, 24576u32, 2191832u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2191836u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(888u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002171dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2191840u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2191844u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2191848u32;
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
pub fn block_0x002171e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2191852u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965743u32, 2191856u32);
    emu.adi_no_count(12usize, 0usize, 11u32, 2191860u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2191864u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2191868u32);
    emu.apc_no_count(6usize, 2191868u32, 24576u32, 2191872u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2191876u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(748u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00217204(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2191880u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965665u32, 2191884u32);
    emu.adi_no_count(12usize, 0usize, 9u32, 2191888u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2191892u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2191896u32);
    emu.apc_no_count(6usize, 2191896u32, 24576u32, 2191900u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2191904u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(720u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00217220(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2191908u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965724u32, 2191912u32);
    emu.adi_no_count(12usize, 0usize, 6u32, 2191916u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2191920u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2191924u32);
    emu.apc_no_count(6usize, 2191924u32, 24576u32, 2191928u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2191932u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(692u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021723c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2191936u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965730u32, 2191940u32);
    emu.adi_no_count(12usize, 0usize, 13u32, 2191944u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2191948u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2191952u32);
    emu.apc_no_count(6usize, 2191952u32, 24576u32, 2191956u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2191960u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x00217258(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 0u32, 2191964u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2191968u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2191972u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2191976u32);
    emu.apc_no_count(6usize, 2191976u32, 24576u32, 2191980u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2191984u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(640u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00217270(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(6usize, 11usize, 12u32, 2191988u32)?;
    emu.adi_no_count(11usize, 12usize, 0u32, 2191992u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2191996u32;
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
pub fn block_0x0021727c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2192000u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2192004u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2192008u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2192012u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2192016u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2192020u32)?;
    emu.sw_no_count(20usize, 2usize, 24u32, 2192024u32)?;
    emu.sw_no_count(21usize, 2usize, 20u32, 2192028u32)?;
    emu.sw_no_count(22usize, 2usize, 16u32, 2192032u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2192036u32);
    emu.adi_no_count(10usize, 0usize, 128u32, 2192040u32);
    emu.sw_no_count(0usize, 2usize, 12u32, 2192044u32)?;
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2192060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002172bc));
    } else {
        emu.pc = 2192048u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002172b0));
    }
}
#[inline(always)]
pub fn block_0x002172b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 2usize, 12u32, 2192052u32);
    emu.adi_no_count(18usize, 0usize, 1u32, 2192056u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2192060u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2192216u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217358));
}
#[inline(always)]
pub fn block_0x002172bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 11u32, 2192064u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2192100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002172e4));
    } else {
        emu.pc = 2192068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002172c4));
    }
}
#[inline(always)]
pub fn block_0x002172c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 6u32, 2192072u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2192076u32);
    emu.ori_no_count(10usize, 10usize, 192u32, 2192080u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2192084u32);
    emu.sb_no_count(10usize, 2usize, 12u32, 2192088u32);
    emu.sb_no_count(11usize, 2usize, 13u32, 2192092u32);
    emu.adi_no_count(18usize, 0usize, 2u32, 2192096u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2192100u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2192216u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217358));
}
#[inline(always)]
pub fn block_0x002172e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 16u32, 2192104u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2192156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021731c));
    } else {
        emu.pc = 2192108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002172ec));
    }
}
#[inline]
pub fn block_0x002172ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 12u32, 2192112u32);
    emu.sli_no_count(12usize, 11usize, 20u32, 2192116u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2192120u32);
    emu.ori_no_count(10usize, 10usize, 224u32, 2192124u32);
    emu.sri_no_count(12usize, 12usize, 26u32, 2192128u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2192132u32);
    emu.adi_no_count(12usize, 12usize, 128u32, 2192136u32);
    emu.sb_no_count(10usize, 2usize, 12u32, 2192140u32);
    emu.sb_no_count(12usize, 2usize, 13u32, 2192144u32);
    emu.sb_no_count(11usize, 2usize, 14u32, 2192148u32);
    emu.adi_no_count(18usize, 0usize, 3u32, 2192152u32);
    emu.add_memory_rw_events(12usize);
    let return_addr = 2192156u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2192216u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217358));
}
#[inline]
pub fn block_0x0021731c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 18u32, 2192160u32);
    emu.sli_no_count(12usize, 11usize, 14u32, 2192164u32);
    emu.sli_no_count(13usize, 11usize, 20u32, 2192168u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2192172u32);
    emu.ori_no_count(10usize, 10usize, 240u32, 2192176u32);
    emu.sri_no_count(12usize, 12usize, 26u32, 2192180u32);
    emu.sri_no_count(13usize, 13usize, 26u32, 2192184u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2192188u32);
    emu.adi_no_count(12usize, 12usize, 128u32, 2192192u32);
    emu.adi_no_count(13usize, 13usize, 128u32, 2192196u32);
    emu.sb_no_count(10usize, 2usize, 12u32, 2192200u32);
    emu.sb_no_count(12usize, 2usize, 13u32, 2192204u32);
    emu.sb_no_count(13usize, 2usize, 14u32, 2192208u32);
    emu.sb_no_count(11usize, 2usize, 15u32, 2192212u32);
    emu.adi_no_count(18usize, 0usize, 4u32, 2192216u32);
    emu.add_memory_rw_events(15usize);
    emu.pc = 2192216u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217358));
}
#[inline]
pub fn block_0x00217358(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(19usize, 8usize, 8u32, 2192220u32)?;
    emu.lw_no_count(10usize, 19usize, 4u32, 2192224u32)?;
    emu.lw_no_count(21usize, 19usize, 8u32, 2192228u32)?;
    emu.lw_no_count(20usize, 19usize, 12u32, 2192232u32)?;
    emu.lw_no_count(11usize, 19usize, 0u32, 2192236u32)?;
    emu.sltru_no_count(12usize, 21usize, 10usize, 2192240u32);
    emu.sltiu_no_count(13usize, 20usize, 1u32, 2192244u32);
    emu.anr_no_count(14usize, 13usize, 12usize, 2192248u32);
    emu.adi_no_count(12usize, 21usize, 0u32, 2192252u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a != b {
        emu.pc = 2192260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217384));
    } else {
        emu.pc = 2192256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217380));
    }
}
#[inline(always)]
pub fn block_0x00217380(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2192260u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2192260u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217384));
}
#[inline]
pub fn block_0x00217384(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2192264u32);
    emu.orr_no_count(13usize, 13usize, 21usize, 2192268u32);
    emu.sbr_no_count(13usize, 10usize, 13usize, 2192272u32);
    emu.sltru_no_count(10usize, 10usize, 13usize, 2192276u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2192280u32);
    emu.anr_no_count(22usize, 10usize, 13usize, 2192284u32);
    emu.adr_no_count(10usize, 11usize, 12usize, 2192288u32);
    emu.adi_no_count(9usize, 22usize, 0u32, 2192292u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a < b {
        emu.pc = 2192300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002173ac));
    } else {
        emu.pc = 2192296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002173a8));
    }
}
#[inline(always)]
pub fn block_0x002173a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 18usize, 0u32, 2192300u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2192300u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002173ac));
}
#[inline(always)]
pub fn block_0x002173ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 2usize, 12u32, 2192304u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2192308u32);
    emu.apc_no_count(1usize, 2192308u32, 4294909952u32, 2192312u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2192316u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965612u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002173bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(9usize, 21usize, 9usize, 2192320u32);
    emu.sltru_no_count(10usize, 9usize, 21usize, 2192324u32);
    emu.adr_no_count(10usize, 20usize, 10usize, 2192328u32);
    emu.sw_no_count(9usize, 19usize, 8u32, 2192332u32)?;
    emu.sw_no_count(10usize, 19usize, 12u32, 2192336u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a >= b {
        emu.pc = 2192360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002173e8));
    } else {
        emu.pc = 2192340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002173d4));
    }
}
#[inline(always)]
pub fn block_0x002173d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2192344u32;
    emu.update_insn_clock();
    emu.lw_no_count(19usize, 10usize, 4294965872u32, 2192348u32)?;
    emu.ani_no_count(12usize, 19usize, 255u32, 2192352u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2192356u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2192368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002173f0));
    } else {
        emu.pc = 2192360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002173e8));
    }
}
#[inline(always)]
pub fn block_0x002173e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2192364u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2192368u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2192404u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217414));
}
#[inline(always)]
pub fn block_0x002173f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(20usize, 10usize, 4294965876u32, 2192372u32)?;
    emu.lbu_no_count(10usize, 8usize, 0u32, 2192376u32);
    emu.lw_no_count(9usize, 8usize, 4u32, 2192380u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2192444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021743c));
    } else {
        emu.pc = 2192384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217400));
    }
}
#[inline(always)]
pub fn block_0x00217400(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 3u32, 2192388u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2192444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021743c));
    } else {
        emu.pc = 2192392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217408));
    }
}
#[inline(always)]
pub fn block_0x00217408(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(19usize, 8usize, 0u32, 2192396u32)?;
    emu.sw_no_count(20usize, 8usize, 4u32, 2192400u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2192404u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2192404u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217414));
}
#[inline]
pub fn block_0x00217414(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2192408u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2192412u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2192416u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2192420u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2192424u32)?;
    emu.lw_no_count(20usize, 2usize, 24u32, 2192428u32)?;
    emu.lw_no_count(21usize, 2usize, 20u32, 2192432u32)?;
    emu.lw_no_count(22usize, 2usize, 16u32, 2192436u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2192440u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2192444u32;
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
pub fn block_0x0021743c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(21usize, 9usize, 4u32, 2192448u32)?;
    emu.lw_no_count(11usize, 21usize, 0u32, 2192452u32)?;
    emu.lw_no_count(18usize, 9usize, 0u32, 2192456u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2192468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217454));
    } else {
        emu.pc = 2192460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021744c));
    }
}
#[inline(always)]
pub fn block_0x0021744c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2192464u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2192468u32;
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
pub fn block_0x00217454(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 21usize, 4u32, 2192472u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2192492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021746c));
    } else {
        emu.pc = 2192476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021745c));
    }
}
#[inline(always)]
pub fn block_0x0021745c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 21usize, 8u32, 2192480u32)?;
    emu.adi_no_count(10usize, 18usize, 0u32, 2192484u32);
    emu.apc_no_count(1usize, 2192484u32, 4294905856u32, 2192488u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2192492u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966984u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021746c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 12u32, 2192496u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2192500u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2192504u32);
    emu.apc_no_count(1usize, 2192504u32, 4294905856u32, 2192508u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2192512u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966964u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00217480(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2192516u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2192392u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217408));
}
#[inline(always)]
pub fn block_0x00217484(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2192520u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2192524u32)?;
    emu.adi_no_count(10usize, 0usize, 128u32, 2192528u32);
    emu.sw_no_count(0usize, 2usize, 8u32, 2192532u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2192548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002174a4));
    } else {
        emu.pc = 2192536u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217498));
    }
}
#[inline(always)]
pub fn block_0x00217498(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 2usize, 8u32, 2192540u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2192544u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2192548u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2192704u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217540));
}
#[inline(always)]
pub fn block_0x002174a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 11u32, 2192552u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2192588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002174cc));
    } else {
        emu.pc = 2192556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002174ac));
    }
}
#[inline(always)]
pub fn block_0x002174ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 6u32, 2192560u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2192564u32);
    emu.ori_no_count(10usize, 10usize, 192u32, 2192568u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2192572u32);
    emu.sb_no_count(10usize, 2usize, 8u32, 2192576u32);
    emu.sb_no_count(11usize, 2usize, 9u32, 2192580u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2192584u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2192588u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2192704u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217540));
}
#[inline(always)]
pub fn block_0x002174cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 16u32, 2192592u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2192644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217504));
    } else {
        emu.pc = 2192596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002174d4));
    }
}
#[inline]
pub fn block_0x002174d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 12u32, 2192600u32);
    emu.sli_no_count(12usize, 11usize, 20u32, 2192604u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2192608u32);
    emu.ori_no_count(10usize, 10usize, 224u32, 2192612u32);
    emu.sri_no_count(12usize, 12usize, 26u32, 2192616u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2192620u32);
    emu.adi_no_count(12usize, 12usize, 128u32, 2192624u32);
    emu.sb_no_count(10usize, 2usize, 8u32, 2192628u32);
    emu.sb_no_count(12usize, 2usize, 9u32, 2192632u32);
    emu.sb_no_count(11usize, 2usize, 10u32, 2192636u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2192640u32);
    emu.add_memory_rw_events(12usize);
    let return_addr = 2192644u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2192704u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217540));
}
#[inline]
pub fn block_0x00217504(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 18u32, 2192648u32);
    emu.sli_no_count(12usize, 11usize, 14u32, 2192652u32);
    emu.sli_no_count(13usize, 11usize, 20u32, 2192656u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2192660u32);
    emu.ori_no_count(10usize, 10usize, 240u32, 2192664u32);
    emu.sri_no_count(12usize, 12usize, 26u32, 2192668u32);
    emu.sri_no_count(13usize, 13usize, 26u32, 2192672u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2192676u32);
    emu.adi_no_count(12usize, 12usize, 128u32, 2192680u32);
    emu.adi_no_count(13usize, 13usize, 128u32, 2192684u32);
    emu.sb_no_count(10usize, 2usize, 8u32, 2192688u32);
    emu.sb_no_count(12usize, 2usize, 9u32, 2192692u32);
    emu.sb_no_count(13usize, 2usize, 10u32, 2192696u32);
    emu.sb_no_count(11usize, 2usize, 11u32, 2192700u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2192704u32);
    emu.add_memory_rw_events(15usize);
    emu.pc = 2192704u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217540));
}
#[inline(always)]
pub fn block_0x00217540(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2192708u32);
    emu.adi_no_count(11usize, 2usize, 8u32, 2192712u32);
    emu.apc_no_count(1usize, 2192712u32, 4294905856u32, 2192716u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2192720u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1752u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00217550(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2192724u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2192728u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2192732u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2192736u32;
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
pub fn block_0x00217560(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2192740u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2192744u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2192748u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2192752u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2192756u32)?;
    emu.adi_no_count(12usize, 0usize, 128u32, 2192760u32);
    emu.sw_no_count(0usize, 2usize, 12u32, 2192764u32)?;
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2192780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021758c));
    } else {
        emu.pc = 2192768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217580));
    }
}
#[inline(always)]
pub fn block_0x00217580(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 2usize, 12u32, 2192772u32);
    emu.adi_no_count(8usize, 0usize, 1u32, 2192776u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2192780u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2192936u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217628));
}
#[inline(always)]
pub fn block_0x0021758c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 11usize, 11u32, 2192784u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2192820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002175b4));
    } else {
        emu.pc = 2192788u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217594));
    }
}
#[inline(always)]
pub fn block_0x00217594(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 11usize, 6u32, 2192792u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2192796u32);
    emu.ori_no_count(12usize, 12usize, 192u32, 2192800u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2192804u32);
    emu.sb_no_count(12usize, 2usize, 12u32, 2192808u32);
    emu.sb_no_count(11usize, 2usize, 13u32, 2192812u32);
    emu.adi_no_count(8usize, 0usize, 2u32, 2192816u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2192820u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2192936u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217628));
}
#[inline(always)]
pub fn block_0x002175b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 11usize, 16u32, 2192824u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2192876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002175ec));
    } else {
        emu.pc = 2192828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002175bc));
    }
}
#[inline]
pub fn block_0x002175bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 11usize, 12u32, 2192832u32);
    emu.sli_no_count(13usize, 11usize, 20u32, 2192836u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2192840u32);
    emu.ori_no_count(12usize, 12usize, 224u32, 2192844u32);
    emu.sri_no_count(13usize, 13usize, 26u32, 2192848u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2192852u32);
    emu.adi_no_count(13usize, 13usize, 128u32, 2192856u32);
    emu.sb_no_count(12usize, 2usize, 12u32, 2192860u32);
    emu.sb_no_count(13usize, 2usize, 13u32, 2192864u32);
    emu.sb_no_count(11usize, 2usize, 14u32, 2192868u32);
    emu.adi_no_count(8usize, 0usize, 3u32, 2192872u32);
    emu.add_memory_rw_events(12usize);
    let return_addr = 2192876u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2192936u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217628));
}
#[inline]
pub fn block_0x002175ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 11usize, 18u32, 2192880u32);
    emu.sli_no_count(13usize, 11usize, 14u32, 2192884u32);
    emu.sli_no_count(14usize, 11usize, 20u32, 2192888u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2192892u32);
    emu.ori_no_count(12usize, 12usize, 240u32, 2192896u32);
    emu.sri_no_count(13usize, 13usize, 26u32, 2192900u32);
    emu.sri_no_count(14usize, 14usize, 26u32, 2192904u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2192908u32);
    emu.adi_no_count(13usize, 13usize, 128u32, 2192912u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2192916u32);
    emu.sb_no_count(12usize, 2usize, 12u32, 2192920u32);
    emu.sb_no_count(13usize, 2usize, 13u32, 2192924u32);
    emu.sb_no_count(14usize, 2usize, 14u32, 2192928u32);
    emu.sb_no_count(11usize, 2usize, 15u32, 2192932u32);
    emu.adi_no_count(8usize, 0usize, 4u32, 2192936u32);
    emu.add_memory_rw_events(15usize);
    emu.pc = 2192936u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217628));
}
#[inline(always)]
pub fn block_0x00217628(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(9usize, 10usize, 8u32, 2192940u32)?;
    emu.lw_no_count(10usize, 9usize, 0u32, 2192944u32)?;
    emu.lw_no_count(18usize, 9usize, 8u32, 2192948u32)?;
    emu.sbr_no_count(10usize, 10usize, 18usize, 2192952u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2193016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217678));
    } else {
        emu.pc = 2192956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021763c));
    }
}
#[inline(always)]
pub fn block_0x0021763c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 4u32, 2192960u32)?;
    emu.adr_no_count(10usize, 10usize, 18usize, 2192964u32);
    emu.adi_no_count(11usize, 2usize, 12u32, 2192968u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2192972u32);
    emu.apc_no_count(1usize, 2192972u32, 4294905856u32, 2192976u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2192980u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1748u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00217654(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 18usize, 8usize, 2192984u32);
    emu.sw_no_count(8usize, 9usize, 8u32, 2192988u32)?;
    emu.adi_no_count(10usize, 0usize, 0u32, 2192992u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2192996u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2193000u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2193004u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2193008u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2193012u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2193016u32;
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
pub fn block_0x00217678(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2193020u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2193024u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2193028u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2193032u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2193036u32);
    emu.apc_no_count(1usize, 2193036u32, 0u32, 2193040u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2193044u32;
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
pub fn block_0x00217694(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 9usize, 8u32, 2193048u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2193052u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2192956u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021763c));
}
#[inline(always)]
pub fn block_0x0021769c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 0u32, 2193056u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2193060u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965756u32, 2193064u32);
    emu.apc_no_count(6usize, 2193064u32, 20480u32, 2193068u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2193072u32;
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
#[inline(always)]
pub fn block_0x002176b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 0u32, 2193076u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2193080u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965804u32, 2193084u32);
    emu.apc_no_count(6usize, 2193084u32, 20480u32, 2193088u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2193092u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1320u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002176c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 0u32, 2193096u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2193100u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965780u32, 2193104u32);
    emu.apc_no_count(6usize, 2193104u32, 20480u32, 2193108u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2193112u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1300u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002176d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2193116u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2193120u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2193124u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2193128u32)?;
    emu.sw_no_count(18usize, 2usize, 0u32, 2193132u32)?;
    emu.lbu_no_count(11usize, 10usize, 0u32, 2193136u32);
    emu.lw_no_count(8usize, 10usize, 4u32, 2193140u32)?;
    emu.adi_no_count(10usize, 0usize, 4u32, 2193144u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2193180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021771c));
    } else {
        emu.pc = 2193148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002176fc));
    }
}
#[inline(always)]
pub fn block_0x002176fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 3u32, 2193152u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2193180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021771c));
    } else {
        emu.pc = 2193156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217704));
    }
}
#[inline(always)]
pub fn block_0x00217704(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2193160u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2193164u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2193168u32)?;
    emu.lw_no_count(18usize, 2usize, 0u32, 2193172u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2193176u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2193180u32;
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
pub fn block_0x0021771c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 8usize, 4u32, 2193184u32)?;
    emu.lw_no_count(11usize, 18usize, 0u32, 2193188u32)?;
    emu.lw_no_count(9usize, 8usize, 0u32, 2193192u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2193204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217734));
    } else {
        emu.pc = 2193196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021772c));
    }
}
#[inline(always)]
pub fn block_0x0021772c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2193200u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2193204u32;
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
pub fn block_0x00217734(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 18usize, 4u32, 2193208u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2193228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021774c));
    } else {
        emu.pc = 2193212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021773c));
    }
}
#[inline(always)]
pub fn block_0x0021773c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 8u32, 2193216u32)?;
    emu.adi_no_count(10usize, 9usize, 0u32, 2193220u32);
    emu.apc_no_count(1usize, 2193220u32, 4294905856u32, 2193224u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2193228u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966248u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021774c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 12u32, 2193232u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2193236u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2193240u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2193244u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2193248u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2193252u32)?;
    emu.lw_no_count(18usize, 2usize, 0u32, 2193256u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2193260u32);
    emu.apc_no_count(6usize, 2193260u32, 4294905856u32, 2193264u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2193268u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966208u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00217774(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2193272u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2193276u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2193280u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2193284u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2193288u32)?;
    emu.adi_no_count(9usize, 10usize, 0u32, 2193292u32);
    emu.adi_no_count(18usize, 0usize, 4u32, 2193296u32);
    emu.sb_no_count(18usize, 2usize, 8u32, 2193300u32);
    emu.sw_no_count(11usize, 2usize, 16u32, 2193304u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2193308u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965780u32, 2193312u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2193316u32);
    emu.apc_no_count(1usize, 2193316u32, 20480u32, 2193320u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2193324u32;
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
pub fn block_0x002177ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2193376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002177e0));
    } else {
        emu.pc = 2193328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002177b0));
    }
}
#[inline(always)]
pub fn block_0x002177b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 8u32, 2193332u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2193492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217854));
    } else {
        emu.pc = 2193336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002177b8));
    }
}
#[inline(always)]
pub fn block_0x002177b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2193340u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2193344u32)?;
    emu.sw_no_count(10usize, 9usize, 0u32, 2193348u32)?;
    emu.sw_no_count(11usize, 9usize, 4u32, 2193352u32)?;
    emu.add_memory_rw_events(4usize);
    emu.pc = 2193352u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002177c8));
}
#[inline(always)]
pub fn block_0x002177c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2193356u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2193360u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2193364u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2193368u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2193372u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2193376u32;
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
pub fn block_0x002177e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 8u32, 2193380u32);
    emu.lw_no_count(8usize, 2usize, 12u32, 2193384u32)?;
    emu.sb_no_count(18usize, 9usize, 0u32, 2193388u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a < b {
        emu.pc = 2193400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002177f8));
    } else {
        emu.pc = 2193392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002177f0));
    }
}
#[inline(always)]
pub fn block_0x002177f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 3u32, 2193396u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2193352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002177c8));
    } else {
        emu.pc = 2193400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002177f8));
    }
}
#[inline(always)]
pub fn block_0x002177f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 8usize, 4u32, 2193404u32)?;
    emu.lw_no_count(11usize, 18usize, 0u32, 2193408u32)?;
    emu.lw_no_count(9usize, 8usize, 0u32, 2193412u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2193424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217810));
    } else {
        emu.pc = 2193416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217808));
    }
}
#[inline(always)]
pub fn block_0x00217808(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2193420u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2193424u32;
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
pub fn block_0x00217810(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 18usize, 4u32, 2193428u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2193448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217828));
    } else {
        emu.pc = 2193432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217818));
    }
}
#[inline(always)]
pub fn block_0x00217818(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 8u32, 2193436u32)?;
    emu.adi_no_count(10usize, 9usize, 0u32, 2193440u32);
    emu.apc_no_count(1usize, 2193440u32, 4294905856u32, 2193444u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2193448u32;
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
pub fn block_0x00217828(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 12u32, 2193452u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2193456u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2193460u32);
    emu.apc_no_count(1usize, 2193460u32, 4294905856u32, 2193464u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2193468u32;
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
pub fn block_0x0021783c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2193472u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2193476u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2193480u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2193484u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2193488u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2193492u32;
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
pub fn block_0x00217854(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2193496u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965968u32, 2193500u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2193504u32);
    emu.sw_no_count(0usize, 2usize, 40u32, 2193508u32)?;
    emu.adi_no_count(12usize, 0usize, 4u32, 2193512u32);
    emu.sw_no_count(10usize, 2usize, 24u32, 2193516u32)?;
    emu.sw_no_count(11usize, 2usize, 28u32, 2193520u32)?;
    emu.sw_no_count(12usize, 2usize, 32u32, 2193524u32)?;
    emu.sw_no_count(0usize, 2usize, 36u32, 2193528u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2193532u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965976u32, 2193536u32);
    emu.adi_no_count(10usize, 2usize, 24u32, 2193540u32);
    emu.apc_no_count(1usize, 2193540u32, 16384u32, 2193544u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2193548u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966176u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
