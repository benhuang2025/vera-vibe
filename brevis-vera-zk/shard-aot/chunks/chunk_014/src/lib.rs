pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2154196u32;
pub const PC_MAX: u32 = 2156436u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 107usize] = [
        block_0x0020ded4,
        block_0x0020df10,
        block_0x0020df30,
        block_0x0020df60,
        block_0x0020df68,
        block_0x0020df70,
        block_0x0020df78,
        block_0x0020df8c,
        block_0x0020dfb4,
        block_0x0020dfbc,
        block_0x0020dfc4,
        block_0x0020dfdc,
        block_0x0020dfe4,
        block_0x0020dff0,
        block_0x0020e010,
        block_0x0020e014,
        block_0x0020e044,
        block_0x0020e04c,
        block_0x0020e064,
        block_0x0020e068,
        block_0x0020e078,
        block_0x0020e07c,
        block_0x0020e084,
        block_0x0020e094,
        block_0x0020e098,
        block_0x0020e0a8,
        block_0x0020e0b0,
        block_0x0020e0d4,
        block_0x0020e0dc,
        block_0x0020e0e4,
        block_0x0020e0e8,
        block_0x0020e110,
        block_0x0020e118,
        block_0x0020e138,
        block_0x0020e13c,
        block_0x0020e158,
        block_0x0020e160,
        block_0x0020e17c,
        block_0x0020e184,
        block_0x0020e194,
        block_0x0020e19c,
        block_0x0020e1a4,
        block_0x0020e1b0,
        block_0x0020e1cc,
        block_0x0020e1e4,
        block_0x0020e1f4,
        block_0x0020e204,
        block_0x0020e244,
        block_0x0020e260,
        block_0x0020e27c,
        block_0x0020e288,
        block_0x0020e294,
        block_0x0020e2a0,
        block_0x0020e2a4,
        block_0x0020e2a8,
        block_0x0020e2b4,
        block_0x0020e2bc,
        block_0x0020e2c8,
        block_0x0020e2d0,
        block_0x0020e2ec,
        block_0x0020e320,
        block_0x0020e328,
        block_0x0020e32c,
        block_0x0020e338,
        block_0x0020e340,
        block_0x0020e34c,
        block_0x0020e354,
        block_0x0020e360,
        block_0x0020e36c,
        block_0x0020e384,
        block_0x0020e478,
        block_0x0020e484,
        block_0x0020e490,
        block_0x0020e494,
        block_0x0020e498,
        block_0x0020e4b0,
        block_0x0020e4c4,
        block_0x0020e4d4,
        block_0x0020e4dc,
        block_0x0020e4e4,
        block_0x0020e530,
        block_0x0020e544,
        block_0x0020e554,
        block_0x0020e568,
        block_0x0020e56c,
        block_0x0020e570,
        block_0x0020e574,
        block_0x0020e57c,
        block_0x0020e580,
        block_0x0020e584,
        block_0x0020e58c,
        block_0x0020e59c,
        block_0x0020e5c4,
        block_0x0020e634,
        block_0x0020e644,
        block_0x0020e6b4,
        block_0x0020e6b8,
        block_0x0020e6dc,
        block_0x0020e6ec,
        block_0x0020e700,
        block_0x0020e71c,
        block_0x0020e720,
        block_0x0020e740,
        block_0x0020e754,
        block_0x0020e764,
        block_0x0020e778,
        block_0x0020e794,
    ];
    const IDX: [u16; 561usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 2u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 3u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 4u16, 0u16, 5u16, 0u16,
        6u16, 0u16, 7u16, 0u16, 0u16, 0u16, 0u16, 8u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 9u16, 0u16, 10u16, 0u16, 11u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 12u16, 0u16, 13u16, 0u16, 0u16, 14u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 15u16, 16u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 17u16, 0u16, 18u16, 0u16, 0u16, 0u16, 0u16, 0u16, 19u16, 20u16, 0u16, 0u16,
        0u16, 21u16, 22u16, 0u16, 23u16, 0u16, 0u16, 0u16, 24u16, 25u16, 0u16, 0u16,
        0u16, 26u16, 0u16, 27u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 28u16,
        0u16, 29u16, 0u16, 30u16, 31u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 32u16, 0u16, 33u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 34u16, 35u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 36u16, 0u16, 37u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 38u16, 0u16, 39u16, 0u16, 0u16, 0u16, 40u16, 0u16, 41u16, 0u16,
        42u16, 0u16, 0u16, 43u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 44u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 45u16, 0u16, 0u16, 0u16, 46u16, 0u16, 0u16, 0u16, 47u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 48u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 49u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 50u16, 0u16, 0u16, 51u16, 0u16, 0u16, 52u16, 0u16, 0u16, 53u16,
        54u16, 55u16, 0u16, 0u16, 56u16, 0u16, 57u16, 0u16, 0u16, 58u16, 0u16, 59u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 60u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 61u16, 0u16, 62u16, 63u16, 0u16, 0u16, 64u16,
        0u16, 65u16, 0u16, 0u16, 66u16, 0u16, 67u16, 0u16, 0u16, 68u16, 0u16, 0u16,
        69u16, 0u16, 0u16, 0u16, 0u16, 0u16, 70u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 71u16, 0u16, 0u16, 72u16, 0u16, 0u16, 73u16, 74u16, 75u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 76u16, 0u16, 0u16, 0u16, 0u16, 77u16, 0u16, 0u16, 0u16,
        78u16, 0u16, 79u16, 0u16, 80u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 81u16, 0u16, 0u16,
        0u16, 0u16, 82u16, 0u16, 0u16, 0u16, 83u16, 0u16, 0u16, 0u16, 0u16, 84u16, 85u16,
        86u16, 87u16, 0u16, 88u16, 89u16, 90u16, 0u16, 91u16, 0u16, 0u16, 0u16, 92u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 93u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 94u16, 0u16,
        0u16, 0u16, 95u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 96u16, 97u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 98u16, 0u16, 0u16, 0u16, 99u16, 0u16, 0u16, 0u16, 0u16, 100u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 101u16, 102u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        103u16, 0u16, 0u16, 0u16, 0u16, 104u16, 0u16, 0u16, 0u16, 105u16, 0u16, 0u16,
        0u16, 0u16, 106u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 107u16,
    ];
    if pc < 2154196u32 || pc > 2156436u32 {
        return None;
    }
    let word_offset = ((pc - 2154196u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x0020ded4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967200u32, 2154200u32);
    emu.sw_no_count(1usize, 2usize, 92u32, 2154204u32)?;
    emu.sw_no_count(8usize, 2usize, 88u32, 2154208u32)?;
    emu.sw_no_count(9usize, 2usize, 84u32, 2154212u32)?;
    emu.sw_no_count(18usize, 2usize, 80u32, 2154216u32)?;
    emu.sw_no_count(19usize, 2usize, 76u32, 2154220u32)?;
    emu.sw_no_count(20usize, 2usize, 72u32, 2154224u32)?;
    emu.sw_no_count(21usize, 2usize, 68u32, 2154228u32)?;
    emu.sw_no_count(22usize, 2usize, 64u32, 2154232u32)?;
    emu.sw_no_count(23usize, 2usize, 60u32, 2154236u32)?;
    emu.sw_no_count(24usize, 2usize, 56u32, 2154240u32)?;
    emu.adi_no_count(18usize, 14usize, 0u32, 2154244u32);
    emu.lw_no_count(19usize, 2usize, 96u32, 2154248u32)?;
    emu.adi_no_count(14usize, 0usize, 3u32, 2154252u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a >= b {
        emu.pc = 2155076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e244));
    } else {
        emu.pc = 2154256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020df10));
    }
}
#[inline(always)]
pub fn block_0x0020df10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 17usize, 0u32, 2154260u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2154264u32);
    emu.sli_no_count(6usize, 12usize, 1u32, 2154268u32);
    let a = 0u32.wrapping_add(1048576u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2154272u32;
    emu.update_insn_clock();
    emu.sri_no_count(17usize, 6usize, 21u32, 2154276u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2154280u32);
    emu.anr_no_count(5usize, 12usize, 10usize, 2154284u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2154380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020df8c));
    } else {
        emu.pc = 2154288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020df30));
    }
}
#[inline]
pub fn block_0x0020df30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(14usize, 0usize, 11usize, 2154292u32);
    emu.adi_no_count(7usize, 11usize, 4294967295u32, 2154296u32);
    emu.adr_no_count(28usize, 5usize, 14usize, 2154300u32);
    emu.adi_no_count(14usize, 7usize, 1u32, 2154304u32);
    emu.sltiu_no_count(7usize, 14usize, 1u32, 2154308u32);
    emu.adr_no_count(10usize, 7usize, 10usize, 2154312u32);
    emu.adr_no_count(10usize, 28usize, 10usize, 2154316u32);
    emu.sri_no_count(7usize, 6usize, 1u32, 2154320u32);
    let a = 0u32.wrapping_add(2146435072u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2154324u32;
    emu.update_insn_clock();
    emu.xrr_no_count(7usize, 7usize, 6usize, 2154328u32);
    emu.orr_no_count(7usize, 11usize, 7usize, 2154332u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2154420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dfb4));
    } else {
        emu.pc = 2154336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020df60));
    }
}
#[inline(always)]
pub fn block_0x0020df60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(28usize, 12usize, 6usize, 2154340u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2154428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dfbc));
    } else {
        emu.pc = 2154344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020df68));
    }
}
#[inline(always)]
pub fn block_0x0020df68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(7usize, 14usize, 1u32, 2154348u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a != b {
        emu.pc = 2154436u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dfc4));
    } else {
        emu.pc = 2154352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020df70));
    }
}
#[inline(always)]
pub fn block_0x0020df70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.orr_no_count(11usize, 11usize, 5usize, 2154356u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2154512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e010));
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
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(6usize, 0usize, 0u32, 2154364u32);
    emu.adi_no_count(11usize, 17usize, 4294966221u32, 2154368u32);
    emu.xri_no_count(17usize, 7usize, 1u32, 2154372u32);
    emu.adi_no_count(5usize, 0usize, 1u32, 2154376u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2154380u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2154516u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e014));
}
#[inline]
pub fn block_0x0020df8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 31u32, 2154384u32);
    emu.orr_no_count(10usize, 6usize, 10usize, 2154388u32);
    emu.sli_no_count(14usize, 11usize, 1u32, 2154392u32);
    emu.sli_no_count(10usize, 10usize, 11u32, 2154396u32);
    emu.sri_no_count(10usize, 10usize, 11u32, 2154400u32);
    emu.sri_no_count(7usize, 6usize, 1u32, 2154404u32);
    let a = 0u32.wrapping_add(2146435072u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2154408u32;
    emu.update_insn_clock();
    emu.xrr_no_count(7usize, 7usize, 6usize, 2154412u32);
    emu.orr_no_count(7usize, 11usize, 7usize, 2154416u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a != b {
        emu.pc = 2154336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020df60));
    } else {
        emu.pc = 2154420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dfb4));
    }
}
#[inline(always)]
pub fn block_0x0020dfb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 0usize, 3u32, 2154424u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2154428u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2154516u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e014));
}
#[inline(always)]
pub fn block_0x0020dfbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 0usize, 2u32, 2154432u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2154436u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2154516u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e014));
}
#[inline(always)]
pub fn block_0x0020dfc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(1048576u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2154440u32;
    emu.update_insn_clock();
    emu.xrr_no_count(11usize, 10usize, 11usize, 2154444u32);
    emu.orr_no_count(5usize, 14usize, 11usize, 2154448u32);
    emu.sltiu_no_count(11usize, 5usize, 1u32, 2154452u32);
    emu.sli_no_count(6usize, 14usize, 1u32, 2154456u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a != b {
        emu.pc = 2154468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dfe4));
    } else {
        emu.pc = 2154460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dfdc));
    }
}
#[inline(always)]
pub fn block_0x0020dfdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(4194304u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2154464u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    let return_addr = 2154468u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2154480u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020dff0));
}
#[inline(always)]
pub fn block_0x0020dfe4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(14usize, 14usize, 31u32, 2154472u32);
    emu.sli_no_count(10usize, 10usize, 1u32, 2154476u32);
    emu.orr_no_count(10usize, 10usize, 14usize, 2154480u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2154480u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020dff0));
}
#[inline(always)]
pub fn block_0x0020dff0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 11usize, 4294967295u32, 2154484u32);
    emu.adi_no_count(5usize, 11usize, 1u32, 2154488u32);
    emu.sbr_no_count(11usize, 17usize, 11usize, 2154492u32);
    emu.anr_no_count(14usize, 14usize, 6usize, 2154496u32);
    emu.sltiu_no_count(6usize, 5usize, 1u32, 2154500u32);
    emu.adi_no_count(11usize, 11usize, 4294966220u32, 2154504u32);
    emu.xri_no_count(17usize, 7usize, 1u32, 2154508u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2154512u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2154516u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e014));
}
#[inline(always)]
pub fn block_0x0020e010(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 0usize, 4u32, 2154516u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2154516u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e014));
}
#[inline]
pub fn block_0x0020e014(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(7usize, 17usize, 255u32, 2154520u32);
    emu.sri_no_count(12usize, 12usize, 31u32, 2154524u32);
    emu.adi_no_count(23usize, 0usize, 1u32, 2154528u32);
    emu.sw_no_count(14usize, 2usize, 0u32, 2154532u32)?;
    emu.sw_no_count(10usize, 2usize, 4u32, 2154536u32)?;
    emu.sw_no_count(23usize, 2usize, 8u32, 2154540u32)?;
    emu.sw_no_count(0usize, 2usize, 12u32, 2154544u32)?;
    emu.sw_no_count(5usize, 2usize, 16u32, 2154548u32)?;
    emu.sw_no_count(6usize, 2usize, 20u32, 2154552u32)?;
    emu.sh_no_count(11usize, 2usize, 24u32, 2154556u32)?;
    emu.sb_no_count(17usize, 2usize, 26u32, 2154560u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a >= b {
        emu.pc = 2154596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e064));
    } else {
        emu.pc = 2154564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e044));
    }
}
#[inline(always)]
pub fn block_0x0020e044(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2154568u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a != b {
        emu.pc = 2154620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e07c));
    } else {
        emu.pc = 2154572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e04c));
    }
}
#[inline(always)]
pub fn block_0x0020e04c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(23usize, 0usize, 0u32, 2154576u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2154580u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1562u32, 2154584u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2154588u32);
    emu.adi_no_count(24usize, 0usize, 1u32, 2154592u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2154596u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2154996u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e1f4));
}
#[inline(always)]
pub fn block_0x0020e064(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2154648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e098));
    } else {
        emu.pc = 2154600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e068));
    }
}
#[inline(always)]
pub fn block_0x0020e068(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(24usize, a);
    emu.pc = 2154604u32;
    emu.update_insn_clock();
    emu.adi_no_count(24usize, 24usize, 1561u32, 2154608u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2154612u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2154664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e0a8));
    } else {
        emu.pc = 2154616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e078));
    }
}
#[inline(always)]
pub fn block_0x0020e078(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2154620u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2154672u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e0b0));
}
#[inline(always)]
pub fn block_0x0020e07c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(23usize, 0usize, 1u32, 2154624u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2154884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e184));
    } else {
        emu.pc = 2154628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e084));
    }
}
#[inline(always)]
pub fn block_0x0020e084(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(24usize, a);
    emu.pc = 2154632u32;
    emu.update_insn_clock();
    emu.adi_no_count(24usize, 24usize, 1561u32, 2154636u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2154640u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2154900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e194));
    } else {
        emu.pc = 2154644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e094));
    }
}
#[inline(always)]
pub fn block_0x0020e094(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2154648u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2154908u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e19c));
}
#[inline(always)]
pub fn block_0x0020e098(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(24usize, a);
    emu.pc = 2154652u32;
    emu.update_insn_clock();
    emu.adi_no_count(24usize, 24usize, 1560u32, 2154656u32);
    emu.adi_no_count(10usize, 24usize, 0u32, 2154660u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2154672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e0b0));
    } else {
        emu.pc = 2154664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e0a8));
    }
}
#[inline(always)]
pub fn block_0x0020e0a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(23usize, 12usize, 0u32, 2154668u32);
    emu.adi_no_count(24usize, 10usize, 0u32, 2154672u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2154672u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e0b0));
}
#[inline]
pub fn block_0x0020e0b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 11usize, 16u32, 2154676u32);
    emu.sai_no_count(10usize, 11usize, 1055u32, 2154680u32);
    emu.ani_no_count(10usize, 10usize, 4294967279u32, 2154684u32);
    emu.adi_no_count(10usize, 10usize, 5u32, 2154688u32);
    emu.sai_no_count(11usize, 11usize, 1040u32, 2154692u32);
    emu.mul_no_count(10usize, 10usize, 11usize, 2154696u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2154700u32);
    emu.adi_no_count(21usize, 10usize, 21u32, 2154704u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a < b {
        emu.pc = 2155104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e260));
    } else {
        emu.pc = 2154708u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e0d4));
    }
}
#[inline(always)]
pub fn block_0x0020e0d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 18usize, 15u32, 2154712u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2154724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e0e4));
    } else {
        emu.pc = 2154716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e0dc));
    }
}
#[inline(always)]
pub fn block_0x0020e0dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(4294934528u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2154720u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    let return_addr = 2154724u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2154728u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e0e8));
}
#[inline(always)]
pub fn block_0x0020e0e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 0usize, 18usize, 2154728u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2154728u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e0e8));
}
#[inline]
pub fn block_0x0020e0e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 10usize, 16u32, 2154732u32);
    emu.sai_no_count(20usize, 10usize, 1040u32, 2154736u32);
    emu.adi_no_count(10usize, 2usize, 44u32, 2154740u32);
    emu.adi_no_count(11usize, 2usize, 0u32, 2154744u32);
    emu.adi_no_count(22usize, 15usize, 0u32, 2154748u32);
    emu.adi_no_count(12usize, 15usize, 0u32, 2154752u32);
    emu.adi_no_count(13usize, 21usize, 0u32, 2154756u32);
    emu.adi_no_count(14usize, 20usize, 0u32, 2154760u32);
    emu.apc_no_count(1usize, 2154760u32, 4096u32, 2154764u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2154768u32;
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
pub fn block_0x0020e110(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 44u32, 2154772u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2154812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e13c));
    } else {
        emu.pc = 2154776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e118));
    }
}
#[inline(always)]
pub fn block_0x0020e118(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 44u32, 2154780u32)?;
    emu.lw_no_count(11usize, 2usize, 48u32, 2154784u32)?;
    emu.lw_no_count(12usize, 2usize, 52u32, 2154788u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2154792u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2154796u32)?;
    emu.sw_no_count(12usize, 2usize, 40u32, 2154800u32)?;
    emu.lh_no_count(12usize, 2usize, 40u32, 2154804u32)?;
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(20usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2154848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e160));
    } else {
        emu.pc = 2154808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e138));
    }
}
#[inline(always)]
pub fn block_0x0020e138(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2154812u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2154916u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e1a4));
}
#[inline(always)]
pub fn block_0x0020e13c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 32u32, 2154816u32);
    emu.adi_no_count(11usize, 2usize, 0u32, 2154820u32);
    emu.adi_no_count(12usize, 22usize, 0u32, 2154824u32);
    emu.adi_no_count(13usize, 21usize, 0u32, 2154828u32);
    emu.adi_no_count(14usize, 20usize, 0u32, 2154832u32);
    emu.apc_no_count(1usize, 2154832u32, 20480u32, 2154836u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2154840u32;
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
pub fn block_0x0020e158(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lh_no_count(12usize, 2usize, 40u32, 2154844u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(20usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2154916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e1a4));
    } else {
        emu.pc = 2154848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e160));
    }
}
#[inline(always)]
pub fn block_0x0020e160(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 32u32, 2154852u32)?;
    emu.lw_no_count(11usize, 2usize, 36u32, 2154856u32)?;
    emu.adi_no_count(13usize, 18usize, 0u32, 2154860u32);
    emu.adi_no_count(14usize, 8usize, 0u32, 2154864u32);
    emu.adi_no_count(15usize, 19usize, 0u32, 2154868u32);
    emu.apc_no_count(1usize, 2154868u32, 0u32, 2154872u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2154876u32;
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
pub fn block_0x0020e17c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 10usize, 0u32, 2154880u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2154884u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2155012u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e204));
}
#[inline(always)]
pub fn block_0x0020e184(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(24usize, a);
    emu.pc = 2154888u32;
    emu.update_insn_clock();
    emu.adi_no_count(24usize, 24usize, 1560u32, 2154892u32);
    emu.adi_no_count(10usize, 24usize, 0u32, 2154896u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2154908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e19c));
    } else {
        emu.pc = 2154900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e194));
    }
}
#[inline(always)]
pub fn block_0x0020e194(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(23usize, 12usize, 0u32, 2154904u32);
    emu.adi_no_count(24usize, 10usize, 0u32, 2154908u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2154908u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e19c));
}
#[inline(always)]
pub fn block_0x0020e19c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2154912u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a != b {
        emu.pc = 2154980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e1e4));
    } else {
        emu.pc = 2154916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e1a4));
    }
}
#[inline(always)]
pub fn block_0x0020e1a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 2u32, 2154920u32);
    emu.sh_no_count(11usize, 8usize, 0u32, 2154924u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2154956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e1cc));
    } else {
        emu.pc = 2154928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e1b0));
    }
}
#[inline(always)]
pub fn block_0x0020e1b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2154932u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1477u32, 2154936u32);
    emu.sw_no_count(10usize, 8usize, 4u32, 2154940u32)?;
    emu.sw_no_count(11usize, 8usize, 8u32, 2154944u32)?;
    emu.sh_no_count(0usize, 8usize, 12u32, 2154948u32)?;
    emu.sw_no_count(18usize, 8usize, 16u32, 2154952u32)?;
    emu.add_memory_rw_events(7usize);
    let return_addr = 2154956u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2155012u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e204));
}
#[inline(always)]
pub fn block_0x0020e1cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2154960u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1568u32, 2154964u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2154968u32);
    emu.sw_no_count(10usize, 8usize, 4u32, 2154972u32)?;
    emu.sw_no_count(11usize, 8usize, 8u32, 2154976u32)?;
    emu.add_memory_rw_events(6usize);
    let return_addr = 2154980u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2155012u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e204));
}
#[inline(always)]
pub fn block_0x0020e1e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2154984u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2154988u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1565u32, 2154992u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2154996u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2154996u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e1f4));
}
#[inline(always)]
pub fn block_0x0020e1f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sh_no_count(10usize, 8usize, 0u32, 2155000u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2155004u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2155008u32)?;
    emu.adi_no_count(11usize, 0usize, 1u32, 2155012u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2155012u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e204));
}
#[inline]
pub fn block_0x0020e204(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(24usize, 9usize, 0u32, 2155016u32)?;
    emu.sw_no_count(23usize, 9usize, 4u32, 2155020u32)?;
    emu.sw_no_count(8usize, 9usize, 8u32, 2155024u32)?;
    emu.sw_no_count(11usize, 9usize, 12u32, 2155028u32)?;
    emu.lw_no_count(1usize, 2usize, 92u32, 2155032u32)?;
    emu.lw_no_count(8usize, 2usize, 88u32, 2155036u32)?;
    emu.lw_no_count(9usize, 2usize, 84u32, 2155040u32)?;
    emu.lw_no_count(18usize, 2usize, 80u32, 2155044u32)?;
    emu.lw_no_count(19usize, 2usize, 76u32, 2155048u32)?;
    emu.lw_no_count(20usize, 2usize, 72u32, 2155052u32)?;
    emu.lw_no_count(21usize, 2usize, 68u32, 2155056u32)?;
    emu.lw_no_count(22usize, 2usize, 64u32, 2155060u32)?;
    emu.lw_no_count(23usize, 2usize, 60u32, 2155064u32)?;
    emu.lw_no_count(24usize, 2usize, 56u32, 2155068u32)?;
    emu.adi_no_count(2usize, 2usize, 96u32, 2155072u32);
    emu.add_memory_rw_events(16usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2155076u32;
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
pub fn block_0x0020e244(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2155080u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1424u32, 2155084u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2155088u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1572u32, 2155092u32);
    emu.adi_no_count(11usize, 0usize, 34u32, 2155096u32);
    emu.apc_no_count(1usize, 2155096u32, 4294959104u32, 2155100u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2155104u32;
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
pub fn block_0x0020e260(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2155108u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1588u32, 2155112u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2155116u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1628u32, 2155120u32);
    emu.adi_no_count(11usize, 0usize, 37u32, 2155124u32);
    emu.apc_no_count(1usize, 2155124u32, 4294959104u32, 2155128u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2155132u32;
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
pub fn block_0x0020e27c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 11usize, 3u32, 2155136u32);
    emu.ani_no_count(13usize, 13usize, 4294967292u32, 2155140u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2155156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e294));
    } else {
        emu.pc = 2155144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e288));
    }
}
#[inline(always)]
pub fn block_0x0020e288(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 0u32, 2155148u32);
    emu.adi_no_count(14usize, 12usize, 4294967288u32, 2155152u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2155156u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2155216u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e2d0));
}
#[inline(always)]
pub fn block_0x0020e294(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(14usize, 13usize, 11usize, 2155160u32);
    emu.adi_no_count(13usize, 12usize, 0u32, 2155164u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2155172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e2a4));
    } else {
        emu.pc = 2155168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e2a0));
    }
}
#[inline(always)]
pub fn block_0x0020e2a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 14usize, 0u32, 2155172u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2155172u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e2a4));
}
#[inline(always)]
pub fn block_0x0020e2a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2155208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e2c8));
    } else {
        emu.pc = 2155176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e2a8));
    }
}
#[inline(always)]
pub fn block_0x0020e2a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 0u32, 2155180u32);
    emu.sbr_no_count(15usize, 0usize, 13usize, 2155184u32);
    emu.adi_no_count(16usize, 11usize, 0u32, 2155188u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2155188u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e2b4));
}
#[inline(always)]
pub fn block_0x0020e2b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(17usize, 16usize, 0u32, 2155192u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2155360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e360));
    } else {
        emu.pc = 2155196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e2bc));
    }
}
#[inline(always)]
pub fn block_0x0020e2bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2155200u32);
    emu.adi_no_count(16usize, 16usize, 1u32, 2155204u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2155188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e2b4));
    } else {
        emu.pc = 2155208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e2c8));
    }
}
#[inline(always)]
pub fn block_0x0020e2c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 12usize, 4294967288u32, 2155212u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2155304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e328));
    } else {
        emu.pc = 2155216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e2d0));
    }
}
#[inline(always)]
pub fn block_0x0020e2d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2155220u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 11usize, 4u32, 2155224u32);
    let a = 0u32.wrapping_add(2155905024u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2155228u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 256u32, 2155232u32);
    emu.adi_no_count(17usize, 16usize, 1u32, 2155236u32);
    emu.mul_no_count(17usize, 10usize, 17usize, 2155240u32);
    emu.adi_no_count(5usize, 5usize, 128u32, 2155244u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2155244u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e2ec));
}
#[inline]
pub fn block_0x0020e2ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(6usize, 11usize, 13usize, 2155248u32);
    emu.adr_no_count(7usize, 15usize, 13usize, 2155252u32);
    emu.lw_no_count(6usize, 6usize, 0u32, 2155256u32)?;
    emu.lw_no_count(7usize, 7usize, 0u32, 2155260u32)?;
    emu.xrr_no_count(6usize, 6usize, 17usize, 2155264u32);
    emu.xrr_no_count(7usize, 7usize, 17usize, 2155268u32);
    emu.sbr_no_count(28usize, 16usize, 6usize, 2155272u32);
    emu.orr_no_count(6usize, 28usize, 6usize, 2155276u32);
    emu.sbr_no_count(28usize, 16usize, 7usize, 2155280u32);
    emu.orr_no_count(7usize, 28usize, 7usize, 2155284u32);
    emu.anr_no_count(6usize, 6usize, 7usize, 2155288u32);
    emu.anr_no_count(6usize, 6usize, 5usize, 2155292u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a != b {
        emu.pc = 2155304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e328));
    } else {
        emu.pc = 2155296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e320));
    }
}
#[inline(always)]
pub fn block_0x0020e320(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 8u32, 2155300u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a >= b {
        emu.pc = 2155244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e2ec));
    } else {
        emu.pc = 2155304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e328));
    }
}
#[inline(always)]
pub fn block_0x0020e328(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2155340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e34c));
    } else {
        emu.pc = 2155308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e32c));
    }
}
#[inline(always)]
pub fn block_0x0020e32c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(14usize, 11usize, 13usize, 2155312u32);
    emu.sbr_no_count(11usize, 0usize, 13usize, 2155316u32);
    emu.sbr_no_count(12usize, 0usize, 12usize, 2155320u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2155320u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e338));
}
#[inline(always)]
pub fn block_0x0020e338(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(13usize, 14usize, 0u32, 2155324u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2155348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e354));
    } else {
        emu.pc = 2155328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e340));
    }
}
#[inline(always)]
pub fn block_0x0020e340(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2155332u32);
    emu.adi_no_count(14usize, 14usize, 1u32, 2155336u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2155320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e338));
    } else {
        emu.pc = 2155340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e34c));
    }
}
#[inline(always)]
pub fn block_0x0020e34c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2155344u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2155348u32;
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
pub fn block_0x0020e354(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 0usize, 11usize, 2155352u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2155356u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2155360u32;
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
pub fn block_0x0020e360(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 0usize, 14usize, 2155364u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2155368u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2155372u32;
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
pub fn block_0x0020e36c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2155376u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2155380u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965595u32, 2155384u32);
    emu.adi_no_count(11usize, 0usize, 43u32, 2155388u32);
    emu.apc_no_count(1usize, 2155388u32, 4294959104u32, 2155392u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2155396u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(548u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0020e384(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 61u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(73728u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2155400u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2155404u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294965640u32, 2155408u32);
    emu.adi_no_count(11usize, 11usize, 4294965295u32, 2155412u32);
    emu.sltru_no_count(11usize, 10usize, 11usize, 2155416u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2155420u32);
    emu.ani_no_count(11usize, 11usize, 17u32, 2155424u32);
    emu.sli_no_count(12usize, 11usize, 2u32, 2155428u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2155432u32);
    emu.lw_no_count(12usize, 12usize, 32u32, 2155436u32)?;
    emu.sli_no_count(14usize, 10usize, 11u32, 2155440u32);
    emu.sli_no_count(12usize, 12usize, 11u32, 2155444u32);
    emu.sltru_no_count(12usize, 14usize, 12usize, 2155448u32);
    emu.xri_no_count(12usize, 12usize, 1u32, 2155452u32);
    emu.sli_no_count(12usize, 12usize, 3u32, 2155456u32);
    emu.orr_no_count(11usize, 11usize, 12usize, 2155460u32);
    emu.sli_no_count(12usize, 11usize, 2u32, 2155464u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2155468u32);
    emu.lw_no_count(12usize, 12usize, 16u32, 2155472u32)?;
    emu.sli_no_count(12usize, 12usize, 11u32, 2155476u32);
    emu.sltru_no_count(12usize, 14usize, 12usize, 2155480u32);
    emu.xri_no_count(12usize, 12usize, 1u32, 2155484u32);
    emu.sli_no_count(12usize, 12usize, 2u32, 2155488u32);
    emu.orr_no_count(11usize, 11usize, 12usize, 2155492u32);
    emu.sli_no_count(12usize, 11usize, 2u32, 2155496u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2155500u32);
    emu.lw_no_count(12usize, 12usize, 8u32, 2155504u32)?;
    emu.sli_no_count(12usize, 12usize, 11u32, 2155508u32);
    emu.sltru_no_count(12usize, 14usize, 12usize, 2155512u32);
    emu.xri_no_count(12usize, 12usize, 1u32, 2155516u32);
    emu.sli_no_count(12usize, 12usize, 1u32, 2155520u32);
    emu.orr_no_count(11usize, 11usize, 12usize, 2155524u32);
    emu.sli_no_count(12usize, 11usize, 2u32, 2155528u32);
    emu.adr_no_count(12usize, 12usize, 13usize, 2155532u32);
    emu.lw_no_count(12usize, 12usize, 4u32, 2155536u32)?;
    emu.sli_no_count(12usize, 12usize, 11u32, 2155540u32);
    emu.sltru_no_count(12usize, 14usize, 12usize, 2155544u32);
    emu.xri_no_count(12usize, 12usize, 1u32, 2155548u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2155552u32);
    emu.sli_no_count(12usize, 11usize, 2u32, 2155556u32);
    emu.adr_no_count(12usize, 12usize, 13usize, 2155560u32);
    emu.lw_no_count(12usize, 12usize, 4u32, 2155564u32)?;
    emu.sli_no_count(12usize, 12usize, 11u32, 2155568u32);
    emu.sltru_no_count(12usize, 14usize, 12usize, 2155572u32);
    emu.xri_no_count(12usize, 12usize, 1u32, 2155576u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2155580u32);
    emu.sli_no_count(12usize, 11usize, 2u32, 2155584u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2155588u32);
    emu.lw_no_count(12usize, 12usize, 0u32, 2155592u32)?;
    emu.sli_no_count(12usize, 12usize, 11u32, 2155596u32);
    emu.xrr_no_count(15usize, 12usize, 14usize, 2155600u32);
    emu.sltru_no_count(12usize, 12usize, 14usize, 2155604u32);
    emu.sltiu_no_count(14usize, 15usize, 1u32, 2155608u32);
    emu.adr_no_count(11usize, 12usize, 11usize, 2155612u32);
    emu.adr_no_count(14usize, 14usize, 11usize, 2155616u32);
    emu.sli_no_count(11usize, 14usize, 2u32, 2155620u32);
    emu.adr_no_count(13usize, 13usize, 11usize, 2155624u32);
    emu.lw_no_count(11usize, 13usize, 0u32, 2155628u32)?;
    emu.adi_no_count(12usize, 0usize, 32u32, 2155632u32);
    emu.sri_no_count(11usize, 11usize, 21u32, 2155636u32);
    emu.add_memory_rw_events(60usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2155668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e494));
    } else {
        emu.pc = 2155640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e478));
    }
}
#[inline(always)]
pub fn block_0x0020e478(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 13usize, 4u32, 2155644u32)?;
    emu.sri_no_count(12usize, 12usize, 21u32, 2155648u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a != b {
        emu.pc = 2155672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e498));
    } else {
        emu.pc = 2155652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e484));
    }
}
#[inline(always)]
pub fn block_0x0020e484(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(13usize, 11usize, 4294967295u32, 2155656u32);
    emu.adr_no_count(13usize, 12usize, 13usize, 2155660u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2155696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e4b0));
    } else {
        emu.pc = 2155664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e490));
    }
}
#[inline(always)]
pub fn block_0x0020e490(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2155668u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2155740u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e4dc));
}
#[inline(always)]
pub fn block_0x0020e494(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 751u32, 2155672u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2155672u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e498));
}
#[inline(always)]
pub fn block_0x0020e498(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 13usize, 4294967292u32, 2155676u32)?;
    emu.sli_no_count(13usize, 13usize, 11u32, 2155680u32);
    emu.sri_no_count(14usize, 13usize, 11u32, 2155684u32);
    emu.xri_no_count(13usize, 11usize, 4294967295u32, 2155688u32);
    emu.adr_no_count(13usize, 12usize, 13usize, 2155692u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2155740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e4dc));
    } else {
        emu.pc = 2155696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e4b0));
    }
}
#[inline(always)]
pub fn block_0x0020e4b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 0u32, 2155700u32);
    emu.sbr_no_count(10usize, 10usize, 14usize, 2155704u32);
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2155708u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2155712u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1644u32, 2155716u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2155716u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e4c4));
}
#[inline(always)]
pub fn block_0x0020e4c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 14usize, 11usize, 2155720u32);
    emu.lbu_no_count(15usize, 15usize, 0u32, 2155724u32);
    emu.adr_no_count(13usize, 13usize, 15usize, 2155728u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2155740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e4dc));
    } else {
        emu.pc = 2155732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e4d4));
    }
}
#[inline(always)]
pub fn block_0x0020e4d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 11usize, 1u32, 2155736u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2155716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e4c4));
    } else {
        emu.pc = 2155740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e4dc));
    }
}
#[inline(always)]
pub fn block_0x0020e4dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 11usize, 1u32, 2155744u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2155748u32;
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
pub fn block_0x0020e4e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967168u32, 2155752u32);
    emu.sw_no_count(1usize, 2usize, 124u32, 2155756u32)?;
    emu.sw_no_count(8usize, 2usize, 120u32, 2155760u32)?;
    emu.sw_no_count(9usize, 2usize, 116u32, 2155764u32)?;
    emu.sw_no_count(18usize, 2usize, 112u32, 2155768u32)?;
    emu.sw_no_count(19usize, 2usize, 108u32, 2155772u32)?;
    emu.sw_no_count(20usize, 2usize, 104u32, 2155776u32)?;
    emu.sw_no_count(21usize, 2usize, 100u32, 2155780u32)?;
    emu.sw_no_count(22usize, 2usize, 96u32, 2155784u32)?;
    emu.sw_no_count(23usize, 2usize, 92u32, 2155788u32)?;
    emu.sw_no_count(24usize, 2usize, 88u32, 2155792u32)?;
    emu.sw_no_count(25usize, 2usize, 84u32, 2155796u32)?;
    emu.sw_no_count(26usize, 2usize, 80u32, 2155800u32)?;
    emu.sw_no_count(27usize, 2usize, 76u32, 2155804u32)?;
    emu.adi_no_count(26usize, 12usize, 0u32, 2155808u32);
    emu.lw_no_count(14usize, 11usize, 0u32, 2155812u32)?;
    emu.lw_no_count(16usize, 11usize, 4u32, 2155816u32)?;
    emu.orr_no_count(12usize, 14usize, 16usize, 2155820u32);
    emu.add_memory_rw_events(18usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2158932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2158932u32));
    } else {
        emu.pc = 2155824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e530));
    }
}
#[inline(always)]
pub fn block_0x0020e530(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2155828u32);
    emu.lw_no_count(17usize, 11usize, 8u32, 2155832u32)?;
    emu.lw_no_count(6usize, 11usize, 12u32, 2155836u32)?;
    emu.orr_no_count(10usize, 17usize, 6usize, 2155840u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2158960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2158960u32));
    } else {
        emu.pc = 2155844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e544));
    }
}
#[inline(always)]
pub fn block_0x0020e544(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 11usize, 16u32, 2155848u32)?;
    emu.lw_no_count(5usize, 11usize, 20u32, 2155852u32)?;
    emu.orr_no_count(12usize, 10usize, 5usize, 2155856u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2158988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2158988u32));
    } else {
        emu.pc = 2155860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e554));
    }
}
#[inline(always)]
pub fn block_0x0020e554(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 14usize, 10usize, 2155864u32);
    emu.sltru_no_count(7usize, 15usize, 14usize, 2155868u32);
    emu.adr_no_count(5usize, 16usize, 5usize, 2155872u32);
    emu.adr_no_count(12usize, 5usize, 7usize, 2155876u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2155884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e56c));
    } else {
        emu.pc = 2155880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e568));
    }
}
#[inline(always)]
pub fn block_0x0020e568(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 12usize, 16usize, 2155884u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2155884u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e56c));
}
#[inline(always)]
pub fn block_0x0020e56c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a != b {
        emu.pc = 2159016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2159016u32));
    } else {
        emu.pc = 2155888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e570));
    }
}
#[inline(always)]
pub fn block_0x0020e570(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2155900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e57c));
    } else {
        emu.pc = 2155892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e574));
    }
}
#[inline(always)]
pub fn block_0x0020e574(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 16usize, 6usize, 2155896u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2155900u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2155904u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e580));
}
#[inline(always)]
pub fn block_0x0020e57c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 14usize, 17usize, 2155904u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2155904u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e580));
}
#[inline(always)]
pub fn block_0x0020e580(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a != b {
        emu.pc = 2159044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2159044u32));
    } else {
        emu.pc = 2155908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e584));
    }
}
#[inline(always)]
pub fn block_0x0020e584(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 16u32, 2155912u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a >= b {
        emu.pc = 2159072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2159072u32));
    } else {
        emu.pc = 2155916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e58c));
    }
}
#[inline(always)]
pub fn block_0x0020e58c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 15usize, 10usize, 2155920u32);
    emu.adr_no_count(10usize, 5usize, 10usize, 2155924u32);
    emu.sri_no_count(12usize, 10usize, 29u32, 2155928u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2159100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2159100u32));
    } else {
        emu.pc = 2155932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e59c));
    }
}
#[inline]
pub fn block_0x0020e59c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(5usize, 15usize, 1u32, 2155936u32);
    let a = 0u32.wrapping_add(1431654400u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2155940u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(858992640u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2155944u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(252645376u32);
    emu.write_reg_no_count(28usize, a);
    emu.pc = 2155948u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(31usize, a);
    emu.pc = 2155952u32;
    emu.update_insn_clock();
    emu.adi_no_count(30usize, 12usize, 1365u32, 2155956u32);
    emu.adi_no_count(29usize, 7usize, 819u32, 2155960u32);
    emu.adi_no_count(28usize, 28usize, 4294967055u32, 2155964u32);
    emu.adi_no_count(7usize, 31usize, 257u32, 2155968u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2156100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e644));
    } else {
        emu.pc = 2155972u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e5c4));
    }
}
#[inline(never)]
pub fn block_0x0020e5c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.orr_no_count(12usize, 15usize, 5usize, 2155976u32);
    emu.sri_no_count(31usize, 12usize, 2u32, 2155980u32);
    emu.orr_no_count(12usize, 12usize, 31usize, 2155984u32);
    emu.sri_no_count(31usize, 12usize, 4u32, 2155988u32);
    emu.orr_no_count(12usize, 12usize, 31usize, 2155992u32);
    emu.sri_no_count(31usize, 12usize, 8u32, 2155996u32);
    emu.orr_no_count(12usize, 12usize, 31usize, 2156000u32);
    emu.sri_no_count(31usize, 12usize, 16u32, 2156004u32);
    emu.orr_no_count(12usize, 12usize, 31usize, 2156008u32);
    emu.xri_no_count(12usize, 12usize, 4294967295u32, 2156012u32);
    emu.sri_no_count(31usize, 12usize, 1u32, 2156016u32);
    emu.anr_no_count(30usize, 31usize, 30usize, 2156020u32);
    emu.sbr_no_count(12usize, 12usize, 30usize, 2156024u32);
    emu.anr_no_count(30usize, 12usize, 29usize, 2156028u32);
    emu.sri_no_count(12usize, 12usize, 2u32, 2156032u32);
    emu.anr_no_count(12usize, 12usize, 29usize, 2156036u32);
    emu.adr_no_count(12usize, 30usize, 12usize, 2156040u32);
    emu.sri_no_count(29usize, 12usize, 4u32, 2156044u32);
    emu.adr_no_count(12usize, 12usize, 29usize, 2156048u32);
    emu.anr_no_count(12usize, 12usize, 28usize, 2156052u32);
    emu.mul_no_count(12usize, 12usize, 7usize, 2156056u32);
    emu.sri_no_count(12usize, 12usize, 24u32, 2156060u32);
    emu.adi_no_count(7usize, 12usize, 32u32, 2156064u32);
    emu.lhu_no_count(30usize, 11usize, 24u32, 2156068u32)?;
    emu.ani_no_count(31usize, 7usize, 63u32, 2156072u32);
    emu.adi_no_count(9usize, 31usize, 4294967264u32, 2156076u32);
    emu.xri_no_count(8usize, 31usize, 4294967295u32, 2156080u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2156212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e6b4));
    } else {
        emu.pc = 2156084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e634));
    }
}
#[inline(always)]
pub fn block_0x0020e634(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.slr_no_count(10usize, 10usize, 7usize, 2156088u32);
    emu.srr_no_count(11usize, 5usize, 8usize, 2156092u32);
    emu.orr_no_count(11usize, 10usize, 11usize, 2156096u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2156100u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156216u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e6b8));
}
#[inline(never)]
pub fn block_0x0020e644(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 10usize, 1u32, 2156104u32);
    emu.orr_no_count(12usize, 10usize, 12usize, 2156108u32);
    emu.sri_no_count(31usize, 12usize, 2u32, 2156112u32);
    emu.orr_no_count(12usize, 12usize, 31usize, 2156116u32);
    emu.sri_no_count(31usize, 12usize, 4u32, 2156120u32);
    emu.orr_no_count(12usize, 12usize, 31usize, 2156124u32);
    emu.sri_no_count(31usize, 12usize, 8u32, 2156128u32);
    emu.orr_no_count(12usize, 12usize, 31usize, 2156132u32);
    emu.sri_no_count(31usize, 12usize, 16u32, 2156136u32);
    emu.orr_no_count(12usize, 12usize, 31usize, 2156140u32);
    emu.xri_no_count(12usize, 12usize, 4294967295u32, 2156144u32);
    emu.sri_no_count(31usize, 12usize, 1u32, 2156148u32);
    emu.anr_no_count(30usize, 31usize, 30usize, 2156152u32);
    emu.sbr_no_count(12usize, 12usize, 30usize, 2156156u32);
    emu.anr_no_count(30usize, 12usize, 29usize, 2156160u32);
    emu.sri_no_count(12usize, 12usize, 2u32, 2156164u32);
    emu.anr_no_count(12usize, 12usize, 29usize, 2156168u32);
    emu.adr_no_count(12usize, 30usize, 12usize, 2156172u32);
    emu.sri_no_count(29usize, 12usize, 4u32, 2156176u32);
    emu.adr_no_count(12usize, 12usize, 29usize, 2156180u32);
    emu.anr_no_count(12usize, 12usize, 28usize, 2156184u32);
    emu.mul_no_count(12usize, 12usize, 7usize, 2156188u32);
    emu.sri_no_count(7usize, 12usize, 24u32, 2156192u32);
    emu.lhu_no_count(30usize, 11usize, 24u32, 2156196u32)?;
    emu.ani_no_count(31usize, 7usize, 63u32, 2156200u32);
    emu.adi_no_count(9usize, 31usize, 4294967264u32, 2156204u32);
    emu.xri_no_count(8usize, 31usize, 4294967295u32, 2156208u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2156084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e634));
    } else {
        emu.pc = 2156212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e6b4));
    }
}
#[inline(always)]
pub fn block_0x0020e6b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.slr_no_count(11usize, 15usize, 31usize, 2156216u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2156216u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e6b8));
}
#[inline]
pub fn block_0x0020e6b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sai_no_count(5usize, 9usize, 1055u32, 2156220u32);
    emu.sltru_no_count(10usize, 14usize, 17usize, 2156224u32);
    emu.sbr_no_count(12usize, 16usize, 6usize, 2156228u32);
    emu.sbr_no_count(28usize, 14usize, 17usize, 2156232u32);
    emu.sbr_no_count(10usize, 12usize, 10usize, 2156236u32);
    emu.sw_no_count(28usize, 2usize, 24u32, 2156240u32)?;
    emu.sw_no_count(10usize, 2usize, 28u32, 2156244u32)?;
    emu.sh_no_count(30usize, 2usize, 32u32, 2156248u32)?;
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2156288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e700));
    } else {
        emu.pc = 2156252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e6dc));
    }
}
#[inline(always)]
pub fn block_0x0020e6dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.slr_no_count(17usize, 28usize, 31usize, 2156256u32);
    emu.slr_no_count(12usize, 28usize, 7usize, 2156260u32);
    emu.anr_no_count(6usize, 5usize, 12usize, 2156264u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2156316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e71c));
    } else {
        emu.pc = 2156268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e6ec));
    }
}
#[inline(always)]
pub fn block_0x0020e6ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.srr_no_count(12usize, 6usize, 7usize, 2156272u32);
    emu.sli_no_count(29usize, 17usize, 1u32, 2156276u32);
    emu.slr_no_count(29usize, 29usize, 8usize, 2156280u32);
    emu.orr_no_count(29usize, 12usize, 29usize, 2156284u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2156288u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156320u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e720));
}
#[inline(always)]
pub fn block_0x0020e700(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.slr_no_count(12usize, 10usize, 7usize, 2156292u32);
    emu.sri_no_count(17usize, 28usize, 1u32, 2156296u32);
    emu.srr_no_count(17usize, 17usize, 8usize, 2156300u32);
    emu.orr_no_count(17usize, 12usize, 17usize, 2156304u32);
    emu.slr_no_count(12usize, 28usize, 7usize, 2156308u32);
    emu.anr_no_count(6usize, 5usize, 12usize, 2156312u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2156268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e6ec));
    } else {
        emu.pc = 2156316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e71c));
    }
}
#[inline(always)]
pub fn block_0x0020e71c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.srr_no_count(29usize, 17usize, 31usize, 2156320u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2156320u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e720));
}
#[inline(always)]
pub fn block_0x0020e720(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.srr_no_count(12usize, 17usize, 7usize, 2156324u32);
    emu.xrr_no_count(28usize, 29usize, 28usize, 2156328u32);
    emu.anr_no_count(12usize, 5usize, 12usize, 2156332u32);
    emu.xrr_no_count(10usize, 12usize, 10usize, 2156336u32);
    emu.orr_no_count(10usize, 28usize, 10usize, 2156340u32);
    emu.sw_no_count(29usize, 2usize, 40u32, 2156344u32)?;
    emu.sw_no_count(12usize, 2usize, 44u32, 2156348u32)?;
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2158896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2158896u32));
    } else {
        emu.pc = 2156352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e740));
    }
}
#[inline(always)]
pub fn block_0x0020e740(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(14usize, 2usize, 24u32, 2156356u32)?;
    emu.sw_no_count(16usize, 2usize, 28u32, 2156360u32)?;
    emu.sh_no_count(30usize, 2usize, 32u32, 2156364u32)?;
    emu.slr_no_count(10usize, 14usize, 31usize, 2156368u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2156408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e778));
    } else {
        emu.pc = 2156372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e754));
    }
}
#[inline(always)]
pub fn block_0x0020e754(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(28usize, 10usize, 0u32, 2156376u32);
    emu.anr_no_count(29usize, 5usize, 10usize, 2156380u32);
    emu.srr_no_count(10usize, 10usize, 31usize, 2156384u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2156436u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e794));
    } else {
        emu.pc = 2156388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e764));
    }
}
#[inline(always)]
pub fn block_0x0020e764(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 28usize, 1u32, 2156392u32);
    emu.slr_no_count(12usize, 12usize, 8usize, 2156396u32);
    emu.srr_no_count(31usize, 29usize, 31usize, 2156400u32);
    emu.orr_no_count(31usize, 31usize, 12usize, 2156404u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2156408u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156440u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(2156440u32));
}
#[inline(always)]
pub fn block_0x0020e778(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 14usize, 1u32, 2156412u32);
    emu.srr_no_count(12usize, 12usize, 8usize, 2156416u32);
    emu.slr_no_count(28usize, 16usize, 31usize, 2156420u32);
    emu.orr_no_count(28usize, 28usize, 12usize, 2156424u32);
    emu.anr_no_count(29usize, 5usize, 10usize, 2156428u32);
    emu.srr_no_count(10usize, 28usize, 31usize, 2156432u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2156388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e764));
    } else {
        emu.pc = 2156436u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e794));
    }
}
#[inline(always)]
pub fn block_0x0020e794(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(31usize, 10usize, 0u32, 2156440u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2156440u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(2156440u32));
}
