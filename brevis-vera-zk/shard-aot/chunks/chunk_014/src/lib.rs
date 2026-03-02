pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2152392u32;
pub const PC_MAX: u32 = 2154412u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 112usize] = [
        block_0x0020d7c8,
        block_0x0020d800,
        block_0x0020d80c,
        block_0x0020d828,
        block_0x0020d858,
        block_0x0020d860,
        block_0x0020d868,
        block_0x0020d870,
        block_0x0020d884,
        block_0x0020d8ac,
        block_0x0020d8b4,
        block_0x0020d8bc,
        block_0x0020d8d4,
        block_0x0020d8dc,
        block_0x0020d8e8,
        block_0x0020d908,
        block_0x0020d90c,
        block_0x0020d93c,
        block_0x0020d944,
        block_0x0020d95c,
        block_0x0020d960,
        block_0x0020d970,
        block_0x0020d974,
        block_0x0020d97c,
        block_0x0020d98c,
        block_0x0020d990,
        block_0x0020d9a0,
        block_0x0020d9a8,
        block_0x0020d9c8,
        block_0x0020d9d0,
        block_0x0020d9ec,
        block_0x0020da04,
        block_0x0020da24,
        block_0x0020da2c,
        block_0x0020da3c,
        block_0x0020da44,
        block_0x0020da4c,
        block_0x0020da58,
        block_0x0020da74,
        block_0x0020da84,
        block_0x0020da94,
        block_0x0020dad0,
        block_0x0020dae8,
        block_0x0020db04,
        block_0x0020db20,
        block_0x0020db5c,
        block_0x0020db7c,
        block_0x0020dbac,
        block_0x0020dbb4,
        block_0x0020dbbc,
        block_0x0020dbc4,
        block_0x0020dbd8,
        block_0x0020dc00,
        block_0x0020dc08,
        block_0x0020dc10,
        block_0x0020dc28,
        block_0x0020dc30,
        block_0x0020dc3c,
        block_0x0020dc5c,
        block_0x0020dc60,
        block_0x0020dc90,
        block_0x0020dc98,
        block_0x0020dcb0,
        block_0x0020dcb4,
        block_0x0020dcc4,
        block_0x0020dcc8,
        block_0x0020dcd0,
        block_0x0020dce0,
        block_0x0020dce4,
        block_0x0020dcf4,
        block_0x0020dcfc,
        block_0x0020dd20,
        block_0x0020dd28,
        block_0x0020dd30,
        block_0x0020dd34,
        block_0x0020dd5c,
        block_0x0020dd64,
        block_0x0020dd84,
        block_0x0020dd88,
        block_0x0020dda4,
        block_0x0020ddac,
        block_0x0020ddc8,
        block_0x0020ddd0,
        block_0x0020dde0,
        block_0x0020dde8,
        block_0x0020ddf0,
        block_0x0020ddfc,
        block_0x0020de18,
        block_0x0020de30,
        block_0x0020de40,
        block_0x0020de50,
        block_0x0020de90,
        block_0x0020deac,
        block_0x0020dec8,
        block_0x0020ded4,
        block_0x0020dee0,
        block_0x0020deec,
        block_0x0020def0,
        block_0x0020def4,
        block_0x0020df00,
        block_0x0020df08,
        block_0x0020df14,
        block_0x0020df1c,
        block_0x0020df38,
        block_0x0020df6c,
        block_0x0020df74,
        block_0x0020df78,
        block_0x0020df84,
        block_0x0020df8c,
        block_0x0020df98,
        block_0x0020dfa0,
        block_0x0020dfac,
    ];
    const IDX: [u16; 506usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 2u16, 0u16, 0u16, 3u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 4u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 5u16, 0u16, 6u16,
        0u16, 7u16, 0u16, 8u16, 0u16, 0u16, 0u16, 0u16, 9u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 10u16, 0u16, 11u16, 0u16, 12u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 13u16, 0u16, 14u16, 0u16, 0u16, 15u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 16u16, 17u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 18u16, 0u16, 19u16, 0u16, 0u16, 0u16, 0u16, 0u16, 20u16, 21u16, 0u16,
        0u16, 0u16, 22u16, 23u16, 0u16, 24u16, 0u16, 0u16, 0u16, 25u16, 26u16, 0u16,
        0u16, 0u16, 27u16, 0u16, 28u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 29u16,
        0u16, 30u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 31u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 32u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 33u16, 0u16, 34u16, 0u16,
        0u16, 0u16, 35u16, 0u16, 36u16, 0u16, 37u16, 0u16, 0u16, 38u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 39u16, 0u16, 0u16, 0u16, 40u16, 0u16, 0u16, 0u16, 41u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        42u16, 0u16, 0u16, 0u16, 0u16, 0u16, 43u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        44u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 45u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 46u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 47u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 48u16, 0u16, 49u16, 0u16, 50u16, 0u16, 51u16, 0u16, 0u16, 0u16,
        0u16, 52u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 53u16, 0u16,
        54u16, 0u16, 55u16, 0u16, 0u16, 0u16, 0u16, 0u16, 56u16, 0u16, 57u16, 0u16, 0u16,
        58u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 59u16, 60u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 61u16, 0u16, 62u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 63u16, 64u16, 0u16, 0u16, 0u16, 65u16, 66u16, 0u16, 67u16,
        0u16, 0u16, 0u16, 68u16, 69u16, 0u16, 0u16, 0u16, 70u16, 0u16, 71u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 72u16, 0u16, 73u16, 0u16, 74u16, 75u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 76u16, 0u16, 77u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 78u16, 79u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        80u16, 0u16, 81u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 82u16, 0u16, 83u16, 0u16,
        0u16, 0u16, 84u16, 0u16, 85u16, 0u16, 86u16, 0u16, 0u16, 87u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 88u16, 0u16, 0u16, 0u16, 0u16, 0u16, 89u16, 0u16, 0u16, 0u16,
        90u16, 0u16, 0u16, 0u16, 91u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 92u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 93u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 94u16, 0u16, 0u16, 95u16, 0u16,
        0u16, 96u16, 0u16, 0u16, 97u16, 98u16, 99u16, 0u16, 0u16, 100u16, 0u16, 101u16,
        0u16, 0u16, 102u16, 0u16, 103u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 104u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 105u16,
        0u16, 106u16, 107u16, 0u16, 0u16, 108u16, 0u16, 109u16, 0u16, 0u16, 110u16, 0u16,
        111u16, 0u16, 0u16, 112u16,
    ];
    if pc < 2152392u32 || pc > 2154412u32 {
        return None;
    }
    let word_offset = ((pc - 2152392u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x0020d7c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967200u32, 2152396u32);
    emu.sw_no_count(1usize, 2usize, 92u32, 2152400u32)?;
    emu.sw_no_count(8usize, 2usize, 88u32, 2152404u32)?;
    emu.sw_no_count(9usize, 2usize, 84u32, 2152408u32)?;
    emu.sw_no_count(18usize, 2usize, 80u32, 2152412u32)?;
    emu.sw_no_count(19usize, 2usize, 76u32, 2152416u32)?;
    emu.sw_no_count(20usize, 2usize, 72u32, 2152420u32)?;
    emu.sw_no_count(21usize, 2usize, 68u32, 2152424u32)?;
    emu.sw_no_count(22usize, 2usize, 64u32, 2152428u32)?;
    emu.sw_no_count(23usize, 2usize, 60u32, 2152432u32)?;
    emu.adi_no_count(18usize, 14usize, 0u32, 2152436u32);
    emu.lw_no_count(19usize, 2usize, 96u32, 2152440u32)?;
    emu.adi_no_count(14usize, 0usize, 3u32, 2152444u32);
    emu.add_memory_rw_events(13usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a >= b {
        emu.pc = 2153192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dae8));
    } else {
        emu.pc = 2152448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d800));
    }
}
#[inline(always)]
pub fn block_0x0020d800(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2152452u32);
    emu.adi_no_count(10usize, 0usize, 16u32, 2152456u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2153220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020db04));
    } else {
        emu.pc = 2152460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d80c));
    }
}
#[inline(always)]
pub fn block_0x0020d80c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 17usize, 0u32, 2152464u32);
    emu.sli_no_count(6usize, 12usize, 1u32, 2152468u32);
    let a = 0u32.wrapping_add(1048576u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2152472u32;
    emu.update_insn_clock();
    emu.sri_no_count(17usize, 6usize, 21u32, 2152476u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2152480u32);
    emu.anr_no_count(5usize, 12usize, 10usize, 2152484u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2152580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d884));
    } else {
        emu.pc = 2152488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d828));
    }
}
#[inline]
pub fn block_0x0020d828(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(14usize, 0usize, 11usize, 2152492u32);
    emu.adi_no_count(7usize, 11usize, 4294967295u32, 2152496u32);
    emu.adr_no_count(28usize, 5usize, 14usize, 2152500u32);
    emu.adi_no_count(14usize, 7usize, 1u32, 2152504u32);
    emu.sltiu_no_count(7usize, 14usize, 1u32, 2152508u32);
    emu.adr_no_count(10usize, 7usize, 10usize, 2152512u32);
    emu.adr_no_count(10usize, 28usize, 10usize, 2152516u32);
    emu.sri_no_count(7usize, 6usize, 1u32, 2152520u32);
    let a = 0u32.wrapping_add(2146435072u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2152524u32;
    emu.update_insn_clock();
    emu.xrr_no_count(7usize, 7usize, 6usize, 2152528u32);
    emu.orr_no_count(7usize, 11usize, 7usize, 2152532u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2152620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d8ac));
    } else {
        emu.pc = 2152536u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d858));
    }
}
#[inline(always)]
pub fn block_0x0020d858(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(28usize, 12usize, 6usize, 2152540u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2152628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d8b4));
    } else {
        emu.pc = 2152544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d860));
    }
}
#[inline(always)]
pub fn block_0x0020d860(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(7usize, 14usize, 1u32, 2152548u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a != b {
        emu.pc = 2152636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d8bc));
    } else {
        emu.pc = 2152552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d868));
    }
}
#[inline(always)]
pub fn block_0x0020d868(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.orr_no_count(11usize, 11usize, 5usize, 2152556u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2152712u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d908));
    } else {
        emu.pc = 2152560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d870));
    }
}
#[inline(always)]
pub fn block_0x0020d870(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(6usize, 0usize, 0u32, 2152564u32);
    emu.adi_no_count(17usize, 17usize, 4294966221u32, 2152568u32);
    emu.xri_no_count(11usize, 7usize, 1u32, 2152572u32);
    emu.adi_no_count(5usize, 0usize, 1u32, 2152576u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2152580u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2152716u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d90c));
}
#[inline]
pub fn block_0x0020d884(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 31u32, 2152584u32);
    emu.orr_no_count(10usize, 6usize, 10usize, 2152588u32);
    emu.sli_no_count(14usize, 11usize, 1u32, 2152592u32);
    emu.sli_no_count(10usize, 10usize, 11u32, 2152596u32);
    emu.sri_no_count(10usize, 10usize, 11u32, 2152600u32);
    emu.sri_no_count(7usize, 6usize, 1u32, 2152604u32);
    let a = 0u32.wrapping_add(2146435072u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2152608u32;
    emu.update_insn_clock();
    emu.xrr_no_count(7usize, 7usize, 6usize, 2152612u32);
    emu.orr_no_count(7usize, 11usize, 7usize, 2152616u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a != b {
        emu.pc = 2152536u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d858));
    } else {
        emu.pc = 2152620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d8ac));
    }
}
#[inline(always)]
pub fn block_0x0020d8ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 3u32, 2152624u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2152628u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2152716u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d90c));
}
#[inline(always)]
pub fn block_0x0020d8b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 2u32, 2152632u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2152636u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2152716u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d90c));
}
#[inline(always)]
pub fn block_0x0020d8bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(1048576u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2152640u32;
    emu.update_insn_clock();
    emu.xrr_no_count(11usize, 10usize, 11usize, 2152644u32);
    emu.orr_no_count(5usize, 14usize, 11usize, 2152648u32);
    emu.sltiu_no_count(11usize, 5usize, 1u32, 2152652u32);
    emu.sli_no_count(6usize, 14usize, 1u32, 2152656u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a != b {
        emu.pc = 2152668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d8dc));
    } else {
        emu.pc = 2152660u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d8d4));
    }
}
#[inline(always)]
pub fn block_0x0020d8d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(4194304u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2152664u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    let return_addr = 2152668u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2152680u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d8e8));
}
#[inline(always)]
pub fn block_0x0020d8dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(14usize, 14usize, 31u32, 2152672u32);
    emu.sli_no_count(10usize, 10usize, 1u32, 2152676u32);
    emu.orr_no_count(10usize, 10usize, 14usize, 2152680u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2152680u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d8e8));
}
#[inline(always)]
pub fn block_0x0020d8e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 11usize, 4294967295u32, 2152684u32);
    emu.adi_no_count(5usize, 11usize, 1u32, 2152688u32);
    emu.sbr_no_count(11usize, 17usize, 11usize, 2152692u32);
    emu.anr_no_count(14usize, 14usize, 6usize, 2152696u32);
    emu.sltiu_no_count(6usize, 5usize, 1u32, 2152700u32);
    emu.adi_no_count(17usize, 11usize, 4294966220u32, 2152704u32);
    emu.xri_no_count(11usize, 7usize, 1u32, 2152708u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2152712u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2152716u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d90c));
}
#[inline(always)]
pub fn block_0x0020d908(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 4u32, 2152716u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2152716u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d90c));
}
#[inline]
pub fn block_0x0020d90c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(7usize, 11usize, 255u32, 2152720u32);
    emu.sri_no_count(12usize, 12usize, 31u32, 2152724u32);
    emu.adi_no_count(22usize, 0usize, 1u32, 2152728u32);
    emu.sw_no_count(14usize, 2usize, 0u32, 2152732u32)?;
    emu.sw_no_count(10usize, 2usize, 4u32, 2152736u32)?;
    emu.sw_no_count(22usize, 2usize, 8u32, 2152740u32)?;
    emu.sw_no_count(0usize, 2usize, 12u32, 2152744u32)?;
    emu.sw_no_count(5usize, 2usize, 16u32, 2152748u32)?;
    emu.sw_no_count(6usize, 2usize, 20u32, 2152752u32)?;
    emu.sh_no_count(17usize, 2usize, 24u32, 2152756u32)?;
    emu.sb_no_count(11usize, 2usize, 26u32, 2152760u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a >= b {
        emu.pc = 2152796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d95c));
    } else {
        emu.pc = 2152764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d93c));
    }
}
#[inline(always)]
pub fn block_0x0020d93c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2152768u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a != b {
        emu.pc = 2152820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d974));
    } else {
        emu.pc = 2152772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d944));
    }
}
#[inline(always)]
pub fn block_0x0020d944(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(22usize, 0usize, 0u32, 2152776u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2152780u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 522u32, 2152784u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2152788u32);
    emu.adi_no_count(23usize, 0usize, 1u32, 2152792u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2152796u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2153092u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020da84));
}
#[inline(always)]
pub fn block_0x0020d95c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2152848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d990));
    } else {
        emu.pc = 2152800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d960));
    }
}
#[inline(always)]
pub fn block_0x0020d960(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(23usize, a);
    emu.pc = 2152804u32;
    emu.update_insn_clock();
    emu.adi_no_count(23usize, 23usize, 521u32, 2152808u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2152812u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2152864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d9a0));
    } else {
        emu.pc = 2152816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d970));
    }
}
#[inline(always)]
pub fn block_0x0020d970(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2152820u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2152872u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d9a8));
}
#[inline(always)]
pub fn block_0x0020d974(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(22usize, 0usize, 1u32, 2152824u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2153004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020da2c));
    } else {
        emu.pc = 2152828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d97c));
    }
}
#[inline(always)]
pub fn block_0x0020d97c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(23usize, a);
    emu.pc = 2152832u32;
    emu.update_insn_clock();
    emu.adi_no_count(23usize, 23usize, 521u32, 2152836u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2152840u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2153020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020da3c));
    } else {
        emu.pc = 2152844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d98c));
    }
}
#[inline(always)]
pub fn block_0x0020d98c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2152848u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2153028u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020da44));
}
#[inline(always)]
pub fn block_0x0020d990(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(23usize, a);
    emu.pc = 2152852u32;
    emu.update_insn_clock();
    emu.adi_no_count(23usize, 23usize, 520u32, 2152856u32);
    emu.adi_no_count(10usize, 23usize, 0u32, 2152860u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2152872u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d9a8));
    } else {
        emu.pc = 2152864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d9a0));
    }
}
#[inline(always)]
pub fn block_0x0020d9a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(22usize, 12usize, 0u32, 2152868u32);
    emu.adi_no_count(23usize, 10usize, 0u32, 2152872u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2152872u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d9a8));
}
#[inline(always)]
pub fn block_0x0020d9a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 48u32, 2152876u32);
    emu.adi_no_count(11usize, 2usize, 0u32, 2152880u32);
    emu.adi_no_count(21usize, 15usize, 0u32, 2152884u32);
    emu.adi_no_count(12usize, 15usize, 0u32, 2152888u32);
    emu.adi_no_count(20usize, 16usize, 0u32, 2152892u32);
    emu.adi_no_count(13usize, 16usize, 0u32, 2152896u32);
    emu.apc_no_count(1usize, 2152896u32, 0u32, 2152900u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2152904u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1904u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020d9c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 48u32, 2152908u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2152940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d9ec));
    } else {
        emu.pc = 2152912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d9d0));
    }
}
#[inline(always)]
pub fn block_0x0020d9d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 48u32, 2152916u32)?;
    emu.lw_no_count(11usize, 2usize, 52u32, 2152920u32)?;
    emu.lw_no_count(12usize, 2usize, 56u32, 2152924u32)?;
    emu.sw_no_count(10usize, 2usize, 36u32, 2152928u32)?;
    emu.sw_no_count(11usize, 2usize, 40u32, 2152932u32)?;
    emu.sw_no_count(12usize, 2usize, 44u32, 2152936u32)?;
    emu.add_memory_rw_events(7usize);
    let return_addr = 2152940u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2152964u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020da04));
}
#[inline(always)]
pub fn block_0x0020d9ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 36u32, 2152944u32);
    emu.adi_no_count(11usize, 2usize, 0u32, 2152948u32);
    emu.adi_no_count(12usize, 21usize, 0u32, 2152952u32);
    emu.adi_no_count(13usize, 20usize, 0u32, 2152956u32);
    emu.apc_no_count(1usize, 2152956u32, 16384u32, 2152960u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2152964u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1112u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020da04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 36u32, 2152968u32)?;
    emu.lw_no_count(11usize, 2usize, 40u32, 2152972u32)?;
    emu.lh_no_count(12usize, 2usize, 44u32, 2152976u32)?;
    emu.adi_no_count(13usize, 18usize, 0u32, 2152980u32);
    emu.adi_no_count(14usize, 8usize, 0u32, 2152984u32);
    emu.adi_no_count(15usize, 19usize, 0u32, 2152988u32);
    emu.apc_no_count(1usize, 2152988u32, 0u32, 2152992u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2152996u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966352u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020da24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 10usize, 0u32, 2153000u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2153004u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2153108u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020da94));
}
#[inline(always)]
pub fn block_0x0020da2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(23usize, a);
    emu.pc = 2153008u32;
    emu.update_insn_clock();
    emu.adi_no_count(23usize, 23usize, 520u32, 2153012u32);
    emu.adi_no_count(10usize, 23usize, 0u32, 2153016u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2153028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020da44));
    } else {
        emu.pc = 2153020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020da3c));
    }
}
#[inline(always)]
pub fn block_0x0020da3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(22usize, 12usize, 0u32, 2153024u32);
    emu.adi_no_count(23usize, 10usize, 0u32, 2153028u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2153028u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020da44));
}
#[inline(always)]
pub fn block_0x0020da44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2153032u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a != b {
        emu.pc = 2153076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020da74));
    } else {
        emu.pc = 2153036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020da4c));
    }
}
#[inline(always)]
pub fn block_0x0020da4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 2u32, 2153040u32);
    emu.sh_no_count(11usize, 8usize, 0u32, 2153044u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2153168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dad0));
    } else {
        emu.pc = 2153048u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020da58));
    }
}
#[inline(always)]
pub fn block_0x0020da58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2153052u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 437u32, 2153056u32);
    emu.sw_no_count(10usize, 8usize, 4u32, 2153060u32)?;
    emu.sw_no_count(11usize, 8usize, 8u32, 2153064u32)?;
    emu.sh_no_count(0usize, 8usize, 12u32, 2153068u32)?;
    emu.sw_no_count(18usize, 8usize, 16u32, 2153072u32)?;
    emu.add_memory_rw_events(7usize);
    let return_addr = 2153076u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2153108u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020da94));
}
#[inline(always)]
pub fn block_0x0020da74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2153080u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2153084u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 525u32, 2153088u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2153092u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2153092u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020da84));
}
#[inline(always)]
pub fn block_0x0020da84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sh_no_count(10usize, 8usize, 0u32, 2153096u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2153100u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2153104u32)?;
    emu.adi_no_count(11usize, 0usize, 1u32, 2153108u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2153108u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020da94));
}
#[inline]
pub fn block_0x0020da94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(23usize, 9usize, 0u32, 2153112u32)?;
    emu.sw_no_count(22usize, 9usize, 4u32, 2153116u32)?;
    emu.sw_no_count(8usize, 9usize, 8u32, 2153120u32)?;
    emu.sw_no_count(11usize, 9usize, 12u32, 2153124u32)?;
    emu.lw_no_count(1usize, 2usize, 92u32, 2153128u32)?;
    emu.lw_no_count(8usize, 2usize, 88u32, 2153132u32)?;
    emu.lw_no_count(9usize, 2usize, 84u32, 2153136u32)?;
    emu.lw_no_count(18usize, 2usize, 80u32, 2153140u32)?;
    emu.lw_no_count(19usize, 2usize, 76u32, 2153144u32)?;
    emu.lw_no_count(20usize, 2usize, 72u32, 2153148u32)?;
    emu.lw_no_count(21usize, 2usize, 68u32, 2153152u32)?;
    emu.lw_no_count(22usize, 2usize, 64u32, 2153156u32)?;
    emu.lw_no_count(23usize, 2usize, 60u32, 2153160u32)?;
    emu.adi_no_count(2usize, 2usize, 96u32, 2153164u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2153168u32;
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
pub fn block_0x0020dad0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2153172u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 528u32, 2153176u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2153180u32);
    emu.sw_no_count(10usize, 8usize, 4u32, 2153184u32)?;
    emu.sw_no_count(11usize, 8usize, 8u32, 2153188u32)?;
    emu.add_memory_rw_events(6usize);
    let return_addr = 2153192u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2153108u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020da94));
}
#[inline(always)]
pub fn block_0x0020dae8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2153196u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 384u32, 2153200u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2153204u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 440u32, 2153208u32);
    emu.adi_no_count(11usize, 0usize, 34u32, 2153212u32);
    emu.apc_no_count(1usize, 2153212u32, 4294959104u32, 2153216u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2153220u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1776u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020db04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2153224u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 456u32, 2153228u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2153232u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 504u32, 2153236u32);
    emu.adi_no_count(11usize, 0usize, 45u32, 2153240u32);
    emu.apc_no_count(1usize, 2153240u32, 4294959104u32, 2153244u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2153248u32;
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
pub fn block_0x0020db20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967200u32, 2153252u32);
    emu.sw_no_count(1usize, 2usize, 92u32, 2153256u32)?;
    emu.sw_no_count(8usize, 2usize, 88u32, 2153260u32)?;
    emu.sw_no_count(9usize, 2usize, 84u32, 2153264u32)?;
    emu.sw_no_count(18usize, 2usize, 80u32, 2153268u32)?;
    emu.sw_no_count(19usize, 2usize, 76u32, 2153272u32)?;
    emu.sw_no_count(20usize, 2usize, 72u32, 2153276u32)?;
    emu.sw_no_count(21usize, 2usize, 68u32, 2153280u32)?;
    emu.sw_no_count(22usize, 2usize, 64u32, 2153284u32)?;
    emu.sw_no_count(23usize, 2usize, 60u32, 2153288u32)?;
    emu.sw_no_count(24usize, 2usize, 56u32, 2153292u32)?;
    emu.adi_no_count(18usize, 14usize, 0u32, 2153296u32);
    emu.lw_no_count(19usize, 2usize, 96u32, 2153300u32)?;
    emu.adi_no_count(14usize, 0usize, 3u32, 2153304u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a >= b {
        emu.pc = 2154128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020de90));
    } else {
        emu.pc = 2153308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020db5c));
    }
}
#[inline(always)]
pub fn block_0x0020db5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 17usize, 0u32, 2153312u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2153316u32);
    emu.sli_no_count(6usize, 12usize, 1u32, 2153320u32);
    let a = 0u32.wrapping_add(1048576u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2153324u32;
    emu.update_insn_clock();
    emu.sri_no_count(17usize, 6usize, 21u32, 2153328u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2153332u32);
    emu.anr_no_count(5usize, 12usize, 10usize, 2153336u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2153432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dbd8));
    } else {
        emu.pc = 2153340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020db7c));
    }
}
#[inline]
pub fn block_0x0020db7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(14usize, 0usize, 11usize, 2153344u32);
    emu.adi_no_count(7usize, 11usize, 4294967295u32, 2153348u32);
    emu.adr_no_count(28usize, 5usize, 14usize, 2153352u32);
    emu.adi_no_count(14usize, 7usize, 1u32, 2153356u32);
    emu.sltiu_no_count(7usize, 14usize, 1u32, 2153360u32);
    emu.adr_no_count(10usize, 7usize, 10usize, 2153364u32);
    emu.adr_no_count(10usize, 28usize, 10usize, 2153368u32);
    emu.sri_no_count(7usize, 6usize, 1u32, 2153372u32);
    let a = 0u32.wrapping_add(2146435072u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2153376u32;
    emu.update_insn_clock();
    emu.xrr_no_count(7usize, 7usize, 6usize, 2153380u32);
    emu.orr_no_count(7usize, 11usize, 7usize, 2153384u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2153472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dc00));
    } else {
        emu.pc = 2153388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dbac));
    }
}
#[inline(always)]
pub fn block_0x0020dbac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(28usize, 12usize, 6usize, 2153392u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2153480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dc08));
    } else {
        emu.pc = 2153396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dbb4));
    }
}
#[inline(always)]
pub fn block_0x0020dbb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(7usize, 14usize, 1u32, 2153400u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a != b {
        emu.pc = 2153488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dc10));
    } else {
        emu.pc = 2153404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dbbc));
    }
}
#[inline(always)]
pub fn block_0x0020dbbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.orr_no_count(11usize, 11usize, 5usize, 2153408u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2153564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dc5c));
    } else {
        emu.pc = 2153412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dbc4));
    }
}
#[inline(always)]
pub fn block_0x0020dbc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(6usize, 0usize, 0u32, 2153416u32);
    emu.adi_no_count(11usize, 17usize, 4294966221u32, 2153420u32);
    emu.xri_no_count(17usize, 7usize, 1u32, 2153424u32);
    emu.adi_no_count(5usize, 0usize, 1u32, 2153428u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2153432u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2153568u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020dc60));
}
#[inline]
pub fn block_0x0020dbd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 31u32, 2153436u32);
    emu.orr_no_count(10usize, 6usize, 10usize, 2153440u32);
    emu.sli_no_count(14usize, 11usize, 1u32, 2153444u32);
    emu.sli_no_count(10usize, 10usize, 11u32, 2153448u32);
    emu.sri_no_count(10usize, 10usize, 11u32, 2153452u32);
    emu.sri_no_count(7usize, 6usize, 1u32, 2153456u32);
    let a = 0u32.wrapping_add(2146435072u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2153460u32;
    emu.update_insn_clock();
    emu.xrr_no_count(7usize, 7usize, 6usize, 2153464u32);
    emu.orr_no_count(7usize, 11usize, 7usize, 2153468u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a != b {
        emu.pc = 2153388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dbac));
    } else {
        emu.pc = 2153472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dc00));
    }
}
#[inline(always)]
pub fn block_0x0020dc00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 0usize, 3u32, 2153476u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2153480u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2153568u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020dc60));
}
#[inline(always)]
pub fn block_0x0020dc08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 0usize, 2u32, 2153484u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2153488u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2153568u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020dc60));
}
#[inline(always)]
pub fn block_0x0020dc10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(1048576u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2153492u32;
    emu.update_insn_clock();
    emu.xrr_no_count(11usize, 10usize, 11usize, 2153496u32);
    emu.orr_no_count(5usize, 14usize, 11usize, 2153500u32);
    emu.sltiu_no_count(11usize, 5usize, 1u32, 2153504u32);
    emu.sli_no_count(6usize, 14usize, 1u32, 2153508u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a != b {
        emu.pc = 2153520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dc30));
    } else {
        emu.pc = 2153512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dc28));
    }
}
#[inline(always)]
pub fn block_0x0020dc28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(4194304u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2153516u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    let return_addr = 2153520u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2153532u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020dc3c));
}
#[inline(always)]
pub fn block_0x0020dc30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(14usize, 14usize, 31u32, 2153524u32);
    emu.sli_no_count(10usize, 10usize, 1u32, 2153528u32);
    emu.orr_no_count(10usize, 10usize, 14usize, 2153532u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2153532u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020dc3c));
}
#[inline(always)]
pub fn block_0x0020dc3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 11usize, 4294967295u32, 2153536u32);
    emu.adi_no_count(5usize, 11usize, 1u32, 2153540u32);
    emu.sbr_no_count(11usize, 17usize, 11usize, 2153544u32);
    emu.anr_no_count(14usize, 14usize, 6usize, 2153548u32);
    emu.sltiu_no_count(6usize, 5usize, 1u32, 2153552u32);
    emu.adi_no_count(11usize, 11usize, 4294966220u32, 2153556u32);
    emu.xri_no_count(17usize, 7usize, 1u32, 2153560u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2153564u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2153568u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020dc60));
}
#[inline(always)]
pub fn block_0x0020dc5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 0usize, 4u32, 2153568u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2153568u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020dc60));
}
#[inline]
pub fn block_0x0020dc60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(7usize, 17usize, 255u32, 2153572u32);
    emu.sri_no_count(12usize, 12usize, 31u32, 2153576u32);
    emu.adi_no_count(23usize, 0usize, 1u32, 2153580u32);
    emu.sw_no_count(14usize, 2usize, 0u32, 2153584u32)?;
    emu.sw_no_count(10usize, 2usize, 4u32, 2153588u32)?;
    emu.sw_no_count(23usize, 2usize, 8u32, 2153592u32)?;
    emu.sw_no_count(0usize, 2usize, 12u32, 2153596u32)?;
    emu.sw_no_count(5usize, 2usize, 16u32, 2153600u32)?;
    emu.sw_no_count(6usize, 2usize, 20u32, 2153604u32)?;
    emu.sh_no_count(11usize, 2usize, 24u32, 2153608u32)?;
    emu.sb_no_count(17usize, 2usize, 26u32, 2153612u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a >= b {
        emu.pc = 2153648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dcb0));
    } else {
        emu.pc = 2153616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dc90));
    }
}
#[inline(always)]
pub fn block_0x0020dc90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2153620u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a != b {
        emu.pc = 2153672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dcc8));
    } else {
        emu.pc = 2153624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dc98));
    }
}
#[inline(always)]
pub fn block_0x0020dc98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(23usize, 0usize, 0u32, 2153628u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2153632u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 522u32, 2153636u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2153640u32);
    emu.adi_no_count(24usize, 0usize, 1u32, 2153644u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2153648u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2154048u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020de40));
}
#[inline(always)]
pub fn block_0x0020dcb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2153700u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dce4));
    } else {
        emu.pc = 2153652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dcb4));
    }
}
#[inline(always)]
pub fn block_0x0020dcb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(24usize, a);
    emu.pc = 2153656u32;
    emu.update_insn_clock();
    emu.adi_no_count(24usize, 24usize, 521u32, 2153660u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2153664u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2153716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dcf4));
    } else {
        emu.pc = 2153668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dcc4));
    }
}
#[inline(always)]
pub fn block_0x0020dcc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2153672u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2153724u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020dcfc));
}
#[inline(always)]
pub fn block_0x0020dcc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(23usize, 0usize, 1u32, 2153676u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2153936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ddd0));
    } else {
        emu.pc = 2153680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dcd0));
    }
}
#[inline(always)]
pub fn block_0x0020dcd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(24usize, a);
    emu.pc = 2153684u32;
    emu.update_insn_clock();
    emu.adi_no_count(24usize, 24usize, 521u32, 2153688u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2153692u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2153952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dde0));
    } else {
        emu.pc = 2153696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dce0));
    }
}
#[inline(always)]
pub fn block_0x0020dce0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2153700u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2153960u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020dde8));
}
#[inline(always)]
pub fn block_0x0020dce4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(24usize, a);
    emu.pc = 2153704u32;
    emu.update_insn_clock();
    emu.adi_no_count(24usize, 24usize, 520u32, 2153708u32);
    emu.adi_no_count(10usize, 24usize, 0u32, 2153712u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2153724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dcfc));
    } else {
        emu.pc = 2153716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dcf4));
    }
}
#[inline(always)]
pub fn block_0x0020dcf4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(23usize, 12usize, 0u32, 2153720u32);
    emu.adi_no_count(24usize, 10usize, 0u32, 2153724u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2153724u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020dcfc));
}
#[inline]
pub fn block_0x0020dcfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 11usize, 16u32, 2153728u32);
    emu.sai_no_count(10usize, 11usize, 1055u32, 2153732u32);
    emu.ani_no_count(10usize, 10usize, 4294967279u32, 2153736u32);
    emu.adi_no_count(10usize, 10usize, 5u32, 2153740u32);
    emu.sai_no_count(11usize, 11usize, 1040u32, 2153744u32);
    emu.mul_no_count(10usize, 10usize, 11usize, 2153748u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2153752u32);
    emu.adi_no_count(21usize, 10usize, 21u32, 2153756u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a < b {
        emu.pc = 2154156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020deac));
    } else {
        emu.pc = 2153760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dd20));
    }
}
#[inline(always)]
pub fn block_0x0020dd20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 18usize, 15u32, 2153764u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2153776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dd30));
    } else {
        emu.pc = 2153768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dd28));
    }
}
#[inline(always)]
pub fn block_0x0020dd28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(4294934528u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2153772u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    let return_addr = 2153776u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2153780u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020dd34));
}
#[inline(always)]
pub fn block_0x0020dd30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 0usize, 18usize, 2153780u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2153780u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020dd34));
}
#[inline]
pub fn block_0x0020dd34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 10usize, 16u32, 2153784u32);
    emu.sai_no_count(20usize, 10usize, 1040u32, 2153788u32);
    emu.adi_no_count(10usize, 2usize, 44u32, 2153792u32);
    emu.adi_no_count(11usize, 2usize, 0u32, 2153796u32);
    emu.adi_no_count(22usize, 15usize, 0u32, 2153800u32);
    emu.adi_no_count(12usize, 15usize, 0u32, 2153804u32);
    emu.adi_no_count(13usize, 21usize, 0u32, 2153808u32);
    emu.adi_no_count(14usize, 20usize, 0u32, 2153812u32);
    emu.apc_no_count(1usize, 2153812u32, 4096u32, 2153816u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2153820u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(344u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020dd5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 44u32, 2153824u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2153864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dd88));
    } else {
        emu.pc = 2153828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dd64));
    }
}
#[inline(always)]
pub fn block_0x0020dd64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 44u32, 2153832u32)?;
    emu.lw_no_count(11usize, 2usize, 48u32, 2153836u32)?;
    emu.lw_no_count(12usize, 2usize, 52u32, 2153840u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2153844u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2153848u32)?;
    emu.sw_no_count(12usize, 2usize, 40u32, 2153852u32)?;
    emu.lh_no_count(12usize, 2usize, 40u32, 2153856u32)?;
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(20usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2153900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ddac));
    } else {
        emu.pc = 2153860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dd84));
    }
}
#[inline(always)]
pub fn block_0x0020dd84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2153864u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2153968u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ddf0));
}
#[inline(always)]
pub fn block_0x0020dd88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 32u32, 2153868u32);
    emu.adi_no_count(11usize, 2usize, 0u32, 2153872u32);
    emu.adi_no_count(12usize, 22usize, 0u32, 2153876u32);
    emu.adi_no_count(13usize, 21usize, 0u32, 2153880u32);
    emu.adi_no_count(14usize, 20usize, 0u32, 2153884u32);
    emu.apc_no_count(1usize, 2153884u32, 20480u32, 2153888u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2153892u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967108u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020dda4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lh_no_count(12usize, 2usize, 40u32, 2153896u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(20usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2153968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ddf0));
    } else {
        emu.pc = 2153900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ddac));
    }
}
#[inline(always)]
pub fn block_0x0020ddac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 32u32, 2153904u32)?;
    emu.lw_no_count(11usize, 2usize, 36u32, 2153908u32)?;
    emu.adi_no_count(13usize, 18usize, 0u32, 2153912u32);
    emu.adi_no_count(14usize, 8usize, 0u32, 2153916u32);
    emu.adi_no_count(15usize, 19usize, 0u32, 2153920u32);
    emu.apc_no_count(1usize, 2153920u32, 0u32, 2153924u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2153928u32;
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
pub fn block_0x0020ddc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 10usize, 0u32, 2153932u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2153936u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2154064u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020de50));
}
#[inline(always)]
pub fn block_0x0020ddd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(24usize, a);
    emu.pc = 2153940u32;
    emu.update_insn_clock();
    emu.adi_no_count(24usize, 24usize, 520u32, 2153944u32);
    emu.adi_no_count(10usize, 24usize, 0u32, 2153948u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2153960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dde8));
    } else {
        emu.pc = 2153952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dde0));
    }
}
#[inline(always)]
pub fn block_0x0020dde0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(23usize, 12usize, 0u32, 2153956u32);
    emu.adi_no_count(24usize, 10usize, 0u32, 2153960u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2153960u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020dde8));
}
#[inline(always)]
pub fn block_0x0020dde8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2153964u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a != b {
        emu.pc = 2154032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020de30));
    } else {
        emu.pc = 2153968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ddf0));
    }
}
#[inline(always)]
pub fn block_0x0020ddf0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 2u32, 2153972u32);
    emu.sh_no_count(11usize, 8usize, 0u32, 2153976u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2154008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020de18));
    } else {
        emu.pc = 2153980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ddfc));
    }
}
#[inline(always)]
pub fn block_0x0020ddfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2153984u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 437u32, 2153988u32);
    emu.sw_no_count(10usize, 8usize, 4u32, 2153992u32)?;
    emu.sw_no_count(11usize, 8usize, 8u32, 2153996u32)?;
    emu.sh_no_count(0usize, 8usize, 12u32, 2154000u32)?;
    emu.sw_no_count(18usize, 8usize, 16u32, 2154004u32)?;
    emu.add_memory_rw_events(7usize);
    let return_addr = 2154008u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2154064u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020de50));
}
#[inline(always)]
pub fn block_0x0020de18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2154012u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 528u32, 2154016u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2154020u32);
    emu.sw_no_count(10usize, 8usize, 4u32, 2154024u32)?;
    emu.sw_no_count(11usize, 8usize, 8u32, 2154028u32)?;
    emu.add_memory_rw_events(6usize);
    let return_addr = 2154032u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2154064u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020de50));
}
#[inline(always)]
pub fn block_0x0020de30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2154036u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2154040u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 525u32, 2154044u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2154048u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2154048u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020de40));
}
#[inline(always)]
pub fn block_0x0020de40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sh_no_count(10usize, 8usize, 0u32, 2154052u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2154056u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2154060u32)?;
    emu.adi_no_count(11usize, 0usize, 1u32, 2154064u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2154064u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020de50));
}
#[inline]
pub fn block_0x0020de50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(24usize, 9usize, 0u32, 2154068u32)?;
    emu.sw_no_count(23usize, 9usize, 4u32, 2154072u32)?;
    emu.sw_no_count(8usize, 9usize, 8u32, 2154076u32)?;
    emu.sw_no_count(11usize, 9usize, 12u32, 2154080u32)?;
    emu.lw_no_count(1usize, 2usize, 92u32, 2154084u32)?;
    emu.lw_no_count(8usize, 2usize, 88u32, 2154088u32)?;
    emu.lw_no_count(9usize, 2usize, 84u32, 2154092u32)?;
    emu.lw_no_count(18usize, 2usize, 80u32, 2154096u32)?;
    emu.lw_no_count(19usize, 2usize, 76u32, 2154100u32)?;
    emu.lw_no_count(20usize, 2usize, 72u32, 2154104u32)?;
    emu.lw_no_count(21usize, 2usize, 68u32, 2154108u32)?;
    emu.lw_no_count(22usize, 2usize, 64u32, 2154112u32)?;
    emu.lw_no_count(23usize, 2usize, 60u32, 2154116u32)?;
    emu.lw_no_count(24usize, 2usize, 56u32, 2154120u32)?;
    emu.adi_no_count(2usize, 2usize, 96u32, 2154124u32);
    emu.add_memory_rw_events(16usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2154128u32;
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
pub fn block_0x0020de90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2154132u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 384u32, 2154136u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2154140u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 532u32, 2154144u32);
    emu.adi_no_count(11usize, 0usize, 34u32, 2154148u32);
    emu.apc_no_count(1usize, 2154148u32, 4294959104u32, 2154152u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2154156u32;
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
pub fn block_0x0020deac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2154160u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 548u32, 2154164u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2154168u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 588u32, 2154172u32);
    emu.adi_no_count(11usize, 0usize, 37u32, 2154176u32);
    emu.apc_no_count(1usize, 2154176u32, 4294959104u32, 2154180u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2154184u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(812u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020dec8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 11usize, 3u32, 2154188u32);
    emu.ani_no_count(13usize, 13usize, 4294967292u32, 2154192u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2154208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dee0));
    } else {
        emu.pc = 2154196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ded4));
    }
}
#[inline(always)]
pub fn block_0x0020ded4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 0u32, 2154200u32);
    emu.adi_no_count(14usize, 12usize, 4294967288u32, 2154204u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2154208u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2154268u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020df1c));
}
#[inline(always)]
pub fn block_0x0020dee0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(14usize, 13usize, 11usize, 2154212u32);
    emu.adi_no_count(13usize, 12usize, 0u32, 2154216u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2154224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020def0));
    } else {
        emu.pc = 2154220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020deec));
    }
}
#[inline(always)]
pub fn block_0x0020deec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 14usize, 0u32, 2154224u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2154224u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020def0));
}
#[inline(always)]
pub fn block_0x0020def0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2154260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020df14));
    } else {
        emu.pc = 2154228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020def4));
    }
}
#[inline(always)]
pub fn block_0x0020def4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 0u32, 2154232u32);
    emu.sbr_no_count(15usize, 0usize, 13usize, 2154236u32);
    emu.adi_no_count(16usize, 11usize, 0u32, 2154240u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2154240u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020df00));
}
#[inline(always)]
pub fn block_0x0020df00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(17usize, 16usize, 0u32, 2154244u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2154412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dfac));
    } else {
        emu.pc = 2154248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020df08));
    }
}
#[inline(always)]
pub fn block_0x0020df08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2154252u32);
    emu.adi_no_count(16usize, 16usize, 1u32, 2154256u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2154240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020df00));
    } else {
        emu.pc = 2154260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020df14));
    }
}
#[inline(always)]
pub fn block_0x0020df14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 12usize, 4294967288u32, 2154264u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2154356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020df74));
    } else {
        emu.pc = 2154268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020df1c));
    }
}
#[inline(always)]
pub fn block_0x0020df1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2154272u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 11usize, 4u32, 2154276u32);
    let a = 0u32.wrapping_add(2155905024u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2154280u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 256u32, 2154284u32);
    emu.adi_no_count(17usize, 16usize, 1u32, 2154288u32);
    emu.mul_no_count(17usize, 10usize, 17usize, 2154292u32);
    emu.adi_no_count(5usize, 5usize, 128u32, 2154296u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2154296u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020df38));
}
#[inline]
pub fn block_0x0020df38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(6usize, 11usize, 13usize, 2154300u32);
    emu.adr_no_count(7usize, 15usize, 13usize, 2154304u32);
    emu.lw_no_count(6usize, 6usize, 0u32, 2154308u32)?;
    emu.lw_no_count(7usize, 7usize, 0u32, 2154312u32)?;
    emu.xrr_no_count(6usize, 6usize, 17usize, 2154316u32);
    emu.xrr_no_count(7usize, 7usize, 17usize, 2154320u32);
    emu.sbr_no_count(28usize, 16usize, 6usize, 2154324u32);
    emu.orr_no_count(6usize, 28usize, 6usize, 2154328u32);
    emu.sbr_no_count(28usize, 16usize, 7usize, 2154332u32);
    emu.orr_no_count(7usize, 28usize, 7usize, 2154336u32);
    emu.anr_no_count(6usize, 6usize, 7usize, 2154340u32);
    emu.anr_no_count(6usize, 6usize, 5usize, 2154344u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a != b {
        emu.pc = 2154356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020df74));
    } else {
        emu.pc = 2154348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020df6c));
    }
}
#[inline(always)]
pub fn block_0x0020df6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 8u32, 2154352u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a >= b {
        emu.pc = 2154296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020df38));
    } else {
        emu.pc = 2154356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020df74));
    }
}
#[inline(always)]
pub fn block_0x0020df74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2154392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020df98));
    } else {
        emu.pc = 2154360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020df78));
    }
}
#[inline(always)]
pub fn block_0x0020df78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(14usize, 11usize, 13usize, 2154364u32);
    emu.sbr_no_count(11usize, 0usize, 13usize, 2154368u32);
    emu.sbr_no_count(12usize, 0usize, 12usize, 2154372u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2154372u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020df84));
}
#[inline(always)]
pub fn block_0x0020df84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(13usize, 14usize, 0u32, 2154376u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2154400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dfa0));
    } else {
        emu.pc = 2154380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020df8c));
    }
}
#[inline(always)]
pub fn block_0x0020df8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2154384u32);
    emu.adi_no_count(14usize, 14usize, 1u32, 2154388u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2154372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020df84));
    } else {
        emu.pc = 2154392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020df98));
    }
}
#[inline(always)]
pub fn block_0x0020df98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2154396u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2154400u32;
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
pub fn block_0x0020dfa0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 0usize, 11usize, 2154404u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2154408u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2154412u32;
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
pub fn block_0x0020dfac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 0usize, 14usize, 2154416u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2154420u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2154424u32;
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
