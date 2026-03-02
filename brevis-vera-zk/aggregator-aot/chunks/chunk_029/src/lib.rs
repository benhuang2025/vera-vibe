pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2222336u32;
pub const PC_MAX: u32 = 2224792u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 110usize] = [
        block_0x0021e900,
        block_0x0021e908,
        block_0x0021e910,
        block_0x0021e918,
        block_0x0021e920,
        block_0x0021e928,
        block_0x0021e95c,
        block_0x0021e964,
        block_0x0021e998,
        block_0x0021ea78,
        block_0x0021ea80,
        block_0x0021ea88,
        block_0x0021ea90,
        block_0x0021eaa4,
        block_0x0021eaac,
        block_0x0021eab0,
        block_0x0021eac8,
        block_0x0021eacc,
        block_0x0021ead8,
        block_0x0021eae4,
        block_0x0021eaec,
        block_0x0021eaf0,
        block_0x0021eb08,
        block_0x0021eb10,
        block_0x0021eb18,
        block_0x0021eb34,
        block_0x0021eb50,
        block_0x0021eb84,
        block_0x0021eb88,
        block_0x0021eb9c,
        block_0x0021eba0,
        block_0x0021eba8,
        block_0x0021ebb4,
        block_0x0021ec30,
        block_0x0021ec34,
        block_0x0021ec58,
        block_0x0021ec80,
        block_0x0021eca4,
        block_0x0021ed04,
        block_0x0021ed64,
        block_0x0021edc4,
        block_0x0021ede8,
        block_0x0021edfc,
        block_0x0021ee00,
        block_0x0021ee04,
        block_0x0021ee08,
        block_0x0021ee0c,
        block_0x0021ee20,
        block_0x0021ee34,
        block_0x0021ee4c,
        block_0x0021ee64,
        block_0x0021ee68,
        block_0x0021ee84,
        block_0x0021ee94,
        block_0x0021eeac,
        block_0x0021eeb0,
        block_0x0021eebc,
        block_0x0021eec0,
        block_0x0021eecc,
        block_0x0021eed0,
        block_0x0021eee4,
        block_0x0021eef8,
        block_0x0021ef10,
        block_0x0021ef28,
        block_0x0021ef2c,
        block_0x0021ef6c,
        block_0x0021ef80,
        block_0x0021ef90,
        block_0x0021ef98,
        block_0x0021efa4,
        block_0x0021efa8,
        block_0x0021efb4,
        block_0x0021efbc,
        block_0x0021efd0,
        block_0x0021efe4,
        block_0x0021f004,
        block_0x0021f008,
        block_0x0021f03c,
        block_0x0021f054,
        block_0x0021f070,
        block_0x0021f08c,
        block_0x0021f090,
        block_0x0021f0a8,
        block_0x0021f0c4,
        block_0x0021f0e0,
        block_0x0021f0e4,
        block_0x0021f0f8,
        block_0x0021f0fc,
        block_0x0021f114,
        block_0x0021f128,
        block_0x0021f138,
        block_0x0021f144,
        block_0x0021f14c,
        block_0x0021f170,
        block_0x0021f184,
        block_0x0021f1dc,
        block_0x0021f1e0,
        block_0x0021f1e8,
        block_0x0021f1ec,
        block_0x0021f1f0,
        block_0x0021f204,
        block_0x0021f21c,
        block_0x0021f230,
        block_0x0021f240,
        block_0x0021f24c,
        block_0x0021f254,
        block_0x0021f278,
        block_0x0021f27c,
        block_0x0021f288,
        block_0x0021f298,
    ];
    const IDX: [u16; 615usize] = [
        1u16, 0u16, 2u16, 0u16, 3u16, 0u16, 4u16, 0u16, 5u16, 0u16, 6u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 7u16, 0u16, 8u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 9u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 10u16, 0u16, 11u16, 0u16, 12u16, 0u16, 13u16, 0u16, 0u16, 0u16,
        0u16, 14u16, 0u16, 15u16, 16u16, 0u16, 0u16, 0u16, 0u16, 0u16, 17u16, 18u16,
        0u16, 0u16, 19u16, 0u16, 0u16, 20u16, 0u16, 21u16, 22u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 23u16, 0u16, 24u16, 0u16, 25u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 26u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 27u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 28u16, 29u16, 0u16, 0u16, 0u16, 0u16, 30u16,
        31u16, 0u16, 32u16, 0u16, 0u16, 33u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 34u16, 35u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 36u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 37u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        38u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 39u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 40u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 41u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 42u16, 0u16, 0u16, 0u16, 0u16, 43u16, 44u16, 45u16, 46u16,
        47u16, 0u16, 0u16, 0u16, 0u16, 48u16, 0u16, 0u16, 0u16, 0u16, 49u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 50u16, 0u16, 0u16, 0u16, 0u16, 0u16, 51u16, 52u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 53u16, 0u16, 0u16, 0u16, 54u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 55u16, 56u16, 0u16, 0u16, 57u16, 58u16, 0u16, 0u16, 59u16, 60u16, 0u16,
        0u16, 0u16, 0u16, 61u16, 0u16, 0u16, 0u16, 0u16, 62u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 63u16, 0u16, 0u16, 0u16, 0u16, 0u16, 64u16, 65u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 66u16, 0u16,
        0u16, 0u16, 0u16, 67u16, 0u16, 0u16, 0u16, 68u16, 0u16, 69u16, 0u16, 0u16, 70u16,
        71u16, 0u16, 0u16, 72u16, 0u16, 73u16, 0u16, 0u16, 0u16, 0u16, 74u16, 0u16, 0u16,
        0u16, 0u16, 75u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 76u16, 77u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 78u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 79u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 80u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 81u16, 82u16, 0u16, 0u16, 0u16, 0u16, 0u16, 83u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 84u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        85u16, 86u16, 0u16, 0u16, 0u16, 0u16, 87u16, 88u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        89u16, 0u16, 0u16, 0u16, 0u16, 90u16, 0u16, 0u16, 0u16, 91u16, 0u16, 0u16, 92u16,
        0u16, 93u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 94u16, 0u16, 0u16,
        0u16, 0u16, 95u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 96u16, 97u16,
        0u16, 98u16, 99u16, 100u16, 0u16, 0u16, 0u16, 0u16, 101u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 102u16, 0u16, 0u16, 0u16, 0u16, 103u16, 0u16, 0u16, 0u16, 104u16,
        0u16, 0u16, 105u16, 0u16, 106u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        107u16, 108u16, 0u16, 0u16, 109u16, 0u16, 0u16, 0u16, 110u16,
    ];
    if pc < 2222336u32 || pc > 2224792u32 {
        return None;
    }
    let word_offset = ((pc - 2222336u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x0021e900(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 32u32, 2222340u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2222352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e910));
    } else {
        emu.pc = 2222344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e908));
    }
}
#[inline(always)]
pub fn block_0x0021e908(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2222348u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2222352u32;
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
pub fn block_0x0021e910(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 127u32, 2222356u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2222368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e920));
    } else {
        emu.pc = 2222360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e918));
    }
}
#[inline(always)]
pub fn block_0x0021e918(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2222364u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2222368u32;
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
pub fn block_0x0021e920(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 10usize, 16u32, 2222372u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2222428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e95c));
    } else {
        emu.pc = 2222376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e928));
    }
}
#[inline]
pub fn block_0x0021e928(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 10usize, 16u32, 2222380u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2222384u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1538u32, 2222388u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2222392u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1618u32, 2222396u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2222400u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 1908u32, 2222404u32);
    emu.adi_no_count(12usize, 0usize, 40u32, 2222408u32);
    emu.adi_no_count(14usize, 0usize, 290u32, 2222412u32);
    emu.sri_no_count(10usize, 10usize, 16u32, 2222416u32);
    emu.adi_no_count(16usize, 0usize, 297u32, 2222420u32);
    emu.apc_no_count(6usize, 2222420u32, 0u32, 2222424u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2222428u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966964u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021e95c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 10usize, 17u32, 2222432u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2222488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e998));
    } else {
        emu.pc = 2222436u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e964));
    }
}
#[inline]
pub fn block_0x0021e964(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 10usize, 16u32, 2222440u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2222444u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 756u32, 2222448u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2222452u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 844u32, 2222456u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2222460u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 1052u32, 2222464u32);
    emu.adi_no_count(12usize, 0usize, 44u32, 2222468u32);
    emu.adi_no_count(14usize, 0usize, 208u32, 2222472u32);
    emu.sri_no_count(10usize, 10usize, 16u32, 2222476u32);
    emu.adi_no_count(16usize, 0usize, 486u32, 2222480u32);
    emu.apc_no_count(6usize, 2222480u32, 0u32, 2222484u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2222488u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966904u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0021e998(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 56u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2097152u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2222492u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(172032u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2222496u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4294791168u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2222500u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4294782976u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2222504u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4294774784u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2222508u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4294770688u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2222512u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4294766592u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2222516u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4294049792u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2222520u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(917504u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2222524u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967264u32, 2222528u32);
    emu.adi_no_count(12usize, 12usize, 1760u32, 2222532u32);
    emu.adi_no_count(13usize, 13usize, 4294965440u32, 2222536u32);
    emu.adi_no_count(14usize, 14usize, 336u32, 2222540u32);
    emu.anr_no_count(7usize, 10usize, 11usize, 2222544u32);
    emu.xrr_no_count(12usize, 7usize, 12usize, 2222548u32);
    emu.adi_no_count(7usize, 15usize, 1040u32, 2222552u32);
    emu.adi_no_count(15usize, 15usize, 4294965248u32, 2222556u32);
    emu.adr_no_count(16usize, 10usize, 16usize, 2222560u32);
    emu.adi_no_count(17usize, 17usize, 4294966448u32, 2222564u32);
    emu.adi_no_count(5usize, 5usize, 4294967040u32, 2222568u32);
    emu.adi_no_count(6usize, 6usize, 496u32, 2222572u32);
    emu.adr_no_count(13usize, 10usize, 13usize, 2222576u32);
    emu.adi_no_count(11usize, 11usize, 30u32, 2222580u32);
    emu.adr_no_count(14usize, 10usize, 14usize, 2222584u32);
    emu.adr_no_count(7usize, 10usize, 7usize, 2222588u32);
    emu.adr_no_count(15usize, 10usize, 15usize, 2222592u32);
    emu.adr_no_count(17usize, 10usize, 17usize, 2222596u32);
    emu.adr_no_count(5usize, 10usize, 5usize, 2222600u32);
    emu.sltru_no_count(6usize, 10usize, 6usize, 2222604u32);
    emu.anr_no_count(10usize, 10usize, 11usize, 2222608u32);
    let a = 0u32.wrapping_add(4294963200u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2222612u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1630u32, 2222616u32);
    emu.sltru_no_count(11usize, 15usize, 11usize, 2222620u32);
    let a = 0u32.wrapping_add(4294254592u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2222624u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 688u32, 2222628u32);
    emu.sltru_no_count(15usize, 5usize, 15usize, 2222632u32);
    let a = 0u32.wrapping_add(180224u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2222636u32;
    emu.update_insn_clock();
    emu.adi_no_count(5usize, 5usize, 4294965278u32, 2222640u32);
    emu.xrr_no_count(10usize, 10usize, 5usize, 2222644u32);
    emu.sltiu_no_count(14usize, 14usize, 4294967282u32, 2222648u32);
    emu.sltiu_no_count(5usize, 7usize, 4294967281u32, 2222652u32);
    emu.anr_no_count(14usize, 14usize, 5usize, 2222656u32);
    emu.sltiu_no_count(17usize, 17usize, 4294967291u32, 2222660u32);
    emu.anr_no_count(15usize, 17usize, 15usize, 2222664u32);
    emu.sltiu_no_count(13usize, 13usize, 4294967290u32, 2222668u32);
    emu.sltru_no_count(12usize, 0usize, 12usize, 2222672u32);
    emu.anr_no_count(12usize, 12usize, 13usize, 2222676u32);
    emu.anr_no_count(11usize, 14usize, 11usize, 2222680u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2222684u32);
    emu.anr_no_count(10usize, 12usize, 10usize, 2222688u32);
    emu.sltiu_no_count(12usize, 16usize, 4294965790u32, 2222692u32);
    emu.anr_no_count(11usize, 11usize, 12usize, 2222696u32);
    emu.anr_no_count(10usize, 10usize, 11usize, 2222700u32);
    emu.anr_no_count(11usize, 15usize, 6usize, 2222704u32);
    emu.anr_no_count(10usize, 10usize, 11usize, 2222708u32);
    emu.add_memory_rw_events(56usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2222712u32;
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
pub fn block_0x0021ea78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2222712u32, 0u32, 2222716u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2222720u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(556u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021ea80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2222720u32, 0u32, 2222724u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2222728u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(644u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021ea88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2222728u32, 0u32, 2222732u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2222736u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(732u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021ea90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2222740u32);
    emu.adi_no_count(10usize, 10usize, 3u32, 2222744u32);
    emu.ani_no_count(10usize, 10usize, 4294967292u32, 2222748u32);
    emu.sbr_no_count(5usize, 10usize, 12usize, 2222752u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2222796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eacc));
    } else {
        emu.pc = 2222756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eaa4));
    }
}
#[inline(always)]
pub fn block_0x0021eaa4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2222760u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2222792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eac8));
    } else {
        emu.pc = 2222764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eaac));
    }
}
#[inline(always)]
pub fn block_0x0021eaac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 12usize, 11usize, 2222768u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2222768u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021eab0));
}
#[inline(always)]
pub fn block_0x0021eab0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(13usize, 12usize, 0u32, 2222772u32);
    emu.adi_no_count(12usize, 12usize, 1u32, 2222776u32);
    emu.slti_no_count(13usize, 13usize, 4294967232u32, 2222780u32);
    emu.xri_no_count(13usize, 13usize, 1u32, 2222784u32);
    emu.adr_no_count(10usize, 10usize, 13usize, 2222788u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2222768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eab0));
    } else {
        emu.pc = 2222792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eac8));
    }
}
#[inline(always)]
pub fn block_0x0021eac8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2222796u32;
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
pub fn block_0x0021eacc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(13usize, 11usize, 5usize, 2222800u32);
    emu.sri_no_count(17usize, 13usize, 2u32, 2222804u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2222756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eaa4));
    } else {
        emu.pc = 2222808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ead8));
    }
}
#[inline(always)]
pub fn block_0x0021ead8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(5usize, 12usize, 5usize, 2222812u32);
    emu.ani_no_count(11usize, 13usize, 3u32, 2222816u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2222828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eaec));
    } else {
        emu.pc = 2222820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eae4));
    }
}
#[inline(always)]
pub fn block_0x0021eae4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2222824u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2222828u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2222856u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021eb08));
}
#[inline(always)]
pub fn block_0x0021eaec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2222832u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2222832u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021eaf0));
}
#[inline(always)]
pub fn block_0x0021eaf0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(14usize, 12usize, 0u32, 2222836u32);
    emu.adi_no_count(12usize, 12usize, 1u32, 2222840u32);
    emu.slti_no_count(14usize, 14usize, 4294967232u32, 2222844u32);
    emu.xri_no_count(14usize, 14usize, 1u32, 2222848u32);
    emu.adr_no_count(10usize, 10usize, 14usize, 2222852u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2222832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eaf0));
    } else {
        emu.pc = 2222856u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eb08));
    }
}
#[inline(always)]
pub fn block_0x0021eb08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2222860u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2222900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eb34));
    } else {
        emu.pc = 2222864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eb10));
    }
}
#[inline(always)]
pub fn block_0x0021eb10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(13usize, 13usize, 4294967292u32, 2222868u32);
    emu.adr_no_count(13usize, 5usize, 13usize, 2222872u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2222872u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021eb18));
}
#[inline(always)]
pub fn block_0x0021eb18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(14usize, 13usize, 0u32, 2222876u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2222880u32);
    emu.slti_no_count(14usize, 14usize, 4294967232u32, 2222884u32);
    emu.xri_no_count(14usize, 14usize, 1u32, 2222888u32);
    emu.adr_no_count(12usize, 12usize, 14usize, 2222892u32);
    emu.adi_no_count(13usize, 13usize, 1u32, 2222896u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2222872u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eb18));
    } else {
        emu.pc = 2222900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eb34));
    }
}
#[inline(always)]
pub fn block_0x0021eb34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2222904u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(16711680u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2222908u32;
    emu.update_insn_clock();
    emu.adr_no_count(10usize, 12usize, 10usize, 2222912u32);
    emu.adi_no_count(12usize, 11usize, 257u32, 2222916u32);
    emu.adi_no_count(11usize, 13usize, 255u32, 2222920u32);
    emu.adi_no_count(14usize, 0usize, 4u32, 2222924u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2222928u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2222980u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021eb84));
}
#[inline]
pub fn block_0x0021eb50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(6usize, 16usize, 2u32, 2222932u32);
    emu.sbr_no_count(17usize, 13usize, 16usize, 2222936u32);
    emu.ani_no_count(7usize, 16usize, 3u32, 2222940u32);
    emu.anr_no_count(28usize, 5usize, 11usize, 2222944u32);
    emu.sri_no_count(29usize, 5usize, 8u32, 2222948u32);
    emu.adr_no_count(5usize, 15usize, 6usize, 2222952u32);
    emu.anr_no_count(6usize, 29usize, 11usize, 2222956u32);
    emu.adr_no_count(6usize, 6usize, 28usize, 2222960u32);
    emu.sli_no_count(28usize, 6usize, 16u32, 2222964u32);
    emu.adr_no_count(6usize, 28usize, 6usize, 2222968u32);
    emu.sri_no_count(6usize, 6usize, 16u32, 2222972u32);
    emu.adr_no_count(10usize, 6usize, 10usize, 2222976u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a != b {
        emu.pc = 2223156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ec34));
    } else {
        emu.pc = 2222980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eb84));
    }
}
#[inline(always)]
pub fn block_0x0021eb84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2222792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eac8));
    } else {
        emu.pc = 2222984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eb88));
    }
}
#[inline(always)]
pub fn block_0x0021eb88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 17usize, 0u32, 2222988u32);
    emu.adi_no_count(15usize, 5usize, 0u32, 2222992u32);
    emu.adi_no_count(17usize, 0usize, 192u32, 2222996u32);
    emu.adi_no_count(16usize, 13usize, 0u32, 2223000u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2223008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eba0));
    } else {
        emu.pc = 2223004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eb9c));
    }
}
#[inline(always)]
pub fn block_0x0021eb9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 192u32, 2223008u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2223008u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021eba0));
}
#[inline(always)]
pub fn block_0x0021eba0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 0u32, 2223012u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2222928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eb50));
    } else {
        emu.pc = 2223016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eba8));
    }
}
#[inline(always)]
pub fn block_0x0021eba8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(17usize, 16usize, 2u32, 2223020u32);
    emu.sli_no_count(17usize, 17usize, 4u32, 2223024u32);
    emu.adi_no_count(6usize, 15usize, 0u32, 2223028u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2223028u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ebb4));
}
#[inline(never)]
pub fn block_0x0021ebb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 31u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(7usize, 6usize, 0u32, 2223032u32)?;
    emu.lw_no_count(28usize, 6usize, 4u32, 2223036u32)?;
    emu.lw_no_count(29usize, 6usize, 8u32, 2223040u32)?;
    emu.lw_no_count(30usize, 6usize, 12u32, 2223044u32)?;
    emu.xri_no_count(31usize, 7usize, 4294967295u32, 2223048u32);
    emu.sri_no_count(7usize, 7usize, 6u32, 2223052u32);
    emu.sri_no_count(31usize, 31usize, 7u32, 2223056u32);
    emu.orr_no_count(7usize, 31usize, 7usize, 2223060u32);
    emu.xri_no_count(31usize, 28usize, 4294967295u32, 2223064u32);
    emu.sri_no_count(28usize, 28usize, 6u32, 2223068u32);
    emu.sri_no_count(31usize, 31usize, 7u32, 2223072u32);
    emu.orr_no_count(28usize, 31usize, 28usize, 2223076u32);
    emu.xri_no_count(31usize, 29usize, 4294967295u32, 2223080u32);
    emu.sri_no_count(29usize, 29usize, 6u32, 2223084u32);
    emu.sri_no_count(31usize, 31usize, 7u32, 2223088u32);
    emu.orr_no_count(29usize, 31usize, 29usize, 2223092u32);
    emu.xri_no_count(31usize, 30usize, 4294967295u32, 2223096u32);
    emu.sri_no_count(30usize, 30usize, 6u32, 2223100u32);
    emu.sri_no_count(31usize, 31usize, 7u32, 2223104u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2223108u32);
    emu.anr_no_count(7usize, 7usize, 12usize, 2223112u32);
    emu.adr_no_count(5usize, 7usize, 5usize, 2223116u32);
    emu.adi_no_count(17usize, 17usize, 4294967280u32, 2223120u32);
    emu.anr_no_count(7usize, 28usize, 12usize, 2223124u32);
    emu.anr_no_count(28usize, 29usize, 12usize, 2223128u32);
    emu.anr_no_count(29usize, 30usize, 12usize, 2223132u32);
    emu.adr_no_count(7usize, 7usize, 28usize, 2223136u32);
    emu.adr_no_count(5usize, 5usize, 7usize, 2223140u32);
    emu.adr_no_count(5usize, 5usize, 29usize, 2223144u32);
    emu.adi_no_count(6usize, 6usize, 16u32, 2223148u32);
    emu.add_memory_rw_events(30usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a != b {
        emu.pc = 2223028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ebb4));
    } else {
        emu.pc = 2223152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ec30));
    }
}
#[inline(always)]
pub fn block_0x0021ec30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2223156u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2222928u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021eb50));
}
#[inline]
pub fn block_0x0021ec34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 0u32, 2223160u32);
    emu.ani_no_count(16usize, 16usize, 252u32, 2223164u32);
    emu.sli_no_count(16usize, 16usize, 2u32, 2223168u32);
    emu.adr_no_count(15usize, 15usize, 16usize, 2223172u32);
    emu.sltiu_no_count(16usize, 13usize, 192u32, 2223176u32);
    emu.sbr_no_count(16usize, 0usize, 16usize, 2223180u32);
    emu.anr_no_count(13usize, 13usize, 16usize, 2223184u32);
    emu.ani_no_count(13usize, 13usize, 3u32, 2223188u32);
    emu.sli_no_count(13usize, 13usize, 2u32, 2223192u32);
    emu.add_memory_rw_events(9usize);
    emu.pc = 2223192u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ec58));
}
#[inline]
pub fn block_0x0021ec58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 15usize, 0u32, 2223196u32)?;
    emu.adi_no_count(15usize, 15usize, 4u32, 2223200u32);
    emu.xri_no_count(17usize, 16usize, 4294967295u32, 2223204u32);
    emu.sri_no_count(16usize, 16usize, 6u32, 2223208u32);
    emu.sri_no_count(17usize, 17usize, 7u32, 2223212u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2223216u32);
    emu.anr_no_count(16usize, 16usize, 12usize, 2223220u32);
    emu.adi_no_count(13usize, 13usize, 4294967292u32, 2223224u32);
    emu.adr_no_count(14usize, 16usize, 14usize, 2223228u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2223192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ec58));
    } else {
        emu.pc = 2223232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ec80));
    }
}
#[inline]
pub fn block_0x0021ec80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(12usize, 14usize, 11usize, 2223236u32);
    emu.sri_no_count(14usize, 14usize, 8u32, 2223240u32);
    emu.anr_no_count(11usize, 14usize, 11usize, 2223244u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2223248u32);
    emu.sli_no_count(12usize, 11usize, 16u32, 2223252u32);
    emu.adr_no_count(11usize, 12usize, 11usize, 2223256u32);
    emu.sri_no_count(11usize, 11usize, 16u32, 2223260u32);
    emu.adr_no_count(10usize, 11usize, 10usize, 2223264u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2223268u32;
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
pub fn block_0x0021eca4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2223272u32);
    emu.sw_no_count(10usize, 2usize, 0u32, 2223276u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2223280u32)?;
    emu.adi_no_count(10usize, 2usize, 0u32, 2223284u32);
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2223288u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1064u32, 2223292u32);
    emu.adi_no_count(13usize, 2usize, 4u32, 2223296u32);
    let a = 0u32.wrapping_add(2244608u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2223300u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965460u32, 2223304u32);
    emu.adi_no_count(15usize, 0usize, 2u32, 2223308u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2223312u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2223316u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2223320u32)?;
    emu.sw_no_count(13usize, 2usize, 40u32, 2223324u32)?;
    emu.sw_no_count(11usize, 2usize, 44u32, 2223328u32)?;
    emu.adi_no_count(10usize, 2usize, 32u32, 2223332u32);
    emu.sw_no_count(14usize, 2usize, 8u32, 2223336u32)?;
    emu.sw_no_count(15usize, 2usize, 12u32, 2223340u32)?;
    emu.sw_no_count(10usize, 2usize, 16u32, 2223344u32)?;
    emu.sw_no_count(15usize, 2usize, 20u32, 2223348u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2223352u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2223356u32);
    emu.apc_no_count(1usize, 2223356u32, 4294955008u32, 2223360u32);
    emu.add_memory_rw_events(24usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2223364u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966880u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021ed04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2223368u32);
    emu.sw_no_count(10usize, 2usize, 0u32, 2223372u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2223376u32)?;
    emu.adi_no_count(10usize, 2usize, 0u32, 2223380u32);
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2223384u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1064u32, 2223388u32);
    emu.adi_no_count(13usize, 2usize, 4u32, 2223392u32);
    let a = 0u32.wrapping_add(2244608u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2223396u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965476u32, 2223400u32);
    emu.adi_no_count(15usize, 0usize, 2u32, 2223404u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2223408u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2223412u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2223416u32)?;
    emu.sw_no_count(13usize, 2usize, 40u32, 2223420u32)?;
    emu.sw_no_count(11usize, 2usize, 44u32, 2223424u32)?;
    emu.adi_no_count(10usize, 2usize, 32u32, 2223428u32);
    emu.sw_no_count(14usize, 2usize, 8u32, 2223432u32)?;
    emu.sw_no_count(15usize, 2usize, 12u32, 2223436u32)?;
    emu.sw_no_count(10usize, 2usize, 16u32, 2223440u32)?;
    emu.sw_no_count(15usize, 2usize, 20u32, 2223444u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2223448u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2223452u32);
    emu.apc_no_count(1usize, 2223452u32, 4294955008u32, 2223456u32);
    emu.add_memory_rw_events(24usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2223460u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966784u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021ed64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2223464u32);
    emu.sw_no_count(10usize, 2usize, 0u32, 2223468u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2223472u32)?;
    emu.adi_no_count(10usize, 2usize, 0u32, 2223476u32);
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2223480u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1064u32, 2223484u32);
    emu.adi_no_count(13usize, 2usize, 4u32, 2223488u32);
    let a = 0u32.wrapping_add(2244608u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2223492u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965528u32, 2223496u32);
    emu.adi_no_count(15usize, 0usize, 2u32, 2223500u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2223504u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2223508u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2223512u32)?;
    emu.sw_no_count(13usize, 2usize, 40u32, 2223516u32)?;
    emu.sw_no_count(11usize, 2usize, 44u32, 2223520u32)?;
    emu.adi_no_count(10usize, 2usize, 32u32, 2223524u32);
    emu.sw_no_count(14usize, 2usize, 8u32, 2223528u32)?;
    emu.sw_no_count(15usize, 2usize, 12u32, 2223532u32)?;
    emu.sw_no_count(10usize, 2usize, 16u32, 2223536u32)?;
    emu.sw_no_count(15usize, 2usize, 20u32, 2223540u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2223544u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2223548u32);
    emu.apc_no_count(1usize, 2223548u32, 4294955008u32, 2223552u32);
    emu.add_memory_rw_events(24usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2223556u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966688u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021edc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2223560u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2223564u32)?;
    emu.adi_no_count(14usize, 13usize, 0u32, 2223568u32);
    emu.adi_no_count(13usize, 12usize, 0u32, 2223572u32);
    emu.adi_no_count(12usize, 11usize, 0u32, 2223576u32);
    emu.adi_no_count(11usize, 10usize, 0u32, 2223580u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2223584u32);
    emu.apc_no_count(1usize, 2223584u32, 0u32, 2223588u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2223592u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(28u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021ede8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2223596u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2223600u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2223604u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2223608u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2223612u32;
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
pub fn block_0x0021edfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2223788u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eeac));
    } else {
        emu.pc = 2223616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ee00));
    }
}
#[inline(always)]
pub fn block_0x0021ee00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a != b {
        emu.pc = 2223788u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eeac));
    } else {
        emu.pc = 2223620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ee04));
    }
}
#[inline(always)]
pub fn block_0x0021ee04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2224100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021efe4));
    } else {
        emu.pc = 2223624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ee08));
    }
}
#[inline(always)]
pub fn block_0x0021ee08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a >= b {
        emu.pc = 2224132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f004));
    } else {
        emu.pc = 2223628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ee0c));
    }
}
#[inline(always)]
pub fn block_0x0021ee0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(17usize, 13usize, 16u32, 2223632u32);
    emu.sltru_no_count(15usize, 17usize, 12usize, 2223636u32);
    emu.xri_no_count(16usize, 15usize, 1u32, 2223640u32);
    emu.adi_no_count(15usize, 13usize, 0u32, 2223644u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2224272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f090));
    } else {
        emu.pc = 2223648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ee20));
    }
}
#[inline(always)]
pub fn block_0x0021ee20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(16usize, 16usize, 4u32, 2223652u32);
    emu.sri_no_count(5usize, 15usize, 8u32, 2223656u32);
    emu.sltru_no_count(17usize, 5usize, 12usize, 2223660u32);
    emu.xri_no_count(17usize, 17usize, 1u32, 2223664u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a >= b {
        emu.pc = 2224296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f0a8));
    } else {
        emu.pc = 2223668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ee34));
    }
}
#[inline(always)]
pub fn block_0x0021ee34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(5usize, 17usize, 3u32, 2223672u32);
    emu.sri_no_count(17usize, 15usize, 4u32, 2223676u32);
    emu.orr_no_count(16usize, 5usize, 16usize, 2223680u32);
    emu.sltru_no_count(5usize, 17usize, 12usize, 2223684u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2223688u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2224324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f0c4));
    } else {
        emu.pc = 2223692u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ee4c));
    }
}
#[inline(always)]
pub fn block_0x0021ee4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(5usize, 5usize, 2u32, 2223696u32);
    emu.sri_no_count(17usize, 15usize, 2u32, 2223700u32);
    emu.orr_no_count(16usize, 16usize, 5usize, 2223704u32);
    emu.sltru_no_count(5usize, 17usize, 12usize, 2223708u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2223712u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a < b {
        emu.pc = 2223720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ee68));
    } else {
        emu.pc = 2223716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ee64));
    }
}
#[inline(always)]
pub fn block_0x0021ee64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 17usize, 0u32, 2223720u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2223720u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ee68));
}
#[inline(always)]
pub fn block_0x0021ee68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(5usize, 5usize, 1u32, 2223724u32);
    emu.sri_no_count(15usize, 15usize, 1u32, 2223728u32);
    emu.sltru_no_count(15usize, 15usize, 12usize, 2223732u32);
    emu.xri_no_count(15usize, 15usize, 1u32, 2223736u32);
    emu.orr_no_count(15usize, 5usize, 15usize, 2223740u32);
    emu.orr_no_count(15usize, 16usize, 15usize, 2223744u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2224356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f0e4));
    } else {
        emu.pc = 2223748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ee84));
    }
}
#[inline(always)]
pub fn block_0x0021ee84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 31u32, 2223752u32);
    emu.adi_no_count(5usize, 16usize, 4294967264u32, 2223756u32);
    emu.slr_no_count(17usize, 13usize, 16usize, 2223760u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2224376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f0f8));
    } else {
        emu.pc = 2223764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ee94));
    }
}
#[inline(always)]
pub fn block_0x0021ee94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.slr_no_count(14usize, 14usize, 16usize, 2223768u32);
    emu.xri_no_count(15usize, 16usize, 4294967295u32, 2223772u32);
    emu.sri_no_count(6usize, 13usize, 1u32, 2223776u32);
    emu.srr_no_count(15usize, 6usize, 15usize, 2223780u32);
    emu.orr_no_count(15usize, 14usize, 15usize, 2223784u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2223788u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2224380u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f0fc));
}
#[inline(always)]
pub fn block_0x0021eeac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2223808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eec0));
    } else {
        emu.pc = 2223792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eeb0));
    }
}
#[inline(always)]
pub fn block_0x0021eeb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 12usize, 14usize, 2223796u32);
    emu.adi_no_count(17usize, 0usize, 0u32, 2223800u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2223820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eecc));
    } else {
        emu.pc = 2223804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eebc));
    }
}
#[inline(always)]
pub fn block_0x0021eebc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2223808u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2224060u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021efbc));
}
#[inline(always)]
pub fn block_0x0021eec0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 11usize, 13usize, 2223812u32);
    emu.adi_no_count(17usize, 0usize, 0u32, 2223816u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2224060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021efbc));
    } else {
        emu.pc = 2223820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eecc));
    }
}
#[inline(always)]
pub fn block_0x0021eecc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2224060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021efbc));
    } else {
        emu.pc = 2223824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eed0));
    }
}
#[inline(always)]
pub fn block_0x0021eed0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(17usize, 12usize, 16u32, 2223828u32);
    emu.sltru_no_count(15usize, 17usize, 14usize, 2223832u32);
    emu.xri_no_count(16usize, 15usize, 1u32, 2223836u32);
    emu.adi_no_count(15usize, 12usize, 0u32, 2223840u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2224188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f03c));
    } else {
        emu.pc = 2223844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eee4));
    }
}
#[inline(always)]
pub fn block_0x0021eee4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(16usize, 16usize, 4u32, 2223848u32);
    emu.sri_no_count(5usize, 15usize, 8u32, 2223852u32);
    emu.sltru_no_count(17usize, 5usize, 14usize, 2223856u32);
    emu.xri_no_count(17usize, 17usize, 1u32, 2223860u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a >= b {
        emu.pc = 2224212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f054));
    } else {
        emu.pc = 2223864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eef8));
    }
}
#[inline(always)]
pub fn block_0x0021eef8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(5usize, 17usize, 3u32, 2223868u32);
    emu.sri_no_count(17usize, 15usize, 4u32, 2223872u32);
    emu.orr_no_count(16usize, 5usize, 16usize, 2223876u32);
    emu.sltru_no_count(5usize, 17usize, 14usize, 2223880u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2223884u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2224240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f070));
    } else {
        emu.pc = 2223888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ef10));
    }
}
#[inline(always)]
pub fn block_0x0021ef10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(5usize, 5usize, 2u32, 2223892u32);
    emu.sri_no_count(17usize, 15usize, 2u32, 2223896u32);
    emu.orr_no_count(16usize, 16usize, 5usize, 2223900u32);
    emu.sltru_no_count(5usize, 17usize, 14usize, 2223904u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2223908u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a < b {
        emu.pc = 2223916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ef2c));
    } else {
        emu.pc = 2223912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ef28));
    }
}
#[inline(always)]
pub fn block_0x0021ef28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 17usize, 0u32, 2223916u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2223916u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ef2c));
}
#[inline]
pub fn block_0x0021ef2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 0usize, 0u32, 2223920u32);
    emu.sli_no_count(5usize, 5usize, 1u32, 2223924u32);
    emu.sri_no_count(15usize, 15usize, 1u32, 2223928u32);
    emu.sltru_no_count(15usize, 15usize, 14usize, 2223932u32);
    emu.xri_no_count(15usize, 15usize, 1u32, 2223936u32);
    emu.orr_no_count(15usize, 5usize, 15usize, 2223940u32);
    emu.sri_no_count(5usize, 13usize, 1u32, 2223944u32);
    emu.orr_no_count(16usize, 16usize, 15usize, 2223948u32);
    emu.xri_no_count(15usize, 16usize, 31u32, 2223952u32);
    emu.srr_no_count(15usize, 5usize, 15usize, 2223956u32);
    emu.slr_no_count(5usize, 14usize, 16usize, 2223960u32);
    emu.orr_no_count(15usize, 5usize, 15usize, 2223964u32);
    emu.adi_no_count(5usize, 0usize, 1u32, 2223968u32);
    emu.slr_no_count(5usize, 5usize, 16usize, 2223972u32);
    emu.slr_no_count(6usize, 13usize, 16usize, 2223976u32);
    emu.add_memory_rw_events(16usize);
    let return_addr = 2223980u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2224000u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ef80));
}
#[inline(always)]
pub fn block_0x0021ef6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(16usize, 6usize, 1u32, 2223984u32);
    emu.sli_no_count(6usize, 15usize, 31u32, 2223988u32);
    emu.sri_no_count(15usize, 15usize, 1u32, 2223992u32);
    emu.orr_no_count(6usize, 16usize, 6usize, 2223996u32);
    emu.sri_no_count(5usize, 5usize, 1u32, 2224000u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2224000u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ef80));
}
#[inline(always)]
pub fn block_0x0021ef80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 11usize, 6usize, 2224004u32);
    emu.sbr_no_count(7usize, 12usize, 15usize, 2224008u32);
    emu.sbr_no_count(16usize, 7usize, 16usize, 2224012u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2223980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ef6c));
    } else {
        emu.pc = 2224016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ef90));
    }
}
#[inline(always)]
pub fn block_0x0021ef90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 11usize, 6usize, 2224020u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2224040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021efa8));
    } else {
        emu.pc = 2224024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ef98));
    }
}
#[inline(always)]
pub fn block_0x0021ef98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 16usize, 14usize, 2224028u32);
    emu.orr_no_count(17usize, 5usize, 17usize, 2224032u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2224052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021efb4));
    } else {
        emu.pc = 2224036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021efa4));
    }
}
#[inline(always)]
pub fn block_0x0021efa4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2224040u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2224080u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021efd0));
}
#[inline(always)]
pub fn block_0x0021efa8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 11usize, 13usize, 2224044u32);
    emu.orr_no_count(17usize, 5usize, 17usize, 2224048u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2224080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021efd0));
    } else {
        emu.pc = 2224052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021efb4));
    }
}
#[inline(always)]
pub fn block_0x0021efb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 16usize, 0u32, 2224056u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2224060u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2223980u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ef6c));
}
#[inline(always)]
pub fn block_0x0021efbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(17usize, 10usize, 0u32, 2224064u32)?;
    emu.sw_no_count(17usize, 10usize, 4u32, 2224068u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2224072u32)?;
    emu.sw_no_count(12usize, 10usize, 12u32, 2224076u32)?;
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2224080u32;
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
pub fn block_0x0021efd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(17usize, 10usize, 0u32, 2224084u32)?;
    emu.sw_no_count(0usize, 10usize, 4u32, 2224088u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2224092u32)?;
    emu.sw_no_count(16usize, 10usize, 12u32, 2224096u32)?;
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2224100u32;
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
pub fn block_0x0021efe4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.divu_no_count(17usize, 11usize, 13usize, 2224104u32);
    emu.mul_no_count(12usize, 17usize, 13usize, 2224108u32);
    emu.sbr_no_count(11usize, 11usize, 12usize, 2224112u32);
    emu.sw_no_count(17usize, 10usize, 0u32, 2224116u32)?;
    emu.sw_no_count(0usize, 10usize, 4u32, 2224120u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2224124u32)?;
    emu.sw_no_count(0usize, 10usize, 12u32, 2224128u32)?;
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2224132u32;
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
pub fn block_0x0021f004(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2224496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f170));
    } else {
        emu.pc = 2224136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f008));
    }
}
#[inline]
pub fn block_0x0021f008(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.divu_no_count(13usize, 11usize, 12usize, 2224140u32);
    emu.mul_no_count(12usize, 13usize, 12usize, 2224144u32);
    emu.sltru_no_count(15usize, 0usize, 13usize, 2224148u32);
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2224152u32);
    emu.sbr_no_count(11usize, 11usize, 12usize, 2224156u32);
    emu.adi_no_count(17usize, 13usize, 1u32, 2224160u32);
    emu.sltiu_no_count(12usize, 17usize, 1u32, 2224164u32);
    emu.adr_no_count(15usize, 15usize, 12usize, 2224168u32);
    emu.sw_no_count(17usize, 10usize, 0u32, 2224172u32)?;
    emu.sw_no_count(15usize, 10usize, 4u32, 2224176u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2224180u32)?;
    emu.sw_no_count(0usize, 10usize, 12u32, 2224184u32)?;
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2224188u32;
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
pub fn block_0x0021f03c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 17usize, 0u32, 2224192u32);
    emu.sli_no_count(16usize, 16usize, 4u32, 2224196u32);
    emu.sri_no_count(5usize, 17usize, 8u32, 2224200u32);
    emu.sltru_no_count(17usize, 5usize, 14usize, 2224204u32);
    emu.xri_no_count(17usize, 17usize, 1u32, 2224208u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a < b {
        emu.pc = 2223864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eef8));
    } else {
        emu.pc = 2224212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f054));
    }
}
#[inline(always)]
pub fn block_0x0021f054(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 5usize, 0u32, 2224216u32);
    emu.sli_no_count(5usize, 17usize, 3u32, 2224220u32);
    emu.sri_no_count(17usize, 15usize, 4u32, 2224224u32);
    emu.orr_no_count(16usize, 5usize, 16usize, 2224228u32);
    emu.sltru_no_count(5usize, 17usize, 14usize, 2224232u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2224236u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a < b {
        emu.pc = 2223888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ef10));
    } else {
        emu.pc = 2224240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f070));
    }
}
#[inline(always)]
pub fn block_0x0021f070(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 17usize, 0u32, 2224244u32);
    emu.sli_no_count(5usize, 5usize, 2u32, 2224248u32);
    emu.sri_no_count(17usize, 17usize, 2u32, 2224252u32);
    emu.orr_no_count(16usize, 16usize, 5usize, 2224256u32);
    emu.sltru_no_count(5usize, 17usize, 14usize, 2224260u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2224264u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2223912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ef28));
    } else {
        emu.pc = 2224268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f08c));
    }
}
#[inline(always)]
pub fn block_0x0021f08c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2224272u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2223916u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ef2c));
}
#[inline(always)]
pub fn block_0x0021f090(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 17usize, 0u32, 2224276u32);
    emu.sli_no_count(16usize, 16usize, 4u32, 2224280u32);
    emu.sri_no_count(5usize, 17usize, 8u32, 2224284u32);
    emu.sltru_no_count(17usize, 5usize, 12usize, 2224288u32);
    emu.xri_no_count(17usize, 17usize, 1u32, 2224292u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a < b {
        emu.pc = 2223668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ee34));
    } else {
        emu.pc = 2224296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f0a8));
    }
}
#[inline(always)]
pub fn block_0x0021f0a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 5usize, 0u32, 2224300u32);
    emu.sli_no_count(5usize, 17usize, 3u32, 2224304u32);
    emu.sri_no_count(17usize, 15usize, 4u32, 2224308u32);
    emu.orr_no_count(16usize, 5usize, 16usize, 2224312u32);
    emu.sltru_no_count(5usize, 17usize, 12usize, 2224316u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2224320u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a < b {
        emu.pc = 2223692u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ee4c));
    } else {
        emu.pc = 2224324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f0c4));
    }
}
#[inline(always)]
pub fn block_0x0021f0c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 17usize, 0u32, 2224328u32);
    emu.sli_no_count(5usize, 5usize, 2u32, 2224332u32);
    emu.sri_no_count(17usize, 17usize, 2u32, 2224336u32);
    emu.orr_no_count(16usize, 16usize, 5usize, 2224340u32);
    emu.sltru_no_count(5usize, 17usize, 12usize, 2224344u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2224348u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2223716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ee64));
    } else {
        emu.pc = 2224352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f0e0));
    }
}
#[inline(always)]
pub fn block_0x0021f0e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2224356u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2223720u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ee68));
}
#[inline(always)]
pub fn block_0x0021f0e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 32u32, 2224360u32);
    emu.sbr_no_count(16usize, 16usize, 15usize, 2224364u32);
    emu.adi_no_count(5usize, 16usize, 4294967264u32, 2224368u32);
    emu.slr_no_count(17usize, 13usize, 16usize, 2224372u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2223764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ee94));
    } else {
        emu.pc = 2224376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f0f8));
    }
}
#[inline(always)]
pub fn block_0x0021f0f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 17usize, 0u32, 2224380u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2224380u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f0fc));
}
#[inline(always)]
pub fn block_0x0021f0fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 0u32, 2224384u32);
    emu.sai_no_count(5usize, 5usize, 1055u32, 2224388u32);
    emu.anr_no_count(17usize, 5usize, 17usize, 2224392u32);
    emu.adi_no_count(5usize, 0usize, 1u32, 2224396u32);
    emu.slr_no_count(16usize, 5usize, 16usize, 2224400u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2224404u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2224424u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f128));
}
#[inline(always)]
pub fn block_0x0021f114(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(17usize, 17usize, 1u32, 2224408u32);
    emu.sli_no_count(5usize, 15usize, 31u32, 2224412u32);
    emu.sri_no_count(15usize, 15usize, 1u32, 2224416u32);
    emu.orr_no_count(17usize, 17usize, 5usize, 2224420u32);
    emu.sri_no_count(16usize, 16usize, 1u32, 2224424u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2224424u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f128));
}
#[inline(always)]
pub fn block_0x0021f128(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 11usize, 17usize, 2224428u32);
    emu.sbr_no_count(6usize, 12usize, 15usize, 2224432u32);
    emu.sbr_no_count(5usize, 6usize, 5usize, 2224436u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2224404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f114));
    } else {
        emu.pc = 2224440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f138));
    }
}
#[inline(always)]
pub fn block_0x0021f138(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 11usize, 17usize, 2224444u32);
    emu.orr_no_count(14usize, 16usize, 14usize, 2224448u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2224460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f14c));
    } else {
        emu.pc = 2224452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f144));
    }
}
#[inline(always)]
pub fn block_0x0021f144(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 5usize, 0u32, 2224456u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2224460u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2224404u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f114));
}
#[inline]
pub fn block_0x0021f14c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.divu_no_count(12usize, 11usize, 13usize, 2224464u32);
    emu.mul_no_count(13usize, 12usize, 13usize, 2224468u32);
    emu.sbr_no_count(11usize, 11usize, 13usize, 2224472u32);
    emu.orr_no_count(17usize, 12usize, 14usize, 2224476u32);
    emu.sw_no_count(17usize, 10usize, 0u32, 2224480u32)?;
    emu.sw_no_count(0usize, 10usize, 4u32, 2224484u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2224488u32)?;
    emu.sw_no_count(0usize, 10usize, 12u32, 2224492u32)?;
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2224496u32;
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
pub fn block_0x0021f170(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(17usize, 13usize, 16u32, 2224500u32);
    emu.divu_no_count(15usize, 12usize, 13usize, 2224504u32);
    emu.mul_no_count(16usize, 15usize, 13usize, 2224508u32);
    emu.sbr_no_count(16usize, 12usize, 16usize, 2224512u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a != b {
        emu.pc = 2224604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f1dc));
    } else {
        emu.pc = 2224516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f184));
    }
}
#[inline]
pub fn block_0x0021f184(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 11usize, 16u32, 2224520u32);
    emu.sli_no_count(16usize, 16usize, 16u32, 2224524u32);
    emu.sli_no_count(11usize, 11usize, 16u32, 2224528u32);
    emu.orr_no_count(14usize, 16usize, 12usize, 2224532u32);
    emu.sri_no_count(11usize, 11usize, 16u32, 2224536u32);
    emu.divu_no_count(14usize, 14usize, 13usize, 2224540u32);
    emu.mul_no_count(16usize, 14usize, 13usize, 2224544u32);
    emu.sbr_no_count(12usize, 12usize, 16usize, 2224548u32);
    emu.sli_no_count(16usize, 14usize, 16u32, 2224552u32);
    emu.sri_no_count(14usize, 14usize, 16u32, 2224556u32);
    emu.orr_no_count(15usize, 14usize, 15usize, 2224560u32);
    emu.sli_no_count(12usize, 12usize, 16u32, 2224564u32);
    emu.orr_no_count(11usize, 12usize, 11usize, 2224568u32);
    emu.divu_no_count(12usize, 11usize, 13usize, 2224572u32);
    emu.mul_no_count(13usize, 12usize, 13usize, 2224576u32);
    emu.sbr_no_count(11usize, 11usize, 13usize, 2224580u32);
    emu.orr_no_count(17usize, 16usize, 12usize, 2224584u32);
    emu.sw_no_count(17usize, 10usize, 0u32, 2224588u32)?;
    emu.sw_no_count(15usize, 10usize, 4u32, 2224592u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2224596u32)?;
    emu.sw_no_count(0usize, 10usize, 12u32, 2224600u32)?;
    emu.add_memory_rw_events(22usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2224604u32;
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
pub fn block_0x0021f1dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2224616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f1e8));
    } else {
        emu.pc = 2224608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f1e0));
    }
}
#[inline(always)]
pub fn block_0x0021f1e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 16usize, 14usize, 2224612u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2224616u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2224620u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f1ec));
}
#[inline(always)]
pub fn block_0x0021f1e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 11usize, 13usize, 2224620u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2224620u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f1ec));
}
#[inline(always)]
pub fn block_0x0021f1ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2224644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f204));
    } else {
        emu.pc = 2224624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f1f0));
    }
}
#[inline(always)]
pub fn block_0x0021f1f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 10usize, 0u32, 2224628u32)?;
    emu.sw_no_count(15usize, 10usize, 4u32, 2224632u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2224636u32)?;
    emu.sw_no_count(16usize, 10usize, 12u32, 2224640u32)?;
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2224644u32;
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
pub fn block_0x0021f204(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(17usize, 13usize, 1u32, 2224648u32);
    emu.sli_no_count(14usize, 14usize, 31u32, 2224652u32);
    emu.sli_no_count(5usize, 13usize, 31u32, 2224656u32);
    emu.orr_no_count(14usize, 14usize, 17usize, 2224660u32);
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2224664u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(6usize);
    let return_addr = 2224668u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2224688u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f230));
}
#[inline(always)]
pub fn block_0x0021f21c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(5usize, 5usize, 1u32, 2224672u32);
    emu.sli_no_count(6usize, 14usize, 31u32, 2224676u32);
    emu.sri_no_count(14usize, 14usize, 1u32, 2224680u32);
    emu.orr_no_count(5usize, 5usize, 6usize, 2224684u32);
    emu.sri_no_count(17usize, 17usize, 1u32, 2224688u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2224688u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f230));
}
#[inline(always)]
pub fn block_0x0021f230(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(6usize, 11usize, 5usize, 2224692u32);
    emu.sbr_no_count(7usize, 16usize, 14usize, 2224696u32);
    emu.sbr_no_count(6usize, 7usize, 6usize, 2224700u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(6usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2224668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f21c));
    } else {
        emu.pc = 2224704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f240));
    }
}
#[inline(always)]
pub fn block_0x0021f240(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 11usize, 5usize, 2224708u32);
    emu.orr_no_count(12usize, 17usize, 12usize, 2224712u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2224724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f254));
    } else {
        emu.pc = 2224716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f24c));
    }
}
#[inline(always)]
pub fn block_0x0021f24c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 6usize, 0u32, 2224720u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2224724u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2224668u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f21c));
}
#[inline]
pub fn block_0x0021f254(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.divu_no_count(14usize, 11usize, 13usize, 2224728u32);
    emu.mul_no_count(13usize, 14usize, 13usize, 2224732u32);
    emu.sbr_no_count(11usize, 11usize, 13usize, 2224736u32);
    emu.orr_no_count(17usize, 14usize, 12usize, 2224740u32);
    emu.sw_no_count(17usize, 10usize, 0u32, 2224744u32)?;
    emu.sw_no_count(15usize, 10usize, 4u32, 2224748u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2224752u32)?;
    emu.sw_no_count(0usize, 10usize, 12u32, 2224756u32)?;
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2224760u32;
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
pub fn block_0x0021f278(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2224792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f298));
    } else {
        emu.pc = 2224764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f27c));
    }
}
#[inline(always)]
pub fn block_0x0021f27c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(13usize, 10usize, 0u32, 2224768u32);
    emu.lbu_no_count(14usize, 11usize, 0u32, 2224772u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2224800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2224800u32));
    } else {
        emu.pc = 2224776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f288));
    }
}
#[inline(always)]
pub fn block_0x0021f288(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2224780u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2224784u32);
    emu.adi_no_count(10usize, 10usize, 1u32, 2224788u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2224764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f27c));
    } else {
        emu.pc = 2224792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f298));
    }
}
#[inline(always)]
pub fn block_0x0021f298(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2224796u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2224800u32;
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
