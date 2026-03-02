pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2160072u32;
pub const PC_MAX: u32 = 2162576u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 113usize] = [
        block_0x0020f5c8,
        block_0x0020f5f4,
        block_0x0020f5fc,
        block_0x0020f600,
        block_0x0020f608,
        block_0x0020f618,
        block_0x0020f620,
        block_0x0020f624,
        block_0x0020f630,
        block_0x0020f640,
        block_0x0020f648,
        block_0x0020f64c,
        block_0x0020f658,
        block_0x0020f684,
        block_0x0020f68c,
        block_0x0020f690,
        block_0x0020f6ac,
        block_0x0020f6bc,
        block_0x0020f6c0,
        block_0x0020f6c8,
        block_0x0020f6cc,
        block_0x0020f6d4,
        block_0x0020f6f4,
        block_0x0020f6fc,
        block_0x0020f700,
        block_0x0020f70c,
        block_0x0020f714,
        block_0x0020f724,
        block_0x0020f740,
        block_0x0020f748,
        block_0x0020f778,
        block_0x0020f790,
        block_0x0020f79c,
        block_0x0020f7a0,
        block_0x0020f7cc,
        block_0x0020f7d0,
        block_0x0020f7e0,
        block_0x0020f814,
        block_0x0020f830,
        block_0x0020f838,
        block_0x0020f83c,
        block_0x0020f84c,
        block_0x0020f850,
        block_0x0020f858,
        block_0x0020f85c,
        block_0x0020f86c,
        block_0x0020f888,
        block_0x0020f8a4,
        block_0x0020f8bc,
        block_0x0020f92c,
        block_0x0020f964,
        block_0x0020f984,
        block_0x0020f99c,
        block_0x0020f9ac,
        block_0x0020f9b4,
        block_0x0020f9c4,
        block_0x0020f9cc,
        block_0x0020f9d4,
        block_0x0020f9e4,
        block_0x0020f9ec,
        block_0x0020fb24,
        block_0x0020fb34,
        block_0x0020fb3c,
        block_0x0020fb4c,
        block_0x0020fb58,
        block_0x0020fb68,
        block_0x0020fb74,
        block_0x0020fb84,
        block_0x0020fb88,
        block_0x0020fb98,
        block_0x0020fccc,
        block_0x0020fce0,
        block_0x0020fce4,
        block_0x0020fcec,
        block_0x0020fcf0,
        block_0x0020fd04,
        block_0x0020fd24,
        block_0x0020fd40,
        block_0x0020fd94,
        block_0x0020fd9c,
        block_0x0020fdd8,
        block_0x0020fde4,
        block_0x0020fdf8,
        block_0x0020fdfc,
        block_0x0020fe04,
        block_0x0020fe08,
        block_0x0020fe20,
        block_0x0020fe24,
        block_0x0020fe38,
        block_0x0020fe44,
        block_0x0020fe4c,
        block_0x0020fe58,
        block_0x0020fe5c,
        block_0x0020fe60,
        block_0x0020fe8c,
        block_0x0020fe90,
        block_0x0020feb4,
        block_0x0020feb8,
        block_0x0020fec0,
        block_0x0020feec,
        block_0x0020fef8,
        block_0x0020ff0c,
        block_0x0020ff10,
        block_0x0020ff20,
        block_0x0020ff24,
        block_0x0020ff44,
        block_0x0020ff4c,
        block_0x0020ff50,
        block_0x0020ff5c,
        block_0x0020ff80,
        block_0x0020ff84,
        block_0x0020ff8c,
        block_0x0020ff90,
    ];
    const IDX: [u16; 627usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16,
        3u16, 4u16, 0u16, 5u16, 0u16, 0u16, 0u16, 6u16, 0u16, 7u16, 8u16, 0u16, 0u16,
        9u16, 0u16, 0u16, 0u16, 10u16, 0u16, 11u16, 12u16, 0u16, 0u16, 13u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 14u16, 0u16, 15u16, 16u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 17u16, 0u16, 0u16, 0u16, 18u16, 19u16, 0u16, 20u16,
        21u16, 0u16, 22u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 23u16, 0u16, 24u16,
        25u16, 0u16, 0u16, 26u16, 0u16, 27u16, 0u16, 0u16, 0u16, 28u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 29u16, 0u16, 30u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 31u16, 0u16, 0u16, 0u16, 0u16, 0u16, 32u16, 0u16, 0u16,
        33u16, 34u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 35u16,
        36u16, 0u16, 0u16, 0u16, 37u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 38u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 39u16, 0u16,
        40u16, 41u16, 0u16, 0u16, 0u16, 42u16, 43u16, 0u16, 44u16, 45u16, 0u16, 0u16,
        0u16, 46u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 47u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 48u16, 0u16, 0u16, 0u16, 0u16, 0u16, 49u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 50u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 51u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 52u16, 0u16, 0u16, 0u16, 0u16, 0u16, 53u16,
        0u16, 0u16, 0u16, 54u16, 0u16, 55u16, 0u16, 0u16, 0u16, 56u16, 0u16, 57u16, 0u16,
        58u16, 0u16, 0u16, 0u16, 59u16, 0u16, 60u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 61u16, 0u16, 0u16, 0u16, 62u16, 0u16, 63u16,
        0u16, 0u16, 0u16, 64u16, 0u16, 0u16, 65u16, 0u16, 0u16, 0u16, 66u16, 0u16, 0u16,
        67u16, 0u16, 0u16, 0u16, 68u16, 69u16, 0u16, 0u16, 0u16, 70u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 71u16, 0u16, 0u16, 0u16, 0u16,
        72u16, 73u16, 0u16, 74u16, 75u16, 0u16, 0u16, 0u16, 0u16, 76u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 77u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 78u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 79u16, 0u16, 80u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 81u16, 0u16,
        0u16, 82u16, 0u16, 0u16, 0u16, 0u16, 83u16, 84u16, 0u16, 85u16, 86u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 87u16, 88u16, 0u16, 0u16, 0u16, 0u16, 89u16, 0u16, 0u16,
        90u16, 0u16, 91u16, 0u16, 0u16, 92u16, 93u16, 94u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 95u16, 96u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 97u16, 98u16, 0u16, 99u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 100u16, 0u16, 0u16, 101u16, 0u16, 0u16, 0u16, 0u16,
        102u16, 103u16, 0u16, 0u16, 0u16, 104u16, 105u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 106u16, 0u16, 107u16, 108u16, 0u16, 0u16, 109u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 110u16, 111u16, 0u16, 112u16, 113u16,
    ];
    if pc < 2160072u32 || pc > 2162576u32 {
        return None;
    }
    let word_offset = ((pc - 2160072u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x0020f5c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2160076u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2160080u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2160084u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2160088u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2160092u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2160096u32)?;
    emu.sw_no_count(20usize, 2usize, 24u32, 2160100u32)?;
    emu.sw_no_count(21usize, 2usize, 20u32, 2160104u32)?;
    emu.sw_no_count(22usize, 2usize, 16u32, 2160108u32)?;
    emu.sw_no_count(23usize, 2usize, 12u32, 2160112u32)?;
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(29usize);
    if a == b {
        emu.pc = 2160128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f600));
    } else {
        emu.pc = 2160116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f5f4));
    }
}
#[inline(always)]
pub fn block_0x0020f5f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 31usize, 29usize, 2160120u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a != b {
        emu.pc = 2160136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f608));
    } else {
        emu.pc = 2160124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f5fc));
    }
}
#[inline(always)]
pub fn block_0x0020f5fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2160128u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2160540u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f79c));
}
#[inline(always)]
pub fn block_0x0020f600(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 30usize, 28usize, 2160132u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2160540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f79c));
    } else {
        emu.pc = 2160136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f608));
    }
}
#[inline(always)]
pub fn block_0x0020f608(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 28usize, 30usize, 2160140u32);
    emu.sbr_no_count(6usize, 29usize, 31usize, 2160144u32);
    emu.sbr_no_count(5usize, 6usize, 5usize, 2160148u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2160164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f624));
    } else {
        emu.pc = 2160152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f618));
    }
}
#[inline(always)]
pub fn block_0x0020f618(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 31usize, 5usize, 2160156u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a != b {
        emu.pc = 2160176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f630));
    } else {
        emu.pc = 2160160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f620));
    }
}
#[inline(always)]
pub fn block_0x0020f620(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2160164u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2160540u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f79c));
}
#[inline(always)]
pub fn block_0x0020f624(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(5usize, 28usize, 30usize, 2160168u32);
    emu.sltru_no_count(5usize, 30usize, 5usize, 2160172u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2160540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f79c));
    } else {
        emu.pc = 2160176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f630));
    }
}
#[inline(always)]
pub fn block_0x0020f630(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 28usize, 16usize, 2160180u32);
    emu.sbr_no_count(6usize, 29usize, 17usize, 2160184u32);
    emu.sbr_no_count(5usize, 6usize, 5usize, 2160188u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2160204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f64c));
    } else {
        emu.pc = 2160192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f640));
    }
}
#[inline(always)]
pub fn block_0x0020f640(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 17usize, 5usize, 2160196u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a != b {
        emu.pc = 2160216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f658));
    } else {
        emu.pc = 2160200u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f648));
    }
}
#[inline(always)]
pub fn block_0x0020f648(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2160204u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2160316u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f6bc));
}
#[inline(always)]
pub fn block_0x0020f64c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(5usize, 28usize, 16usize, 2160208u32);
    emu.sltru_no_count(5usize, 16usize, 5usize, 2160212u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2160316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f6bc));
    } else {
        emu.pc = 2160216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f658));
    }
}
#[inline]
pub fn block_0x0020f658(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(6usize, 16usize, 31u32, 2160220u32);
    emu.sli_no_count(7usize, 17usize, 1u32, 2160224u32);
    emu.sli_no_count(5usize, 16usize, 1u32, 2160228u32);
    emu.sri_no_count(8usize, 30usize, 31u32, 2160232u32);
    emu.orr_no_count(6usize, 7usize, 6usize, 2160236u32);
    emu.sltru_no_count(7usize, 28usize, 5usize, 2160240u32);
    emu.sbr_no_count(6usize, 29usize, 6usize, 2160244u32);
    emu.sbr_no_count(6usize, 6usize, 7usize, 2160248u32);
    emu.sli_no_count(7usize, 31usize, 1u32, 2160252u32);
    emu.orr_no_count(7usize, 7usize, 8usize, 2160256u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2160300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f6ac));
    } else {
        emu.pc = 2160260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f684));
    }
}
#[inline(always)]
pub fn block_0x0020f684(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 6usize, 7usize, 2160264u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a != b {
        emu.pc = 2160316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f6bc));
    } else {
        emu.pc = 2160268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f68c));
    }
}
#[inline(always)]
pub fn block_0x0020f68c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2160732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f85c));
    } else {
        emu.pc = 2160272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f690));
    }
}
#[inline(always)]
pub fn block_0x0020f690(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2160276u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966744u32, 2160280u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2160284u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2160288u32);
    emu.adi_no_count(12usize, 14usize, 0u32, 2160292u32);
    emu.apc_no_count(1usize, 2160292u32, 16384u32, 2160296u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160300u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(836u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020f6ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(5usize, 28usize, 5usize, 2160304u32);
    emu.sli_no_count(6usize, 30usize, 1u32, 2160308u32);
    emu.sltru_no_count(5usize, 5usize, 6usize, 2160312u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2160268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f68c));
    } else {
        emu.pc = 2160316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f6bc));
    }
}
#[inline(always)]
pub fn block_0x0020f6bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2160332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f6cc));
    } else {
        emu.pc = 2160320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f6c0));
    }
}
#[inline(always)]
pub fn block_0x0020f6c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 31usize, 17usize, 2160324u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a != b {
        emu.pc = 2160340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f6d4));
    } else {
        emu.pc = 2160328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f6c8));
    }
}
#[inline(always)]
pub fn block_0x0020f6c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2160332u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2160540u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f79c));
}
#[inline(always)]
pub fn block_0x0020f6cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 30usize, 16usize, 2160336u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2160540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f79c));
    } else {
        emu.pc = 2160340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f6d4));
    }
}
#[inline(always)]
pub fn block_0x0020f6d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 16usize, 30usize, 2160344u32);
    emu.sbr_no_count(17usize, 17usize, 31usize, 2160348u32);
    emu.sbr_no_count(16usize, 16usize, 30usize, 2160352u32);
    emu.sbr_no_count(17usize, 17usize, 5usize, 2160356u32);
    emu.sbr_no_count(5usize, 29usize, 17usize, 2160360u32);
    emu.sltru_no_count(6usize, 28usize, 16usize, 2160364u32);
    emu.sbr_no_count(5usize, 5usize, 6usize, 2160368u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2160528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f790));
    } else {
        emu.pc = 2160372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f6f4));
    }
}
#[inline(always)]
pub fn block_0x0020f6f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 17usize, 5usize, 2160376u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a != b {
        emu.pc = 2160540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f79c));
    } else {
        emu.pc = 2160380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f6fc));
    }
}
#[inline(always)]
pub fn block_0x0020f6fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2160748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f86c));
    } else {
        emu.pc = 2160384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f700));
    }
}
#[inline(always)]
pub fn block_0x0020f700(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 0u32, 2160388u32);
    emu.adr_no_count(8usize, 11usize, 13usize, 2160392u32);
    emu.adi_no_count(17usize, 0usize, 57u32, 2160396u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2160396u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f70c));
}
#[inline(always)]
pub fn block_0x0020f70c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(5usize, 13usize, 16usize, 2160400u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2160588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f7cc));
    } else {
        emu.pc = 2160404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f714));
    }
}
#[inline(always)]
pub fn block_0x0020f714(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(5usize, 8usize, 16usize, 2160408u32);
    emu.lbu_no_count(5usize, 5usize, 4294967295u32, 2160412u32);
    emu.adi_no_count(16usize, 16usize, 4294967295u32, 2160416u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2160396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f70c));
    } else {
        emu.pc = 2160420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f724));
    }
}
#[inline(always)]
pub fn block_0x0020f724(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 8usize, 16usize, 2160424u32);
    emu.lbu_no_count(15usize, 8usize, 0u32, 2160428u32);
    emu.adr_no_count(17usize, 13usize, 16usize, 2160432u32);
    emu.adi_no_count(5usize, 15usize, 1u32, 2160436u32);
    emu.adi_no_count(15usize, 17usize, 1u32, 2160440u32);
    emu.sb_no_count(5usize, 8usize, 0u32, 2160444u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2160804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f8a4));
    } else {
        emu.pc = 2160448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f740));
    }
}
#[inline(always)]
pub fn block_0x0020f740(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 4294967295u32, 2160452u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2160728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f858));
    } else {
        emu.pc = 2160456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f748));
    }
}
#[inline]
pub fn block_0x0020f748(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(15usize, 16usize, 4294967295u32, 2160460u32);
    emu.adi_no_count(16usize, 8usize, 1u32, 2160464u32);
    emu.adi_no_count(8usize, 11usize, 0u32, 2160468u32);
    emu.adi_no_count(11usize, 0usize, 48u32, 2160472u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2160476u32);
    emu.adi_no_count(10usize, 16usize, 0u32, 2160480u32);
    emu.adi_no_count(18usize, 12usize, 0u32, 2160484u32);
    emu.adi_no_count(12usize, 15usize, 0u32, 2160488u32);
    emu.adi_no_count(19usize, 13usize, 0u32, 2160492u32);
    emu.adi_no_count(20usize, 14usize, 0u32, 2160496u32);
    emu.apc_no_count(1usize, 2160496u32, 4294914048u32, 2160500u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160504u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1668u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020f778(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 8usize, 0u32, 2160508u32);
    emu.adi_no_count(14usize, 20usize, 0u32, 2160512u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2160516u32);
    emu.adi_no_count(12usize, 18usize, 0u32, 2160520u32);
    emu.adi_no_count(13usize, 19usize, 0u32, 2160524u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2160528u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2160728u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f858));
}
#[inline(always)]
pub fn block_0x0020f790(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(17usize, 28usize, 16usize, 2160532u32);
    emu.sltru_no_count(16usize, 16usize, 17usize, 2160536u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2160380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f6fc));
    } else {
        emu.pc = 2160540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f79c));
    }
}
#[inline(always)]
pub fn block_0x0020f79c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 10usize, 0u32, 2160544u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2160544u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f7a0));
}
#[inline]
pub fn block_0x0020f7a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2160548u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2160552u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2160556u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2160560u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2160564u32)?;
    emu.lw_no_count(20usize, 2usize, 24u32, 2160568u32)?;
    emu.lw_no_count(21usize, 2usize, 20u32, 2160572u32)?;
    emu.lw_no_count(22usize, 2usize, 16u32, 2160576u32)?;
    emu.lw_no_count(23usize, 2usize, 12u32, 2160580u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2160584u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160588u32;
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
pub fn block_0x0020f7cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2160688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f830));
    } else {
        emu.pc = 2160592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f7d0));
    }
}
#[inline(always)]
pub fn block_0x0020f7d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 0usize, 49u32, 2160596u32);
    emu.adi_no_count(16usize, 13usize, 4294967295u32, 2160600u32);
    emu.sb_no_count(17usize, 11usize, 0u32, 2160604u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2160696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f838));
    } else {
        emu.pc = 2160608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f7e0));
    }
}
#[inline]
pub fn block_0x0020f7e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 11usize, 1u32, 2160612u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2160616u32);
    emu.adi_no_count(11usize, 0usize, 48u32, 2160620u32);
    emu.adi_no_count(9usize, 0usize, 48u32, 2160624u32);
    emu.adi_no_count(19usize, 10usize, 0u32, 2160628u32);
    emu.adi_no_count(10usize, 17usize, 0u32, 2160632u32);
    emu.adi_no_count(20usize, 12usize, 0u32, 2160636u32);
    emu.adi_no_count(12usize, 16usize, 0u32, 2160640u32);
    emu.adi_no_count(21usize, 13usize, 0u32, 2160644u32);
    emu.adi_no_count(23usize, 14usize, 0u32, 2160648u32);
    emu.adi_no_count(22usize, 15usize, 0u32, 2160652u32);
    emu.apc_no_count(1usize, 2160652u32, 4294914048u32, 2160656u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160660u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1512u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020f814(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 22usize, 0u32, 2160664u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2160668u32);
    emu.adi_no_count(14usize, 23usize, 0u32, 2160672u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2160676u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2160680u32);
    emu.adi_no_count(13usize, 21usize, 0u32, 2160684u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2160688u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2160700u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f83c));
}
#[inline(always)]
pub fn block_0x0020f830(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 49u32, 2160692u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2160696u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2160700u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f83c));
}
#[inline(always)]
pub fn block_0x0020f838(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 48u32, 2160700u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2160700u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f83c));
}
#[inline(always)]
pub fn block_0x0020f83c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 14usize, 1u32, 2160704u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2160708u32);
    emu.sai_no_count(14usize, 14usize, 1040u32, 2160712u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2160728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f858));
    } else {
        emu.pc = 2160716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f84c));
    }
}
#[inline(always)]
pub fn block_0x0020f84c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(15usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2160728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f858));
    } else {
        emu.pc = 2160720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f850));
    }
}
#[inline(always)]
pub fn block_0x0020f850(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(9usize, 8usize, 0u32, 2160724u32);
    emu.adi_no_count(13usize, 13usize, 1u32, 2160728u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2160728u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f858));
}
#[inline(always)]
pub fn block_0x0020f858(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2160776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f888));
    } else {
        emu.pc = 2160732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f85c));
    }
}
#[inline(always)]
pub fn block_0x0020f85c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 10usize, 0u32, 2160736u32)?;
    emu.sw_no_count(13usize, 10usize, 4u32, 2160740u32)?;
    emu.sh_no_count(14usize, 10usize, 8u32, 2160744u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2160748u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2160544u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f7a0));
}
#[inline(always)]
pub fn block_0x0020f86c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2160752u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966712u32, 2160756u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2160760u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2160764u32);
    emu.adi_no_count(12usize, 14usize, 0u32, 2160768u32);
    emu.apc_no_count(1usize, 2160768u32, 16384u32, 2160772u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160776u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(360u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020f888(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2160780u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966728u32, 2160784u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2160788u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2160792u32);
    emu.adi_no_count(12usize, 14usize, 0u32, 2160796u32);
    emu.apc_no_count(1usize, 2160796u32, 16384u32, 2160800u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160804u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(332u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020f8a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2160808u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 268u32, 2160812u32);
    emu.adi_no_count(10usize, 15usize, 0u32, 2160816u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2160820u32);
    emu.apc_no_count(1usize, 2160820u32, 16384u32, 2160824u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160828u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(300u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0020f8bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2160832u32);
    emu.sw_no_count(10usize, 2usize, 8u32, 2160836u32)?;
    emu.sw_no_count(11usize, 2usize, 12u32, 2160840u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2160844u32)?;
    emu.sw_no_count(13usize, 2usize, 20u32, 2160848u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2160852u32);
    let a = 0u32.wrapping_add(2166784u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2160856u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1832u32, 2160860u32);
    emu.adi_no_count(12usize, 2usize, 16u32, 2160864u32);
    let a = 0u32.wrapping_add(2166784u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2160868u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1816u32, 2160872u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2160876u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 4294966764u32, 2160880u32);
    emu.adi_no_count(16usize, 0usize, 2u32, 2160884u32);
    emu.sw_no_count(0usize, 2usize, 40u32, 2160888u32)?;
    emu.adi_no_count(17usize, 2usize, 48u32, 2160892u32);
    emu.sw_no_count(10usize, 2usize, 48u32, 2160896u32)?;
    emu.sw_no_count(11usize, 2usize, 52u32, 2160900u32)?;
    emu.sw_no_count(12usize, 2usize, 56u32, 2160904u32)?;
    emu.sw_no_count(13usize, 2usize, 60u32, 2160908u32)?;
    emu.sw_no_count(15usize, 2usize, 24u32, 2160912u32)?;
    emu.sw_no_count(16usize, 2usize, 28u32, 2160916u32)?;
    emu.sw_no_count(17usize, 2usize, 32u32, 2160920u32)?;
    emu.sw_no_count(16usize, 2usize, 36u32, 2160924u32)?;
    emu.adi_no_count(10usize, 2usize, 24u32, 2160928u32);
    emu.adi_no_count(11usize, 14usize, 0u32, 2160932u32);
    emu.apc_no_count(1usize, 2160932u32, 4294955008u32, 2160936u32);
    emu.add_memory_rw_events(28usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160940u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965416u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f92c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2160944u32);
    emu.adi_no_count(11usize, 10usize, 0u32, 2160948u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2160952u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966808u32, 2160956u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2160960u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2160964u32)?;
    emu.adi_no_count(13usize, 0usize, 4u32, 2160968u32);
    emu.sw_no_count(10usize, 2usize, 8u32, 2160972u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2160976u32)?;
    emu.sw_no_count(13usize, 2usize, 16u32, 2160980u32)?;
    emu.sw_no_count(0usize, 2usize, 20u32, 2160984u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2160988u32);
    emu.apc_no_count(1usize, 2160988u32, 4294955008u32, 2160992u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160996u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965360u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020f964(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2161000u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2161004u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2161008u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2161012u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2161016u32)?;
    emu.adi_no_count(13usize, 0usize, 39u32, 2161020u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2161024u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2161068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f9ac));
    } else {
        emu.pc = 2161028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f984));
    }
}
#[inline(always)]
pub fn block_0x0020f984(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 11usize, 2u32, 2161032u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2161036u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966816u32, 2161040u32);
    emu.adr_no_count(10usize, 10usize, 13usize, 2161044u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2161048u32)?;
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(10usize);
    let return_addr = 2161052u32;
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
pub fn block_0x0020f99c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2161056u32);
    let a = 0u32.wrapping_add(12288u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2161060u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 92u32, 2161064u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2161068u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2161904u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fcf0));
}
#[inline(always)]
pub fn block_0x0020f9ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 92u32, 2161072u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2161092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f9c4));
    } else {
        emu.pc = 2161076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f9b4));
    }
}
#[inline(always)]
pub fn block_0x0020f9b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2161080u32);
    let a = 0u32.wrapping_add(24576u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2161084u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966364u32, 2161088u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2161092u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2161904u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fcf0));
}
#[inline(always)]
pub fn block_0x0020f9c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(12usize, 12usize, 1u32, 2161096u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2161524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fb74));
    } else {
        emu.pc = 2161100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f9cc));
    }
}
#[inline(always)]
pub fn block_0x0020f9cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 767u32, 2161104u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2161524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fb74));
    } else {
        emu.pc = 2161108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f9d4));
    }
}
#[inline(always)]
pub fn block_0x0020f9d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 11usize, 0u32, 2161112u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2161116u32);
    emu.apc_no_count(1usize, 2161116u32, 4294959104u32, 2161120u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161124u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1524u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020f9e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 9usize, 0u32, 2161128u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2161524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fb74));
    } else {
        emu.pc = 2161132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f9ec));
    }
}
#[inline(never)]
pub fn block_0x0020f9ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 78u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(14usize, 11usize, 1u32, 2161136u32);
    let a = 0u32.wrapping_add(1431654400u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2161140u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(858992640u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2161144u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(252645376u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2161148u32;
    emu.update_insn_clock();
    emu.sri_no_count(15usize, 11usize, 20u32, 2161152u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2161156u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 456u32, 2161160u32);
    emu.sli_no_count(17usize, 11usize, 12u32, 2161164u32);
    emu.sli_no_count(5usize, 11usize, 16u32, 2161168u32);
    emu.sli_no_count(6usize, 11usize, 20u32, 2161172u32);
    emu.sli_no_count(7usize, 11usize, 24u32, 2161176u32);
    emu.orr_no_count(14usize, 11usize, 14usize, 2161180u32);
    emu.ani_no_count(11usize, 11usize, 15u32, 2161184u32);
    emu.adr_no_count(15usize, 16usize, 15usize, 2161188u32);
    emu.sri_no_count(17usize, 17usize, 28u32, 2161192u32);
    emu.sri_no_count(5usize, 5usize, 28u32, 2161196u32);
    emu.sri_no_count(6usize, 6usize, 28u32, 2161200u32);
    emu.sri_no_count(7usize, 7usize, 28u32, 2161204u32);
    emu.adr_no_count(11usize, 16usize, 11usize, 2161208u32);
    emu.adr_no_count(17usize, 16usize, 17usize, 2161212u32);
    emu.adr_no_count(5usize, 16usize, 5usize, 2161216u32);
    emu.adr_no_count(6usize, 16usize, 6usize, 2161220u32);
    emu.adr_no_count(16usize, 16usize, 7usize, 2161224u32);
    emu.sri_no_count(7usize, 14usize, 2u32, 2161228u32);
    emu.orr_no_count(14usize, 14usize, 7usize, 2161232u32);
    emu.sri_no_count(7usize, 14usize, 4u32, 2161236u32);
    emu.orr_no_count(14usize, 14usize, 7usize, 2161240u32);
    emu.sri_no_count(7usize, 14usize, 8u32, 2161244u32);
    emu.orr_no_count(14usize, 14usize, 7usize, 2161248u32);
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2161252u32;
    emu.update_insn_clock();
    emu.lbu_no_count(15usize, 15usize, 0u32, 2161256u32);
    emu.lbu_no_count(17usize, 17usize, 0u32, 2161260u32);
    emu.lbu_no_count(5usize, 5usize, 0u32, 2161264u32);
    emu.lbu_no_count(6usize, 6usize, 0u32, 2161268u32);
    emu.sh_no_count(0usize, 2usize, 12u32, 2161272u32)?;
    emu.sb_no_count(0usize, 2usize, 14u32, 2161276u32);
    emu.sb_no_count(15usize, 2usize, 15u32, 2161280u32);
    emu.sb_no_count(17usize, 2usize, 16u32, 2161284u32);
    emu.adi_no_count(15usize, 0usize, 125u32, 2161288u32);
    emu.adi_no_count(13usize, 13usize, 1365u32, 2161292u32);
    emu.lbu_no_count(16usize, 16usize, 0u32, 2161296u32);
    emu.lbu_no_count(11usize, 11usize, 0u32, 2161300u32);
    emu.sb_no_count(5usize, 2usize, 17u32, 2161304u32);
    emu.sb_no_count(6usize, 2usize, 18u32, 2161308u32);
    emu.sb_no_count(16usize, 2usize, 19u32, 2161312u32);
    emu.sri_no_count(16usize, 14usize, 16u32, 2161316u32);
    emu.orr_no_count(14usize, 14usize, 16usize, 2161320u32);
    emu.xri_no_count(14usize, 14usize, 4294967295u32, 2161324u32);
    emu.sri_no_count(16usize, 14usize, 1u32, 2161328u32);
    emu.anr_no_count(13usize, 16usize, 13usize, 2161332u32);
    emu.adi_no_count(16usize, 2usize, 12u32, 2161336u32);
    emu.adi_no_count(12usize, 12usize, 819u32, 2161340u32);
    emu.ani_no_count(14usize, 14usize, 4294967294u32, 2161344u32);
    emu.sbr_no_count(14usize, 14usize, 13usize, 2161348u32);
    emu.anr_no_count(13usize, 14usize, 12usize, 2161352u32);
    emu.sri_no_count(14usize, 14usize, 2u32, 2161356u32);
    emu.anr_no_count(12usize, 14usize, 12usize, 2161360u32);
    emu.adi_no_count(14usize, 0usize, 92u32, 2161364u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2161368u32);
    emu.sri_no_count(13usize, 12usize, 4u32, 2161372u32);
    emu.adr_no_count(12usize, 12usize, 13usize, 2161376u32);
    emu.adi_no_count(13usize, 0usize, 117u32, 2161380u32);
    emu.adi_no_count(10usize, 10usize, 4294967055u32, 2161384u32);
    emu.anr_no_count(10usize, 12usize, 10usize, 2161388u32);
    emu.adi_no_count(12usize, 0usize, 123u32, 2161392u32);
    emu.adi_no_count(17usize, 7usize, 257u32, 2161396u32);
    emu.mul_no_count(10usize, 10usize, 17usize, 2161400u32);
    emu.sri_no_count(10usize, 10usize, 26u32, 2161404u32);
    emu.adi_no_count(9usize, 10usize, 4294967294u32, 2161408u32);
    emu.adr_no_count(10usize, 16usize, 10usize, 2161412u32);
    emu.adr_no_count(16usize, 16usize, 9usize, 2161416u32);
    emu.sb_no_count(14usize, 16usize, 0u32, 2161420u32);
    emu.sb_no_count(13usize, 10usize, 4294967295u32, 2161424u32);
    emu.sb_no_count(12usize, 10usize, 0u32, 2161428u32);
    emu.sb_no_count(11usize, 2usize, 20u32, 2161432u32);
    emu.sb_no_count(15usize, 2usize, 21u32, 2161436u32);
    emu.adi_no_count(11usize, 2usize, 12u32, 2161440u32);
    emu.add_memory_rw_events(78usize);
    let return_addr = 2161444u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2161868u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fccc));
}
#[inline(always)]
pub fn block_0x0020fb24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2161448u32);
    let a = 0u32.wrapping_add(28672u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2161452u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966876u32, 2161456u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2161460u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2161904u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fcf0));
}
#[inline(always)]
pub fn block_0x0020fb34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 12usize, 256u32, 2161464u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2161524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fb74));
    } else {
        emu.pc = 2161468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fb3c));
    }
}
#[inline(always)]
pub fn block_0x0020fb3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2161472u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2161476u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1884u32, 2161480u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2161484u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2161904u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fcf0));
}
#[inline(always)]
pub fn block_0x0020fb4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2161488u32);
    let a = 0u32.wrapping_add(28672u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2161492u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(3usize);
    let return_addr = 2161496u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2161900u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fcec));
}
#[inline(always)]
pub fn block_0x0020fb58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2161500u32);
    let a = 0u32.wrapping_add(28672u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2161504u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1116u32, 2161508u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2161512u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2161904u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fcf0));
}
#[inline(always)]
pub fn block_0x0020fb68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 8u32, 2161516u32);
    emu.sri_no_count(12usize, 12usize, 24u32, 2161520u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2161892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fce4));
    } else {
        emu.pc = 2161524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fb74));
    }
}
#[inline(always)]
pub fn block_0x0020fb74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 11usize, 0u32, 2161528u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2161532u32);
    emu.apc_no_count(1usize, 2161532u32, 8192u32, 2161536u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161540u32;
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
pub fn block_0x0020fb84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2161560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fb98));
    } else {
        emu.pc = 2161544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fb88));
    }
}
#[inline(always)]
pub fn block_0x0020fb88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(9usize, 8usize, 0u32, 2161548u32)?;
    emu.adi_no_count(18usize, 0usize, 129u32, 2161552u32);
    emu.adi_no_count(9usize, 0usize, 128u32, 2161556u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2161560u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2161924u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fd04));
}
#[inline(never)]
pub fn block_0x0020fb98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 77u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(14usize, 9usize, 1u32, 2161564u32);
    let a = 0u32.wrapping_add(1431654400u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2161568u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(858992640u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2161572u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(252645376u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2161576u32;
    emu.update_insn_clock();
    emu.sri_no_count(15usize, 9usize, 20u32, 2161580u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2161584u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 456u32, 2161588u32);
    emu.sli_no_count(17usize, 9usize, 12u32, 2161592u32);
    emu.sli_no_count(5usize, 9usize, 16u32, 2161596u32);
    emu.sli_no_count(6usize, 9usize, 20u32, 2161600u32);
    emu.sli_no_count(7usize, 9usize, 24u32, 2161604u32);
    emu.orr_no_count(14usize, 9usize, 14usize, 2161608u32);
    emu.ani_no_count(13usize, 9usize, 15u32, 2161612u32);
    emu.adr_no_count(15usize, 16usize, 15usize, 2161616u32);
    emu.sri_no_count(17usize, 17usize, 28u32, 2161620u32);
    emu.sri_no_count(5usize, 5usize, 28u32, 2161624u32);
    emu.sri_no_count(6usize, 6usize, 28u32, 2161628u32);
    emu.sri_no_count(7usize, 7usize, 28u32, 2161632u32);
    emu.adr_no_count(13usize, 16usize, 13usize, 2161636u32);
    emu.adr_no_count(17usize, 16usize, 17usize, 2161640u32);
    emu.adr_no_count(5usize, 16usize, 5usize, 2161644u32);
    emu.adr_no_count(6usize, 16usize, 6usize, 2161648u32);
    emu.adr_no_count(16usize, 16usize, 7usize, 2161652u32);
    emu.sri_no_count(7usize, 14usize, 2u32, 2161656u32);
    emu.orr_no_count(14usize, 14usize, 7usize, 2161660u32);
    emu.sri_no_count(7usize, 14usize, 4u32, 2161664u32);
    emu.orr_no_count(14usize, 14usize, 7usize, 2161668u32);
    emu.sri_no_count(7usize, 14usize, 8u32, 2161672u32);
    emu.orr_no_count(14usize, 14usize, 7usize, 2161676u32);
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2161680u32;
    emu.update_insn_clock();
    emu.lbu_no_count(15usize, 15usize, 0u32, 2161684u32);
    emu.lbu_no_count(17usize, 17usize, 0u32, 2161688u32);
    emu.lbu_no_count(5usize, 5usize, 0u32, 2161692u32);
    emu.lbu_no_count(6usize, 6usize, 0u32, 2161696u32);
    emu.sh_no_count(0usize, 2usize, 22u32, 2161700u32)?;
    emu.sb_no_count(0usize, 2usize, 24u32, 2161704u32);
    emu.sb_no_count(15usize, 2usize, 25u32, 2161708u32);
    emu.sb_no_count(17usize, 2usize, 26u32, 2161712u32);
    emu.adi_no_count(15usize, 0usize, 125u32, 2161716u32);
    emu.adi_no_count(12usize, 12usize, 1365u32, 2161720u32);
    emu.lbu_no_count(16usize, 16usize, 0u32, 2161724u32);
    emu.lbu_no_count(13usize, 13usize, 0u32, 2161728u32);
    emu.sb_no_count(5usize, 2usize, 27u32, 2161732u32);
    emu.sb_no_count(6usize, 2usize, 28u32, 2161736u32);
    emu.sb_no_count(16usize, 2usize, 29u32, 2161740u32);
    emu.sri_no_count(16usize, 14usize, 16u32, 2161744u32);
    emu.orr_no_count(14usize, 14usize, 16usize, 2161748u32);
    emu.xri_no_count(14usize, 14usize, 4294967295u32, 2161752u32);
    emu.sri_no_count(16usize, 14usize, 1u32, 2161756u32);
    emu.anr_no_count(12usize, 16usize, 12usize, 2161760u32);
    emu.adi_no_count(16usize, 2usize, 22u32, 2161764u32);
    emu.adi_no_count(11usize, 11usize, 819u32, 2161768u32);
    emu.ani_no_count(14usize, 14usize, 4294967294u32, 2161772u32);
    emu.sbr_no_count(14usize, 14usize, 12usize, 2161776u32);
    emu.anr_no_count(12usize, 14usize, 11usize, 2161780u32);
    emu.sri_no_count(14usize, 14usize, 2u32, 2161784u32);
    emu.anr_no_count(11usize, 14usize, 11usize, 2161788u32);
    emu.adi_no_count(14usize, 0usize, 92u32, 2161792u32);
    emu.adr_no_count(11usize, 12usize, 11usize, 2161796u32);
    emu.sri_no_count(12usize, 11usize, 4u32, 2161800u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2161804u32);
    emu.adi_no_count(12usize, 0usize, 117u32, 2161808u32);
    emu.adi_no_count(10usize, 10usize, 4294967055u32, 2161812u32);
    emu.anr_no_count(10usize, 11usize, 10usize, 2161816u32);
    emu.adi_no_count(11usize, 0usize, 123u32, 2161820u32);
    emu.adi_no_count(17usize, 7usize, 257u32, 2161824u32);
    emu.mul_no_count(10usize, 10usize, 17usize, 2161828u32);
    emu.sri_no_count(10usize, 10usize, 26u32, 2161832u32);
    emu.adi_no_count(9usize, 10usize, 4294967294u32, 2161836u32);
    emu.adr_no_count(10usize, 16usize, 10usize, 2161840u32);
    emu.adr_no_count(16usize, 16usize, 9usize, 2161844u32);
    emu.sb_no_count(14usize, 16usize, 0u32, 2161848u32);
    emu.sb_no_count(12usize, 10usize, 4294967295u32, 2161852u32);
    emu.sb_no_count(11usize, 10usize, 0u32, 2161856u32);
    emu.sb_no_count(13usize, 2usize, 30u32, 2161860u32);
    emu.sb_no_count(15usize, 2usize, 31u32, 2161864u32);
    emu.adi_no_count(11usize, 2usize, 22u32, 2161868u32);
    emu.add_memory_rw_events(77usize);
    emu.pc = 2161868u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fccc));
}
#[inline(always)]
pub fn block_0x0020fccc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 10u32, 2161872u32);
    emu.adi_no_count(18usize, 0usize, 10u32, 2161876u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2161880u32);
    emu.apc_no_count(1usize, 2161880u32, 4294914048u32, 2161884u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161888u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(532u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020fce0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2161892u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2161924u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fd04));
}
#[inline(always)]
pub fn block_0x0020fce4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2161896u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2161900u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    emu.pc = 2161900u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fcec));
}
#[inline(always)]
pub fn block_0x0020fcec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 10usize, 604u32, 2161904u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2161904u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fcf0));
}
#[inline(always)]
pub fn block_0x0020fcf0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 8usize, 0u32, 2161908u32)?;
    emu.sh_no_count(0usize, 8usize, 4u32, 2161912u32)?;
    emu.sh_no_count(0usize, 8usize, 6u32, 2161916u32)?;
    emu.sh_no_count(0usize, 8usize, 8u32, 2161920u32)?;
    emu.adi_no_count(18usize, 0usize, 2u32, 2161924u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2161924u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fd04));
}
#[inline(always)]
pub fn block_0x0020fd04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(9usize, 8usize, 12u32, 2161928u32);
    emu.sb_no_count(18usize, 8usize, 13u32, 2161932u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2161936u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2161940u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2161944u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2161948u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2161952u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161956u32;
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
pub fn block_0x0020fd24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 0u32, 2161960u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2161964u32)?;
    emu.adi_no_count(13usize, 10usize, 0u32, 2161968u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2161972u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2161976u32);
    emu.apc_no_count(6usize, 2161976u32, 0u32, 2161980u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2161984u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(8u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020fd40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2161988u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2161992u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2161996u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2162000u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2162004u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2162008u32)?;
    emu.sw_no_count(20usize, 2usize, 40u32, 2162012u32)?;
    emu.sw_no_count(21usize, 2usize, 36u32, 2162016u32)?;
    emu.sw_no_count(22usize, 2usize, 32u32, 2162020u32)?;
    emu.sw_no_count(23usize, 2usize, 28u32, 2162024u32)?;
    emu.sw_no_count(24usize, 2usize, 24u32, 2162028u32)?;
    emu.sw_no_count(25usize, 2usize, 20u32, 2162032u32)?;
    emu.adi_no_count(8usize, 12usize, 0u32, 2162036u32);
    let a = 0u32.wrapping_add(3758096384u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2162040u32;
    emu.update_insn_clock();
    emu.lw_no_count(21usize, 8usize, 16u32, 2162044u32)?;
    emu.adi_no_count(12usize, 12usize, 32u32, 2162048u32);
    emu.sw_no_count(10usize, 2usize, 4u32, 2162052u32)?;
    emu.sw_no_count(11usize, 2usize, 8u32, 2162056u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2162060u32)?;
    emu.sw_no_count(0usize, 2usize, 16u32, 2162064u32)?;
    emu.add_memory_rw_events(20usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a == b {
        emu.pc = 2162360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020feb8));
    } else {
        emu.pc = 2162068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fd94));
    }
}
#[inline(always)]
pub fn block_0x0020fd94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 8usize, 20u32, 2162072u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2162512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ff50));
    } else {
        emu.pc = 2162076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fd9c));
    }
}
#[inline]
pub fn block_0x0020fd9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 0u32, 2162080u32);
    emu.sli_no_count(12usize, 11usize, 3u32, 2162084u32);
    emu.sli_no_count(13usize, 11usize, 5u32, 2162088u32);
    emu.adi_no_count(10usize, 21usize, 24u32, 2162092u32);
    emu.lw_no_count(23usize, 8usize, 0u32, 2162096u32)?;
    emu.lw_no_count(19usize, 8usize, 8u32, 2162100u32)?;
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2162104u32);
    emu.adi_no_count(20usize, 0usize, 2u32, 2162108u32);
    emu.sbr_no_count(13usize, 13usize, 12usize, 2162112u32);
    emu.sli_no_count(11usize, 11usize, 3u32, 2162116u32);
    emu.adr_no_count(22usize, 21usize, 13usize, 2162120u32);
    emu.sri_no_count(11usize, 11usize, 3u32, 2162124u32);
    emu.adi_no_count(9usize, 11usize, 1u32, 2162128u32);
    emu.adi_no_count(23usize, 23usize, 4u32, 2162132u32);
    emu.adi_no_count(24usize, 0usize, 1u32, 2162136u32);
    emu.add_memory_rw_events(15usize);
    emu.pc = 2162136u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fdd8));
}
#[inline(always)]
pub fn block_0x0020fdd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 23usize, 0u32, 2162140u32)?;
    emu.adi_no_count(25usize, 10usize, 0u32, 2162144u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2162172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fdfc));
    } else {
        emu.pc = 2162148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fde4));
    }
}
#[inline(always)]
pub fn block_0x0020fde4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 2usize, 8u32, 2162152u32)?;
    emu.lw_no_count(10usize, 2usize, 4u32, 2162156u32)?;
    emu.lw_no_count(11usize, 23usize, 4294967292u32, 2162160u32)?;
    emu.lw_no_count(13usize, 13usize, 12u32, 2162164u32)?;
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2162168u32;
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
pub fn block_0x0020fdf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2162564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ff84));
    } else {
        emu.pc = 2162172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fdfc));
    }
}
#[inline(always)]
pub fn block_0x0020fdfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lhu_no_count(10usize, 21usize, 8u32, 2162176u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2162232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fe38));
    } else {
        emu.pc = 2162180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fe04));
    }
}
#[inline(always)]
pub fn block_0x0020fe04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2162252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fe4c));
    } else {
        emu.pc = 2162184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fe08));
    }
}
#[inline(always)]
pub fn block_0x0020fe08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 21usize, 12u32, 2162188u32)?;
    emu.sli_no_count(10usize, 10usize, 3u32, 2162192u32);
    emu.adr_no_count(10usize, 19usize, 10usize, 2162196u32);
    emu.lhu_no_count(11usize, 10usize, 4u32, 2162200u32)?;
    emu.lhu_no_count(10usize, 21usize, 0u32, 2162204u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2162244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fe44));
    } else {
        emu.pc = 2162208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fe20));
    }
}
#[inline(always)]
pub fn block_0x0020fe20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2162268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fe5c));
    } else {
        emu.pc = 2162212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fe24));
    }
}
#[inline(always)]
pub fn block_0x0020fe24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 21usize, 4u32, 2162216u32)?;
    emu.sli_no_count(10usize, 10usize, 3u32, 2162220u32);
    emu.adr_no_count(10usize, 19usize, 10usize, 2162224u32);
    emu.lhu_no_count(12usize, 10usize, 4u32, 2162228u32)?;
    emu.add_memory_rw_events(5usize);
    let return_addr = 2162232u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2162272u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fe60));
}
#[inline(always)]
pub fn block_0x0020fe38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lhu_no_count(11usize, 21usize, 10u32, 2162236u32)?;
    emu.lhu_no_count(10usize, 21usize, 0u32, 2162240u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2162208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fe20));
    } else {
        emu.pc = 2162244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fe44));
    }
}
#[inline(always)]
pub fn block_0x0020fe44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2162248u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2162252u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2162272u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fe60));
}
#[inline(always)]
pub fn block_0x0020fe4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2162256u32);
    emu.lhu_no_count(10usize, 21usize, 0u32, 2162260u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2162208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fe20));
    } else {
        emu.pc = 2162264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fe58));
    }
}
#[inline(always)]
pub fn block_0x0020fe58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2162268u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2162244u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fe44));
}
#[inline(always)]
pub fn block_0x0020fe5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lhu_no_count(12usize, 21usize, 2u32, 2162272u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2162272u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fe60));
}
#[inline]
pub fn block_0x0020fe60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 21usize, 16u32, 2162276u32)?;
    emu.lw_no_count(13usize, 21usize, 20u32, 2162280u32)?;
    emu.sli_no_count(10usize, 10usize, 3u32, 2162284u32);
    emu.adr_no_count(14usize, 19usize, 10usize, 2162288u32);
    emu.lw_no_count(10usize, 14usize, 0u32, 2162292u32)?;
    emu.lw_no_count(14usize, 14usize, 4u32, 2162296u32)?;
    emu.sw_no_count(13usize, 2usize, 12u32, 2162300u32)?;
    emu.sh_no_count(11usize, 2usize, 16u32, 2162304u32)?;
    emu.sh_no_count(12usize, 2usize, 18u32, 2162308u32)?;
    emu.adi_no_count(11usize, 2usize, 4u32, 2162312u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2162316u32;
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
pub fn block_0x0020fe8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2162564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ff84));
    } else {
        emu.pc = 2162320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fe90));
    }
}
#[inline]
pub fn block_0x0020fe90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 18usize, 1u32, 2162324u32);
    emu.xrr_no_count(10usize, 25usize, 22usize, 2162328u32);
    emu.sltiu_no_count(10usize, 10usize, 1u32, 2162332u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2162336u32);
    emu.ani_no_count(10usize, 10usize, 24u32, 2162340u32);
    emu.adr_no_count(10usize, 25usize, 10usize, 2162344u32);
    emu.adi_no_count(23usize, 23usize, 8u32, 2162348u32);
    emu.adi_no_count(21usize, 25usize, 0u32, 2162352u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2162136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fdd8));
    } else {
        emu.pc = 2162356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020feb4));
    }
}
#[inline(always)]
pub fn block_0x0020feb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2162360u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2162500u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ff44));
}
#[inline(always)]
pub fn block_0x0020feb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 12u32, 2162364u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2162512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ff50));
    } else {
        emu.pc = 2162368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fec0));
    }
}
#[inline]
pub fn block_0x0020fec0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 0u32, 2162372u32);
    emu.lw_no_count(20usize, 8usize, 0u32, 2162376u32)?;
    emu.lw_no_count(21usize, 8usize, 8u32, 2162380u32)?;
    emu.sli_no_count(19usize, 10usize, 3u32, 2162384u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2162388u32);
    emu.sli_no_count(10usize, 10usize, 3u32, 2162392u32);
    emu.sri_no_count(10usize, 10usize, 3u32, 2162396u32);
    emu.adi_no_count(9usize, 10usize, 1u32, 2162400u32);
    emu.adr_no_count(19usize, 21usize, 19usize, 2162404u32);
    emu.adi_no_count(10usize, 21usize, 8u32, 2162408u32);
    emu.adi_no_count(20usize, 20usize, 4u32, 2162412u32);
    emu.add_memory_rw_events(11usize);
    emu.pc = 2162412u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020feec));
}
#[inline(always)]
pub fn block_0x0020feec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 20usize, 0u32, 2162416u32)?;
    emu.adi_no_count(22usize, 10usize, 0u32, 2162420u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2162448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ff10));
    } else {
        emu.pc = 2162424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fef8));
    }
}
#[inline(always)]
pub fn block_0x0020fef8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 2usize, 8u32, 2162428u32)?;
    emu.lw_no_count(10usize, 2usize, 4u32, 2162432u32)?;
    emu.lw_no_count(11usize, 20usize, 4294967292u32, 2162436u32)?;
    emu.lw_no_count(13usize, 13usize, 12u32, 2162440u32)?;
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2162444u32;
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
pub fn block_0x0020ff0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2162564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ff84));
    } else {
        emu.pc = 2162448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ff10));
    }
}
#[inline(always)]
pub fn block_0x0020ff10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 21usize, 0u32, 2162452u32)?;
    emu.lw_no_count(12usize, 21usize, 4u32, 2162456u32)?;
    emu.adi_no_count(11usize, 2usize, 4u32, 2162460u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2162464u32;
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
pub fn block_0x0020ff20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2162564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ff84));
    } else {
        emu.pc = 2162468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ff24));
    }
}
#[inline(always)]
pub fn block_0x0020ff24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 18usize, 1u32, 2162472u32);
    emu.xrr_no_count(10usize, 22usize, 19usize, 2162476u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2162480u32);
    emu.sli_no_count(10usize, 10usize, 3u32, 2162484u32);
    emu.adr_no_count(10usize, 22usize, 10usize, 2162488u32);
    emu.adi_no_count(20usize, 20usize, 8u32, 2162492u32);
    emu.adi_no_count(21usize, 22usize, 0u32, 2162496u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2162412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020feec));
    } else {
        emu.pc = 2162500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ff44));
    }
}
#[inline(always)]
pub fn block_0x0020ff44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 4u32, 2162504u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a < b {
        emu.pc = 2162524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ff5c));
    } else {
        emu.pc = 2162508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ff4c));
    }
}
#[inline(always)]
pub fn block_0x0020ff4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2162512u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2162572u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ff8c));
}
#[inline(always)]
pub fn block_0x0020ff50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2162516u32);
    emu.lw_no_count(10usize, 8usize, 4u32, 2162520u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(0usize);
    if a >= b {
        emu.pc = 2162572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ff8c));
    } else {
        emu.pc = 2162524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ff5c));
    }
}
#[inline]
pub fn block_0x0020ff5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 8usize, 0u32, 2162528u32)?;
    emu.sli_no_count(9usize, 9usize, 3u32, 2162532u32);
    emu.lw_no_count(10usize, 2usize, 4u32, 2162536u32)?;
    emu.lw_no_count(13usize, 2usize, 8u32, 2162540u32)?;
    emu.adr_no_count(9usize, 11usize, 9usize, 2162544u32);
    emu.lw_no_count(11usize, 9usize, 0u32, 2162548u32)?;
    emu.lw_no_count(12usize, 9usize, 4u32, 2162552u32)?;
    emu.lw_no_count(13usize, 13usize, 12u32, 2162556u32)?;
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2162560u32;
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
pub fn block_0x0020ff80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2162572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ff8c));
    } else {
        emu.pc = 2162564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ff84));
    }
}
#[inline(always)]
pub fn block_0x0020ff84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2162568u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2162572u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2162576u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ff90));
}
#[inline(always)]
pub fn block_0x0020ff8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2162576u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2162576u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ff90));
}
#[inline]
pub fn block_0x0020ff90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2162580u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2162584u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2162588u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2162592u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2162596u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2162600u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2162604u32)?;
    emu.lw_no_count(22usize, 2usize, 32u32, 2162608u32)?;
    emu.lw_no_count(23usize, 2usize, 28u32, 2162612u32)?;
    emu.lw_no_count(24usize, 2usize, 24u32, 2162616u32)?;
    emu.lw_no_count(25usize, 2usize, 20u32, 2162620u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2162624u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162628u32;
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
