pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2127624u32;
pub const PC_MAX: u32 = 2130704u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 107usize] = [
        block_0x00207708,
        block_0x00207724,
        block_0x00207734,
        block_0x00207738,
        block_0x00207748,
        block_0x0020774c,
        block_0x00207758,
        block_0x00207760,
        block_0x00207764,
        block_0x0020776c,
        block_0x00207780,
        block_0x00207794,
        block_0x002077a8,
        block_0x002077b0,
        block_0x002077d0,
        block_0x002077e8,
        block_0x00207810,
        block_0x00207814,
        block_0x00207824,
        block_0x00207828,
        block_0x00207838,
        block_0x0020783c,
        block_0x0020784c,
        block_0x00207850,
        block_0x00207860,
        block_0x00207864,
        block_0x00207874,
        block_0x002078a0,
        block_0x002078fc,
        block_0x0020796c,
        block_0x00207990,
        block_0x0020799c,
        block_0x002079a4,
        block_0x002079b0,
        block_0x002079c8,
        block_0x00207a54,
        block_0x00207a68,
        block_0x00207a74,
        block_0x00207a88,
        block_0x00207a9c,
        block_0x00207ae4,
        block_0x00207af8,
        block_0x00207b00,
        block_0x00207c3c,
        block_0x00207c48,
        block_0x00207c54,
        block_0x00207c58,
        block_0x00207c5c,
        block_0x00207ca0,
        block_0x00207cd8,
        block_0x00207ce8,
        block_0x00207cfc,
        block_0x00207d28,
        block_0x00207d60,
        block_0x00207d6c,
        block_0x00207e04,
        block_0x00207e24,
        block_0x00207e34,
        block_0x00207e44,
        block_0x00207e64,
        block_0x00207e70,
        block_0x00207e84,
        block_0x00207e98,
        block_0x00207ea0,
        block_0x00207ebc,
        block_0x00207ed8,
        block_0x00207ee0,
        block_0x00207ef4,
        block_0x00207f14,
        block_0x00207f84,
        block_0x00207fa8,
        block_0x00207fb4,
        block_0x00207fbc,
        block_0x00207fc8,
        block_0x00207fe0,
        block_0x00207ff8,
        block_0x00208000,
        block_0x00208014,
        block_0x00208028,
        block_0x0020803c,
        block_0x00208040,
        block_0x00208054,
        block_0x00208068,
        block_0x002080b0,
        block_0x002080c8,
        block_0x002080cc,
        block_0x002080d8,
        block_0x002080f8,
        block_0x002080fc,
        block_0x00208114,
        block_0x00208134,
        block_0x00208148,
        block_0x0020815c,
        block_0x0020817c,
        block_0x00208188,
        block_0x002081c4,
        block_0x002081f0,
        block_0x00208228,
        block_0x00208260,
        block_0x002082a4,
        block_0x002082b4,
        block_0x002082bc,
        block_0x002082c4,
        block_0x002082cc,
        block_0x002082d0,
        block_0x002082f8,
        block_0x00208310,
    ];
    const IDX: [u16; 771usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 0u16, 3u16, 4u16,
        0u16, 0u16, 0u16, 5u16, 6u16, 0u16, 0u16, 7u16, 0u16, 8u16, 9u16, 0u16, 10u16,
        0u16, 0u16, 0u16, 0u16, 11u16, 0u16, 0u16, 0u16, 0u16, 12u16, 0u16, 0u16, 0u16,
        0u16, 13u16, 0u16, 14u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 15u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 16u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 17u16, 18u16, 0u16, 0u16, 0u16, 19u16, 20u16, 0u16, 0u16, 0u16, 21u16,
        22u16, 0u16, 0u16, 0u16, 23u16, 24u16, 0u16, 0u16, 0u16, 25u16, 26u16, 0u16,
        0u16, 0u16, 27u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        28u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 29u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 30u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 31u16, 0u16, 0u16, 32u16, 0u16,
        33u16, 0u16, 0u16, 34u16, 0u16, 0u16, 0u16, 0u16, 0u16, 35u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 36u16, 0u16, 0u16, 0u16, 0u16, 37u16, 0u16, 0u16,
        38u16, 0u16, 0u16, 0u16, 0u16, 39u16, 0u16, 0u16, 0u16, 0u16, 40u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 41u16, 0u16, 0u16, 0u16, 0u16, 42u16, 0u16, 43u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 44u16, 0u16, 0u16,
        45u16, 0u16, 0u16, 46u16, 47u16, 48u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 49u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 50u16, 0u16, 0u16,
        0u16, 51u16, 0u16, 0u16, 0u16, 0u16, 52u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 53u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 54u16, 0u16, 0u16, 55u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 56u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 57u16, 0u16, 0u16, 0u16, 58u16, 0u16, 0u16, 0u16, 59u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 60u16, 0u16, 0u16, 61u16, 0u16, 0u16, 0u16, 0u16,
        62u16, 0u16, 0u16, 0u16, 0u16, 63u16, 0u16, 64u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 65u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 66u16, 0u16, 67u16, 0u16, 0u16,
        0u16, 0u16, 68u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 69u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 70u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 71u16, 0u16, 0u16, 72u16, 0u16,
        73u16, 0u16, 0u16, 74u16, 0u16, 0u16, 0u16, 0u16, 0u16, 75u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 76u16, 0u16, 77u16, 0u16, 0u16, 0u16, 0u16, 78u16, 0u16, 0u16, 0u16,
        0u16, 79u16, 0u16, 0u16, 0u16, 0u16, 80u16, 81u16, 0u16, 0u16, 0u16, 0u16, 82u16,
        0u16, 0u16, 0u16, 0u16, 83u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 84u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 85u16, 86u16, 0u16, 0u16, 87u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 88u16, 89u16, 0u16, 0u16, 0u16, 0u16, 0u16, 90u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 91u16, 0u16, 0u16, 0u16, 0u16, 92u16, 0u16, 0u16, 0u16, 0u16,
        93u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 94u16, 0u16, 0u16, 95u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        96u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 97u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 98u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        99u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 100u16, 0u16, 0u16, 0u16, 101u16, 0u16, 102u16, 0u16,
        103u16, 0u16, 104u16, 105u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 106u16, 0u16, 0u16, 0u16, 0u16, 0u16, 107u16,
    ];
    if pc < 2127624u32 || pc > 2130704u32 {
        return None;
    }
    let word_offset = ((pc - 2127624u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x00207708(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2127628u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2127632u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2127636u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2127640u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2127644u32);
    emu.lw_no_count(12usize, 10usize, 0u32, 2127648u32)?;
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2127672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207738));
    } else {
        emu.pc = 2127652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207724));
    }
}
#[inline(always)]
pub fn block_0x00207724(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2127656u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2127660u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2127664u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2127688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207748));
    } else {
        emu.pc = 2127668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207734));
    }
}
#[inline(always)]
pub fn block_0x00207734(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2127672u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2127692u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020774c));
}
#[inline(always)]
pub fn block_0x00207738(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 12usize, 1u32, 2127676u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2127680u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2127684u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2127692u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020774c));
    } else {
        emu.pc = 2127688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207748));
    }
}
#[inline(always)]
pub fn block_0x00207748(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 4u32, 2127692u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2127692u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020774c));
}
#[inline(always)]
pub fn block_0x0020774c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 10usize, 27u32, 2127696u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2127700u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2127724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020776c));
    } else {
        emu.pc = 2127704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207758));
    }
}
#[inline(always)]
pub fn block_0x00207758(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 9usize, 5u32, 2127708u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2127724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020776c));
    } else {
        emu.pc = 2127712u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207760));
    }
}
#[inline(always)]
pub fn block_0x00207760(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2127744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207780));
    } else {
        emu.pc = 2127716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207764));
    }
}
#[inline(always)]
pub fn block_0x00207764(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2127720u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2127724u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2127764u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207794));
}
#[inline(always)]
pub fn block_0x0020776c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2127728u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1136u32, 2127732u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2127736u32);
    emu.apc_no_count(1usize, 2127736u32, 77824u32, 2127740u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127744u32;
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
pub fn block_0x00207780(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 4u32, 2127748u32)?;
    emu.sli_no_count(12usize, 12usize, 5u32, 2127752u32);
    emu.sw_no_count(10usize, 2usize, 24u32, 2127756u32)?;
    emu.sw_no_count(12usize, 2usize, 32u32, 2127760u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2127764u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2127764u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207794));
}
#[inline(always)]
pub fn block_0x00207794(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 28u32, 2127768u32)?;
    emu.adi_no_count(10usize, 2usize, 12u32, 2127772u32);
    emu.adi_no_count(12usize, 2usize, 24u32, 2127776u32);
    emu.apc_no_count(1usize, 2127776u32, 0u32, 2127780u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127784u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966936u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002077a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2127788u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2127824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002077d0));
    } else {
        emu.pc = 2127792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002077b0));
    }
}
#[inline(always)]
pub fn block_0x002077b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 16u32, 2127796u32)?;
    emu.sw_no_count(9usize, 8usize, 0u32, 2127800u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2127804u32)?;
    emu.lw_no_count(1usize, 2usize, 44u32, 2127808u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2127812u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2127816u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2127820u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127824u32;
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
pub fn block_0x002077d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 16u32, 2127828u32)?;
    emu.lw_no_count(11usize, 2usize, 20u32, 2127832u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2127836u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1136u32, 2127840u32);
    emu.apc_no_count(1usize, 2127840u32, 77824u32, 2127844u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127848u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966480u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002077e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2127852u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2127856u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2127860u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2127864u32);
    emu.adi_no_count(10usize, 0usize, 3u32, 2127868u32);
    emu.sw_no_count(10usize, 2usize, 4u32, 2127872u32)?;
    emu.adi_no_count(11usize, 8usize, 12u32, 2127876u32);
    emu.adi_no_count(10usize, 2usize, 4u32, 2127880u32);
    emu.apc_no_count(1usize, 2127880u32, 4294950912u32, 2127884u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127888u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1832u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207810(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2127988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207874));
    } else {
        emu.pc = 2127892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207814));
    }
}
#[inline(always)]
pub fn block_0x00207814(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 8usize, 44u32, 2127896u32);
    emu.adi_no_count(10usize, 2usize, 4u32, 2127900u32);
    emu.apc_no_count(1usize, 2127900u32, 4294950912u32, 2127904u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127908u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1812u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207824(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2127988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207874));
    } else {
        emu.pc = 2127912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207828));
    }
}
#[inline(always)]
pub fn block_0x00207828(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 8usize, 76u32, 2127916u32);
    emu.adi_no_count(10usize, 2usize, 4u32, 2127920u32);
    emu.apc_no_count(1usize, 2127920u32, 4294950912u32, 2127924u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127928u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1792u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207838(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2127988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207874));
    } else {
        emu.pc = 2127932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020783c));
    }
}
#[inline(always)]
pub fn block_0x0020783c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 4u32, 2127936u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2127940u32);
    emu.apc_no_count(1usize, 2127940u32, 4294946816u32, 2127944u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127948u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(828u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020784c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2127988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207874));
    } else {
        emu.pc = 2127952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207850));
    }
}
#[inline(always)]
pub fn block_0x00207850(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 8usize, 108u32, 2127956u32);
    emu.adi_no_count(10usize, 2usize, 4u32, 2127960u32);
    emu.apc_no_count(1usize, 2127960u32, 4294950912u32, 2127964u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127968u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1752u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207860(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2127988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207874));
    } else {
        emu.pc = 2127972u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207864));
    }
}
#[inline(always)]
pub fn block_0x00207864(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2127976u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2127980u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2127984u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127988u32;
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
pub fn block_0x00207874(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 0u32, 2127992u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2127996u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1196u32, 2128000u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2128004u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1180u32, 2128008u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2128012u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1216u32, 2128016u32);
    emu.adi_no_count(11usize, 0usize, 20u32, 2128020u32);
    emu.adi_no_count(12usize, 2usize, 0u32, 2128024u32);
    emu.apc_no_count(1usize, 2128024u32, 86016u32, 2128028u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128032u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967284u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002078a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 23u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294966944u32, 2128036u32);
    emu.sw_no_count(1usize, 2usize, 348u32, 2128040u32)?;
    emu.sw_no_count(8usize, 2usize, 344u32, 2128044u32)?;
    emu.sw_no_count(9usize, 2usize, 340u32, 2128048u32)?;
    emu.sw_no_count(18usize, 2usize, 336u32, 2128052u32)?;
    emu.sw_no_count(19usize, 2usize, 332u32, 2128056u32)?;
    emu.sw_no_count(20usize, 2usize, 328u32, 2128060u32)?;
    emu.sw_no_count(21usize, 2usize, 324u32, 2128064u32)?;
    emu.sw_no_count(22usize, 2usize, 320u32, 2128068u32)?;
    emu.sw_no_count(23usize, 2usize, 316u32, 2128072u32)?;
    emu.sw_no_count(24usize, 2usize, 312u32, 2128076u32)?;
    emu.sw_no_count(25usize, 2usize, 308u32, 2128080u32)?;
    emu.sw_no_count(26usize, 2usize, 304u32, 2128084u32)?;
    emu.sw_no_count(27usize, 2usize, 300u32, 2128088u32)?;
    emu.adi_no_count(9usize, 12usize, 0u32, 2128092u32);
    emu.adi_no_count(19usize, 11usize, 0u32, 2128096u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2128100u32);
    emu.adi_no_count(18usize, 2usize, 48u32, 2128104u32);
    emu.adi_no_count(12usize, 0usize, 65u32, 2128108u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2128112u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2128116u32);
    emu.apc_no_count(1usize, 2128116u32, 4096u32, 2128120u32);
    emu.add_memory_rw_events(23usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128124u32;
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
#[inline(never)]
pub fn block_0x002078fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(1779032064u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2128128u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(3144134656u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2128132u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1013903360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2128136u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2773479424u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2128140u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1359892480u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2128144u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2600824832u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2128148u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(528736256u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2128152u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1639u32, 2128156u32);
    emu.adi_no_count(11usize, 11usize, 4294966917u32, 2128160u32);
    emu.adi_no_count(12usize, 12usize, 882u32, 2128164u32);
    emu.adi_no_count(13usize, 13usize, 1338u32, 2128168u32);
    emu.sw_no_count(10usize, 2usize, 8u32, 2128172u32)?;
    emu.sw_no_count(11usize, 2usize, 12u32, 2128176u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2128180u32)?;
    emu.sw_no_count(13usize, 2usize, 20u32, 2128184u32)?;
    let a = 0u32.wrapping_add(1541459968u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2128188u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 14usize, 639u32, 2128192u32);
    emu.adi_no_count(12usize, 15usize, 4294965388u32, 2128196u32);
    emu.adi_no_count(13usize, 16usize, 4294965675u32, 2128200u32);
    emu.adi_no_count(10usize, 10usize, 4294966553u32, 2128204u32);
    emu.sw_no_count(11usize, 2usize, 24u32, 2128208u32)?;
    emu.sw_no_count(12usize, 2usize, 28u32, 2128212u32)?;
    emu.sw_no_count(13usize, 2usize, 32u32, 2128216u32)?;
    emu.sw_no_count(10usize, 2usize, 36u32, 2128220u32)?;
    emu.sri_no_count(12usize, 9usize, 6u32, 2128224u32);
    emu.sw_no_count(0usize, 2usize, 40u32, 2128228u32)?;
    emu.sw_no_count(0usize, 2usize, 44u32, 2128232u32)?;
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2128284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020799c));
    } else {
        emu.pc = 2128236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020796c));
    }
}
#[inline]
pub fn block_0x0020796c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(20usize, 9usize, 4294967232u32, 2128240u32);
    emu.ani_no_count(9usize, 9usize, 63u32, 2128244u32);
    emu.adr_no_count(20usize, 19usize, 20usize, 2128248u32);
    emu.sw_no_count(12usize, 2usize, 40u32, 2128252u32)?;
    emu.sw_no_count(0usize, 2usize, 44u32, 2128256u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2128260u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2128264u32);
    emu.apc_no_count(1usize, 2128264u32, 61440u32, 2128268u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128272u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(704u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207990(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2128276u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2128280u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2128284u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2128292u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002079a4));
}
#[inline(always)]
pub fn block_0x0020799c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2128288u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2128292u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2128292u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002079a4));
}
#[inline(always)]
pub fn block_0x002079a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 9usize, 0u32, 2128296u32);
    emu.apc_no_count(1usize, 2128296u32, 4096u32, 2128300u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128304u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(908u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002079b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(9usize, 2usize, 112u32, 2128308u32);
    emu.adi_no_count(10usize, 2usize, 120u32, 2128312u32);
    emu.adi_no_count(11usize, 2usize, 8u32, 2128316u32);
    emu.adi_no_count(12usize, 0usize, 112u32, 2128320u32);
    emu.apc_no_count(1usize, 2128320u32, 4096u32, 2128324u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128328u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(884u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x002079c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 35u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 2usize, 160u32, 2128332u32);
    emu.lbu_no_count(18usize, 2usize, 224u32, 2128336u32);
    emu.lw_no_count(10usize, 2usize, 152u32, 2128340u32)?;
    emu.lw_no_count(11usize, 2usize, 156u32, 2128344u32)?;
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2128348u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 0usize, 128u32, 2128352u32);
    emu.adi_no_count(12usize, 12usize, 4294967040u32, 2128356u32);
    emu.sli_no_count(14usize, 11usize, 9u32, 2128360u32);
    emu.sli_no_count(15usize, 10usize, 9u32, 2128364u32);
    emu.sli_no_count(16usize, 18usize, 3u32, 2128368u32);
    emu.sli_no_count(17usize, 10usize, 1u32, 2128372u32);
    emu.sli_no_count(11usize, 11usize, 1u32, 2128376u32);
    emu.orr_no_count(16usize, 15usize, 16usize, 2128380u32);
    emu.anr_no_count(17usize, 17usize, 12usize, 2128384u32);
    emu.sri_no_count(15usize, 15usize, 24u32, 2128388u32);
    emu.anr_no_count(11usize, 11usize, 12usize, 2128392u32);
    emu.orr_no_count(15usize, 17usize, 15usize, 2128396u32);
    emu.sri_no_count(17usize, 14usize, 24u32, 2128400u32);
    emu.orr_no_count(11usize, 11usize, 17usize, 2128404u32);
    emu.adi_no_count(17usize, 0usize, 63u32, 2128408u32);
    emu.sri_no_count(5usize, 10usize, 23u32, 2128412u32);
    emu.orr_no_count(10usize, 14usize, 5usize, 2128416u32);
    emu.anr_no_count(14usize, 16usize, 12usize, 2128420u32);
    emu.anr_no_count(12usize, 10usize, 12usize, 2128424u32);
    emu.sli_no_count(10usize, 18usize, 27u32, 2128428u32);
    emu.orr_no_count(15usize, 10usize, 15usize, 2128432u32);
    emu.adr_no_count(10usize, 9usize, 18usize, 2128436u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2128440u32);
    emu.sli_no_count(14usize, 14usize, 8u32, 2128444u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2128448u32);
    emu.orr_no_count(11usize, 5usize, 11usize, 2128452u32);
    emu.orr_no_count(20usize, 15usize, 14usize, 2128456u32);
    emu.orr_no_count(19usize, 11usize, 12usize, 2128460u32);
    emu.sb_no_count(13usize, 10usize, 0u32, 2128464u32);
    emu.add_memory_rw_events(34usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2128500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207a74));
    } else {
        emu.pc = 2128468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207a54));
    }
}
#[inline(always)]
pub fn block_0x00207a54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 10usize, 1u32, 2128472u32);
    emu.xri_no_count(12usize, 18usize, 63u32, 2128476u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2128480u32);
    emu.apc_no_count(1usize, 2128480u32, 4096u32, 2128484u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128488u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(476u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207a68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(10usize, 18usize, 56u32, 2128492u32);
    emu.adi_no_count(11usize, 0usize, 7u32, 2128496u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2128612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207ae4));
    } else {
        emu.pc = 2128500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207a74));
    }
}
#[inline(always)]
pub fn block_0x00207a74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 120u32, 2128504u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2128508u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2128512u32);
    emu.apc_no_count(1usize, 2128512u32, 61440u32, 2128516u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128520u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(456u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207a88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 236u32, 2128524u32);
    emu.adi_no_count(12usize, 0usize, 56u32, 2128528u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2128532u32);
    emu.apc_no_count(1usize, 2128532u32, 4096u32, 2128536u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128540u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(424u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00207a9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 20usize, 24u32, 2128544u32);
    emu.sri_no_count(11usize, 20usize, 16u32, 2128548u32);
    emu.sri_no_count(12usize, 20usize, 8u32, 2128552u32);
    emu.sri_no_count(13usize, 19usize, 24u32, 2128556u32);
    emu.sri_no_count(14usize, 19usize, 16u32, 2128560u32);
    emu.sb_no_count(20usize, 2usize, 296u32, 2128564u32);
    emu.sb_no_count(12usize, 2usize, 297u32, 2128568u32);
    emu.sb_no_count(11usize, 2usize, 298u32, 2128572u32);
    emu.sb_no_count(10usize, 2usize, 299u32, 2128576u32);
    emu.sri_no_count(10usize, 19usize, 8u32, 2128580u32);
    emu.sb_no_count(19usize, 2usize, 292u32, 2128584u32);
    emu.sb_no_count(10usize, 2usize, 293u32, 2128588u32);
    emu.sb_no_count(14usize, 2usize, 294u32, 2128592u32);
    emu.sb_no_count(13usize, 2usize, 295u32, 2128596u32);
    emu.adi_no_count(10usize, 2usize, 120u32, 2128600u32);
    emu.adi_no_count(11usize, 2usize, 236u32, 2128604u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2128608u32);
    emu.add_memory_rw_events(18usize);
    let return_addr = 2128612u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2128632u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207af8));
}
#[inline(always)]
pub fn block_0x00207ae4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(19usize, 2usize, 216u32, 2128616u32)?;
    emu.sw_no_count(20usize, 2usize, 220u32, 2128620u32)?;
    emu.adi_no_count(10usize, 2usize, 120u32, 2128624u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2128628u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2128632u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2128632u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207af8));
}
#[inline(always)]
pub fn block_0x00207af8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2128632u32, 61440u32, 2128636u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128640u32;
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
#[inline(never)]
pub fn block_0x00207b00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 79u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(28usize, 2usize, 120u32, 2128644u32)?;
    emu.lw_no_count(6usize, 2usize, 124u32, 2128648u32)?;
    emu.lw_no_count(16usize, 2usize, 128u32, 2128652u32)?;
    emu.lw_no_count(14usize, 2usize, 132u32, 2128656u32)?;
    emu.lw_no_count(13usize, 2usize, 136u32, 2128660u32)?;
    emu.lw_no_count(12usize, 2usize, 140u32, 2128664u32)?;
    emu.lw_no_count(11usize, 2usize, 144u32, 2128668u32)?;
    emu.lw_no_count(10usize, 2usize, 148u32, 2128672u32)?;
    emu.sri_no_count(31usize, 28usize, 24u32, 2128676u32);
    emu.sri_no_count(9usize, 6usize, 24u32, 2128680u32);
    emu.sri_no_count(30usize, 16usize, 24u32, 2128684u32);
    emu.sri_no_count(29usize, 14usize, 24u32, 2128688u32);
    emu.sri_no_count(7usize, 13usize, 24u32, 2128692u32);
    emu.sri_no_count(5usize, 12usize, 24u32, 2128696u32);
    emu.sri_no_count(17usize, 11usize, 24u32, 2128700u32);
    emu.sri_no_count(15usize, 10usize, 24u32, 2128704u32);
    emu.sri_no_count(18usize, 28usize, 8u32, 2128708u32);
    emu.sri_no_count(19usize, 28usize, 16u32, 2128712u32);
    emu.sri_no_count(20usize, 6usize, 8u32, 2128716u32);
    emu.sri_no_count(21usize, 6usize, 16u32, 2128720u32);
    emu.sri_no_count(22usize, 16usize, 8u32, 2128724u32);
    emu.sri_no_count(23usize, 16usize, 16u32, 2128728u32);
    emu.sri_no_count(24usize, 14usize, 8u32, 2128732u32);
    emu.sri_no_count(25usize, 14usize, 16u32, 2128736u32);
    emu.sri_no_count(26usize, 13usize, 8u32, 2128740u32);
    emu.sri_no_count(27usize, 13usize, 16u32, 2128744u32);
    emu.sri_no_count(1usize, 12usize, 8u32, 2128748u32);
    emu.sb_no_count(31usize, 8usize, 0u32, 2128752u32);
    emu.sri_no_count(31usize, 12usize, 16u32, 2128756u32);
    emu.sb_no_count(19usize, 8usize, 1u32, 2128760u32);
    emu.sri_no_count(19usize, 11usize, 8u32, 2128764u32);
    emu.sb_no_count(18usize, 8usize, 2u32, 2128768u32);
    emu.sb_no_count(28usize, 8usize, 3u32, 2128772u32);
    emu.sri_no_count(28usize, 11usize, 16u32, 2128776u32);
    emu.sb_no_count(9usize, 8usize, 4u32, 2128780u32);
    emu.sb_no_count(21usize, 8usize, 5u32, 2128784u32);
    emu.sb_no_count(20usize, 8usize, 6u32, 2128788u32);
    emu.sb_no_count(6usize, 8usize, 7u32, 2128792u32);
    emu.sri_no_count(6usize, 10usize, 8u32, 2128796u32);
    emu.sb_no_count(30usize, 8usize, 8u32, 2128800u32);
    emu.sb_no_count(23usize, 8usize, 9u32, 2128804u32);
    emu.sb_no_count(22usize, 8usize, 10u32, 2128808u32);
    emu.sb_no_count(16usize, 8usize, 11u32, 2128812u32);
    emu.sri_no_count(16usize, 10usize, 16u32, 2128816u32);
    emu.sb_no_count(29usize, 8usize, 12u32, 2128820u32);
    emu.sb_no_count(25usize, 8usize, 13u32, 2128824u32);
    emu.sb_no_count(24usize, 8usize, 14u32, 2128828u32);
    emu.sb_no_count(14usize, 8usize, 15u32, 2128832u32);
    emu.sb_no_count(7usize, 8usize, 16u32, 2128836u32);
    emu.sb_no_count(27usize, 8usize, 17u32, 2128840u32);
    emu.sb_no_count(26usize, 8usize, 18u32, 2128844u32);
    emu.sb_no_count(13usize, 8usize, 19u32, 2128848u32);
    emu.sb_no_count(5usize, 8usize, 20u32, 2128852u32);
    emu.sb_no_count(31usize, 8usize, 21u32, 2128856u32);
    emu.sb_no_count(1usize, 8usize, 22u32, 2128860u32);
    emu.sb_no_count(12usize, 8usize, 23u32, 2128864u32);
    emu.sb_no_count(17usize, 8usize, 24u32, 2128868u32);
    emu.sb_no_count(28usize, 8usize, 25u32, 2128872u32);
    emu.sb_no_count(19usize, 8usize, 26u32, 2128876u32);
    emu.sb_no_count(11usize, 8usize, 27u32, 2128880u32);
    emu.sb_no_count(15usize, 8usize, 28u32, 2128884u32);
    emu.sb_no_count(16usize, 8usize, 29u32, 2128888u32);
    emu.sb_no_count(6usize, 8usize, 30u32, 2128892u32);
    emu.sb_no_count(10usize, 8usize, 31u32, 2128896u32);
    emu.lw_no_count(1usize, 2usize, 348u32, 2128900u32)?;
    emu.lw_no_count(8usize, 2usize, 344u32, 2128904u32)?;
    emu.lw_no_count(9usize, 2usize, 340u32, 2128908u32)?;
    emu.lw_no_count(18usize, 2usize, 336u32, 2128912u32)?;
    emu.lw_no_count(19usize, 2usize, 332u32, 2128916u32)?;
    emu.lw_no_count(20usize, 2usize, 328u32, 2128920u32)?;
    emu.lw_no_count(21usize, 2usize, 324u32, 2128924u32)?;
    emu.lw_no_count(22usize, 2usize, 320u32, 2128928u32)?;
    emu.lw_no_count(23usize, 2usize, 316u32, 2128932u32)?;
    emu.lw_no_count(24usize, 2usize, 312u32, 2128936u32)?;
    emu.lw_no_count(25usize, 2usize, 308u32, 2128940u32)?;
    emu.lw_no_count(26usize, 2usize, 304u32, 2128944u32)?;
    emu.lw_no_count(27usize, 2usize, 300u32, 2128948u32)?;
    emu.adi_no_count(2usize, 2usize, 352u32, 2128952u32);
    emu.add_memory_rw_events(79usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128956u32;
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
pub fn block_0x00207c3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 10usize, 0u32, 2128960u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2128964u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2128984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207c58));
    } else {
        emu.pc = 2128968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207c48));
    }
}
#[inline(always)]
pub fn block_0x00207c48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 11usize, 4u32, 2128972u32)?;
    emu.lw_no_count(6usize, 11usize, 0u32, 2128976u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2128984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207c58));
    } else {
        emu.pc = 2128980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207c54));
    }
}
#[inline(always)]
pub fn block_0x00207c54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2128984u32;
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
pub fn block_0x00207c58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128988u32;
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
pub fn block_0x00207c5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294966576u32, 2128992u32);
    emu.sw_no_count(1usize, 2usize, 716u32, 2128996u32)?;
    emu.sw_no_count(8usize, 2usize, 712u32, 2129000u32)?;
    emu.sw_no_count(9usize, 2usize, 708u32, 2129004u32)?;
    emu.sw_no_count(18usize, 2usize, 704u32, 2129008u32)?;
    emu.sw_no_count(19usize, 2usize, 700u32, 2129012u32)?;
    emu.sw_no_count(20usize, 2usize, 696u32, 2129016u32)?;
    emu.sw_no_count(21usize, 2usize, 692u32, 2129020u32)?;
    emu.sw_no_count(22usize, 2usize, 688u32, 2129024u32)?;
    emu.sw_no_count(23usize, 2usize, 684u32, 2129028u32)?;
    emu.sw_no_count(24usize, 2usize, 680u32, 2129032u32)?;
    emu.sw_no_count(25usize, 2usize, 676u32, 2129036u32)?;
    emu.sw_no_count(26usize, 2usize, 672u32, 2129040u32)?;
    emu.sw_no_count(27usize, 2usize, 668u32, 2129044u32)?;
    emu.adi_no_count(10usize, 2usize, 400u32, 2129048u32);
    emu.apc_no_count(1usize, 2129048u32, 4096u32, 2129052u32);
    emu.add_memory_rw_events(17usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129056u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965352u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00207ca0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 404u32, 2129060u32)?;
    emu.lw_no_count(11usize, 2usize, 408u32, 2129064u32)?;
    emu.sw_no_count(10usize, 2usize, 140u32, 2129068u32)?;
    emu.sw_no_count(11usize, 2usize, 144u32, 2129072u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2129076u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965612u32, 2129080u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2129084u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965864u32, 2129088u32);
    emu.adi_no_count(10usize, 2usize, 512u32, 2129092u32);
    emu.adi_no_count(11usize, 2usize, 140u32, 2129096u32);
    emu.adi_no_count(13usize, 0usize, 13u32, 2129100u32);
    emu.adi_no_count(15usize, 0usize, 5u32, 2129104u32);
    emu.apc_no_count(1usize, 2129104u32, 4294942720u32, 2129108u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129112u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967220u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207cd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(8usize, 2usize, 560u32, 2129116u32)?;
    emu.lw_no_count(18usize, 2usize, 512u32, 2129120u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(9usize, a);
    emu.pc = 2129124u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2130612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002082b4));
    } else {
        emu.pc = 2129128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207ce8));
    }
}
#[inline(always)]
pub fn block_0x00207ce8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 2usize, 516u32, 2129132u32);
    emu.adi_no_count(10usize, 2usize, 4u32, 2129136u32);
    emu.adi_no_count(12usize, 0usize, 44u32, 2129140u32);
    emu.apc_no_count(1usize, 2129140u32, 4096u32, 2129144u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129148u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(64u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00207cfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 564u32, 2129152u32)?;
    emu.lw_no_count(11usize, 2usize, 568u32, 2129156u32)?;
    emu.lw_no_count(12usize, 2usize, 572u32, 2129160u32)?;
    emu.sw_no_count(18usize, 2usize, 0u32, 2129164u32)?;
    emu.sw_no_count(8usize, 2usize, 48u32, 2129168u32)?;
    emu.sw_no_count(10usize, 2usize, 52u32, 2129172u32)?;
    emu.sw_no_count(11usize, 2usize, 56u32, 2129176u32)?;
    emu.sw_no_count(12usize, 2usize, 60u32, 2129180u32)?;
    emu.adi_no_count(10usize, 2usize, 400u32, 2129184u32);
    emu.apc_no_count(1usize, 2129184u32, 0u32, 2129188u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129192u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(2016u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00207d28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 404u32, 2129196u32)?;
    emu.lw_no_count(11usize, 2usize, 408u32, 2129200u32)?;
    emu.sw_no_count(10usize, 2usize, 140u32, 2129204u32)?;
    emu.sw_no_count(11usize, 2usize, 144u32, 2129208u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2129212u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965500u32, 2129216u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2129220u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965564u32, 2129224u32);
    emu.adi_no_count(10usize, 2usize, 512u32, 2129228u32);
    emu.adi_no_count(11usize, 2usize, 140u32, 2129232u32);
    emu.adi_no_count(13usize, 0usize, 9u32, 2129236u32);
    emu.adi_no_count(15usize, 0usize, 6u32, 2129240u32);
    emu.apc_no_count(1usize, 2129240u32, 4294942720u32, 2129244u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129248u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965256u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207d60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 512u32, 2129252u32)?;
    emu.lw_no_count(8usize, 2usize, 516u32, 2129256u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2130620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002082bc));
    } else {
        emu.pc = 2129260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207d6c));
    }
}
#[inline(never)]
pub fn block_0x00207d6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 38u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(9usize, 2usize, 520u32, 2129264u32)?;
    emu.lw_no_count(18usize, 2usize, 528u32, 2129268u32)?;
    emu.lw_no_count(19usize, 2usize, 532u32, 2129272u32)?;
    emu.lw_no_count(10usize, 2usize, 536u32, 2129276u32)?;
    emu.lw_no_count(11usize, 2usize, 540u32, 2129280u32)?;
    emu.lw_no_count(12usize, 2usize, 544u32, 2129284u32)?;
    emu.lw_no_count(13usize, 2usize, 548u32, 2129288u32)?;
    emu.lw_no_count(14usize, 2usize, 552u32, 2129292u32)?;
    emu.lw_no_count(15usize, 2usize, 572u32, 2129296u32)?;
    emu.lw_no_count(16usize, 2usize, 576u32, 2129300u32)?;
    emu.lw_no_count(17usize, 2usize, 580u32, 2129304u32)?;
    emu.lw_no_count(5usize, 2usize, 584u32, 2129308u32)?;
    emu.sw_no_count(10usize, 2usize, 96u32, 2129312u32)?;
    emu.sw_no_count(11usize, 2usize, 100u32, 2129316u32)?;
    emu.sw_no_count(12usize, 2usize, 104u32, 2129320u32)?;
    emu.sw_no_count(13usize, 2usize, 108u32, 2129324u32)?;
    emu.lw_no_count(10usize, 2usize, 556u32, 2129328u32)?;
    emu.lw_no_count(11usize, 2usize, 560u32, 2129332u32)?;
    emu.lw_no_count(12usize, 2usize, 564u32, 2129336u32)?;
    emu.lw_no_count(13usize, 2usize, 568u32, 2129340u32)?;
    emu.sw_no_count(14usize, 2usize, 112u32, 2129344u32)?;
    emu.sw_no_count(10usize, 2usize, 116u32, 2129348u32)?;
    emu.sw_no_count(11usize, 2usize, 120u32, 2129352u32)?;
    emu.sw_no_count(12usize, 2usize, 124u32, 2129356u32)?;
    emu.lw_no_count(10usize, 2usize, 588u32, 2129360u32)?;
    emu.lw_no_count(11usize, 2usize, 592u32, 2129364u32)?;
    emu.lw_no_count(12usize, 2usize, 596u32, 2129368u32)?;
    emu.sw_no_count(5usize, 2usize, 80u32, 2129372u32)?;
    emu.sw_no_count(10usize, 2usize, 84u32, 2129376u32)?;
    emu.sw_no_count(11usize, 2usize, 88u32, 2129380u32)?;
    emu.sw_no_count(12usize, 2usize, 92u32, 2129384u32)?;
    emu.sw_no_count(13usize, 2usize, 64u32, 2129388u32)?;
    emu.sw_no_count(15usize, 2usize, 68u32, 2129392u32)?;
    emu.sw_no_count(16usize, 2usize, 72u32, 2129396u32)?;
    emu.sw_no_count(17usize, 2usize, 76u32, 2129400u32)?;
    emu.adi_no_count(10usize, 2usize, 400u32, 2129404u32);
    emu.apc_no_count(1usize, 2129404u32, 0u32, 2129408u32);
    emu.add_memory_rw_events(38usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129412u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1796u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207e04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 404u32, 2129416u32)?;
    emu.lw_no_count(11usize, 2usize, 408u32, 2129420u32)?;
    emu.sw_no_count(10usize, 2usize, 140u32, 2129424u32)?;
    emu.sw_no_count(11usize, 2usize, 144u32, 2129428u32)?;
    emu.adi_no_count(10usize, 2usize, 512u32, 2129432u32);
    emu.adi_no_count(11usize, 2usize, 140u32, 2129436u32);
    emu.apc_no_count(1usize, 2129436u32, 4294938624u32, 2129440u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129444u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965932u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207e24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 512u32, 2129448u32)?;
    emu.lw_no_count(20usize, 2usize, 516u32, 2129452u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(23usize, a);
    emu.pc = 2129456u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2130628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002082c4));
    } else {
        emu.pc = 2129460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207e34));
    }
}
#[inline(always)]
pub fn block_0x00207e34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(22usize, 2usize, 520u32, 2129464u32)?;
    emu.adi_no_count(10usize, 2usize, 400u32, 2129468u32);
    emu.apc_no_count(1usize, 2129468u32, 0u32, 2129472u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129476u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1732u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207e44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 404u32, 2129480u32)?;
    emu.lw_no_count(11usize, 2usize, 408u32, 2129484u32)?;
    emu.sw_no_count(10usize, 2usize, 140u32, 2129488u32)?;
    emu.sw_no_count(11usize, 2usize, 144u32, 2129492u32)?;
    emu.adi_no_count(10usize, 2usize, 512u32, 2129496u32);
    emu.adi_no_count(11usize, 2usize, 140u32, 2129500u32);
    emu.apc_no_count(1usize, 2129500u32, 4294938624u32, 2129504u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129508u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965868u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207e64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 512u32, 2129512u32)?;
    emu.lw_no_count(21usize, 2usize, 516u32, 2129516u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2130636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002082cc));
    } else {
        emu.pc = 2129520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207e70));
    }
}
#[inline(always)]
pub fn block_0x00207e70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(23usize, 2usize, 520u32, 2129524u32)?;
    emu.adi_no_count(10usize, 2usize, 128u32, 2129528u32);
    emu.adi_no_count(11usize, 2usize, 0u32, 2129532u32);
    emu.apc_no_count(1usize, 2129532u32, 61440u32, 2129536u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129540u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965712u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207e84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 512u32, 2129544u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2129548u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2129552u32);
    emu.apc_no_count(1usize, 2129552u32, 4294946816u32, 2129556u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129560u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1792u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207e98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 512u32, 2129564u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2130372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002081c4));
    } else {
        emu.pc = 2129568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207ea0));
    }
}
#[inline(always)]
pub fn block_0x00207ea0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(24usize, 2usize, 516u32, 2129572u32)?;
    emu.lw_no_count(25usize, 2usize, 520u32, 2129576u32)?;
    emu.adi_no_count(11usize, 2usize, 524u32, 2129580u32);
    emu.adi_no_count(10usize, 2usize, 148u32, 2129584u32);
    emu.adi_no_count(12usize, 0usize, 60u32, 2129588u32);
    emu.apc_no_count(1usize, 2129588u32, 4096u32, 2129592u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129596u32;
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
pub fn block_0x00207ebc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(24usize, 2usize, 140u32, 2129600u32)?;
    emu.sw_no_count(25usize, 2usize, 144u32, 2129604u32)?;
    emu.adi_no_count(10usize, 2usize, 512u32, 2129608u32);
    emu.adi_no_count(11usize, 2usize, 96u32, 2129612u32);
    emu.adi_no_count(12usize, 2usize, 64u32, 2129616u32);
    emu.apc_no_count(1usize, 2129616u32, 4294959104u32, 2129620u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129624u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(476u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207ed8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 512u32, 2129628u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2130416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002081f0));
    } else {
        emu.pc = 2129632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207ee0));
    }
}
#[inline(always)]
pub fn block_0x00207ee0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 2usize, 516u32, 2129636u32);
    emu.adi_no_count(10usize, 2usize, 208u32, 2129640u32);
    emu.adi_no_count(12usize, 0usize, 64u32, 2129644u32);
    emu.apc_no_count(1usize, 2129644u32, 4096u32, 2129648u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129652u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966856u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207ef4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(26usize, 2usize, 132u32, 2129656u32)?;
    emu.lw_no_count(24usize, 2usize, 136u32, 2129660u32)?;
    emu.adi_no_count(25usize, 2usize, 552u32, 2129664u32);
    emu.adi_no_count(12usize, 0usize, 65u32, 2129668u32);
    emu.adi_no_count(10usize, 25usize, 0u32, 2129672u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2129676u32);
    emu.apc_no_count(1usize, 2129676u32, 4096u32, 2129680u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129684u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966576u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00207f14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(1779032064u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2129688u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(3144134656u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2129692u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1013903360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2129696u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2773479424u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2129700u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1359892480u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2129704u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2600824832u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2129708u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(528736256u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2129712u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1639u32, 2129716u32);
    emu.adi_no_count(11usize, 11usize, 4294966917u32, 2129720u32);
    emu.adi_no_count(12usize, 12usize, 882u32, 2129724u32);
    emu.adi_no_count(13usize, 13usize, 1338u32, 2129728u32);
    emu.sw_no_count(10usize, 2usize, 512u32, 2129732u32)?;
    emu.sw_no_count(11usize, 2usize, 516u32, 2129736u32)?;
    emu.sw_no_count(12usize, 2usize, 520u32, 2129740u32)?;
    emu.sw_no_count(13usize, 2usize, 524u32, 2129744u32)?;
    let a = 0u32.wrapping_add(1541459968u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2129748u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 14usize, 639u32, 2129752u32);
    emu.adi_no_count(12usize, 15usize, 4294965388u32, 2129756u32);
    emu.adi_no_count(13usize, 16usize, 4294965675u32, 2129760u32);
    emu.adi_no_count(10usize, 10usize, 4294966553u32, 2129764u32);
    emu.sw_no_count(11usize, 2usize, 528u32, 2129768u32)?;
    emu.sw_no_count(12usize, 2usize, 532u32, 2129772u32)?;
    emu.sw_no_count(13usize, 2usize, 536u32, 2129776u32)?;
    emu.sw_no_count(10usize, 2usize, 540u32, 2129780u32)?;
    emu.sri_no_count(12usize, 24usize, 6u32, 2129784u32);
    emu.sw_no_count(0usize, 2usize, 544u32, 2129788u32)?;
    emu.sw_no_count(0usize, 2usize, 548u32, 2129792u32)?;
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2129844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207fb4));
    } else {
        emu.pc = 2129796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207f84));
    }
}
#[inline]
pub fn block_0x00207f84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(27usize, 24usize, 4294967232u32, 2129800u32);
    emu.ani_no_count(24usize, 24usize, 63u32, 2129804u32);
    emu.adr_no_count(27usize, 26usize, 27usize, 2129808u32);
    emu.sw_no_count(12usize, 2usize, 544u32, 2129812u32)?;
    emu.sw_no_count(0usize, 2usize, 548u32, 2129816u32)?;
    emu.adi_no_count(10usize, 2usize, 512u32, 2129820u32);
    emu.adi_no_count(11usize, 26usize, 0u32, 2129824u32);
    emu.apc_no_count(1usize, 2129824u32, 40960u32, 2129828u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129832u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966068u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207fa8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 25usize, 0u32, 2129836u32);
    emu.adi_no_count(11usize, 27usize, 0u32, 2129840u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2129844u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2129852u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207fbc));
}
#[inline(always)]
pub fn block_0x00207fb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 25usize, 0u32, 2129848u32);
    emu.adi_no_count(11usize, 26usize, 0u32, 2129852u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2129852u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207fbc));
}
#[inline(always)]
pub fn block_0x00207fbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 24usize, 0u32, 2129856u32);
    emu.apc_no_count(1usize, 2129856u32, 4096u32, 2129860u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129864u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966644u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207fc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(24usize, 2usize, 616u32, 2129868u32);
    emu.adi_no_count(10usize, 2usize, 400u32, 2129872u32);
    emu.adi_no_count(11usize, 2usize, 512u32, 2129876u32);
    emu.adi_no_count(12usize, 0usize, 112u32, 2129880u32);
    emu.apc_no_count(1usize, 2129880u32, 4096u32, 2129884u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129888u32;
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
pub fn block_0x00207fe0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 512u32, 2129892u32);
    emu.adi_no_count(11usize, 2usize, 140u32, 2129896u32);
    emu.adi_no_count(12usize, 2usize, 400u32, 2129900u32);
    emu.adi_no_count(13usize, 2usize, 208u32, 2129904u32);
    emu.apc_no_count(1usize, 2129904u32, 4294946816u32, 2129908u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129912u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966460u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207ff8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 512u32, 2129916u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2130472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208228));
    } else {
        emu.pc = 2129920u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208000));
    }
}
#[inline(always)]
pub fn block_0x00208000(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 272u32, 2129924u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2129928u32);
    emu.adi_no_count(12usize, 19usize, 0u32, 2129932u32);
    emu.apc_no_count(1usize, 2129932u32, 0u32, 2129936u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129940u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965396u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208014(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 304u32, 2129944u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2129948u32);
    emu.adi_no_count(12usize, 22usize, 0u32, 2129952u32);
    emu.apc_no_count(1usize, 2129952u32, 57344u32, 2129956u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129960u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1060u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208028(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 304u32, 2129964u32);
    emu.adi_no_count(11usize, 2usize, 0u32, 2129968u32);
    emu.adi_no_count(12usize, 0usize, 32u32, 2129972u32);
    emu.apc_no_count(1usize, 2129972u32, 94208u32, 2129976u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129980u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966484u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020803c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2130528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208260));
    } else {
        emu.pc = 2129984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208040));
    }
}
#[inline(always)]
pub fn block_0x00208040(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 336u32, 2129988u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2129992u32);
    emu.adi_no_count(12usize, 23usize, 0u32, 2129996u32);
    emu.apc_no_count(1usize, 2129996u32, 57344u32, 2130000u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130004u32;
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
pub fn block_0x00208054(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 368u32, 2130008u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2130012u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2130016u32);
    emu.apc_no_count(1usize, 2130016u32, 0u32, 2130020u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130024u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965312u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00208068(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 16u32, 2130028u32)?;
    emu.lw_no_count(11usize, 2usize, 20u32, 2130032u32)?;
    emu.lw_no_count(12usize, 2usize, 24u32, 2130036u32)?;
    emu.lw_no_count(13usize, 2usize, 28u32, 2130040u32)?;
    emu.sw_no_count(10usize, 2usize, 416u32, 2130044u32)?;
    emu.sw_no_count(11usize, 2usize, 420u32, 2130048u32)?;
    emu.sw_no_count(12usize, 2usize, 424u32, 2130052u32)?;
    emu.sw_no_count(13usize, 2usize, 428u32, 2130056u32)?;
    emu.lw_no_count(10usize, 2usize, 0u32, 2130060u32)?;
    emu.lw_no_count(11usize, 2usize, 4u32, 2130064u32)?;
    emu.lw_no_count(12usize, 2usize, 8u32, 2130068u32)?;
    emu.lw_no_count(13usize, 2usize, 12u32, 2130072u32)?;
    emu.sw_no_count(10usize, 2usize, 400u32, 2130076u32)?;
    emu.sw_no_count(11usize, 2usize, 404u32, 2130080u32)?;
    emu.sw_no_count(12usize, 2usize, 408u32, 2130084u32)?;
    emu.sw_no_count(13usize, 2usize, 412u32, 2130088u32)?;
    emu.apc_no_count(1usize, 2130088u32, 0u32, 2130092u32);
    emu.add_memory_rw_events(18usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130096u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(792u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002080b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2130100u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1452u32, 2130104u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2130108u32);
    emu.adi_no_count(12usize, 0usize, 12u32, 2130112u32);
    emu.apc_no_count(1usize, 2130112u32, 4096u32, 2130116u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130120u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(156u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002080c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2130596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002082a4));
    } else {
        emu.pc = 2130124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002080cc));
    }
}
#[inline(always)]
pub fn block_0x002080cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 10usize, 0u32, 2130128u32);
    emu.apc_no_count(1usize, 2130128u32, 0u32, 2130132u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130136u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(752u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002080d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2130140u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1452u32, 2130144u32);
    emu.adi_no_count(18usize, 0usize, 1u32, 2130148u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2130152u32);
    emu.adi_no_count(12usize, 0usize, 11u32, 2130156u32);
    emu.adi_no_count(19usize, 0usize, 11u32, 2130160u32);
    emu.apc_no_count(1usize, 2130160u32, 4096u32, 2130164u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130168u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(108u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002080f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2130680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002082f8));
    } else {
        emu.pc = 2130172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002080fc));
    }
}
#[inline(always)]
pub fn block_0x002080fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2130176u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2130180u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1332u32, 2130184u32);
    emu.adi_no_count(12usize, 0usize, 11u32, 2130188u32);
    emu.apc_no_count(1usize, 2130188u32, 4096u32, 2130192u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130196u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966312u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208114(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(19usize, 8usize, 0u32, 2130200u32)?;
    emu.sw_no_count(9usize, 8usize, 4u32, 2130204u32)?;
    emu.sw_no_count(19usize, 8usize, 8u32, 2130208u32)?;
    emu.adi_no_count(10usize, 2usize, 524u32, 2130212u32);
    emu.adi_no_count(11usize, 2usize, 400u32, 2130216u32);
    emu.adi_no_count(12usize, 0usize, 32u32, 2130220u32);
    emu.apc_no_count(1usize, 2130220u32, 4096u32, 2130224u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130228u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966280u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208134(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 556u32, 2130232u32);
    emu.adi_no_count(11usize, 2usize, 368u32, 2130236u32);
    emu.adi_no_count(12usize, 0usize, 32u32, 2130240u32);
    emu.apc_no_count(1usize, 2130240u32, 4096u32, 2130244u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130248u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966260u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208148(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 588u32, 2130252u32);
    emu.adi_no_count(11usize, 2usize, 272u32, 2130256u32);
    emu.adi_no_count(12usize, 0usize, 32u32, 2130260u32);
    emu.apc_no_count(1usize, 2130260u32, 4096u32, 2130264u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130268u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966240u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020815c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 2usize, 512u32, 2130272u32)?;
    emu.sw_no_count(8usize, 2usize, 516u32, 2130276u32)?;
    emu.sw_no_count(18usize, 2usize, 520u32, 2130280u32)?;
    emu.adi_no_count(10usize, 2usize, 620u32, 2130284u32);
    emu.adi_no_count(11usize, 2usize, 336u32, 2130288u32);
    emu.adi_no_count(12usize, 0usize, 32u32, 2130292u32);
    emu.apc_no_count(1usize, 2130292u32, 4096u32, 2130296u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130300u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966208u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020817c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 512u32, 2130304u32);
    emu.apc_no_count(1usize, 2130304u32, 4294963200u32, 2130308u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130312u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1640u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00208188(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 716u32, 2130316u32)?;
    emu.lw_no_count(8usize, 2usize, 712u32, 2130320u32)?;
    emu.lw_no_count(9usize, 2usize, 708u32, 2130324u32)?;
    emu.lw_no_count(18usize, 2usize, 704u32, 2130328u32)?;
    emu.lw_no_count(19usize, 2usize, 700u32, 2130332u32)?;
    emu.lw_no_count(20usize, 2usize, 696u32, 2130336u32)?;
    emu.lw_no_count(21usize, 2usize, 692u32, 2130340u32)?;
    emu.lw_no_count(22usize, 2usize, 688u32, 2130344u32)?;
    emu.lw_no_count(23usize, 2usize, 684u32, 2130348u32)?;
    emu.lw_no_count(24usize, 2usize, 680u32, 2130352u32)?;
    emu.lw_no_count(25usize, 2usize, 676u32, 2130356u32)?;
    emu.lw_no_count(26usize, 2usize, 672u32, 2130360u32)?;
    emu.lw_no_count(27usize, 2usize, 668u32, 2130364u32)?;
    emu.adi_no_count(2usize, 2usize, 720u32, 2130368u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130372u32;
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
pub fn block_0x002081c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 2usize, 512u32, 2130376u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2130380u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1424u32, 2130384u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2130388u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1344u32, 2130392u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2130396u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1436u32, 2130400u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2130404u32);
    emu.adi_no_count(12usize, 2usize, 512u32, 2130408u32);
    emu.apc_no_count(1usize, 2130408u32, 81920u32, 2130412u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130416u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1700u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002081f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 516u32, 2130420u32)?;
    emu.lw_no_count(11usize, 2usize, 520u32, 2130424u32)?;
    emu.sw_no_count(10usize, 2usize, 400u32, 2130428u32)?;
    emu.sw_no_count(11usize, 2usize, 404u32, 2130432u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2130436u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1392u32, 2130440u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2130444u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1344u32, 2130448u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2130452u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1408u32, 2130456u32);
    emu.adi_no_count(11usize, 0usize, 13u32, 2130460u32);
    emu.adi_no_count(12usize, 2usize, 400u32, 2130464u32);
    emu.apc_no_count(1usize, 2130464u32, 81920u32, 2130468u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130472u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1644u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00208228(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 516u32, 2130476u32)?;
    emu.lw_no_count(11usize, 2usize, 520u32, 2130480u32)?;
    emu.sw_no_count(10usize, 2usize, 400u32, 2130484u32)?;
    emu.sw_no_count(11usize, 2usize, 404u32, 2130488u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2130492u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1360u32, 2130496u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2130500u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1344u32, 2130504u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2130508u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1376u32, 2130512u32);
    emu.adi_no_count(11usize, 0usize, 15u32, 2130516u32);
    emu.adi_no_count(12usize, 2usize, 400u32, 2130520u32);
    emu.apc_no_count(1usize, 2130520u32, 81920u32, 2130524u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130528u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1588u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00208260(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2130532u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1308u32, 2130536u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2130540u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2130544u32);
    emu.sw_no_count(10usize, 2usize, 512u32, 2130548u32)?;
    emu.sw_no_count(11usize, 2usize, 516u32, 2130552u32)?;
    emu.sw_no_count(12usize, 2usize, 520u32, 2130556u32)?;
    emu.sw_no_count(0usize, 2usize, 524u32, 2130560u32)?;
    emu.sw_no_count(0usize, 2usize, 528u32, 2130564u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2130568u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1316u32, 2130572u32);
    emu.adi_no_count(11usize, 2usize, 304u32, 2130576u32);
    emu.adi_no_count(12usize, 2usize, 0u32, 2130580u32);
    emu.adi_no_count(13usize, 2usize, 512u32, 2130584u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2130588u32);
    emu.apc_no_count(1usize, 2130588u32, 4294959104u32, 2130592u32);
    emu.add_memory_rw_events(17usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130596u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966484u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002082a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2130600u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2130604u32);
    emu.apc_no_count(1usize, 2130604u32, 73728u32, 2130608u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130612u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(540u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002082b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 2usize, 512u32, 2130616u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2130620u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2130640u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002082d0));
}
#[inline(always)]
pub fn block_0x002082bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(8usize, 2usize, 512u32, 2130624u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2130628u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2130640u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002082d0));
}
#[inline(always)]
pub fn block_0x002082c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(20usize, 2usize, 512u32, 2130632u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2130636u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2130640u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002082d0));
}
#[inline(always)]
pub fn block_0x002082cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(21usize, 2usize, 512u32, 2130640u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2130640u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002082d0));
}
#[inline]
pub fn block_0x002082d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2130644u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1232u32, 2130648u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2130652u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1180u32, 2130656u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2130660u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1256u32, 2130664u32);
    emu.adi_no_count(11usize, 0usize, 22u32, 2130668u32);
    emu.adi_no_count(12usize, 2usize, 512u32, 2130672u32);
    emu.apc_no_count(1usize, 2130672u32, 81920u32, 2130676u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130680u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1436u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002082f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2130684u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1272u32, 2130688u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2130692u32);
    emu.adi_no_count(11usize, 0usize, 11u32, 2130696u32);
    emu.apc_no_count(1usize, 2130696u32, 73728u32, 2130700u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130704u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(424u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208310(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2130708u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1452u32, 2130712u32);
    emu.adi_no_count(13usize, 10usize, 0u32, 2130716u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2130720u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2130724u32);
    emu.apc_no_count(6usize, 2130724u32, 4096u32, 2130728u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2130732u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966840u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
