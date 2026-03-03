pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2211724u32;
pub const PC_MAX: u32 = 2214020u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 109usize] = [
        block_0x0021bf8c,
        block_0x0021bf9c,
        block_0x0021bfa4,
        block_0x0021bfac,
        block_0x0021bfcc,
        block_0x0021bfe8,
        block_0x0021c000,
        block_0x0021c004,
        block_0x0021c014,
        block_0x0021c018,
        block_0x0021c020,
        block_0x0021c054,
        block_0x0021c068,
        block_0x0021c078,
        block_0x0021c084,
        block_0x0021c088,
        block_0x0021c0a8,
        block_0x0021c0b8,
        block_0x0021c10c,
        block_0x0021c110,
        block_0x0021c12c,
        block_0x0021c130,
        block_0x0021c138,
        block_0x0021c14c,
        block_0x0021c158,
        block_0x0021c168,
        block_0x0021c16c,
        block_0x0021c170,
        block_0x0021c1a0,
        block_0x0021c1bc,
        block_0x0021c1c4,
        block_0x0021c1cc,
        block_0x0021c1d4,
        block_0x0021c1dc,
        block_0x0021c1ec,
        block_0x0021c214,
        block_0x0021c220,
        block_0x0021c248,
        block_0x0021c250,
        block_0x0021c254,
        block_0x0021c26c,
        block_0x0021c2a0,
        block_0x0021c2b4,
        block_0x0021c2b8,
        block_0x0021c2e0,
        block_0x0021c2f4,
        block_0x0021c308,
        block_0x0021c30c,
        block_0x0021c310,
        block_0x0021c33c,
        block_0x0021c34c,
        block_0x0021c3a4,
        block_0x0021c3a8,
        block_0x0021c3c4,
        block_0x0021c3c8,
        block_0x0021c3cc,
        block_0x0021c3f8,
        block_0x0021c410,
        block_0x0021c428,
        block_0x0021c448,
        block_0x0021c45c,
        block_0x0021c470,
        block_0x0021c49c,
        block_0x0021c4a4,
        block_0x0021c4c0,
        block_0x0021c4e4,
        block_0x0021c4e8,
        block_0x0021c4ec,
        block_0x0021c500,
        block_0x0021c518,
        block_0x0021c520,
        block_0x0021c538,
        block_0x0021c53c,
        block_0x0021c550,
        block_0x0021c568,
        block_0x0021c570,
        block_0x0021c588,
        block_0x0021c5b4,
        block_0x0021c5b8,
        block_0x0021c5d4,
        block_0x0021c5d8,
        block_0x0021c5e0,
        block_0x0021c5ec,
        block_0x0021c5f4,
        block_0x0021c610,
        block_0x0021c62c,
        block_0x0021c640,
        block_0x0021c658,
        block_0x0021c660,
        block_0x0021c678,
        block_0x0021c67c,
        block_0x0021c690,
        block_0x0021c6a8,
        block_0x0021c6b0,
        block_0x0021c6c8,
        block_0x0021c6e8,
        block_0x0021c6f0,
        block_0x0021c6f4,
        block_0x0021c714,
        block_0x0021c72c,
        block_0x0021c820,
        block_0x0021c82c,
        block_0x0021c838,
        block_0x0021c83c,
        block_0x0021c840,
        block_0x0021c858,
        block_0x0021c86c,
        block_0x0021c87c,
        block_0x0021c884,
    ];
    const IDX: [u16; 575usize] = [
        1u16, 0u16, 0u16, 0u16, 2u16, 0u16, 3u16, 0u16, 4u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 5u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 6u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 7u16, 8u16, 0u16, 0u16, 0u16, 9u16, 10u16, 0u16, 11u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 12u16, 0u16,
        0u16, 0u16, 0u16, 13u16, 0u16, 0u16, 0u16, 14u16, 0u16, 0u16, 15u16, 16u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 17u16, 0u16, 0u16, 0u16, 18u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 19u16, 20u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        21u16, 22u16, 0u16, 23u16, 0u16, 0u16, 0u16, 0u16, 24u16, 0u16, 0u16, 25u16,
        0u16, 0u16, 0u16, 26u16, 27u16, 28u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 29u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 30u16, 0u16,
        31u16, 0u16, 32u16, 0u16, 33u16, 0u16, 34u16, 0u16, 0u16, 0u16, 35u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 36u16, 0u16, 0u16, 37u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 38u16, 0u16, 39u16, 40u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 41u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 42u16, 0u16, 0u16, 0u16, 0u16, 43u16, 44u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 45u16, 0u16, 0u16, 0u16, 0u16, 46u16,
        0u16, 0u16, 0u16, 0u16, 47u16, 48u16, 49u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 50u16, 0u16, 0u16, 0u16, 51u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 52u16, 53u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 54u16,
        55u16, 56u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 57u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 58u16, 0u16, 0u16, 0u16, 0u16, 0u16, 59u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 60u16, 0u16, 0u16, 0u16, 0u16, 61u16, 0u16,
        0u16, 0u16, 0u16, 62u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 63u16, 0u16, 64u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 65u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 66u16, 67u16, 68u16, 0u16, 0u16, 0u16, 0u16,
        69u16, 0u16, 0u16, 0u16, 0u16, 0u16, 70u16, 0u16, 71u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 72u16, 73u16, 0u16, 0u16, 0u16, 0u16, 74u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        75u16, 0u16, 76u16, 0u16, 0u16, 0u16, 0u16, 0u16, 77u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 78u16, 79u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 80u16, 81u16, 0u16, 82u16, 0u16, 0u16, 83u16, 0u16, 84u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 85u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 86u16, 0u16,
        0u16, 0u16, 0u16, 87u16, 0u16, 0u16, 0u16, 0u16, 0u16, 88u16, 0u16, 89u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 90u16, 91u16, 0u16, 0u16, 0u16, 0u16, 92u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 93u16, 0u16, 94u16, 0u16, 0u16, 0u16, 0u16, 0u16, 95u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 96u16, 0u16, 97u16, 98u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 99u16, 0u16, 0u16, 0u16, 0u16, 0u16, 100u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 101u16, 0u16, 0u16, 102u16, 0u16, 0u16,
        103u16, 104u16, 105u16, 0u16, 0u16, 0u16, 0u16, 0u16, 106u16, 0u16, 0u16, 0u16,
        0u16, 107u16, 0u16, 0u16, 0u16, 108u16, 0u16, 109u16,
    ];
    if pc < 2211724u32 || pc > 2214020u32 {
        return None;
    }
    let word_offset = ((pc - 2211724u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x0021bf8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 10usize, 0u32, 2211728u32);
    emu.lbu_no_count(12usize, 10usize, 5u32, 2211732u32);
    emu.lbu_no_count(10usize, 10usize, 4u32, 2211736u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2211864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c018));
    } else {
        emu.pc = 2211740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bf9c));
    }
}
#[inline(always)]
pub fn block_0x0021bf9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 1u32, 2211744u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2211756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bfac));
    } else {
        emu.pc = 2211748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bfa4));
    }
}
#[inline(always)]
pub fn block_0x0021bfa4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2211752u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2211756u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2211860u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c014));
}
#[inline(always)]
pub fn block_0x0021bfac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2211760u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2211764u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2211768u32)?;
    emu.adi_no_count(8usize, 11usize, 0u32, 2211772u32);
    emu.lw_no_count(10usize, 11usize, 0u32, 2211776u32)?;
    emu.lbu_no_count(11usize, 10usize, 10u32, 2211780u32);
    emu.ani_no_count(11usize, 11usize, 128u32, 2211784u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2211816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bfe8));
    } else {
        emu.pc = 2211788u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bfcc));
    }
}
#[inline(always)]
pub fn block_0x0021bfcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 4u32, 2211792u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2211796u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2211800u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2211804u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1933u32, 2211808u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2211812u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2211816u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2211840u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c000));
}
#[inline(always)]
pub fn block_0x0021bfe8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 4u32, 2211820u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2211824u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2211828u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2211832u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1932u32, 2211836u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2211840u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2211840u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c000));
}
#[inline(always)]
pub fn block_0x0021c000(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2211844u32;
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
pub fn block_0x0021c004(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 8usize, 0u32, 2211848u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2211852u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2211856u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2211860u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2211860u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c014));
}
#[inline(always)]
pub fn block_0x0021c014(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(10usize, 11usize, 4u32, 2211864u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2211864u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c018));
}
#[inline(always)]
pub fn block_0x0021c018(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 1u32, 2211868u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2211872u32;
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
pub fn block_0x0021c020(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2211876u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2211880u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2211884u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2211888u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2211892u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2211896u32)?;
    emu.sw_no_count(20usize, 2usize, 40u32, 2211900u32)?;
    emu.sw_no_count(21usize, 2usize, 36u32, 2211904u32)?;
    emu.adi_no_count(9usize, 11usize, 0u32, 2211908u32);
    emu.lbu_no_count(11usize, 10usize, 8u32, 2211912u32);
    emu.lw_no_count(19usize, 10usize, 0u32, 2211916u32)?;
    emu.adi_no_count(8usize, 0usize, 1u32, 2211920u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2212208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c170));
    } else {
        emu.pc = 2211924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c054));
    }
}
#[inline(always)]
pub fn block_0x0021c054(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 10usize, 4u32, 2211928u32)?;
    emu.lbu_no_count(11usize, 18usize, 10u32, 2211932u32);
    emu.ani_no_count(11usize, 11usize, 128u32, 2211936u32);
    emu.adi_no_count(20usize, 10usize, 0u32, 2211940u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2211972u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c084));
    } else {
        emu.pc = 2211944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c068));
    }
}
#[inline(always)]
pub fn block_0x0021c068(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(13usize, 0usize, 19usize, 2211948u32);
    emu.adi_no_count(13usize, 13usize, 1u32, 2211952u32);
    emu.adi_no_count(21usize, 12usize, 0u32, 2211956u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2212144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c130));
    } else {
        emu.pc = 2211960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c078));
    }
}
#[inline(always)]
pub fn block_0x0021c078(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2211964u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1923u32, 2211968u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2211972u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2212152u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c138));
}
#[inline(always)]
pub fn block_0x0021c084(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2212024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c0b8));
    } else {
        emu.pc = 2211976u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c088));
    }
}
#[inline(always)]
pub fn block_0x0021c088(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 18usize, 4u32, 2211980u32)?;
    emu.lw_no_count(10usize, 18usize, 0u32, 2211984u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2211988u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2211992u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1936u32, 2211996u32);
    emu.adi_no_count(21usize, 12usize, 0u32, 2212000u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2212004u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2212008u32;
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
pub fn block_0x0021c0a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 21usize, 0u32, 2212012u32);
    emu.adi_no_count(11usize, 10usize, 0u32, 2212016u32);
    emu.adi_no_count(10usize, 20usize, 0u32, 2212020u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2212208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c170));
    } else {
        emu.pc = 2212024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c0b8));
    }
}
#[inline]
pub fn block_0x0021c0b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 0usize, 1u32, 2212028u32);
    emu.adi_no_count(10usize, 2usize, 19u32, 2212032u32);
    emu.lw_no_count(11usize, 18usize, 0u32, 2212036u32)?;
    emu.lw_no_count(13usize, 18usize, 4u32, 2212040u32)?;
    emu.lw_no_count(14usize, 18usize, 8u32, 2212044u32)?;
    emu.lw_no_count(15usize, 18usize, 12u32, 2212048u32)?;
    emu.adi_no_count(16usize, 2usize, 4u32, 2212052u32);
    emu.sw_no_count(11usize, 2usize, 4u32, 2212056u32)?;
    emu.sw_no_count(13usize, 2usize, 8u32, 2212060u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2212064u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2212068u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1896u32, 2212072u32);
    emu.lw_no_count(12usize, 12usize, 12u32, 2212076u32)?;
    emu.sb_no_count(8usize, 2usize, 19u32, 2212080u32);
    emu.sw_no_count(16usize, 2usize, 20u32, 2212084u32)?;
    emu.sw_no_count(10usize, 2usize, 24u32, 2212088u32)?;
    emu.sw_no_count(14usize, 2usize, 28u32, 2212092u32)?;
    emu.sw_no_count(15usize, 2usize, 32u32, 2212096u32)?;
    emu.adi_no_count(11usize, 2usize, 20u32, 2212100u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2212104u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2212108u32;
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
pub fn block_0x0021c10c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2212204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c16c));
    } else {
        emu.pc = 2212112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c110));
    }
}
#[inline(always)]
pub fn block_0x0021c110(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 24u32, 2212116u32)?;
    emu.lw_no_count(10usize, 2usize, 20u32, 2212120u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2212124u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2212128u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1930u32, 2212132u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2212136u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2212140u32;
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
pub fn block_0x0021c12c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2212144u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2212200u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c168));
}
#[inline(always)]
pub fn block_0x0021c130(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2212148u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1935u32, 2212152u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2212152u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c138));
}
#[inline(always)]
pub fn block_0x0021c138(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 4u32, 2212156u32)?;
    emu.lw_no_count(10usize, 18usize, 0u32, 2212160u32)?;
    emu.lw_no_count(14usize, 12usize, 12u32, 2212164u32)?;
    emu.adi_no_count(12usize, 13usize, 0u32, 2212168u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2212172u32;
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
pub fn block_0x0021c14c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 10usize, 0u32, 2212176u32);
    emu.adi_no_count(10usize, 20usize, 0u32, 2212180u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2212208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c170));
    } else {
        emu.pc = 2212184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c158));
    }
}
#[inline(always)]
pub fn block_0x0021c158(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 21usize, 12u32, 2212188u32)?;
    emu.adi_no_count(10usize, 9usize, 0u32, 2212192u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2212196u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2212200u32;
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
pub fn block_0x0021c168(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 10usize, 0u32, 2212204u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2212204u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c16c));
}
#[inline(always)]
pub fn block_0x0021c16c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 20usize, 0u32, 2212208u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2212208u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c170));
}
#[inline]
pub fn block_0x0021c170(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 19usize, 1u32, 2212212u32);
    emu.sw_no_count(19usize, 10usize, 0u32, 2212216u32)?;
    emu.sb_no_count(8usize, 10usize, 8u32, 2212220u32);
    emu.lw_no_count(1usize, 2usize, 60u32, 2212224u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2212228u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2212232u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2212236u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2212240u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2212244u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2212248u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2212252u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2212256u32;
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
pub fn block_0x0021c1a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2212260u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2212264u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2212268u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2212272u32)?;
    emu.lw_no_count(11usize, 10usize, 0u32, 2212276u32)?;
    emu.lbu_no_count(8usize, 10usize, 8u32, 2212280u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2212436u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c254));
    } else {
        emu.pc = 2212284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c1bc));
    }
}
#[inline(always)]
pub fn block_0x0021c1bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(8usize, 8usize, 1u32, 2212288u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2212300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c1cc));
    } else {
        emu.pc = 2212292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c1c4));
    }
}
#[inline(always)]
pub fn block_0x0021c1c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 0usize, 1u32, 2212296u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2212300u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2212432u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c250));
}
#[inline(always)]
pub fn block_0x0021c1cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 1u32, 2212304u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2212384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c220));
    } else {
        emu.pc = 2212308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c1d4));
    }
}
#[inline(always)]
pub fn block_0x0021c1d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 10usize, 9u32, 2212312u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2212384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c220));
    } else {
        emu.pc = 2212316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c1dc));
    }
}
#[inline(always)]
pub fn block_0x0021c1dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 4u32, 2212320u32)?;
    emu.lbu_no_count(12usize, 11usize, 10u32, 2212324u32);
    emu.ani_no_count(12usize, 12usize, 128u32, 2212328u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2212384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c220));
    } else {
        emu.pc = 2212332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c1ec));
    }
}
#[inline]
pub fn block_0x0021c1ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 4u32, 2212336u32)?;
    emu.lw_no_count(13usize, 11usize, 0u32, 2212340u32)?;
    emu.lw_no_count(14usize, 12usize, 12u32, 2212344u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2212348u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1939u32, 2212352u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2212356u32);
    emu.adi_no_count(8usize, 0usize, 1u32, 2212360u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2212364u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2212368u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2212372u32;
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
pub fn block_0x0021c214(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 10usize, 0u32, 2212376u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2212380u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2212432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c250));
    } else {
        emu.pc = 2212384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c220));
    }
}
#[inline]
pub fn block_0x0021c220(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 4u32, 2212388u32)?;
    emu.lw_no_count(12usize, 11usize, 4u32, 2212392u32)?;
    emu.lw_no_count(13usize, 11usize, 0u32, 2212396u32)?;
    emu.lw_no_count(14usize, 12usize, 12u32, 2212400u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2212404u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1938u32, 2212408u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2212412u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2212416u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2212420u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2212424u32;
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
pub fn block_0x0021c248(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 10usize, 0u32, 2212428u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2212432u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2212432u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c250));
}
#[inline(always)]
pub fn block_0x0021c250(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(8usize, 10usize, 8u32, 2212436u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2212436u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c254));
}
#[inline(always)]
pub fn block_0x0021c254(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 8usize, 1u32, 2212440u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2212444u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2212448u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2212452u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2212456u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2212460u32;
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
pub fn block_0x0021c26c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2212464u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2212468u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2212472u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2212476u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2212480u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2212484u32)?;
    emu.sw_no_count(20usize, 2usize, 40u32, 2212488u32)?;
    emu.sw_no_count(21usize, 2usize, 36u32, 2212492u32)?;
    emu.adi_no_count(8usize, 11usize, 0u32, 2212496u32);
    emu.lbu_no_count(11usize, 10usize, 4u32, 2212500u32);
    emu.adi_no_count(19usize, 0usize, 1u32, 2212504u32);
    emu.adi_no_count(18usize, 0usize, 1u32, 2212508u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2212812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c3cc));
    } else {
        emu.pc = 2212512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c2a0));
    }
}
#[inline(always)]
pub fn block_0x0021c2a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(9usize, 10usize, 0u32, 2212516u32)?;
    emu.lbu_no_count(11usize, 10usize, 5u32, 2212520u32);
    emu.lbu_no_count(13usize, 9usize, 10u32, 2212524u32);
    emu.ani_no_count(13usize, 13usize, 128u32, 2212528u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2212620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c30c));
    } else {
        emu.pc = 2212532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c2b4));
    }
}
#[inline(always)]
pub fn block_0x0021c2b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2212596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c2f4));
    } else {
        emu.pc = 2212536u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c2b8));
    }
}
#[inline]
pub fn block_0x0021c2b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 9usize, 4u32, 2212540u32)?;
    emu.lw_no_count(13usize, 9usize, 0u32, 2212544u32)?;
    emu.lw_no_count(14usize, 11usize, 12u32, 2212548u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2212552u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1923u32, 2212556u32);
    emu.adi_no_count(18usize, 12usize, 0u32, 2212560u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2212564u32);
    emu.adi_no_count(20usize, 10usize, 0u32, 2212568u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2212572u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2212576u32;
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
pub fn block_0x0021c2e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 18usize, 0u32, 2212580u32);
    emu.adi_no_count(11usize, 10usize, 0u32, 2212584u32);
    emu.adi_no_count(10usize, 20usize, 0u32, 2212588u32);
    emu.adi_no_count(18usize, 0usize, 1u32, 2212592u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2212812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c3cc));
    } else {
        emu.pc = 2212596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c2f4));
    }
}
#[inline(always)]
pub fn block_0x0021c2f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 12usize, 12u32, 2212600u32)?;
    emu.adi_no_count(20usize, 10usize, 0u32, 2212604u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2212608u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2212612u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2212616u32;
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
pub fn block_0x0021c308(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2212620u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2212804u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c3c4));
}
#[inline(always)]
pub fn block_0x0021c30c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2212684u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c34c));
    } else {
        emu.pc = 2212624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c310));
    }
}
#[inline]
pub fn block_0x0021c310(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 9usize, 4u32, 2212628u32)?;
    emu.lw_no_count(13usize, 9usize, 0u32, 2212632u32)?;
    emu.lw_no_count(14usize, 11usize, 12u32, 2212636u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2212640u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1940u32, 2212644u32);
    emu.adi_no_count(20usize, 12usize, 0u32, 2212648u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2212652u32);
    emu.adi_no_count(18usize, 0usize, 1u32, 2212656u32);
    emu.adi_no_count(21usize, 10usize, 0u32, 2212660u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2212664u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2212668u32;
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
pub fn block_0x0021c33c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 20usize, 0u32, 2212672u32);
    emu.adi_no_count(11usize, 10usize, 0u32, 2212676u32);
    emu.adi_no_count(10usize, 21usize, 0u32, 2212680u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2212812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c3cc));
    } else {
        emu.pc = 2212684u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c34c));
    }
}
#[inline]
pub fn block_0x0021c34c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 10usize, 0u32, 2212688u32);
    emu.adi_no_count(18usize, 0usize, 1u32, 2212692u32);
    emu.adi_no_count(10usize, 2usize, 19u32, 2212696u32);
    emu.lw_no_count(11usize, 9usize, 0u32, 2212700u32)?;
    emu.lw_no_count(13usize, 9usize, 4u32, 2212704u32)?;
    emu.lw_no_count(14usize, 9usize, 8u32, 2212708u32)?;
    emu.lw_no_count(15usize, 9usize, 12u32, 2212712u32)?;
    emu.adi_no_count(16usize, 2usize, 4u32, 2212716u32);
    emu.sw_no_count(11usize, 2usize, 4u32, 2212720u32)?;
    emu.sw_no_count(13usize, 2usize, 8u32, 2212724u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2212728u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2212732u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1896u32, 2212736u32);
    emu.lw_no_count(12usize, 12usize, 12u32, 2212740u32)?;
    emu.sb_no_count(18usize, 2usize, 19u32, 2212744u32);
    emu.sw_no_count(16usize, 2usize, 20u32, 2212748u32)?;
    emu.sw_no_count(10usize, 2usize, 24u32, 2212752u32)?;
    emu.sw_no_count(14usize, 2usize, 28u32, 2212756u32)?;
    emu.sw_no_count(15usize, 2usize, 32u32, 2212760u32)?;
    emu.adi_no_count(11usize, 2usize, 20u32, 2212764u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2212768u32);
    emu.add_memory_rw_events(22usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2212772u32;
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
pub fn block_0x0021c3a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2212808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c3c8));
    } else {
        emu.pc = 2212776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c3a8));
    }
}
#[inline(always)]
pub fn block_0x0021c3a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 24u32, 2212780u32)?;
    emu.lw_no_count(10usize, 2usize, 20u32, 2212784u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2212788u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2212792u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1930u32, 2212796u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2212800u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2212804u32;
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
pub fn block_0x0021c3c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2212808u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2212808u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c3c8));
}
#[inline(always)]
pub fn block_0x0021c3c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 20usize, 0u32, 2212812u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2212812u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c3cc));
}
#[inline]
pub fn block_0x0021c3cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(18usize, 10usize, 4u32, 2212816u32);
    emu.sb_no_count(19usize, 10usize, 5u32, 2212820u32);
    emu.lw_no_count(1usize, 2usize, 60u32, 2212824u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2212828u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2212832u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2212836u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2212840u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2212844u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2212848u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2212852u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2212856u32;
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
pub fn block_0x0021c3f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2212860u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2212864u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2212868u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2212872u32);
    emu.lbu_no_count(10usize, 10usize, 4u32, 2212876u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2212904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c428));
    } else {
        emu.pc = 2212880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c410));
    }
}
#[inline(always)]
pub fn block_0x0021c410(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2212884u32);
    emu.sb_no_count(10usize, 8usize, 4u32, 2212888u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2212892u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2212896u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2212900u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2212904u32;
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
pub fn block_0x0021c428(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 0u32, 2212908u32)?;
    emu.lw_no_count(11usize, 10usize, 4u32, 2212912u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2212916u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2212920u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2212924u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1942u32, 2212928u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2212932u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2212936u32;
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
pub fn block_0x0021c448(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(10usize, 8usize, 4u32, 2212940u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2212944u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2212948u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2212952u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2212956u32;
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
pub fn block_0x0021c45c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 0u32, 2212960u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2212964u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1896u32, 2212968u32);
    emu.apc_no_count(6usize, 2212968u32, 4096u32, 2212972u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2212976u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965360u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021c470(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967136u32, 2212980u32);
    emu.sw_no_count(1usize, 2usize, 156u32, 2212984u32)?;
    emu.sw_no_count(8usize, 2usize, 152u32, 2212988u32)?;
    emu.sw_no_count(9usize, 2usize, 148u32, 2212992u32)?;
    emu.sw_no_count(18usize, 2usize, 144u32, 2212996u32)?;
    emu.sw_no_count(19usize, 2usize, 140u32, 2213000u32)?;
    emu.adi_no_count(8usize, 11usize, 0u32, 2213004u32);
    emu.lw_no_count(11usize, 11usize, 8u32, 2213008u32)?;
    emu.sli_no_count(12usize, 11usize, 6u32, 2213012u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2213016u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2213100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c4ec));
    } else {
        emu.pc = 2213020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c49c));
    }
}
#[inline(always)]
pub fn block_0x0021c49c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 11usize, 5u32, 2213024u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2213180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c53c));
    } else {
        emu.pc = 2213028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c4a4));
    }
}
#[inline(always)]
pub fn block_0x0021c4a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 0u32, 2213032u32)?;
    emu.adi_no_count(18usize, 2usize, 12u32, 2213036u32);
    emu.adi_no_count(11usize, 2usize, 12u32, 2213040u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2213044u32);
    emu.adi_no_count(19usize, 0usize, 10u32, 2213048u32);
    emu.apc_no_count(1usize, 2213048u32, 4294963200u32, 2213052u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2213056u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966360u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021c4c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(15usize, 19usize, 10usize, 2213060u32);
    emu.adr_no_count(14usize, 18usize, 10usize, 2213064u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2213068u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2213072u32);
    emu.adi_no_count(18usize, 0usize, 1u32, 2213076u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2213080u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2213084u32);
    emu.apc_no_count(1usize, 2213084u32, 4096u32, 2213088u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2213092u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965888u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021c4e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2213304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c5b8));
    } else {
        emu.pc = 2213096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c4e8));
    }
}
#[inline(always)]
pub fn block_0x0021c4e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2213100u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2213620u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c6f4));
}
#[inline(always)]
pub fn block_0x0021c4ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 0u32, 2213104u32);
    emu.lw_no_count(10usize, 9usize, 0u32, 2213108u32)?;
    emu.adi_no_count(11usize, 2usize, 139u32, 2213112u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2213116u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2213120u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2213144u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c518));
}
#[inline(always)]
pub fn block_0x0021c500(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 87u32, 2213124u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2213128u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2213132u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2213136u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2213140u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2213256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c588));
    } else {
        emu.pc = 2213144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c518));
    }
}
#[inline(always)]
pub fn block_0x0021c518(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(13usize, 10usize, 15u32, 2213148u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2213120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c500));
    } else {
        emu.pc = 2213152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c520));
    }
}
#[inline(always)]
pub fn block_0x0021c520(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 48u32, 2213156u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2213160u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2213164u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2213168u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2213172u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2213144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c518));
    } else {
        emu.pc = 2213176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c538));
    }
}
#[inline(always)]
pub fn block_0x0021c538(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2213180u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2213256u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c588));
}
#[inline(always)]
pub fn block_0x0021c53c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 0u32, 2213184u32);
    emu.lw_no_count(10usize, 9usize, 0u32, 2213188u32)?;
    emu.adi_no_count(11usize, 2usize, 139u32, 2213192u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2213196u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2213200u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2213224u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c568));
}
#[inline(always)]
pub fn block_0x0021c550(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 55u32, 2213204u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2213208u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2213212u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2213216u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2213220u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2213256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c588));
    } else {
        emu.pc = 2213224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c568));
    }
}
#[inline(always)]
pub fn block_0x0021c568(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(13usize, 10usize, 15u32, 2213228u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2213200u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c550));
    } else {
        emu.pc = 2213232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c570));
    }
}
#[inline(always)]
pub fn block_0x0021c570(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 48u32, 2213236u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2213240u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2213244u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2213248u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2213252u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2213224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c568));
    } else {
        emu.pc = 2213256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c588));
    }
}
#[inline]
pub fn block_0x0021c588(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2213260u32);
    emu.sbr_no_count(10usize, 10usize, 15usize, 2213264u32);
    emu.adi_no_count(14usize, 10usize, 128u32, 2213268u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2213272u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1328u32, 2213276u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2213280u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2213284u32);
    emu.adi_no_count(18usize, 0usize, 1u32, 2213288u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2213292u32);
    emu.apc_no_count(1usize, 2213292u32, 4096u32, 2213296u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2213300u32;
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
pub fn block_0x0021c5b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2213620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c6f4));
    } else {
        emu.pc = 2213304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c5b8));
    }
}
#[inline(always)]
pub fn block_0x0021c5b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 8usize, 4u32, 2213308u32)?;
    emu.lw_no_count(10usize, 8usize, 0u32, 2213312u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2213316u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2213320u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1943u32, 2213324u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2213328u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2213332u32;
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
pub fn block_0x0021c5d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2213344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c5e0));
    } else {
        emu.pc = 2213336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c5d8));
    }
}
#[inline(always)]
pub fn block_0x0021c5d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 1u32, 2213340u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2213344u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2213620u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c6f4));
}
#[inline(always)]
pub fn block_0x0021c5e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 8u32, 2213348u32)?;
    emu.sli_no_count(11usize, 10usize, 6u32, 2213352u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2213420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c62c));
    } else {
        emu.pc = 2213356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c5ec));
    }
}
#[inline(always)]
pub fn block_0x0021c5ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 10usize, 5u32, 2213360u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2213500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c67c));
    } else {
        emu.pc = 2213364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c5f4));
    }
}
#[inline(always)]
pub fn block_0x0021c5f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 4u32, 2213368u32)?;
    emu.adi_no_count(9usize, 2usize, 12u32, 2213372u32);
    emu.adi_no_count(11usize, 2usize, 12u32, 2213376u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2213380u32);
    emu.adi_no_count(18usize, 0usize, 10u32, 2213384u32);
    emu.apc_no_count(1usize, 2213384u32, 4294963200u32, 2213388u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2213392u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966024u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021c610(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(15usize, 18usize, 10usize, 2213396u32);
    emu.adr_no_count(14usize, 9usize, 10usize, 2213400u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2213404u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2213408u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2213412u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2213416u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2213420u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2213608u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c6e8));
}
#[inline(always)]
pub fn block_0x0021c62c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 0u32, 2213424u32);
    emu.lw_no_count(10usize, 9usize, 4u32, 2213428u32)?;
    emu.adi_no_count(11usize, 2usize, 139u32, 2213432u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2213436u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2213440u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2213464u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c658));
}
#[inline(always)]
pub fn block_0x0021c640(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 87u32, 2213444u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2213448u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2213452u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2213456u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2213460u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2213576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c6c8));
    } else {
        emu.pc = 2213464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c658));
    }
}
#[inline(always)]
pub fn block_0x0021c658(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(13usize, 10usize, 15u32, 2213468u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2213440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c640));
    } else {
        emu.pc = 2213472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c660));
    }
}
#[inline(always)]
pub fn block_0x0021c660(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 48u32, 2213476u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2213480u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2213484u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2213488u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2213492u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2213464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c658));
    } else {
        emu.pc = 2213496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c678));
    }
}
#[inline(always)]
pub fn block_0x0021c678(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2213500u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2213576u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c6c8));
}
#[inline(always)]
pub fn block_0x0021c67c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 0u32, 2213504u32);
    emu.lw_no_count(10usize, 9usize, 4u32, 2213508u32)?;
    emu.adi_no_count(11usize, 2usize, 139u32, 2213512u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2213516u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2213520u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2213544u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c6a8));
}
#[inline(always)]
pub fn block_0x0021c690(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 55u32, 2213524u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2213528u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2213532u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2213536u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2213540u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2213576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c6c8));
    } else {
        emu.pc = 2213544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c6a8));
    }
}
#[inline(always)]
pub fn block_0x0021c6a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(13usize, 10usize, 15u32, 2213548u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2213520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c690));
    } else {
        emu.pc = 2213552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c6b0));
    }
}
#[inline(always)]
pub fn block_0x0021c6b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 48u32, 2213556u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2213560u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2213564u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2213568u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2213572u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2213544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c6a8));
    } else {
        emu.pc = 2213576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c6c8));
    }
}
#[inline(always)]
pub fn block_0x0021c6c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2213580u32);
    emu.sbr_no_count(10usize, 10usize, 15usize, 2213584u32);
    emu.adi_no_count(14usize, 10usize, 128u32, 2213588u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2213592u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1328u32, 2213596u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2213600u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2213604u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2213608u32);
    emu.add_memory_rw_events(8usize);
    emu.pc = 2213608u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c6e8));
}
#[inline(always)]
pub fn block_0x0021c6e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2213608u32, 4096u32, 2213612u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2213616u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965364u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021c6f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2213620u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2213620u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c6f4));
}
#[inline(always)]
pub fn block_0x0021c6f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2213624u32);
    emu.lw_no_count(1usize, 2usize, 156u32, 2213628u32)?;
    emu.lw_no_count(8usize, 2usize, 152u32, 2213632u32)?;
    emu.lw_no_count(9usize, 2usize, 148u32, 2213636u32)?;
    emu.lw_no_count(18usize, 2usize, 144u32, 2213640u32)?;
    emu.lw_no_count(19usize, 2usize, 140u32, 2213644u32)?;
    emu.adi_no_count(2usize, 2usize, 160u32, 2213648u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2213652u32;
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
pub fn block_0x0021c714(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2213656u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2213660u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965896u32, 2213664u32);
    emu.adi_no_count(11usize, 0usize, 43u32, 2213668u32);
    emu.apc_no_count(1usize, 2213668u32, 4294963200u32, 2213672u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2213676u32;
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
#[inline(never)]
pub fn block_0x0021c72c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 61u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(73728u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2213680u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2213684u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294965940u32, 2213688u32);
    emu.adi_no_count(11usize, 11usize, 4294965295u32, 2213692u32);
    emu.sltru_no_count(11usize, 10usize, 11usize, 2213696u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2213700u32);
    emu.ani_no_count(11usize, 11usize, 17u32, 2213704u32);
    emu.sli_no_count(12usize, 11usize, 2u32, 2213708u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2213712u32);
    emu.lw_no_count(12usize, 12usize, 32u32, 2213716u32)?;
    emu.sli_no_count(14usize, 10usize, 11u32, 2213720u32);
    emu.sli_no_count(12usize, 12usize, 11u32, 2213724u32);
    emu.sltru_no_count(12usize, 14usize, 12usize, 2213728u32);
    emu.xri_no_count(12usize, 12usize, 1u32, 2213732u32);
    emu.sli_no_count(12usize, 12usize, 3u32, 2213736u32);
    emu.orr_no_count(11usize, 11usize, 12usize, 2213740u32);
    emu.sli_no_count(12usize, 11usize, 2u32, 2213744u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2213748u32);
    emu.lw_no_count(12usize, 12usize, 16u32, 2213752u32)?;
    emu.sli_no_count(12usize, 12usize, 11u32, 2213756u32);
    emu.sltru_no_count(12usize, 14usize, 12usize, 2213760u32);
    emu.xri_no_count(12usize, 12usize, 1u32, 2213764u32);
    emu.sli_no_count(12usize, 12usize, 2u32, 2213768u32);
    emu.orr_no_count(11usize, 11usize, 12usize, 2213772u32);
    emu.sli_no_count(12usize, 11usize, 2u32, 2213776u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2213780u32);
    emu.lw_no_count(12usize, 12usize, 8u32, 2213784u32)?;
    emu.sli_no_count(12usize, 12usize, 11u32, 2213788u32);
    emu.sltru_no_count(12usize, 14usize, 12usize, 2213792u32);
    emu.xri_no_count(12usize, 12usize, 1u32, 2213796u32);
    emu.sli_no_count(12usize, 12usize, 1u32, 2213800u32);
    emu.orr_no_count(11usize, 11usize, 12usize, 2213804u32);
    emu.sli_no_count(12usize, 11usize, 2u32, 2213808u32);
    emu.adr_no_count(12usize, 12usize, 13usize, 2213812u32);
    emu.lw_no_count(12usize, 12usize, 4u32, 2213816u32)?;
    emu.sli_no_count(12usize, 12usize, 11u32, 2213820u32);
    emu.sltru_no_count(12usize, 14usize, 12usize, 2213824u32);
    emu.xri_no_count(12usize, 12usize, 1u32, 2213828u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2213832u32);
    emu.sli_no_count(12usize, 11usize, 2u32, 2213836u32);
    emu.adr_no_count(12usize, 12usize, 13usize, 2213840u32);
    emu.lw_no_count(12usize, 12usize, 4u32, 2213844u32)?;
    emu.sli_no_count(12usize, 12usize, 11u32, 2213848u32);
    emu.sltru_no_count(12usize, 14usize, 12usize, 2213852u32);
    emu.xri_no_count(12usize, 12usize, 1u32, 2213856u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2213860u32);
    emu.sli_no_count(12usize, 11usize, 2u32, 2213864u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2213868u32);
    emu.lw_no_count(12usize, 12usize, 0u32, 2213872u32)?;
    emu.sli_no_count(12usize, 12usize, 11u32, 2213876u32);
    emu.xrr_no_count(15usize, 12usize, 14usize, 2213880u32);
    emu.sltru_no_count(12usize, 12usize, 14usize, 2213884u32);
    emu.sltiu_no_count(14usize, 15usize, 1u32, 2213888u32);
    emu.adr_no_count(11usize, 12usize, 11usize, 2213892u32);
    emu.adr_no_count(14usize, 14usize, 11usize, 2213896u32);
    emu.sli_no_count(11usize, 14usize, 2u32, 2213900u32);
    emu.adr_no_count(13usize, 13usize, 11usize, 2213904u32);
    emu.lw_no_count(11usize, 13usize, 0u32, 2213908u32)?;
    emu.adi_no_count(12usize, 0usize, 32u32, 2213912u32);
    emu.sri_no_count(11usize, 11usize, 21u32, 2213916u32);
    emu.add_memory_rw_events(60usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2213948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c83c));
    } else {
        emu.pc = 2213920u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c820));
    }
}
#[inline(always)]
pub fn block_0x0021c820(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 13usize, 4u32, 2213924u32)?;
    emu.sri_no_count(12usize, 12usize, 21u32, 2213928u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a != b {
        emu.pc = 2213952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c840));
    } else {
        emu.pc = 2213932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c82c));
    }
}
#[inline(always)]
pub fn block_0x0021c82c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(13usize, 11usize, 4294967295u32, 2213936u32);
    emu.adr_no_count(13usize, 12usize, 13usize, 2213940u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2213976u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c858));
    } else {
        emu.pc = 2213944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c838));
    }
}
#[inline(always)]
pub fn block_0x0021c838(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2213948u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2214020u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c884));
}
#[inline(always)]
pub fn block_0x0021c83c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 751u32, 2213952u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2213952u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c840));
}
#[inline(always)]
pub fn block_0x0021c840(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 13usize, 4294967292u32, 2213956u32)?;
    emu.sli_no_count(13usize, 13usize, 11u32, 2213960u32);
    emu.sri_no_count(14usize, 13usize, 11u32, 2213964u32);
    emu.xri_no_count(13usize, 11usize, 4294967295u32, 2213968u32);
    emu.adr_no_count(13usize, 12usize, 13usize, 2213972u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2214020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c884));
    } else {
        emu.pc = 2213976u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c858));
    }
}
#[inline(always)]
pub fn block_0x0021c858(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 0u32, 2213980u32);
    emu.sbr_no_count(10usize, 10usize, 14usize, 2213984u32);
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2213988u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2213992u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1945u32, 2213996u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2213996u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c86c));
}
#[inline(always)]
pub fn block_0x0021c86c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 14usize, 11usize, 2214000u32);
    emu.lbu_no_count(15usize, 15usize, 0u32, 2214004u32);
    emu.adr_no_count(13usize, 13usize, 15usize, 2214008u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2214020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c884));
    } else {
        emu.pc = 2214012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c87c));
    }
}
#[inline(always)]
pub fn block_0x0021c87c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 11usize, 1u32, 2214016u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2213996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c86c));
    } else {
        emu.pc = 2214020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c884));
    }
}
#[inline(always)]
pub fn block_0x0021c884(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 11usize, 1u32, 2214024u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2214028u32;
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
