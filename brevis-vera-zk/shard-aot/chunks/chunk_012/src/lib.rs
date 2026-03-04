pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2149060u32;
pub const PC_MAX: u32 = 2151320u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 122usize] = [
        block_0x0020cac4,
        block_0x0020caec,
        block_0x0020caf8,
        block_0x0020cb00,
        block_0x0020cb28,
        block_0x0020cb2c,
        block_0x0020cb48,
        block_0x0020cb58,
        block_0x0020cb6c,
        block_0x0020cb70,
        block_0x0020cb7c,
        block_0x0020cb88,
        block_0x0020cba8,
        block_0x0020cbb0,
        block_0x0020cbbc,
        block_0x0020cbc4,
        block_0x0020cbd0,
        block_0x0020cbf8,
        block_0x0020cc0c,
        block_0x0020cc10,
        block_0x0020cc34,
        block_0x0020cc50,
        block_0x0020cc68,
        block_0x0020cc80,
        block_0x0020cc98,
        block_0x0020ccc4,
        block_0x0020ccfc,
        block_0x0020cd08,
        block_0x0020cd0c,
        block_0x0020cd2c,
        block_0x0020cd40,
        block_0x0020cd44,
        block_0x0020cd98,
        block_0x0020cda0,
        block_0x0020cda8,
        block_0x0020cdb8,
        block_0x0020cdc0,
        block_0x0020cdc8,
        block_0x0020cdd0,
        block_0x0020cdd8,
        block_0x0020cddc,
        block_0x0020cdf0,
        block_0x0020cdf8,
        block_0x0020cdfc,
        block_0x0020ce04,
        block_0x0020ce28,
        block_0x0020ce40,
        block_0x0020ced8,
        block_0x0020ceec,
        block_0x0020cf08,
        block_0x0020cf10,
        block_0x0020cf18,
        block_0x0020cf1c,
        block_0x0020cf24,
        block_0x0020cf2c,
        block_0x0020cf3c,
        block_0x0020cf40,
        block_0x0020cf4c,
        block_0x0020cf54,
        block_0x0020cf60,
        block_0x0020cf64,
        block_0x0020cf70,
        block_0x0020cf78,
        block_0x0020cf80,
        block_0x0020cfb4,
        block_0x0020cfbc,
        block_0x0020cfc0,
        block_0x0020cfc8,
        block_0x0020cfd4,
        block_0x0020cfdc,
        block_0x0020cfe4,
        block_0x0020cfe8,
        block_0x0020cff8,
        block_0x0020d000,
        block_0x0020d00c,
        block_0x0020d010,
        block_0x0020d014,
        block_0x0020d020,
        block_0x0020d024,
        block_0x0020d034,
        block_0x0020d044,
        block_0x0020d058,
        block_0x0020d05c,
        block_0x0020d060,
        block_0x0020d064,
        block_0x0020d07c,
        block_0x0020d098,
        block_0x0020d09c,
        block_0x0020d0a0,
        block_0x0020d0a8,
        block_0x0020d0b0,
        block_0x0020d0b4,
        block_0x0020d0f0,
        block_0x0020d11c,
        block_0x0020d13c,
        block_0x0020d144,
        block_0x0020d164,
        block_0x0020d194,
        block_0x0020d1d0,
        block_0x0020d208,
        block_0x0020d224,
        block_0x0020d234,
        block_0x0020d240,
        block_0x0020d244,
        block_0x0020d26c,
        block_0x0020d27c,
        block_0x0020d2cc,
        block_0x0020d2d0,
        block_0x0020d2e8,
        block_0x0020d2ec,
        block_0x0020d2fc,
        block_0x0020d300,
        block_0x0020d31c,
        block_0x0020d320,
        block_0x0020d328,
        block_0x0020d33c,
        block_0x0020d344,
        block_0x0020d35c,
        block_0x0020d364,
        block_0x0020d380,
        block_0x0020d388,
        block_0x0020d398,
    ];
    const IDX: [u16; 566usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16,
        3u16, 0u16, 4u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 5u16,
        6u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 7u16, 0u16, 0u16, 0u16, 8u16, 0u16,
        0u16, 0u16, 0u16, 9u16, 10u16, 0u16, 0u16, 11u16, 0u16, 0u16, 12u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 13u16, 0u16, 14u16, 0u16, 0u16, 15u16, 0u16, 16u16,
        0u16, 0u16, 17u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 18u16,
        0u16, 0u16, 0u16, 0u16, 19u16, 20u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 21u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 22u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 23u16, 0u16, 0u16, 0u16, 0u16, 0u16, 24u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        25u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 26u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 27u16,
        0u16, 0u16, 28u16, 29u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 30u16, 0u16,
        0u16, 0u16, 0u16, 31u16, 32u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 33u16,
        0u16, 34u16, 0u16, 35u16, 0u16, 0u16, 0u16, 36u16, 0u16, 37u16, 0u16, 38u16,
        0u16, 39u16, 0u16, 40u16, 41u16, 0u16, 0u16, 0u16, 0u16, 42u16, 0u16, 43u16,
        44u16, 0u16, 45u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 46u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 47u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 48u16, 0u16, 0u16, 0u16, 0u16, 49u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 50u16, 0u16, 51u16, 0u16, 52u16, 53u16, 0u16, 54u16, 0u16, 55u16,
        0u16, 0u16, 0u16, 56u16, 57u16, 0u16, 0u16, 58u16, 0u16, 59u16, 0u16, 0u16,
        60u16, 61u16, 0u16, 0u16, 62u16, 0u16, 63u16, 0u16, 64u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 65u16, 0u16, 66u16, 67u16,
        0u16, 68u16, 0u16, 0u16, 69u16, 0u16, 70u16, 0u16, 71u16, 72u16, 0u16, 0u16,
        0u16, 73u16, 0u16, 74u16, 0u16, 0u16, 75u16, 76u16, 77u16, 0u16, 0u16, 78u16,
        79u16, 0u16, 0u16, 0u16, 80u16, 0u16, 0u16, 0u16, 81u16, 0u16, 0u16, 0u16, 0u16,
        82u16, 83u16, 84u16, 85u16, 0u16, 0u16, 0u16, 0u16, 0u16, 86u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 87u16, 88u16, 89u16, 0u16, 90u16, 0u16, 91u16, 92u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 93u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 94u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 95u16, 0u16, 96u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 97u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 98u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 99u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 100u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        101u16, 0u16, 0u16, 0u16, 102u16, 0u16, 0u16, 103u16, 104u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 105u16, 0u16, 0u16, 0u16, 106u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 107u16, 108u16, 0u16, 0u16, 0u16, 0u16, 0u16, 109u16,
        110u16, 0u16, 0u16, 0u16, 111u16, 112u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        113u16, 114u16, 0u16, 115u16, 0u16, 0u16, 0u16, 0u16, 116u16, 0u16, 117u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 118u16, 0u16, 119u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        120u16, 0u16, 121u16, 0u16, 0u16, 0u16, 122u16,
    ];
    if pc < 2149060u32 || pc > 2151320u32 {
        return None;
    }
    let word_offset = ((pc - 2149060u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x0020cac4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2149064u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2149068u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2149072u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2149076u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2149080u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2149084u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2149088u32)?;
    emu.adi_no_count(8usize, 11usize, 0u32, 2149092u32);
    emu.adi_no_count(11usize, 0usize, 1280u32, 2149096u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a >= b {
        emu.pc = 2149428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cc34));
    } else {
        emu.pc = 2149100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020caec));
    }
}
#[inline(always)]
pub fn block_0x0020caec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 160u32, 2149104u32)?;
    emu.sri_no_count(19usize, 8usize, 5u32, 2149108u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2149192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cb48));
    } else {
        emu.pc = 2149112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020caf8));
    }
}
#[inline(always)]
pub fn block_0x0020caf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 40u32, 2149116u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2149480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cc68));
    } else {
        emu.pc = 2149120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cb00));
    }
}
#[inline]
pub fn block_0x0020cb00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 0usize, 12usize, 2149124u32);
    emu.sli_no_count(13usize, 12usize, 2u32, 2149128u32);
    emu.adr_no_count(12usize, 12usize, 19usize, 2149132u32);
    emu.adr_no_count(14usize, 13usize, 10usize, 2149136u32);
    emu.adi_no_count(13usize, 12usize, 4294967295u32, 2149140u32);
    emu.sli_no_count(15usize, 12usize, 2u32, 2149144u32);
    emu.adi_no_count(12usize, 14usize, 4294967292u32, 2149148u32);
    emu.adr_no_count(14usize, 15usize, 10usize, 2149152u32);
    emu.adi_no_count(14usize, 14usize, 4294967292u32, 2149156u32);
    emu.adi_no_count(15usize, 0usize, 39u32, 2149160u32);
    emu.add_memory_rw_events(10usize);
    emu.pc = 2149160u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cb28));
}
#[inline(always)]
pub fn block_0x0020cb28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2149456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cc50));
    } else {
        emu.pc = 2149164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cb2c));
    }
}
#[inline(always)]
pub fn block_0x0020cb2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 12usize, 0u32, 2149168u32)?;
    emu.adi_no_count(11usize, 11usize, 1u32, 2149172u32);
    emu.adi_no_count(12usize, 12usize, 4294967292u32, 2149176u32);
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2149180u32);
    emu.sw_no_count(16usize, 14usize, 0u32, 2149184u32)?;
    emu.adi_no_count(14usize, 14usize, 4294967292u32, 2149188u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2149160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cb28));
    } else {
        emu.pc = 2149192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cb48));
    }
}
#[inline(always)]
pub fn block_0x0020cb48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(20usize, 8usize, 31u32, 2149196u32);
    emu.adi_no_count(11usize, 0usize, 32u32, 2149200u32);
    emu.sli_no_count(9usize, 19usize, 2u32, 2149204u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a < b {
        emu.pc = 2149232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cb70));
    } else {
        emu.pc = 2149208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cb58));
    }
}
#[inline(always)]
pub fn block_0x0020cb58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2149212u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2149216u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2149220u32);
    emu.apc_no_count(1usize, 2149220u32, 4294926336u32, 2149224u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2149228u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1068u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020cb6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2149232u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2149232u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cb70));
}
#[inline(always)]
pub fn block_0x0020cb70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 10usize, 160u32, 2149236u32)?;
    emu.adr_no_count(14usize, 14usize, 19usize, 2149240u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2149388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cc0c));
    } else {
        emu.pc = 2149244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cb7c));
    }
}
#[inline(always)]
pub fn block_0x0020cb7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 14usize, 4294967295u32, 2149248u32);
    emu.adi_no_count(11usize, 0usize, 39u32, 2149252u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2149456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cc50));
    } else {
        emu.pc = 2149256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cb88));
    }
}
#[inline(always)]
pub fn block_0x0020cb88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(13usize, 13usize, 2u32, 2149260u32);
    emu.adr_no_count(13usize, 10usize, 13usize, 2149264u32);
    emu.lw_no_count(11usize, 13usize, 0u32, 2149268u32)?;
    emu.sbr_no_count(12usize, 0usize, 8usize, 2149272u32);
    emu.srr_no_count(15usize, 11usize, 12usize, 2149276u32);
    emu.sli_no_count(13usize, 14usize, 2u32, 2149280u32);
    emu.adi_no_count(11usize, 14usize, 0u32, 2149284u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2149308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cbbc));
    } else {
        emu.pc = 2149288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cba8));
    }
}
#[inline(always)]
pub fn block_0x0020cba8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 39u32, 2149292u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2149504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cc80));
    } else {
        emu.pc = 2149296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cbb0));
    }
}
#[inline(always)]
pub fn block_0x0020cbb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 10usize, 13usize, 2149300u32);
    emu.sw_no_count(15usize, 11usize, 0u32, 2149304u32)?;
    emu.adi_no_count(11usize, 14usize, 1u32, 2149308u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2149308u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cbbc));
}
#[inline(always)]
pub fn block_0x0020cbbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 19usize, 1u32, 2149312u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a >= b {
        emu.pc = 2149368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cbf8));
    } else {
        emu.pc = 2149316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cbc4));
    }
}
#[inline(always)]
pub fn block_0x0020cbc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(12usize, 12usize, 31u32, 2149320u32);
    emu.adr_no_count(13usize, 13usize, 10usize, 2149324u32);
    emu.adi_no_count(13usize, 13usize, 4294967292u32, 2149328u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2149328u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cbd0));
}
#[inline]
pub fn block_0x0020cbd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 13usize, 0u32, 2149332u32)?;
    emu.lw_no_count(16usize, 13usize, 4294967292u32, 2149336u32)?;
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2149340u32);
    emu.slr_no_count(15usize, 15usize, 20usize, 2149344u32);
    emu.srr_no_count(16usize, 16usize, 12usize, 2149348u32);
    emu.adi_no_count(17usize, 13usize, 4294967292u32, 2149352u32);
    emu.orr_no_count(15usize, 16usize, 15usize, 2149356u32);
    emu.sw_no_count(15usize, 13usize, 0u32, 2149360u32)?;
    emu.adi_no_count(13usize, 17usize, 0u32, 2149364u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a < b {
        emu.pc = 2149328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cbd0));
    } else {
        emu.pc = 2149368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cbf8));
    }
}
#[inline(always)]
pub fn block_0x0020cbf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(9usize, 10usize, 9usize, 2149372u32);
    emu.lw_no_count(12usize, 9usize, 0u32, 2149376u32)?;
    emu.slr_no_count(12usize, 12usize, 20usize, 2149380u32);
    emu.sw_no_count(12usize, 9usize, 0u32, 2149384u32)?;
    emu.add_memory_rw_events(5usize);
    let return_addr = 2149388u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2149392u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cc10));
}
#[inline(always)]
pub fn block_0x0020cc0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 14usize, 0u32, 2149392u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2149392u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cc10));
}
#[inline]
pub fn block_0x0020cc10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 10usize, 160u32, 2149396u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2149400u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2149404u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2149408u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2149412u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2149416u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2149420u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2149424u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2149428u32;
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
pub fn block_0x0020cc34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2149432u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1202u32, 2149436u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2149440u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1160u32, 2149444u32);
    emu.adi_no_count(11usize, 0usize, 29u32, 2149448u32);
    emu.apc_no_count(1usize, 2149448u32, 0u32, 2149452u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2149456u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965592u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020cc50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2149460u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1160u32, 2149464u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2149468u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2149472u32);
    emu.apc_no_count(1usize, 2149472u32, 0u32, 2149476u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2149480u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965628u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020cc68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 12usize, 4294967295u32, 2149484u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2149488u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1160u32, 2149492u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2149496u32);
    emu.apc_no_count(1usize, 2149496u32, 0u32, 2149500u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2149504u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965604u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020cc80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2149508u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1160u32, 2149512u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2149516u32);
    emu.adi_no_count(10usize, 14usize, 0u32, 2149520u32);
    emu.apc_no_count(1usize, 2149520u32, 0u32, 2149524u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2149528u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965580u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020cc98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2149532u32);
    emu.sw_no_count(8usize, 2usize, 28u32, 2149536u32)?;
    emu.sw_no_count(9usize, 2usize, 24u32, 2149540u32)?;
    emu.sw_no_count(18usize, 2usize, 20u32, 2149544u32)?;
    emu.sw_no_count(19usize, 2usize, 16u32, 2149548u32)?;
    emu.sw_no_count(20usize, 2usize, 12u32, 2149552u32)?;
    emu.sw_no_count(21usize, 2usize, 8u32, 2149556u32)?;
    emu.sw_no_count(22usize, 2usize, 4u32, 2149560u32)?;
    emu.sli_no_count(12usize, 12usize, 2u32, 2149564u32);
    emu.adr_no_count(12usize, 11usize, 12usize, 2149568u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2149832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cdc8));
    } else {
        emu.pc = 2149572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ccc4));
    }
}
#[inline]
pub fn block_0x0020ccc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 10usize, 0u32, 2149576u32);
    emu.adi_no_count(5usize, 0usize, 0u32, 2149580u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2149584u32);
    emu.sli_no_count(6usize, 14usize, 2u32, 2149588u32);
    emu.sltru_no_count(15usize, 0usize, 14usize, 2149592u32);
    emu.adi_no_count(17usize, 14usize, 1u32, 2149596u32);
    emu.adi_no_count(7usize, 14usize, 4294967295u32, 2149600u32);
    emu.adr_no_count(6usize, 13usize, 6usize, 2149604u32);
    emu.sli_no_count(15usize, 15usize, 2u32, 2149608u32);
    emu.sli_no_count(28usize, 7usize, 2u32, 2149612u32);
    emu.adr_no_count(7usize, 13usize, 15usize, 2149616u32);
    emu.sri_no_count(28usize, 28usize, 2u32, 2149620u32);
    emu.adi_no_count(28usize, 28usize, 1u32, 2149624u32);
    emu.adi_no_count(29usize, 0usize, 40u32, 2149628u32);
    emu.add_memory_rw_events(14usize);
    emu.pc = 2149628u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ccfc));
}
#[inline(always)]
pub fn block_0x0020ccfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(9usize, 5usize, 2u32, 2149632u32);
    emu.adr_no_count(9usize, 16usize, 9usize, 2149636u32);
    emu.adi_no_count(15usize, 11usize, 0u32, 2149640u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2149640u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cd08));
}
#[inline(always)]
pub fn block_0x0020cd08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2149892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ce04));
    } else {
        emu.pc = 2149644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cd0c));
    }
}
#[inline(always)]
pub fn block_0x0020cd0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(30usize, 5usize, 0u32, 2149648u32);
    emu.adi_no_count(31usize, 9usize, 0u32, 2149652u32);
    emu.lw_no_count(8usize, 15usize, 0u32, 2149656u32)?;
    emu.adi_no_count(5usize, 5usize, 1u32, 2149660u32);
    emu.adi_no_count(11usize, 15usize, 4u32, 2149664u32);
    emu.adi_no_count(9usize, 9usize, 4u32, 2149668u32);
    emu.adi_no_count(15usize, 11usize, 0u32, 2149672u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2149640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cd08));
    } else {
        emu.pc = 2149676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cd2c));
    }
}
#[inline(always)]
pub fn block_0x0020cd2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 0u32, 2149680u32);
    emu.adi_no_count(9usize, 28usize, 0u32, 2149684u32);
    emu.adi_no_count(15usize, 30usize, 0u32, 2149688u32);
    emu.adi_no_count(20usize, 7usize, 0u32, 2149692u32);
    emu.adi_no_count(21usize, 13usize, 0u32, 2149696u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2149696u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cd40));
}
#[inline(always)]
pub fn block_0x0020cd40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2149928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ce28));
    } else {
        emu.pc = 2149700u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cd44));
    }
}
#[inline]
pub fn block_0x0020cd44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 20usize, 0u32, 2149704u32);
    emu.lw_no_count(21usize, 21usize, 0u32, 2149708u32)?;
    emu.lw_no_count(20usize, 31usize, 0u32, 2149712u32)?;
    emu.mulhu_no_count(22usize, 21usize, 8usize, 2149716u32);
    emu.adr_no_count(18usize, 20usize, 18usize, 2149720u32);
    emu.sltru_no_count(20usize, 18usize, 20usize, 2149724u32);
    emu.adr_no_count(22usize, 20usize, 22usize, 2149728u32);
    emu.xrr_no_count(20usize, 19usize, 6usize, 2149732u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2149736u32);
    emu.adi_no_count(9usize, 9usize, 4294967295u32, 2149740u32);
    emu.sltru_no_count(20usize, 0usize, 20usize, 2149744u32);
    emu.sli_no_count(20usize, 20usize, 2u32, 2149748u32);
    emu.adr_no_count(20usize, 19usize, 20usize, 2149752u32);
    emu.mul_no_count(21usize, 21usize, 8usize, 2149756u32);
    emu.adr_no_count(21usize, 18usize, 21usize, 2149760u32);
    emu.sltru_no_count(18usize, 21usize, 18usize, 2149764u32);
    emu.sw_no_count(21usize, 31usize, 0u32, 2149768u32)?;
    emu.adr_no_count(18usize, 22usize, 18usize, 2149772u32);
    emu.adi_no_count(31usize, 31usize, 4u32, 2149776u32);
    emu.adi_no_count(21usize, 19usize, 0u32, 2149780u32);
    emu.add_memory_rw_events(20usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2149696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cd40));
    } else {
        emu.pc = 2149784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cd98));
    }
}
#[inline(always)]
pub fn block_0x0020cd98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 14usize, 0u32, 2149788u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2149816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cdb8));
    } else {
        emu.pc = 2149792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cda0));
    }
}
#[inline(always)]
pub fn block_0x0020cda0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 30usize, 14usize, 2149796u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2149928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ce28));
    } else {
        emu.pc = 2149800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cda8));
    }
}
#[inline(always)]
pub fn block_0x0020cda8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(15usize, 15usize, 2u32, 2149804u32);
    emu.adr_no_count(15usize, 16usize, 15usize, 2149808u32);
    emu.sw_no_count(18usize, 15usize, 0u32, 2149812u32)?;
    emu.adi_no_count(15usize, 17usize, 0u32, 2149816u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2149816u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cdb8));
}
#[inline(always)]
pub fn block_0x0020cdb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 15usize, 30usize, 2149820u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2149628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ccfc));
    } else {
        emu.pc = 2149824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cdc0));
    }
}
#[inline(always)]
pub fn block_0x0020cdc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 15usize, 0u32, 2149828u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2149832u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2149628u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ccfc));
}
#[inline(always)]
pub fn block_0x0020cdc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 0u32, 2149836u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2149840u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2149840u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cdd0));
}
#[inline(always)]
pub fn block_0x0020cdd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(13usize, 0usize, 13usize, 2149844u32);
    emu.adi_no_count(14usize, 11usize, 0u32, 2149848u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2149848u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cdd8));
}
#[inline(always)]
pub fn block_0x0020cdd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2149892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ce04));
    } else {
        emu.pc = 2149852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cddc));
    }
}
#[inline(always)]
pub fn block_0x0020cddc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 14usize, 0u32, 2149856u32)?;
    emu.adi_no_count(11usize, 14usize, 4u32, 2149860u32);
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2149864u32);
    emu.adi_no_count(14usize, 11usize, 0u32, 2149868u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2149848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cdd8));
    } else {
        emu.pc = 2149872u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cdf0));
    }
}
#[inline(always)]
pub fn block_0x0020cdf0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(14usize, 13usize, 4294967295u32, 2149876u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2149884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cdfc));
    } else {
        emu.pc = 2149880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cdf8));
    }
}
#[inline(always)]
pub fn block_0x0020cdf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 14usize, 0u32, 2149884u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2149884u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cdfc));
}
#[inline(always)]
pub fn block_0x0020cdfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(13usize, 0usize, 13usize, 2149888u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2149892u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2149840u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cdd0));
}
#[inline]
pub fn block_0x0020ce04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(8usize, 2usize, 28u32, 2149896u32)?;
    emu.lw_no_count(9usize, 2usize, 24u32, 2149900u32)?;
    emu.lw_no_count(18usize, 2usize, 20u32, 2149904u32)?;
    emu.lw_no_count(19usize, 2usize, 16u32, 2149908u32)?;
    emu.lw_no_count(20usize, 2usize, 12u32, 2149912u32)?;
    emu.lw_no_count(21usize, 2usize, 8u32, 2149916u32)?;
    emu.lw_no_count(22usize, 2usize, 4u32, 2149920u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2149924u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2149928u32;
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
pub fn block_0x0020ce28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2149932u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1160u32, 2149936u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2149940u32);
    emu.adi_no_count(10usize, 15usize, 0u32, 2149944u32);
    emu.apc_no_count(1usize, 2149944u32, 4294963200u32, 2149948u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2149952u32;
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
#[inline(never)]
pub fn block_0x0020ce40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 38u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967216u32, 2149956u32);
    emu.sw_no_count(1usize, 2usize, 76u32, 2149960u32)?;
    emu.sw_no_count(8usize, 2usize, 72u32, 2149964u32)?;
    emu.sw_no_count(9usize, 2usize, 68u32, 2149968u32)?;
    emu.sw_no_count(18usize, 2usize, 64u32, 2149972u32)?;
    emu.sw_no_count(19usize, 2usize, 60u32, 2149976u32)?;
    emu.sw_no_count(20usize, 2usize, 56u32, 2149980u32)?;
    emu.sw_no_count(21usize, 2usize, 52u32, 2149984u32)?;
    emu.sw_no_count(22usize, 2usize, 48u32, 2149988u32)?;
    emu.sw_no_count(23usize, 2usize, 44u32, 2149992u32)?;
    emu.sw_no_count(24usize, 2usize, 40u32, 2149996u32)?;
    emu.sw_no_count(25usize, 2usize, 36u32, 2150000u32)?;
    emu.sw_no_count(26usize, 2usize, 32u32, 2150004u32)?;
    emu.sw_no_count(27usize, 2usize, 28u32, 2150008u32)?;
    emu.adi_no_count(8usize, 12usize, 0u32, 2150012u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2150016u32);
    emu.adi_no_count(20usize, 0usize, 0u32, 2150020u32);
    emu.adi_no_count(21usize, 0usize, 0u32, 2150024u32);
    emu.adi_no_count(25usize, 0usize, 0u32, 2150028u32);
    let a = 0u32.wrapping_add(168431616u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2150032u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2150036u32;
    emu.update_insn_clock();
    emu.lw_no_count(13usize, 10usize, 0u32, 2150040u32)?;
    emu.sw_no_count(13usize, 2usize, 24u32, 2150044u32)?;
    emu.lw_no_count(13usize, 10usize, 4u32, 2150048u32)?;
    emu.sw_no_count(13usize, 2usize, 20u32, 2150052u32)?;
    emu.lw_no_count(22usize, 10usize, 8u32, 2150056u32)?;
    emu.adi_no_count(10usize, 9usize, 4294967295u32, 2150060u32);
    emu.sw_no_count(10usize, 2usize, 12u32, 2150064u32)?;
    emu.adi_no_count(10usize, 9usize, 4u32, 2150068u32);
    emu.sw_no_count(10usize, 2usize, 8u32, 2150072u32)?;
    emu.sbr_no_count(10usize, 0usize, 8usize, 2150076u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2150080u32)?;
    emu.adi_no_count(27usize, 11usize, 4294965770u32, 2150084u32);
    emu.adi_no_count(19usize, 12usize, 256u32, 2150088u32);
    emu.adi_no_count(24usize, 0usize, 10u32, 2150092u32);
    let a = 0u32.wrapping_add(2155905024u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2150096u32;
    emu.update_insn_clock();
    emu.adi_no_count(23usize, 10usize, 128u32, 2150100u32);
    emu.add_memory_rw_events(38usize);
    let return_addr = 2150104u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2150160u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cf10));
}
#[inline(always)]
pub fn block_0x0020ced8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2150108u32)?;
    emu.adr_no_count(10usize, 10usize, 26usize, 2150112u32);
    emu.lbu_no_count(10usize, 10usize, 0u32, 2150116u32);
    emu.adi_no_count(10usize, 10usize, 4294967286u32, 2150120u32);
    emu.sltiu_no_count(10usize, 10usize, 1u32, 2150124u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2150124u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ceec));
}
#[inline(always)]
pub fn block_0x0020ceec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(10usize, 22usize, 0u32, 2150128u32);
    emu.lw_no_count(10usize, 2usize, 20u32, 2150132u32)?;
    emu.lw_no_count(13usize, 10usize, 12u32, 2150136u32)?;
    emu.sbr_no_count(12usize, 26usize, 20usize, 2150140u32);
    emu.adr_no_count(11usize, 9usize, 20usize, 2150144u32);
    emu.lw_no_count(10usize, 2usize, 24u32, 2150148u32)?;
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2150152u32;
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
pub fn block_0x0020cf08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 18usize, 0u32, 2150156u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2150576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d0b0));
    } else {
        emu.pc = 2150160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cf10));
    }
}
#[inline(always)]
pub fn block_0x0020cf10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 25usize, 1u32, 2150164u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2150568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d0a8));
    } else {
        emu.pc = 2150168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cf18));
    }
}
#[inline(always)]
pub fn block_0x0020cf18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a >= b {
        emu.pc = 2150188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cf2c));
    } else {
        emu.pc = 2150172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cf1c));
    }
}
#[inline(always)]
pub fn block_0x0020cf1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 21usize, 0u32, 2150176u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2150180u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2150496u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d060));
}
#[inline(always)]
pub fn block_0x0020cf24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(21usize, 26usize, 0u32, 2150184u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a < b {
        emu.pc = 2150496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d060));
    } else {
        emu.pc = 2150188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cf2c));
    }
}
#[inline(always)]
pub fn block_0x0020cf2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 8usize, 21usize, 2150192u32);
    emu.adr_no_count(10usize, 9usize, 21usize, 2150196u32);
    emu.adi_no_count(12usize, 0usize, 7u32, 2150200u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2150244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cf64));
    } else {
        emu.pc = 2150204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cf3c));
    }
}
#[inline(always)]
pub fn block_0x0020cf3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2150492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d05c));
    } else {
        emu.pc = 2150208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cf40));
    }
}
#[inline(always)]
pub fn block_0x0020cf40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2150212u32);
    emu.lw_no_count(12usize, 2usize, 16u32, 2150216u32)?;
    emu.adr_no_count(12usize, 12usize, 21usize, 2150220u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2150220u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cf4c));
}
#[inline(always)]
pub fn block_0x0020cf4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(13usize, 10usize, 0u32, 2150224u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2150416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d010));
    } else {
        emu.pc = 2150228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cf54));
    }
}
#[inline(always)]
pub fn block_0x0020cf54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2150232u32);
    emu.adi_no_count(10usize, 10usize, 1u32, 2150236u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2150220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cf4c));
    } else {
        emu.pc = 2150240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cf60));
    }
}
#[inline(always)]
pub fn block_0x0020cf60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2150244u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2150492u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d05c));
}
#[inline(always)]
pub fn block_0x0020cf64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 3u32, 2150248u32);
    emu.ani_no_count(12usize, 12usize, 4294967292u32, 2150252u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2150336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cfc0));
    } else {
        emu.pc = 2150256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cf70));
    }
}
#[inline(always)]
pub fn block_0x0020cf70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2150260u32);
    emu.adi_no_count(13usize, 11usize, 4294967288u32, 2150264u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2150264u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cf78));
}
#[inline(always)]
pub fn block_0x0020cf78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 2usize, 8u32, 2150268u32)?;
    emu.adr_no_count(14usize, 14usize, 21usize, 2150272u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2150272u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cf80));
}
#[inline]
pub fn block_0x0020cf80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 10usize, 12usize, 2150276u32);
    emu.adr_no_count(16usize, 14usize, 12usize, 2150280u32);
    emu.lw_no_count(15usize, 15usize, 0u32, 2150284u32)?;
    emu.lw_no_count(16usize, 16usize, 0u32, 2150288u32)?;
    emu.xrr_no_count(17usize, 15usize, 27usize, 2150292u32);
    emu.xrr_no_count(16usize, 16usize, 27usize, 2150296u32);
    emu.sbr_no_count(17usize, 19usize, 17usize, 2150300u32);
    emu.orr_no_count(15usize, 17usize, 15usize, 2150304u32);
    emu.sbr_no_count(17usize, 19usize, 16usize, 2150308u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2150312u32);
    emu.anr_no_count(15usize, 15usize, 16usize, 2150316u32);
    emu.anr_no_count(15usize, 15usize, 23usize, 2150320u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2150372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cfe4));
    } else {
        emu.pc = 2150324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cfb4));
    }
}
#[inline(always)]
pub fn block_0x0020cfb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 8u32, 2150328u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2150272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cf80));
    } else {
        emu.pc = 2150332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cfbc));
    }
}
#[inline(always)]
pub fn block_0x0020cfbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2150336u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2150372u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cfe4));
}
#[inline(always)]
pub fn block_0x0020cfc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 0u32, 2150340u32);
    emu.sbr_no_count(12usize, 12usize, 10usize, 2150344u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2150344u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cfc8));
}
#[inline(always)]
pub fn block_0x0020cfc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(14usize, 10usize, 13usize, 2150348u32);
    emu.lbu_no_count(14usize, 14usize, 0u32, 2150352u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2150420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d014));
    } else {
        emu.pc = 2150356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cfd4));
    }
}
#[inline(always)]
pub fn block_0x0020cfd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 1u32, 2150360u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2150344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cfc8));
    } else {
        emu.pc = 2150364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cfdc));
    }
}
#[inline(always)]
pub fn block_0x0020cfdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 11usize, 4294967288u32, 2150368u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2150264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cf78));
    } else {
        emu.pc = 2150372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cfe4));
    }
}
#[inline(always)]
pub fn block_0x0020cfe4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2150492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d05c));
    } else {
        emu.pc = 2150376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cfe8));
    }
}
#[inline(always)]
pub fn block_0x0020cfe8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 10usize, 12usize, 2150380u32);
    emu.sbr_no_count(10usize, 0usize, 12usize, 2150384u32);
    emu.lw_no_count(12usize, 2usize, 16u32, 2150388u32)?;
    emu.adr_no_count(12usize, 12usize, 21usize, 2150392u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2150392u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cff8));
}
#[inline(always)]
pub fn block_0x0020cff8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(13usize, 11usize, 0u32, 2150396u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2150436u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d024));
    } else {
        emu.pc = 2150400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d000));
    }
}
#[inline(always)]
pub fn block_0x0020d000(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2150404u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2150408u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2150392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cff8));
    } else {
        emu.pc = 2150412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d00c));
    }
}
#[inline(always)]
pub fn block_0x0020d00c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2150416u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2150492u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d05c));
}
#[inline(always)]
pub fn block_0x0020d010(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(13usize, 0usize, 11usize, 2150420u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2150420u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d014));
}
#[inline(always)]
pub fn block_0x0020d014(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 21usize, 13usize, 2150424u32);
    emu.adi_no_count(26usize, 10usize, 1u32, 2150428u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2150180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cf24));
    } else {
        emu.pc = 2150432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d020));
    }
}
#[inline(always)]
pub fn block_0x0020d020(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2150436u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2150452u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d034));
}
#[inline(always)]
pub fn block_0x0020d024(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(13usize, 0usize, 10usize, 2150440u32);
    emu.adr_no_count(10usize, 21usize, 13usize, 2150444u32);
    emu.adi_no_count(26usize, 10usize, 1u32, 2150448u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2150180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cf24));
    } else {
        emu.pc = 2150452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d034));
    }
}
#[inline(always)]
pub fn block_0x0020d034(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(21usize, 9usize, 21usize, 2150456u32);
    emu.adr_no_count(13usize, 21usize, 13usize, 2150460u32);
    emu.lbu_no_count(10usize, 13usize, 0u32, 2150464u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2150180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cf24));
    } else {
        emu.pc = 2150468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d044));
    }
}
#[inline(always)]
pub fn block_0x0020d044(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 0usize, 0u32, 2150472u32);
    emu.adi_no_count(18usize, 26usize, 0u32, 2150476u32);
    emu.adi_no_count(21usize, 26usize, 0u32, 2150480u32);
    emu.lbu_no_count(10usize, 22usize, 0u32, 2150484u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2150556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d09c));
    } else {
        emu.pc = 2150488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d058));
    }
}
#[inline(always)]
pub fn block_0x0020d058(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2150492u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2150524u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d07c));
}
#[inline(always)]
pub fn block_0x0020d05c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 8usize, 0u32, 2150496u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2150496u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d060));
}
#[inline(always)]
pub fn block_0x0020d060(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2150568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d0a8));
    } else {
        emu.pc = 2150500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d064));
    }
}
#[inline(always)]
pub fn block_0x0020d064(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 0usize, 1u32, 2150504u32);
    emu.adi_no_count(18usize, 20usize, 0u32, 2150508u32);
    emu.adi_no_count(21usize, 26usize, 0u32, 2150512u32);
    emu.adi_no_count(26usize, 8usize, 0u32, 2150516u32);
    emu.lbu_no_count(10usize, 22usize, 0u32, 2150520u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2150556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d09c));
    } else {
        emu.pc = 2150524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d07c));
    }
}
#[inline(always)]
pub fn block_0x0020d07c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 20u32, 2150528u32)?;
    emu.lw_no_count(13usize, 10usize, 12u32, 2150532u32)?;
    emu.adi_no_count(12usize, 0usize, 4u32, 2150536u32);
    emu.lw_no_count(10usize, 2usize, 24u32, 2150540u32)?;
    let a = 0u32.wrapping_add(2183168u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2150544u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1492u32, 2150548u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2150552u32;
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
pub fn block_0x0020d098(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2150576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d0b0));
    } else {
        emu.pc = 2150556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d09c));
    }
}
#[inline(always)]
pub fn block_0x0020d09c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a != b {
        emu.pc = 2150104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ced8));
    } else {
        emu.pc = 2150560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d0a0));
    }
}
#[inline(always)]
pub fn block_0x0020d0a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2150564u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2150568u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2150124u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ceec));
}
#[inline(always)]
pub fn block_0x0020d0a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2150572u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2150576u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2150580u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d0b4));
}
#[inline(always)]
pub fn block_0x0020d0b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2150580u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2150580u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d0b4));
}
#[inline]
pub fn block_0x0020d0b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 76u32, 2150584u32)?;
    emu.lw_no_count(8usize, 2usize, 72u32, 2150588u32)?;
    emu.lw_no_count(9usize, 2usize, 68u32, 2150592u32)?;
    emu.lw_no_count(18usize, 2usize, 64u32, 2150596u32)?;
    emu.lw_no_count(19usize, 2usize, 60u32, 2150600u32)?;
    emu.lw_no_count(20usize, 2usize, 56u32, 2150604u32)?;
    emu.lw_no_count(21usize, 2usize, 52u32, 2150608u32)?;
    emu.lw_no_count(22usize, 2usize, 48u32, 2150612u32)?;
    emu.lw_no_count(23usize, 2usize, 44u32, 2150616u32)?;
    emu.lw_no_count(24usize, 2usize, 40u32, 2150620u32)?;
    emu.lw_no_count(25usize, 2usize, 36u32, 2150624u32)?;
    emu.lw_no_count(26usize, 2usize, 32u32, 2150628u32)?;
    emu.lw_no_count(27usize, 2usize, 28u32, 2150632u32)?;
    emu.adi_no_count(2usize, 2usize, 80u32, 2150636u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2150640u32;
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
pub fn block_0x0020d0f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2150644u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2150648u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2150652u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2150656u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2150660u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2150664u32)?;
    emu.lw_no_count(9usize, 10usize, 8u32, 2150668u32)?;
    emu.lbu_no_count(12usize, 9usize, 0u32, 2150672u32);
    emu.lw_no_count(8usize, 10usize, 0u32, 2150676u32)?;
    emu.lw_no_count(18usize, 10usize, 4u32, 2150680u32)?;
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2150756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d164));
    } else {
        emu.pc = 2150684u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d11c));
    }
}
#[inline(always)]
pub fn block_0x0020d11c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 18usize, 12u32, 2150688u32)?;
    let a = 0u32.wrapping_add(2183168u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2150692u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1492u32, 2150696u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2150700u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2150704u32);
    emu.adi_no_count(19usize, 11usize, 0u32, 2150708u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2150712u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2150716u32;
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
pub fn block_0x0020d13c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 19usize, 0u32, 2150720u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2150756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d164));
    } else {
        emu.pc = 2150724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d144));
    }
}
#[inline(always)]
pub fn block_0x0020d144(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2150728u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2150732u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2150736u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2150740u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2150744u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2150748u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2150752u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2150756u32;
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
pub fn block_0x0020d164(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 11usize, 4294967286u32, 2150760u32);
    emu.sltiu_no_count(10usize, 10usize, 1u32, 2150764u32);
    emu.sb_no_count(10usize, 9usize, 0u32, 2150768u32);
    emu.lw_no_count(6usize, 18usize, 16u32, 2150772u32)?;
    emu.adi_no_count(10usize, 8usize, 0u32, 2150776u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2150780u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2150784u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2150788u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2150792u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2150796u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2150800u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2150804u32;
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
pub fn block_0x0020d194(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967216u32, 2150808u32);
    emu.sw_no_count(1usize, 2usize, 76u32, 2150812u32)?;
    emu.sw_no_count(8usize, 2usize, 72u32, 2150816u32)?;
    emu.sw_no_count(9usize, 2usize, 68u32, 2150820u32)?;
    emu.sw_no_count(18usize, 2usize, 64u32, 2150824u32)?;
    emu.sw_no_count(19usize, 2usize, 60u32, 2150828u32)?;
    emu.sw_no_count(20usize, 2usize, 56u32, 2150832u32)?;
    emu.sw_no_count(21usize, 2usize, 52u32, 2150836u32)?;
    emu.sw_no_count(22usize, 2usize, 48u32, 2150840u32)?;
    emu.sw_no_count(23usize, 2usize, 44u32, 2150844u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2150848u32);
    emu.lbu_no_count(10usize, 10usize, 4u32, 2150852u32);
    emu.adi_no_count(21usize, 0usize, 1u32, 2150856u32);
    emu.adi_no_count(20usize, 0usize, 1u32, 2150860u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2150920u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d208));
    } else {
        emu.pc = 2150864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d1d0));
    }
}
#[inline]
pub fn block_0x0020d1d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(20usize, 8usize, 4u32, 2150868u32);
    emu.sb_no_count(21usize, 8usize, 5u32, 2150872u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2150876u32);
    emu.lw_no_count(1usize, 2usize, 76u32, 2150880u32)?;
    emu.lw_no_count(8usize, 2usize, 72u32, 2150884u32)?;
    emu.lw_no_count(9usize, 2usize, 68u32, 2150888u32)?;
    emu.lw_no_count(18usize, 2usize, 64u32, 2150892u32)?;
    emu.lw_no_count(19usize, 2usize, 60u32, 2150896u32)?;
    emu.lw_no_count(20usize, 2usize, 56u32, 2150900u32)?;
    emu.lw_no_count(21usize, 2usize, 52u32, 2150904u32)?;
    emu.lw_no_count(22usize, 2usize, 48u32, 2150908u32)?;
    emu.lw_no_count(23usize, 2usize, 44u32, 2150912u32)?;
    emu.adi_no_count(2usize, 2usize, 80u32, 2150916u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2150920u32;
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
pub fn block_0x0020d208(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 14usize, 0u32, 2150924u32);
    emu.adi_no_count(9usize, 13usize, 0u32, 2150928u32);
    emu.lw_no_count(19usize, 8usize, 0u32, 2150932u32)?;
    emu.lbu_no_count(10usize, 8usize, 5u32, 2150936u32);
    emu.lbu_no_count(13usize, 19usize, 10u32, 2150940u32);
    emu.ani_no_count(13usize, 13usize, 128u32, 2150944u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2150976u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d240));
    } else {
        emu.pc = 2150948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d224));
    }
}
#[inline(always)]
pub fn block_0x0020d224(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(13usize, 10usize, 3u32, 2150952u32);
    emu.adi_no_count(22usize, 11usize, 0u32, 2150956u32);
    emu.adi_no_count(23usize, 12usize, 0u32, 2150960u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2151200u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d320));
    } else {
        emu.pc = 2150964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d234));
    }
}
#[inline(always)]
pub fn block_0x0020d234(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2150968u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1284u32, 2150972u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2150976u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2151208u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d328));
}
#[inline(always)]
pub fn block_0x0020d240(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2151036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d27c));
    } else {
        emu.pc = 2150980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d244));
    }
}
#[inline]
pub fn block_0x0020d244(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 19usize, 4u32, 2150984u32)?;
    emu.lw_no_count(10usize, 19usize, 0u32, 2150988u32)?;
    emu.lw_no_count(14usize, 13usize, 12u32, 2150992u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2150996u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1291u32, 2151000u32);
    emu.adi_no_count(20usize, 12usize, 0u32, 2151004u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2151008u32);
    emu.adi_no_count(22usize, 11usize, 0u32, 2151012u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2151016u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2151020u32;
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
pub fn block_0x0020d26c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 22usize, 0u32, 2151024u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2151028u32);
    emu.adi_no_count(20usize, 0usize, 1u32, 2151032u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2150864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d1d0));
    } else {
        emu.pc = 2151036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d27c));
    }
}
#[inline]
pub fn block_0x0020d27c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 1u32, 2151040u32);
    emu.adi_no_count(10usize, 2usize, 27u32, 2151044u32);
    emu.lw_no_count(13usize, 19usize, 0u32, 2151048u32)?;
    emu.lw_no_count(14usize, 19usize, 4u32, 2151052u32)?;
    emu.lw_no_count(15usize, 19usize, 8u32, 2151056u32)?;
    emu.lw_no_count(16usize, 19usize, 12u32, 2151060u32)?;
    emu.adi_no_count(17usize, 2usize, 12u32, 2151064u32);
    emu.sw_no_count(13usize, 2usize, 12u32, 2151068u32)?;
    emu.sw_no_count(14usize, 2usize, 16u32, 2151072u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2151076u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2151080u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1260u32, 2151084u32);
    emu.sb_no_count(20usize, 2usize, 27u32, 2151088u32);
    emu.sw_no_count(17usize, 2usize, 28u32, 2151092u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2151096u32)?;
    emu.sw_no_count(15usize, 2usize, 36u32, 2151100u32)?;
    emu.sw_no_count(16usize, 2usize, 40u32, 2151104u32)?;
    emu.adi_no_count(10usize, 2usize, 12u32, 2151108u32);
    emu.apc_no_count(1usize, 2151108u32, 0u32, 2151112u32);
    emu.add_memory_rw_events(20usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2151116u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966140u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020d2cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2150864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d1d0));
    } else {
        emu.pc = 2151120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d2d0));
    }
}
#[inline(always)]
pub fn block_0x0020d2d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2151124u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1289u32, 2151128u32);
    emu.adi_no_count(10usize, 2usize, 12u32, 2151132u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2151136u32);
    emu.apc_no_count(1usize, 2151136u32, 0u32, 2151140u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2151144u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966112u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020d2e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2150864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d1d0));
    } else {
        emu.pc = 2151148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d2ec));
    }
}
#[inline(always)]
pub fn block_0x0020d2ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 12u32, 2151152u32)?;
    emu.adi_no_count(11usize, 2usize, 28u32, 2151156u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2151160u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2151164u32;
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
pub fn block_0x0020d2fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2150864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d1d0));
    } else {
        emu.pc = 2151168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d300));
    }
}
#[inline(always)]
pub fn block_0x0020d300(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 32u32, 2151172u32)?;
    emu.lw_no_count(10usize, 2usize, 28u32, 2151176u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2151180u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2151184u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1294u32, 2151188u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2151192u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2151196u32;
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
pub fn block_0x0020d31c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2151200u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2151320u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d398));
}
#[inline(always)]
pub fn block_0x0020d320(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2151204u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1287u32, 2151208u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2151208u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d328));
}
#[inline(always)]
pub fn block_0x0020d328(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 19usize, 4u32, 2151212u32)?;
    emu.lw_no_count(10usize, 19usize, 0u32, 2151216u32)?;
    emu.lw_no_count(14usize, 12usize, 12u32, 2151220u32)?;
    emu.adi_no_count(12usize, 13usize, 0u32, 2151224u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2151228u32;
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
pub fn block_0x0020d33c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 1u32, 2151232u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2150864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d1d0));
    } else {
        emu.pc = 2151236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d344));
    }
}
#[inline(always)]
pub fn block_0x0020d344(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 23usize, 0u32, 2151240u32);
    emu.adi_no_count(11usize, 22usize, 0u32, 2151244u32);
    emu.lw_no_count(13usize, 19usize, 4u32, 2151248u32)?;
    emu.lw_no_count(10usize, 19usize, 0u32, 2151252u32)?;
    emu.lw_no_count(13usize, 13usize, 12u32, 2151256u32)?;
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2151260u32;
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
pub fn block_0x0020d35c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 1u32, 2151264u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2150864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d1d0));
    } else {
        emu.pc = 2151268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d364));
    }
}
#[inline(always)]
pub fn block_0x0020d364(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 19usize, 4u32, 2151272u32)?;
    emu.lw_no_count(10usize, 19usize, 0u32, 2151276u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2151280u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2151284u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1289u32, 2151288u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2151292u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2151296u32;
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
pub fn block_0x0020d380(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 1u32, 2151300u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2150864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d1d0));
    } else {
        emu.pc = 2151304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d388));
    }
}
#[inline(always)]
pub fn block_0x0020d388(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 12u32, 2151308u32)?;
    emu.adi_no_count(10usize, 9usize, 0u32, 2151312u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2151316u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2151320u32;
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
pub fn block_0x0020d398(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 10usize, 0u32, 2151324u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2151328u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2150864u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d1d0));
}
