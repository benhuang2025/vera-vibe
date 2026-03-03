pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2135316u32;
pub const PC_MAX: u32 = 2137808u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 109usize] = [
        block_0x00209514,
        block_0x00209534,
        block_0x0020953c,
        block_0x00209590,
        block_0x002095a0,
        block_0x002095a8,
        block_0x002095c0,
        block_0x002095d0,
        block_0x002095d8,
        block_0x002095e0,
        block_0x002095f0,
        block_0x00209604,
        block_0x0020961c,
        block_0x00209670,
        block_0x002096a8,
        block_0x002096b8,
        block_0x002096c0,
        block_0x002096d4,
        block_0x002096d8,
        block_0x0020970c,
        block_0x00209740,
        block_0x00209748,
        block_0x00209758,
        block_0x0020975c,
        block_0x00209768,
        block_0x0020976c,
        block_0x0020977c,
        block_0x00209780,
        block_0x0020978c,
        block_0x00209794,
        block_0x00209798,
        block_0x002097c0,
        block_0x002097c8,
        block_0x002097d0,
        block_0x002097e0,
        block_0x00209808,
        block_0x00209810,
        block_0x0020983c,
        block_0x00209844,
        block_0x00209848,
        block_0x00209850,
        block_0x0020985c,
        block_0x0020986c,
        block_0x0020987c,
        block_0x00209884,
        block_0x002098a4,
        block_0x002098c0,
        block_0x002098c4,
        block_0x002098e0,
        block_0x002098e8,
        block_0x00209914,
        block_0x0020994c,
        block_0x00209978,
        block_0x002099a8,
        block_0x002099bc,
        block_0x002099e4,
        block_0x00209a04,
        block_0x00209a10,
        block_0x00209a4c,
        block_0x00209a64,
        block_0x00209a98,
        block_0x00209ab4,
        block_0x00209abc,
        block_0x00209aec,
        block_0x00209b04,
        block_0x00209b18,
        block_0x00209b28,
        block_0x00209b44,
        block_0x00209b4c,
        block_0x00209b54,
        block_0x00209b74,
        block_0x00209b80,
        block_0x00209b94,
        block_0x00209ba8,
        block_0x00209bb8,
        block_0x00209bf0,
        block_0x00209bf8,
        block_0x00209c34,
        block_0x00209c4c,
        block_0x00209c7c,
        block_0x00209c98,
        block_0x00209ca0,
        block_0x00209ccc,
        block_0x00209ce4,
        block_0x00209cf8,
        block_0x00209d08,
        block_0x00209d20,
        block_0x00209d28,
        block_0x00209d48,
        block_0x00209d54,
        block_0x00209d68,
        block_0x00209d7c,
        block_0x00209d8c,
        block_0x00209dbc,
        block_0x00209dc8,
        block_0x00209dd0,
        block_0x00209dd8,
        block_0x00209de4,
        block_0x00209dec,
        block_0x00209e0c,
        block_0x00209e38,
        block_0x00209e50,
        block_0x00209e5c,
        block_0x00209e78,
        block_0x00209e98,
        block_0x00209ea8,
        block_0x00209eb4,
        block_0x00209eb8,
        block_0x00209ed0,
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
    if pc < 2135316u32 || pc > 2137808u32 {
        return None;
    }
    let word_offset = ((pc - 2135316u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x00209514(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2135320u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2135324u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2135328u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2135332u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2135336u32)?;
    emu.adi_no_count(8usize, 11usize, 0u32, 2135340u32);
    emu.apc_no_count(1usize, 2135340u32, 4294938624u32, 2135344u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135348u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(464u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209534(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 255u32, 2135352u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2135580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020961c));
    } else {
        emu.pc = 2135356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020953c));
    }
}
#[inline]
pub fn block_0x0020953c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(8usize, 2usize, 40u32, 2135360u32)?;
    emu.adi_no_count(10usize, 2usize, 40u32, 2135364u32);
    let a = 0u32.wrapping_add(2146304u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2135368u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966404u32, 2135372u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2135376u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965424u32, 2135380u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2135384u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2135388u32)?;
    emu.adi_no_count(14usize, 2usize, 32u32, 2135392u32);
    emu.sw_no_count(10usize, 2usize, 32u32, 2135396u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2135400u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2135404u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2135408u32)?;
    emu.sw_no_count(13usize, 2usize, 12u32, 2135412u32)?;
    emu.sw_no_count(14usize, 2usize, 16u32, 2135416u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2135420u32)?;
    emu.adi_no_count(10usize, 2usize, 0u32, 2135424u32);
    emu.adi_no_count(11usize, 2usize, 47u32, 2135428u32);
    emu.adi_no_count(12usize, 2usize, 8u32, 2135432u32);
    emu.apc_no_count(1usize, 2135432u32, 4294963200u32, 2135436u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135440u32;
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
pub fn block_0x00209590(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 0u32, 2135444u32);
    emu.lw_no_count(8usize, 2usize, 4u32, 2135448u32)?;
    emu.adi_no_count(11usize, 0usize, 4u32, 2135452u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2135488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002095c0));
    } else {
        emu.pc = 2135456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002095a0));
    }
}
#[inline(always)]
pub fn block_0x002095a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 3u32, 2135460u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2135488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002095c0));
    } else {
        emu.pc = 2135464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002095a8));
    }
}
#[inline(always)]
pub fn block_0x002095a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2135468u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2135472u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2135476u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2135480u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2135484u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135488u32;
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
pub fn block_0x002095c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 8usize, 4u32, 2135492u32)?;
    emu.lw_no_count(11usize, 18usize, 0u32, 2135496u32)?;
    emu.lw_no_count(9usize, 8usize, 0u32, 2135500u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2135512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002095d8));
    } else {
        emu.pc = 2135504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002095d0));
    }
}
#[inline(always)]
pub fn block_0x002095d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2135508u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2135512u32;
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
pub fn block_0x002095d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 18usize, 4u32, 2135516u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2135536u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002095f0));
    } else {
        emu.pc = 2135520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002095e0));
    }
}
#[inline(always)]
pub fn block_0x002095e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 8u32, 2135524u32)?;
    emu.adi_no_count(10usize, 9usize, 0u32, 2135528u32);
    emu.apc_no_count(1usize, 2135528u32, 4294934528u32, 2135532u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135536u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966096u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002095f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 12u32, 2135540u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2135544u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2135548u32);
    emu.apc_no_count(1usize, 2135548u32, 4294934528u32, 2135552u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135556u32;
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
pub fn block_0x00209604(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2135560u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2135564u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2135568u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2135572u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2135576u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135580u32;
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
pub fn block_0x0020961c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(8usize, 2usize, 0u32, 2135584u32)?;
    emu.adi_no_count(10usize, 2usize, 0u32, 2135588u32);
    let a = 0u32.wrapping_add(2146304u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2135592u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966404u32, 2135596u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2135600u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965456u32, 2135604u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2135608u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2135612u32)?;
    emu.adi_no_count(14usize, 2usize, 32u32, 2135616u32);
    emu.sw_no_count(10usize, 2usize, 32u32, 2135620u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2135624u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2135628u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2135632u32)?;
    emu.sw_no_count(13usize, 2usize, 12u32, 2135636u32)?;
    emu.sw_no_count(14usize, 2usize, 16u32, 2135640u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2135644u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2135648u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965472u32, 2135652u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2135656u32);
    emu.apc_no_count(1usize, 2135656u32, 12288u32, 2135660u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135664u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966764u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00209670(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 10usize, 0u32, 2135668u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2135672u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965976u32, 2135676u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2135680u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966144u32, 2135684u32);
    emu.sli_no_count(10usize, 10usize, 2u32, 2135688u32);
    emu.adr_no_count(12usize, 12usize, 10usize, 2135692u32);
    emu.adr_no_count(10usize, 13usize, 10usize, 2135696u32);
    emu.lw_no_count(12usize, 12usize, 0u32, 2135700u32)?;
    emu.lw_no_count(13usize, 10usize, 0u32, 2135704u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2135708u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2135712u32);
    emu.apc_no_count(6usize, 2135712u32, 28672u32, 2135716u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2135720u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(2000u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002096a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2135724u32;
    emu.update_insn_clock();
    emu.lw_no_count(13usize, 12usize, 4294966448u32, 2135728u32)?;
    emu.adi_no_count(12usize, 10usize, 0u32, 2135732u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2135744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002096c0));
    } else {
        emu.pc = 2135736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002096b8));
    }
}
#[inline(always)]
pub fn block_0x002096b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2134016u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2135740u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1300u32, 2135744u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2135744u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002096c0));
}
#[inline(always)]
pub fn block_0x002096c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2135748u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2135752u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2135756u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2135760u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2135764u32;
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
pub fn block_0x002096d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.update_insn_clock();
    return Err(format!("Unimplemented instruction (UNIMP) at PC {:#x}", 2135764u32));
}
#[inline]
pub fn block_0x002096d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2330587136u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2135772u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4112453632u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2135776u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2965131264u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2135780u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2020298752u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2135784u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 428u32, 2135788u32);
    emu.adi_no_count(12usize, 12usize, 1473u32, 2135792u32);
    emu.adi_no_count(13usize, 13usize, 4294965685u32, 2135796u32);
    emu.adi_no_count(14usize, 14usize, 4294965580u32, 2135800u32);
    emu.sw_no_count(14usize, 10usize, 0u32, 2135804u32)?;
    emu.sw_no_count(13usize, 10usize, 4u32, 2135808u32)?;
    emu.sw_no_count(12usize, 10usize, 8u32, 2135812u32)?;
    emu.sw_no_count(11usize, 10usize, 12u32, 2135816u32)?;
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135820u32;
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
pub fn block_0x0020970c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(3112902656u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2135824u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1470513152u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2135828u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1676365824u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2135832u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(3603652608u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2135836u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966129u32, 2135840u32);
    emu.adi_no_count(12usize, 12usize, 376u32, 2135844u32);
    emu.adi_no_count(13usize, 13usize, 44u32, 2135848u32);
    emu.adi_no_count(14usize, 14usize, 4294966637u32, 2135852u32);
    emu.sw_no_count(14usize, 10usize, 0u32, 2135856u32)?;
    emu.sw_no_count(13usize, 10usize, 4u32, 2135860u32)?;
    emu.sw_no_count(12usize, 10usize, 8u32, 2135864u32)?;
    emu.sw_no_count(11usize, 10usize, 12u32, 2135868u32)?;
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135872u32;
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
pub fn block_0x00209740(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2135876u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2135896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209758));
    } else {
        emu.pc = 2135880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209748));
    }
}
#[inline(always)]
pub fn block_0x00209748(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 4u32, 2135884u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2135888u32);
    emu.apc_no_count(6usize, 2135888u32, 4294934528u32, 2135892u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2135896u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965736u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209758(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135900u32;
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
pub fn block_0x0020975c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2135904u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2135908u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2135932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020977c));
    } else {
        emu.pc = 2135912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209768));
    }
}
#[inline(always)]
pub fn block_0x00209768(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2135932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020977c));
    } else {
        emu.pc = 2135916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020976c));
    }
}
#[inline(always)]
pub fn block_0x0020976c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 4u32, 2135920u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2135924u32);
    emu.apc_no_count(6usize, 2135924u32, 4294934528u32, 2135928u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2135932u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965700u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020977c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135936u32;
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
pub fn block_0x00209780(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 255u32, 2135940u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2135944u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2135960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209798));
    } else {
        emu.pc = 2135948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020978c));
    }
}
#[inline(always)]
pub fn block_0x0020978c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 3u32, 2135952u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2135960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209798));
    } else {
        emu.pc = 2135956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209794));
    }
}
#[inline(always)]
pub fn block_0x00209794(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135960u32;
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
pub fn block_0x00209798(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2135964u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2135968u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2135972u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2135976u32)?;
    emu.sw_no_count(18usize, 2usize, 0u32, 2135980u32)?;
    emu.lw_no_count(18usize, 11usize, 4u32, 2135984u32)?;
    emu.lw_no_count(12usize, 18usize, 0u32, 2135988u32)?;
    emu.adi_no_count(8usize, 11usize, 0u32, 2135992u32);
    emu.lw_no_count(9usize, 11usize, 0u32, 2135996u32)?;
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2136008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002097c8));
    } else {
        emu.pc = 2136000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002097c0));
    }
}
#[inline(always)]
pub fn block_0x002097c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2136004u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2136008u32;
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
pub fn block_0x002097c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 18usize, 4u32, 2136012u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2136032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002097e0));
    } else {
        emu.pc = 2136016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002097d0));
    }
}
#[inline(always)]
pub fn block_0x002097d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 8u32, 2136020u32)?;
    emu.adi_no_count(10usize, 9usize, 0u32, 2136024u32);
    emu.apc_no_count(1usize, 2136024u32, 4294934528u32, 2136028u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2136032u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x002097e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 12u32, 2136036u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2136040u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2136044u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2136048u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2136052u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2136056u32)?;
    emu.lw_no_count(18usize, 2usize, 0u32, 2136060u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2136064u32);
    emu.apc_no_count(6usize, 2136064u32, 4294934528u32, 2136068u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2136072u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x00209808(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2136076u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2136080u32;
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
pub fn block_0x00209810(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2136084u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2136088u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2136092u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2136096u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2136100u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2136104u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2136108u32)?;
    emu.lw_no_count(8usize, 10usize, 8u32, 2136112u32)?;
    emu.adi_no_count(12usize, 0usize, 128u32, 2136116u32);
    emu.sri_no_count(19usize, 11usize, 11u32, 2136120u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2136132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209844));
    } else {
        emu.pc = 2136124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020983c));
    }
}
#[inline(always)]
pub fn block_0x0020983c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 1u32, 2136128u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2136132u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2136156u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020985c));
}
#[inline(always)]
pub fn block_0x00209844(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2136144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209850));
    } else {
        emu.pc = 2136136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209848));
    }
}
#[inline(always)]
pub fn block_0x00209848(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 2u32, 2136140u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2136144u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2136156u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020985c));
}
#[inline(always)]
pub fn block_0x00209850(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 11usize, 16u32, 2136148u32);
    emu.sltru_no_count(9usize, 0usize, 12usize, 2136152u32);
    emu.adi_no_count(9usize, 9usize, 3u32, 2136156u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2136156u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020985c));
}
#[inline(always)]
pub fn block_0x0020985c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2136160u32)?;
    emu.sbr_no_count(13usize, 12usize, 8usize, 2136164u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2136168u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2136196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209884));
    } else {
        emu.pc = 2136172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020986c));
    }
}
#[inline(always)]
pub fn block_0x0020986c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 4u32, 2136176u32)?;
    emu.adi_no_count(14usize, 0usize, 128u32, 2136180u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2136184u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2136256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002098c0));
    } else {
        emu.pc = 2136188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020987c));
    }
}
#[inline(always)]
pub fn block_0x0020987c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 12usize, 0u32, 2136192u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2136196u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2136396u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020994c));
}
#[inline(always)]
pub fn block_0x00209884(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2136200u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2136204u32);
    emu.adi_no_count(18usize, 10usize, 0u32, 2136208u32);
    emu.adi_no_count(20usize, 11usize, 0u32, 2136212u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2136216u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2136220u32);
    emu.apc_no_count(1usize, 2136220u32, 4294963200u32, 2136224u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2136228u32;
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
pub fn block_0x002098a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 20usize, 0u32, 2136232u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2136236u32);
    emu.lw_no_count(12usize, 18usize, 8u32, 2136240u32)?;
    emu.lw_no_count(13usize, 18usize, 4u32, 2136244u32)?;
    emu.adi_no_count(14usize, 0usize, 128u32, 2136248u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2136252u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a < b {
        emu.pc = 2136188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020987c));
    } else {
        emu.pc = 2136256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002098c0));
    }
}
#[inline(always)]
pub fn block_0x002098c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2136288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002098e0));
    } else {
        emu.pc = 2136260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002098c4));
    }
}
#[inline(always)]
pub fn block_0x002098c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 6u32, 2136264u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2136268u32);
    emu.ori_no_count(13usize, 13usize, 192u32, 2136272u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2136276u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2136280u32);
    emu.sb_no_count(11usize, 12usize, 1u32, 2136284u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2136288u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2136396u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020994c));
}
#[inline(always)]
pub fn block_0x002098e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 16u32, 2136292u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2136340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209914));
    } else {
        emu.pc = 2136296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002098e8));
    }
}
#[inline]
pub fn block_0x002098e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 12u32, 2136300u32);
    emu.sli_no_count(14usize, 11usize, 20u32, 2136304u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2136308u32);
    emu.ori_no_count(13usize, 13usize, 224u32, 2136312u32);
    emu.sri_no_count(14usize, 14usize, 26u32, 2136316u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2136320u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2136324u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2136328u32);
    emu.sb_no_count(14usize, 12usize, 1u32, 2136332u32);
    emu.sb_no_count(11usize, 12usize, 2u32, 2136336u32);
    emu.add_memory_rw_events(11usize);
    let return_addr = 2136340u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2136396u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020994c));
}
#[inline]
pub fn block_0x00209914(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 18u32, 2136344u32);
    emu.sli_no_count(14usize, 11usize, 14u32, 2136348u32);
    emu.sli_no_count(15usize, 11usize, 20u32, 2136352u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2136356u32);
    emu.ori_no_count(13usize, 13usize, 240u32, 2136360u32);
    emu.sri_no_count(14usize, 14usize, 26u32, 2136364u32);
    emu.sri_no_count(15usize, 15usize, 26u32, 2136368u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2136372u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2136376u32);
    emu.adi_no_count(15usize, 15usize, 128u32, 2136380u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2136384u32);
    emu.sb_no_count(14usize, 12usize, 1u32, 2136388u32);
    emu.sb_no_count(15usize, 12usize, 2u32, 2136392u32);
    emu.sb_no_count(11usize, 12usize, 3u32, 2136396u32);
    emu.add_memory_rw_events(14usize);
    emu.pc = 2136396u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020994c));
}
#[inline]
pub fn block_0x0020994c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 9usize, 8usize, 2136400u32);
    emu.sw_no_count(8usize, 10usize, 8u32, 2136404u32)?;
    emu.adi_no_count(10usize, 0usize, 0u32, 2136408u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2136412u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2136416u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2136420u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2136424u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2136428u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2136432u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2136436u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2136440u32;
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
pub fn block_0x00209978(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2136444u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2136448u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2136452u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2136456u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2136460u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2136464u32)?;
    emu.adi_no_count(8usize, 12usize, 0u32, 2136468u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2136472u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2136476u32)?;
    emu.lw_no_count(18usize, 9usize, 8u32, 2136480u32)?;
    emu.sbr_no_count(10usize, 10usize, 18usize, 2136484u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2136548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002099e4));
    } else {
        emu.pc = 2136488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002099a8));
    }
}
#[inline(always)]
pub fn block_0x002099a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 4u32, 2136492u32)?;
    emu.adr_no_count(10usize, 10usize, 18usize, 2136496u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2136500u32);
    emu.apc_no_count(1usize, 2136500u32, 4294938624u32, 2136504u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2136508u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1728u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002099bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 18usize, 8usize, 2136512u32);
    emu.sw_no_count(8usize, 9usize, 8u32, 2136516u32)?;
    emu.adi_no_count(10usize, 0usize, 0u32, 2136520u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2136524u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2136528u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2136532u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2136536u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2136540u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2136544u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2136548u32;
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
pub fn block_0x002099e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2136552u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2136556u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2136560u32);
    emu.adi_no_count(19usize, 11usize, 0u32, 2136564u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2136568u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2136572u32);
    emu.apc_no_count(1usize, 2136572u32, 4294963200u32, 2136576u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2136580u32;
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
pub fn block_0x00209a04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 19usize, 0u32, 2136584u32);
    emu.lw_no_count(18usize, 9usize, 8u32, 2136588u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2136592u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2136488u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002099a8));
}
#[inline]
pub fn block_0x00209a10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2136596u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2136600u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2136604u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2136608u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2136612u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2136616u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2136620u32)?;
    emu.adi_no_count(8usize, 13usize, 0u32, 2136624u32);
    emu.adi_no_count(19usize, 12usize, 0u32, 2136628u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2136632u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2136636u32)?;
    emu.lw_no_count(20usize, 9usize, 8u32, 2136640u32)?;
    emu.sbr_no_count(11usize, 11usize, 20usize, 2136644u32);
    emu.adi_no_count(18usize, 10usize, 0u32, 2136648u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2136728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209a98));
    } else {
        emu.pc = 2136652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209a4c));
    }
}
#[inline(always)]
pub fn block_0x00209a4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 4u32, 2136656u32)?;
    emu.adr_no_count(10usize, 10usize, 20usize, 2136660u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2136664u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2136668u32);
    emu.apc_no_count(1usize, 2136668u32, 4294938624u32, 2136672u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2136676u32;
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
#[inline]
pub fn block_0x00209a64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(20usize, 20usize, 8usize, 2136680u32);
    emu.sw_no_count(20usize, 9usize, 8u32, 2136684u32)?;
    emu.adi_no_count(10usize, 0usize, 4u32, 2136688u32);
    emu.sb_no_count(10usize, 18usize, 0u32, 2136692u32);
    emu.sw_no_count(8usize, 18usize, 4u32, 2136696u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2136700u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2136704u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2136708u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2136712u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2136716u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2136720u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2136724u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2136728u32;
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
pub fn block_0x00209a98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2136732u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2136736u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2136740u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2136744u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2136748u32);
    emu.apc_no_count(1usize, 2136748u32, 4294963200u32, 2136752u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2136756u32;
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
pub fn block_0x00209ab4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(20usize, 9usize, 8u32, 2136760u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2136764u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2136652u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209a4c));
}
#[inline]
pub fn block_0x00209abc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2136768u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2136772u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2136776u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2136780u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2136784u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2136788u32)?;
    emu.sw_no_count(20usize, 2usize, 24u32, 2136792u32)?;
    emu.sw_no_count(21usize, 2usize, 20u32, 2136796u32)?;
    emu.sw_no_count(22usize, 2usize, 16u32, 2136800u32)?;
    emu.sw_no_count(23usize, 2usize, 12u32, 2136804u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2136808u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2136908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209b4c));
    } else {
        emu.pc = 2136812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209aec));
    }
}
#[inline(always)]
pub fn block_0x00209aec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 12usize, 0u32, 2136816u32);
    emu.adi_no_count(19usize, 11usize, 0u32, 2136820u32);
    emu.adi_no_count(9usize, 0usize, 0u32, 2136824u32);
    emu.sli_no_count(22usize, 13usize, 3u32, 2136828u32);
    emu.adr_no_count(22usize, 12usize, 22usize, 2136832u32);
    emu.adi_no_count(10usize, 12usize, 4u32, 2136836u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2136836u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209b04));
}
#[inline(always)]
pub fn block_0x00209b04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2136840u32)?;
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2136844u32);
    emu.adr_no_count(9usize, 11usize, 9usize, 2136848u32);
    emu.adi_no_count(10usize, 10usize, 8u32, 2136852u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2136836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209b04));
    } else {
        emu.pc = 2136856u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209b18));
    }
}
#[inline(always)]
pub fn block_0x00209b18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 19usize, 0u32, 2136860u32)?;
    emu.lw_no_count(20usize, 19usize, 8u32, 2136864u32)?;
    emu.sbr_no_count(10usize, 10usize, 20usize, 2136868u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2136960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209b80));
    } else {
        emu.pc = 2136872u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209b28));
    }
}
#[inline(always)]
pub fn block_0x00209b28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2136876u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2136880u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2136884u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2136888u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2136892u32);
    emu.apc_no_count(1usize, 2136892u32, 4294963200u32, 2136896u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2136900u32;
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
pub fn block_0x00209b44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(20usize, 19usize, 8u32, 2136904u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2136908u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2136960u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209b80));
}
#[inline(always)]
pub fn block_0x00209b4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2136912u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2136916u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2137016u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209bb8));
}
#[inline(always)]
pub fn block_0x00209b54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2136920u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2136924u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2136928u32);
    emu.adi_no_count(23usize, 11usize, 0u32, 2136932u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2136936u32);
    emu.adi_no_count(12usize, 21usize, 0u32, 2136940u32);
    emu.apc_no_count(1usize, 2136940u32, 4294963200u32, 2136944u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2136948u32;
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
pub fn block_0x00209b74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 23usize, 0u32, 2136952u32);
    emu.lw_no_count(20usize, 19usize, 8u32, 2136956u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2136960u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2136980u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209b94));
}
#[inline(always)]
pub fn block_0x00209b80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 19usize, 0u32, 2136964u32)?;
    emu.lw_no_count(21usize, 18usize, 4u32, 2136968u32)?;
    emu.lw_no_count(11usize, 18usize, 0u32, 2136972u32)?;
    emu.sbr_no_count(10usize, 10usize, 20usize, 2136976u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2136916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209b54));
    } else {
        emu.pc = 2136980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209b94));
    }
}
#[inline(always)]
pub fn block_0x00209b94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 19usize, 4u32, 2136984u32)?;
    emu.adr_no_count(10usize, 10usize, 20usize, 2136988u32);
    emu.adi_no_count(12usize, 21usize, 0u32, 2136992u32);
    emu.apc_no_count(1usize, 2136992u32, 4294938624u32, 2136996u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137000u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1236u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209ba8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(20usize, 20usize, 21usize, 2137004u32);
    emu.adi_no_count(18usize, 18usize, 8u32, 2137008u32);
    emu.sw_no_count(20usize, 19usize, 8u32, 2137012u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2136960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209b80));
    } else {
        emu.pc = 2137016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209bb8));
    }
}
#[inline]
pub fn block_0x00209bb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2137020u32);
    emu.sb_no_count(10usize, 8usize, 0u32, 2137024u32);
    emu.sw_no_count(9usize, 8usize, 4u32, 2137028u32)?;
    emu.lw_no_count(1usize, 2usize, 44u32, 2137032u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2137036u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2137040u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2137044u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2137048u32)?;
    emu.lw_no_count(20usize, 2usize, 24u32, 2137052u32)?;
    emu.lw_no_count(21usize, 2usize, 20u32, 2137056u32)?;
    emu.lw_no_count(22usize, 2usize, 16u32, 2137060u32)?;
    emu.lw_no_count(23usize, 2usize, 12u32, 2137064u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2137068u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137072u32;
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
pub fn block_0x00209bf0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2137076u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137080u32;
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
pub fn block_0x00209bf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2137084u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2137088u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2137092u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2137096u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2137100u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2137104u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2137108u32)?;
    emu.adi_no_count(9usize, 13usize, 0u32, 2137112u32);
    emu.adi_no_count(19usize, 12usize, 0u32, 2137116u32);
    emu.adi_no_count(8usize, 11usize, 0u32, 2137120u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2137124u32)?;
    emu.lw_no_count(20usize, 8usize, 8u32, 2137128u32)?;
    emu.sbr_no_count(11usize, 11usize, 20usize, 2137132u32);
    emu.adi_no_count(18usize, 10usize, 0u32, 2137136u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2137212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209c7c));
    } else {
        emu.pc = 2137140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209c34));
    }
}
#[inline(always)]
pub fn block_0x00209c34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 4u32, 2137144u32)?;
    emu.adr_no_count(10usize, 10usize, 20usize, 2137148u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2137152u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2137156u32);
    emu.apc_no_count(1usize, 2137156u32, 4294938624u32, 2137160u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137164u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1072u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00209c4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(9usize, 20usize, 9usize, 2137168u32);
    emu.adi_no_count(10usize, 0usize, 4u32, 2137172u32);
    emu.sw_no_count(9usize, 8usize, 8u32, 2137176u32)?;
    emu.sb_no_count(10usize, 18usize, 0u32, 2137180u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2137184u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2137188u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2137192u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2137196u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2137200u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2137204u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2137208u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137212u32;
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
pub fn block_0x00209c7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2137216u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2137220u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2137224u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2137228u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2137232u32);
    emu.apc_no_count(1usize, 2137232u32, 4294963200u32, 2137236u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137240u32;
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
pub fn block_0x00209c98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(20usize, 8usize, 8u32, 2137244u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2137248u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2137140u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209c34));
}
#[inline]
pub fn block_0x00209ca0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2137252u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2137256u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2137260u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2137264u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2137268u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2137272u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2137276u32)?;
    emu.sw_no_count(21usize, 2usize, 4u32, 2137280u32)?;
    emu.sw_no_count(22usize, 2usize, 0u32, 2137284u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2137288u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2137484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209d8c));
    } else {
        emu.pc = 2137292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209ccc));
    }
}
#[inline(always)]
pub fn block_0x00209ccc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 12usize, 0u32, 2137296u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2137300u32);
    emu.adi_no_count(12usize, 0usize, 0u32, 2137304u32);
    emu.sli_no_count(21usize, 13usize, 3u32, 2137308u32);
    emu.adr_no_count(21usize, 9usize, 21usize, 2137312u32);
    emu.adi_no_count(10usize, 9usize, 4u32, 2137316u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2137316u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209ce4));
}
#[inline(always)]
pub fn block_0x00209ce4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2137320u32)?;
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2137324u32);
    emu.adr_no_count(12usize, 11usize, 12usize, 2137328u32);
    emu.adi_no_count(10usize, 10usize, 8u32, 2137332u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2137316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209ce4));
    } else {
        emu.pc = 2137336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209cf8));
    }
}
#[inline(always)]
pub fn block_0x00209cf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 0u32, 2137340u32)?;
    emu.lw_no_count(19usize, 18usize, 8u32, 2137344u32)?;
    emu.sbr_no_count(10usize, 10usize, 19usize, 2137348u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2137428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209d54));
    } else {
        emu.pc = 2137352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209d08));
    }
}
#[inline(always)]
pub fn block_0x00209d08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2137356u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2137360u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2137364u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2137368u32);
    emu.apc_no_count(1usize, 2137368u32, 4294963200u32, 2137372u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137376u32;
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
pub fn block_0x00209d20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(19usize, 18usize, 8u32, 2137380u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2137384u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2137428u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209d54));
}
#[inline(always)]
pub fn block_0x00209d28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2137388u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2137392u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2137396u32);
    emu.adi_no_count(22usize, 11usize, 0u32, 2137400u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2137404u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2137408u32);
    emu.apc_no_count(1usize, 2137408u32, 4294963200u32, 2137412u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137416u32;
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
pub fn block_0x00209d48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 22usize, 0u32, 2137420u32);
    emu.lw_no_count(19usize, 18usize, 8u32, 2137424u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2137428u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2137448u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209d68));
}
#[inline(always)]
pub fn block_0x00209d54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 0u32, 2137432u32)?;
    emu.lw_no_count(20usize, 9usize, 4u32, 2137436u32)?;
    emu.lw_no_count(11usize, 9usize, 0u32, 2137440u32)?;
    emu.sbr_no_count(10usize, 10usize, 19usize, 2137444u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2137384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209d28));
    } else {
        emu.pc = 2137448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209d68));
    }
}
#[inline(always)]
pub fn block_0x00209d68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 4u32, 2137452u32)?;
    emu.adr_no_count(10usize, 10usize, 19usize, 2137456u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2137460u32);
    emu.apc_no_count(1usize, 2137460u32, 4294938624u32, 2137464u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137468u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(768u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209d7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(19usize, 19usize, 20usize, 2137472u32);
    emu.adi_no_count(9usize, 9usize, 8u32, 2137476u32);
    emu.sw_no_count(19usize, 18usize, 8u32, 2137480u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2137428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209d54));
    } else {
        emu.pc = 2137484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209d8c));
    }
}
#[inline]
pub fn block_0x00209d8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2137488u32);
    emu.sb_no_count(10usize, 8usize, 0u32, 2137492u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2137496u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2137500u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2137504u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2137508u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2137512u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2137516u32)?;
    emu.lw_no_count(21usize, 2usize, 4u32, 2137520u32)?;
    emu.lw_no_count(22usize, 2usize, 0u32, 2137524u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2137528u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137532u32;
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
pub fn block_0x00209dbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 4u32, 2137536u32);
    emu.sb_no_count(11usize, 10usize, 0u32, 2137540u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137544u32;
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
pub fn block_0x00209dc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2137548u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137552u32;
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
pub fn block_0x00209dd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(5usize, 2137552u32, 4096u32, 2137556u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(5usize);
    let return_addr = 2137560u32;
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
pub fn block_0x00209dd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 10usize, 0u32, 2137564u32);
    emu.lbu_no_count(10usize, 10usize, 13u32, 2137568u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2137580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209dec));
    } else {
        emu.pc = 2137572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209de4));
    }
}
#[inline(always)]
pub fn block_0x00209de4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2137572u32, 4096u32, 2137576u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137580u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(832u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209dec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 8usize, 8u32, 2137584u32)?;
    emu.lw_no_count(10usize, 8usize, 0u32, 2137588u32)?;
    emu.lw_no_count(11usize, 8usize, 4u32, 2137592u32)?;
    emu.adi_no_count(13usize, 0usize, 3u32, 2137596u32);
    emu.sb_no_count(13usize, 2usize, 7u32, 2137600u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2137604u32)?;
    emu.apc_no_count(1usize, 2137604u32, 0u32, 2137608u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137612u32;
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
pub fn block_0x00209e0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 12u32, 2137616u32)?;
    emu.sw_no_count(11usize, 2usize, 16u32, 2137620u32)?;
    emu.adi_no_count(11usize, 2usize, 8u32, 2137624u32);
    emu.adi_no_count(12usize, 2usize, 12u32, 2137628u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2137632u32;
    emu.update_insn_clock();
    emu.lbu_no_count(13usize, 10usize, 4294966440u32, 2137636u32);
    emu.adi_no_count(14usize, 2usize, 7u32, 2137640u32);
    emu.sw_no_count(11usize, 2usize, 20u32, 2137644u32)?;
    emu.sw_no_count(12usize, 2usize, 24u32, 2137648u32)?;
    emu.sw_no_count(14usize, 2usize, 28u32, 2137652u32)?;
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2137784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209eb8));
    } else {
        emu.pc = 2137656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209e38));
    }
}
#[inline(always)]
pub fn block_0x00209e38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(18usize, a);
    emu.pc = 2137660u32;
    emu.update_insn_clock();
    emu.lw_no_count(19usize, 18usize, 4294966444u32, 2137664u32)?;
    emu.adi_no_count(9usize, 0usize, 1u32, 2137668u32);
    emu.sb_no_count(9usize, 10usize, 4294966440u32, 2137672u32);
    emu.sw_no_count(0usize, 18usize, 4294966444u32, 2137676u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2137784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209eb8));
    } else {
        emu.pc = 2137680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209e50));
    }
}
#[inline(always)]
pub fn block_0x00209e50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 19usize, 8u32, 2137684u32);
    emu.apc_no_count(1usize, 2137684u32, 4096u32, 2137688u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137692u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967004u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209e5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 11usize, 0u32, 2137696u32);
    emu.adi_no_count(11usize, 11usize, 4u32, 2137700u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2137704u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966312u32, 2137708u32);
    emu.adi_no_count(10usize, 2usize, 20u32, 2137712u32);
    emu.apc_no_count(1usize, 2137712u32, 0u32, 2137716u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137720u32;
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
pub fn block_0x00209e78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(0usize, 8usize, 0u32, 2137724u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2137728u32;
    emu.update_insn_clock();
    emu.lw_no_count(10usize, 18usize, 4294966444u32, 2137732u32)?;
    emu.sb_no_count(9usize, 11usize, 4294966440u32, 2137736u32);
    emu.sw_no_count(19usize, 18usize, 4294966444u32, 2137740u32)?;
    emu.sw_no_count(9usize, 2usize, 32u32, 2137744u32)?;
    emu.sw_no_count(10usize, 2usize, 36u32, 2137748u32)?;
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2137808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209ed0));
    } else {
        emu.pc = 2137752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209e98));
    }
}
#[inline(always)]
pub fn block_0x00209e98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2137756u32)?;
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2137760u32);
    emu.sw_no_count(11usize, 10usize, 0u32, 2137764u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2137808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209ed0));
    } else {
        emu.pc = 2137768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209ea8));
    }
}
#[inline(always)]
pub fn block_0x00209ea8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 36u32, 2137772u32);
    emu.apc_no_count(1usize, 2137772u32, 4096u32, 2137776u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137780u32;
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
pub fn block_0x00209eb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2137784u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2137808u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209ed0));
}
#[inline(always)]
pub fn block_0x00209eb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2137788u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966352u32, 2137792u32);
    emu.adi_no_count(10usize, 2usize, 20u32, 2137796u32);
    emu.adi_no_count(11usize, 2usize, 43u32, 2137800u32);
    emu.apc_no_count(1usize, 2137800u32, 0u32, 2137804u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137808u32;
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
pub fn block_0x00209ed0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2137808u32, 4096u32, 2137812u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2137816u32;
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
