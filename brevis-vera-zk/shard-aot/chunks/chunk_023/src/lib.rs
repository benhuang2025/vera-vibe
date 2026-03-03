pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2174824u32;
pub const PC_MAX: u32 = 2176636u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 107usize] = [
        block_0x00212f68,
        block_0x00212fb8,
        block_0x00212fd0,
        block_0x00212fe0,
        block_0x00212ff4,
        block_0x00212ff8,
        block_0x00212ffc,
        block_0x00213000,
        block_0x00213008,
        block_0x0021300c,
        block_0x00213010,
        block_0x00213040,
        block_0x002130a8,
        block_0x00213104,
        block_0x0021316c,
        block_0x00213180,
        block_0x00213190,
        block_0x002131a0,
        block_0x002131a4,
        block_0x002131b4,
        block_0x002131cc,
        block_0x002131d0,
        block_0x002131e0,
        block_0x002131e8,
        block_0x002131fc,
        block_0x00213208,
        block_0x00213224,
        block_0x0021322c,
        block_0x00213234,
        block_0x00213238,
        block_0x00213248,
        block_0x00213260,
        block_0x00213278,
        block_0x0021327c,
        block_0x00213298,
        block_0x002132a4,
        block_0x002132a8,
        block_0x002132c0,
        block_0x002132d8,
        block_0x002132f0,
        block_0x002132f4,
        block_0x00213300,
        block_0x00213304,
        block_0x0021330c,
        block_0x00213310,
        block_0x00213324,
        block_0x00213354,
        block_0x00213358,
        block_0x00213360,
        block_0x00213374,
        block_0x00213384,
        block_0x00213388,
        block_0x00213390,
        block_0x002133ac,
        block_0x002133b0,
        block_0x002133c8,
        block_0x002133cc,
        block_0x002133e0,
        block_0x002133e8,
        block_0x002133f0,
        block_0x002133f4,
        block_0x002133f8,
        block_0x002133fc,
        block_0x00213410,
        block_0x00213438,
        block_0x0021343c,
        block_0x00213444,
        block_0x0021344c,
        block_0x00213460,
        block_0x00213464,
        block_0x0021346c,
        block_0x00213470,
        block_0x00213484,
        block_0x002134b0,
        block_0x002134b4,
        block_0x002134bc,
        block_0x002134c4,
        block_0x002134cc,
        block_0x002134d0,
        block_0x002134d8,
        block_0x002134f4,
        block_0x002134f8,
        block_0x00213510,
        block_0x00213524,
        block_0x00213528,
        block_0x00213530,
        block_0x00213540,
        block_0x0021354c,
        block_0x00213550,
        block_0x0021355c,
        block_0x00213564,
        block_0x00213574,
        block_0x00213590,
        block_0x00213598,
        block_0x002135ac,
        block_0x002135b4,
        block_0x002135c4,
        block_0x002135d8,
        block_0x002135ec,
        block_0x00213600,
        block_0x00213610,
        block_0x00213624,
        block_0x00213634,
        block_0x00213668,
        block_0x0021366c,
        block_0x00213678,
        block_0x0021367c,
    ];
    const IDX: [u16; 454usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        3u16, 0u16, 0u16, 0u16, 4u16, 0u16, 0u16, 0u16, 0u16, 5u16, 6u16, 7u16, 8u16,
        0u16, 9u16, 10u16, 11u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 12u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 13u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 14u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 15u16,
        0u16, 0u16, 0u16, 0u16, 16u16, 0u16, 0u16, 0u16, 17u16, 0u16, 0u16, 0u16, 18u16,
        19u16, 0u16, 0u16, 0u16, 20u16, 0u16, 0u16, 0u16, 0u16, 0u16, 21u16, 22u16, 0u16,
        0u16, 0u16, 23u16, 0u16, 24u16, 0u16, 0u16, 0u16, 0u16, 25u16, 0u16, 0u16, 26u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 27u16, 0u16, 28u16, 0u16, 29u16, 30u16, 0u16,
        0u16, 0u16, 31u16, 0u16, 0u16, 0u16, 0u16, 0u16, 32u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 33u16, 34u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 35u16, 0u16, 0u16, 36u16,
        37u16, 0u16, 0u16, 0u16, 0u16, 0u16, 38u16, 0u16, 0u16, 0u16, 0u16, 0u16, 39u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 40u16, 41u16, 0u16, 0u16, 42u16, 43u16, 0u16,
        44u16, 45u16, 0u16, 0u16, 0u16, 0u16, 46u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 47u16, 48u16, 0u16, 49u16, 0u16, 0u16, 0u16, 0u16,
        50u16, 0u16, 0u16, 0u16, 51u16, 52u16, 0u16, 53u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 54u16, 55u16, 0u16, 0u16, 0u16, 0u16, 0u16, 56u16, 57u16, 0u16, 0u16, 0u16,
        0u16, 58u16, 0u16, 59u16, 0u16, 60u16, 61u16, 62u16, 63u16, 0u16, 0u16, 0u16,
        0u16, 64u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 65u16, 66u16,
        0u16, 67u16, 0u16, 68u16, 0u16, 0u16, 0u16, 0u16, 69u16, 70u16, 0u16, 71u16,
        72u16, 0u16, 0u16, 0u16, 0u16, 73u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 74u16, 75u16, 0u16, 76u16, 0u16, 77u16, 0u16, 78u16, 79u16,
        0u16, 80u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 81u16, 82u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 83u16, 0u16, 0u16, 0u16, 0u16, 84u16, 85u16, 0u16, 86u16, 0u16, 0u16,
        0u16, 87u16, 0u16, 0u16, 88u16, 89u16, 0u16, 0u16, 90u16, 0u16, 91u16, 0u16,
        0u16, 0u16, 92u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 93u16, 0u16, 94u16, 0u16,
        0u16, 0u16, 0u16, 95u16, 0u16, 96u16, 0u16, 0u16, 0u16, 97u16, 0u16, 0u16, 0u16,
        0u16, 98u16, 0u16, 0u16, 0u16, 0u16, 99u16, 0u16, 0u16, 0u16, 0u16, 100u16, 0u16,
        0u16, 0u16, 101u16, 0u16, 0u16, 0u16, 0u16, 102u16, 0u16, 0u16, 0u16, 103u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 104u16,
        105u16, 0u16, 0u16, 106u16, 107u16,
    ];
    if pc < 2174824u32 || pc > 2176636u32 {
        return None;
    }
    let word_offset = ((pc - 2174824u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x00212f68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294966416u32, 2174828u32);
    emu.sw_no_count(1usize, 2usize, 876u32, 2174832u32)?;
    emu.sw_no_count(8usize, 2usize, 872u32, 2174836u32)?;
    emu.sw_no_count(9usize, 2usize, 868u32, 2174840u32)?;
    emu.sw_no_count(18usize, 2usize, 864u32, 2174844u32)?;
    emu.sw_no_count(19usize, 2usize, 860u32, 2174848u32)?;
    emu.sw_no_count(20usize, 2usize, 856u32, 2174852u32)?;
    emu.sw_no_count(21usize, 2usize, 852u32, 2174856u32)?;
    emu.sw_no_count(22usize, 2usize, 848u32, 2174860u32)?;
    emu.sw_no_count(23usize, 2usize, 844u32, 2174864u32)?;
    emu.sw_no_count(24usize, 2usize, 840u32, 2174868u32)?;
    emu.sw_no_count(25usize, 2usize, 836u32, 2174872u32)?;
    emu.sw_no_count(26usize, 2usize, 832u32, 2174876u32)?;
    emu.sw_no_count(27usize, 2usize, 828u32, 2174880u32)?;
    emu.adi_no_count(19usize, 14usize, 0u32, 2174884u32);
    emu.adi_no_count(18usize, 13usize, 0u32, 2174888u32);
    emu.lw_no_count(5usize, 11usize, 0u32, 2174892u32)?;
    emu.lw_no_count(13usize, 11usize, 4u32, 2174896u32)?;
    emu.orr_no_count(14usize, 5usize, 13usize, 2174900u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2177796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2177796u32));
    } else {
        emu.pc = 2174904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212fb8));
    }
}
#[inline(always)]
pub fn block_0x00212fb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2174908u32);
    emu.sw_no_count(12usize, 2usize, 4u32, 2174912u32)?;
    emu.lw_no_count(10usize, 11usize, 8u32, 2174916u32)?;
    emu.lw_no_count(14usize, 11usize, 12u32, 2174920u32)?;
    emu.orr_no_count(15usize, 10usize, 14usize, 2174924u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2177824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2177824u32));
    } else {
        emu.pc = 2174928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212fd0));
    }
}
#[inline(always)]
pub fn block_0x00212fd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 11usize, 16u32, 2174932u32)?;
    emu.lw_no_count(16usize, 11usize, 20u32, 2174936u32)?;
    emu.orr_no_count(17usize, 15usize, 16usize, 2174940u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2177852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2177852u32));
    } else {
        emu.pc = 2174944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212fe0));
    }
}
#[inline(always)]
pub fn block_0x00212fe0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 5usize, 15usize, 2174948u32);
    emu.sltru_no_count(15usize, 15usize, 5usize, 2174952u32);
    emu.adr_no_count(16usize, 13usize, 16usize, 2174956u32);
    emu.adr_no_count(16usize, 16usize, 15usize, 2174960u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2174968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212ff8));
    } else {
        emu.pc = 2174964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212ff4));
    }
}
#[inline(always)]
pub fn block_0x00212ff4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 16usize, 13usize, 2174968u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2174968u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212ff8));
}
#[inline(always)]
pub fn block_0x00212ff8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2177880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2177880u32));
    } else {
        emu.pc = 2174972u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212ffc));
    }
}
#[inline(always)]
pub fn block_0x00212ffc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2174984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213008));
    } else {
        emu.pc = 2174976u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213000));
    }
}
#[inline(always)]
pub fn block_0x00213000(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 13usize, 14usize, 2174980u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2174984u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2174988u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021300c));
}
#[inline(always)]
pub fn block_0x00213008(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 5usize, 10usize, 2174988u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2174988u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021300c));
}
#[inline(always)]
pub fn block_0x0021300c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2177908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2177908u32));
    } else {
        emu.pc = 2174992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213010));
    }
}
#[inline]
pub fn block_0x00213010(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lh_no_count(20usize, 11usize, 24u32, 2174996u32)?;
    emu.sltiu_no_count(10usize, 5usize, 1u32, 2175000u32);
    let a = 0u32.wrapping_add(1431654400u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2175004u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(858992640u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2175008u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(252645376u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2175012u32;
    emu.update_insn_clock();
    emu.sbr_no_count(16usize, 13usize, 10usize, 2175016u32);
    emu.adi_no_count(15usize, 11usize, 1365u32, 2175020u32);
    emu.adi_no_count(14usize, 14usize, 819u32, 2175024u32);
    emu.adi_no_count(10usize, 17usize, 4294967055u32, 2175028u32);
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2175032u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 257u32, 2175036u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a != b {
        emu.pc = 2175144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002130a8));
    } else {
        emu.pc = 2175040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213040));
    }
}
#[inline(never)]
pub fn block_0x00213040(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 26u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 5usize, 4294967295u32, 2175044u32);
    emu.sri_no_count(17usize, 16usize, 1u32, 2175048u32);
    emu.orr_no_count(16usize, 16usize, 17usize, 2175052u32);
    emu.sri_no_count(17usize, 16usize, 2u32, 2175056u32);
    emu.orr_no_count(16usize, 16usize, 17usize, 2175060u32);
    emu.sri_no_count(17usize, 16usize, 4u32, 2175064u32);
    emu.orr_no_count(16usize, 16usize, 17usize, 2175068u32);
    emu.sri_no_count(17usize, 16usize, 8u32, 2175072u32);
    emu.orr_no_count(16usize, 16usize, 17usize, 2175076u32);
    emu.sri_no_count(17usize, 16usize, 16u32, 2175080u32);
    emu.orr_no_count(16usize, 16usize, 17usize, 2175084u32);
    emu.xri_no_count(16usize, 16usize, 4294967295u32, 2175088u32);
    emu.sri_no_count(17usize, 16usize, 1u32, 2175092u32);
    emu.anr_no_count(15usize, 17usize, 15usize, 2175096u32);
    emu.sbr_no_count(15usize, 16usize, 15usize, 2175100u32);
    emu.anr_no_count(16usize, 15usize, 14usize, 2175104u32);
    emu.sri_no_count(15usize, 15usize, 2u32, 2175108u32);
    emu.anr_no_count(14usize, 15usize, 14usize, 2175112u32);
    emu.adr_no_count(14usize, 16usize, 14usize, 2175116u32);
    emu.sri_no_count(15usize, 14usize, 4u32, 2175120u32);
    emu.adr_no_count(14usize, 14usize, 15usize, 2175124u32);
    emu.anr_no_count(10usize, 14usize, 10usize, 2175128u32);
    emu.mul_no_count(10usize, 10usize, 11usize, 2175132u32);
    emu.sri_no_count(10usize, 10usize, 24u32, 2175136u32);
    emu.adi_no_count(10usize, 10usize, 32u32, 2175140u32);
    emu.add_memory_rw_events(26usize);
    let return_addr = 2175144u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2175236u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213104));
}
#[inline]
pub fn block_0x002130a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 23u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(17usize, 16usize, 1u32, 2175148u32);
    emu.orr_no_count(16usize, 16usize, 17usize, 2175152u32);
    emu.sri_no_count(17usize, 16usize, 2u32, 2175156u32);
    emu.orr_no_count(16usize, 16usize, 17usize, 2175160u32);
    emu.sri_no_count(17usize, 16usize, 4u32, 2175164u32);
    emu.orr_no_count(16usize, 16usize, 17usize, 2175168u32);
    emu.sri_no_count(17usize, 16usize, 8u32, 2175172u32);
    emu.orr_no_count(16usize, 16usize, 17usize, 2175176u32);
    emu.sri_no_count(17usize, 16usize, 16u32, 2175180u32);
    emu.orr_no_count(16usize, 16usize, 17usize, 2175184u32);
    emu.xri_no_count(16usize, 16usize, 4294967295u32, 2175188u32);
    emu.sri_no_count(17usize, 16usize, 1u32, 2175192u32);
    emu.anr_no_count(15usize, 17usize, 15usize, 2175196u32);
    emu.sbr_no_count(15usize, 16usize, 15usize, 2175200u32);
    emu.anr_no_count(16usize, 15usize, 14usize, 2175204u32);
    emu.sri_no_count(15usize, 15usize, 2u32, 2175208u32);
    emu.anr_no_count(14usize, 15usize, 14usize, 2175212u32);
    emu.adr_no_count(14usize, 16usize, 14usize, 2175216u32);
    emu.sri_no_count(15usize, 14usize, 4u32, 2175220u32);
    emu.adr_no_count(14usize, 14usize, 15usize, 2175224u32);
    emu.anr_no_count(10usize, 14usize, 10usize, 2175228u32);
    emu.mul_no_count(10usize, 10usize, 11usize, 2175232u32);
    emu.sri_no_count(10usize, 10usize, 24u32, 2175236u32);
    emu.add_memory_rw_events(23usize);
    emu.pc = 2175236u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213104));
}
#[inline(never)]
pub fn block_0x00213104(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 26u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 20usize, 10usize, 2175240u32);
    let a = 0u32.wrapping_add(1292914688u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2175244u32;
    emu.update_insn_clock();
    emu.sltiu_no_count(14usize, 13usize, 1u32, 2175248u32);
    emu.adi_no_count(11usize, 11usize, 4294966594u32, 2175252u32);
    emu.mulh_no_count(15usize, 10usize, 11usize, 2175256u32);
    emu.mul_no_count(10usize, 10usize, 11usize, 2175260u32);
    emu.adi_no_count(11usize, 0usize, 2u32, 2175264u32);
    emu.sbr_no_count(11usize, 11usize, 14usize, 2175268u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2175272u32);
    emu.anr_no_count(13usize, 14usize, 13usize, 2175276u32);
    let a = 0u32.wrapping_add(1142116352u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2175280u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 128u32, 2175284u32);
    emu.sw_no_count(11usize, 2usize, 168u32, 2175288u32)?;
    emu.adr_no_count(14usize, 10usize, 14usize, 2175292u32);
    emu.sw_no_count(5usize, 2usize, 8u32, 2175296u32)?;
    emu.sw_no_count(13usize, 2usize, 12u32, 2175300u32)?;
    emu.sltru_no_count(10usize, 14usize, 10usize, 2175304u32);
    emu.adr_no_count(10usize, 15usize, 10usize, 2175308u32);
    emu.adi_no_count(21usize, 10usize, 19u32, 2175312u32);
    emu.sli_no_count(10usize, 21usize, 16u32, 2175316u32);
    emu.sai_no_count(22usize, 10usize, 1040u32, 2175320u32);
    emu.adi_no_count(10usize, 2usize, 16u32, 2175324u32);
    emu.adi_no_count(12usize, 0usize, 152u32, 2175328u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2175332u32);
    emu.apc_no_count(1usize, 2175332u32, 4294901760u32, 2175336u32);
    emu.add_memory_rw_events(26usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2175340u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966808u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021316c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 176u32, 2175344u32);
    emu.adi_no_count(12usize, 0usize, 156u32, 2175348u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2175352u32);
    emu.apc_no_count(1usize, 2175352u32, 4294901760u32, 2175356u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2175360u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966788u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00213180(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2175364u32);
    emu.sw_no_count(10usize, 2usize, 332u32, 2175368u32)?;
    emu.sw_no_count(10usize, 2usize, 172u32, 2175372u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(20usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2175412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002131b4));
    } else {
        emu.pc = 2175376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213190));
    }
}
#[inline(always)]
pub fn block_0x00213190(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 8u32, 2175380u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2175384u32);
    emu.apc_no_count(1usize, 2175384u32, 4294942720u32, 2175388u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2175392u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965248u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002131a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(22usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2175440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002131d0));
    } else {
        emu.pc = 2175396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002131a4));
    }
}
#[inline(always)]
pub fn block_0x002131a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 21usize, 17u32, 2175400u32);
    emu.sri_no_count(11usize, 11usize, 17u32, 2175404u32);
    emu.adi_no_count(10usize, 2usize, 172u32, 2175408u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2175412u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2175456u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002131e0));
}
#[inline(always)]
pub fn block_0x002131b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 0usize, 20usize, 2175416u32);
    emu.sli_no_count(10usize, 10usize, 16u32, 2175420u32);
    emu.sai_no_count(11usize, 10usize, 1040u32, 2175424u32);
    emu.adi_no_count(10usize, 2usize, 172u32, 2175428u32);
    emu.apc_no_count(1usize, 2175428u32, 4294938624u32, 2175432u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2175436u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(2004u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002131cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(22usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2175396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002131a4));
    } else {
        emu.pc = 2175440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002131d0));
    }
}
#[inline(always)]
pub fn block_0x002131d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 0usize, 22usize, 2175444u32);
    emu.sli_no_count(10usize, 10usize, 16u32, 2175448u32);
    emu.sri_no_count(11usize, 10usize, 16u32, 2175452u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2175456u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2175456u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002131e0));
}
#[inline(always)]
pub fn block_0x002131e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2175456u32, 4294963200u32, 2175460u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2175464u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965848u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002131e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 664u32, 2175468u32);
    emu.adi_no_count(11usize, 2usize, 172u32, 2175472u32);
    emu.adi_no_count(12usize, 0usize, 164u32, 2175476u32);
    emu.apc_no_count(1usize, 2175476u32, 4294901760u32, 2175480u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2175484u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966912u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002131fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 10u32, 2175488u32);
    emu.adi_no_count(24usize, 18usize, 0u32, 2175492u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a < b {
        emu.pc = 2175612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021327c));
    } else {
        emu.pc = 2175496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213208));
    }
}
#[inline(always)]
pub fn block_0x00213208(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 2usize, 660u32, 2175500u32);
    emu.adi_no_count(25usize, 0usize, 41u32, 2175504u32);
    emu.adi_no_count(26usize, 0usize, 9u32, 2175508u32);
    let a = 0u32.wrapping_add(1000001536u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2175512u32;
    emu.update_insn_clock();
    emu.adi_no_count(20usize, 10usize, 4294965760u32, 2175516u32);
    emu.adi_no_count(24usize, 18usize, 0u32, 2175520u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2175524u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2175532u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021322c));
}
#[inline(always)]
pub fn block_0x00213224(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(24usize, 24usize, 4294967287u32, 2175528u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a >= b {
        emu.pc = 2175612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021327c));
    } else {
        emu.pc = 2175532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021322c));
    }
}
#[inline(always)]
pub fn block_0x0021322c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 824u32, 2175536u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(25usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2177704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2177704u32));
    } else {
        emu.pc = 2175540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213234));
    }
}
#[inline(always)]
pub fn block_0x00213234(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2175524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213224));
    } else {
        emu.pc = 2175544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213238));
    }
}
#[inline(always)]
pub fn block_0x00213238(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2175548u32);
    emu.sli_no_count(10usize, 10usize, 2u32, 2175552u32);
    emu.sbr_no_count(27usize, 0usize, 10usize, 2175556u32);
    emu.adr_no_count(23usize, 8usize, 10usize, 2175560u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2175560u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213248));
}
#[inline(always)]
pub fn block_0x00213248(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(21usize, 23usize, 0u32, 2175564u32)?;
    emu.adi_no_count(10usize, 21usize, 0u32, 2175568u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2175572u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2175576u32);
    emu.apc_no_count(1usize, 2175576u32, 4096u32, 2175580u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2175584u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966620u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00213260(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.mul_no_count(11usize, 10usize, 20usize, 2175588u32);
    emu.sw_no_count(10usize, 23usize, 0u32, 2175592u32)?;
    emu.adi_no_count(27usize, 27usize, 4u32, 2175596u32);
    emu.sbr_no_count(11usize, 21usize, 11usize, 2175600u32);
    emu.adi_no_count(23usize, 23usize, 4294967292u32, 2175604u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(27usize);
    if a != b {
        emu.pc = 2175560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213248));
    } else {
        emu.pc = 2175608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213278));
    }
}
#[inline(always)]
pub fn block_0x00213278(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2175612u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2175524u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213224));
}
#[inline(always)]
pub fn block_0x0021327c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(24usize, 24usize, 2u32, 2175616u32);
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2175620u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965284u32, 2175624u32);
    emu.adr_no_count(10usize, 10usize, 24usize, 2175628u32);
    emu.lw_no_count(20usize, 10usize, 0u32, 2175632u32)?;
    emu.sli_no_count(20usize, 20usize, 1u32, 2175636u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2177936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2177936u32));
    } else {
        emu.pc = 2175640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213298));
    }
}
#[inline(always)]
pub fn block_0x00213298(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 824u32, 2175644u32)?;
    emu.adi_no_count(11usize, 0usize, 41u32, 2175648u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2177704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2177704u32));
    } else {
        emu.pc = 2175652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002132a4));
    }
}
#[inline(always)]
pub fn block_0x002132a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2175732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002132f4));
    } else {
        emu.pc = 2175656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002132a8));
    }
}
#[inline(always)]
pub fn block_0x002132a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2175660u32);
    emu.sli_no_count(10usize, 10usize, 2u32, 2175664u32);
    emu.adi_no_count(12usize, 2usize, 664u32, 2175668u32);
    emu.adr_no_count(12usize, 10usize, 12usize, 2175672u32);
    emu.sbr_no_count(8usize, 0usize, 10usize, 2175676u32);
    emu.adi_no_count(23usize, 12usize, 4294967292u32, 2175680u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2175680u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002132c0));
}
#[inline(always)]
pub fn block_0x002132c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(21usize, 23usize, 0u32, 2175684u32)?;
    emu.adi_no_count(10usize, 21usize, 0u32, 2175688u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2175692u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2175696u32);
    emu.apc_no_count(1usize, 2175696u32, 4096u32, 2175700u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2175704u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966500u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002132d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.mul_no_count(11usize, 10usize, 20usize, 2175708u32);
    emu.sw_no_count(10usize, 23usize, 0u32, 2175712u32)?;
    emu.adi_no_count(8usize, 8usize, 4u32, 2175716u32);
    emu.sbr_no_count(11usize, 21usize, 11usize, 2175720u32);
    emu.adi_no_count(23usize, 23usize, 4294967292u32, 2175724u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a != b {
        emu.pc = 2175680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002132c0));
    } else {
        emu.pc = 2175728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002132f0));
    }
}
#[inline(always)]
pub fn block_0x002132f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 824u32, 2175732u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2175732u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002132f4));
}
#[inline(always)]
pub fn block_0x002132f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 2usize, 168u32, 2175736u32)?;
    emu.adi_no_count(13usize, 15usize, 0u32, 2175740u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2175748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213304));
    } else {
        emu.pc = 2175744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213300));
    }
}
#[inline(always)]
pub fn block_0x00213300(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 10usize, 0u32, 2175748u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2175748u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213304));
}
#[inline(always)]
pub fn block_0x00213304(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 41u32, 2175752u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2177724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2177724u32));
    } else {
        emu.pc = 2175756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021330c));
    }
}
#[inline(always)]
pub fn block_0x0021330c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2175860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213374));
    } else {
        emu.pc = 2175760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213310));
    }
}
#[inline(always)]
pub fn block_0x00213310(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 0u32, 2175764u32);
    emu.sli_no_count(10usize, 13usize, 2u32, 2175768u32);
    emu.adi_no_count(11usize, 2usize, 8u32, 2175772u32);
    emu.adr_no_count(12usize, 11usize, 10usize, 2175776u32);
    emu.adi_no_count(14usize, 2usize, 664u32, 2175780u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2175780u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213324));
}
#[inline]
pub fn block_0x00213324(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(17usize, 14usize, 0u32, 2175784u32)?;
    emu.lw_no_count(5usize, 11usize, 0u32, 2175788u32)?;
    emu.ani_no_count(16usize, 16usize, 1u32, 2175792u32);
    emu.adi_no_count(11usize, 11usize, 4u32, 2175796u32);
    emu.adr_no_count(5usize, 17usize, 5usize, 2175800u32);
    emu.sltru_no_count(17usize, 5usize, 17usize, 2175804u32);
    emu.adr_no_count(16usize, 5usize, 16usize, 2175808u32);
    emu.sltru_no_count(5usize, 16usize, 5usize, 2175812u32);
    emu.sw_no_count(16usize, 14usize, 0u32, 2175816u32)?;
    emu.orr_no_count(16usize, 17usize, 5usize, 2175820u32);
    emu.adi_no_count(14usize, 14usize, 4u32, 2175824u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2175780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213324));
    } else {
        emu.pc = 2175828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213354));
    }
}
#[inline(always)]
pub fn block_0x00213354(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2175860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213374));
    } else {
        emu.pc = 2175832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213358));
    }
}
#[inline(always)]
pub fn block_0x00213358(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 40u32, 2175836u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2178084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2178084u32));
    } else {
        emu.pc = 2175840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213360));
    }
}
#[inline(always)]
pub fn block_0x00213360(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 2usize, 664u32, 2175844u32);
    emu.adr_no_count(10usize, 11usize, 10usize, 2175848u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2175852u32);
    emu.sw_no_count(11usize, 10usize, 0u32, 2175856u32)?;
    emu.adi_no_count(13usize, 13usize, 1u32, 2175860u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2175860u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213374));
}
#[inline(always)]
pub fn block_0x00213374(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 332u32, 2175864u32)?;
    emu.sw_no_count(13usize, 2usize, 824u32, 2175868u32)?;
    emu.adi_no_count(14usize, 10usize, 0u32, 2175872u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2175880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213388));
    } else {
        emu.pc = 2175876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213384));
    }
}
#[inline(always)]
pub fn block_0x00213384(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 13usize, 0u32, 2175880u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2175880u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213388));
}
#[inline(always)]
pub fn block_0x00213388(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 41u32, 2175884u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a >= b {
        emu.pc = 2177748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2177748u32));
    } else {
        emu.pc = 2175888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213390));
    }
}
#[inline(always)]
pub fn block_0x00213390(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(14usize, 14usize, 2u32, 2175892u32);
    emu.adi_no_count(12usize, 2usize, 172u32, 2175896u32);
    emu.adi_no_count(13usize, 2usize, 664u32, 2175900u32);
    emu.sbr_no_count(11usize, 0usize, 14usize, 2175904u32);
    emu.adi_no_count(14usize, 14usize, 4294967292u32, 2175908u32);
    emu.adr_no_count(12usize, 12usize, 14usize, 2175912u32);
    emu.adr_no_count(13usize, 13usize, 14usize, 2175916u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2175916u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002133ac));
}
#[inline(always)]
pub fn block_0x002133ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2175988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002133f4));
    } else {
        emu.pc = 2175920u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002133b0));
    }
}
#[inline(always)]
pub fn block_0x002133b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 13usize, 0u32, 2175924u32)?;
    emu.lw_no_count(16usize, 12usize, 0u32, 2175928u32)?;
    emu.adi_no_count(11usize, 11usize, 4u32, 2175932u32);
    emu.adi_no_count(12usize, 12usize, 4294967292u32, 2175936u32);
    emu.adi_no_count(13usize, 13usize, 4294967292u32, 2175940u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2175916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002133ac));
    } else {
        emu.pc = 2175944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002133c8));
    }
}
#[inline(always)]
pub fn block_0x002133c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2175992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002133f8));
    } else {
        emu.pc = 2175948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002133cc));
    }
}
#[inline(always)]
pub fn block_0x002133cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(22usize, 22usize, 1u32, 2175952u32);
    emu.sli_no_count(11usize, 22usize, 16u32, 2175956u32);
    emu.sai_no_count(21usize, 11usize, 1040u32, 2175960u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2175964u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(21usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2176096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213460));
    } else {
        emu.pc = 2175968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002133e0));
    }
}
#[inline(always)]
pub fn block_0x002133e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(12usize, 21usize, 19usize, 2175972u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2176436u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002135b4));
    } else {
        emu.pc = 2175976u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002133e8));
    }
}
#[inline(always)]
pub fn block_0x002133e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 18usize, 0u32, 2175980u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2176452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002135c4));
    } else {
        emu.pc = 2175984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002133f0));
    }
}
#[inline(always)]
pub fn block_0x002133f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2175988u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2176100u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213464));
}
#[inline(always)]
pub fn block_0x002133f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2175948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002133cc));
    } else {
        emu.pc = 2175992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002133f8));
    }
}
#[inline(always)]
pub fn block_0x002133f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2176076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021344c));
    } else {
        emu.pc = 2175996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002133fc));
    }
}
#[inline(always)]
pub fn block_0x002133fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2176000u32);
    emu.sli_no_count(13usize, 15usize, 2u32, 2176004u32);
    emu.adi_no_count(16usize, 2usize, 8u32, 2176008u32);
    emu.adr_no_count(11usize, 16usize, 13usize, 2176012u32);
    emu.adi_no_count(14usize, 0usize, 10u32, 2176016u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2176016u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213410));
}
#[inline]
pub fn block_0x00213410(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(17usize, 16usize, 0u32, 2176020u32)?;
    emu.mulhu_no_count(5usize, 17usize, 14usize, 2176024u32);
    emu.mul_no_count(17usize, 17usize, 14usize, 2176028u32);
    emu.adr_no_count(12usize, 17usize, 12usize, 2176032u32);
    emu.sw_no_count(12usize, 16usize, 0u32, 2176036u32)?;
    emu.adi_no_count(16usize, 16usize, 4u32, 2176040u32);
    emu.sltru_no_count(12usize, 12usize, 17usize, 2176044u32);
    emu.adi_no_count(13usize, 13usize, 4294967292u32, 2176048u32);
    emu.adr_no_count(12usize, 5usize, 12usize, 2176052u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2176016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213410));
    } else {
        emu.pc = 2176056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213438));
    }
}
#[inline(always)]
pub fn block_0x00213438(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2176076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021344c));
    } else {
        emu.pc = 2176060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021343c));
    }
}
#[inline(always)]
pub fn block_0x0021343c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 40u32, 2176064u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2178084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2178084u32));
    } else {
        emu.pc = 2176068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213444));
    }
}
#[inline(always)]
pub fn block_0x00213444(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(12usize, 11usize, 0u32, 2176072u32)?;
    emu.adi_no_count(15usize, 15usize, 1u32, 2176076u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2176076u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021344c));
}
#[inline(always)]
pub fn block_0x0021344c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(15usize, 2usize, 168u32, 2176080u32)?;
    emu.sli_no_count(11usize, 22usize, 16u32, 2176084u32);
    emu.sai_no_count(21usize, 11usize, 1040u32, 2176088u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2176092u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(21usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2175968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002133e0));
    } else {
        emu.pc = 2176096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213460));
    }
}
#[inline(always)]
pub fn block_0x00213460(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 0u32, 2176100u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2176100u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213464));
}
#[inline(always)]
pub fn block_0x00213464(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 41u32, 2176104u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2177704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2177704u32));
    } else {
        emu.pc = 2176108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021346c));
    }
}
#[inline(always)]
pub fn block_0x0021346c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2176196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002134c4));
    } else {
        emu.pc = 2176112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213470));
    }
}
#[inline(always)]
pub fn block_0x00213470(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 0u32, 2176116u32);
    emu.sli_no_count(14usize, 10usize, 2u32, 2176120u32);
    emu.adi_no_count(17usize, 2usize, 172u32, 2176124u32);
    emu.adr_no_count(12usize, 17usize, 14usize, 2176128u32);
    emu.adi_no_count(16usize, 0usize, 5u32, 2176132u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2176132u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213484));
}
#[inline]
pub fn block_0x00213484(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(5usize, 17usize, 0u32, 2176136u32)?;
    emu.mulhu_no_count(6usize, 5usize, 16usize, 2176140u32);
    emu.sli_no_count(7usize, 5usize, 2u32, 2176144u32);
    emu.adr_no_count(5usize, 7usize, 5usize, 2176148u32);
    emu.adr_no_count(13usize, 5usize, 13usize, 2176152u32);
    emu.sw_no_count(13usize, 17usize, 0u32, 2176156u32)?;
    emu.adi_no_count(17usize, 17usize, 4u32, 2176160u32);
    emu.sltru_no_count(13usize, 13usize, 5usize, 2176164u32);
    emu.adi_no_count(14usize, 14usize, 4294967292u32, 2176168u32);
    emu.adr_no_count(13usize, 6usize, 13usize, 2176172u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a != b {
        emu.pc = 2176132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213484));
    } else {
        emu.pc = 2176176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002134b0));
    }
}
#[inline(always)]
pub fn block_0x002134b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2176196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002134c4));
    } else {
        emu.pc = 2176180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002134b4));
    }
}
#[inline(always)]
pub fn block_0x002134b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 40u32, 2176184u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2178084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2178084u32));
    } else {
        emu.pc = 2176188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002134bc));
    }
}
#[inline(always)]
pub fn block_0x002134bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(13usize, 12usize, 0u32, 2176192u32)?;
    emu.adi_no_count(10usize, 10usize, 1u32, 2176196u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2176196u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002134c4));
}
#[inline(always)]
pub fn block_0x002134c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 332u32, 2176200u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2176208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002134d0));
    } else {
        emu.pc = 2176204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002134cc));
    }
}
#[inline(always)]
pub fn block_0x002134cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 15usize, 0u32, 2176208u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2176208u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002134d0));
}
#[inline(always)]
pub fn block_0x002134d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 41u32, 2176212u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2177704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2177704u32));
    } else {
        emu.pc = 2176216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002134d8));
    }
}
#[inline(always)]
pub fn block_0x002134d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 10usize, 2u32, 2176220u32);
    emu.adi_no_count(13usize, 2usize, 172u32, 2176224u32);
    emu.adi_no_count(14usize, 2usize, 8u32, 2176228u32);
    emu.sbr_no_count(10usize, 0usize, 12usize, 2176232u32);
    emu.adi_no_count(15usize, 12usize, 4294967292u32, 2176236u32);
    emu.adr_no_count(12usize, 13usize, 15usize, 2176240u32);
    emu.adr_no_count(13usize, 14usize, 15usize, 2176244u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2176244u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002134f4));
}
#[inline(always)]
pub fn block_0x002134f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2176304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213530));
    } else {
        emu.pc = 2176248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002134f8));
    }
}
#[inline(always)]
pub fn block_0x002134f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 13usize, 0u32, 2176252u32)?;
    emu.lw_no_count(15usize, 12usize, 0u32, 2176256u32)?;
    emu.adi_no_count(10usize, 10usize, 4u32, 2176260u32);
    emu.adi_no_count(12usize, 12usize, 4294967292u32, 2176264u32);
    emu.adi_no_count(13usize, 13usize, 4294967292u32, 2176268u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2176244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002134f4));
    } else {
        emu.pc = 2176272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213510));
    }
}
#[inline(always)]
pub fn block_0x00213510(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 14usize, 15usize, 2176276u32);
    emu.sltru_no_count(12usize, 15usize, 14usize, 2176280u32);
    emu.sbr_no_count(10usize, 12usize, 10usize, 2176284u32);
    emu.lw_no_count(14usize, 2usize, 4u32, 2176288u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2176320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213540));
    } else {
        emu.pc = 2176292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213524));
    }
}
#[inline(always)]
pub fn block_0x00213524(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2177560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2177560u32));
    } else {
        emu.pc = 2176296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213528));
    }
}
#[inline(always)]
pub fn block_0x00213528(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 0u32, 2176300u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2176304u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2177632u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(2177632u32));
}
#[inline(always)]
pub fn block_0x00213530(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(10usize, 10usize, 1u32, 2176308u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2176312u32);
    emu.lw_no_count(14usize, 2usize, 4u32, 2176316u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2176292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213524));
    } else {
        emu.pc = 2176320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213540));
    }
}
#[inline(always)]
pub fn block_0x00213540(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 255u32, 2176324u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2176328u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2177628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2177628u32));
    } else {
        emu.pc = 2176332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021354c));
    }
}
#[inline(always)]
pub fn block_0x0021354c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2178040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2178040u32));
    } else {
        emu.pc = 2176336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213550));
    }
}
#[inline(always)]
pub fn block_0x00213550(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2176340u32);
    emu.adr_no_count(23usize, 14usize, 20usize, 2176344u32);
    emu.adi_no_count(10usize, 0usize, 57u32, 2176348u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2176348u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021355c));
}
#[inline(always)]
pub fn block_0x0021355c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(12usize, 20usize, 11usize, 2176352u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2177504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2177504u32));
    } else {
        emu.pc = 2176356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213564));
    }
}
#[inline(always)]
pub fn block_0x00213564(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(12usize, 23usize, 11usize, 2176360u32);
    emu.lbu_no_count(12usize, 12usize, 4294967295u32, 2176364u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2176368u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2176348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021355c));
    } else {
        emu.pc = 2176372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213574));
    }
}
#[inline(always)]
pub fn block_0x00213574(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(23usize, 23usize, 11usize, 2176376u32);
    emu.lbu_no_count(10usize, 23usize, 0u32, 2176380u32);
    emu.adr_no_count(12usize, 20usize, 11usize, 2176384u32);
    emu.adi_no_count(13usize, 10usize, 1u32, 2176388u32);
    emu.adi_no_count(10usize, 12usize, 1u32, 2176392u32);
    emu.sb_no_count(13usize, 23usize, 0u32, 2176396u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a < b {
        emu.pc = 2178064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2178064u32));
    } else {
        emu.pc = 2176400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213590));
    }
}
#[inline(always)]
pub fn block_0x00213590(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4294967295u32, 2176404u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2177628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2177628u32));
    } else {
        emu.pc = 2176408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213598));
    }
}
#[inline(always)]
pub fn block_0x00213598(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(12usize, 11usize, 4294967295u32, 2176412u32);
    emu.adi_no_count(10usize, 23usize, 1u32, 2176416u32);
    emu.adi_no_count(11usize, 0usize, 48u32, 2176420u32);
    emu.apc_no_count(1usize, 2176420u32, 4294901760u32, 2176424u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2176428u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x002135ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 2usize, 4u32, 2176432u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2176436u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2177628u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(2177628u32));
}
#[inline(always)]
pub fn block_0x002135b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(12usize, 22usize, 19usize, 2176440u32);
    emu.sli_no_count(12usize, 12usize, 16u32, 2176444u32);
    emu.sai_no_count(20usize, 12usize, 1040u32, 2176448u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2176100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213464));
    } else {
        emu.pc = 2176452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002135c4));
    }
}
#[inline(always)]
pub fn block_0x002135c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 336u32, 2176456u32);
    emu.adi_no_count(11usize, 2usize, 172u32, 2176460u32);
    emu.adi_no_count(12usize, 0usize, 164u32, 2176464u32);
    emu.apc_no_count(1usize, 2176464u32, 4294901760u32, 2176468u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2176472u32;
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
#[inline(always)]
pub fn block_0x002135d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 336u32, 2176476u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2176480u32);
    emu.adi_no_count(27usize, 0usize, 1u32, 2176484u32);
    emu.apc_no_count(1usize, 2176484u32, 4294938624u32, 2176488u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2176492u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(948u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002135ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 500u32, 2176496u32);
    emu.adi_no_count(11usize, 2usize, 172u32, 2176500u32);
    emu.adi_no_count(12usize, 0usize, 164u32, 2176504u32);
    emu.apc_no_count(1usize, 2176504u32, 4294901760u32, 2176508u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2176512u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965884u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00213600(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 500u32, 2176516u32);
    emu.adi_no_count(11usize, 0usize, 2u32, 2176520u32);
    emu.apc_no_count(1usize, 2176520u32, 4294938624u32, 2176524u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2176528u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(912u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00213610(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 664u32, 2176532u32);
    emu.adi_no_count(11usize, 2usize, 172u32, 2176536u32);
    emu.adi_no_count(12usize, 0usize, 164u32, 2176540u32);
    emu.apc_no_count(1usize, 2176540u32, 4294901760u32, 2176544u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2176548u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965848u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00213624(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 664u32, 2176552u32);
    emu.adi_no_count(11usize, 0usize, 3u32, 2176556u32);
    emu.apc_no_count(1usize, 2176556u32, 4294938624u32, 2176560u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2176564u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(876u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00213634(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2176568u32);
    emu.adi_no_count(12usize, 18usize, 1u32, 2176572u32);
    emu.lw_no_count(15usize, 2usize, 168u32, 2176576u32)?;
    emu.lw_no_count(16usize, 2usize, 824u32, 2176580u32)?;
    emu.lw_no_count(17usize, 2usize, 660u32, 2176584u32)?;
    emu.lw_no_count(5usize, 2usize, 496u32, 2176588u32)?;
    emu.lw_no_count(10usize, 2usize, 332u32, 2176592u32)?;
    emu.adi_no_count(1usize, 2usize, 660u32, 2176596u32);
    emu.adi_no_count(25usize, 2usize, 4u32, 2176600u32);
    emu.adi_no_count(29usize, 2usize, 332u32, 2176604u32);
    emu.adi_no_count(30usize, 2usize, 168u32, 2176608u32);
    emu.adi_no_count(31usize, 0usize, 41u32, 2176612u32);
    emu.adi_no_count(24usize, 0usize, 10u32, 2176616u32);
    emu.add_memory_rw_events(13usize);
    emu.pc = 2176616u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213668));
}
#[inline(always)]
pub fn block_0x00213668(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        return Ok(crate::NextStep::Dynamic(2177772u32));
    } else {
        emu.pc = 2176620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021366c));
    }
}
#[inline(always)]
pub fn block_0x0021366c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 27usize, 0u32, 2176624u32);
    emu.sli_no_count(13usize, 15usize, 2u32, 2176628u32);
    emu.adi_no_count(14usize, 2usize, 8u32, 2176632u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2176632u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213678));
}
#[inline(always)]
pub fn block_0x00213678(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2177460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2177460u32));
    } else {
        emu.pc = 2176636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021367c));
    }
}
#[inline(always)]
pub fn block_0x0021367c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(6usize, 14usize, 0u32, 2176640u32)?;
    emu.adi_no_count(14usize, 14usize, 4u32, 2176644u32);
    emu.adi_no_count(13usize, 13usize, 4294967292u32, 2176648u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2176632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213678));
    } else {
        emu.pc = 2176652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2176652u32));
    }
}
