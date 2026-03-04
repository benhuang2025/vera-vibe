pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2120792u32;
pub const PC_MAX: u32 = 2124804u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 115usize] = [
        block_0x00205c58,
        block_0x00205c64,
        block_0x00205c90,
        block_0x00205cb8,
        block_0x00205cc4,
        block_0x00205ce0,
        block_0x00205d14,
        block_0x00205d20,
        block_0x00205d3c,
        block_0x00205d58,
        block_0x00205d74,
        block_0x00205d90,
        block_0x00205dac,
        block_0x00205dc8,
        block_0x00205de4,
        block_0x00205df4,
        block_0x00205e24,
        block_0x00205e30,
        block_0x00205e44,
        block_0x00205e60,
        block_0x00205e68,
        block_0x00205e70,
        block_0x00205e8c,
        block_0x00205e94,
        block_0x00205e98,
        block_0x00205eb4,
        block_0x00205ee8,
        block_0x00205ef4,
        block_0x00205f18,
        block_0x00205f24,
        block_0x00205f44,
        block_0x00205f50,
        block_0x00205f58,
        block_0x00205f60,
        block_0x00205f68,
        block_0x00205f70,
        block_0x00205fa0,
        block_0x00205fbc,
        block_0x00205fd8,
        block_0x00205ff4,
        block_0x0020601c,
        block_0x00206028,
        block_0x00206044,
        block_0x00206068,
        block_0x00206074,
        block_0x00206090,
        block_0x002060ac,
        block_0x0020625c,
        block_0x00206278,
        block_0x0020628c,
        block_0x002062a4,
        block_0x002062bc,
        block_0x002062d0,
        block_0x002062dc,
        block_0x002062f4,
        block_0x00206308,
        block_0x0020631c,
        block_0x00206334,
        block_0x00206340,
        block_0x0020636c,
        block_0x00206374,
        block_0x002064f4,
        block_0x00206514,
        block_0x00206528,
        block_0x00206540,
        block_0x00206558,
        block_0x0020656c,
        block_0x00206578,
        block_0x00206588,
        block_0x002065a0,
        block_0x002065b4,
        block_0x002065c8,
        block_0x002065e0,
        block_0x002065ec,
        block_0x0020661c,
        block_0x00206628,
        block_0x00206660,
        block_0x00206668,
        block_0x00206698,
        block_0x002066a0,
        block_0x002066a4,
        block_0x002066b0,
        block_0x002066ec,
        block_0x00206748,
        block_0x0020677c,
        block_0x002067ec,
        block_0x002067fc,
        block_0x0020680c,
        block_0x0020687c,
        block_0x0020688c,
        block_0x00206898,
        block_0x00206928,
        block_0x00206938,
        block_0x0020698c,
        block_0x002069a0,
        block_0x002069b8,
        block_0x002069cc,
        block_0x002069e0,
        block_0x002069f4,
        block_0x00206a08,
        block_0x00206a18,
        block_0x00206a50,
        block_0x00206aa4,
        block_0x00206af8,
        block_0x00206b14,
        block_0x00206b30,
        block_0x00206b48,
        block_0x00206b60,
        block_0x00206b8c,
        block_0x00206b9c,
        block_0x00206bac,
        block_0x00206bb4,
        block_0x00206bbc,
        block_0x00206bdc,
        block_0x00206c04,
    ];
    const IDX: [u16; 1004usize] = [
        1u16, 0u16, 0u16, 2u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 3u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 4u16, 0u16,
        0u16, 5u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 6u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 7u16, 0u16, 0u16, 8u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 9u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 10u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 11u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        12u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 13u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 14u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 15u16, 0u16, 0u16, 0u16, 16u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 17u16, 0u16,
        0u16, 18u16, 0u16, 0u16, 0u16, 0u16, 19u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        20u16, 0u16, 21u16, 0u16, 22u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 23u16, 0u16,
        24u16, 25u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 26u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 27u16, 0u16, 0u16, 28u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 29u16, 0u16, 0u16, 30u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 31u16, 0u16, 0u16, 32u16, 0u16, 33u16, 0u16, 34u16,
        0u16, 35u16, 0u16, 36u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 37u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 38u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 39u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 40u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 41u16, 0u16, 0u16, 42u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 43u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        44u16, 0u16, 0u16, 45u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 46u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 47u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 48u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 49u16, 0u16, 0u16, 0u16, 0u16, 50u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        51u16, 0u16, 0u16, 0u16, 0u16, 0u16, 52u16, 0u16, 0u16, 0u16, 0u16, 53u16, 0u16,
        0u16, 54u16, 0u16, 0u16, 0u16, 0u16, 0u16, 55u16, 0u16, 0u16, 0u16, 0u16, 56u16,
        0u16, 0u16, 0u16, 0u16, 57u16, 0u16, 0u16, 0u16, 0u16, 0u16, 58u16, 0u16, 0u16,
        59u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 60u16, 0u16,
        61u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 62u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        63u16, 0u16, 0u16, 0u16, 0u16, 64u16, 0u16, 0u16, 0u16, 0u16, 0u16, 65u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 66u16, 0u16, 0u16, 0u16, 0u16, 67u16, 0u16, 0u16, 68u16,
        0u16, 0u16, 0u16, 69u16, 0u16, 0u16, 0u16, 0u16, 0u16, 70u16, 0u16, 0u16, 0u16,
        0u16, 71u16, 0u16, 0u16, 0u16, 0u16, 72u16, 0u16, 0u16, 0u16, 0u16, 0u16, 73u16,
        0u16, 0u16, 74u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 75u16, 0u16, 0u16, 76u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 77u16, 0u16, 78u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 79u16, 0u16, 80u16, 81u16, 0u16, 0u16, 82u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 83u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 84u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 85u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        86u16, 0u16, 0u16, 0u16, 87u16, 0u16, 0u16, 0u16, 88u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 89u16, 0u16, 0u16,
        0u16, 90u16, 0u16, 0u16, 91u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 92u16, 0u16, 0u16, 0u16, 93u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        94u16, 0u16, 0u16, 0u16, 0u16, 95u16, 0u16, 0u16, 0u16, 0u16, 0u16, 96u16, 0u16,
        0u16, 0u16, 0u16, 97u16, 0u16, 0u16, 0u16, 0u16, 98u16, 0u16, 0u16, 0u16, 0u16,
        99u16, 0u16, 0u16, 0u16, 0u16, 100u16, 0u16, 0u16, 0u16, 101u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 102u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 103u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 104u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 105u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 106u16, 0u16, 0u16, 0u16, 0u16, 0u16, 107u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 108u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 109u16,
        0u16, 0u16, 0u16, 110u16, 0u16, 0u16, 0u16, 111u16, 0u16, 112u16, 0u16, 113u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 114u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 115u16,
    ];
    if pc < 2120792u32 || pc > 2124804u32 {
        return None;
    }
    let word_offset = ((pc - 2120792u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x00205c58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2120796u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2120800u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120804u32;
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
pub fn block_0x00205c64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2120808u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2120812u32)?;
    emu.lw_no_count(12usize, 10usize, 0u32, 2120816u32)?;
    emu.lbu_no_count(10usize, 12usize, 0u32, 2120820u32);
    emu.sli_no_count(10usize, 10usize, 2u32, 2120824u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2120828u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 232u32, 2120832u32);
    emu.adr_no_count(10usize, 10usize, 13usize, 2120836u32);
    emu.lw_no_count(13usize, 10usize, 0u32, 2120840u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2120844u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2120848u32;
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
pub fn block_0x00205c90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 1u32, 2120852u32);
    emu.sw_no_count(12usize, 2usize, 4u32, 2120856u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2120860u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 872u32, 2120864u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2120868u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 856u32, 2120872u32);
    emu.adi_no_count(12usize, 0usize, 6u32, 2120876u32);
    emu.adi_no_count(13usize, 2usize, 4u32, 2120880u32);
    emu.apc_no_count(1usize, 2120880u32, 98304u32, 2120884u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120888u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966368u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205cb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2120892u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2120896u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120900u32;
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
pub fn block_0x00205cc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2120904u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 895u32, 2120908u32);
    emu.adi_no_count(12usize, 0usize, 5u32, 2120912u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2120916u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2120920u32);
    emu.apc_no_count(6usize, 2120920u32, 98304u32, 2120924u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2120928u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965576u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00205ce0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4u32, 2120932u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2120936u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2120940u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 972u32, 2120944u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2120948u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966212u32, 2120952u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2120956u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 956u32, 2120960u32);
    emu.adi_no_count(12usize, 0usize, 19u32, 2120964u32);
    emu.adi_no_count(14usize, 0usize, 8u32, 2120968u32);
    emu.adi_no_count(15usize, 2usize, 8u32, 2120972u32);
    emu.apc_no_count(1usize, 2120972u32, 98304u32, 2120976u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120980u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965624u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205d14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2120984u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2120988u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120992u32;
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
pub fn block_0x00205d20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2120996u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965918u32, 2121000u32);
    emu.adi_no_count(12usize, 0usize, 16u32, 2121004u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2121008u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2121012u32);
    emu.apc_no_count(6usize, 2121012u32, 98304u32, 2121016u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2121020u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x00205d3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2121024u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965870u32, 2121028u32);
    emu.adi_no_count(12usize, 0usize, 16u32, 2121032u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2121036u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2121040u32);
    emu.apc_no_count(6usize, 2121040u32, 98304u32, 2121044u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2121048u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x00205d58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2121052u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 906u32, 2121056u32);
    emu.adi_no_count(12usize, 0usize, 24u32, 2121060u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2121064u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2121068u32);
    emu.apc_no_count(6usize, 2121068u32, 98304u32, 2121072u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2121076u32;
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
pub fn block_0x00205d74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2121080u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 878u32, 2121084u32);
    emu.adi_no_count(12usize, 0usize, 17u32, 2121088u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2121092u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2121096u32);
    emu.apc_no_count(6usize, 2121096u32, 98304u32, 2121100u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2121104u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x00205d90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2121108u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 900u32, 2121112u32);
    emu.adi_no_count(12usize, 0usize, 6u32, 2121116u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2121120u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2121124u32);
    emu.apc_no_count(6usize, 2121124u32, 98304u32, 2121128u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2121132u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x00205dac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2121136u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966220u32, 2121140u32);
    emu.adi_no_count(12usize, 0usize, 8u32, 2121144u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2121148u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2121152u32);
    emu.apc_no_count(6usize, 2121152u32, 98304u32, 2121156u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2121160u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965344u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205dc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2121164u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 930u32, 2121168u32);
    emu.adi_no_count(12usize, 0usize, 25u32, 2121172u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2121176u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2121180u32);
    emu.apc_no_count(6usize, 2121180u32, 98304u32, 2121184u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2121188u32;
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
#[inline(always)]
pub fn block_0x00205de4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2121192u32)?;
    emu.lbu_no_count(13usize, 12usize, 0u32, 2121196u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2121200u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2121264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205e30));
    } else {
        emu.pc = 2121204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205df4));
    }
}
#[inline]
pub fn block_0x00205df4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2121208u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2121212u32)?;
    emu.adi_no_count(12usize, 12usize, 1u32, 2121216u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2121220u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2121224u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966104u32, 2121228u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2121232u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 372u32, 2121236u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2121240u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2121244u32);
    emu.apc_no_count(1usize, 2121244u32, 98304u32, 2121248u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121252u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966004u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205e24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2121256u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2121260u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121264u32;
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
pub fn block_0x00205e30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2121268u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966120u32, 2121272u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2121276u32);
    emu.apc_no_count(6usize, 2121276u32, 94208u32, 2121280u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2121284u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(2020u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205e44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2121288u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2121292u32)?;
    emu.lw_no_count(12usize, 10usize, 0u32, 2121296u32)?;
    emu.lw_no_count(14usize, 12usize, 0u32, 2121300u32)?;
    emu.adi_no_count(13usize, 0usize, 1u32, 2121304u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2121308u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2121356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205e8c));
    } else {
        emu.pc = 2121312u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205e60));
    }
}
#[inline(always)]
pub fn block_0x00205e60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2121316u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(13usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2121364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205e94));
    } else {
        emu.pc = 2121320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205e68));
    }
}
#[inline(always)]
pub fn block_0x00205e68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 2u32, 2121324u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2121396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205eb4));
    } else {
        emu.pc = 2121328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205e70));
    }
}
#[inline(always)]
pub fn block_0x00205e70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2121332u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 539u32, 2121336u32);
    emu.adi_no_count(12usize, 0usize, 12u32, 2121340u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2121344u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2121348u32);
    emu.apc_no_count(6usize, 2121348u32, 94208u32, 2121352u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2121356u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1948u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205e8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 14usize, 4294967294u32, 2121360u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(13usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2121320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205e68));
    } else {
        emu.pc = 2121364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205e94));
    }
}
#[inline(always)]
pub fn block_0x00205e94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2121460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205ef4));
    } else {
        emu.pc = 2121368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205e98));
    }
}
#[inline(always)]
pub fn block_0x00205e98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2121372u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 513u32, 2121376u32);
    emu.adi_no_count(12usize, 0usize, 26u32, 2121380u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2121384u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2121388u32);
    emu.apc_no_count(6usize, 2121388u32, 94208u32, 2121392u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2121396u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1908u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00205eb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4u32, 2121400u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2121404u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2121408u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 568u32, 2121412u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2121416u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 578u32, 2121420u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2121424u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 552u32, 2121428u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2121432u32);
    emu.adi_no_count(14usize, 0usize, 3u32, 2121436u32);
    emu.adi_no_count(15usize, 2usize, 8u32, 2121440u32);
    emu.apc_no_count(1usize, 2121440u32, 94208u32, 2121444u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121448u32;
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
#[inline(always)]
pub fn block_0x00205ee8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2121452u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2121456u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121460u32;
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
pub fn block_0x00205ef4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(12usize, 2usize, 4u32, 2121464u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2121468u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966132u32, 2121472u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2121476u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 448u32, 2121480u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2121484u32);
    emu.adi_no_count(13usize, 2usize, 4u32, 2121488u32);
    emu.apc_no_count(1usize, 2121488u32, 98304u32, 2121492u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121496u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965760u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205f18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2121500u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2121504u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121508u32;
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
pub fn block_0x00205f24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2121512u32)?;
    emu.lw_no_count(13usize, 10usize, 4u32, 2121516u32)?;
    emu.adi_no_count(14usize, 11usize, 0u32, 2121520u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2121524u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2121528u32);
    emu.adi_no_count(12usize, 14usize, 0u32, 2121532u32);
    emu.apc_no_count(6usize, 2121532u32, 69632u32, 2121536u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2121540u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1132u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205f44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 8u32, 2121544u32)?;
    emu.sli_no_count(13usize, 12usize, 6u32, 2121548u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2121568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205f60));
    } else {
        emu.pc = 2121552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205f50));
    }
}
#[inline(always)]
pub fn block_0x00205f50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 5u32, 2121556u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2121576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205f68));
    } else {
        emu.pc = 2121560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205f58));
    }
}
#[inline(always)]
pub fn block_0x00205f58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2121560u32, 86016u32, 2121564u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2121568u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966992u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205f60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2121568u32, 86016u32, 2121572u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2121576u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x00205f68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2121576u32, 86016u32, 2121580u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2121584u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966600u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00205f70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2121588u32);
    emu.adi_no_count(16usize, 14usize, 0u32, 2121592u32);
    emu.adi_no_count(15usize, 13usize, 0u32, 2121596u32);
    emu.sw_no_count(11usize, 2usize, 8u32, 2121600u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2121604u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2121608u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 388u32, 2121612u32);
    emu.adi_no_count(11usize, 2usize, 8u32, 2121616u32);
    emu.adi_no_count(13usize, 2usize, 12u32, 2121620u32);
    emu.adi_no_count(14usize, 12usize, 0u32, 2121624u32);
    emu.apc_no_count(1usize, 2121624u32, 86016u32, 2121628u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121632u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1664u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205fa0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2121636u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2121640u32)?;
    emu.adi_no_count(12usize, 10usize, 0u32, 2121644u32);
    emu.lw_no_count(13usize, 10usize, 0u32, 2121648u32)?;
    emu.adi_no_count(14usize, 0usize, 3u32, 2121652u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2121656u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2121688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205fd8));
    } else {
        emu.pc = 2121660u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205fbc));
    }
}
#[inline(always)]
pub fn block_0x00205fbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 2u32, 2121664u32);
    emu.sli_no_count(11usize, 11usize, 2u32, 2121668u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2121672u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 272u32, 2121676u32);
    emu.adr_no_count(11usize, 11usize, 13usize, 2121680u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2121684u32)?;
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2121688u32;
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
pub fn block_0x00205fd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 13usize, 4294967292u32, 2121692u32);
    emu.sli_no_count(11usize, 11usize, 2u32, 2121696u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2121700u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 272u32, 2121704u32);
    emu.adr_no_count(11usize, 11usize, 13usize, 2121708u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2121712u32)?;
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2121716u32;
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
pub fn block_0x00205ff4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4u32, 2121720u32);
    emu.sw_no_count(12usize, 2usize, 4u32, 2121724u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2121728u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966132u32, 2121732u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2121736u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 448u32, 2121740u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2121744u32);
    emu.adi_no_count(13usize, 2usize, 4u32, 2121748u32);
    emu.apc_no_count(1usize, 2121748u32, 98304u32, 2121752u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121756u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965500u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020601c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2121760u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2121764u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121768u32;
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
pub fn block_0x00206028(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2121772u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 506u32, 2121776u32);
    emu.adi_no_count(12usize, 0usize, 7u32, 2121780u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2121784u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2121788u32);
    emu.apc_no_count(6usize, 2121788u32, 94208u32, 2121792u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2121796u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1508u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00206044(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(12usize, 2usize, 8u32, 2121800u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2121804u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 488u32, 2121808u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2121812u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 472u32, 2121816u32);
    emu.adi_no_count(12usize, 0usize, 5u32, 2121820u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2121824u32);
    emu.apc_no_count(1usize, 2121824u32, 98304u32, 2121828u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121832u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965424u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206068(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2121836u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2121840u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121844u32;
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
pub fn block_0x00206074(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2121848u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 493u32, 2121852u32);
    emu.adi_no_count(12usize, 0usize, 13u32, 2121856u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2121860u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2121864u32);
    emu.apc_no_count(6usize, 2121864u32, 94208u32, 2121868u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2121872u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1432u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206090(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2121876u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 464u32, 2121880u32);
    emu.adi_no_count(12usize, 0usize, 6u32, 2121884u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2121888u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2121892u32);
    emu.apc_no_count(6usize, 2121892u32, 94208u32, 2121896u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2121900u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1404u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x002060ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 108u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967152u32, 2121904u32);
    emu.sw_no_count(1usize, 2usize, 140u32, 2121908u32)?;
    emu.sw_no_count(8usize, 2usize, 136u32, 2121912u32)?;
    emu.sw_no_count(9usize, 2usize, 132u32, 2121916u32)?;
    emu.sw_no_count(18usize, 2usize, 128u32, 2121920u32)?;
    emu.sw_no_count(19usize, 2usize, 124u32, 2121924u32)?;
    emu.sw_no_count(20usize, 2usize, 120u32, 2121928u32)?;
    emu.sw_no_count(21usize, 2usize, 116u32, 2121932u32)?;
    emu.sw_no_count(22usize, 2usize, 112u32, 2121936u32)?;
    emu.sw_no_count(23usize, 2usize, 108u32, 2121940u32)?;
    emu.sw_no_count(24usize, 2usize, 104u32, 2121944u32)?;
    emu.sw_no_count(25usize, 2usize, 100u32, 2121948u32)?;
    emu.sw_no_count(26usize, 2usize, 96u32, 2121952u32)?;
    emu.sw_no_count(27usize, 2usize, 92u32, 2121956u32)?;
    emu.adi_no_count(9usize, 12usize, 0u32, 2121960u32);
    emu.adi_no_count(24usize, 10usize, 0u32, 2121964u32);
    emu.lbu_no_count(10usize, 11usize, 28u32, 2121968u32);
    emu.lbu_no_count(12usize, 11usize, 29u32, 2121972u32);
    emu.lbu_no_count(13usize, 11usize, 30u32, 2121976u32);
    emu.lbu_no_count(14usize, 11usize, 31u32, 2121980u32);
    emu.lbu_no_count(15usize, 11usize, 24u32, 2121984u32);
    emu.lbu_no_count(16usize, 11usize, 25u32, 2121988u32);
    emu.lbu_no_count(17usize, 11usize, 26u32, 2121992u32);
    emu.lbu_no_count(5usize, 11usize, 27u32, 2121996u32);
    emu.lbu_no_count(6usize, 11usize, 20u32, 2122000u32);
    emu.lbu_no_count(7usize, 11usize, 21u32, 2122004u32);
    emu.lbu_no_count(28usize, 11usize, 22u32, 2122008u32);
    emu.lbu_no_count(29usize, 11usize, 23u32, 2122012u32);
    emu.lbu_no_count(30usize, 11usize, 16u32, 2122016u32);
    emu.lbu_no_count(31usize, 11usize, 17u32, 2122020u32);
    emu.lbu_no_count(8usize, 11usize, 18u32, 2122024u32);
    emu.lbu_no_count(18usize, 11usize, 19u32, 2122028u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2122032u32);
    emu.sli_no_count(13usize, 13usize, 16u32, 2122036u32);
    emu.sli_no_count(14usize, 14usize, 24u32, 2122040u32);
    emu.sli_no_count(16usize, 16usize, 8u32, 2122044u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2122048u32);
    emu.orr_no_count(12usize, 14usize, 13usize, 2122052u32);
    emu.orr_no_count(13usize, 16usize, 15usize, 2122056u32);
    emu.lbu_no_count(19usize, 11usize, 12u32, 2122060u32);
    emu.lbu_no_count(20usize, 11usize, 13u32, 2122064u32);
    emu.lbu_no_count(21usize, 11usize, 14u32, 2122068u32);
    emu.lbu_no_count(22usize, 11usize, 15u32, 2122072u32);
    emu.sli_no_count(17usize, 17usize, 16u32, 2122076u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2122080u32);
    emu.sli_no_count(7usize, 7usize, 8u32, 2122084u32);
    emu.sli_no_count(28usize, 28usize, 16u32, 2122088u32);
    emu.sli_no_count(29usize, 29usize, 24u32, 2122092u32);
    emu.sli_no_count(31usize, 31usize, 8u32, 2122096u32);
    emu.orr_no_count(14usize, 5usize, 17usize, 2122100u32);
    emu.orr_no_count(15usize, 7usize, 6usize, 2122104u32);
    emu.orr_no_count(16usize, 29usize, 28usize, 2122108u32);
    emu.orr_no_count(17usize, 31usize, 30usize, 2122112u32);
    emu.lbu_no_count(5usize, 11usize, 8u32, 2122116u32);
    emu.lbu_no_count(6usize, 11usize, 9u32, 2122120u32);
    emu.lbu_no_count(7usize, 11usize, 10u32, 2122124u32);
    emu.lbu_no_count(28usize, 11usize, 11u32, 2122128u32);
    emu.sli_no_count(8usize, 8usize, 16u32, 2122132u32);
    emu.sli_no_count(18usize, 18usize, 24u32, 2122136u32);
    emu.sli_no_count(20usize, 20usize, 8u32, 2122140u32);
    emu.sli_no_count(21usize, 21usize, 16u32, 2122144u32);
    emu.sli_no_count(22usize, 22usize, 24u32, 2122148u32);
    emu.sli_no_count(6usize, 6usize, 8u32, 2122152u32);
    emu.orr_no_count(29usize, 18usize, 8usize, 2122156u32);
    emu.orr_no_count(30usize, 20usize, 19usize, 2122160u32);
    emu.orr_no_count(31usize, 22usize, 21usize, 2122164u32);
    emu.orr_no_count(5usize, 6usize, 5usize, 2122168u32);
    emu.lbu_no_count(6usize, 11usize, 4u32, 2122172u32);
    emu.lbu_no_count(8usize, 11usize, 5u32, 2122176u32);
    emu.lbu_no_count(18usize, 11usize, 6u32, 2122180u32);
    emu.lbu_no_count(19usize, 11usize, 7u32, 2122184u32);
    emu.sli_no_count(7usize, 7usize, 16u32, 2122188u32);
    emu.sli_no_count(28usize, 28usize, 24u32, 2122192u32);
    emu.sli_no_count(8usize, 8usize, 8u32, 2122196u32);
    emu.sli_no_count(18usize, 18usize, 16u32, 2122200u32);
    emu.sli_no_count(19usize, 19usize, 24u32, 2122204u32);
    emu.orr_no_count(7usize, 28usize, 7usize, 2122208u32);
    emu.orr_no_count(6usize, 8usize, 6usize, 2122212u32);
    emu.lbu_no_count(28usize, 11usize, 0u32, 2122216u32);
    emu.lbu_no_count(8usize, 11usize, 1u32, 2122220u32);
    emu.orr_no_count(18usize, 19usize, 18usize, 2122224u32);
    emu.lbu_no_count(19usize, 11usize, 2u32, 2122228u32);
    emu.lbu_no_count(11usize, 11usize, 3u32, 2122232u32);
    emu.sli_no_count(8usize, 8usize, 8u32, 2122236u32);
    emu.orr_no_count(28usize, 8usize, 28usize, 2122240u32);
    emu.sli_no_count(19usize, 19usize, 16u32, 2122244u32);
    emu.sli_no_count(11usize, 11usize, 24u32, 2122248u32);
    emu.orr_no_count(11usize, 11usize, 19usize, 2122252u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2122256u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2122260u32);
    emu.orr_no_count(12usize, 16usize, 15usize, 2122264u32);
    emu.orr_no_count(14usize, 29usize, 17usize, 2122268u32);
    emu.orr_no_count(15usize, 31usize, 30usize, 2122272u32);
    emu.orr_no_count(16usize, 7usize, 5usize, 2122276u32);
    emu.orr_no_count(17usize, 18usize, 6usize, 2122280u32);
    emu.orr_no_count(11usize, 11usize, 28usize, 2122284u32);
    emu.sw_no_count(14usize, 2usize, 44u32, 2122288u32)?;
    emu.sw_no_count(12usize, 2usize, 48u32, 2122292u32)?;
    emu.sw_no_count(13usize, 2usize, 52u32, 2122296u32)?;
    emu.sw_no_count(10usize, 2usize, 56u32, 2122300u32)?;
    emu.sw_no_count(11usize, 2usize, 28u32, 2122304u32)?;
    emu.sw_no_count(17usize, 2usize, 32u32, 2122308u32)?;
    emu.sw_no_count(16usize, 2usize, 36u32, 2122312u32)?;
    emu.sw_no_count(15usize, 2usize, 40u32, 2122316u32)?;
    emu.adi_no_count(10usize, 2usize, 60u32, 2122320u32);
    emu.adi_no_count(11usize, 2usize, 28u32, 2122324u32);
    emu.apc_no_count(1usize, 2122324u32, 45056u32, 2122328u32);
    emu.add_memory_rw_events(108usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2122332u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1780u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020625c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(4234354688u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2122336u32;
    emu.update_insn_clock();
    emu.lw_no_count(8usize, 2usize, 60u32, 2122340u32)?;
    emu.lw_no_count(18usize, 2usize, 64u32, 2122344u32)?;
    emu.lw_no_count(20usize, 2usize, 68u32, 2122348u32)?;
    emu.adi_no_count(10usize, 10usize, 1361u32, 2122352u32);
    emu.sltru_no_count(10usize, 8usize, 10usize, 2122356u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a >= b {
        emu.pc = 2122460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002062dc));
    } else {
        emu.pc = 2122360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206278));
    }
}
#[inline(always)]
pub fn block_0x00206278(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 18usize, 10usize, 2122364u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2122368u32);
    emu.slti_no_count(10usize, 10usize, 0u32, 2122372u32);
    emu.lw_no_count(21usize, 2usize, 72u32, 2122376u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a < b {
        emu.pc = 2122484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002062f4));
    } else {
        emu.pc = 2122380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020628c));
    }
}
#[inline(always)]
pub fn block_0x0020628c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 20usize, 10usize, 2122384u32);
    let a = 0u32.wrapping_add(2803343360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2122388u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966916u32, 2122392u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2122396u32);
    emu.lw_no_count(22usize, 2usize, 76u32, 2122400u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a < b {
        emu.pc = 2122504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206308));
    } else {
        emu.pc = 2122404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002062a4));
    }
}
#[inline(always)]
pub fn block_0x002062a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 21usize, 10usize, 2122408u32);
    let a = 0u32.wrapping_add(3169255424u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2122412u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965933u32, 2122416u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2122420u32);
    emu.lw_no_count(23usize, 2usize, 80u32, 2122424u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a < b {
        emu.pc = 2122524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020631c));
    } else {
        emu.pc = 2122428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002062bc));
    }
}
#[inline(always)]
pub fn block_0x002062bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 22usize, 10usize, 2122432u32);
    emu.sltiu_no_count(10usize, 10usize, 4294967295u32, 2122436u32);
    emu.lw_no_count(19usize, 2usize, 84u32, 2122440u32)?;
    emu.adi_no_count(26usize, 0usize, 1u32, 2122444u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a < b {
        emu.pc = 2122548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206334));
    } else {
        emu.pc = 2122448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002062d0));
    }
}
#[inline(always)]
pub fn block_0x002062d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 23usize, 10usize, 2122452u32);
    emu.sltiu_no_count(10usize, 10usize, 4294967295u32, 2122456u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2122460u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2122560u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00206340));
}
#[inline(always)]
pub fn block_0x002062dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 18usize, 10usize, 2122464u32);
    let a = 0u32.wrapping_add(4089040896u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2122468u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965954u32, 2122472u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2122476u32);
    emu.lw_no_count(21usize, 2usize, 72u32, 2122480u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a >= b {
        emu.pc = 2122380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020628c));
    } else {
        emu.pc = 2122484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002062f4));
    }
}
#[inline(always)]
pub fn block_0x002062f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 20usize, 10usize, 2122488u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2122492u32);
    emu.slti_no_count(10usize, 10usize, 0u32, 2122496u32);
    emu.lw_no_count(22usize, 2usize, 76u32, 2122500u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a >= b {
        emu.pc = 2122404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002062a4));
    } else {
        emu.pc = 2122504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206308));
    }
}
#[inline(always)]
pub fn block_0x00206308(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 21usize, 10usize, 2122508u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2122512u32);
    emu.slti_no_count(10usize, 10usize, 0u32, 2122516u32);
    emu.lw_no_count(23usize, 2usize, 80u32, 2122520u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a >= b {
        emu.pc = 2122428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002062bc));
    } else {
        emu.pc = 2122524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020631c));
    }
}
#[inline(always)]
pub fn block_0x0020631c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 22usize, 10usize, 2122528u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2122532u32);
    emu.slti_no_count(10usize, 10usize, 0u32, 2122536u32);
    emu.lw_no_count(19usize, 2usize, 84u32, 2122540u32)?;
    emu.adi_no_count(26usize, 0usize, 1u32, 2122544u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a >= b {
        emu.pc = 2122448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002062d0));
    } else {
        emu.pc = 2122548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206334));
    }
}
#[inline(always)]
pub fn block_0x00206334(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 23usize, 10usize, 2122552u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2122556u32);
    emu.slti_no_count(10usize, 10usize, 0u32, 2122560u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2122560u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00206340));
}
#[inline]
pub fn block_0x00206340(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(25usize, 2usize, 88u32, 2122564u32)?;
    emu.sltru_no_count(10usize, 19usize, 10usize, 2122568u32);
    emu.sbr_no_count(11usize, 25usize, 10usize, 2122572u32);
    emu.sltru_no_count(12usize, 11usize, 25usize, 2122576u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2122580u32);
    emu.sbr_no_count(12usize, 12usize, 10usize, 2122584u32);
    emu.sltiu_no_count(10usize, 11usize, 1u32, 2122588u32);
    emu.adr_no_count(10usize, 12usize, 10usize, 2122592u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2122596u32);
    emu.apc_no_count(1usize, 2122596u32, 65536u32, 2122600u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2122604u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(212u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020636c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 255u32, 2122608u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2123128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206578));
    } else {
        emu.pc = 2122612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206374));
    }
}
#[inline(never)]
pub fn block_0x00206374(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 96u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(19usize, 2usize, 12u32, 2122616u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2122620u32)?;
    emu.sw_no_count(8usize, 2usize, 20u32, 2122624u32)?;
    emu.sw_no_count(24usize, 2usize, 24u32, 2122628u32)?;
    emu.lbu_no_count(10usize, 9usize, 28u32, 2122632u32);
    emu.lbu_no_count(11usize, 9usize, 29u32, 2122636u32);
    emu.lbu_no_count(12usize, 9usize, 30u32, 2122640u32);
    emu.lbu_no_count(13usize, 9usize, 31u32, 2122644u32);
    emu.lbu_no_count(14usize, 9usize, 24u32, 2122648u32);
    emu.lbu_no_count(15usize, 9usize, 25u32, 2122652u32);
    emu.lbu_no_count(16usize, 9usize, 26u32, 2122656u32);
    emu.lbu_no_count(17usize, 9usize, 27u32, 2122660u32);
    emu.lbu_no_count(5usize, 9usize, 20u32, 2122664u32);
    emu.lbu_no_count(6usize, 9usize, 21u32, 2122668u32);
    emu.lbu_no_count(7usize, 9usize, 22u32, 2122672u32);
    emu.lbu_no_count(28usize, 9usize, 23u32, 2122676u32);
    emu.lbu_no_count(29usize, 9usize, 16u32, 2122680u32);
    emu.lbu_no_count(30usize, 9usize, 17u32, 2122684u32);
    emu.lbu_no_count(31usize, 9usize, 18u32, 2122688u32);
    emu.lbu_no_count(8usize, 9usize, 19u32, 2122692u32);
    emu.sli_no_count(11usize, 11usize, 8u32, 2122696u32);
    emu.sli_no_count(12usize, 12usize, 16u32, 2122700u32);
    emu.sli_no_count(13usize, 13usize, 24u32, 2122704u32);
    emu.sli_no_count(15usize, 15usize, 8u32, 2122708u32);
    emu.orr_no_count(10usize, 11usize, 10usize, 2122712u32);
    emu.orr_no_count(11usize, 13usize, 12usize, 2122716u32);
    emu.orr_no_count(12usize, 15usize, 14usize, 2122720u32);
    emu.lbu_no_count(18usize, 9usize, 12u32, 2122724u32);
    emu.lbu_no_count(19usize, 9usize, 13u32, 2122728u32);
    emu.lbu_no_count(24usize, 9usize, 14u32, 2122732u32);
    emu.lbu_no_count(26usize, 9usize, 15u32, 2122736u32);
    emu.sli_no_count(16usize, 16usize, 16u32, 2122740u32);
    emu.sli_no_count(17usize, 17usize, 24u32, 2122744u32);
    emu.sli_no_count(6usize, 6usize, 8u32, 2122748u32);
    emu.sli_no_count(7usize, 7usize, 16u32, 2122752u32);
    emu.sli_no_count(28usize, 28usize, 24u32, 2122756u32);
    emu.sli_no_count(30usize, 30usize, 8u32, 2122760u32);
    emu.orr_no_count(13usize, 17usize, 16usize, 2122764u32);
    emu.orr_no_count(14usize, 6usize, 5usize, 2122768u32);
    emu.orr_no_count(15usize, 28usize, 7usize, 2122772u32);
    emu.orr_no_count(16usize, 30usize, 29usize, 2122776u32);
    emu.lbu_no_count(17usize, 9usize, 8u32, 2122780u32);
    emu.lbu_no_count(5usize, 9usize, 9u32, 2122784u32);
    emu.lbu_no_count(6usize, 9usize, 10u32, 2122788u32);
    emu.lbu_no_count(7usize, 9usize, 11u32, 2122792u32);
    emu.sli_no_count(31usize, 31usize, 16u32, 2122796u32);
    emu.sli_no_count(8usize, 8usize, 24u32, 2122800u32);
    emu.sli_no_count(19usize, 19usize, 8u32, 2122804u32);
    emu.sli_no_count(24usize, 24usize, 16u32, 2122808u32);
    emu.sli_no_count(26usize, 26usize, 24u32, 2122812u32);
    emu.sli_no_count(5usize, 5usize, 8u32, 2122816u32);
    emu.orr_no_count(28usize, 8usize, 31usize, 2122820u32);
    emu.orr_no_count(29usize, 19usize, 18usize, 2122824u32);
    emu.orr_no_count(30usize, 26usize, 24usize, 2122828u32);
    emu.orr_no_count(17usize, 5usize, 17usize, 2122832u32);
    emu.lbu_no_count(5usize, 9usize, 4u32, 2122836u32);
    emu.lbu_no_count(31usize, 9usize, 5u32, 2122840u32);
    emu.lbu_no_count(8usize, 9usize, 6u32, 2122844u32);
    emu.lbu_no_count(18usize, 9usize, 7u32, 2122848u32);
    emu.sli_no_count(6usize, 6usize, 16u32, 2122852u32);
    emu.sli_no_count(7usize, 7usize, 24u32, 2122856u32);
    emu.sli_no_count(31usize, 31usize, 8u32, 2122860u32);
    emu.sli_no_count(8usize, 8usize, 16u32, 2122864u32);
    emu.sli_no_count(18usize, 18usize, 24u32, 2122868u32);
    emu.orr_no_count(6usize, 7usize, 6usize, 2122872u32);
    emu.orr_no_count(5usize, 31usize, 5usize, 2122876u32);
    emu.lbu_no_count(7usize, 9usize, 0u32, 2122880u32);
    emu.lbu_no_count(31usize, 9usize, 1u32, 2122884u32);
    emu.orr_no_count(8usize, 18usize, 8usize, 2122888u32);
    emu.lbu_no_count(18usize, 9usize, 2u32, 2122892u32);
    emu.lbu_no_count(9usize, 9usize, 3u32, 2122896u32);
    emu.sli_no_count(31usize, 31usize, 8u32, 2122900u32);
    emu.orr_no_count(7usize, 31usize, 7usize, 2122904u32);
    emu.sli_no_count(18usize, 18usize, 16u32, 2122908u32);
    emu.sli_no_count(9usize, 9usize, 24u32, 2122912u32);
    emu.orr_no_count(31usize, 9usize, 18usize, 2122916u32);
    emu.orr_no_count(10usize, 11usize, 10usize, 2122920u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2122924u32);
    emu.orr_no_count(14usize, 15usize, 14usize, 2122928u32);
    emu.orr_no_count(11usize, 28usize, 16usize, 2122932u32);
    emu.orr_no_count(13usize, 30usize, 29usize, 2122936u32);
    emu.orr_no_count(15usize, 6usize, 17usize, 2122940u32);
    emu.orr_no_count(16usize, 8usize, 5usize, 2122944u32);
    emu.orr_no_count(17usize, 31usize, 7usize, 2122948u32);
    emu.sw_no_count(11usize, 2usize, 44u32, 2122952u32)?;
    emu.sw_no_count(14usize, 2usize, 48u32, 2122956u32)?;
    emu.sw_no_count(12usize, 2usize, 52u32, 2122960u32)?;
    emu.sw_no_count(10usize, 2usize, 56u32, 2122964u32)?;
    emu.sw_no_count(17usize, 2usize, 28u32, 2122968u32)?;
    emu.sw_no_count(16usize, 2usize, 32u32, 2122972u32)?;
    emu.sw_no_count(15usize, 2usize, 36u32, 2122976u32)?;
    emu.sw_no_count(13usize, 2usize, 40u32, 2122980u32)?;
    emu.adi_no_count(10usize, 2usize, 60u32, 2122984u32);
    emu.adi_no_count(11usize, 2usize, 28u32, 2122988u32);
    emu.apc_no_count(1usize, 2122988u32, 45056u32, 2122992u32);
    emu.add_memory_rw_events(96usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2122996u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1116u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002064f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(4234354688u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2123000u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 2usize, 60u32, 2123004u32)?;
    emu.lw_no_count(27usize, 2usize, 64u32, 2123008u32)?;
    emu.lw_no_count(26usize, 2usize, 68u32, 2123012u32)?;
    emu.adi_no_count(10usize, 10usize, 1361u32, 2123016u32);
    emu.sw_no_count(11usize, 2usize, 8u32, 2123020u32)?;
    emu.sltru_no_count(10usize, 11usize, 10usize, 2123024u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(27usize);
    if a >= b {
        emu.pc = 2123144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206588));
    } else {
        emu.pc = 2123028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206514));
    }
}
#[inline(always)]
pub fn block_0x00206514(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 27usize, 10usize, 2123032u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2123036u32);
    emu.slti_no_count(10usize, 10usize, 0u32, 2123040u32);
    emu.lw_no_count(18usize, 2usize, 72u32, 2123044u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a < b {
        emu.pc = 2123168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002065a0));
    } else {
        emu.pc = 2123048u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206528));
    }
}
#[inline(always)]
pub fn block_0x00206528(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 26usize, 10usize, 2123052u32);
    let a = 0u32.wrapping_add(2803343360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2123056u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966916u32, 2123060u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2123064u32);
    emu.lw_no_count(8usize, 2usize, 76u32, 2123068u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a < b {
        emu.pc = 2123188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002065b4));
    } else {
        emu.pc = 2123072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206540));
    }
}
#[inline(always)]
pub fn block_0x00206540(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 18usize, 10usize, 2123076u32);
    let a = 0u32.wrapping_add(3169255424u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2123080u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965933u32, 2123084u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2123088u32);
    emu.lw_no_count(9usize, 2usize, 80u32, 2123092u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a < b {
        emu.pc = 2123208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002065c8));
    } else {
        emu.pc = 2123096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206558));
    }
}
#[inline(always)]
pub fn block_0x00206558(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 8usize, 10usize, 2123100u32);
    emu.sltiu_no_count(11usize, 10usize, 4294967295u32, 2123104u32);
    emu.lw_no_count(24usize, 2usize, 84u32, 2123108u32)?;
    emu.adi_no_count(10usize, 0usize, 4294967295u32, 2123112u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a < b {
        emu.pc = 2123232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002065e0));
    } else {
        emu.pc = 2123116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020656c));
    }
}
#[inline(always)]
pub fn block_0x0020656c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 9usize, 11usize, 2123120u32);
    emu.sltiu_no_count(11usize, 11usize, 4294967295u32, 2123124u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2123128u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2123244u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002065ec));
}
#[inline(always)]
pub fn block_0x00206578(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2123132u32);
    emu.sw_no_count(10usize, 24usize, 0u32, 2123136u32)?;
    emu.sw_no_count(0usize, 24usize, 4u32, 2123140u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2123144u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2123440u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002066b0));
}
#[inline(always)]
pub fn block_0x00206588(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 27usize, 10usize, 2123148u32);
    let a = 0u32.wrapping_add(4089040896u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2123152u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965954u32, 2123156u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2123160u32);
    emu.lw_no_count(18usize, 2usize, 72u32, 2123164u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a >= b {
        emu.pc = 2123048u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206528));
    } else {
        emu.pc = 2123168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002065a0));
    }
}
#[inline(always)]
pub fn block_0x002065a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 26usize, 10usize, 2123172u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2123176u32);
    emu.slti_no_count(10usize, 10usize, 0u32, 2123180u32);
    emu.lw_no_count(8usize, 2usize, 76u32, 2123184u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a >= b {
        emu.pc = 2123072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206540));
    } else {
        emu.pc = 2123188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002065b4));
    }
}
#[inline(always)]
pub fn block_0x002065b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 18usize, 10usize, 2123192u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2123196u32);
    emu.slti_no_count(10usize, 10usize, 0u32, 2123200u32);
    emu.lw_no_count(9usize, 2usize, 80u32, 2123204u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a >= b {
        emu.pc = 2123096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206558));
    } else {
        emu.pc = 2123208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002065c8));
    }
}
#[inline(always)]
pub fn block_0x002065c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 8usize, 10usize, 2123212u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2123216u32);
    emu.slti_no_count(11usize, 10usize, 0u32, 2123220u32);
    emu.lw_no_count(24usize, 2usize, 84u32, 2123224u32)?;
    emu.adi_no_count(10usize, 0usize, 4294967295u32, 2123228u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a >= b {
        emu.pc = 2123116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020656c));
    } else {
        emu.pc = 2123232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002065e0));
    }
}
#[inline(always)]
pub fn block_0x002065e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(11usize, 9usize, 11usize, 2123236u32);
    emu.sbr_no_count(11usize, 0usize, 11usize, 2123240u32);
    emu.slti_no_count(11usize, 11usize, 0u32, 2123244u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2123244u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002065ec));
}
#[inline]
pub fn block_0x002065ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(19usize, 2usize, 88u32, 2123248u32)?;
    emu.sltru_no_count(11usize, 24usize, 11usize, 2123252u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2123256u32);
    emu.adr_no_count(12usize, 19usize, 12usize, 2123260u32);
    emu.sltru_no_count(13usize, 12usize, 19usize, 2123264u32);
    emu.adr_no_count(10usize, 10usize, 13usize, 2123268u32);
    emu.sbr_no_count(13usize, 12usize, 11usize, 2123272u32);
    emu.sbr_no_count(10usize, 10usize, 11usize, 2123276u32);
    emu.sltru_no_count(11usize, 13usize, 12usize, 2123280u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2123284u32);
    emu.apc_no_count(1usize, 2123284u32, 65536u32, 2123288u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2123292u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966820u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020661c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 10usize, 255u32, 2123296u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2123300u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2123428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002066a4));
    } else {
        emu.pc = 2123304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206628));
    }
}
#[inline]
pub fn block_0x00206628(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.orr_no_count(10usize, 21usize, 20usize, 2123308u32);
    emu.orr_no_count(11usize, 22usize, 23usize, 2123312u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2123316u32);
    emu.lw_no_count(11usize, 2usize, 12u32, 2123320u32)?;
    emu.orr_no_count(11usize, 11usize, 25usize, 2123324u32);
    emu.lw_no_count(12usize, 2usize, 20u32, 2123328u32)?;
    emu.orr_no_count(11usize, 11usize, 12usize, 2123332u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2123336u32);
    emu.lw_no_count(11usize, 2usize, 16u32, 2123340u32)?;
    emu.orr_no_count(10usize, 10usize, 11usize, 2123344u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2123348u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2123352u32);
    emu.apc_no_count(1usize, 2123352u32, 65536u32, 2123356u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2123360u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966752u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206660(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 255u32, 2123364u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2123424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002066a0));
    } else {
        emu.pc = 2123368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206668));
    }
}
#[inline]
pub fn block_0x00206668(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.orr_no_count(10usize, 18usize, 26usize, 2123372u32);
    emu.orr_no_count(11usize, 8usize, 9usize, 2123376u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2123380u32);
    emu.orr_no_count(11usize, 24usize, 19usize, 2123384u32);
    emu.lw_no_count(12usize, 2usize, 8u32, 2123388u32)?;
    emu.orr_no_count(11usize, 11usize, 12usize, 2123392u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2123396u32);
    emu.orr_no_count(10usize, 10usize, 27usize, 2123400u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2123404u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2123408u32);
    emu.apc_no_count(1usize, 2123408u32, 65536u32, 2123412u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2123416u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966696u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206698(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 255u32, 2123420u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2123500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002066ec));
    } else {
        emu.pc = 2123424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002066a0));
    }
}
#[inline(always)]
pub fn block_0x002066a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2123428u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2123428u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002066a4));
}
#[inline(always)]
pub fn block_0x002066a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 24u32, 2123432u32)?;
    emu.sw_no_count(10usize, 11usize, 0u32, 2123436u32)?;
    emu.sw_no_count(0usize, 11usize, 4u32, 2123440u32)?;
    emu.add_memory_rw_events(3usize);
    emu.pc = 2123440u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002066b0));
}
#[inline]
pub fn block_0x002066b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 140u32, 2123444u32)?;
    emu.lw_no_count(8usize, 2usize, 136u32, 2123448u32)?;
    emu.lw_no_count(9usize, 2usize, 132u32, 2123452u32)?;
    emu.lw_no_count(18usize, 2usize, 128u32, 2123456u32)?;
    emu.lw_no_count(19usize, 2usize, 124u32, 2123460u32)?;
    emu.lw_no_count(20usize, 2usize, 120u32, 2123464u32)?;
    emu.lw_no_count(21usize, 2usize, 116u32, 2123468u32)?;
    emu.lw_no_count(22usize, 2usize, 112u32, 2123472u32)?;
    emu.lw_no_count(23usize, 2usize, 108u32, 2123476u32)?;
    emu.lw_no_count(24usize, 2usize, 104u32, 2123480u32)?;
    emu.lw_no_count(25usize, 2usize, 100u32, 2123484u32)?;
    emu.lw_no_count(26usize, 2usize, 96u32, 2123488u32)?;
    emu.lw_no_count(27usize, 2usize, 92u32, 2123492u32)?;
    emu.adi_no_count(2usize, 2usize, 144u32, 2123496u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2123500u32;
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
pub fn block_0x002066ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 23u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 24u32, 2123504u32)?;
    emu.sw_no_count(0usize, 10usize, 0u32, 2123508u32)?;
    emu.lw_no_count(11usize, 2usize, 20u32, 2123512u32)?;
    emu.sw_no_count(11usize, 10usize, 4u32, 2123516u32)?;
    emu.lw_no_count(11usize, 2usize, 16u32, 2123520u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2123524u32)?;
    emu.sw_no_count(20usize, 10usize, 12u32, 2123528u32)?;
    emu.sw_no_count(21usize, 10usize, 16u32, 2123532u32)?;
    emu.sw_no_count(22usize, 10usize, 20u32, 2123536u32)?;
    emu.sw_no_count(23usize, 10usize, 24u32, 2123540u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2123544u32)?;
    emu.sw_no_count(11usize, 10usize, 28u32, 2123548u32)?;
    emu.sw_no_count(25usize, 10usize, 32u32, 2123552u32)?;
    emu.lw_no_count(11usize, 2usize, 8u32, 2123556u32)?;
    emu.sw_no_count(11usize, 10usize, 36u32, 2123560u32)?;
    emu.sw_no_count(27usize, 10usize, 40u32, 2123564u32)?;
    emu.sw_no_count(26usize, 10usize, 44u32, 2123568u32)?;
    emu.sw_no_count(18usize, 10usize, 48u32, 2123572u32)?;
    emu.sw_no_count(8usize, 10usize, 52u32, 2123576u32)?;
    emu.sw_no_count(9usize, 10usize, 56u32, 2123580u32)?;
    emu.sw_no_count(24usize, 10usize, 60u32, 2123584u32)?;
    emu.sw_no_count(19usize, 10usize, 64u32, 2123588u32)?;
    emu.add_memory_rw_events(23usize);
    let return_addr = 2123592u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2123440u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002066b0));
}
#[inline]
pub fn block_0x00206748(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294966656u32, 2123596u32);
    emu.sw_no_count(1usize, 2usize, 636u32, 2123600u32)?;
    emu.sw_no_count(8usize, 2usize, 632u32, 2123604u32)?;
    emu.sw_no_count(9usize, 2usize, 628u32, 2123608u32)?;
    emu.sw_no_count(18usize, 2usize, 624u32, 2123612u32)?;
    emu.sw_no_count(19usize, 2usize, 620u32, 2123616u32)?;
    emu.adi_no_count(18usize, 13usize, 0u32, 2123620u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2123624u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2123628u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2123632u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2123636u32);
    emu.apc_no_count(1usize, 2123636u32, 24576u32, 2123640u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2123644u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1216u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0020677c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 16u32, 2123648u32)?;
    emu.lw_no_count(11usize, 18usize, 20u32, 2123652u32)?;
    emu.lw_no_count(12usize, 18usize, 24u32, 2123656u32)?;
    emu.lw_no_count(13usize, 18usize, 28u32, 2123660u32)?;
    emu.sw_no_count(10usize, 2usize, 380u32, 2123664u32)?;
    emu.sw_no_count(11usize, 2usize, 384u32, 2123668u32)?;
    emu.sw_no_count(12usize, 2usize, 388u32, 2123672u32)?;
    emu.sw_no_count(13usize, 2usize, 392u32, 2123676u32)?;
    emu.lw_no_count(10usize, 18usize, 0u32, 2123680u32)?;
    emu.lw_no_count(11usize, 18usize, 4u32, 2123684u32)?;
    emu.lw_no_count(12usize, 18usize, 8u32, 2123688u32)?;
    emu.lw_no_count(13usize, 18usize, 12u32, 2123692u32)?;
    emu.sw_no_count(0usize, 2usize, 476u32, 2123696u32)?;
    emu.sw_no_count(0usize, 2usize, 480u32, 2123700u32)?;
    emu.sw_no_count(0usize, 2usize, 484u32, 2123704u32)?;
    emu.sw_no_count(0usize, 2usize, 488u32, 2123708u32)?;
    emu.sw_no_count(10usize, 2usize, 364u32, 2123712u32)?;
    emu.sw_no_count(11usize, 2usize, 368u32, 2123716u32)?;
    emu.sw_no_count(12usize, 2usize, 372u32, 2123720u32)?;
    emu.sw_no_count(13usize, 2usize, 376u32, 2123724u32)?;
    emu.sw_no_count(0usize, 2usize, 460u32, 2123728u32)?;
    emu.sw_no_count(0usize, 2usize, 464u32, 2123732u32)?;
    emu.sw_no_count(0usize, 2usize, 468u32, 2123736u32)?;
    emu.sw_no_count(0usize, 2usize, 472u32, 2123740u32)?;
    emu.adi_no_count(10usize, 2usize, 364u32, 2123744u32);
    emu.adi_no_count(11usize, 2usize, 460u32, 2123748u32);
    emu.apc_no_count(1usize, 2123748u32, 24576u32, 2123752u32);
    emu.add_memory_rw_events(28usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2123756u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1996u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002067ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(10usize, 10usize, 4294967295u32, 2123760u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2123764u32);
    emu.apc_no_count(1usize, 2123764u32, 65536u32, 2123768u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2123772u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966320u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002067fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 10usize, 255u32, 2123776u32);
    emu.adi_no_count(19usize, 0usize, 1u32, 2123780u32);
    emu.sb_no_count(10usize, 2usize, 364u32, 2123784u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2124764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206bdc));
    } else {
        emu.pc = 2123788u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020680c));
    }
}
#[inline(never)]
pub fn block_0x0020680c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 48u32, 2123792u32)?;
    emu.lw_no_count(11usize, 18usize, 52u32, 2123796u32)?;
    emu.lw_no_count(12usize, 18usize, 56u32, 2123800u32)?;
    emu.lw_no_count(13usize, 18usize, 60u32, 2123804u32)?;
    emu.sw_no_count(10usize, 2usize, 380u32, 2123808u32)?;
    emu.sw_no_count(11usize, 2usize, 384u32, 2123812u32)?;
    emu.sw_no_count(12usize, 2usize, 388u32, 2123816u32)?;
    emu.sw_no_count(13usize, 2usize, 392u32, 2123820u32)?;
    emu.lw_no_count(10usize, 18usize, 32u32, 2123824u32)?;
    emu.lw_no_count(11usize, 18usize, 36u32, 2123828u32)?;
    emu.lw_no_count(12usize, 18usize, 40u32, 2123832u32)?;
    emu.lw_no_count(13usize, 18usize, 44u32, 2123836u32)?;
    emu.sw_no_count(0usize, 2usize, 476u32, 2123840u32)?;
    emu.sw_no_count(0usize, 2usize, 480u32, 2123844u32)?;
    emu.sw_no_count(0usize, 2usize, 484u32, 2123848u32)?;
    emu.sw_no_count(0usize, 2usize, 488u32, 2123852u32)?;
    emu.sw_no_count(10usize, 2usize, 364u32, 2123856u32)?;
    emu.sw_no_count(11usize, 2usize, 368u32, 2123860u32)?;
    emu.sw_no_count(12usize, 2usize, 372u32, 2123864u32)?;
    emu.sw_no_count(13usize, 2usize, 376u32, 2123868u32)?;
    emu.sw_no_count(0usize, 2usize, 460u32, 2123872u32)?;
    emu.sw_no_count(0usize, 2usize, 464u32, 2123876u32)?;
    emu.sw_no_count(0usize, 2usize, 468u32, 2123880u32)?;
    emu.sw_no_count(0usize, 2usize, 472u32, 2123884u32)?;
    emu.adi_no_count(10usize, 2usize, 364u32, 2123888u32);
    emu.adi_no_count(11usize, 2usize, 460u32, 2123892u32);
    emu.apc_no_count(1usize, 2123892u32, 24576u32, 2123896u32);
    emu.add_memory_rw_events(28usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2123900u32;
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
#[inline(always)]
pub fn block_0x0020687c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(10usize, 10usize, 4294967295u32, 2123904u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2123908u32);
    emu.apc_no_count(1usize, 2123908u32, 65536u32, 2123912u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2123916u32;
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
#[inline(always)]
pub fn block_0x0020688c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 10usize, 255u32, 2123920u32);
    emu.sb_no_count(10usize, 2usize, 364u32, 2123924u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2124764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206bdc));
    } else {
        emu.pc = 2123928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206898));
    }
}
#[inline(never)]
pub fn block_0x00206898(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 36u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 48u32, 2123932u32)?;
    emu.lw_no_count(11usize, 18usize, 52u32, 2123936u32)?;
    emu.lw_no_count(12usize, 18usize, 56u32, 2123940u32)?;
    emu.lw_no_count(13usize, 18usize, 60u32, 2123944u32)?;
    emu.sw_no_count(10usize, 2usize, 88u32, 2123948u32)?;
    emu.sw_no_count(11usize, 2usize, 92u32, 2123952u32)?;
    emu.sw_no_count(12usize, 2usize, 96u32, 2123956u32)?;
    emu.sw_no_count(13usize, 2usize, 100u32, 2123960u32)?;
    emu.lw_no_count(10usize, 18usize, 32u32, 2123964u32)?;
    emu.lw_no_count(11usize, 18usize, 36u32, 2123968u32)?;
    emu.lw_no_count(12usize, 18usize, 40u32, 2123972u32)?;
    emu.lw_no_count(13usize, 18usize, 44u32, 2123976u32)?;
    emu.sw_no_count(10usize, 2usize, 72u32, 2123980u32)?;
    emu.sw_no_count(11usize, 2usize, 76u32, 2123984u32)?;
    emu.sw_no_count(12usize, 2usize, 80u32, 2123988u32)?;
    emu.sw_no_count(13usize, 2usize, 84u32, 2123992u32)?;
    emu.lw_no_count(10usize, 18usize, 16u32, 2123996u32)?;
    emu.lw_no_count(11usize, 18usize, 20u32, 2124000u32)?;
    emu.lw_no_count(12usize, 18usize, 24u32, 2124004u32)?;
    emu.lw_no_count(13usize, 18usize, 28u32, 2124008u32)?;
    emu.sw_no_count(10usize, 2usize, 56u32, 2124012u32)?;
    emu.sw_no_count(11usize, 2usize, 60u32, 2124016u32)?;
    emu.sw_no_count(12usize, 2usize, 64u32, 2124020u32)?;
    emu.sw_no_count(13usize, 2usize, 68u32, 2124024u32)?;
    emu.lw_no_count(10usize, 18usize, 0u32, 2124028u32)?;
    emu.lw_no_count(11usize, 18usize, 4u32, 2124032u32)?;
    emu.lw_no_count(12usize, 18usize, 8u32, 2124036u32)?;
    emu.lw_no_count(13usize, 18usize, 12u32, 2124040u32)?;
    emu.sw_no_count(10usize, 2usize, 40u32, 2124044u32)?;
    emu.sw_no_count(11usize, 2usize, 44u32, 2124048u32)?;
    emu.sw_no_count(12usize, 2usize, 48u32, 2124052u32)?;
    emu.sw_no_count(13usize, 2usize, 52u32, 2124056u32)?;
    emu.adi_no_count(10usize, 2usize, 460u32, 2124060u32);
    emu.adi_no_count(11usize, 2usize, 72u32, 2124064u32);
    emu.apc_no_count(1usize, 2124064u32, 24576u32, 2124068u32);
    emu.add_memory_rw_events(36usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2124072u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965560u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206928(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 492u32, 2124076u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2124080u32);
    emu.sb_no_count(10usize, 2usize, 268u32, 2124084u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2124804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206c04));
    } else {
        emu.pc = 2124088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206938));
    }
}
#[inline]
pub fn block_0x00206938(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 476u32, 2124092u32)?;
    emu.lw_no_count(11usize, 2usize, 480u32, 2124096u32)?;
    emu.lw_no_count(12usize, 2usize, 484u32, 2124100u32)?;
    emu.lw_no_count(13usize, 2usize, 488u32, 2124104u32)?;
    emu.sw_no_count(10usize, 2usize, 120u32, 2124108u32)?;
    emu.sw_no_count(11usize, 2usize, 124u32, 2124112u32)?;
    emu.sw_no_count(12usize, 2usize, 128u32, 2124116u32)?;
    emu.sw_no_count(13usize, 2usize, 132u32, 2124120u32)?;
    emu.lw_no_count(10usize, 2usize, 460u32, 2124124u32)?;
    emu.lw_no_count(11usize, 2usize, 464u32, 2124128u32)?;
    emu.lw_no_count(12usize, 2usize, 468u32, 2124132u32)?;
    emu.lw_no_count(13usize, 2usize, 472u32, 2124136u32)?;
    emu.sw_no_count(10usize, 2usize, 104u32, 2124140u32)?;
    emu.sw_no_count(11usize, 2usize, 108u32, 2124144u32)?;
    emu.sw_no_count(12usize, 2usize, 112u32, 2124148u32)?;
    emu.sw_no_count(13usize, 2usize, 116u32, 2124152u32)?;
    emu.adi_no_count(10usize, 2usize, 136u32, 2124156u32);
    emu.adi_no_count(11usize, 2usize, 8u32, 2124160u32);
    emu.adi_no_count(12usize, 2usize, 104u32, 2124164u32);
    emu.apc_no_count(1usize, 2124164u32, 24576u32, 2124168u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2124172u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(660u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020698c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 168u32, 2124176u32);
    emu.adi_no_count(12usize, 2usize, 104u32, 2124180u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2124184u32);
    emu.apc_no_count(1usize, 2124184u32, 24576u32, 2124188u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2124192u32;
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
pub fn block_0x002069a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2124196u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1072u32, 2124200u32);
    emu.adi_no_count(10usize, 2usize, 460u32, 2124204u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2124208u32);
    emu.apc_no_count(1usize, 2124208u32, 8192u32, 2124212u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2124216u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(900u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002069b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 268u32, 2124220u32);
    emu.adi_no_count(11usize, 2usize, 460u32, 2124224u32);
    emu.adi_no_count(12usize, 2usize, 136u32, 2124228u32);
    emu.apc_no_count(1usize, 2124228u32, 4294950912u32, 2124232u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2124236u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1980u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002069cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 460u32, 2124240u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2124244u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2124248u32);
    emu.apc_no_count(1usize, 2124248u32, 8192u32, 2124252u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2124256u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(860u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002069e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 364u32, 2124260u32);
    emu.adi_no_count(11usize, 2usize, 460u32, 2124264u32);
    emu.adi_no_count(12usize, 2usize, 168u32, 2124268u32);
    emu.apc_no_count(1usize, 2124268u32, 4294950912u32, 2124272u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2124276u32;
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
pub fn block_0x002069f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 460u32, 2124280u32);
    emu.adi_no_count(11usize, 2usize, 268u32, 2124284u32);
    emu.adi_no_count(12usize, 2usize, 364u32, 2124288u32);
    emu.apc_no_count(1usize, 2124288u32, 4294959104u32, 2124292u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2124296u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965560u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206a08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 2usize, 524u32, 2124300u32);
    emu.adi_no_count(10usize, 2usize, 200u32, 2124304u32);
    emu.apc_no_count(1usize, 2124304u32, 40960u32, 2124308u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2124312u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1312u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00206a18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 2usize, 604u32, 2124316u32)?;
    emu.sw_no_count(0usize, 2usize, 608u32, 2124320u32)?;
    emu.sw_no_count(0usize, 2usize, 612u32, 2124324u32)?;
    emu.sw_no_count(0usize, 2usize, 616u32, 2124328u32)?;
    emu.lbu_no_count(13usize, 2usize, 232u32, 2124332u32);
    emu.sw_no_count(0usize, 2usize, 588u32, 2124336u32)?;
    emu.sw_no_count(0usize, 2usize, 592u32, 2124340u32)?;
    emu.sw_no_count(0usize, 2usize, 596u32, 2124344u32)?;
    emu.sw_no_count(0usize, 2usize, 600u32, 2124348u32)?;
    emu.adi_no_count(10usize, 2usize, 556u32, 2124352u32);
    emu.adi_no_count(11usize, 2usize, 588u32, 2124356u32);
    emu.adi_no_count(12usize, 2usize, 200u32, 2124360u32);
    emu.apc_no_count(1usize, 2124360u32, 40960u32, 2124364u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2124368u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(836u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00206a50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 476u32, 2124372u32)?;
    emu.lw_no_count(11usize, 2usize, 480u32, 2124376u32)?;
    emu.lw_no_count(12usize, 2usize, 484u32, 2124380u32)?;
    emu.lw_no_count(13usize, 2usize, 488u32, 2124384u32)?;
    emu.sw_no_count(10usize, 2usize, 380u32, 2124388u32)?;
    emu.sw_no_count(11usize, 2usize, 384u32, 2124392u32)?;
    emu.sw_no_count(12usize, 2usize, 388u32, 2124396u32)?;
    emu.sw_no_count(13usize, 2usize, 392u32, 2124400u32)?;
    emu.lw_no_count(10usize, 2usize, 460u32, 2124404u32)?;
    emu.lw_no_count(11usize, 2usize, 464u32, 2124408u32)?;
    emu.lw_no_count(12usize, 2usize, 468u32, 2124412u32)?;
    emu.lw_no_count(13usize, 2usize, 472u32, 2124416u32)?;
    emu.sw_no_count(10usize, 2usize, 364u32, 2124420u32)?;
    emu.sw_no_count(11usize, 2usize, 368u32, 2124424u32)?;
    emu.sw_no_count(12usize, 2usize, 372u32, 2124428u32)?;
    emu.sw_no_count(13usize, 2usize, 376u32, 2124432u32)?;
    emu.adi_no_count(10usize, 2usize, 268u32, 2124436u32);
    emu.adi_no_count(11usize, 2usize, 364u32, 2124440u32);
    emu.adi_no_count(12usize, 2usize, 556u32, 2124444u32);
    emu.apc_no_count(1usize, 2124444u32, 12288u32, 2124448u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2124452u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(408u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00206aa4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 508u32, 2124456u32)?;
    emu.lw_no_count(11usize, 2usize, 512u32, 2124460u32)?;
    emu.lw_no_count(12usize, 2usize, 516u32, 2124464u32)?;
    emu.lw_no_count(13usize, 2usize, 520u32, 2124468u32)?;
    emu.sw_no_count(10usize, 2usize, 380u32, 2124472u32)?;
    emu.sw_no_count(11usize, 2usize, 384u32, 2124476u32)?;
    emu.sw_no_count(12usize, 2usize, 388u32, 2124480u32)?;
    emu.sw_no_count(13usize, 2usize, 392u32, 2124484u32)?;
    emu.lw_no_count(10usize, 2usize, 492u32, 2124488u32)?;
    emu.lw_no_count(11usize, 2usize, 496u32, 2124492u32)?;
    emu.lw_no_count(12usize, 2usize, 500u32, 2124496u32)?;
    emu.lw_no_count(13usize, 2usize, 504u32, 2124500u32)?;
    emu.sw_no_count(10usize, 2usize, 364u32, 2124504u32)?;
    emu.sw_no_count(11usize, 2usize, 368u32, 2124508u32)?;
    emu.sw_no_count(12usize, 2usize, 372u32, 2124512u32)?;
    emu.sw_no_count(13usize, 2usize, 376u32, 2124516u32)?;
    emu.adi_no_count(10usize, 2usize, 300u32, 2124520u32);
    emu.adi_no_count(11usize, 2usize, 364u32, 2124524u32);
    emu.adi_no_count(12usize, 2usize, 556u32, 2124528u32);
    emu.apc_no_count(1usize, 2124528u32, 12288u32, 2124532u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2124536u32;
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
pub fn block_0x00206af8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(9usize, 2usize, 232u32, 2124540u32);
    emu.sb_no_count(0usize, 2usize, 332u32, 2124544u32);
    emu.adi_no_count(10usize, 2usize, 364u32, 2124548u32);
    emu.adi_no_count(11usize, 2usize, 268u32, 2124552u32);
    emu.adi_no_count(12usize, 0usize, 68u32, 2124556u32);
    emu.apc_no_count(1usize, 2124556u32, 8192u32, 2124560u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2124564u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(552u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206b14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(9usize, 2usize, 432u32, 2124568u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2124572u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965292u32, 2124576u32);
    emu.adi_no_count(10usize, 2usize, 268u32, 2124580u32);
    emu.adi_no_count(12usize, 0usize, 68u32, 2124584u32);
    emu.apc_no_count(1usize, 2124584u32, 8192u32, 2124588u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2124592u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(524u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206b30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 200u32, 2124596u32);
    emu.adi_no_count(11usize, 2usize, 268u32, 2124600u32);
    emu.adi_no_count(12usize, 2usize, 364u32, 2124604u32);
    emu.adi_no_count(13usize, 9usize, 0u32, 2124608u32);
    emu.apc_no_count(1usize, 2124608u32, 40960u32, 2124612u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2124616u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(588u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206b48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 2usize, 300u32, 2124620u32);
    emu.adi_no_count(12usize, 2usize, 396u32, 2124624u32);
    emu.adi_no_count(10usize, 2usize, 232u32, 2124628u32);
    emu.adi_no_count(13usize, 9usize, 0u32, 2124632u32);
    emu.apc_no_count(1usize, 2124632u32, 40960u32, 2124636u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2124640u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(564u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00206b60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 332u32, 2124644u32);
    emu.lbu_no_count(11usize, 2usize, 428u32, 2124648u32);
    emu.sbr_no_count(12usize, 0usize, 9usize, 2124652u32);
    emu.xrr_no_count(11usize, 11usize, 10usize, 2124656u32);
    emu.anr_no_count(11usize, 11usize, 12usize, 2124660u32);
    emu.xrr_no_count(10usize, 11usize, 10usize, 2124664u32);
    emu.sb_no_count(10usize, 2usize, 264u32, 2124668u32);
    emu.adi_no_count(10usize, 2usize, 364u32, 2124672u32);
    emu.adi_no_count(11usize, 2usize, 200u32, 2124676u32);
    emu.apc_no_count(1usize, 2124676u32, 40960u32, 2124680u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2124684u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965544u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206b8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 460u32, 2124688u32);
    emu.adi_no_count(11usize, 2usize, 364u32, 2124692u32);
    emu.apc_no_count(1usize, 2124692u32, 24576u32, 2124696u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2124700u32;
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
pub fn block_0x00206b9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 40u32, 2124704u32);
    emu.adi_no_count(11usize, 2usize, 460u32, 2124708u32);
    emu.apc_no_count(1usize, 2124708u32, 24576u32, 2124712u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2124716u32;
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
pub fn block_0x00206bac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2124720u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2124732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206bbc));
    } else {
        emu.pc = 2124724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206bb4));
    }
}
#[inline(always)]
pub fn block_0x00206bb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 8usize, 4u32, 2124728u32)?;
    emu.adi_no_count(11usize, 0usize, 1u32, 2124732u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2124732u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00206bbc));
}
#[inline(always)]
pub fn block_0x00206bbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 8usize, 0u32, 2124736u32)?;
    emu.lw_no_count(1usize, 2usize, 636u32, 2124740u32)?;
    emu.lw_no_count(8usize, 2usize, 632u32, 2124744u32)?;
    emu.lw_no_count(9usize, 2usize, 628u32, 2124748u32)?;
    emu.lw_no_count(18usize, 2usize, 624u32, 2124752u32)?;
    emu.lw_no_count(19usize, 2usize, 620u32, 2124756u32)?;
    emu.adi_no_count(2usize, 2usize, 640u32, 2124760u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2124764u32;
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
pub fn block_0x00206bdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 2usize, 460u32, 2124768u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2124772u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 625u32, 2124776u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2124780u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 628u32, 2124784u32);
    emu.adi_no_count(11usize, 2usize, 364u32, 2124788u32);
    emu.adi_no_count(13usize, 2usize, 460u32, 2124792u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2124796u32);
    emu.apc_no_count(1usize, 2124796u32, 12288u32, 2124800u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2124804u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965964u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00206c04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 2usize, 364u32, 2124808u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2124812u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965552u32, 2124816u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2124820u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965556u32, 2124824u32);
    emu.adi_no_count(11usize, 2usize, 268u32, 2124828u32);
    emu.adi_no_count(13usize, 2usize, 364u32, 2124832u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2124836u32);
    emu.apc_no_count(1usize, 2124836u32, 12288u32, 2124840u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2124844u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965924u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
