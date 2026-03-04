pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2173332u32;
pub const PC_MAX: u32 = 2175100u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 123usize] = [
        block_0x00212994,
        block_0x002129a8,
        block_0x002129ac,
        block_0x002129c4,
        block_0x002129c8,
        block_0x002129cc,
        block_0x002129dc,
        block_0x00212a10,
        block_0x00212a14,
        block_0x00212a24,
        block_0x00212a28,
        block_0x00212a2c,
        block_0x00212a38,
        block_0x00212a3c,
        block_0x00212a40,
        block_0x00212a50,
        block_0x00212a54,
        block_0x00212a6c,
        block_0x00212a70,
        block_0x00212a74,
        block_0x00212a84,
        block_0x00212ab8,
        block_0x00212abc,
        block_0x00212ac8,
        block_0x00212acc,
        block_0x00212ad0,
        block_0x00212ad4,
        block_0x00212af0,
        block_0x00212af4,
        block_0x00212af8,
        block_0x00212b10,
        block_0x00212b14,
        block_0x00212b2c,
        block_0x00212b3c,
        block_0x00212b44,
        block_0x00212b58,
        block_0x00212b68,
        block_0x00212b6c,
        block_0x00212b78,
        block_0x00212b7c,
        block_0x00212b90,
        block_0x00212bc0,
        block_0x00212bc4,
        block_0x00212bc8,
        block_0x00212bdc,
        block_0x00212be4,
        block_0x00212be8,
        block_0x00212bec,
        block_0x00212c00,
        block_0x00212c04,
        block_0x00212c1c,
        block_0x00212c30,
        block_0x00212c34,
        block_0x00212c44,
        block_0x00212c48,
        block_0x00212c4c,
        block_0x00212c60,
        block_0x00212c88,
        block_0x00212c90,
        block_0x00212c94,
        block_0x00212c9c,
        block_0x00212ca4,
        block_0x00212ca8,
        block_0x00212cb4,
        block_0x00212cc8,
        block_0x00212cf0,
        block_0x00212cf4,
        block_0x00212cf8,
        block_0x00212d00,
        block_0x00212d08,
        block_0x00212d1c,
        block_0x00212d44,
        block_0x00212d48,
        block_0x00212d4c,
        block_0x00212d54,
        block_0x00212d64,
        block_0x00212d6c,
        block_0x00212d70,
        block_0x00212d78,
        block_0x00212d88,
        block_0x00212d94,
        block_0x00212d98,
        block_0x00212da0,
        block_0x00212dbc,
        block_0x00212dc0,
        block_0x00212dd8,
        block_0x00212ddc,
        block_0x00212df0,
        block_0x00212df8,
        block_0x00212e08,
        block_0x00212e24,
        block_0x00212e30,
        block_0x00212e48,
        block_0x00212e4c,
        block_0x00212e50,
        block_0x00212e60,
        block_0x00212e64,
        block_0x00212e68,
        block_0x00212e78,
        block_0x00212e90,
        block_0x00212e94,
        block_0x00212ea0,
        block_0x00212ea4,
        block_0x00212ef0,
        block_0x00212ef4,
        block_0x00212f00,
        block_0x00212f18,
        block_0x00212f2c,
        block_0x00212f38,
        block_0x00212f50,
        block_0x00212f58,
        block_0x00212f5c,
        block_0x00212f78,
        block_0x00212f90,
        block_0x00212fa8,
        block_0x00212fc4,
        block_0x00212fe0,
        block_0x00212ffc,
        block_0x00213018,
        block_0x00213034,
        block_0x00213050,
        block_0x00213064,
        block_0x0021307c,
    ];
    const IDX: [u16; 443usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 2u16, 3u16, 0u16, 0u16, 0u16, 0u16, 0u16, 4u16,
        5u16, 6u16, 0u16, 0u16, 0u16, 7u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 8u16, 9u16, 0u16, 0u16, 0u16, 10u16, 11u16, 12u16,
        0u16, 0u16, 13u16, 14u16, 15u16, 0u16, 0u16, 0u16, 16u16, 17u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 18u16, 19u16, 20u16, 0u16, 0u16, 0u16, 21u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 22u16, 23u16, 0u16, 0u16,
        24u16, 25u16, 26u16, 27u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 28u16, 29u16,
        30u16, 0u16, 0u16, 0u16, 0u16, 0u16, 31u16, 32u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        33u16, 0u16, 0u16, 0u16, 34u16, 0u16, 35u16, 0u16, 0u16, 0u16, 0u16, 36u16, 0u16,
        0u16, 0u16, 37u16, 38u16, 0u16, 0u16, 39u16, 40u16, 0u16, 0u16, 0u16, 0u16,
        41u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 42u16,
        43u16, 44u16, 0u16, 0u16, 0u16, 0u16, 45u16, 0u16, 46u16, 47u16, 48u16, 0u16,
        0u16, 0u16, 0u16, 49u16, 50u16, 0u16, 0u16, 0u16, 0u16, 0u16, 51u16, 0u16, 0u16,
        0u16, 0u16, 52u16, 53u16, 0u16, 0u16, 0u16, 54u16, 55u16, 56u16, 0u16, 0u16,
        0u16, 0u16, 57u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 58u16,
        0u16, 59u16, 60u16, 0u16, 61u16, 0u16, 62u16, 63u16, 0u16, 0u16, 64u16, 0u16,
        0u16, 0u16, 0u16, 65u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        66u16, 67u16, 68u16, 0u16, 69u16, 0u16, 70u16, 0u16, 0u16, 0u16, 0u16, 71u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 72u16, 73u16, 74u16, 0u16,
        75u16, 0u16, 0u16, 0u16, 76u16, 0u16, 77u16, 78u16, 0u16, 79u16, 0u16, 0u16,
        0u16, 80u16, 0u16, 0u16, 81u16, 82u16, 0u16, 83u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 84u16, 85u16, 0u16, 0u16, 0u16, 0u16, 0u16, 86u16, 87u16, 0u16, 0u16, 0u16,
        0u16, 88u16, 0u16, 89u16, 0u16, 0u16, 0u16, 90u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 91u16, 0u16, 0u16, 92u16, 0u16, 0u16, 0u16, 0u16, 0u16, 93u16, 94u16,
        95u16, 0u16, 0u16, 0u16, 96u16, 97u16, 98u16, 0u16, 0u16, 0u16, 99u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 100u16, 101u16, 0u16, 0u16, 102u16, 103u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 104u16, 105u16, 0u16, 0u16, 106u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 107u16, 0u16, 0u16, 0u16, 0u16, 108u16, 0u16, 0u16, 109u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 110u16, 0u16, 111u16, 112u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 113u16, 0u16, 0u16, 0u16, 0u16, 0u16, 114u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        115u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 116u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 117u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 118u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 119u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 120u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 121u16, 0u16, 0u16, 0u16, 0u16, 122u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 123u16,
    ];
    if pc < 2173332u32 || pc > 2175100u32 {
        return None;
    }
    let word_offset = ((pc - 2173332u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x00212994(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(14usize, 10usize, 2u32, 2173336u32);
    emu.sbr_no_count(12usize, 0usize, 14usize, 2173340u32);
    emu.adi_no_count(15usize, 2usize, 680u32, 2173344u32);
    emu.adr_no_count(15usize, 15usize, 14usize, 2173348u32);
    emu.adr_no_count(16usize, 22usize, 14usize, 2173352u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2173352u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002129a8));
}
#[inline(always)]
pub fn block_0x002129a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2173480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212a28));
    } else {
        emu.pc = 2173356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002129ac));
    }
}
#[inline(always)]
pub fn block_0x002129ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(17usize, 16usize, 0u32, 2173360u32)?;
    emu.lw_no_count(5usize, 15usize, 0u32, 2173364u32)?;
    emu.adi_no_count(12usize, 12usize, 4u32, 2173368u32);
    emu.adi_no_count(15usize, 15usize, 4294967292u32, 2173372u32);
    emu.adi_no_count(16usize, 16usize, 4294967292u32, 2173376u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2173352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002129a8));
    } else {
        emu.pc = 2173380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002129c4));
    }
}
#[inline(always)]
pub fn block_0x002129c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2173484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212a2c));
    } else {
        emu.pc = 2173384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002129c8));
    }
}
#[inline(always)]
pub fn block_0x002129c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2173460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212a14));
    } else {
        emu.pc = 2173388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002129cc));
    }
}
#[inline(always)]
pub fn block_0x002129cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 2usize, 684u32, 2173392u32);
    emu.adi_no_count(15usize, 0usize, 1u32, 2173396u32);
    emu.adr_no_count(14usize, 12usize, 14usize, 2173400u32);
    emu.adi_no_count(13usize, 2usize, 28u32, 2173404u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2173404u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002129dc));
}
#[inline]
pub fn block_0x002129dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 12usize, 0u32, 2173408u32)?;
    emu.lw_no_count(17usize, 13usize, 0u32, 2173412u32)?;
    emu.ani_no_count(15usize, 15usize, 1u32, 2173416u32);
    emu.adi_no_count(12usize, 12usize, 4u32, 2173420u32);
    emu.xri_no_count(16usize, 16usize, 4294967295u32, 2173424u32);
    emu.adr_no_count(16usize, 17usize, 16usize, 2173428u32);
    emu.sltru_no_count(17usize, 16usize, 17usize, 2173432u32);
    emu.adr_no_count(15usize, 16usize, 15usize, 2173436u32);
    emu.sltru_no_count(16usize, 15usize, 16usize, 2173440u32);
    emu.sw_no_count(15usize, 13usize, 0u32, 2173444u32)?;
    emu.orr_no_count(15usize, 17usize, 16usize, 2173448u32);
    emu.adi_no_count(13usize, 13usize, 4u32, 2173452u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2173404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002129dc));
    } else {
        emu.pc = 2173456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212a10));
    }
}
#[inline(always)]
pub fn block_0x00212a10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        return Ok(crate::NextStep::Direct(block_0x00212f5c));
    } else {
        emu.pc = 2173460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212a14));
    }
}
#[inline(always)]
pub fn block_0x00212a14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 188u32, 2173464u32)?;
    emu.adi_no_count(11usize, 11usize, 2u32, 2173468u32);
    emu.adi_no_count(21usize, 18usize, 0u32, 2173472u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2173500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212a3c));
    } else {
        emu.pc = 2173476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212a24));
    }
}
#[inline(always)]
pub fn block_0x00212a24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2173480u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2173496u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212a38));
}
#[inline(always)]
pub fn block_0x00212a28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2173384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002129c8));
    } else {
        emu.pc = 2173484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212a2c));
    }
}
#[inline(always)]
pub fn block_0x00212a2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 13usize, 0u32, 2173488u32);
    emu.adi_no_count(21usize, 18usize, 0u32, 2173492u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2173500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212a3c));
    } else {
        emu.pc = 2173496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212a38));
    }
}
#[inline(always)]
pub fn block_0x00212a38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(21usize, 10usize, 0u32, 2173500u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2173500u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212a3c));
}
#[inline(always)]
pub fn block_0x00212a3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a >= b {
        emu.pc = 2174864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212f90));
    } else {
        emu.pc = 2173504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212a40));
    }
}
#[inline(always)]
pub fn block_0x00212a40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 21usize, 2u32, 2173508u32);
    emu.sbr_no_count(13usize, 0usize, 12usize, 2173512u32);
    emu.adr_no_count(14usize, 7usize, 12usize, 2173516u32);
    emu.adr_no_count(15usize, 22usize, 12usize, 2173520u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2173520u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212a50));
}
#[inline(always)]
pub fn block_0x00212a50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2173640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212ac8));
    } else {
        emu.pc = 2173524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212a54));
    }
}
#[inline(always)]
pub fn block_0x00212a54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 15usize, 0u32, 2173528u32)?;
    emu.lw_no_count(17usize, 14usize, 0u32, 2173532u32)?;
    emu.adi_no_count(13usize, 13usize, 4u32, 2173536u32);
    emu.adi_no_count(14usize, 14usize, 4294967292u32, 2173540u32);
    emu.adi_no_count(15usize, 15usize, 4294967292u32, 2173544u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2173520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212a50));
    } else {
        emu.pc = 2173548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212a6c));
    }
}
#[inline(always)]
pub fn block_0x00212a6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a < b {
        emu.pc = 2173644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212acc));
    } else {
        emu.pc = 2173552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212a70));
    }
}
#[inline(always)]
pub fn block_0x00212a70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a == b {
        emu.pc = 2173628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212abc));
    } else {
        emu.pc = 2173556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212a74));
    }
}
#[inline(always)]
pub fn block_0x00212a74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 520u32, 2173560u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2173564u32);
    emu.adr_no_count(12usize, 10usize, 12usize, 2173568u32);
    emu.adi_no_count(13usize, 2usize, 28u32, 2173572u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2173572u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212a84));
}
#[inline]
pub fn block_0x00212a84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 10usize, 0u32, 2173576u32)?;
    emu.lw_no_count(16usize, 13usize, 0u32, 2173580u32)?;
    emu.ani_no_count(14usize, 14usize, 1u32, 2173584u32);
    emu.adi_no_count(10usize, 10usize, 4u32, 2173588u32);
    emu.xri_no_count(15usize, 15usize, 4294967295u32, 2173592u32);
    emu.adr_no_count(15usize, 16usize, 15usize, 2173596u32);
    emu.sltru_no_count(16usize, 15usize, 16usize, 2173600u32);
    emu.adr_no_count(14usize, 15usize, 14usize, 2173604u32);
    emu.sltru_no_count(15usize, 14usize, 15usize, 2173608u32);
    emu.sw_no_count(14usize, 13usize, 0u32, 2173612u32)?;
    emu.orr_no_count(14usize, 16usize, 15usize, 2173616u32);
    emu.adi_no_count(13usize, 13usize, 4u32, 2173620u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2173572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212a84));
    } else {
        emu.pc = 2173624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212ab8));
    }
}
#[inline(always)]
pub fn block_0x00212ab8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        return Ok(crate::NextStep::Direct(block_0x00212f5c));
    } else {
        emu.pc = 2173628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212abc));
    }
}
#[inline(always)]
pub fn block_0x00212abc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(21usize, 2usize, 188u32, 2173632u32)?;
    emu.adi_no_count(11usize, 11usize, 1u32, 2173636u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2173640u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2173648u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212ad0));
}
#[inline(always)]
pub fn block_0x00212ac8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2173552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212a70));
    } else {
        emu.pc = 2173644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212acc));
    }
}
#[inline(always)]
pub fn block_0x00212acc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(21usize, 10usize, 0u32, 2173648u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2173648u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212ad0));
}
#[inline(always)]
pub fn block_0x00212ad0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2175100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021307c));
    } else {
        emu.pc = 2173652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212ad4));
    }
}
#[inline(always)]
pub fn block_0x00212ad4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(25usize, 2usize, 352u32, 2173656u32)?;
    emu.adi_no_count(10usize, 11usize, 48u32, 2173660u32);
    emu.lw_no_count(19usize, 2usize, 20u32, 2173664u32)?;
    emu.adr_no_count(19usize, 19usize, 20usize, 2173668u32);
    emu.sb_no_count(10usize, 19usize, 0u32, 2173672u32);
    emu.adi_no_count(10usize, 25usize, 0u32, 2173676u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(25usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a < b {
        emu.pc = 2173684u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212af4));
    } else {
        emu.pc = 2173680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212af0));
    }
}
#[inline(always)]
pub fn block_0x00212af0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 21usize, 0u32, 2173684u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2173684u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212af4));
}
#[inline(always)]
pub fn block_0x00212af4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        return Ok(crate::NextStep::Direct(block_0x00212f18));
    } else {
        emu.pc = 2173688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212af8));
    }
}
#[inline(always)]
pub fn block_0x00212af8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(23usize, 20usize, 1u32, 2173692u32);
    emu.sli_no_count(12usize, 10usize, 2u32, 2173696u32);
    emu.sbr_no_count(10usize, 0usize, 12usize, 2173700u32);
    emu.adi_no_count(11usize, 2usize, 188u32, 2173704u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2173708u32);
    emu.adr_no_count(12usize, 22usize, 12usize, 2173712u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2173712u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212b10));
}
#[inline(always)]
pub fn block_0x00212b10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2173756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212b3c));
    } else {
        emu.pc = 2173716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212b14));
    }
}
#[inline(always)]
pub fn block_0x00212b14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 12usize, 0u32, 2173720u32)?;
    emu.lw_no_count(14usize, 11usize, 0u32, 2173724u32)?;
    emu.adi_no_count(10usize, 10usize, 4u32, 2173728u32);
    emu.adi_no_count(11usize, 11usize, 4294967292u32, 2173732u32);
    emu.adi_no_count(12usize, 12usize, 4294967292u32, 2173736u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2173712u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212b10));
    } else {
        emu.pc = 2173740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212b2c));
    }
}
#[inline(always)]
pub fn block_0x00212b2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 13usize, 14usize, 2173744u32);
    emu.sltru_no_count(11usize, 14usize, 13usize, 2173748u32);
    emu.sbr_no_count(24usize, 11usize, 10usize, 2173752u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2173756u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2173764u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212b44));
}
#[inline(always)]
pub fn block_0x00212b3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(24usize, 10usize, 1u32, 2173760u32);
    emu.adi_no_count(24usize, 24usize, 4294967295u32, 2173764u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2173764u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212b44));
}
#[inline(always)]
pub fn block_0x00212b44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1176u32, 2173768u32);
    emu.adi_no_count(11usize, 2usize, 28u32, 2173772u32);
    emu.adi_no_count(12usize, 0usize, 164u32, 2173776u32);
    emu.apc_no_count(1usize, 2173776u32, 4294901760u32, 2173780u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2173784u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1336u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00212b58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 2usize, 1336u32, 2173788u32)?;
    emu.lw_no_count(11usize, 2usize, 516u32, 2173792u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2173796u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2173804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212b6c));
    } else {
        emu.pc = 2173800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212b68));
    }
}
#[inline(always)]
pub fn block_0x00212b68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 12usize, 0u32, 2173804u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2173804u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212b6c));
}
#[inline(always)]
pub fn block_0x00212b6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(6usize, 0usize, 40u32, 2173808u32);
    emu.adi_no_count(7usize, 2usize, 516u32, 2173812u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2174744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212f18));
    } else {
        emu.pc = 2173816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212b78));
    }
}
#[inline(always)]
pub fn block_0x00212b78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2173916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212bdc));
    } else {
        emu.pc = 2173820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212b7c));
    }
}
#[inline(always)]
pub fn block_0x00212b7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 0u32, 2173824u32);
    emu.sli_no_count(12usize, 10usize, 2u32, 2173828u32);
    emu.adi_no_count(13usize, 2usize, 356u32, 2173832u32);
    emu.adr_no_count(14usize, 13usize, 12usize, 2173836u32);
    emu.adi_no_count(15usize, 2usize, 1176u32, 2173840u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2173840u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212b90));
}
#[inline]
pub fn block_0x00212b90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(17usize, 15usize, 0u32, 2173844u32)?;
    emu.lw_no_count(5usize, 13usize, 0u32, 2173848u32)?;
    emu.ani_no_count(16usize, 16usize, 1u32, 2173852u32);
    emu.adi_no_count(13usize, 13usize, 4u32, 2173856u32);
    emu.adr_no_count(5usize, 17usize, 5usize, 2173860u32);
    emu.sltru_no_count(17usize, 5usize, 17usize, 2173864u32);
    emu.adr_no_count(16usize, 5usize, 16usize, 2173868u32);
    emu.sltru_no_count(5usize, 16usize, 5usize, 2173872u32);
    emu.sw_no_count(16usize, 15usize, 0u32, 2173876u32)?;
    emu.orr_no_count(16usize, 17usize, 5usize, 2173880u32);
    emu.adi_no_count(15usize, 15usize, 4u32, 2173884u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2173840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212b90));
    } else {
        emu.pc = 2173888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212bc0));
    }
}
#[inline(always)]
pub fn block_0x00212bc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2173916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212bdc));
    } else {
        emu.pc = 2173892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212bc4));
    }
}
#[inline(always)]
pub fn block_0x00212bc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2175076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213064));
    } else {
        emu.pc = 2173896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212bc8));
    }
}
#[inline(always)]
pub fn block_0x00212bc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 2usize, 1176u32, 2173900u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2173904u32);
    emu.adi_no_count(13usize, 0usize, 1u32, 2173908u32);
    emu.sw_no_count(13usize, 12usize, 0u32, 2173912u32)?;
    emu.adi_no_count(10usize, 10usize, 1u32, 2173916u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2173916u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212bdc));
}
#[inline(always)]
pub fn block_0x00212bdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 1336u32, 2173920u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a < b {
        emu.pc = 2173928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212be8));
    } else {
        emu.pc = 2173924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212be4));
    }
}
#[inline(always)]
pub fn block_0x00212be4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2173928u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2173928u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212be8));
}
#[inline(always)]
pub fn block_0x00212be8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        return Ok(crate::NextStep::Direct(block_0x00212f18));
    } else {
        emu.pc = 2173932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212bec));
    }
}
#[inline(always)]
pub fn block_0x00212bec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(13usize, 10usize, 2u32, 2173936u32);
    emu.sbr_no_count(10usize, 0usize, 13usize, 2173940u32);
    emu.adi_no_count(12usize, 2usize, 1172u32, 2173944u32);
    emu.adr_no_count(12usize, 12usize, 13usize, 2173948u32);
    emu.adr_no_count(13usize, 7usize, 13usize, 2173952u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2173952u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212c00));
}
#[inline(always)]
pub fn block_0x00212c00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2174004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212c34));
    } else {
        emu.pc = 2173956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212c04));
    }
}
#[inline(always)]
pub fn block_0x00212c04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 13usize, 0u32, 2173960u32)?;
    emu.lw_no_count(15usize, 12usize, 0u32, 2173964u32)?;
    emu.adi_no_count(10usize, 10usize, 4u32, 2173968u32);
    emu.adi_no_count(12usize, 12usize, 4294967292u32, 2173972u32);
    emu.adi_no_count(13usize, 13usize, 4294967292u32, 2173976u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2173952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212c00));
    } else {
        emu.pc = 2173980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212c1c));
    }
}
#[inline(always)]
pub fn block_0x00212c1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 14usize, 15usize, 2173984u32);
    emu.sltru_no_count(12usize, 15usize, 14usize, 2173988u32);
    emu.sbr_no_count(10usize, 12usize, 10usize, 2173992u32);
    emu.lw_no_count(12usize, 2usize, 24u32, 2173996u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(24usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2174020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212c44));
    } else {
        emu.pc = 2174000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212c30));
    }
}
#[inline(always)]
pub fn block_0x00212c30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2174004u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2174316u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212d6c));
}
#[inline(always)]
pub fn block_0x00212c34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(10usize, 10usize, 1u32, 2174008u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2174012u32);
    emu.lw_no_count(12usize, 2usize, 24u32, 2174016u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(24usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2174316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212d6c));
    } else {
        emu.pc = 2174020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212c44));
    }
}
#[inline(always)]
pub fn block_0x00212c44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2174316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212d6c));
    } else {
        emu.pc = 2174024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212c48));
    }
}
#[inline(always)]
pub fn block_0x00212c48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a == b {
        emu.pc = 2174120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212ca8));
    } else {
        emu.pc = 2174028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212c4c));
    }
}
#[inline(always)]
pub fn block_0x00212c4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2174032u32);
    emu.sli_no_count(13usize, 21usize, 2u32, 2174036u32);
    emu.adi_no_count(10usize, 2usize, 28u32, 2174040u32);
    emu.adr_no_count(10usize, 10usize, 13usize, 2174044u32);
    emu.adi_no_count(14usize, 2usize, 28u32, 2174048u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2174048u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212c60));
}
#[inline]
pub fn block_0x00212c60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 14usize, 0u32, 2174052u32)?;
    emu.mulhu_no_count(16usize, 15usize, 9usize, 2174056u32);
    emu.mul_no_count(15usize, 15usize, 9usize, 2174060u32);
    emu.adr_no_count(12usize, 15usize, 12usize, 2174064u32);
    emu.sw_no_count(12usize, 14usize, 0u32, 2174068u32)?;
    emu.adi_no_count(14usize, 14usize, 4u32, 2174072u32);
    emu.sltru_no_count(12usize, 12usize, 15usize, 2174076u32);
    emu.adi_no_count(13usize, 13usize, 4294967292u32, 2174080u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2174084u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2174048u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212c60));
    } else {
        emu.pc = 2174088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212c88));
    }
}
#[inline(always)]
pub fn block_0x00212c88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(24usize, 2usize, 16u32, 2174092u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2174108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212c9c));
    } else {
        emu.pc = 2174096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212c90));
    }
}
#[inline(always)]
pub fn block_0x00212c90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a == b {
        emu.pc = 2175076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213064));
    } else {
        emu.pc = 2174100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212c94));
    }
}
#[inline(always)]
pub fn block_0x00212c94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(12usize, 10usize, 0u32, 2174104u32)?;
    emu.adi_no_count(21usize, 21usize, 1u32, 2174108u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2174108u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212c9c));
}
#[inline(always)]
pub fn block_0x00212c9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(21usize, 2usize, 188u32, 2174112u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a != b {
        emu.pc = 2174132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212cb4));
    } else {
        emu.pc = 2174116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212ca4));
    }
}
#[inline(always)]
pub fn block_0x00212ca4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2174120u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2174208u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212d00));
}
#[inline(always)]
pub fn block_0x00212ca8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(24usize, 2usize, 16u32, 2174124u32)?;
    emu.sw_no_count(21usize, 2usize, 188u32, 2174128u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a == b {
        emu.pc = 2174208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212d00));
    } else {
        emu.pc = 2174132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212cb4));
    }
}
#[inline(always)]
pub fn block_0x00212cb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2174136u32);
    emu.sli_no_count(13usize, 25usize, 2u32, 2174140u32);
    emu.adi_no_count(10usize, 2usize, 192u32, 2174144u32);
    emu.adr_no_count(10usize, 10usize, 13usize, 2174148u32);
    emu.adi_no_count(14usize, 2usize, 192u32, 2174152u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2174152u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212cc8));
}
#[inline]
pub fn block_0x00212cc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 14usize, 0u32, 2174156u32)?;
    emu.mulhu_no_count(16usize, 15usize, 9usize, 2174160u32);
    emu.mul_no_count(15usize, 15usize, 9usize, 2174164u32);
    emu.adr_no_count(12usize, 15usize, 12usize, 2174168u32);
    emu.sw_no_count(12usize, 14usize, 0u32, 2174172u32)?;
    emu.adi_no_count(14usize, 14usize, 4u32, 2174176u32);
    emu.sltru_no_count(12usize, 12usize, 15usize, 2174180u32);
    emu.adi_no_count(13usize, 13usize, 4294967292u32, 2174184u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2174188u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2174152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212cc8));
    } else {
        emu.pc = 2174192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212cf0));
    }
}
#[inline(always)]
pub fn block_0x00212cf0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2174208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212d00));
    } else {
        emu.pc = 2174196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212cf4));
    }
}
#[inline(always)]
pub fn block_0x00212cf4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a == b {
        emu.pc = 2175076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213064));
    } else {
        emu.pc = 2174200u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212cf8));
    }
}
#[inline(always)]
pub fn block_0x00212cf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(12usize, 10usize, 0u32, 2174204u32)?;
    emu.adi_no_count(25usize, 25usize, 1u32, 2174208u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2174208u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212d00));
}
#[inline(always)]
pub fn block_0x00212d00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(25usize, 2usize, 352u32, 2174212u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2174292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212d54));
    } else {
        emu.pc = 2174216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212d08));
    }
}
#[inline(always)]
pub fn block_0x00212d08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2174220u32);
    emu.sli_no_count(13usize, 11usize, 2u32, 2174224u32);
    emu.adi_no_count(10usize, 2usize, 356u32, 2174228u32);
    emu.adr_no_count(10usize, 10usize, 13usize, 2174232u32);
    emu.adi_no_count(14usize, 2usize, 356u32, 2174236u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2174236u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212d1c));
}
#[inline]
pub fn block_0x00212d1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 14usize, 0u32, 2174240u32)?;
    emu.mulhu_no_count(16usize, 15usize, 9usize, 2174244u32);
    emu.mul_no_count(15usize, 15usize, 9usize, 2174248u32);
    emu.adr_no_count(12usize, 15usize, 12usize, 2174252u32);
    emu.sw_no_count(12usize, 14usize, 0u32, 2174256u32)?;
    emu.adi_no_count(14usize, 14usize, 4u32, 2174260u32);
    emu.sltru_no_count(12usize, 12usize, 15usize, 2174264u32);
    emu.adi_no_count(13usize, 13usize, 4294967292u32, 2174268u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2174272u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2174236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212d1c));
    } else {
        emu.pc = 2174276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212d44));
    }
}
#[inline(always)]
pub fn block_0x00212d44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2174292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212d54));
    } else {
        emu.pc = 2174280u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212d48));
    }
}
#[inline(always)]
pub fn block_0x00212d48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2175076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213064));
    } else {
        emu.pc = 2174284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212d4c));
    }
}
#[inline(always)]
pub fn block_0x00212d4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(12usize, 10usize, 0u32, 2174288u32)?;
    emu.adi_no_count(11usize, 11usize, 1u32, 2174292u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2174292u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212d54));
}
#[inline(always)]
pub fn block_0x00212d54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 2usize, 516u32, 2174296u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2174300u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2174304u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a < b {
        emu.pc = 2172976u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2172976u32));
    } else {
        emu.pc = 2174308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212d64));
    }
}
#[inline(always)]
pub fn block_0x00212d64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 21usize, 0u32, 2174312u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2174316u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2172976u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(2172976u32));
}
#[inline(always)]
pub fn block_0x00212d6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2174544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212e50));
    } else {
        emu.pc = 2174320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212d70));
    }
}
#[inline(always)]
pub fn block_0x00212d70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(22usize, 2usize, 8u32, 2174324u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(24usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2174428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212ddc));
    } else {
        emu.pc = 2174328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212d78));
    }
}
#[inline(always)]
pub fn block_0x00212d78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 28u32, 2174332u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2174336u32);
    emu.apc_no_count(1usize, 2174336u32, 4294942720u32, 2174340u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2174344u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966596u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00212d88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 188u32, 2174348u32)?;
    emu.lw_no_count(10usize, 2usize, 680u32, 2174352u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2174360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212d98));
    } else {
        emu.pc = 2174356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212d94));
    }
}
#[inline(always)]
pub fn block_0x00212d94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 11usize, 0u32, 2174360u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2174360u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212d98));
}
#[inline(always)]
pub fn block_0x00212d98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 41u32, 2174364u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2174744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212f18));
    } else {
        emu.pc = 2174368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212da0));
    }
}
#[inline(always)]
pub fn block_0x00212da0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 10usize, 2u32, 2174372u32);
    emu.adi_no_count(12usize, 2usize, 520u32, 2174376u32);
    emu.adi_no_count(13usize, 2usize, 28u32, 2174380u32);
    emu.sbr_no_count(10usize, 0usize, 11usize, 2174384u32);
    emu.adi_no_count(14usize, 11usize, 4294967292u32, 2174388u32);
    emu.adr_no_count(11usize, 12usize, 14usize, 2174392u32);
    emu.adr_no_count(12usize, 13usize, 14usize, 2174396u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2174396u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212dbc));
}
#[inline(always)]
pub fn block_0x00212dbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2174704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212ef0));
    } else {
        emu.pc = 2174400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212dc0));
    }
}
#[inline(always)]
pub fn block_0x00212dc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 12usize, 0u32, 2174404u32)?;
    emu.lw_no_count(14usize, 11usize, 0u32, 2174408u32)?;
    emu.adi_no_count(10usize, 10usize, 4u32, 2174412u32);
    emu.adi_no_count(11usize, 11usize, 4294967292u32, 2174416u32);
    emu.adi_no_count(12usize, 12usize, 4294967292u32, 2174420u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2174396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212dbc));
    } else {
        emu.pc = 2174424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212dd8));
    }
}
#[inline(always)]
pub fn block_0x00212dd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2174708u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212ef4));
    } else {
        emu.pc = 2174428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212ddc));
    }
}
#[inline(always)]
pub fn block_0x00212ddc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2174432u32);
    emu.lw_no_count(21usize, 2usize, 20u32, 2174436u32)?;
    emu.adr_no_count(8usize, 21usize, 23usize, 2174440u32);
    emu.adi_no_count(10usize, 0usize, 4294967295u32, 2174444u32);
    emu.adi_no_count(12usize, 0usize, 57u32, 2174448u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2174448u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212df0));
}
#[inline(always)]
pub fn block_0x00212df0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(13usize, 20usize, 11usize, 2174452u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2174564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212e64));
    } else {
        emu.pc = 2174456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212df8));
    }
}
#[inline(always)]
pub fn block_0x00212df8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(13usize, 19usize, 11usize, 2174460u32);
    emu.lbu_no_count(13usize, 13usize, 0u32, 2174464u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2174468u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2174448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212df0));
    } else {
        emu.pc = 2174472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212e08));
    }
}
#[inline(always)]
pub fn block_0x00212e08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 20usize, 11usize, 2174476u32);
    emu.adr_no_count(12usize, 10usize, 21usize, 2174480u32);
    emu.lbu_no_count(13usize, 12usize, 1u32, 2174484u32);
    emu.adi_no_count(13usize, 13usize, 1u32, 2174488u32);
    emu.adi_no_count(10usize, 10usize, 2u32, 2174492u32);
    emu.sb_no_count(13usize, 12usize, 1u32, 2174496u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a < b {
        emu.pc = 2175056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213050));
    } else {
        emu.pc = 2174500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212e24));
    }
}
#[inline(always)]
pub fn block_0x00212e24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4294967295u32, 2174504u32);
    emu.lw_no_count(18usize, 2usize, 16u32, 2174508u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2174624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212ea0));
    } else {
        emu.pc = 2174512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212e30));
    }
}
#[inline(always)]
pub fn block_0x00212e30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(12usize, 11usize, 4294967295u32, 2174516u32);
    emu.adr_no_count(10usize, 19usize, 11usize, 2174520u32);
    emu.adi_no_count(10usize, 10usize, 2u32, 2174524u32);
    emu.adi_no_count(11usize, 0usize, 48u32, 2174528u32);
    emu.apc_no_count(1usize, 2174528u32, 4294901760u32, 2174532u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2174536u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(336u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00212e48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a >= b {
        emu.pc = 2174628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212ea4));
    } else {
        emu.pc = 2174540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212e4c));
    }
}
#[inline(always)]
pub fn block_0x00212e4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2174544u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2174720u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212f00));
}
#[inline(always)]
pub fn block_0x00212e50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 2usize, 16u32, 2174548u32)?;
    emu.lw_no_count(21usize, 2usize, 20u32, 2174552u32)?;
    emu.lw_no_count(22usize, 2usize, 8u32, 2174556u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a >= b {
        emu.pc = 2174628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212ea4));
    } else {
        emu.pc = 2174560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212e60));
    }
}
#[inline(always)]
pub fn block_0x00212e60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2174564u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2174720u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212f00));
}
#[inline(always)]
pub fn block_0x00212e64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a == b {
        emu.pc = 2174764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212f2c));
    } else {
        emu.pc = 2174568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212e68));
    }
}
#[inline(always)]
pub fn block_0x00212e68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 49u32, 2174572u32);
    emu.sb_no_count(10usize, 21usize, 0u32, 2174576u32);
    emu.lw_no_count(18usize, 2usize, 16u32, 2174580u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2174800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212f50));
    } else {
        emu.pc = 2174584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212e78));
    }
}
#[inline(always)]
pub fn block_0x00212e78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 21usize, 1u32, 2174588u32);
    emu.adi_no_count(11usize, 0usize, 48u32, 2174592u32);
    emu.adi_no_count(9usize, 0usize, 48u32, 2174596u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2174600u32);
    emu.apc_no_count(1usize, 2174600u32, 4294901760u32, 2174604u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2174608u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(264u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00212e90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a >= b {
        emu.pc = 2174776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212f38));
    } else {
        emu.pc = 2174612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212e94));
    }
}
#[inline(always)]
pub fn block_0x00212e94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(9usize, 8usize, 0u32, 2174616u32);
    emu.adi_no_count(23usize, 20usize, 2u32, 2174620u32);
    emu.adi_no_count(22usize, 22usize, 1u32, 2174624u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2174624u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212ea0));
}
#[inline(always)]
pub fn block_0x00212ea0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a < b {
        emu.pc = 2174720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212f00));
    } else {
        emu.pc = 2174628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212ea4));
    }
}
#[inline]
pub fn block_0x00212ea4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 4u32, 2174632u32)?;
    emu.sw_no_count(21usize, 10usize, 0u32, 2174636u32)?;
    emu.sw_no_count(23usize, 10usize, 4u32, 2174640u32)?;
    emu.sh_no_count(22usize, 10usize, 8u32, 2174644u32)?;
    emu.lw_no_count(1usize, 2usize, 1388u32, 2174648u32)?;
    emu.lw_no_count(8usize, 2usize, 1384u32, 2174652u32)?;
    emu.lw_no_count(9usize, 2usize, 1380u32, 2174656u32)?;
    emu.lw_no_count(18usize, 2usize, 1376u32, 2174660u32)?;
    emu.lw_no_count(19usize, 2usize, 1372u32, 2174664u32)?;
    emu.lw_no_count(20usize, 2usize, 1368u32, 2174668u32)?;
    emu.lw_no_count(21usize, 2usize, 1364u32, 2174672u32)?;
    emu.lw_no_count(22usize, 2usize, 1360u32, 2174676u32)?;
    emu.lw_no_count(23usize, 2usize, 1356u32, 2174680u32)?;
    emu.lw_no_count(24usize, 2usize, 1352u32, 2174684u32)?;
    emu.lw_no_count(25usize, 2usize, 1348u32, 2174688u32)?;
    emu.lw_no_count(26usize, 2usize, 1344u32, 2174692u32)?;
    emu.lw_no_count(27usize, 2usize, 1340u32, 2174696u32)?;
    emu.adi_no_count(2usize, 2usize, 1392u32, 2174700u32);
    emu.add_memory_rw_events(19usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2174704u32;
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
pub fn block_0x00212ef0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2174428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212ddc));
    } else {
        emu.pc = 2174708u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212ef4));
    }
}
#[inline(always)]
pub fn block_0x00212ef4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 2usize, 16u32, 2174712u32)?;
    emu.lw_no_count(21usize, 2usize, 20u32, 2174716u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a >= b {
        emu.pc = 2174628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212ea4));
    } else {
        emu.pc = 2174720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212f00));
    }
}
#[inline(always)]
pub fn block_0x00212f00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2174724u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965952u32, 2174728u32);
    emu.adi_no_count(10usize, 23usize, 0u32, 2174732u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2174736u32);
    emu.apc_no_count(1usize, 2174736u32, 4096u32, 2174740u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2174744u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966924u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00212f18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2174748u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1160u32, 2174752u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2174756u32);
    emu.apc_no_count(1usize, 2174756u32, 4096u32, 2174760u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2174764u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966904u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00212f2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 49u32, 2174768u32);
    emu.lw_no_count(18usize, 2usize, 16u32, 2174772u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a < b {
        emu.pc = 2174612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212e94));
    } else {
        emu.pc = 2174776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212f38));
    }
}
#[inline(always)]
pub fn block_0x00212f38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2174780u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965936u32, 2174784u32);
    emu.adi_no_count(10usize, 23usize, 0u32, 2174788u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2174792u32);
    emu.apc_no_count(1usize, 2174792u32, 4294938624u32, 2174796u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2174800u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1684u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00212f50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 48u32, 2174804u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a < b {
        emu.pc = 2174612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212e94));
    } else {
        emu.pc = 2174808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212f58));
    }
}
#[inline(always)]
pub fn block_0x00212f58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2174812u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2174776u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212f38));
}
#[inline(always)]
pub fn block_0x00212f5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2174816u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1176u32, 2174820u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2174824u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1160u32, 2174828u32);
    emu.adi_no_count(11usize, 0usize, 26u32, 2174832u32);
    emu.apc_no_count(1usize, 2174832u32, 4294938624u32, 2174836u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2174840u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1584u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00212f78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2174844u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1160u32, 2174848u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2174852u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2174856u32);
    emu.apc_no_count(1usize, 2174856u32, 4096u32, 2174860u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2174864u32;
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
pub fn block_0x00212f90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2174868u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1160u32, 2174872u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2174876u32);
    emu.adi_no_count(10usize, 21usize, 0u32, 2174880u32);
    emu.apc_no_count(1usize, 2174880u32, 4096u32, 2174884u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2174888u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966780u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00212fa8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2174892u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965720u32, 2174896u32);
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2174900u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965748u32, 2174904u32);
    emu.adi_no_count(11usize, 0usize, 28u32, 2174908u32);
    emu.apc_no_count(1usize, 2174908u32, 4294938624u32, 2174912u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2174916u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1508u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00212fc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2174920u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965764u32, 2174924u32);
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2174928u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965796u32, 2174932u32);
    emu.adi_no_count(11usize, 0usize, 29u32, 2174936u32);
    emu.apc_no_count(1usize, 2174936u32, 4294938624u32, 2174940u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2174944u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1480u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00212fe0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2174948u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965812u32, 2174952u32);
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2174956u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965840u32, 2174960u32);
    emu.adi_no_count(11usize, 0usize, 28u32, 2174964u32);
    emu.apc_no_count(1usize, 2174964u32, 4294938624u32, 2174968u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2174972u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1452u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00212ffc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2174976u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966040u32, 2174980u32);
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2174984u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966096u32, 2174988u32);
    emu.adi_no_count(11usize, 0usize, 54u32, 2174992u32);
    emu.apc_no_count(1usize, 2174992u32, 4294938624u32, 2174996u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2175000u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1424u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00213018(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2175004u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965968u32, 2175008u32);
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2175012u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966024u32, 2175016u32);
    emu.adi_no_count(11usize, 0usize, 55u32, 2175020u32);
    emu.apc_no_count(1usize, 2175020u32, 4294938624u32, 2175024u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2175028u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1396u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00213034(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2175032u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965856u32, 2175036u32);
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2175040u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965904u32, 2175044u32);
    emu.adi_no_count(11usize, 0usize, 45u32, 2175048u32);
    emu.apc_no_count(1usize, 2175048u32, 4294938624u32, 2175052u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2175056u32;
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
pub fn block_0x00213050(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2175060u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1308u32, 2175064u32);
    emu.adi_no_count(11usize, 23usize, 0u32, 2175068u32);
    emu.apc_no_count(1usize, 2175068u32, 4096u32, 2175072u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2175076u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966584u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00213064(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2175080u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1160u32, 2175084u32);
    emu.adi_no_count(10usize, 0usize, 40u32, 2175088u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2175092u32);
    emu.apc_no_count(1usize, 2175092u32, 4294938624u32, 2175096u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2175100u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1384u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021307c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2175104u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965920u32, 2175108u32);
    emu.adi_no_count(10usize, 24usize, 0u32, 2175112u32);
    emu.adi_no_count(11usize, 24usize, 0u32, 2175116u32);
    emu.apc_no_count(1usize, 2175116u32, 4294938624u32, 2175120u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2175124u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1360u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
