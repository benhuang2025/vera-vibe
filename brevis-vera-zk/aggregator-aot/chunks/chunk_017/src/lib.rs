pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2193808u32;
pub const PC_MAX: u32 = 2196136u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 110usize] = [
        block_0x00217990,
        block_0x002179a8,
        block_0x002179b4,
        block_0x002179e8,
        block_0x002179f4,
        block_0x002179fc,
        block_0x00217a1c,
        block_0x00217a24,
        block_0x00217a54,
        block_0x00217a90,
        block_0x00217ab8,
        block_0x00217abc,
        block_0x00217ae0,
        block_0x00217ae4,
        block_0x00217af4,
        block_0x00217b0c,
        block_0x00217b20,
        block_0x00217b28,
        block_0x00217b38,
        block_0x00217b40,
        block_0x00217b4c,
        block_0x00217b74,
        block_0x00217b84,
        block_0x00217b8c,
        block_0x00217b94,
        block_0x00217ba4,
        block_0x00217bb8,
        block_0x00217bbc,
        block_0x00217bd0,
        block_0x00217bdc,
        block_0x00217be4,
        block_0x00217c04,
        block_0x00217c0c,
        block_0x00217c3c,
        block_0x00217c78,
        block_0x00217c88,
        block_0x00217c98,
        block_0x00217cb8,
        block_0x00217cc4,
        block_0x00217ccc,
        block_0x00217cec,
        block_0x00217cf4,
        block_0x00217d24,
        block_0x00217d60,
        block_0x00217d74,
        block_0x00217d8c,
        block_0x00217db0,
        block_0x00217dcc,
        block_0x00217dd4,
        block_0x00217de8,
        block_0x00217dfc,
        block_0x00217e10,
        block_0x00217e34,
        block_0x00217e3c,
        block_0x00217e54,
        block_0x00217e64,
        block_0x00217e6c,
        block_0x00217e74,
        block_0x00217e84,
        block_0x00217eac,
        block_0x00217ee4,
        block_0x00217ee8,
        block_0x00217ef0,
        block_0x00217f00,
        block_0x00217f18,
        block_0x00217f28,
        block_0x00217f30,
        block_0x00217f40,
        block_0x00217f48,
        block_0x00217f50,
        block_0x00217f60,
        block_0x00217f74,
        block_0x00217f8c,
        block_0x00217fc4,
        block_0x00217ffc,
        block_0x00218000,
        block_0x00218008,
        block_0x00218018,
        block_0x00218030,
        block_0x00218040,
        block_0x00218048,
        block_0x00218058,
        block_0x00218060,
        block_0x00218068,
        block_0x00218078,
        block_0x0021808c,
        block_0x002180a4,
        block_0x002180dc,
        block_0x00218114,
        block_0x00218118,
        block_0x00218120,
        block_0x00218130,
        block_0x00218148,
        block_0x00218158,
        block_0x00218160,
        block_0x00218170,
        block_0x00218178,
        block_0x00218180,
        block_0x00218190,
        block_0x002181a4,
        block_0x002181bc,
        block_0x002181f4,
        block_0x002181f8,
        block_0x0021820c,
        block_0x00218214,
        block_0x0021821c,
        block_0x0021824c,
        block_0x00218260,
        block_0x00218288,
        block_0x002182a8,
    ];
    const IDX: [u16; 583usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 3u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 4u16, 0u16, 0u16, 5u16,
        0u16, 6u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 7u16, 0u16, 8u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 9u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 10u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 11u16, 12u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 13u16, 14u16, 0u16, 0u16, 0u16, 15u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 16u16, 0u16, 0u16, 0u16, 0u16, 17u16, 0u16, 18u16, 0u16,
        0u16, 0u16, 19u16, 0u16, 20u16, 0u16, 0u16, 21u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 22u16, 0u16, 0u16, 0u16, 23u16, 0u16, 24u16, 0u16, 25u16,
        0u16, 0u16, 0u16, 26u16, 0u16, 0u16, 0u16, 0u16, 27u16, 28u16, 0u16, 0u16, 0u16,
        0u16, 29u16, 0u16, 0u16, 30u16, 0u16, 31u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 32u16, 0u16, 33u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 34u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 35u16, 0u16, 0u16, 0u16, 36u16, 0u16, 0u16, 0u16, 37u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 38u16, 0u16, 0u16, 39u16, 0u16, 40u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 41u16, 0u16, 42u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 43u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 44u16, 0u16, 0u16,
        0u16, 0u16, 45u16, 0u16, 0u16, 0u16, 0u16, 0u16, 46u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 47u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 48u16, 0u16,
        49u16, 0u16, 0u16, 0u16, 0u16, 50u16, 0u16, 0u16, 0u16, 0u16, 51u16, 0u16, 0u16,
        0u16, 0u16, 52u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 53u16, 0u16,
        54u16, 0u16, 0u16, 0u16, 0u16, 0u16, 55u16, 0u16, 0u16, 0u16, 56u16, 0u16, 57u16,
        0u16, 58u16, 0u16, 0u16, 0u16, 59u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 60u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 61u16, 62u16, 0u16, 63u16, 0u16, 0u16, 0u16, 64u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 65u16, 0u16, 0u16, 0u16, 66u16, 0u16, 67u16, 0u16, 0u16, 0u16,
        68u16, 0u16, 69u16, 0u16, 70u16, 0u16, 0u16, 0u16, 71u16, 0u16, 0u16, 0u16, 0u16,
        72u16, 0u16, 0u16, 0u16, 0u16, 0u16, 73u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 74u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 75u16, 76u16, 0u16, 77u16, 0u16,
        0u16, 0u16, 78u16, 0u16, 0u16, 0u16, 0u16, 0u16, 79u16, 0u16, 0u16, 0u16, 80u16,
        0u16, 81u16, 0u16, 0u16, 0u16, 82u16, 0u16, 83u16, 0u16, 84u16, 0u16, 0u16, 0u16,
        85u16, 0u16, 0u16, 0u16, 0u16, 86u16, 0u16, 0u16, 0u16, 0u16, 0u16, 87u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 88u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        89u16, 90u16, 0u16, 91u16, 0u16, 0u16, 0u16, 92u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        93u16, 0u16, 0u16, 0u16, 94u16, 0u16, 95u16, 0u16, 0u16, 0u16, 96u16, 0u16,
        97u16, 0u16, 98u16, 0u16, 0u16, 0u16, 99u16, 0u16, 0u16, 0u16, 0u16, 100u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 101u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 102u16, 103u16, 0u16, 0u16, 0u16, 0u16,
        104u16, 0u16, 105u16, 0u16, 106u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 107u16, 0u16, 0u16, 0u16, 0u16, 108u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 109u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 110u16,
    ];
    if pc < 2193808u32 || pc > 2196136u32 {
        return None;
    }
    let word_offset = ((pc - 2193808u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x00217990(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 0u32, 2193812u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2193816u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2193820u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2193824u32);
    emu.apc_no_count(6usize, 2193824u32, 24576u32, 2193828u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2193832u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(192u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002179a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(6usize, 11usize, 12u32, 2193836u32)?;
    emu.adi_no_count(11usize, 12usize, 0u32, 2193840u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2193844u32;
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
pub fn block_0x002179b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2193848u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2193852u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2193856u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2193860u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2193864u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2193868u32)?;
    emu.sw_no_count(20usize, 2usize, 24u32, 2193872u32)?;
    emu.sw_no_count(21usize, 2usize, 20u32, 2193876u32)?;
    emu.sw_no_count(22usize, 2usize, 16u32, 2193880u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2193884u32);
    emu.adi_no_count(10usize, 0usize, 128u32, 2193888u32);
    emu.sw_no_count(0usize, 2usize, 12u32, 2193892u32)?;
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2193908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002179f4));
    } else {
        emu.pc = 2193896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002179e8));
    }
}
#[inline(always)]
pub fn block_0x002179e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 2usize, 12u32, 2193900u32);
    emu.adi_no_count(18usize, 0usize, 1u32, 2193904u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2193908u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194064u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217a90));
}
#[inline(always)]
pub fn block_0x002179f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 11u32, 2193912u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2193948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217a1c));
    } else {
        emu.pc = 2193916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002179fc));
    }
}
#[inline(always)]
pub fn block_0x002179fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 6u32, 2193920u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2193924u32);
    emu.ori_no_count(10usize, 10usize, 192u32, 2193928u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2193932u32);
    emu.sb_no_count(10usize, 2usize, 12u32, 2193936u32);
    emu.sb_no_count(11usize, 2usize, 13u32, 2193940u32);
    emu.adi_no_count(18usize, 0usize, 2u32, 2193944u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2193948u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194064u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217a90));
}
#[inline(always)]
pub fn block_0x00217a1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 16u32, 2193952u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2194004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217a54));
    } else {
        emu.pc = 2193956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217a24));
    }
}
#[inline]
pub fn block_0x00217a24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 12u32, 2193960u32);
    emu.sli_no_count(12usize, 11usize, 20u32, 2193964u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2193968u32);
    emu.ori_no_count(10usize, 10usize, 224u32, 2193972u32);
    emu.sri_no_count(12usize, 12usize, 26u32, 2193976u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2193980u32);
    emu.adi_no_count(12usize, 12usize, 128u32, 2193984u32);
    emu.sb_no_count(10usize, 2usize, 12u32, 2193988u32);
    emu.sb_no_count(12usize, 2usize, 13u32, 2193992u32);
    emu.sb_no_count(11usize, 2usize, 14u32, 2193996u32);
    emu.adi_no_count(18usize, 0usize, 3u32, 2194000u32);
    emu.add_memory_rw_events(12usize);
    let return_addr = 2194004u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194064u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217a90));
}
#[inline]
pub fn block_0x00217a54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 18u32, 2194008u32);
    emu.sli_no_count(12usize, 11usize, 14u32, 2194012u32);
    emu.sli_no_count(13usize, 11usize, 20u32, 2194016u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2194020u32);
    emu.ori_no_count(10usize, 10usize, 240u32, 2194024u32);
    emu.sri_no_count(12usize, 12usize, 26u32, 2194028u32);
    emu.sri_no_count(13usize, 13usize, 26u32, 2194032u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2194036u32);
    emu.adi_no_count(12usize, 12usize, 128u32, 2194040u32);
    emu.adi_no_count(13usize, 13usize, 128u32, 2194044u32);
    emu.sb_no_count(10usize, 2usize, 12u32, 2194048u32);
    emu.sb_no_count(12usize, 2usize, 13u32, 2194052u32);
    emu.sb_no_count(13usize, 2usize, 14u32, 2194056u32);
    emu.sb_no_count(11usize, 2usize, 15u32, 2194060u32);
    emu.adi_no_count(18usize, 0usize, 4u32, 2194064u32);
    emu.add_memory_rw_events(15usize);
    emu.pc = 2194064u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217a90));
}
#[inline]
pub fn block_0x00217a90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(19usize, 8usize, 8u32, 2194068u32)?;
    emu.lw_no_count(10usize, 19usize, 4u32, 2194072u32)?;
    emu.lw_no_count(21usize, 19usize, 8u32, 2194076u32)?;
    emu.lw_no_count(20usize, 19usize, 12u32, 2194080u32)?;
    emu.lw_no_count(11usize, 19usize, 0u32, 2194084u32)?;
    emu.sltru_no_count(12usize, 21usize, 10usize, 2194088u32);
    emu.sltiu_no_count(13usize, 20usize, 1u32, 2194092u32);
    emu.anr_no_count(14usize, 13usize, 12usize, 2194096u32);
    emu.adi_no_count(12usize, 21usize, 0u32, 2194100u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a != b {
        emu.pc = 2194108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217abc));
    } else {
        emu.pc = 2194104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217ab8));
    }
}
#[inline(always)]
pub fn block_0x00217ab8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2194108u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2194108u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217abc));
}
#[inline]
pub fn block_0x00217abc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2194112u32);
    emu.orr_no_count(13usize, 13usize, 21usize, 2194116u32);
    emu.sbr_no_count(13usize, 10usize, 13usize, 2194120u32);
    emu.sltru_no_count(10usize, 10usize, 13usize, 2194124u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2194128u32);
    emu.anr_no_count(22usize, 10usize, 13usize, 2194132u32);
    emu.adr_no_count(10usize, 11usize, 12usize, 2194136u32);
    emu.adi_no_count(9usize, 22usize, 0u32, 2194140u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a < b {
        emu.pc = 2194148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217ae4));
    } else {
        emu.pc = 2194144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217ae0));
    }
}
#[inline(always)]
pub fn block_0x00217ae0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 18usize, 0u32, 2194148u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2194148u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217ae4));
}
#[inline(always)]
pub fn block_0x00217ae4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 2usize, 12u32, 2194152u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2194156u32);
    emu.apc_no_count(1usize, 2194156u32, 4294909952u32, 2194160u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2194164u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966648u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00217af4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(9usize, 21usize, 9usize, 2194168u32);
    emu.sltru_no_count(10usize, 9usize, 21usize, 2194172u32);
    emu.adr_no_count(10usize, 20usize, 10usize, 2194176u32);
    emu.sw_no_count(9usize, 19usize, 8u32, 2194180u32)?;
    emu.sw_no_count(10usize, 19usize, 12u32, 2194184u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a >= b {
        emu.pc = 2194208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217b20));
    } else {
        emu.pc = 2194188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217b0c));
    }
}
#[inline(always)]
pub fn block_0x00217b0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2194192u32;
    emu.update_insn_clock();
    emu.lw_no_count(19usize, 10usize, 272u32, 2194196u32)?;
    emu.ani_no_count(12usize, 19usize, 255u32, 2194200u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2194204u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2194216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217b28));
    } else {
        emu.pc = 2194208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217b20));
    }
}
#[inline(always)]
pub fn block_0x00217b20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2194212u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2194216u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194252u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217b4c));
}
#[inline(always)]
pub fn block_0x00217b28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(20usize, 10usize, 276u32, 2194220u32)?;
    emu.lbu_no_count(10usize, 8usize, 0u32, 2194224u32);
    emu.lw_no_count(9usize, 8usize, 4u32, 2194228u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2194292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217b74));
    } else {
        emu.pc = 2194232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217b38));
    }
}
#[inline(always)]
pub fn block_0x00217b38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 3u32, 2194236u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2194292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217b74));
    } else {
        emu.pc = 2194240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217b40));
    }
}
#[inline(always)]
pub fn block_0x00217b40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(19usize, 8usize, 0u32, 2194244u32)?;
    emu.sw_no_count(20usize, 8usize, 4u32, 2194248u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2194252u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2194252u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217b4c));
}
#[inline]
pub fn block_0x00217b4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2194256u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2194260u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2194264u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2194268u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2194272u32)?;
    emu.lw_no_count(20usize, 2usize, 24u32, 2194276u32)?;
    emu.lw_no_count(21usize, 2usize, 20u32, 2194280u32)?;
    emu.lw_no_count(22usize, 2usize, 16u32, 2194284u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2194288u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2194292u32;
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
pub fn block_0x00217b74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(21usize, 9usize, 4u32, 2194296u32)?;
    emu.lw_no_count(11usize, 21usize, 0u32, 2194300u32)?;
    emu.lw_no_count(18usize, 9usize, 0u32, 2194304u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2194316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217b8c));
    } else {
        emu.pc = 2194308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217b84));
    }
}
#[inline(always)]
pub fn block_0x00217b84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2194312u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2194316u32;
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
pub fn block_0x00217b8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 21usize, 4u32, 2194320u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2194340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217ba4));
    } else {
        emu.pc = 2194324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217b94));
    }
}
#[inline(always)]
pub fn block_0x00217b94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 21usize, 8u32, 2194328u32)?;
    emu.adi_no_count(10usize, 18usize, 0u32, 2194332u32);
    emu.apc_no_count(1usize, 2194332u32, 4294905856u32, 2194336u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2194340u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966092u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00217ba4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 12u32, 2194344u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2194348u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2194352u32);
    emu.apc_no_count(1usize, 2194352u32, 4294905856u32, 2194356u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2194360u32;
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
#[inline(always)]
pub fn block_0x00217bb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2194364u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194240u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217b40));
}
#[inline(always)]
pub fn block_0x00217bbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2194368u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2194372u32)?;
    emu.adi_no_count(10usize, 0usize, 128u32, 2194376u32);
    emu.sw_no_count(0usize, 2usize, 8u32, 2194380u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2194396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217bdc));
    } else {
        emu.pc = 2194384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217bd0));
    }
}
#[inline(always)]
pub fn block_0x00217bd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 2usize, 8u32, 2194388u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2194392u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2194396u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194552u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217c78));
}
#[inline(always)]
pub fn block_0x00217bdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 11u32, 2194400u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2194436u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217c04));
    } else {
        emu.pc = 2194404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217be4));
    }
}
#[inline(always)]
pub fn block_0x00217be4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 6u32, 2194408u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2194412u32);
    emu.ori_no_count(10usize, 10usize, 192u32, 2194416u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2194420u32);
    emu.sb_no_count(10usize, 2usize, 8u32, 2194424u32);
    emu.sb_no_count(11usize, 2usize, 9u32, 2194428u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2194432u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2194436u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194552u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217c78));
}
#[inline(always)]
pub fn block_0x00217c04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 16u32, 2194440u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2194492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217c3c));
    } else {
        emu.pc = 2194444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217c0c));
    }
}
#[inline]
pub fn block_0x00217c0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 12u32, 2194448u32);
    emu.sli_no_count(12usize, 11usize, 20u32, 2194452u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2194456u32);
    emu.ori_no_count(10usize, 10usize, 224u32, 2194460u32);
    emu.sri_no_count(12usize, 12usize, 26u32, 2194464u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2194468u32);
    emu.adi_no_count(12usize, 12usize, 128u32, 2194472u32);
    emu.sb_no_count(10usize, 2usize, 8u32, 2194476u32);
    emu.sb_no_count(12usize, 2usize, 9u32, 2194480u32);
    emu.sb_no_count(11usize, 2usize, 10u32, 2194484u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2194488u32);
    emu.add_memory_rw_events(12usize);
    let return_addr = 2194492u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194552u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217c78));
}
#[inline]
pub fn block_0x00217c3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 18u32, 2194496u32);
    emu.sli_no_count(12usize, 11usize, 14u32, 2194500u32);
    emu.sli_no_count(13usize, 11usize, 20u32, 2194504u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2194508u32);
    emu.ori_no_count(10usize, 10usize, 240u32, 2194512u32);
    emu.sri_no_count(12usize, 12usize, 26u32, 2194516u32);
    emu.sri_no_count(13usize, 13usize, 26u32, 2194520u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2194524u32);
    emu.adi_no_count(12usize, 12usize, 128u32, 2194528u32);
    emu.adi_no_count(13usize, 13usize, 128u32, 2194532u32);
    emu.sb_no_count(10usize, 2usize, 8u32, 2194536u32);
    emu.sb_no_count(12usize, 2usize, 9u32, 2194540u32);
    emu.sb_no_count(13usize, 2usize, 10u32, 2194544u32);
    emu.sb_no_count(11usize, 2usize, 11u32, 2194548u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2194552u32);
    emu.add_memory_rw_events(15usize);
    emu.pc = 2194552u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217c78));
}
#[inline(always)]
pub fn block_0x00217c78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2194556u32);
    emu.adi_no_count(11usize, 2usize, 8u32, 2194560u32);
    emu.apc_no_count(1usize, 2194560u32, 4294909952u32, 2194564u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2194568u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965988u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00217c88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2194572u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2194576u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2194580u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2194584u32;
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
pub fn block_0x00217c98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2194588u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2194592u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2194596u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2194600u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2194604u32)?;
    emu.adi_no_count(12usize, 0usize, 128u32, 2194608u32);
    emu.sw_no_count(0usize, 2usize, 12u32, 2194612u32)?;
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2194628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217cc4));
    } else {
        emu.pc = 2194616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217cb8));
    }
}
#[inline(always)]
pub fn block_0x00217cb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 2usize, 12u32, 2194620u32);
    emu.adi_no_count(8usize, 0usize, 1u32, 2194624u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2194628u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194784u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217d60));
}
#[inline(always)]
pub fn block_0x00217cc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 11usize, 11u32, 2194632u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2194668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217cec));
    } else {
        emu.pc = 2194636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217ccc));
    }
}
#[inline(always)]
pub fn block_0x00217ccc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 11usize, 6u32, 2194640u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2194644u32);
    emu.ori_no_count(12usize, 12usize, 192u32, 2194648u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2194652u32);
    emu.sb_no_count(12usize, 2usize, 12u32, 2194656u32);
    emu.sb_no_count(11usize, 2usize, 13u32, 2194660u32);
    emu.adi_no_count(8usize, 0usize, 2u32, 2194664u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2194668u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194784u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217d60));
}
#[inline(always)]
pub fn block_0x00217cec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 11usize, 16u32, 2194672u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2194724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217d24));
    } else {
        emu.pc = 2194676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217cf4));
    }
}
#[inline]
pub fn block_0x00217cf4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 11usize, 12u32, 2194680u32);
    emu.sli_no_count(13usize, 11usize, 20u32, 2194684u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2194688u32);
    emu.ori_no_count(12usize, 12usize, 224u32, 2194692u32);
    emu.sri_no_count(13usize, 13usize, 26u32, 2194696u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2194700u32);
    emu.adi_no_count(13usize, 13usize, 128u32, 2194704u32);
    emu.sb_no_count(12usize, 2usize, 12u32, 2194708u32);
    emu.sb_no_count(13usize, 2usize, 13u32, 2194712u32);
    emu.sb_no_count(11usize, 2usize, 14u32, 2194716u32);
    emu.adi_no_count(8usize, 0usize, 3u32, 2194720u32);
    emu.add_memory_rw_events(12usize);
    let return_addr = 2194724u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194784u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217d60));
}
#[inline]
pub fn block_0x00217d24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 11usize, 18u32, 2194728u32);
    emu.sli_no_count(13usize, 11usize, 14u32, 2194732u32);
    emu.sli_no_count(14usize, 11usize, 20u32, 2194736u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2194740u32);
    emu.ori_no_count(12usize, 12usize, 240u32, 2194744u32);
    emu.sri_no_count(13usize, 13usize, 26u32, 2194748u32);
    emu.sri_no_count(14usize, 14usize, 26u32, 2194752u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2194756u32);
    emu.adi_no_count(13usize, 13usize, 128u32, 2194760u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2194764u32);
    emu.sb_no_count(12usize, 2usize, 12u32, 2194768u32);
    emu.sb_no_count(13usize, 2usize, 13u32, 2194772u32);
    emu.sb_no_count(14usize, 2usize, 14u32, 2194776u32);
    emu.sb_no_count(11usize, 2usize, 15u32, 2194780u32);
    emu.adi_no_count(8usize, 0usize, 4u32, 2194784u32);
    emu.add_memory_rw_events(15usize);
    emu.pc = 2194784u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217d60));
}
#[inline(always)]
pub fn block_0x00217d60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(9usize, 10usize, 8u32, 2194788u32)?;
    emu.lw_no_count(10usize, 9usize, 0u32, 2194792u32)?;
    emu.lw_no_count(18usize, 9usize, 8u32, 2194796u32)?;
    emu.sbr_no_count(10usize, 10usize, 18usize, 2194800u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2194864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217db0));
    } else {
        emu.pc = 2194804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217d74));
    }
}
#[inline(always)]
pub fn block_0x00217d74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 4u32, 2194808u32)?;
    emu.adr_no_count(10usize, 10usize, 18usize, 2194812u32);
    emu.adi_no_count(11usize, 2usize, 12u32, 2194816u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2194820u32);
    emu.apc_no_count(1usize, 2194820u32, 4294909952u32, 2194824u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2194828u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965984u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00217d8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 18usize, 8usize, 2194832u32);
    emu.sw_no_count(8usize, 9usize, 8u32, 2194836u32)?;
    emu.adi_no_count(10usize, 0usize, 0u32, 2194840u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2194844u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2194848u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2194852u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2194856u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2194860u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2194864u32;
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
pub fn block_0x00217db0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2194868u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2194872u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2194876u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2194880u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2194884u32);
    emu.apc_no_count(1usize, 2194884u32, 0u32, 2194888u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2194892u32;
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
pub fn block_0x00217dcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 9usize, 8u32, 2194896u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2194900u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194804u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217d74));
}
#[inline(always)]
pub fn block_0x00217dd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 0u32, 2194904u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2194908u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 156u32, 2194912u32);
    emu.apc_no_count(6usize, 2194912u32, 20480u32, 2194916u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2194920u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x00217de8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 0u32, 2194924u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2194928u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 204u32, 2194932u32);
    emu.apc_no_count(6usize, 2194932u32, 20480u32, 2194936u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2194940u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(872u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00217dfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 0u32, 2194944u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2194948u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 180u32, 2194952u32);
    emu.apc_no_count(6usize, 2194952u32, 20480u32, 2194956u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2194960u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(852u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00217e10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2194964u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2194968u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2194972u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2194976u32)?;
    emu.sw_no_count(18usize, 2usize, 0u32, 2194980u32)?;
    emu.lbu_no_count(11usize, 10usize, 0u32, 2194984u32);
    emu.lw_no_count(8usize, 10usize, 4u32, 2194988u32)?;
    emu.adi_no_count(10usize, 0usize, 4u32, 2194992u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2195028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217e54));
    } else {
        emu.pc = 2194996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217e34));
    }
}
#[inline(always)]
pub fn block_0x00217e34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 3u32, 2195000u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2195028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217e54));
    } else {
        emu.pc = 2195004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217e3c));
    }
}
#[inline(always)]
pub fn block_0x00217e3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2195008u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2195012u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2195016u32)?;
    emu.lw_no_count(18usize, 2usize, 0u32, 2195020u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2195024u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195028u32;
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
pub fn block_0x00217e54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 8usize, 4u32, 2195032u32)?;
    emu.lw_no_count(11usize, 18usize, 0u32, 2195036u32)?;
    emu.lw_no_count(9usize, 8usize, 0u32, 2195040u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2195052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217e6c));
    } else {
        emu.pc = 2195044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217e64));
    }
}
#[inline(always)]
pub fn block_0x00217e64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2195048u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2195052u32;
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
pub fn block_0x00217e6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 18usize, 4u32, 2195056u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2195076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217e84));
    } else {
        emu.pc = 2195060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217e74));
    }
}
#[inline(always)]
pub fn block_0x00217e74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 8u32, 2195064u32)?;
    emu.adi_no_count(10usize, 9usize, 0u32, 2195068u32);
    emu.apc_no_count(1usize, 2195068u32, 4294905856u32, 2195072u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195076u32;
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
#[inline]
pub fn block_0x00217e84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 12u32, 2195080u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2195084u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2195088u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2195092u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2195096u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2195100u32)?;
    emu.lw_no_count(18usize, 2usize, 0u32, 2195104u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2195108u32);
    emu.apc_no_count(6usize, 2195108u32, 4294905856u32, 2195112u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2195116u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x00217eac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2195120u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2195124u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2195128u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2195132u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2195136u32)?;
    emu.adi_no_count(9usize, 10usize, 0u32, 2195140u32);
    emu.adi_no_count(18usize, 0usize, 4u32, 2195144u32);
    emu.sb_no_count(18usize, 2usize, 8u32, 2195148u32);
    emu.sw_no_count(11usize, 2usize, 16u32, 2195152u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2195156u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 180u32, 2195160u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2195164u32);
    emu.apc_no_count(1usize, 2195164u32, 20480u32, 2195168u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195172u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x00217ee4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2195224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217f18));
    } else {
        emu.pc = 2195176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217ee8));
    }
}
#[inline(always)]
pub fn block_0x00217ee8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 8u32, 2195180u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2195340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217f8c));
    } else {
        emu.pc = 2195184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217ef0));
    }
}
#[inline(always)]
pub fn block_0x00217ef0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2195188u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2195192u32)?;
    emu.sw_no_count(10usize, 9usize, 0u32, 2195196u32)?;
    emu.sw_no_count(11usize, 9usize, 4u32, 2195200u32)?;
    emu.add_memory_rw_events(4usize);
    emu.pc = 2195200u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217f00));
}
#[inline(always)]
pub fn block_0x00217f00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2195204u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2195208u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2195212u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2195216u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2195220u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195224u32;
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
pub fn block_0x00217f18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 8u32, 2195228u32);
    emu.lw_no_count(8usize, 2usize, 12u32, 2195232u32)?;
    emu.sb_no_count(18usize, 9usize, 0u32, 2195236u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a < b {
        emu.pc = 2195248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217f30));
    } else {
        emu.pc = 2195240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217f28));
    }
}
#[inline(always)]
pub fn block_0x00217f28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 3u32, 2195244u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2195200u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217f00));
    } else {
        emu.pc = 2195248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217f30));
    }
}
#[inline(always)]
pub fn block_0x00217f30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 8usize, 4u32, 2195252u32)?;
    emu.lw_no_count(11usize, 18usize, 0u32, 2195256u32)?;
    emu.lw_no_count(9usize, 8usize, 0u32, 2195260u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2195272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217f48));
    } else {
        emu.pc = 2195264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217f40));
    }
}
#[inline(always)]
pub fn block_0x00217f40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2195268u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2195272u32;
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
pub fn block_0x00217f48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 18usize, 4u32, 2195276u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2195296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217f60));
    } else {
        emu.pc = 2195280u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217f50));
    }
}
#[inline(always)]
pub fn block_0x00217f50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 8u32, 2195284u32)?;
    emu.adi_no_count(10usize, 9usize, 0u32, 2195288u32);
    emu.apc_no_count(1usize, 2195288u32, 4294901760u32, 2195292u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195296u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1936u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00217f60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 12u32, 2195300u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2195304u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2195308u32);
    emu.apc_no_count(1usize, 2195308u32, 4294901760u32, 2195312u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195316u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1916u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00217f74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2195320u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2195324u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2195328u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2195332u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2195336u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195340u32;
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
pub fn block_0x00217f8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2195344u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 368u32, 2195348u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2195352u32);
    emu.sw_no_count(0usize, 2usize, 40u32, 2195356u32)?;
    emu.adi_no_count(12usize, 0usize, 4u32, 2195360u32);
    emu.sw_no_count(10usize, 2usize, 24u32, 2195364u32)?;
    emu.sw_no_count(11usize, 2usize, 28u32, 2195368u32)?;
    emu.sw_no_count(12usize, 2usize, 32u32, 2195372u32)?;
    emu.sw_no_count(0usize, 2usize, 36u32, 2195376u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2195380u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 376u32, 2195384u32);
    emu.adi_no_count(10usize, 2usize, 24u32, 2195388u32);
    emu.apc_no_count(1usize, 2195388u32, 16384u32, 2195392u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195396u32;
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
#[inline]
pub fn block_0x00217fc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2195400u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2195404u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2195408u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2195412u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2195416u32)?;
    emu.adi_no_count(9usize, 10usize, 0u32, 2195420u32);
    emu.adi_no_count(18usize, 0usize, 4u32, 2195424u32);
    emu.sb_no_count(18usize, 2usize, 8u32, 2195428u32);
    emu.sw_no_count(11usize, 2usize, 16u32, 2195432u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2195436u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 156u32, 2195440u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2195444u32);
    emu.apc_no_count(1usize, 2195444u32, 20480u32, 2195448u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195452u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(360u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00217ffc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2195504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218030));
    } else {
        emu.pc = 2195456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218000));
    }
}
#[inline(always)]
pub fn block_0x00218000(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 8u32, 2195460u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2195620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002180a4));
    } else {
        emu.pc = 2195464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218008));
    }
}
#[inline(always)]
pub fn block_0x00218008(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2195468u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2195472u32)?;
    emu.sw_no_count(10usize, 9usize, 0u32, 2195476u32)?;
    emu.sw_no_count(11usize, 9usize, 4u32, 2195480u32)?;
    emu.add_memory_rw_events(4usize);
    emu.pc = 2195480u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218018));
}
#[inline(always)]
pub fn block_0x00218018(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2195484u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2195488u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2195492u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2195496u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2195500u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195504u32;
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
pub fn block_0x00218030(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 8u32, 2195508u32);
    emu.lw_no_count(8usize, 2usize, 12u32, 2195512u32)?;
    emu.sb_no_count(18usize, 9usize, 0u32, 2195516u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a < b {
        emu.pc = 2195528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218048));
    } else {
        emu.pc = 2195520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218040));
    }
}
#[inline(always)]
pub fn block_0x00218040(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 3u32, 2195524u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2195480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218018));
    } else {
        emu.pc = 2195528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218048));
    }
}
#[inline(always)]
pub fn block_0x00218048(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 8usize, 4u32, 2195532u32)?;
    emu.lw_no_count(11usize, 18usize, 0u32, 2195536u32)?;
    emu.lw_no_count(9usize, 8usize, 0u32, 2195540u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2195552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218060));
    } else {
        emu.pc = 2195544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218058));
    }
}
#[inline(always)]
pub fn block_0x00218058(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2195548u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2195552u32;
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
pub fn block_0x00218060(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 18usize, 4u32, 2195556u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2195576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218078));
    } else {
        emu.pc = 2195560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218068));
    }
}
#[inline(always)]
pub fn block_0x00218068(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 8u32, 2195564u32)?;
    emu.adi_no_count(10usize, 9usize, 0u32, 2195568u32);
    emu.apc_no_count(1usize, 2195568u32, 4294901760u32, 2195572u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195576u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1656u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218078(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 12u32, 2195580u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2195584u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2195588u32);
    emu.apc_no_count(1usize, 2195588u32, 4294901760u32, 2195592u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195596u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1636u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021808c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2195600u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2195604u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2195608u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2195612u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2195616u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195620u32;
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
pub fn block_0x002180a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2195624u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 368u32, 2195628u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2195632u32);
    emu.sw_no_count(0usize, 2usize, 40u32, 2195636u32)?;
    emu.adi_no_count(12usize, 0usize, 4u32, 2195640u32);
    emu.sw_no_count(10usize, 2usize, 24u32, 2195644u32)?;
    emu.sw_no_count(11usize, 2usize, 28u32, 2195648u32)?;
    emu.sw_no_count(12usize, 2usize, 32u32, 2195652u32)?;
    emu.sw_no_count(0usize, 2usize, 36u32, 2195656u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2195660u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 376u32, 2195664u32);
    emu.adi_no_count(10usize, 2usize, 24u32, 2195668u32);
    emu.apc_no_count(1usize, 2195668u32, 16384u32, 2195672u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195676u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965896u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002180dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2195680u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2195684u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2195688u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2195692u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2195696u32)?;
    emu.adi_no_count(9usize, 10usize, 0u32, 2195700u32);
    emu.adi_no_count(18usize, 0usize, 4u32, 2195704u32);
    emu.sb_no_count(18usize, 2usize, 8u32, 2195708u32);
    emu.sw_no_count(11usize, 2usize, 16u32, 2195712u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2195716u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 204u32, 2195720u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2195724u32);
    emu.apc_no_count(1usize, 2195724u32, 20480u32, 2195728u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195732u32;
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
pub fn block_0x00218114(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2195784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218148));
    } else {
        emu.pc = 2195736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218118));
    }
}
#[inline(always)]
pub fn block_0x00218118(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 8u32, 2195740u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2195900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002181bc));
    } else {
        emu.pc = 2195744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218120));
    }
}
#[inline(always)]
pub fn block_0x00218120(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2195748u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2195752u32)?;
    emu.sw_no_count(10usize, 9usize, 0u32, 2195756u32)?;
    emu.sw_no_count(11usize, 9usize, 4u32, 2195760u32)?;
    emu.add_memory_rw_events(4usize);
    emu.pc = 2195760u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218130));
}
#[inline(always)]
pub fn block_0x00218130(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2195764u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2195768u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2195772u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2195776u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2195780u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195784u32;
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
pub fn block_0x00218148(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 8u32, 2195788u32);
    emu.lw_no_count(8usize, 2usize, 12u32, 2195792u32)?;
    emu.sb_no_count(18usize, 9usize, 0u32, 2195796u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a < b {
        emu.pc = 2195808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218160));
    } else {
        emu.pc = 2195800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218158));
    }
}
#[inline(always)]
pub fn block_0x00218158(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 3u32, 2195804u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2195760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218130));
    } else {
        emu.pc = 2195808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218160));
    }
}
#[inline(always)]
pub fn block_0x00218160(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 8usize, 4u32, 2195812u32)?;
    emu.lw_no_count(11usize, 18usize, 0u32, 2195816u32)?;
    emu.lw_no_count(9usize, 8usize, 0u32, 2195820u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2195832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218178));
    } else {
        emu.pc = 2195824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218170));
    }
}
#[inline(always)]
pub fn block_0x00218170(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2195828u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2195832u32;
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
pub fn block_0x00218178(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 18usize, 4u32, 2195836u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2195856u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218190));
    } else {
        emu.pc = 2195840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218180));
    }
}
#[inline(always)]
pub fn block_0x00218180(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 8u32, 2195844u32)?;
    emu.adi_no_count(10usize, 9usize, 0u32, 2195848u32);
    emu.apc_no_count(1usize, 2195848u32, 4294901760u32, 2195852u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195856u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1376u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218190(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 12u32, 2195860u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2195864u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2195868u32);
    emu.apc_no_count(1usize, 2195868u32, 4294901760u32, 2195872u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195876u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1356u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002181a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2195880u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2195884u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2195888u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2195892u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2195896u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195900u32;
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
pub fn block_0x002181bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2195904u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 368u32, 2195908u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2195912u32);
    emu.sw_no_count(0usize, 2usize, 40u32, 2195916u32)?;
    emu.adi_no_count(12usize, 0usize, 4u32, 2195920u32);
    emu.sw_no_count(10usize, 2usize, 24u32, 2195924u32)?;
    emu.sw_no_count(11usize, 2usize, 28u32, 2195928u32)?;
    emu.sw_no_count(12usize, 2usize, 32u32, 2195932u32)?;
    emu.sw_no_count(0usize, 2usize, 36u32, 2195936u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2195940u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 376u32, 2195944u32);
    emu.adi_no_count(10usize, 2usize, 24u32, 2195948u32);
    emu.apc_no_count(1usize, 2195948u32, 16384u32, 2195952u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195956u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965616u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002181f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2195988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218214));
    } else {
        emu.pc = 2195960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002181f8));
    }
}
#[inline(always)]
pub fn block_0x002181f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2195964u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2195968u32)?;
    emu.adi_no_count(10usize, 0usize, 2u32, 2195972u32);
    emu.apc_no_count(1usize, 2195972u32, 4294905856u32, 2195976u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195980u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1376u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021820c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2195984u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2195988u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2195988u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218214));
}
#[inline(always)]
pub fn block_0x00218214(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2195992u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195996u32;
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
pub fn block_0x0021821c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2196000u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2196004u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2196008u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2196012u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2196016u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2196020u32)?;
    emu.adi_no_count(8usize, 12usize, 0u32, 2196024u32);
    emu.lw_no_count(9usize, 10usize, 8u32, 2196028u32)?;
    emu.lw_no_count(10usize, 9usize, 0u32, 2196032u32)?;
    emu.lw_no_count(18usize, 9usize, 8u32, 2196036u32)?;
    emu.sbr_no_count(10usize, 10usize, 18usize, 2196040u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2196104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218288));
    } else {
        emu.pc = 2196044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021824c));
    }
}
#[inline(always)]
pub fn block_0x0021824c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 4u32, 2196048u32)?;
    emu.adr_no_count(10usize, 10usize, 18usize, 2196052u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2196056u32);
    emu.apc_no_count(1usize, 2196056u32, 4294905856u32, 2196060u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2196064u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1548u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00218260(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 18usize, 8usize, 2196068u32);
    emu.sw_no_count(8usize, 9usize, 8u32, 2196072u32)?;
    emu.adi_no_count(10usize, 0usize, 0u32, 2196076u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2196080u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2196084u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2196088u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2196092u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2196096u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2196100u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2196104u32;
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
pub fn block_0x00218288(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2196108u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2196112u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2196116u32);
    emu.adi_no_count(19usize, 11usize, 0u32, 2196120u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2196124u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2196128u32);
    emu.apc_no_count(1usize, 2196128u32, 0u32, 2196132u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2196136u32;
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
pub fn block_0x002182a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 19usize, 0u32, 2196140u32);
    emu.lw_no_count(18usize, 9usize, 8u32, 2196144u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2196148u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2196044u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021824c));
}
