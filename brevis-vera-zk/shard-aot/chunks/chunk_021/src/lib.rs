pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2171400u32;
pub const PC_MAX: u32 = 2173328u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 109usize] = [
        block_0x00212208,
        block_0x00212258,
        block_0x0021226c,
        block_0x0021227c,
        block_0x00212290,
        block_0x00212294,
        block_0x00212298,
        block_0x0021229c,
        block_0x002122a4,
        block_0x002122a8,
        block_0x002122ac,
        block_0x002122b4,
        block_0x002122ec,
        block_0x00212354,
        block_0x002123b0,
        block_0x00212420,
        block_0x00212450,
        block_0x00212480,
        block_0x00212494,
        block_0x002124a4,
        block_0x002124b4,
        block_0x002124c4,
        block_0x002124d4,
        block_0x002124d8,
        block_0x002124e8,
        block_0x00212500,
        block_0x00212504,
        block_0x00212520,
        block_0x00212530,
        block_0x00212538,
        block_0x00212540,
        block_0x00212554,
        block_0x00212564,
        block_0x00212568,
        block_0x00212570,
        block_0x00212574,
        block_0x00212588,
        block_0x002125b8,
        block_0x002125bc,
        block_0x002125c4,
        block_0x002125d8,
        block_0x002125e4,
        block_0x002125e8,
        block_0x002125f0,
        block_0x0021260c,
        block_0x00212610,
        block_0x00212628,
        block_0x0021263c,
        block_0x00212648,
        block_0x0021264c,
        block_0x00212660,
        block_0x00212688,
        block_0x0021268c,
        block_0x00212694,
        block_0x0021269c,
        block_0x002126ac,
        block_0x002126b0,
        block_0x002126c4,
        block_0x002126ec,
        block_0x002126f0,
        block_0x002126f8,
        block_0x00212700,
        block_0x00212708,
        block_0x0021271c,
        block_0x00212744,
        block_0x00212748,
        block_0x00212750,
        block_0x00212758,
        block_0x00212760,
        block_0x00212770,
        block_0x00212774,
        block_0x00212788,
        block_0x00212798,
        block_0x002127ac,
        block_0x002127bc,
        block_0x002127d0,
        block_0x002127e0,
        block_0x002127f4,
        block_0x002127f8,
        block_0x00212800,
        block_0x00212830,
        block_0x00212834,
        block_0x0021284c,
        block_0x00212850,
        block_0x00212868,
        block_0x0021286c,
        block_0x00212870,
        block_0x00212880,
        block_0x002128b4,
        block_0x002128b8,
        block_0x002128c8,
        block_0x002128cc,
        block_0x002128d0,
        block_0x002128e0,
        block_0x002128e4,
        block_0x002128e8,
        block_0x002128fc,
        block_0x00212900,
        block_0x00212918,
        block_0x0021291c,
        block_0x00212920,
        block_0x00212930,
        block_0x00212964,
        block_0x00212968,
        block_0x00212978,
        block_0x0021297c,
        block_0x00212980,
        block_0x0021298c,
        block_0x00212990,
    ];
    const IDX: [u16; 483usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 0u16, 0u16, 3u16,
        0u16, 0u16, 0u16, 4u16, 0u16, 0u16, 0u16, 0u16, 5u16, 6u16, 7u16, 8u16, 0u16,
        9u16, 10u16, 11u16, 0u16, 12u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 13u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 14u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 15u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 16u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 17u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 18u16, 0u16, 0u16, 0u16, 0u16, 19u16, 0u16, 0u16, 0u16, 20u16, 0u16,
        0u16, 0u16, 21u16, 0u16, 0u16, 0u16, 22u16, 0u16, 0u16, 0u16, 23u16, 24u16, 0u16,
        0u16, 0u16, 25u16, 0u16, 0u16, 0u16, 0u16, 0u16, 26u16, 27u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 28u16, 0u16, 0u16, 0u16, 29u16, 0u16, 30u16, 0u16, 31u16, 0u16,
        0u16, 0u16, 0u16, 32u16, 0u16, 0u16, 0u16, 33u16, 34u16, 0u16, 35u16, 36u16,
        0u16, 0u16, 0u16, 0u16, 37u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 38u16, 39u16, 0u16, 40u16, 0u16, 0u16, 0u16, 0u16, 41u16, 0u16,
        0u16, 42u16, 43u16, 0u16, 44u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 45u16,
        46u16, 0u16, 0u16, 0u16, 0u16, 0u16, 47u16, 0u16, 0u16, 0u16, 0u16, 48u16, 0u16,
        0u16, 49u16, 50u16, 0u16, 0u16, 0u16, 0u16, 51u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 52u16, 53u16, 0u16, 54u16, 0u16, 55u16, 0u16, 0u16, 0u16,
        56u16, 57u16, 0u16, 0u16, 0u16, 0u16, 58u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 59u16, 60u16, 0u16, 61u16, 0u16, 62u16, 0u16, 63u16, 0u16,
        0u16, 0u16, 0u16, 64u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        65u16, 66u16, 0u16, 67u16, 0u16, 68u16, 0u16, 69u16, 0u16, 0u16, 0u16, 70u16,
        71u16, 0u16, 0u16, 0u16, 0u16, 72u16, 0u16, 0u16, 0u16, 73u16, 0u16, 0u16, 0u16,
        0u16, 74u16, 0u16, 0u16, 0u16, 75u16, 0u16, 0u16, 0u16, 0u16, 76u16, 0u16, 0u16,
        0u16, 77u16, 0u16, 0u16, 0u16, 0u16, 78u16, 79u16, 0u16, 80u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 81u16, 82u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 83u16, 84u16, 0u16, 0u16, 0u16, 0u16, 0u16, 85u16, 86u16, 87u16,
        0u16, 0u16, 0u16, 88u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 89u16, 90u16, 0u16, 0u16, 0u16, 91u16, 92u16, 93u16, 0u16,
        0u16, 0u16, 94u16, 95u16, 96u16, 0u16, 0u16, 0u16, 0u16, 97u16, 98u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 99u16, 100u16, 101u16, 0u16, 0u16, 0u16, 102u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 103u16, 104u16,
        0u16, 0u16, 0u16, 105u16, 106u16, 107u16, 0u16, 0u16, 108u16, 109u16,
    ];
    if pc < 2171400u32 || pc > 2173328u32 {
        return None;
    }
    let word_offset = ((pc - 2171400u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x00212208(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294965904u32, 2171404u32);
    emu.sw_no_count(1usize, 2usize, 1388u32, 2171408u32)?;
    emu.sw_no_count(8usize, 2usize, 1384u32, 2171412u32)?;
    emu.sw_no_count(9usize, 2usize, 1380u32, 2171416u32)?;
    emu.sw_no_count(18usize, 2usize, 1376u32, 2171420u32)?;
    emu.sw_no_count(19usize, 2usize, 1372u32, 2171424u32)?;
    emu.sw_no_count(20usize, 2usize, 1368u32, 2171428u32)?;
    emu.sw_no_count(21usize, 2usize, 1364u32, 2171432u32)?;
    emu.sw_no_count(22usize, 2usize, 1360u32, 2171436u32)?;
    emu.sw_no_count(23usize, 2usize, 1356u32, 2171440u32)?;
    emu.sw_no_count(24usize, 2usize, 1352u32, 2171444u32)?;
    emu.sw_no_count(25usize, 2usize, 1348u32, 2171448u32)?;
    emu.sw_no_count(26usize, 2usize, 1344u32, 2171452u32)?;
    emu.sw_no_count(27usize, 2usize, 1340u32, 2171456u32)?;
    emu.adi_no_count(24usize, 13usize, 0u32, 2171460u32);
    emu.sw_no_count(12usize, 2usize, 20u32, 2171464u32)?;
    emu.lw_no_count(12usize, 11usize, 0u32, 2171468u32)?;
    emu.lw_no_count(13usize, 11usize, 4u32, 2171472u32)?;
    emu.orr_no_count(14usize, 12usize, 13usize, 2171476u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2174888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2174888u32));
    } else {
        emu.pc = 2171480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212258));
    }
}
#[inline(always)]
pub fn block_0x00212258(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(23usize, 10usize, 0u32, 2171484u32);
    emu.lw_no_count(18usize, 11usize, 8u32, 2171488u32)?;
    emu.lw_no_count(20usize, 11usize, 12u32, 2171492u32)?;
    emu.orr_no_count(10usize, 18usize, 20usize, 2171496u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2174916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2174916u32));
    } else {
        emu.pc = 2171500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021226c));
    }
}
#[inline(always)]
pub fn block_0x0021226c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(8usize, 11usize, 16u32, 2171504u32)?;
    emu.lw_no_count(9usize, 11usize, 20u32, 2171508u32)?;
    emu.orr_no_count(10usize, 8usize, 9usize, 2171512u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2174944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2174944u32));
    } else {
        emu.pc = 2171516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021227c));
    }
}
#[inline(always)]
pub fn block_0x0021227c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 12usize, 8usize, 2171520u32);
    emu.sltru_no_count(15usize, 10usize, 12usize, 2171524u32);
    emu.adr_no_count(14usize, 13usize, 9usize, 2171528u32);
    emu.adr_no_count(16usize, 14usize, 15usize, 2171532u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2171540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212294));
    } else {
        emu.pc = 2171536u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212290));
    }
}
#[inline(always)]
pub fn block_0x00212290(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 16usize, 13usize, 2171540u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2171540u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212294));
}
#[inline(always)]
pub fn block_0x00212294(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2174972u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2174972u32));
    } else {
        emu.pc = 2171544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212298));
    }
}
#[inline(always)]
pub fn block_0x00212298(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2171556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002122a4));
    } else {
        emu.pc = 2171548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021229c));
    }
}
#[inline(always)]
pub fn block_0x0021229c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 13usize, 20usize, 2171552u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2171556u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2171560u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002122a8));
}
#[inline(always)]
pub fn block_0x002122a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 12usize, 18usize, 2171560u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2171560u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002122a8));
}
#[inline(always)]
pub fn block_0x002122a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2175000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2175000u32));
    } else {
        emu.pc = 2171564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002122ac));
    }
}
#[inline(always)]
pub fn block_0x002122ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 16u32, 2171568u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2175028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2175028u32));
    } else {
        emu.pc = 2171572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002122b4));
    }
}
#[inline]
pub fn block_0x002122b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lh_no_count(19usize, 11usize, 24u32, 2171576u32)?;
    emu.sltru_no_count(15usize, 10usize, 12usize, 2171580u32);
    emu.sltiu_no_count(5usize, 10usize, 1u32, 2171584u32);
    let a = 0u32.wrapping_add(1431654400u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2171588u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(858992640u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2171592u32;
    emu.update_insn_clock();
    emu.adr_no_count(15usize, 14usize, 15usize, 2171596u32);
    let a = 0u32.wrapping_add(252645376u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2171600u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 16usize, 1365u32, 2171604u32);
    emu.adi_no_count(16usize, 6usize, 819u32, 2171608u32);
    emu.adi_no_count(14usize, 14usize, 4294967055u32, 2171612u32);
    emu.sbr_no_count(5usize, 15usize, 5usize, 2171616u32);
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2171620u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 257u32, 2171624u32);
    emu.add_memory_rw_events(13usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a != b {
        emu.pc = 2171732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212354));
    } else {
        emu.pc = 2171628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002122ec));
    }
}
#[inline(never)]
pub fn block_0x002122ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 26u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2171632u32);
    emu.sri_no_count(5usize, 10usize, 1u32, 2171636u32);
    emu.orr_no_count(10usize, 10usize, 5usize, 2171640u32);
    emu.sri_no_count(5usize, 10usize, 2u32, 2171644u32);
    emu.orr_no_count(10usize, 10usize, 5usize, 2171648u32);
    emu.sri_no_count(5usize, 10usize, 4u32, 2171652u32);
    emu.orr_no_count(10usize, 10usize, 5usize, 2171656u32);
    emu.sri_no_count(5usize, 10usize, 8u32, 2171660u32);
    emu.orr_no_count(10usize, 10usize, 5usize, 2171664u32);
    emu.sri_no_count(5usize, 10usize, 16u32, 2171668u32);
    emu.orr_no_count(10usize, 10usize, 5usize, 2171672u32);
    emu.xri_no_count(10usize, 10usize, 4294967295u32, 2171676u32);
    emu.sri_no_count(5usize, 10usize, 1u32, 2171680u32);
    emu.anr_no_count(17usize, 5usize, 17usize, 2171684u32);
    emu.sbr_no_count(10usize, 10usize, 17usize, 2171688u32);
    emu.anr_no_count(17usize, 10usize, 16usize, 2171692u32);
    emu.sri_no_count(10usize, 10usize, 2u32, 2171696u32);
    emu.anr_no_count(10usize, 10usize, 16usize, 2171700u32);
    emu.adr_no_count(10usize, 17usize, 10usize, 2171704u32);
    emu.sri_no_count(16usize, 10usize, 4u32, 2171708u32);
    emu.adr_no_count(10usize, 10usize, 16usize, 2171712u32);
    emu.anr_no_count(10usize, 10usize, 14usize, 2171716u32);
    emu.mul_no_count(10usize, 10usize, 15usize, 2171720u32);
    emu.sri_no_count(10usize, 10usize, 24u32, 2171724u32);
    emu.adi_no_count(10usize, 10usize, 32u32, 2171728u32);
    emu.add_memory_rw_events(26usize);
    let return_addr = 2171732u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2171824u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002123b0));
}
#[inline]
pub fn block_0x00212354(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 23u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 5usize, 1u32, 2171736u32);
    emu.orr_no_count(10usize, 5usize, 10usize, 2171740u32);
    emu.sri_no_count(5usize, 10usize, 2u32, 2171744u32);
    emu.orr_no_count(10usize, 10usize, 5usize, 2171748u32);
    emu.sri_no_count(5usize, 10usize, 4u32, 2171752u32);
    emu.orr_no_count(10usize, 10usize, 5usize, 2171756u32);
    emu.sri_no_count(5usize, 10usize, 8u32, 2171760u32);
    emu.orr_no_count(10usize, 10usize, 5usize, 2171764u32);
    emu.sri_no_count(5usize, 10usize, 16u32, 2171768u32);
    emu.orr_no_count(10usize, 10usize, 5usize, 2171772u32);
    emu.xri_no_count(10usize, 10usize, 4294967295u32, 2171776u32);
    emu.sri_no_count(5usize, 10usize, 1u32, 2171780u32);
    emu.anr_no_count(17usize, 5usize, 17usize, 2171784u32);
    emu.sbr_no_count(10usize, 10usize, 17usize, 2171788u32);
    emu.anr_no_count(17usize, 10usize, 16usize, 2171792u32);
    emu.sri_no_count(10usize, 10usize, 2u32, 2171796u32);
    emu.anr_no_count(10usize, 10usize, 16usize, 2171800u32);
    emu.adr_no_count(10usize, 17usize, 10usize, 2171804u32);
    emu.sri_no_count(16usize, 10usize, 4u32, 2171808u32);
    emu.adr_no_count(10usize, 10usize, 16usize, 2171812u32);
    emu.anr_no_count(10usize, 10usize, 14usize, 2171816u32);
    emu.mul_no_count(10usize, 10usize, 15usize, 2171820u32);
    emu.sri_no_count(10usize, 10usize, 24u32, 2171824u32);
    emu.add_memory_rw_events(23usize);
    emu.pc = 2171824u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002123b0));
}
#[inline(never)]
pub fn block_0x002123b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(11usize, 11usize, 26u32, 2171828u32);
    emu.sw_no_count(11usize, 2usize, 24u32, 2171832u32)?;
    emu.sbr_no_count(10usize, 19usize, 10usize, 2171836u32);
    let a = 0u32.wrapping_add(1292914688u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2171840u32;
    emu.update_insn_clock();
    emu.sltiu_no_count(14usize, 13usize, 1u32, 2171844u32);
    emu.adi_no_count(11usize, 11usize, 4294966594u32, 2171848u32);
    emu.mulh_no_count(15usize, 10usize, 11usize, 2171852u32);
    emu.mul_no_count(10usize, 10usize, 11usize, 2171856u32);
    emu.adi_no_count(11usize, 14usize, 4294967295u32, 2171860u32);
    emu.anr_no_count(11usize, 11usize, 13usize, 2171864u32);
    emu.adi_no_count(22usize, 0usize, 2u32, 2171868u32);
    emu.sbr_no_count(13usize, 22usize, 14usize, 2171872u32);
    emu.sw_no_count(13usize, 2usize, 188u32, 2171876u32)?;
    let a = 0u32.wrapping_add(1142116352u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2171880u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 128u32, 2171884u32);
    emu.adr_no_count(13usize, 10usize, 13usize, 2171888u32);
    emu.sw_no_count(12usize, 2usize, 28u32, 2171892u32)?;
    emu.sw_no_count(11usize, 2usize, 32u32, 2171896u32)?;
    emu.sltru_no_count(10usize, 13usize, 10usize, 2171900u32);
    emu.adr_no_count(10usize, 15usize, 10usize, 2171904u32);
    emu.adi_no_count(21usize, 10usize, 19u32, 2171908u32);
    emu.sli_no_count(10usize, 21usize, 16u32, 2171912u32);
    emu.sai_no_count(25usize, 10usize, 1040u32, 2171916u32);
    emu.adi_no_count(10usize, 2usize, 36u32, 2171920u32);
    emu.adi_no_count(12usize, 0usize, 152u32, 2171924u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2171928u32);
    emu.apc_no_count(1usize, 2171928u32, 4294905856u32, 2171932u32);
    emu.add_memory_rw_events(28usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2171936u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966136u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00212420(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(10usize, 20usize, 1u32, 2171940u32);
    emu.adi_no_count(11usize, 10usize, 4294967295u32, 2171944u32);
    emu.sbr_no_count(10usize, 22usize, 10usize, 2171948u32);
    emu.anr_no_count(11usize, 11usize, 20usize, 2171952u32);
    emu.sw_no_count(10usize, 2usize, 352u32, 2171956u32)?;
    emu.sw_no_count(18usize, 2usize, 192u32, 2171960u32)?;
    emu.sw_no_count(11usize, 2usize, 196u32, 2171964u32)?;
    emu.adi_no_count(10usize, 2usize, 200u32, 2171968u32);
    emu.adi_no_count(12usize, 0usize, 152u32, 2171972u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2171976u32);
    emu.apc_no_count(1usize, 2171976u32, 4294905856u32, 2171980u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2171984u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966088u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00212450(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(10usize, 9usize, 1u32, 2171988u32);
    emu.adi_no_count(11usize, 10usize, 4294967295u32, 2171992u32);
    emu.sbr_no_count(10usize, 22usize, 10usize, 2171996u32);
    emu.anr_no_count(11usize, 11usize, 9usize, 2172000u32);
    emu.sw_no_count(10usize, 2usize, 516u32, 2172004u32)?;
    emu.sw_no_count(8usize, 2usize, 356u32, 2172008u32)?;
    emu.sw_no_count(11usize, 2usize, 360u32, 2172012u32)?;
    emu.adi_no_count(10usize, 2usize, 364u32, 2172016u32);
    emu.adi_no_count(12usize, 0usize, 152u32, 2172020u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2172024u32);
    emu.apc_no_count(1usize, 2172024u32, 4294905856u32, 2172028u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2172032u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966040u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00212480(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 524u32, 2172036u32);
    emu.adi_no_count(12usize, 0usize, 156u32, 2172040u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2172044u32);
    emu.apc_no_count(1usize, 2172044u32, 4294905856u32, 2172048u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2172052u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966020u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00212494(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2172056u32);
    emu.sw_no_count(10usize, 2usize, 680u32, 2172060u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2172064u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2172136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002124e8));
    } else {
        emu.pc = 2172068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002124a4));
    }
}
#[inline(always)]
pub fn block_0x002124a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 28u32, 2172072u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2172076u32);
    emu.apc_no_count(1usize, 2172076u32, 4294942720u32, 2172080u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2172084u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1560u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002124b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 192u32, 2172088u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2172092u32);
    emu.apc_no_count(1usize, 2172092u32, 4294942720u32, 2172096u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2172100u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1544u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002124c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 356u32, 2172104u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2172108u32);
    emu.apc_no_count(1usize, 2172108u32, 4294942720u32, 2172112u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2172116u32;
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
pub fn block_0x002124d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(25usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2172164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212504));
    } else {
        emu.pc = 2172120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002124d8));
    }
}
#[inline(always)]
pub fn block_0x002124d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 21usize, 17u32, 2172124u32);
    emu.sri_no_count(11usize, 11usize, 17u32, 2172128u32);
    emu.adi_no_count(10usize, 2usize, 520u32, 2172132u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2172136u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2172216u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212538));
}
#[inline(always)]
pub fn block_0x002124e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 0usize, 19usize, 2172140u32);
    emu.sli_no_count(10usize, 10usize, 16u32, 2172144u32);
    emu.sai_no_count(11usize, 10usize, 1040u32, 2172148u32);
    emu.adi_no_count(10usize, 2usize, 520u32, 2172152u32);
    emu.apc_no_count(1usize, 2172152u32, 4294942720u32, 2172156u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2172160u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1484u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00212500(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(25usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2172120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002124d8));
    } else {
        emu.pc = 2172164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212504));
    }
}
#[inline(always)]
pub fn block_0x00212504(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 0usize, 25usize, 2172168u32);
    emu.sli_no_count(10usize, 10usize, 16u32, 2172172u32);
    emu.sri_no_count(19usize, 10usize, 16u32, 2172176u32);
    emu.adi_no_count(10usize, 2usize, 28u32, 2172180u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2172184u32);
    emu.apc_no_count(1usize, 2172184u32, 0u32, 2172188u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2172192u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x00212520(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 192u32, 2172196u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2172200u32);
    emu.apc_no_count(1usize, 2172200u32, 0u32, 2172204u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2172208u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965308u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00212530(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 356u32, 2172212u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2172216u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2172216u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212538));
}
#[inline(always)]
pub fn block_0x00212538(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2172216u32, 0u32, 2172220u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2172224u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965292u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00212540(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1176u32, 2172228u32);
    emu.adi_no_count(11usize, 2usize, 28u32, 2172232u32);
    emu.adi_no_count(12usize, 0usize, 164u32, 2172236u32);
    emu.apc_no_count(1usize, 2172236u32, 4294905856u32, 2172240u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2172244u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966076u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00212554(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 2usize, 1336u32, 2172248u32)?;
    emu.lw_no_count(11usize, 2usize, 516u32, 2172252u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2172256u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2172264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212568));
    } else {
        emu.pc = 2172260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212564));
    }
}
#[inline(always)]
pub fn block_0x00212564(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 12usize, 0u32, 2172264u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2172264u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212568));
}
#[inline(always)]
pub fn block_0x00212568(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 41u32, 2172268u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2174744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2174744u32));
    } else {
        emu.pc = 2172272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212570));
    }
}
#[inline(always)]
pub fn block_0x00212570(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2172376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002125d8));
    } else {
        emu.pc = 2172276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212574));
    }
}
#[inline(always)]
pub fn block_0x00212574(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 0u32, 2172280u32);
    emu.sli_no_count(12usize, 10usize, 2u32, 2172284u32);
    emu.adi_no_count(13usize, 2usize, 356u32, 2172288u32);
    emu.adr_no_count(14usize, 13usize, 12usize, 2172292u32);
    emu.adi_no_count(15usize, 2usize, 1176u32, 2172296u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2172296u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212588));
}
#[inline]
pub fn block_0x00212588(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(17usize, 15usize, 0u32, 2172300u32)?;
    emu.lw_no_count(5usize, 13usize, 0u32, 2172304u32)?;
    emu.ani_no_count(16usize, 16usize, 1u32, 2172308u32);
    emu.adi_no_count(13usize, 13usize, 4u32, 2172312u32);
    emu.adr_no_count(5usize, 17usize, 5usize, 2172316u32);
    emu.sltru_no_count(17usize, 5usize, 17usize, 2172320u32);
    emu.adr_no_count(16usize, 5usize, 16usize, 2172324u32);
    emu.sltru_no_count(5usize, 16usize, 5usize, 2172328u32);
    emu.sw_no_count(16usize, 15usize, 0u32, 2172332u32)?;
    emu.orr_no_count(16usize, 17usize, 5usize, 2172336u32);
    emu.adi_no_count(15usize, 15usize, 4u32, 2172340u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2172296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212588));
    } else {
        emu.pc = 2172344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002125b8));
    }
}
#[inline(always)]
pub fn block_0x002125b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2172376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002125d8));
    } else {
        emu.pc = 2172348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002125bc));
    }
}
#[inline(always)]
pub fn block_0x002125bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 40u32, 2172352u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2175076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2175076u32));
    } else {
        emu.pc = 2172356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002125c4));
    }
}
#[inline(always)]
pub fn block_0x002125c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 2usize, 1176u32, 2172360u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2172364u32);
    emu.adi_no_count(13usize, 0usize, 1u32, 2172368u32);
    emu.sw_no_count(13usize, 12usize, 0u32, 2172372u32)?;
    emu.adi_no_count(10usize, 10usize, 1u32, 2172376u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2172376u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002125d8));
}
#[inline(always)]
pub fn block_0x002125d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 2usize, 680u32, 2172380u32)?;
    emu.sw_no_count(10usize, 2usize, 1336u32, 2172384u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2172392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002125e8));
    } else {
        emu.pc = 2172388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002125e4));
    }
}
#[inline(always)]
pub fn block_0x002125e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 12usize, 0u32, 2172392u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2172392u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002125e8));
}
#[inline(always)]
pub fn block_0x002125e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 41u32, 2172396u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2174744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2174744u32));
    } else {
        emu.pc = 2172400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002125f0));
    }
}
#[inline(always)]
pub fn block_0x002125f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 10usize, 2u32, 2172404u32);
    emu.adi_no_count(13usize, 2usize, 1176u32, 2172408u32);
    emu.adi_no_count(14usize, 2usize, 520u32, 2172412u32);
    emu.sbr_no_count(10usize, 0usize, 12usize, 2172416u32);
    emu.adi_no_count(15usize, 12usize, 4294967292u32, 2172420u32);
    emu.adr_no_count(12usize, 13usize, 15usize, 2172424u32);
    emu.adr_no_count(13usize, 14usize, 15usize, 2172428u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2172428u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021260c));
}
#[inline(always)]
pub fn block_0x0021260c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2172768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212760));
    } else {
        emu.pc = 2172432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212610));
    }
}
#[inline(always)]
pub fn block_0x00212610(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 13usize, 0u32, 2172436u32)?;
    emu.lw_no_count(15usize, 12usize, 0u32, 2172440u32)?;
    emu.adi_no_count(10usize, 10usize, 4u32, 2172444u32);
    emu.adi_no_count(12usize, 12usize, 4294967292u32, 2172448u32);
    emu.adi_no_count(13usize, 13usize, 4294967292u32, 2172452u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2172428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021260c));
    } else {
        emu.pc = 2172456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212628));
    }
}
#[inline(always)]
pub fn block_0x00212628(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 14usize, 15usize, 2172460u32);
    emu.sltru_no_count(12usize, 15usize, 14usize, 2172464u32);
    emu.sbr_no_count(10usize, 12usize, 10usize, 2172468u32);
    emu.lw_no_count(12usize, 2usize, 24u32, 2172472u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2172784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212770));
    } else {
        emu.pc = 2172476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021263c));
    }
}
#[inline(always)]
pub fn block_0x0021263c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 188u32, 2172480u32)?;
    emu.adi_no_count(12usize, 0usize, 41u32, 2172484u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2174744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2174744u32));
    } else {
        emu.pc = 2172488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212648));
    }
}
#[inline(always)]
pub fn block_0x00212648(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2172572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021269c));
    } else {
        emu.pc = 2172492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021264c));
    }
}
#[inline(always)]
pub fn block_0x0021264c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 0u32, 2172496u32);
    emu.sli_no_count(14usize, 10usize, 2u32, 2172500u32);
    emu.adi_no_count(16usize, 2usize, 28u32, 2172504u32);
    emu.adr_no_count(12usize, 16usize, 14usize, 2172508u32);
    emu.adi_no_count(15usize, 0usize, 10u32, 2172512u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2172512u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212660));
}
#[inline]
pub fn block_0x00212660(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(17usize, 16usize, 0u32, 2172516u32)?;
    emu.mulhu_no_count(5usize, 17usize, 15usize, 2172520u32);
    emu.mul_no_count(17usize, 17usize, 15usize, 2172524u32);
    emu.adr_no_count(13usize, 17usize, 13usize, 2172528u32);
    emu.sw_no_count(13usize, 16usize, 0u32, 2172532u32)?;
    emu.adi_no_count(16usize, 16usize, 4u32, 2172536u32);
    emu.sltru_no_count(13usize, 13usize, 17usize, 2172540u32);
    emu.adi_no_count(14usize, 14usize, 4294967292u32, 2172544u32);
    emu.adr_no_count(13usize, 5usize, 13usize, 2172548u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a != b {
        emu.pc = 2172512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212660));
    } else {
        emu.pc = 2172552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212688));
    }
}
#[inline(always)]
pub fn block_0x00212688(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2172572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021269c));
    } else {
        emu.pc = 2172556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021268c));
    }
}
#[inline(always)]
pub fn block_0x0021268c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 40u32, 2172560u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2175076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2175076u32));
    } else {
        emu.pc = 2172564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212694));
    }
}
#[inline(always)]
pub fn block_0x00212694(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(13usize, 12usize, 0u32, 2172568u32)?;
    emu.adi_no_count(10usize, 10usize, 1u32, 2172572u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2172572u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021269c));
}
#[inline(always)]
pub fn block_0x0021269c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 2usize, 352u32, 2172576u32)?;
    emu.adi_no_count(12usize, 0usize, 41u32, 2172580u32);
    emu.sw_no_count(10usize, 2usize, 188u32, 2172584u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2174840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2174840u32));
    } else {
        emu.pc = 2172588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002126ac));
    }
}
#[inline(always)]
pub fn block_0x002126ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2172672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212700));
    } else {
        emu.pc = 2172592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002126b0));
    }
}
#[inline(always)]
pub fn block_0x002126b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2172596u32);
    emu.sli_no_count(14usize, 13usize, 2u32, 2172600u32);
    emu.adi_no_count(16usize, 2usize, 192u32, 2172604u32);
    emu.adr_no_count(10usize, 16usize, 14usize, 2172608u32);
    emu.adi_no_count(15usize, 0usize, 10u32, 2172612u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2172612u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002126c4));
}
#[inline]
pub fn block_0x002126c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(17usize, 16usize, 0u32, 2172616u32)?;
    emu.mulhu_no_count(5usize, 17usize, 15usize, 2172620u32);
    emu.mul_no_count(17usize, 17usize, 15usize, 2172624u32);
    emu.adr_no_count(12usize, 17usize, 12usize, 2172628u32);
    emu.sw_no_count(12usize, 16usize, 0u32, 2172632u32)?;
    emu.adi_no_count(16usize, 16usize, 4u32, 2172636u32);
    emu.sltru_no_count(12usize, 12usize, 17usize, 2172640u32);
    emu.adi_no_count(14usize, 14usize, 4294967292u32, 2172644u32);
    emu.adr_no_count(12usize, 5usize, 12usize, 2172648u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a != b {
        emu.pc = 2172612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002126c4));
    } else {
        emu.pc = 2172652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002126ec));
    }
}
#[inline(always)]
pub fn block_0x002126ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2172672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212700));
    } else {
        emu.pc = 2172656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002126f0));
    }
}
#[inline(always)]
pub fn block_0x002126f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 40u32, 2172660u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2175076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2175076u32));
    } else {
        emu.pc = 2172664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002126f8));
    }
}
#[inline(always)]
pub fn block_0x002126f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(12usize, 10usize, 0u32, 2172668u32)?;
    emu.adi_no_count(13usize, 13usize, 1u32, 2172672u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2172672u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212700));
}
#[inline(always)]
pub fn block_0x00212700(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(13usize, 2usize, 352u32, 2172676u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2172760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212758));
    } else {
        emu.pc = 2172680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212708));
    }
}
#[inline(always)]
pub fn block_0x00212708(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2172684u32);
    emu.sli_no_count(13usize, 11usize, 2u32, 2172688u32);
    emu.adi_no_count(15usize, 2usize, 356u32, 2172692u32);
    emu.adr_no_count(10usize, 15usize, 13usize, 2172696u32);
    emu.adi_no_count(14usize, 0usize, 10u32, 2172700u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2172700u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021271c));
}
#[inline]
pub fn block_0x0021271c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 15usize, 0u32, 2172704u32)?;
    emu.mulhu_no_count(17usize, 16usize, 14usize, 2172708u32);
    emu.mul_no_count(16usize, 16usize, 14usize, 2172712u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2172716u32);
    emu.sw_no_count(12usize, 15usize, 0u32, 2172720u32)?;
    emu.adi_no_count(15usize, 15usize, 4u32, 2172724u32);
    emu.sltru_no_count(12usize, 12usize, 16usize, 2172728u32);
    emu.adi_no_count(13usize, 13usize, 4294967292u32, 2172732u32);
    emu.adr_no_count(12usize, 17usize, 12usize, 2172736u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2172700u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021271c));
    } else {
        emu.pc = 2172740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212744));
    }
}
#[inline(always)]
pub fn block_0x00212744(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2172760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212758));
    } else {
        emu.pc = 2172744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212748));
    }
}
#[inline(always)]
pub fn block_0x00212748(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 40u32, 2172748u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2175076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2175076u32));
    } else {
        emu.pc = 2172752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212750));
    }
}
#[inline(always)]
pub fn block_0x00212750(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(12usize, 10usize, 0u32, 2172756u32)?;
    emu.adi_no_count(11usize, 11usize, 1u32, 2172760u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2172760u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212758));
}
#[inline(always)]
pub fn block_0x00212758(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 2usize, 516u32, 2172764u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2172768u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2172788u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212774));
}
#[inline(always)]
pub fn block_0x00212760(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(10usize, 10usize, 1u32, 2172772u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2172776u32);
    emu.lw_no_count(12usize, 2usize, 24u32, 2172780u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2172476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021263c));
    } else {
        emu.pc = 2172784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212770));
    }
}
#[inline(always)]
pub fn block_0x00212770(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 25usize, 1u32, 2172788u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2172788u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212774));
}
#[inline(always)]
pub fn block_0x00212774(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 684u32, 2172792u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2172796u32);
    emu.adi_no_count(12usize, 0usize, 164u32, 2172800u32);
    emu.apc_no_count(1usize, 2172800u32, 4294905856u32, 2172804u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2172808u32;
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
pub fn block_0x00212788(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 684u32, 2172812u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2172816u32);
    emu.apc_no_count(1usize, 2172816u32, 4294942720u32, 2172820u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2172824u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(820u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00212798(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 848u32, 2172828u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2172832u32);
    emu.adi_no_count(12usize, 0usize, 164u32, 2172836u32);
    emu.apc_no_count(1usize, 2172836u32, 4294905856u32, 2172840u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2172844u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965476u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002127ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 848u32, 2172848u32);
    emu.adi_no_count(11usize, 0usize, 2u32, 2172852u32);
    emu.apc_no_count(1usize, 2172852u32, 4294942720u32, 2172856u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2172860u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(784u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002127bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1012u32, 2172864u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2172868u32);
    emu.adi_no_count(12usize, 0usize, 164u32, 2172872u32);
    emu.apc_no_count(1usize, 2172872u32, 4294905856u32, 2172876u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2172880u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965440u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002127d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1012u32, 2172884u32);
    emu.adi_no_count(11usize, 0usize, 3u32, 2172888u32);
    emu.apc_no_count(1usize, 2172888u32, 4294942720u32, 2172892u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2172896u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x002127e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 1172u32, 2172900u32)?;
    emu.lw_no_count(21usize, 2usize, 188u32, 2172904u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2172908u32);
    emu.sw_no_count(11usize, 2usize, 12u32, 2172912u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a < b {
        emu.pc = 2172920u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002127f8));
    } else {
        emu.pc = 2172916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002127f4));
    }
}
#[inline(always)]
pub fn block_0x002127f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 21usize, 0u32, 2172920u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2172920u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002127f8));
}
#[inline(always)]
pub fn block_0x002127f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 40u32, 2172924u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2174744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2174744u32));
    } else {
        emu.pc = 2172928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212800));
    }
}
#[inline]
pub fn block_0x00212800(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(25usize, 2usize, 8u32, 2172932u32)?;
    emu.sw_no_count(23usize, 2usize, 4u32, 2172936u32)?;
    emu.adi_no_count(23usize, 0usize, 0u32, 2172940u32);
    emu.lw_no_count(26usize, 2usize, 1008u32, 2172944u32)?;
    emu.lw_no_count(27usize, 2usize, 844u32, 2172948u32)?;
    emu.lw_no_count(18usize, 2usize, 680u32, 2172952u32)?;
    emu.adi_no_count(22usize, 2usize, 24u32, 2172956u32);
    emu.adi_no_count(7usize, 2usize, 516u32, 2172960u32);
    emu.adi_no_count(8usize, 0usize, 41u32, 2172964u32);
    emu.adi_no_count(9usize, 0usize, 10u32, 2172968u32);
    emu.sw_no_count(24usize, 2usize, 16u32, 2172972u32)?;
    emu.add_memory_rw_events(12usize);
    let return_addr = 2172976u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2172980u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212834));
}
#[inline(always)]
pub fn block_0x00212830(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a < b {
        emu.pc = 2174744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2174744u32));
    } else {
        emu.pc = 2172980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212834));
    }
}
#[inline(always)]
pub fn block_0x00212834(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 23usize, 0u32, 2172984u32);
    emu.sli_no_count(12usize, 10usize, 2u32, 2172988u32);
    emu.sbr_no_count(11usize, 0usize, 12usize, 2172992u32);
    emu.adi_no_count(13usize, 2usize, 1008u32, 2172996u32);
    emu.adr_no_count(13usize, 13usize, 12usize, 2173000u32);
    emu.adr_no_count(14usize, 22usize, 12usize, 2173004u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2173004u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021284c));
}
#[inline(always)]
pub fn block_0x0021284c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2173132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002128cc));
    } else {
        emu.pc = 2173008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212850));
    }
}
#[inline(always)]
pub fn block_0x00212850(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 14usize, 0u32, 2173012u32)?;
    emu.lw_no_count(16usize, 13usize, 0u32, 2173016u32)?;
    emu.adi_no_count(11usize, 11usize, 4u32, 2173020u32);
    emu.adi_no_count(13usize, 13usize, 4294967292u32, 2173024u32);
    emu.adi_no_count(14usize, 14usize, 4294967292u32, 2173028u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2173004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021284c));
    } else {
        emu.pc = 2173032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212868));
    }
}
#[inline(always)]
pub fn block_0x00212868(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2173136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002128d0));
    } else {
        emu.pc = 2173036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021286c));
    }
}
#[inline(always)]
pub fn block_0x0021286c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2173112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002128b8));
    } else {
        emu.pc = 2173040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212870));
    }
}
#[inline(always)]
pub fn block_0x00212870(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 2usize, 1012u32, 2173044u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2173048u32);
    emu.adr_no_count(12usize, 11usize, 12usize, 2173052u32);
    emu.adi_no_count(13usize, 2usize, 28u32, 2173056u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2173056u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212880));
}
#[inline]
pub fn block_0x00212880(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 11usize, 0u32, 2173060u32)?;
    emu.lw_no_count(16usize, 13usize, 0u32, 2173064u32)?;
    emu.ani_no_count(14usize, 14usize, 1u32, 2173068u32);
    emu.adi_no_count(11usize, 11usize, 4u32, 2173072u32);
    emu.xri_no_count(15usize, 15usize, 4294967295u32, 2173076u32);
    emu.adr_no_count(15usize, 16usize, 15usize, 2173080u32);
    emu.sltru_no_count(16usize, 15usize, 16usize, 2173084u32);
    emu.adr_no_count(14usize, 15usize, 14usize, 2173088u32);
    emu.sltru_no_count(15usize, 14usize, 15usize, 2173092u32);
    emu.sw_no_count(14usize, 13usize, 0u32, 2173096u32)?;
    emu.orr_no_count(14usize, 16usize, 15usize, 2173100u32);
    emu.adi_no_count(13usize, 13usize, 4u32, 2173104u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2173056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212880));
    } else {
        emu.pc = 2173108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002128b4));
    }
}
#[inline(always)]
pub fn block_0x002128b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2174812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2174812u32));
    } else {
        emu.pc = 2173112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002128b8));
    }
}
#[inline(always)]
pub fn block_0x002128b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 188u32, 2173116u32)?;
    emu.adi_no_count(11usize, 0usize, 8u32, 2173120u32);
    emu.adi_no_count(13usize, 26usize, 0u32, 2173124u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2173152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002128e0));
    } else {
        emu.pc = 2173128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002128c8));
    }
}
#[inline(always)]
pub fn block_0x002128c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2173132u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2173156u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002128e4));
}
#[inline(always)]
pub fn block_0x002128cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2173036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021286c));
    } else {
        emu.pc = 2173136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002128d0));
    }
}
#[inline(always)]
pub fn block_0x002128d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2173140u32);
    emu.adi_no_count(10usize, 21usize, 0u32, 2173144u32);
    emu.adi_no_count(13usize, 26usize, 0u32, 2173148u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a < b {
        emu.pc = 2173156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002128e4));
    } else {
        emu.pc = 2173152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002128e0));
    }
}
#[inline(always)]
pub fn block_0x002128e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 10usize, 0u32, 2173156u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2173156u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002128e4));
}
#[inline(always)]
pub fn block_0x002128e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2174840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2174840u32));
    } else {
        emu.pc = 2173160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002128e8));
    }
}
#[inline(always)]
pub fn block_0x002128e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 13usize, 2u32, 2173164u32);
    emu.sbr_no_count(14usize, 0usize, 12usize, 2173168u32);
    emu.adi_no_count(15usize, 2usize, 844u32, 2173172u32);
    emu.adr_no_count(15usize, 15usize, 12usize, 2173176u32);
    emu.adr_no_count(16usize, 22usize, 12usize, 2173180u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2173180u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002128fc));
}
#[inline(always)]
pub fn block_0x002128fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2173308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021297c));
    } else {
        emu.pc = 2173184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212900));
    }
}
#[inline(always)]
pub fn block_0x00212900(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(17usize, 16usize, 0u32, 2173188u32)?;
    emu.lw_no_count(5usize, 15usize, 0u32, 2173192u32)?;
    emu.adi_no_count(14usize, 14usize, 4u32, 2173196u32);
    emu.adi_no_count(15usize, 15usize, 4294967292u32, 2173200u32);
    emu.adi_no_count(16usize, 16usize, 4294967292u32, 2173204u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2173180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002128fc));
    } else {
        emu.pc = 2173208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212918));
    }
}
#[inline(always)]
pub fn block_0x00212918(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a < b {
        emu.pc = 2173312u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212980));
    } else {
        emu.pc = 2173212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021291c));
    }
}
#[inline(always)]
pub fn block_0x0021291c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2173288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212968));
    } else {
        emu.pc = 2173216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212920));
    }
}
#[inline(always)]
pub fn block_0x00212920(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 848u32, 2173220u32);
    emu.adi_no_count(15usize, 0usize, 1u32, 2173224u32);
    emu.adr_no_count(12usize, 10usize, 12usize, 2173228u32);
    emu.adi_no_count(14usize, 2usize, 28u32, 2173232u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2173232u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212930));
}
#[inline]
pub fn block_0x00212930(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 10usize, 0u32, 2173236u32)?;
    emu.lw_no_count(17usize, 14usize, 0u32, 2173240u32)?;
    emu.ani_no_count(15usize, 15usize, 1u32, 2173244u32);
    emu.adi_no_count(10usize, 10usize, 4u32, 2173248u32);
    emu.xri_no_count(16usize, 16usize, 4294967295u32, 2173252u32);
    emu.adr_no_count(16usize, 17usize, 16usize, 2173256u32);
    emu.sltru_no_count(17usize, 16usize, 17usize, 2173260u32);
    emu.adr_no_count(15usize, 16usize, 15usize, 2173264u32);
    emu.sltru_no_count(16usize, 15usize, 16usize, 2173268u32);
    emu.sw_no_count(15usize, 14usize, 0u32, 2173272u32)?;
    emu.orr_no_count(15usize, 17usize, 16usize, 2173276u32);
    emu.adi_no_count(14usize, 14usize, 4u32, 2173280u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2173232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212930));
    } else {
        emu.pc = 2173284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212964));
    }
}
#[inline(always)]
pub fn block_0x00212964(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2174812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2174812u32));
    } else {
        emu.pc = 2173288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212968));
    }
}
#[inline(always)]
pub fn block_0x00212968(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(13usize, 2usize, 188u32, 2173292u32)?;
    emu.ori_no_count(11usize, 11usize, 4u32, 2173296u32);
    emu.adi_no_count(10usize, 27usize, 0u32, 2173300u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2173328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212990));
    } else {
        emu.pc = 2173304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212978));
    }
}
#[inline(always)]
pub fn block_0x00212978(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2173308u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2173324u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021298c));
}
#[inline(always)]
pub fn block_0x0021297c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2173212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021291c));
    } else {
        emu.pc = 2173312u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212980));
    }
}
#[inline(always)]
pub fn block_0x00212980(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 10usize, 0u32, 2173316u32);
    emu.adi_no_count(10usize, 27usize, 0u32, 2173320u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2173328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212990));
    } else {
        emu.pc = 2173324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021298c));
    }
}
#[inline(always)]
pub fn block_0x0021298c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 13usize, 0u32, 2173328u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2173328u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212990));
}
#[inline(always)]
pub fn block_0x00212990(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2174744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2174744u32));
    } else {
        emu.pc = 2173332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2173332u32));
    }
}
