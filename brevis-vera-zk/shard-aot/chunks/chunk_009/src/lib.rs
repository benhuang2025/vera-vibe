pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2140764u32;
pub const PC_MAX: u32 = 2143132u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 118usize] = [
        block_0x0020aa5c,
        block_0x0020aa6c,
        block_0x0020aa78,
        block_0x0020aa94,
        block_0x0020aaa4,
        block_0x0020aae8,
        block_0x0020ab3c,
        block_0x0020ab48,
        block_0x0020ab80,
        block_0x0020ab84,
        block_0x0020ab88,
        block_0x0020aba0,
        block_0x0020aba4,
        block_0x0020abc0,
        block_0x0020abd4,
        block_0x0020abd8,
        block_0x0020abdc,
        block_0x0020ac08,
        block_0x0020ac28,
        block_0x0020ac48,
        block_0x0020ac54,
        block_0x0020ac74,
        block_0x0020aca4,
        block_0x0020acb8,
        block_0x0020acd0,
        block_0x0020ace0,
        block_0x0020ace8,
        block_0x0020acf8,
        block_0x0020ad08,
        block_0x0020ad28,
        block_0x0020ad30,
        block_0x0020ad4c,
        block_0x0020ad58,
        block_0x0020ad9c,
        block_0x0020adac,
        block_0x0020adbc,
        block_0x0020adc4,
        block_0x0020add4,
        block_0x0020adec,
        block_0x0020ae0c,
        block_0x0020ae20,
        block_0x0020ae48,
        block_0x0020ae54,
        block_0x0020ae88,
        block_0x0020ae90,
        block_0x0020ae94,
        block_0x0020aea0,
        block_0x0020aea4,
        block_0x0020aec8,
        block_0x0020af48,
        block_0x0020af50,
        block_0x0020af5c,
        block_0x0020af74,
        block_0x0020af84,
        block_0x0020af8c,
        block_0x0020af90,
        block_0x0020af94,
        block_0x0020afa8,
        block_0x0020afb0,
        block_0x0020afb8,
        block_0x0020afc8,
        block_0x0020afdc,
        block_0x0020b030,
        block_0x0020b03c,
        block_0x0020b040,
        block_0x0020b050,
        block_0x0020b058,
        block_0x0020b060,
        block_0x0020b070,
        block_0x0020b084,
        block_0x0020b0b4,
        block_0x0020b0cc,
        block_0x0020b0e0,
        block_0x0020b0f0,
        block_0x0020b0f8,
        block_0x0020b100,
        block_0x0020b118,
        block_0x0020b124,
        block_0x0020b130,
        block_0x0020b168,
        block_0x0020b194,
        block_0x0020b198,
        block_0x0020b19c,
        block_0x0020b1ac,
        block_0x0020b1b0,
        block_0x0020b1c0,
        block_0x0020b1dc,
        block_0x0020b1e4,
        block_0x0020b208,
        block_0x0020b21c,
        block_0x0020b234,
        block_0x0020b244,
        block_0x0020b24c,
        block_0x0020b250,
        block_0x0020b264,
        block_0x0020b270,
        block_0x0020b278,
        block_0x0020b27c,
        block_0x0020b28c,
        block_0x0020b2a8,
        block_0x0020b2b0,
        block_0x0020b2d0,
        block_0x0020b2e8,
        block_0x0020b304,
        block_0x0020b314,
        block_0x0020b31c,
        block_0x0020b330,
        block_0x0020b334,
        block_0x0020b344,
        block_0x0020b348,
        block_0x0020b350,
        block_0x0020b360,
        block_0x0020b364,
        block_0x0020b370,
        block_0x0020b374,
        block_0x0020b378,
        block_0x0020b394,
        block_0x0020b39c,
    ];
    const IDX: [u16; 593usize] = [
        1u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 3u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 4u16, 0u16, 0u16, 0u16, 5u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 6u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 7u16, 0u16, 0u16, 8u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 9u16, 10u16, 11u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 12u16, 13u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 14u16, 0u16,
        0u16, 0u16, 0u16, 15u16, 16u16, 17u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 18u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 19u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 20u16, 0u16, 0u16, 21u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 22u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 23u16, 0u16, 0u16, 0u16, 0u16, 24u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 25u16, 0u16, 0u16, 0u16, 26u16, 0u16, 27u16, 0u16, 0u16, 0u16, 28u16, 0u16,
        0u16, 0u16, 29u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 30u16, 0u16, 31u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 32u16, 0u16, 0u16, 33u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        34u16, 0u16, 0u16, 0u16, 35u16, 0u16, 0u16, 0u16, 36u16, 0u16, 37u16, 0u16, 0u16,
        0u16, 38u16, 0u16, 0u16, 0u16, 0u16, 0u16, 39u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 40u16, 0u16, 0u16, 0u16, 0u16, 41u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 42u16, 0u16, 0u16, 43u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 44u16, 0u16, 45u16, 46u16, 0u16, 0u16,
        47u16, 48u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 49u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 50u16, 0u16, 51u16, 0u16, 0u16, 52u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 53u16, 0u16, 0u16, 0u16, 54u16, 0u16, 55u16, 56u16, 57u16, 0u16, 0u16,
        0u16, 0u16, 58u16, 0u16, 59u16, 0u16, 60u16, 0u16, 0u16, 0u16, 61u16, 0u16, 0u16,
        0u16, 0u16, 62u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 63u16, 0u16, 0u16,
        64u16, 65u16, 0u16, 0u16, 0u16, 66u16, 0u16, 67u16, 0u16, 68u16, 0u16, 0u16,
        0u16, 69u16, 0u16, 0u16, 0u16, 0u16, 70u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 71u16, 0u16, 0u16, 0u16, 0u16, 0u16, 72u16, 0u16,
        0u16, 0u16, 0u16, 73u16, 0u16, 0u16, 0u16, 74u16, 0u16, 75u16, 0u16, 76u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 77u16, 0u16, 0u16, 78u16, 0u16, 0u16, 79u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 80u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 81u16, 82u16, 83u16, 0u16,
        0u16, 0u16, 84u16, 85u16, 0u16, 0u16, 0u16, 86u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 87u16, 0u16, 88u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 89u16,
        0u16, 0u16, 0u16, 0u16, 90u16, 0u16, 0u16, 0u16, 0u16, 0u16, 91u16, 0u16, 0u16,
        0u16, 92u16, 0u16, 93u16, 94u16, 0u16, 0u16, 0u16, 0u16, 95u16, 0u16, 0u16,
        96u16, 0u16, 97u16, 98u16, 0u16, 0u16, 0u16, 99u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 100u16, 0u16, 101u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 102u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 103u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 104u16,
        0u16, 0u16, 0u16, 105u16, 0u16, 106u16, 0u16, 0u16, 0u16, 0u16, 107u16, 108u16,
        0u16, 0u16, 0u16, 109u16, 110u16, 0u16, 111u16, 0u16, 0u16, 0u16, 112u16, 113u16,
        0u16, 0u16, 114u16, 115u16, 116u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 117u16,
        0u16, 118u16,
    ];
    if pc < 2140764u32 || pc > 2143132u32 {
        return None;
    }
    let word_offset = ((pc - 2140764u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x0020aa5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2140768u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2140772u32)?;
    emu.apc_no_count(1usize, 2140772u32, 4294959104u32, 2140776u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140780u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966932u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020aa6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2140784u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2140788u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140792u32;
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
pub fn block_0x0020aa78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2140796u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2140800u32;
    emu.update_insn_clock();
    emu.lbu_no_count(12usize, 11usize, 4294966465u32, 2140804u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2140808u32);
    emu.sb_no_count(12usize, 2usize, 7u32, 2140812u32);
    emu.sb_no_count(10usize, 11usize, 4294966465u32, 2140816u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2140836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aaa4));
    } else {
        emu.pc = 2140820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aa94));
    }
}
#[inline(always)]
pub fn block_0x0020aa94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2140824u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966465u32, 2140828u32);
    emu.adi_no_count(2usize, 2usize, 32u32, 2140832u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140836u32;
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
pub fn block_0x0020aaa4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2140840u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967284u32, 2140844u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2140848u32);
    emu.sw_no_count(11usize, 2usize, 8u32, 2140852u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2140856u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2140860u32)?;
    emu.sw_no_count(0usize, 2usize, 20u32, 2140864u32)?;
    emu.sw_no_count(0usize, 2usize, 24u32, 2140868u32)?;
    let a = 0u32.wrapping_add(2183168u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2140872u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1272u32, 2140876u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2140880u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294967292u32, 2140884u32);
    emu.adi_no_count(11usize, 2usize, 7u32, 2140888u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2140892u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2140896u32);
    emu.apc_no_count(1usize, 2140896u32, 0u32, 2140900u32);
    emu.add_memory_rw_events(17usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140904u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(404u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020aae8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2140908u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2140912u32)?;
    emu.adi_no_count(11usize, 12usize, 0u32, 2140916u32);
    emu.sb_no_count(14usize, 2usize, 43u32, 2140920u32);
    emu.adi_no_count(12usize, 2usize, 43u32, 2140924u32);
    let a = 0u32.wrapping_add(2142208u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2140928u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966088u32, 2140932u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2140936u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 1496u32, 2140940u32);
    emu.adi_no_count(16usize, 0usize, 1u32, 2140944u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2140948u32)?;
    emu.sw_no_count(12usize, 2usize, 32u32, 2140952u32)?;
    emu.sw_no_count(14usize, 2usize, 36u32, 2140956u32)?;
    emu.adi_no_count(12usize, 2usize, 32u32, 2140960u32);
    emu.lw_no_count(13usize, 13usize, 36u32, 2140964u32)?;
    emu.sw_no_count(15usize, 2usize, 8u32, 2140968u32)?;
    emu.sw_no_count(16usize, 2usize, 12u32, 2140972u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2140976u32)?;
    emu.sw_no_count(16usize, 2usize, 20u32, 2140980u32)?;
    emu.adi_no_count(12usize, 2usize, 8u32, 2140984u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2140988u32;
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
pub fn block_0x0020ab3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2140992u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2140996u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2141000u32;
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
pub fn block_0x0020ab48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2141004u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2141008u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2141012u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2141016u32)?;
    emu.sw_no_count(18usize, 2usize, 0u32, 2141020u32)?;
    emu.lw_no_count(12usize, 11usize, 4u32, 2141024u32)?;
    emu.lw_no_count(8usize, 11usize, 0u32, 2141028u32)?;
    emu.lbu_no_count(18usize, 10usize, 0u32, 2141032u32);
    emu.lw_no_count(9usize, 12usize, 12u32, 2141036u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2141040u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967096u32, 2141044u32);
    emu.adi_no_count(12usize, 0usize, 17u32, 2141048u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2141052u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(9usize);
    let return_addr = 2141056u32;
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
pub fn block_0x0020ab80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2141088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aba0));
    } else {
        emu.pc = 2141060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ab84));
    }
}
#[inline(always)]
pub fn block_0x0020ab84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2141064u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2141064u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ab88));
}
#[inline(always)]
pub fn block_0x0020ab88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2141068u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2141072u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2141076u32)?;
    emu.lw_no_count(18usize, 2usize, 0u32, 2141080u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2141084u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2141088u32;
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
pub fn block_0x0020aba0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2141120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020abc0));
    } else {
        emu.pc = 2141092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aba4));
    }
}
#[inline(always)]
pub fn block_0x0020aba4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2141096u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2141100u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2141104u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2141108u32)?;
    emu.lw_no_count(18usize, 2usize, 0u32, 2141112u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2141116u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2141120u32;
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
pub fn block_0x0020abc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2141124u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967113u32, 2141128u32);
    emu.adi_no_count(12usize, 0usize, 88u32, 2141132u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2141136u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(9usize);
    let return_addr = 2141140u32;
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
pub fn block_0x0020abd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2141060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ab84));
    } else {
        emu.pc = 2141144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020abd8));
    }
}
#[inline(always)]
pub fn block_0x0020abd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2141148u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2141064u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ab88));
}
#[inline]
pub fn block_0x0020abdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2141152u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2141156u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2141160u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2141164u32)?;
    emu.adi_no_count(8usize, 13usize, 0u32, 2141168u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2141172u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2141176u32);
    emu.adi_no_count(10usize, 0usize, 2u32, 2141180u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2141184u32);
    emu.apc_no_count(1usize, 2141184u32, 4294934528u32, 2141188u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2141192u32;
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
pub fn block_0x0020ac08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2141196u32);
    emu.sb_no_count(10usize, 9usize, 0u32, 2141200u32);
    emu.sw_no_count(8usize, 9usize, 4u32, 2141204u32)?;
    emu.lw_no_count(1usize, 2usize, 12u32, 2141208u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2141212u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2141216u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2141220u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2141224u32;
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
pub fn block_0x0020ac28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2141228u32)?;
    emu.lw_no_count(13usize, 10usize, 4u32, 2141232u32)?;
    emu.adi_no_count(14usize, 11usize, 0u32, 2141236u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2141240u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2141244u32);
    emu.adi_no_count(12usize, 14usize, 0u32, 2141248u32);
    emu.apc_no_count(6usize, 2141248u32, 24576u32, 2141252u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2141256u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1564u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020ac48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2141260u32)?;
    emu.apc_no_count(6usize, 2141260u32, 24576u32, 2141264u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2141268u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1500u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020ac54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2141272u32)?;
    emu.lw_no_count(13usize, 10usize, 4u32, 2141276u32)?;
    emu.adi_no_count(14usize, 11usize, 0u32, 2141280u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2141284u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2141288u32);
    emu.adi_no_count(12usize, 14usize, 0u32, 2141292u32);
    emu.apc_no_count(6usize, 2141292u32, 28672u32, 2141296u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2141300u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965712u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020ac74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2141304u32);
    emu.adi_no_count(16usize, 14usize, 0u32, 2141308u32);
    emu.adi_no_count(15usize, 13usize, 0u32, 2141312u32);
    emu.sw_no_count(11usize, 2usize, 8u32, 2141316u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2141320u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2141324u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967204u32, 2141328u32);
    emu.adi_no_count(11usize, 2usize, 8u32, 2141332u32);
    emu.adi_no_count(13usize, 2usize, 12u32, 2141336u32);
    emu.adi_no_count(14usize, 12usize, 0u32, 2141340u32);
    emu.apc_no_count(1usize, 2141340u32, 8192u32, 2141344u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2141348u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965412u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020aca4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2141352u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2141356u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2141360u32)?;
    emu.lw_no_count(6usize, 12usize, 12u32, 2141364u32)?;
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2141368u32;
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
pub fn block_0x0020acb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2141372u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2141376u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2141380u32)?;
    emu.lw_no_count(8usize, 10usize, 0u32, 2141384u32)?;
    emu.lw_no_count(11usize, 8usize, 12u32, 2141388u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2141408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ace0));
    } else {
        emu.pc = 2141392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020acd0));
    }
}
#[inline(always)]
pub fn block_0x0020acd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 16u32, 2141396u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2141400u32);
    emu.apc_no_count(1usize, 2141400u32, 4294926336u32, 2141404u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2141408u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1120u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020ace0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4294967295u32, 2141412u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2141432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020acf8));
    } else {
        emu.pc = 2141416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ace8));
    }
}
#[inline(always)]
pub fn block_0x0020ace8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 4u32, 2141420u32)?;
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2141424u32);
    emu.sw_no_count(10usize, 8usize, 4u32, 2141428u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2141448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ad08));
    } else {
        emu.pc = 2141432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020acf8));
    }
}
#[inline(always)]
pub fn block_0x0020acf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2141436u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2141440u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2141444u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2141448u32;
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
pub fn block_0x0020ad08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 24u32, 2141452u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2141456u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2141460u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2141464u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2141468u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2141472u32);
    emu.apc_no_count(6usize, 2141472u32, 4294926336u32, 2141476u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2141480u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1048u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020ad28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2141480u32, 4294959104u32, 2141484u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2141488u32;
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
pub fn block_0x0020ad30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2141492u32);
    emu.adi_no_count(11usize, 10usize, 0u32, 2141496u32);
    emu.lbu_no_count(12usize, 10usize, 0u32, 2141500u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2141504u32);
    emu.sb_no_count(12usize, 2usize, 7u32, 2141508u32);
    emu.sb_no_count(10usize, 11usize, 0u32, 2141512u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2141528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ad58));
    } else {
        emu.pc = 2141516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ad4c));
    }
}
#[inline(always)]
pub fn block_0x0020ad4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2141520u32);
    emu.adi_no_count(2usize, 2usize, 32u32, 2141524u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2141528u32;
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
pub fn block_0x0020ad58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2141532u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967284u32, 2141536u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2141540u32);
    emu.sw_no_count(11usize, 2usize, 8u32, 2141544u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2141548u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2141552u32)?;
    emu.sw_no_count(0usize, 2usize, 20u32, 2141556u32)?;
    emu.sw_no_count(0usize, 2usize, 24u32, 2141560u32)?;
    let a = 0u32.wrapping_add(2183168u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2141564u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1272u32, 2141568u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2141572u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294967292u32, 2141576u32);
    emu.adi_no_count(11usize, 2usize, 7u32, 2141580u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2141584u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2141588u32);
    emu.apc_no_count(1usize, 2141588u32, 0u32, 2141592u32);
    emu.add_memory_rw_events(17usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2141596u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967008u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020ad9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 4u32, 2141600u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2141604u32)?;
    emu.lw_no_count(6usize, 12usize, 12u32, 2141608u32)?;
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2141612u32;
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
pub fn block_0x0020adac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2141616u32;
    emu.update_insn_clock();
    emu.lw_no_count(12usize, 11usize, 4294966480u32, 2141620u32)?;
    emu.adi_no_count(11usize, 0usize, 2u32, 2141624u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2141652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020add4));
    } else {
        emu.pc = 2141628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020adbc));
    }
}
#[inline(always)]
pub fn block_0x0020adbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 12usize, 8u32, 2141632u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2141728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ae20));
    } else {
        emu.pc = 2141636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020adc4));
    }
}
#[inline(always)]
pub fn block_0x0020adc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 12usize, 12u32, 2141640u32)?;
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2141644u32);
    emu.apc_no_count(6usize, 2141644u32, 0u32, 2141648u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2141652u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(136u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020add4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2141656u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966480u32, 2141660u32);
    emu.lw_no_count(11usize, 12usize, 8u32, 2141664u32)?;
    emu.lw_no_count(12usize, 12usize, 12u32, 2141668u32)?;
    emu.orr_no_count(13usize, 11usize, 12usize, 2141672u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2141768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ae48));
    } else {
        emu.pc = 2141676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020adec));
    }
}
#[inline(always)]
pub fn block_0x0020adec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2141680u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2141684u32;
    emu.update_insn_clock();
    emu.lw_no_count(14usize, 14usize, 4294966472u32, 2141688u32)?;
    emu.lw_no_count(13usize, 13usize, 4294966468u32, 2141692u32)?;
    emu.xrr_no_count(12usize, 14usize, 12usize, 2141696u32);
    emu.xrr_no_count(11usize, 13usize, 11usize, 2141700u32);
    emu.orr_no_count(11usize, 11usize, 12usize, 2141704u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2141768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ae48));
    } else {
        emu.pc = 2141708u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ae0c));
    }
}
#[inline(always)]
pub fn block_0x0020ae0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2141712u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967276u32, 2141716u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2141720u32);
    emu.apc_no_count(6usize, 2141720u32, 0u32, 2141724u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2141728u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(60u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020ae20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 12usize, 0u32, 2141732u32)?;
    emu.lw_no_count(12usize, 12usize, 4u32, 2141736u32)?;
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2141740u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966480u32, 2141744u32);
    emu.lw_no_count(14usize, 13usize, 12u32, 2141748u32)?;
    emu.lw_no_count(13usize, 13usize, 8u32, 2141752u32)?;
    emu.xrr_no_count(12usize, 12usize, 14usize, 2141756u32);
    emu.xrr_no_count(11usize, 11usize, 13usize, 2141760u32);
    emu.orr_no_count(11usize, 11usize, 12usize, 2141764u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2141708u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ae0c));
    } else {
        emu.pc = 2141768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ae48));
    }
}
#[inline(always)]
pub fn block_0x0020ae48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2141772u32);
    emu.apc_no_count(6usize, 2141772u32, 0u32, 2141776u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2141780u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(8u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020ae54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294966656u32, 2141784u32);
    emu.sw_no_count(1usize, 2usize, 636u32, 2141788u32)?;
    emu.sw_no_count(8usize, 2usize, 632u32, 2141792u32)?;
    emu.sw_no_count(9usize, 2usize, 628u32, 2141796u32)?;
    emu.sw_no_count(18usize, 2usize, 624u32, 2141800u32)?;
    emu.sw_no_count(19usize, 2usize, 620u32, 2141804u32)?;
    emu.sw_no_count(20usize, 2usize, 616u32, 2141808u32)?;
    emu.sw_no_count(21usize, 2usize, 612u32, 2141812u32)?;
    emu.sw_no_count(22usize, 2usize, 608u32, 2141816u32)?;
    emu.sw_no_count(23usize, 2usize, 604u32, 2141820u32)?;
    emu.sw_no_count(24usize, 2usize, 600u32, 2141824u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2141828u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2141844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ae94));
    } else {
        emu.pc = 2141832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ae88));
    }
}
#[inline(always)]
pub fn block_0x0020ae88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 11usize, 0u32, 2141836u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2141856u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aea0));
    } else {
        emu.pc = 2141840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ae90));
    }
}
#[inline(always)]
pub fn block_0x0020ae90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2141844u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2141860u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020aea4));
}
#[inline(always)]
pub fn block_0x0020ae94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2141848u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 12u32, 2141852u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2141860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aea4));
    } else {
        emu.pc = 2141856u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aea0));
    }
}
#[inline(always)]
pub fn block_0x0020aea0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 9u32, 2141860u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2141860u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020aea4));
}
#[inline]
pub fn block_0x0020aea4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 8u32, 2141864u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2141868u32)?;
    emu.adi_no_count(9usize, 2usize, 16u32, 2141872u32);
    emu.adi_no_count(10usize, 2usize, 16u32, 2141876u32);
    emu.adi_no_count(12usize, 0usize, 512u32, 2141880u32);
    emu.adi_no_count(18usize, 0usize, 512u32, 2141884u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2141888u32);
    emu.apc_no_count(1usize, 2141888u32, 4294934528u32, 2141892u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2141896u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(188u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0020aec8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 32u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(9usize, 2usize, 528u32, 2141900u32)?;
    emu.sw_no_count(18usize, 2usize, 532u32, 2141904u32)?;
    emu.sw_no_count(0usize, 2usize, 536u32, 2141908u32)?;
    emu.sw_no_count(0usize, 2usize, 540u32, 2141912u32)?;
    emu.lw_no_count(21usize, 8usize, 0u32, 2141916u32)?;
    emu.lw_no_count(19usize, 8usize, 4u32, 2141920u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2141924u32);
    let a = 0u32.wrapping_add(2142208u32);
    emu.write_reg_no_count(20usize, a);
    emu.pc = 2141928u32;
    emu.update_insn_clock();
    emu.adi_no_count(20usize, 20usize, 4294966356u32, 2141932u32);
    let a = 0u32.wrapping_add(2134016u32);
    emu.write_reg_no_count(23usize, a);
    emu.pc = 2141936u32;
    emu.update_insn_clock();
    emu.adi_no_count(23usize, 23usize, 4294967068u32, 2141940u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(22usize, a);
    emu.pc = 2141944u32;
    emu.update_insn_clock();
    emu.adi_no_count(22usize, 22usize, 68u32, 2141948u32);
    emu.adi_no_count(18usize, 0usize, 4u32, 2141952u32);
    emu.sw_no_count(0usize, 2usize, 568u32, 2141956u32)?;
    emu.adi_no_count(11usize, 2usize, 576u32, 2141960u32);
    emu.adi_no_count(9usize, 0usize, 3u32, 2141964u32);
    emu.sw_no_count(10usize, 2usize, 576u32, 2141968u32)?;
    emu.sw_no_count(20usize, 2usize, 580u32, 2141972u32)?;
    emu.sw_no_count(21usize, 2usize, 584u32, 2141976u32)?;
    emu.sw_no_count(23usize, 2usize, 588u32, 2141980u32)?;
    emu.sw_no_count(19usize, 2usize, 592u32, 2141984u32)?;
    emu.sw_no_count(20usize, 2usize, 596u32, 2141988u32)?;
    emu.sw_no_count(22usize, 2usize, 552u32, 2141992u32)?;
    emu.sw_no_count(18usize, 2usize, 556u32, 2141996u32)?;
    emu.sw_no_count(11usize, 2usize, 560u32, 2142000u32)?;
    emu.sw_no_count(9usize, 2usize, 564u32, 2142004u32)?;
    emu.adi_no_count(10usize, 2usize, 544u32, 2142008u32);
    emu.adi_no_count(11usize, 2usize, 528u32, 2142012u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2142016u32);
    emu.apc_no_count(1usize, 2142016u32, 4294959104u32, 2142020u32);
    emu.add_memory_rw_events(32usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2142024u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x0020af48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 544u32, 2142028u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2142096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020af90));
    } else {
        emu.pc = 2142032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020af50));
    }
}
#[inline(always)]
pub fn block_0x0020af50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 2usize, 536u32, 2142036u32)?;
    emu.adi_no_count(10usize, 0usize, 513u32, 2142040u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2142388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b0b4));
    } else {
        emu.pc = 2142044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020af5c));
    }
}
#[inline(always)]
pub fn block_0x0020af5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 12u32, 2142048u32)?;
    emu.lw_no_count(11usize, 8usize, 8u32, 2142052u32)?;
    emu.lw_no_count(14usize, 10usize, 28u32, 2142056u32)?;
    emu.adi_no_count(10usize, 2usize, 576u32, 2142060u32);
    emu.adi_no_count(12usize, 2usize, 16u32, 2142064u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2142068u32;
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
pub fn block_0x0020af74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 576u32, 2142072u32);
    emu.lw_no_count(8usize, 2usize, 580u32, 2142076u32)?;
    emu.adi_no_count(11usize, 0usize, 4u32, 2142080u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2142272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b040));
    } else {
        emu.pc = 2142084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020af84));
    }
}
#[inline(always)]
pub fn block_0x0020af84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 3u32, 2142088u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2142340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b084));
    } else {
        emu.pc = 2142092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020af8c));
    }
}
#[inline(always)]
pub fn block_0x0020af8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2142096u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2142272u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b040));
}
#[inline(always)]
pub fn block_0x0020af90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2142172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020afdc));
    } else {
        emu.pc = 2142100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020af94));
    }
}
#[inline(always)]
pub fn block_0x0020af94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(9usize, 2usize, 548u32, 2142104u32)?;
    emu.lw_no_count(24usize, 9usize, 4u32, 2142108u32)?;
    emu.lw_no_count(11usize, 24usize, 0u32, 2142112u32)?;
    emu.lw_no_count(18usize, 9usize, 0u32, 2142116u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2142128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020afb0));
    } else {
        emu.pc = 2142120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020afa8));
    }
}
#[inline(always)]
pub fn block_0x0020afa8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2142124u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2142128u32;
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
pub fn block_0x0020afb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 24usize, 4u32, 2142132u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2142152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020afc8));
    } else {
        emu.pc = 2142136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020afb8));
    }
}
#[inline(always)]
pub fn block_0x0020afb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 24usize, 8u32, 2142140u32)?;
    emu.adi_no_count(10usize, 18usize, 0u32, 2142144u32);
    emu.apc_no_count(1usize, 2142144u32, 4294926336u32, 2142148u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2142152u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(376u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020afc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 12u32, 2142156u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2142160u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2142164u32);
    emu.apc_no_count(1usize, 2142164u32, 4294926336u32, 2142168u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2142172u32;
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
#[inline]
pub fn block_0x0020afdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 8usize, 8u32, 2142176u32)?;
    emu.lw_no_count(10usize, 8usize, 12u32, 2142180u32)?;
    emu.adi_no_count(12usize, 2usize, 8u32, 2142184u32);
    emu.adi_no_count(18usize, 0usize, 4u32, 2142188u32);
    emu.sw_no_count(0usize, 2usize, 568u32, 2142192u32)?;
    emu.adi_no_count(13usize, 2usize, 576u32, 2142196u32);
    emu.lw_no_count(14usize, 10usize, 36u32, 2142200u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2142204u32)?;
    emu.sw_no_count(20usize, 2usize, 580u32, 2142208u32)?;
    emu.sw_no_count(21usize, 2usize, 584u32, 2142212u32)?;
    emu.sw_no_count(23usize, 2usize, 588u32, 2142216u32)?;
    emu.adi_no_count(9usize, 0usize, 3u32, 2142220u32);
    emu.sw_no_count(19usize, 2usize, 592u32, 2142224u32)?;
    emu.sw_no_count(20usize, 2usize, 596u32, 2142228u32)?;
    emu.sw_no_count(22usize, 2usize, 552u32, 2142232u32)?;
    emu.sw_no_count(18usize, 2usize, 556u32, 2142236u32)?;
    emu.sw_no_count(13usize, 2usize, 560u32, 2142240u32)?;
    emu.sw_no_count(9usize, 2usize, 564u32, 2142244u32)?;
    emu.adi_no_count(10usize, 2usize, 544u32, 2142248u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2142252u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2142256u32;
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
pub fn block_0x0020b030(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 544u32, 2142260u32);
    emu.lw_no_count(8usize, 2usize, 548u32, 2142264u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a < b {
        emu.pc = 2142272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b040));
    } else {
        emu.pc = 2142268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b03c));
    }
}
#[inline(always)]
pub fn block_0x0020b03c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2142340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b084));
    } else {
        emu.pc = 2142272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b040));
    }
}
#[inline(always)]
pub fn block_0x0020b040(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 8usize, 4u32, 2142276u32)?;
    emu.lw_no_count(11usize, 18usize, 0u32, 2142280u32)?;
    emu.lw_no_count(9usize, 8usize, 0u32, 2142284u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2142296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b058));
    } else {
        emu.pc = 2142288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b050));
    }
}
#[inline(always)]
pub fn block_0x0020b050(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2142292u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2142296u32;
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
pub fn block_0x0020b058(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 18usize, 4u32, 2142300u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2142320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b070));
    } else {
        emu.pc = 2142304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b060));
    }
}
#[inline(always)]
pub fn block_0x0020b060(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 8u32, 2142308u32)?;
    emu.adi_no_count(10usize, 9usize, 0u32, 2142312u32);
    emu.apc_no_count(1usize, 2142312u32, 4294926336u32, 2142316u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2142320u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(208u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020b070(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 12u32, 2142324u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2142328u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2142332u32);
    emu.apc_no_count(1usize, 2142332u32, 4294926336u32, 2142336u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2142340u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(188u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020b084(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 636u32, 2142344u32)?;
    emu.lw_no_count(8usize, 2usize, 632u32, 2142348u32)?;
    emu.lw_no_count(9usize, 2usize, 628u32, 2142352u32)?;
    emu.lw_no_count(18usize, 2usize, 624u32, 2142356u32)?;
    emu.lw_no_count(19usize, 2usize, 620u32, 2142360u32)?;
    emu.lw_no_count(20usize, 2usize, 616u32, 2142364u32)?;
    emu.lw_no_count(21usize, 2usize, 612u32, 2142368u32)?;
    emu.lw_no_count(22usize, 2usize, 608u32, 2142372u32)?;
    emu.lw_no_count(23usize, 2usize, 604u32, 2142376u32)?;
    emu.lw_no_count(24usize, 2usize, 600u32, 2142380u32)?;
    emu.adi_no_count(2usize, 2usize, 640u32, 2142384u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2142388u32;
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
pub fn block_0x0020b0b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2142392u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 24u32, 2142396u32);
    emu.adi_no_count(11usize, 0usize, 512u32, 2142400u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2142404u32);
    emu.apc_no_count(1usize, 2142404u32, 36864u32, 2142408u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2142412u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966188u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020b0cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2142416u32;
    emu.update_insn_clock();
    emu.lw_no_count(12usize, 11usize, 4294966496u32, 2142420u32)?;
    emu.adi_no_count(13usize, 12usize, 1u32, 2142424u32);
    emu.sw_no_count(13usize, 11usize, 4294966496u32, 2142428u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2142456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b0f8));
    } else {
        emu.pc = 2142432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b0e0));
    }
}
#[inline(always)]
pub fn block_0x0020b0e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2142436u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966496u32, 2142440u32);
    emu.lbu_no_count(12usize, 11usize, 8u32, 2142444u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2142464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b100));
    } else {
        emu.pc = 2142448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b0f0));
    }
}
#[inline(always)]
pub fn block_0x0020b0f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2142452u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2142456u32;
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
pub fn block_0x0020b0f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2142460u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2142464u32;
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
pub fn block_0x0020b100(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 4u32, 2142468u32)?;
    emu.adi_no_count(12usize, 12usize, 1u32, 2142472u32);
    emu.sw_no_count(12usize, 11usize, 4u32, 2142476u32)?;
    emu.sb_no_count(10usize, 11usize, 8u32, 2142480u32);
    emu.adi_no_count(10usize, 0usize, 2u32, 2142484u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2142488u32;
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
pub fn block_0x0020b118(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2142492u32;
    emu.update_insn_clock();
    emu.sb_no_count(0usize, 10usize, 4294966504u32, 2142496u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2142500u32;
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
pub fn block_0x0020b124(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2142504u32;
    emu.update_insn_clock();
    emu.lw_no_count(10usize, 10usize, 4294966500u32, 2142508u32)?;
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2142512u32;
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
pub fn block_0x0020b130(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2142516u32);
    emu.adi_no_count(11usize, 10usize, 0u32, 2142520u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2142524u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 120u32, 2142528u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2142532u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2142536u32)?;
    emu.adi_no_count(13usize, 0usize, 4u32, 2142540u32);
    emu.sw_no_count(10usize, 2usize, 8u32, 2142544u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2142548u32)?;
    emu.sw_no_count(13usize, 2usize, 16u32, 2142552u32)?;
    emu.sw_no_count(0usize, 2usize, 20u32, 2142556u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2142560u32);
    emu.apc_no_count(1usize, 2142560u32, 4096u32, 2142564u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2142568u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(756u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020b168(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2142572u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2142576u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2142580u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2142584u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2142588u32)?;
    emu.adi_no_count(9usize, 10usize, 0u32, 2142592u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2142596u32)?;
    emu.sli_no_count(18usize, 10usize, 1u32, 2142600u32);
    emu.adi_no_count(12usize, 0usize, 8u32, 2142604u32);
    emu.adi_no_count(8usize, 11usize, 0u32, 2142608u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2142616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b198));
    } else {
        emu.pc = 2142612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b194));
    }
}
#[inline(always)]
pub fn block_0x0020b194(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 8u32, 2142616u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2142616u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b198));
}
#[inline(always)]
pub fn block_0x0020b198(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2142636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b1ac));
    } else {
        emu.pc = 2142620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b19c));
    }
}
#[inline(always)]
pub fn block_0x0020b19c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2142624u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2142628u32);
    emu.apc_no_count(1usize, 2142628u32, 0u32, 2142632u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2142636u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(508u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020b1ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2142656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b1c0));
    } else {
        emu.pc = 2142640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b1b0));
    }
}
#[inline(always)]
pub fn block_0x0020b1b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 9usize, 4u32, 2142644u32)?;
    emu.sw_no_count(11usize, 2usize, 20u32, 2142648u32)?;
    emu.sw_no_count(10usize, 2usize, 28u32, 2142652u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2142656u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2142656u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b1c0));
}
#[inline(always)]
pub fn block_0x0020b1c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 24u32, 2142660u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2142664u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2142668u32);
    emu.adi_no_count(13usize, 2usize, 20u32, 2142672u32);
    emu.adi_no_count(12usize, 18usize, 0u32, 2142676u32);
    emu.apc_no_count(1usize, 2142676u32, 0u32, 2142680u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2142684u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(276u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020b1dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2142688u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2142728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b208));
    } else {
        emu.pc = 2142692u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b1e4));
    }
}
#[inline]
pub fn block_0x0020b1e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2142696u32)?;
    emu.sw_no_count(18usize, 9usize, 0u32, 2142700u32)?;
    emu.sw_no_count(10usize, 9usize, 4u32, 2142704u32)?;
    emu.lw_no_count(1usize, 2usize, 44u32, 2142708u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2142712u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2142716u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2142720u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2142724u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2142728u32;
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
pub fn block_0x0020b208(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2142732u32)?;
    emu.lw_no_count(11usize, 2usize, 16u32, 2142736u32)?;
    emu.adi_no_count(12usize, 8usize, 0u32, 2142740u32);
    emu.apc_no_count(1usize, 2142740u32, 0u32, 2142744u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2142748u32;
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
pub fn block_0x0020b21c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2142752u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2142756u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2142760u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2142764u32)?;
    emu.adr_no_count(9usize, 11usize, 12usize, 2142768u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a < b {
        emu.pc = 2142800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b250));
    } else {
        emu.pc = 2142772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b234));
    }
}
#[inline(always)]
pub fn block_0x0020b234(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 10usize, 0u32, 2142776u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2142780u32)?;
    emu.sli_no_count(11usize, 10usize, 1u32, 2142784u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2142820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b264));
    } else {
        emu.pc = 2142788u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b244));
    }
}
#[inline(always)]
pub fn block_0x0020b244(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 8u32, 2142792u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2142832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b270));
    } else {
        emu.pc = 2142796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b24c));
    }
}
#[inline(always)]
pub fn block_0x0020b24c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2142840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b278));
    } else {
        emu.pc = 2142800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b250));
    }
}
#[inline(always)]
pub fn block_0x0020b250(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2142804u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2142808u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 128u32, 2142812u32);
    emu.apc_no_count(1usize, 2142812u32, 0u32, 2142816u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2142820u32;
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
pub fn block_0x0020b264(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 11usize, 0u32, 2142824u32);
    emu.adi_no_count(11usize, 0usize, 8u32, 2142828u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2142796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b24c));
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
    emu.adi_no_count(9usize, 0usize, 8u32, 2142836u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2142800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b250));
    } else {
        emu.pc = 2142840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b278));
    }
}
#[inline(always)]
pub fn block_0x0020b278(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2142860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b28c));
    } else {
        emu.pc = 2142844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b27c));
    }
}
#[inline(always)]
pub fn block_0x0020b27c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 8usize, 4u32, 2142848u32)?;
    emu.sw_no_count(11usize, 2usize, 24u32, 2142852u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2142856u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2142860u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2142860u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b28c));
}
#[inline(always)]
pub fn block_0x0020b28c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 28u32, 2142864u32)?;
    emu.adi_no_count(10usize, 2usize, 12u32, 2142868u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2142872u32);
    emu.adi_no_count(13usize, 2usize, 24u32, 2142876u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2142880u32);
    emu.apc_no_count(1usize, 2142880u32, 0u32, 2142884u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2142888u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(72u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020b2a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2142892u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2142928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b2d0));
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
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 16u32, 2142900u32)?;
    emu.sw_no_count(9usize, 8usize, 0u32, 2142904u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2142908u32)?;
    emu.lw_no_count(1usize, 2usize, 44u32, 2142912u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2142916u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2142920u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2142924u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2142928u32;
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
pub fn block_0x0020b2d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 16u32, 2142932u32)?;
    emu.lw_no_count(11usize, 2usize, 20u32, 2142936u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2142940u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 128u32, 2142944u32);
    emu.apc_no_count(1usize, 2142944u32, 0u32, 2142948u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2142952u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x0020b2e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2142956u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2142960u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2142964u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2142968u32)?;
    emu.sw_no_count(18usize, 2usize, 0u32, 2142972u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2142976u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2143088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b370));
    } else {
        emu.pc = 2142980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b304));
    }
}
#[inline(always)]
pub fn block_0x0020b304(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 12usize, 0u32, 2142984u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2142988u32);
    emu.lw_no_count(10usize, 13usize, 4u32, 2142992u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2143044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b344));
    } else {
        emu.pc = 2142996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b314));
    }
}
#[inline(always)]
pub fn block_0x0020b314(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 13usize, 8u32, 2143000u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2143044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b344));
    } else {
        emu.pc = 2143004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b31c));
    }
}
#[inline(always)]
pub fn block_0x0020b31c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 13usize, 0u32, 2143008u32)?;
    emu.adi_no_count(12usize, 18usize, 0u32, 2143012u32);
    emu.adi_no_count(13usize, 9usize, 0u32, 2143016u32);
    emu.apc_no_count(1usize, 2143016u32, 4294926336u32, 2143020u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2143024u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966804u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020b330(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2143076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b364));
    } else {
        emu.pc = 2143028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b334));
    }
}
#[inline(always)]
pub fn block_0x0020b334(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2143032u32);
    emu.sw_no_count(10usize, 8usize, 4u32, 2143036u32)?;
    emu.sw_no_count(9usize, 8usize, 8u32, 2143040u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2143044u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2143096u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b378));
}
#[inline(always)]
pub fn block_0x0020b344(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2143124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b394));
    } else {
        emu.pc = 2143048u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b348));
    }
}
#[inline(always)]
pub fn block_0x0020b348(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2143048u32, 4294930432u32, 2143052u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2143056u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(956u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020b350(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2143060u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2143064u32);
    emu.apc_no_count(1usize, 2143064u32, 4294926336u32, 2143068u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2143072u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966724u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020b360(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2143028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b334));
    } else {
        emu.pc = 2143076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b364));
    }
}
#[inline(always)]
pub fn block_0x0020b364(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 8usize, 4u32, 2143080u32)?;
    emu.sw_no_count(9usize, 8usize, 8u32, 2143084u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2143088u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2143092u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b374));
}
#[inline(always)]
pub fn block_0x0020b370(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 8usize, 4u32, 2143092u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2143092u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b374));
}
#[inline(always)]
pub fn block_0x0020b374(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2143096u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2143096u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b378));
}
#[inline(always)]
pub fn block_0x0020b378(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 8usize, 0u32, 2143100u32)?;
    emu.lw_no_count(1usize, 2usize, 12u32, 2143104u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2143108u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2143112u32)?;
    emu.lw_no_count(18usize, 2usize, 0u32, 2143116u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2143120u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2143124u32;
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
pub fn block_0x0020b394(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2143128u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2143028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b334));
    } else {
        emu.pc = 2143132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b39c));
    }
}
#[inline(always)]
pub fn block_0x0020b39c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2143136u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2143076u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b364));
}
