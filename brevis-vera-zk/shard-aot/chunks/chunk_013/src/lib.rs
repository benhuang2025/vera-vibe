pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2151028u32;
pub const PC_MAX: u32 = 2153012u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 106usize] = [
        block_0x0020d274,
        block_0x0020d284,
        block_0x0020d28c,
        block_0x0020d294,
        block_0x0020d2b4,
        block_0x0020d2d0,
        block_0x0020d2e8,
        block_0x0020d2ec,
        block_0x0020d2fc,
        block_0x0020d300,
        block_0x0020d308,
        block_0x0020d33c,
        block_0x0020d350,
        block_0x0020d360,
        block_0x0020d36c,
        block_0x0020d370,
        block_0x0020d390,
        block_0x0020d3a0,
        block_0x0020d3f4,
        block_0x0020d3f8,
        block_0x0020d414,
        block_0x0020d418,
        block_0x0020d420,
        block_0x0020d434,
        block_0x0020d440,
        block_0x0020d450,
        block_0x0020d454,
        block_0x0020d458,
        block_0x0020d488,
        block_0x0020d4a4,
        block_0x0020d4ac,
        block_0x0020d4b4,
        block_0x0020d4bc,
        block_0x0020d4c4,
        block_0x0020d4d4,
        block_0x0020d4fc,
        block_0x0020d508,
        block_0x0020d530,
        block_0x0020d538,
        block_0x0020d53c,
        block_0x0020d554,
        block_0x0020d584,
        block_0x0020d594,
        block_0x0020d5a4,
        block_0x0020d5d4,
        block_0x0020d5e4,
        block_0x0020d5f4,
        block_0x0020d608,
        block_0x0020d628,
        block_0x0020d63c,
        block_0x0020d650,
        block_0x0020d67c,
        block_0x0020d684,
        block_0x0020d6a0,
        block_0x0020d6c4,
        block_0x0020d6c8,
        block_0x0020d6cc,
        block_0x0020d6e0,
        block_0x0020d6f8,
        block_0x0020d700,
        block_0x0020d718,
        block_0x0020d71c,
        block_0x0020d730,
        block_0x0020d748,
        block_0x0020d750,
        block_0x0020d768,
        block_0x0020d794,
        block_0x0020d798,
        block_0x0020d7b4,
        block_0x0020d7b8,
        block_0x0020d7c0,
        block_0x0020d7cc,
        block_0x0020d7d4,
        block_0x0020d7f0,
        block_0x0020d80c,
        block_0x0020d820,
        block_0x0020d838,
        block_0x0020d840,
        block_0x0020d858,
        block_0x0020d85c,
        block_0x0020d870,
        block_0x0020d888,
        block_0x0020d890,
        block_0x0020d8a8,
        block_0x0020d8c8,
        block_0x0020d8d0,
        block_0x0020d8d4,
        block_0x0020d8f4,
        block_0x0020d8f8,
        block_0x0020d904,
        block_0x0020d90c,
        block_0x0020d918,
        block_0x0020d920,
        block_0x0020d958,
        block_0x0020d960,
        block_0x0020d98c,
        block_0x0020d998,
        block_0x0020d9a0,
        block_0x0020d9ac,
        block_0x0020d9c0,
        block_0x0020d9dc,
        block_0x0020d9e8,
        block_0x0020d9f0,
        block_0x0020d9fc,
        block_0x0020da18,
        block_0x0020da34,
    ];
    const IDX: [u16; 497usize] = [
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
        0u16, 0u16, 0u16, 42u16, 0u16, 0u16, 0u16, 43u16, 0u16, 0u16, 0u16, 44u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 45u16, 0u16, 0u16,
        0u16, 46u16, 0u16, 0u16, 0u16, 47u16, 0u16, 0u16, 0u16, 0u16, 48u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 49u16, 0u16, 0u16, 0u16, 0u16, 50u16, 0u16, 0u16,
        0u16, 0u16, 51u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        52u16, 0u16, 53u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 54u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 55u16, 56u16, 57u16, 0u16, 0u16, 0u16, 0u16, 58u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 59u16, 0u16, 60u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        61u16, 62u16, 0u16, 0u16, 0u16, 0u16, 63u16, 0u16, 0u16, 0u16, 0u16, 0u16, 64u16,
        0u16, 65u16, 0u16, 0u16, 0u16, 0u16, 0u16, 66u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 67u16, 68u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        69u16, 70u16, 0u16, 71u16, 0u16, 0u16, 72u16, 0u16, 73u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 74u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 75u16, 0u16, 0u16,
        0u16, 0u16, 76u16, 0u16, 0u16, 0u16, 0u16, 0u16, 77u16, 0u16, 78u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 79u16, 80u16, 0u16, 0u16, 0u16, 0u16, 81u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 82u16, 0u16, 83u16, 0u16, 0u16, 0u16, 0u16, 0u16, 84u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 85u16, 0u16, 86u16, 87u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 88u16, 89u16, 0u16, 0u16, 90u16, 0u16, 91u16, 0u16, 0u16,
        92u16, 0u16, 93u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 94u16, 0u16, 95u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 96u16, 0u16, 0u16, 97u16, 0u16, 98u16, 0u16, 0u16, 99u16, 0u16,
        0u16, 0u16, 0u16, 100u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 101u16, 0u16, 0u16,
        102u16, 0u16, 103u16, 0u16, 0u16, 104u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        105u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 106u16,
    ];
    if pc < 2151028u32 || pc > 2153012u32 {
        return None;
    }
    let word_offset = ((pc - 2151028u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x0020d274(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 10usize, 0u32, 2151032u32);
    emu.lbu_no_count(12usize, 10usize, 5u32, 2151036u32);
    emu.lbu_no_count(10usize, 10usize, 4u32, 2151040u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2151168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d300));
    } else {
        emu.pc = 2151044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d284));
    }
}
#[inline(always)]
pub fn block_0x0020d284(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 1u32, 2151048u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2151060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d294));
    } else {
        emu.pc = 2151052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d28c));
    }
}
#[inline(always)]
pub fn block_0x0020d28c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2151056u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2151060u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2151164u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d2fc));
}
#[inline(always)]
pub fn block_0x0020d294(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2151064u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2151068u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2151072u32)?;
    emu.adi_no_count(8usize, 11usize, 0u32, 2151076u32);
    emu.lw_no_count(10usize, 11usize, 0u32, 2151080u32)?;
    emu.lbu_no_count(11usize, 10usize, 10u32, 2151084u32);
    emu.ani_no_count(11usize, 11usize, 128u32, 2151088u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2151120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d2d0));
    } else {
        emu.pc = 2151092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d2b4));
    }
}
#[inline(always)]
pub fn block_0x0020d2b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 4u32, 2151096u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2151100u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2151104u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2151108u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1057u32, 2151112u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2151116u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2151120u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2151144u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d2e8));
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
    emu.lw_no_count(11usize, 10usize, 4u32, 2151124u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2151128u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2151132u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2151136u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1056u32, 2151140u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2151144u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2151144u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d2e8));
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
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2151148u32;
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
pub fn block_0x0020d2ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 8usize, 0u32, 2151152u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2151156u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2151160u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2151164u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2151164u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d2fc));
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
    emu.sb_no_count(10usize, 11usize, 4u32, 2151168u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2151168u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d300));
}
#[inline(always)]
pub fn block_0x0020d300(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 1u32, 2151172u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2151176u32;
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
pub fn block_0x0020d308(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2151180u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2151184u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2151188u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2151192u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2151196u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2151200u32)?;
    emu.sw_no_count(20usize, 2usize, 40u32, 2151204u32)?;
    emu.sw_no_count(21usize, 2usize, 36u32, 2151208u32)?;
    emu.adi_no_count(9usize, 11usize, 0u32, 2151212u32);
    emu.lbu_no_count(11usize, 10usize, 8u32, 2151216u32);
    emu.lw_no_count(19usize, 10usize, 0u32, 2151220u32)?;
    emu.adi_no_count(8usize, 0usize, 1u32, 2151224u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2151512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d458));
    } else {
        emu.pc = 2151228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d33c));
    }
}
#[inline(always)]
pub fn block_0x0020d33c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 10usize, 4u32, 2151232u32)?;
    emu.lbu_no_count(11usize, 18usize, 10u32, 2151236u32);
    emu.ani_no_count(11usize, 11usize, 128u32, 2151240u32);
    emu.adi_no_count(20usize, 10usize, 0u32, 2151244u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2151276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d36c));
    } else {
        emu.pc = 2151248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d350));
    }
}
#[inline(always)]
pub fn block_0x0020d350(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(13usize, 0usize, 19usize, 2151252u32);
    emu.adi_no_count(13usize, 13usize, 1u32, 2151256u32);
    emu.adi_no_count(21usize, 12usize, 0u32, 2151260u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2151448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d418));
    } else {
        emu.pc = 2151264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d360));
    }
}
#[inline(always)]
pub fn block_0x0020d360(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2151268u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1047u32, 2151272u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2151276u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2151456u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d420));
}
#[inline(always)]
pub fn block_0x0020d36c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2151328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d3a0));
    } else {
        emu.pc = 2151280u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d370));
    }
}
#[inline(always)]
pub fn block_0x0020d370(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 18usize, 4u32, 2151284u32)?;
    emu.lw_no_count(10usize, 18usize, 0u32, 2151288u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2151292u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2151296u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1060u32, 2151300u32);
    emu.adi_no_count(21usize, 12usize, 0u32, 2151304u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2151308u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2151312u32;
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
pub fn block_0x0020d390(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 21usize, 0u32, 2151316u32);
    emu.adi_no_count(11usize, 10usize, 0u32, 2151320u32);
    emu.adi_no_count(10usize, 20usize, 0u32, 2151324u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2151512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d458));
    } else {
        emu.pc = 2151328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d3a0));
    }
}
#[inline]
pub fn block_0x0020d3a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 0usize, 1u32, 2151332u32);
    emu.adi_no_count(10usize, 2usize, 19u32, 2151336u32);
    emu.lw_no_count(11usize, 18usize, 0u32, 2151340u32)?;
    emu.lw_no_count(13usize, 18usize, 4u32, 2151344u32)?;
    emu.lw_no_count(14usize, 18usize, 8u32, 2151348u32)?;
    emu.lw_no_count(15usize, 18usize, 12u32, 2151352u32)?;
    emu.adi_no_count(16usize, 2usize, 4u32, 2151356u32);
    emu.sw_no_count(11usize, 2usize, 4u32, 2151360u32)?;
    emu.sw_no_count(13usize, 2usize, 8u32, 2151364u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2151368u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2151372u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1020u32, 2151376u32);
    emu.lw_no_count(12usize, 12usize, 12u32, 2151380u32)?;
    emu.sb_no_count(8usize, 2usize, 19u32, 2151384u32);
    emu.sw_no_count(16usize, 2usize, 20u32, 2151388u32)?;
    emu.sw_no_count(10usize, 2usize, 24u32, 2151392u32)?;
    emu.sw_no_count(14usize, 2usize, 28u32, 2151396u32)?;
    emu.sw_no_count(15usize, 2usize, 32u32, 2151400u32)?;
    emu.adi_no_count(11usize, 2usize, 20u32, 2151404u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2151408u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2151412u32;
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
pub fn block_0x0020d3f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2151508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d454));
    } else {
        emu.pc = 2151416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d3f8));
    }
}
#[inline(always)]
pub fn block_0x0020d3f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 24u32, 2151420u32)?;
    emu.lw_no_count(10usize, 2usize, 20u32, 2151424u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2151428u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2151432u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1054u32, 2151436u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2151440u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2151444u32;
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
pub fn block_0x0020d414(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2151448u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2151504u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d450));
}
#[inline(always)]
pub fn block_0x0020d418(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2151452u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1059u32, 2151456u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2151456u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d420));
}
#[inline(always)]
pub fn block_0x0020d420(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 4u32, 2151460u32)?;
    emu.lw_no_count(10usize, 18usize, 0u32, 2151464u32)?;
    emu.lw_no_count(14usize, 12usize, 12u32, 2151468u32)?;
    emu.adi_no_count(12usize, 13usize, 0u32, 2151472u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2151476u32;
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
pub fn block_0x0020d434(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 10usize, 0u32, 2151480u32);
    emu.adi_no_count(10usize, 20usize, 0u32, 2151484u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2151512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d458));
    } else {
        emu.pc = 2151488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d440));
    }
}
#[inline(always)]
pub fn block_0x0020d440(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 21usize, 12u32, 2151492u32)?;
    emu.adi_no_count(10usize, 9usize, 0u32, 2151496u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2151500u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2151504u32;
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
pub fn block_0x0020d450(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 10usize, 0u32, 2151508u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2151508u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d454));
}
#[inline(always)]
pub fn block_0x0020d454(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 20usize, 0u32, 2151512u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2151512u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d458));
}
#[inline]
pub fn block_0x0020d458(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 19usize, 1u32, 2151516u32);
    emu.sw_no_count(19usize, 10usize, 0u32, 2151520u32)?;
    emu.sb_no_count(8usize, 10usize, 8u32, 2151524u32);
    emu.lw_no_count(1usize, 2usize, 60u32, 2151528u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2151532u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2151536u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2151540u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2151544u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2151548u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2151552u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2151556u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2151560u32;
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
pub fn block_0x0020d488(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2151564u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2151568u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2151572u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2151576u32)?;
    emu.lw_no_count(11usize, 10usize, 0u32, 2151580u32)?;
    emu.lbu_no_count(8usize, 10usize, 8u32, 2151584u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2151740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d53c));
    } else {
        emu.pc = 2151588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d4a4));
    }
}
#[inline(always)]
pub fn block_0x0020d4a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(8usize, 8usize, 1u32, 2151592u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2151604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d4b4));
    } else {
        emu.pc = 2151596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d4ac));
    }
}
#[inline(always)]
pub fn block_0x0020d4ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 0usize, 1u32, 2151600u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2151604u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2151736u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d538));
}
#[inline(always)]
pub fn block_0x0020d4b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 1u32, 2151608u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2151688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d508));
    } else {
        emu.pc = 2151612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d4bc));
    }
}
#[inline(always)]
pub fn block_0x0020d4bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 10usize, 9u32, 2151616u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2151688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d508));
    } else {
        emu.pc = 2151620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d4c4));
    }
}
#[inline(always)]
pub fn block_0x0020d4c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 4u32, 2151624u32)?;
    emu.lbu_no_count(12usize, 11usize, 10u32, 2151628u32);
    emu.ani_no_count(12usize, 12usize, 128u32, 2151632u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2151688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d508));
    } else {
        emu.pc = 2151636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d4d4));
    }
}
#[inline]
pub fn block_0x0020d4d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 4u32, 2151640u32)?;
    emu.lw_no_count(13usize, 11usize, 0u32, 2151644u32)?;
    emu.lw_no_count(14usize, 12usize, 12u32, 2151648u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2151652u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1063u32, 2151656u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2151660u32);
    emu.adi_no_count(8usize, 0usize, 1u32, 2151664u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2151668u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2151672u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2151676u32;
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
pub fn block_0x0020d4fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 10usize, 0u32, 2151680u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2151684u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2151736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d538));
    } else {
        emu.pc = 2151688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d508));
    }
}
#[inline]
pub fn block_0x0020d508(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 4u32, 2151692u32)?;
    emu.lw_no_count(12usize, 11usize, 4u32, 2151696u32)?;
    emu.lw_no_count(13usize, 11usize, 0u32, 2151700u32)?;
    emu.lw_no_count(14usize, 12usize, 12u32, 2151704u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2151708u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1062u32, 2151712u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2151716u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2151720u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2151724u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2151728u32;
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
pub fn block_0x0020d530(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 10usize, 0u32, 2151732u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2151736u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2151736u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d538));
}
#[inline(always)]
pub fn block_0x0020d538(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(8usize, 10usize, 8u32, 2151740u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2151740u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d53c));
}
#[inline(always)]
pub fn block_0x0020d53c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 8usize, 1u32, 2151744u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2151748u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2151752u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2151756u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2151760u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2151764u32;
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
pub fn block_0x0020d554(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294966192u32, 2151768u32);
    emu.sw_no_count(1usize, 2usize, 1100u32, 2151772u32)?;
    emu.sw_no_count(8usize, 2usize, 1096u32, 2151776u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2151780u32);
    emu.adi_no_count(5usize, 0usize, 4u32, 2151784u32);
    emu.adi_no_count(10usize, 2usize, 1080u32, 2151788u32);
    emu.adi_no_count(15usize, 2usize, 8u32, 2151792u32);
    emu.adi_no_count(16usize, 0usize, 1024u32, 2151796u32);
    emu.adi_no_count(17usize, 2usize, 1032u32, 2151800u32);
    emu.sw_no_count(5usize, 2usize, 0u32, 2151804u32)?;
    emu.apc_no_count(1usize, 2151804u32, 4096u32, 2151808u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2151812u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965292u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020d584(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 2usize, 1080u32, 2151816u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2151820u32);
    emu.apc_no_count(1usize, 2151820u32, 12288u32, 2151824u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2151828u32;
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
pub fn block_0x0020d594(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 1100u32, 2151832u32)?;
    emu.lw_no_count(8usize, 2usize, 1096u32, 2151836u32)?;
    emu.adi_no_count(2usize, 2usize, 1104u32, 2151840u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2151844u32;
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
pub fn block_0x0020d5a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967200u32, 2151848u32);
    emu.sw_no_count(1usize, 2usize, 92u32, 2151852u32)?;
    emu.sw_no_count(8usize, 2usize, 88u32, 2151856u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2151860u32);
    emu.adi_no_count(5usize, 0usize, 4u32, 2151864u32);
    emu.adi_no_count(10usize, 2usize, 72u32, 2151868u32);
    emu.adi_no_count(15usize, 2usize, 7u32, 2151872u32);
    emu.adi_no_count(16usize, 0usize, 17u32, 2151876u32);
    emu.adi_no_count(17usize, 2usize, 24u32, 2151880u32);
    emu.sw_no_count(5usize, 2usize, 0u32, 2151884u32)?;
    emu.apc_no_count(1usize, 2151884u32, 0u32, 2151888u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2151892u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1156u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020d5d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 2usize, 72u32, 2151896u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2151900u32);
    emu.apc_no_count(1usize, 2151900u32, 12288u32, 2151904u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2151908u32;
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
pub fn block_0x0020d5e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 92u32, 2151912u32)?;
    emu.lw_no_count(8usize, 2usize, 88u32, 2151916u32)?;
    emu.adi_no_count(2usize, 2usize, 96u32, 2151920u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2151924u32;
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
pub fn block_0x0020d5f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 0u32, 2151928u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2151932u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1020u32, 2151936u32);
    emu.apc_no_count(6usize, 2151936u32, 12288u32, 2151940u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2151944u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965704u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020d608(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 11usize, 0u32, 2151948u32);
    emu.lw_no_count(13usize, 11usize, 8u32, 2151952u32)?;
    emu.lw_no_count(11usize, 10usize, 0u32, 2151956u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2151960u32)?;
    let a = 0u32.wrapping_add(2097152u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2151964u32;
    emu.update_insn_clock();
    emu.sli_no_count(14usize, 13usize, 3u32, 2151968u32);
    emu.anr_no_count(10usize, 13usize, 10usize, 2151972u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2151996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d63c));
    } else {
        emu.pc = 2151976u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d628));
    }
}
#[inline(always)]
pub fn block_0x0020d628(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(13usize, 0usize, 10usize, 2151980u32);
    emu.adi_no_count(10usize, 15usize, 0u32, 2151984u32);
    emu.adi_no_count(14usize, 0usize, 0u32, 2151988u32);
    emu.apc_no_count(6usize, 2151988u32, 0u32, 2151992u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2151996u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294967152u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020d63c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lhu_no_count(14usize, 15usize, 14u32, 2152000u32)?;
    emu.sltru_no_count(13usize, 0usize, 10usize, 2152004u32);
    emu.adi_no_count(10usize, 15usize, 0u32, 2152008u32);
    emu.apc_no_count(6usize, 2152008u32, 0u32, 2152012u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2152016u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294967052u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020d650(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967136u32, 2152020u32);
    emu.sw_no_count(1usize, 2usize, 156u32, 2152024u32)?;
    emu.sw_no_count(8usize, 2usize, 152u32, 2152028u32)?;
    emu.sw_no_count(9usize, 2usize, 148u32, 2152032u32)?;
    emu.sw_no_count(18usize, 2usize, 144u32, 2152036u32)?;
    emu.sw_no_count(19usize, 2usize, 140u32, 2152040u32)?;
    emu.adi_no_count(8usize, 11usize, 0u32, 2152044u32);
    emu.lw_no_count(11usize, 11usize, 8u32, 2152048u32)?;
    emu.sli_no_count(12usize, 11usize, 6u32, 2152052u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2152056u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2152140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d6cc));
    } else {
        emu.pc = 2152060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d67c));
    }
}
#[inline(always)]
pub fn block_0x0020d67c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 11usize, 5u32, 2152064u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2152220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d71c));
    } else {
        emu.pc = 2152068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d684));
    }
}
#[inline(always)]
pub fn block_0x0020d684(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 0u32, 2152072u32)?;
    emu.adi_no_count(18usize, 2usize, 12u32, 2152076u32);
    emu.adi_no_count(11usize, 2usize, 12u32, 2152080u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2152084u32);
    emu.adi_no_count(19usize, 0usize, 10u32, 2152088u32);
    emu.apc_no_count(1usize, 2152088u32, 4294963200u32, 2152092u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2152096u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965528u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020d6a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(15usize, 19usize, 10usize, 2152100u32);
    emu.adr_no_count(14usize, 18usize, 10usize, 2152104u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2152108u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2152112u32);
    emu.adi_no_count(18usize, 0usize, 1u32, 2152116u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2152120u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2152124u32);
    emu.apc_no_count(1usize, 2152124u32, 12288u32, 2152128u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2152132u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966160u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020d6c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2152344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d798));
    } else {
        emu.pc = 2152136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d6c8));
    }
}
#[inline(always)]
pub fn block_0x0020d6c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2152140u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2152660u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d8d4));
}
#[inline(always)]
pub fn block_0x0020d6cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 0u32, 2152144u32);
    emu.lw_no_count(10usize, 9usize, 0u32, 2152148u32)?;
    emu.adi_no_count(11usize, 2usize, 139u32, 2152152u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2152156u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2152160u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2152184u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d6f8));
}
#[inline(always)]
pub fn block_0x0020d6e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 87u32, 2152164u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2152168u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2152172u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2152176u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2152180u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2152296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d768));
    } else {
        emu.pc = 2152184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d6f8));
    }
}
#[inline(always)]
pub fn block_0x0020d6f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(13usize, 10usize, 15u32, 2152188u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2152160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d6e0));
    } else {
        emu.pc = 2152192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d700));
    }
}
#[inline(always)]
pub fn block_0x0020d700(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 48u32, 2152196u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2152200u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2152204u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2152208u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2152212u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2152184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d6f8));
    } else {
        emu.pc = 2152216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d718));
    }
}
#[inline(always)]
pub fn block_0x0020d718(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2152220u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2152296u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d768));
}
#[inline(always)]
pub fn block_0x0020d71c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 0u32, 2152224u32);
    emu.lw_no_count(10usize, 9usize, 0u32, 2152228u32)?;
    emu.adi_no_count(11usize, 2usize, 139u32, 2152232u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2152236u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2152240u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2152264u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d748));
}
#[inline(always)]
pub fn block_0x0020d730(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 55u32, 2152244u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2152248u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2152252u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2152256u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2152260u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2152296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d768));
    } else {
        emu.pc = 2152264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d748));
    }
}
#[inline(always)]
pub fn block_0x0020d748(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(13usize, 10usize, 15u32, 2152268u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2152240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d730));
    } else {
        emu.pc = 2152272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d750));
    }
}
#[inline(always)]
pub fn block_0x0020d750(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 48u32, 2152276u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2152280u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2152284u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2152288u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2152292u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2152264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d748));
    } else {
        emu.pc = 2152296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d768));
    }
}
#[inline]
pub fn block_0x0020d768(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2152300u32);
    emu.sbr_no_count(10usize, 10usize, 15usize, 2152304u32);
    emu.adi_no_count(14usize, 10usize, 128u32, 2152308u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2152312u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 336u32, 2152316u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2152320u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2152324u32);
    emu.adi_no_count(18usize, 0usize, 1u32, 2152328u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2152332u32);
    emu.apc_no_count(1usize, 2152332u32, 12288u32, 2152336u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2152340u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965952u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020d794(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2152660u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d8d4));
    } else {
        emu.pc = 2152344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d798));
    }
}
#[inline(always)]
pub fn block_0x0020d798(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 8usize, 4u32, 2152348u32)?;
    emu.lw_no_count(10usize, 8usize, 0u32, 2152352u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2152356u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2152360u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1064u32, 2152364u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2152368u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2152372u32;
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
pub fn block_0x0020d7b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2152384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d7c0));
    } else {
        emu.pc = 2152376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d7b8));
    }
}
#[inline(always)]
pub fn block_0x0020d7b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 1u32, 2152380u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2152384u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2152660u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d8d4));
}
#[inline(always)]
pub fn block_0x0020d7c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 8u32, 2152388u32)?;
    emu.sli_no_count(11usize, 10usize, 6u32, 2152392u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2152460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d80c));
    } else {
        emu.pc = 2152396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d7cc));
    }
}
#[inline(always)]
pub fn block_0x0020d7cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 10usize, 5u32, 2152400u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2152540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d85c));
    } else {
        emu.pc = 2152404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d7d4));
    }
}
#[inline(always)]
pub fn block_0x0020d7d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 4u32, 2152408u32)?;
    emu.adi_no_count(9usize, 2usize, 12u32, 2152412u32);
    emu.adi_no_count(11usize, 2usize, 12u32, 2152416u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2152420u32);
    emu.adi_no_count(18usize, 0usize, 10u32, 2152424u32);
    emu.apc_no_count(1usize, 2152424u32, 4294959104u32, 2152428u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2152432u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1992u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020d7f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(15usize, 18usize, 10usize, 2152436u32);
    emu.adr_no_count(14usize, 9usize, 10usize, 2152440u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2152444u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2152448u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2152452u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2152456u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2152460u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2152648u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d8c8));
}
#[inline(always)]
pub fn block_0x0020d80c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 0u32, 2152464u32);
    emu.lw_no_count(10usize, 9usize, 4u32, 2152468u32)?;
    emu.adi_no_count(11usize, 2usize, 139u32, 2152472u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2152476u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2152480u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2152504u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d838));
}
#[inline(always)]
pub fn block_0x0020d820(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 87u32, 2152484u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2152488u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2152492u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2152496u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2152500u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2152616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d8a8));
    } else {
        emu.pc = 2152504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d838));
    }
}
#[inline(always)]
pub fn block_0x0020d838(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(13usize, 10usize, 15u32, 2152508u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2152480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d820));
    } else {
        emu.pc = 2152512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d840));
    }
}
#[inline(always)]
pub fn block_0x0020d840(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 48u32, 2152516u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2152520u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2152524u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2152528u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2152532u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2152504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d838));
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
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2152540u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2152616u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d8a8));
}
#[inline(always)]
pub fn block_0x0020d85c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 0u32, 2152544u32);
    emu.lw_no_count(10usize, 9usize, 4u32, 2152548u32)?;
    emu.adi_no_count(11usize, 2usize, 139u32, 2152552u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2152556u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2152560u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2152584u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d888));
}
#[inline(always)]
pub fn block_0x0020d870(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 55u32, 2152564u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2152568u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2152572u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2152576u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2152580u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2152616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d8a8));
    } else {
        emu.pc = 2152584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d888));
    }
}
#[inline(always)]
pub fn block_0x0020d888(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(13usize, 10usize, 15u32, 2152588u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2152560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d870));
    } else {
        emu.pc = 2152592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d890));
    }
}
#[inline(always)]
pub fn block_0x0020d890(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 48u32, 2152596u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2152600u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2152604u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2152608u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2152612u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2152584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d888));
    } else {
        emu.pc = 2152616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d8a8));
    }
}
#[inline(always)]
pub fn block_0x0020d8a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2152620u32);
    emu.sbr_no_count(10usize, 10usize, 15usize, 2152624u32);
    emu.adi_no_count(14usize, 10usize, 128u32, 2152628u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2152632u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 336u32, 2152636u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2152640u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2152644u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2152648u32);
    emu.add_memory_rw_events(8usize);
    emu.pc = 2152648u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d8c8));
}
#[inline(always)]
pub fn block_0x0020d8c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2152648u32, 12288u32, 2152652u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2152656u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965636u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020d8d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2152660u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2152660u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d8d4));
}
#[inline(always)]
pub fn block_0x0020d8d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2152664u32);
    emu.lw_no_count(1usize, 2usize, 156u32, 2152668u32)?;
    emu.lw_no_count(8usize, 2usize, 152u32, 2152672u32)?;
    emu.lw_no_count(9usize, 2usize, 148u32, 2152676u32)?;
    emu.lw_no_count(18usize, 2usize, 144u32, 2152680u32)?;
    emu.lw_no_count(19usize, 2usize, 140u32, 2152684u32)?;
    emu.adi_no_count(2usize, 2usize, 160u32, 2152688u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2152692u32;
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
pub fn block_0x0020d8f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2152956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d9fc));
    } else {
        emu.pc = 2152696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d8f8));
    }
}
#[inline(always)]
pub fn block_0x0020d8f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(16usize, 10usize, 0u32, 2152700u32);
    emu.adi_no_count(17usize, 0usize, 48u32, 2152704u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2152984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020da18));
    } else {
        emu.pc = 2152708u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d904));
    }
}
#[inline(always)]
pub fn block_0x0020d904(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 3u32, 2152712u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a >= b {
        emu.pc = 2153012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020da34));
    } else {
        emu.pc = 2152716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d90c));
    }
}
#[inline(always)]
pub fn block_0x0020d90c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 2u32, 2152720u32);
    emu.sh_no_count(16usize, 14usize, 0u32, 2152724u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(0usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2152800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d960));
    } else {
        emu.pc = 2152728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d918));
    }
}
#[inline(always)]
pub fn block_0x0020d918(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 14usize, 4u32, 2152732u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a >= b {
        emu.pc = 2152876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d9ac));
    } else {
        emu.pc = 2152736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d920));
    }
}
#[inline]
pub fn block_0x0020d920(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 2u32, 2152740u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2152744u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 1236u32, 2152748u32);
    emu.adi_no_count(17usize, 0usize, 1u32, 2152752u32);
    emu.sbr_no_count(11usize, 11usize, 12usize, 2152756u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2152760u32);
    emu.sw_no_count(12usize, 14usize, 8u32, 2152764u32)?;
    emu.sh_no_count(15usize, 14usize, 12u32, 2152768u32)?;
    emu.sw_no_count(16usize, 14usize, 16u32, 2152772u32)?;
    emu.sw_no_count(17usize, 14usize, 20u32, 2152776u32)?;
    emu.sh_no_count(15usize, 14usize, 24u32, 2152780u32)?;
    emu.sw_no_count(10usize, 14usize, 28u32, 2152784u32)?;
    emu.sw_no_count(11usize, 14usize, 32u32, 2152788u32)?;
    emu.add_memory_rw_events(13usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2152864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d9a0));
    } else {
        emu.pc = 2152792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d958));
    }
}
#[inline(always)]
pub fn block_0x0020d958(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(13usize, 13usize, 11usize, 2152796u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2152800u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2152924u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d9dc));
}
#[inline]
pub fn block_0x0020d960(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(15usize, 0usize, 12usize, 2152804u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2152808u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 1237u32, 2152812u32);
    emu.sh_no_count(16usize, 14usize, 24u32, 2152816u32)?;
    emu.sw_no_count(10usize, 14usize, 28u32, 2152820u32)?;
    emu.sw_no_count(11usize, 14usize, 32u32, 2152824u32)?;
    emu.sw_no_count(17usize, 14usize, 4u32, 2152828u32)?;
    emu.sw_no_count(16usize, 14usize, 8u32, 2152832u32)?;
    emu.sh_no_count(0usize, 14usize, 12u32, 2152836u32)?;
    emu.sw_no_count(15usize, 14usize, 16u32, 2152840u32)?;
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2152864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d9a0));
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
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(13usize, 13usize, 11usize, 2152848u32);
    emu.adi_no_count(11usize, 0usize, 3u32, 2152852u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2152936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d9e8));
    } else {
        emu.pc = 2152856u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d998));
    }
}
#[inline(always)]
pub fn block_0x0020d998(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(13usize, 13usize, 12usize, 2152860u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2152864u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2152924u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d9dc));
}
#[inline(always)]
pub fn block_0x0020d9a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 3u32, 2152868u32);
    emu.adi_no_count(10usize, 14usize, 0u32, 2152872u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2152876u32;
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
pub fn block_0x0020d9ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(12usize, 12usize, 11usize, 2152880u32);
    emu.sw_no_count(11usize, 14usize, 8u32, 2152884u32)?;
    emu.sh_no_count(0usize, 14usize, 12u32, 2152888u32)?;
    emu.sw_no_count(12usize, 14usize, 16u32, 2152892u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2152944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d9f0));
    } else {
        emu.pc = 2152896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d9c0));
    }
}
#[inline(always)]
pub fn block_0x0020d9c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2152900u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2152904u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1236u32, 2152908u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2152912u32);
    emu.sh_no_count(10usize, 14usize, 24u32, 2152916u32)?;
    emu.sw_no_count(11usize, 14usize, 28u32, 2152920u32)?;
    emu.sw_no_count(12usize, 14usize, 32u32, 2152924u32)?;
    emu.add_memory_rw_events(7usize);
    emu.pc = 2152924u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d9dc));
}
#[inline(always)]
pub fn block_0x0020d9dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sh_no_count(0usize, 14usize, 36u32, 2152928u32)?;
    emu.sw_no_count(13usize, 14usize, 40u32, 2152932u32)?;
    emu.adi_no_count(11usize, 0usize, 4u32, 2152936u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2152936u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d9e8));
}
#[inline(always)]
pub fn block_0x0020d9e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 14usize, 0u32, 2152940u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2152944u32;
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
pub fn block_0x0020d9f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 2u32, 2152948u32);
    emu.adi_no_count(10usize, 14usize, 0u32, 2152952u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2152956u32;
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
pub fn block_0x0020d9fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2152960u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1084u32, 2152964u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2152968u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1120u32, 2152972u32);
    emu.adi_no_count(11usize, 0usize, 33u32, 2152976u32);
    emu.apc_no_count(1usize, 2152976u32, 4294963200u32, 2152980u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2152984u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965860u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020da18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2152988u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1136u32, 2152992u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2152996u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1168u32, 2153000u32);
    emu.adi_no_count(11usize, 0usize, 31u32, 2153004u32);
    emu.apc_no_count(1usize, 2153004u32, 4294963200u32, 2153008u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2153012u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965832u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020da34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2153016u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1184u32, 2153020u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2153024u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1220u32, 2153028u32);
    emu.adi_no_count(11usize, 0usize, 34u32, 2153032u32);
    emu.apc_no_count(1usize, 2153032u32, 4294963200u32, 2153036u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2153040u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965804u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
