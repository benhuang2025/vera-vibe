pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2160720u32;
pub const PC_MAX: u32 = 2163224u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 113usize] = [
        block_0x0020f850,
        block_0x0020f87c,
        block_0x0020f884,
        block_0x0020f888,
        block_0x0020f890,
        block_0x0020f8a0,
        block_0x0020f8a8,
        block_0x0020f8ac,
        block_0x0020f8b8,
        block_0x0020f8c8,
        block_0x0020f8d0,
        block_0x0020f8d4,
        block_0x0020f8e0,
        block_0x0020f90c,
        block_0x0020f914,
        block_0x0020f918,
        block_0x0020f934,
        block_0x0020f944,
        block_0x0020f948,
        block_0x0020f950,
        block_0x0020f954,
        block_0x0020f95c,
        block_0x0020f97c,
        block_0x0020f984,
        block_0x0020f988,
        block_0x0020f994,
        block_0x0020f99c,
        block_0x0020f9ac,
        block_0x0020f9c8,
        block_0x0020f9d0,
        block_0x0020fa00,
        block_0x0020fa18,
        block_0x0020fa24,
        block_0x0020fa28,
        block_0x0020fa54,
        block_0x0020fa58,
        block_0x0020fa68,
        block_0x0020fa9c,
        block_0x0020fab8,
        block_0x0020fac0,
        block_0x0020fac4,
        block_0x0020fad4,
        block_0x0020fad8,
        block_0x0020fae0,
        block_0x0020fae4,
        block_0x0020faf4,
        block_0x0020fb10,
        block_0x0020fb2c,
        block_0x0020fb44,
        block_0x0020fbb4,
        block_0x0020fbec,
        block_0x0020fc0c,
        block_0x0020fc24,
        block_0x0020fc34,
        block_0x0020fc3c,
        block_0x0020fc4c,
        block_0x0020fc54,
        block_0x0020fc5c,
        block_0x0020fc6c,
        block_0x0020fc74,
        block_0x0020fdac,
        block_0x0020fdbc,
        block_0x0020fdc4,
        block_0x0020fdd4,
        block_0x0020fde0,
        block_0x0020fdf0,
        block_0x0020fdfc,
        block_0x0020fe0c,
        block_0x0020fe10,
        block_0x0020fe20,
        block_0x0020ff54,
        block_0x0020ff68,
        block_0x0020ff6c,
        block_0x0020ff74,
        block_0x0020ff78,
        block_0x0020ff8c,
        block_0x0020ffac,
        block_0x0020ffc8,
        block_0x0021001c,
        block_0x00210024,
        block_0x00210060,
        block_0x0021006c,
        block_0x00210080,
        block_0x00210084,
        block_0x0021008c,
        block_0x00210090,
        block_0x002100a8,
        block_0x002100ac,
        block_0x002100c0,
        block_0x002100cc,
        block_0x002100d4,
        block_0x002100e0,
        block_0x002100e4,
        block_0x002100e8,
        block_0x00210114,
        block_0x00210118,
        block_0x0021013c,
        block_0x00210140,
        block_0x00210148,
        block_0x00210174,
        block_0x00210180,
        block_0x00210194,
        block_0x00210198,
        block_0x002101a8,
        block_0x002101ac,
        block_0x002101cc,
        block_0x002101d4,
        block_0x002101d8,
        block_0x002101e4,
        block_0x00210208,
        block_0x0021020c,
        block_0x00210214,
        block_0x00210218,
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
    if pc < 2160720u32 || pc > 2163224u32 {
        return None;
    }
    let word_offset = ((pc - 2160720u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x0020f850(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2160724u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2160728u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2160732u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2160736u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2160740u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2160744u32)?;
    emu.sw_no_count(20usize, 2usize, 24u32, 2160748u32)?;
    emu.sw_no_count(21usize, 2usize, 20u32, 2160752u32)?;
    emu.sw_no_count(22usize, 2usize, 16u32, 2160756u32)?;
    emu.sw_no_count(23usize, 2usize, 12u32, 2160760u32)?;
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(29usize);
    if a == b {
        emu.pc = 2160776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f888));
    } else {
        emu.pc = 2160764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f87c));
    }
}
#[inline(always)]
pub fn block_0x0020f87c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 31usize, 29usize, 2160768u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a != b {
        emu.pc = 2160784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f890));
    } else {
        emu.pc = 2160772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f884));
    }
}
#[inline(always)]
pub fn block_0x0020f884(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2160776u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2161188u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fa24));
}
#[inline(always)]
pub fn block_0x0020f888(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 30usize, 28usize, 2160780u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2161188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fa24));
    } else {
        emu.pc = 2160784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f890));
    }
}
#[inline(always)]
pub fn block_0x0020f890(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 28usize, 30usize, 2160788u32);
    emu.sbr_no_count(6usize, 29usize, 31usize, 2160792u32);
    emu.sbr_no_count(5usize, 6usize, 5usize, 2160796u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2160812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f8ac));
    } else {
        emu.pc = 2160800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f8a0));
    }
}
#[inline(always)]
pub fn block_0x0020f8a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 31usize, 5usize, 2160804u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a != b {
        emu.pc = 2160824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f8b8));
    } else {
        emu.pc = 2160808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f8a8));
    }
}
#[inline(always)]
pub fn block_0x0020f8a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2160812u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2161188u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fa24));
}
#[inline(always)]
pub fn block_0x0020f8ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(5usize, 28usize, 30usize, 2160816u32);
    emu.sltru_no_count(5usize, 30usize, 5usize, 2160820u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2161188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fa24));
    } else {
        emu.pc = 2160824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f8b8));
    }
}
#[inline(always)]
pub fn block_0x0020f8b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 28usize, 16usize, 2160828u32);
    emu.sbr_no_count(6usize, 29usize, 17usize, 2160832u32);
    emu.sbr_no_count(5usize, 6usize, 5usize, 2160836u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2160852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f8d4));
    } else {
        emu.pc = 2160840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f8c8));
    }
}
#[inline(always)]
pub fn block_0x0020f8c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 17usize, 5usize, 2160844u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a != b {
        emu.pc = 2160864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f8e0));
    } else {
        emu.pc = 2160848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f8d0));
    }
}
#[inline(always)]
pub fn block_0x0020f8d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2160852u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2160964u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f944));
}
#[inline(always)]
pub fn block_0x0020f8d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(5usize, 28usize, 16usize, 2160856u32);
    emu.sltru_no_count(5usize, 16usize, 5usize, 2160860u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2160964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f944));
    } else {
        emu.pc = 2160864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f8e0));
    }
}
#[inline]
pub fn block_0x0020f8e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(6usize, 16usize, 31u32, 2160868u32);
    emu.sli_no_count(7usize, 17usize, 1u32, 2160872u32);
    emu.sli_no_count(5usize, 16usize, 1u32, 2160876u32);
    emu.sri_no_count(8usize, 30usize, 31u32, 2160880u32);
    emu.orr_no_count(6usize, 7usize, 6usize, 2160884u32);
    emu.sltru_no_count(7usize, 28usize, 5usize, 2160888u32);
    emu.sbr_no_count(6usize, 29usize, 6usize, 2160892u32);
    emu.sbr_no_count(6usize, 6usize, 7usize, 2160896u32);
    emu.sli_no_count(7usize, 31usize, 1u32, 2160900u32);
    emu.orr_no_count(7usize, 7usize, 8usize, 2160904u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2160948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f934));
    } else {
        emu.pc = 2160908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f90c));
    }
}
#[inline(always)]
pub fn block_0x0020f90c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 6usize, 7usize, 2160912u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a != b {
        emu.pc = 2160964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f944));
    } else {
        emu.pc = 2160916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f914));
    }
}
#[inline(always)]
pub fn block_0x0020f914(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2161380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fae4));
    } else {
        emu.pc = 2160920u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f918));
    }
}
#[inline(always)]
pub fn block_0x0020f918(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2160924u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 248u32, 2160928u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2160932u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2160936u32);
    emu.adi_no_count(12usize, 14usize, 0u32, 2160940u32);
    emu.apc_no_count(1usize, 2160940u32, 16384u32, 2160944u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160948u32;
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
pub fn block_0x0020f934(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(5usize, 28usize, 5usize, 2160952u32);
    emu.sli_no_count(6usize, 30usize, 1u32, 2160956u32);
    emu.sltru_no_count(5usize, 5usize, 6usize, 2160960u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2160916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f914));
    } else {
        emu.pc = 2160964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f944));
    }
}
#[inline(always)]
pub fn block_0x0020f944(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2160980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f954));
    } else {
        emu.pc = 2160968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f948));
    }
}
#[inline(always)]
pub fn block_0x0020f948(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 31usize, 17usize, 2160972u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a != b {
        emu.pc = 2160988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f95c));
    } else {
        emu.pc = 2160976u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f950));
    }
}
#[inline(always)]
pub fn block_0x0020f950(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2160980u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2161188u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fa24));
}
#[inline(always)]
pub fn block_0x0020f954(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 30usize, 16usize, 2160984u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2161188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fa24));
    } else {
        emu.pc = 2160988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f95c));
    }
}
#[inline(always)]
pub fn block_0x0020f95c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 16usize, 30usize, 2160992u32);
    emu.sbr_no_count(17usize, 17usize, 31usize, 2160996u32);
    emu.sbr_no_count(16usize, 16usize, 30usize, 2161000u32);
    emu.sbr_no_count(17usize, 17usize, 5usize, 2161004u32);
    emu.sbr_no_count(5usize, 29usize, 17usize, 2161008u32);
    emu.sltru_no_count(6usize, 28usize, 16usize, 2161012u32);
    emu.sbr_no_count(5usize, 5usize, 6usize, 2161016u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2161176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fa18));
    } else {
        emu.pc = 2161020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f97c));
    }
}
#[inline(always)]
pub fn block_0x0020f97c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 17usize, 5usize, 2161024u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a != b {
        emu.pc = 2161188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fa24));
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
        emu.pc = 2161396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020faf4));
    } else {
        emu.pc = 2161032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f988));
    }
}
#[inline(always)]
pub fn block_0x0020f988(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 0u32, 2161036u32);
    emu.adr_no_count(8usize, 11usize, 13usize, 2161040u32);
    emu.adi_no_count(17usize, 0usize, 57u32, 2161044u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2161044u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f994));
}
#[inline(always)]
pub fn block_0x0020f994(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(5usize, 13usize, 16usize, 2161048u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2161236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fa54));
    } else {
        emu.pc = 2161052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f99c));
    }
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
    emu.adr_no_count(5usize, 8usize, 16usize, 2161056u32);
    emu.lbu_no_count(5usize, 5usize, 4294967295u32, 2161060u32);
    emu.adi_no_count(16usize, 16usize, 4294967295u32, 2161064u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2161044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f994));
    } else {
        emu.pc = 2161068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f9ac));
    }
}
#[inline(always)]
pub fn block_0x0020f9ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 8usize, 16usize, 2161072u32);
    emu.lbu_no_count(15usize, 8usize, 0u32, 2161076u32);
    emu.adr_no_count(17usize, 13usize, 16usize, 2161080u32);
    emu.adi_no_count(5usize, 15usize, 1u32, 2161084u32);
    emu.adi_no_count(15usize, 17usize, 1u32, 2161088u32);
    emu.sb_no_count(5usize, 8usize, 0u32, 2161092u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2161452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fb2c));
    } else {
        emu.pc = 2161096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f9c8));
    }
}
#[inline(always)]
pub fn block_0x0020f9c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 4294967295u32, 2161100u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2161376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fae0));
    } else {
        emu.pc = 2161104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f9d0));
    }
}
#[inline]
pub fn block_0x0020f9d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(15usize, 16usize, 4294967295u32, 2161108u32);
    emu.adi_no_count(16usize, 8usize, 1u32, 2161112u32);
    emu.adi_no_count(8usize, 11usize, 0u32, 2161116u32);
    emu.adi_no_count(11usize, 0usize, 48u32, 2161120u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2161124u32);
    emu.adi_no_count(10usize, 16usize, 0u32, 2161128u32);
    emu.adi_no_count(18usize, 12usize, 0u32, 2161132u32);
    emu.adi_no_count(12usize, 15usize, 0u32, 2161136u32);
    emu.adi_no_count(19usize, 13usize, 0u32, 2161140u32);
    emu.adi_no_count(20usize, 14usize, 0u32, 2161144u32);
    emu.apc_no_count(1usize, 2161144u32, 4294914048u32, 2161148u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161152u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1412u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020fa00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 8usize, 0u32, 2161156u32);
    emu.adi_no_count(14usize, 20usize, 0u32, 2161160u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2161164u32);
    emu.adi_no_count(12usize, 18usize, 0u32, 2161168u32);
    emu.adi_no_count(13usize, 19usize, 0u32, 2161172u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2161176u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2161376u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fae0));
}
#[inline(always)]
pub fn block_0x0020fa18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(17usize, 28usize, 16usize, 2161180u32);
    emu.sltru_no_count(16usize, 16usize, 17usize, 2161184u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2161028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f984));
    } else {
        emu.pc = 2161188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fa24));
    }
}
#[inline(always)]
pub fn block_0x0020fa24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 10usize, 0u32, 2161192u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2161192u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fa28));
}
#[inline]
pub fn block_0x0020fa28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2161196u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2161200u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2161204u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2161208u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2161212u32)?;
    emu.lw_no_count(20usize, 2usize, 24u32, 2161216u32)?;
    emu.lw_no_count(21usize, 2usize, 20u32, 2161220u32)?;
    emu.lw_no_count(22usize, 2usize, 16u32, 2161224u32)?;
    emu.lw_no_count(23usize, 2usize, 12u32, 2161228u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2161232u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161236u32;
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
pub fn block_0x0020fa54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2161336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fab8));
    } else {
        emu.pc = 2161240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fa58));
    }
}
#[inline(always)]
pub fn block_0x0020fa58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 0usize, 49u32, 2161244u32);
    emu.adi_no_count(16usize, 13usize, 4294967295u32, 2161248u32);
    emu.sb_no_count(17usize, 11usize, 0u32, 2161252u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2161344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fac0));
    } else {
        emu.pc = 2161256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fa68));
    }
}
#[inline]
pub fn block_0x0020fa68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 11usize, 1u32, 2161260u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2161264u32);
    emu.adi_no_count(11usize, 0usize, 48u32, 2161268u32);
    emu.adi_no_count(9usize, 0usize, 48u32, 2161272u32);
    emu.adi_no_count(19usize, 10usize, 0u32, 2161276u32);
    emu.adi_no_count(10usize, 17usize, 0u32, 2161280u32);
    emu.adi_no_count(20usize, 12usize, 0u32, 2161284u32);
    emu.adi_no_count(12usize, 16usize, 0u32, 2161288u32);
    emu.adi_no_count(21usize, 13usize, 0u32, 2161292u32);
    emu.adi_no_count(23usize, 14usize, 0u32, 2161296u32);
    emu.adi_no_count(22usize, 15usize, 0u32, 2161300u32);
    emu.apc_no_count(1usize, 2161300u32, 4294914048u32, 2161304u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161308u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1256u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020fa9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 22usize, 0u32, 2161312u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2161316u32);
    emu.adi_no_count(14usize, 23usize, 0u32, 2161320u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2161324u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2161328u32);
    emu.adi_no_count(13usize, 21usize, 0u32, 2161332u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2161336u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2161348u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fac4));
}
#[inline(always)]
pub fn block_0x0020fab8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 49u32, 2161340u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2161344u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2161348u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fac4));
}
#[inline(always)]
pub fn block_0x0020fac0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 48u32, 2161348u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2161348u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fac4));
}
#[inline(always)]
pub fn block_0x0020fac4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 14usize, 1u32, 2161352u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2161356u32);
    emu.sai_no_count(14usize, 14usize, 1040u32, 2161360u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2161376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fae0));
    } else {
        emu.pc = 2161364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fad4));
    }
}
#[inline(always)]
pub fn block_0x0020fad4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2161376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fae0));
    } else {
        emu.pc = 2161368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fad8));
    }
}
#[inline(always)]
pub fn block_0x0020fad8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(9usize, 8usize, 0u32, 2161372u32);
    emu.adi_no_count(13usize, 13usize, 1u32, 2161376u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2161376u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fae0));
}
#[inline(always)]
pub fn block_0x0020fae0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2161424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fb10));
    } else {
        emu.pc = 2161380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fae4));
    }
}
#[inline(always)]
pub fn block_0x0020fae4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 10usize, 0u32, 2161384u32)?;
    emu.sw_no_count(13usize, 10usize, 4u32, 2161388u32)?;
    emu.sh_no_count(14usize, 10usize, 8u32, 2161392u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2161396u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2161192u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fa28));
}
#[inline(always)]
pub fn block_0x0020faf4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2161400u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 216u32, 2161404u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2161408u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2161412u32);
    emu.adi_no_count(12usize, 14usize, 0u32, 2161416u32);
    emu.apc_no_count(1usize, 2161416u32, 16384u32, 2161420u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161424u32;
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
pub fn block_0x0020fb10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2161428u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 232u32, 2161432u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2161436u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2161440u32);
    emu.adi_no_count(12usize, 14usize, 0u32, 2161444u32);
    emu.apc_no_count(1usize, 2161444u32, 16384u32, 2161448u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161452u32;
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
pub fn block_0x0020fb2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2161456u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1068u32, 2161460u32);
    emu.adi_no_count(10usize, 15usize, 0u32, 2161464u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2161468u32);
    emu.apc_no_count(1usize, 2161468u32, 16384u32, 2161472u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161476u32;
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
pub fn block_0x0020fb44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2161480u32);
    emu.sw_no_count(10usize, 2usize, 8u32, 2161484u32)?;
    emu.sw_no_count(11usize, 2usize, 12u32, 2161488u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2161492u32)?;
    emu.sw_no_count(13usize, 2usize, 20u32, 2161496u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2161500u32);
    let a = 0u32.wrapping_add(2170880u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2161504u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965680u32, 2161508u32);
    emu.adi_no_count(12usize, 2usize, 16u32, 2161512u32);
    let a = 0u32.wrapping_add(2170880u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2161516u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294965664u32, 2161520u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2161524u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 268u32, 2161528u32);
    emu.adi_no_count(16usize, 0usize, 2u32, 2161532u32);
    emu.sw_no_count(0usize, 2usize, 40u32, 2161536u32)?;
    emu.adi_no_count(17usize, 2usize, 48u32, 2161540u32);
    emu.sw_no_count(10usize, 2usize, 48u32, 2161544u32)?;
    emu.sw_no_count(11usize, 2usize, 52u32, 2161548u32)?;
    emu.sw_no_count(12usize, 2usize, 56u32, 2161552u32)?;
    emu.sw_no_count(13usize, 2usize, 60u32, 2161556u32)?;
    emu.sw_no_count(15usize, 2usize, 24u32, 2161560u32)?;
    emu.sw_no_count(16usize, 2usize, 28u32, 2161564u32)?;
    emu.sw_no_count(17usize, 2usize, 32u32, 2161568u32)?;
    emu.sw_no_count(16usize, 2usize, 36u32, 2161572u32)?;
    emu.adi_no_count(10usize, 2usize, 24u32, 2161576u32);
    emu.adi_no_count(11usize, 14usize, 0u32, 2161580u32);
    emu.apc_no_count(1usize, 2161580u32, 4294955008u32, 2161584u32);
    emu.add_memory_rw_events(28usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161588u32;
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
pub fn block_0x0020fbb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2161592u32);
    emu.adi_no_count(11usize, 10usize, 0u32, 2161596u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2161600u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 312u32, 2161604u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2161608u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2161612u32)?;
    emu.adi_no_count(13usize, 0usize, 4u32, 2161616u32);
    emu.sw_no_count(10usize, 2usize, 8u32, 2161620u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2161624u32)?;
    emu.sw_no_count(13usize, 2usize, 16u32, 2161628u32)?;
    emu.sw_no_count(0usize, 2usize, 20u32, 2161632u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2161636u32);
    emu.apc_no_count(1usize, 2161636u32, 4294955008u32, 2161640u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161644u32;
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
pub fn block_0x0020fbec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2161648u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2161652u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2161656u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2161660u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2161664u32)?;
    emu.adi_no_count(13usize, 0usize, 39u32, 2161668u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2161672u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2161716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fc34));
    } else {
        emu.pc = 2161676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fc0c));
    }
}
#[inline(always)]
pub fn block_0x0020fc0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 11usize, 2u32, 2161680u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2161684u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 320u32, 2161688u32);
    emu.adr_no_count(10usize, 10usize, 13usize, 2161692u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2161696u32)?;
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(10usize);
    let return_addr = 2161700u32;
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
pub fn block_0x0020fc24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2161704u32);
    let a = 0u32.wrapping_add(12288u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2161708u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 92u32, 2161712u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2161716u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2162552u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ff78));
}
#[inline(always)]
pub fn block_0x0020fc34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 92u32, 2161720u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2161740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fc4c));
    } else {
        emu.pc = 2161724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fc3c));
    }
}
#[inline(always)]
pub fn block_0x0020fc3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2161728u32);
    let a = 0u32.wrapping_add(24576u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2161732u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966364u32, 2161736u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2161740u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2162552u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ff78));
}
#[inline(always)]
pub fn block_0x0020fc4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(12usize, 12usize, 1u32, 2161744u32);
    emu.add_memory_rw_events(1usize);
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
        emu.pc = 2161748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fc54));
    }
}
#[inline(always)]
pub fn block_0x0020fc54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 767u32, 2161752u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2162172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fdfc));
    } else {
        emu.pc = 2161756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fc5c));
    }
}
#[inline(always)]
pub fn block_0x0020fc5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 11usize, 0u32, 2161760u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2161764u32);
    emu.apc_no_count(1usize, 2161764u32, 4294959104u32, 2161768u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161772u32;
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
pub fn block_0x0020fc6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 9usize, 0u32, 2161776u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2162172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fdfc));
    } else {
        emu.pc = 2161780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fc74));
    }
}
#[inline(never)]
pub fn block_0x0020fc74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 78u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(14usize, 11usize, 1u32, 2161784u32);
    let a = 0u32.wrapping_add(1431654400u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2161788u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(858992640u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2161792u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(252645376u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2161796u32;
    emu.update_insn_clock();
    emu.sri_no_count(15usize, 11usize, 20u32, 2161800u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2161804u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 1256u32, 2161808u32);
    emu.sli_no_count(17usize, 11usize, 12u32, 2161812u32);
    emu.sli_no_count(5usize, 11usize, 16u32, 2161816u32);
    emu.sli_no_count(6usize, 11usize, 20u32, 2161820u32);
    emu.sli_no_count(7usize, 11usize, 24u32, 2161824u32);
    emu.orr_no_count(14usize, 11usize, 14usize, 2161828u32);
    emu.ani_no_count(11usize, 11usize, 15u32, 2161832u32);
    emu.adr_no_count(15usize, 16usize, 15usize, 2161836u32);
    emu.sri_no_count(17usize, 17usize, 28u32, 2161840u32);
    emu.sri_no_count(5usize, 5usize, 28u32, 2161844u32);
    emu.sri_no_count(6usize, 6usize, 28u32, 2161848u32);
    emu.sri_no_count(7usize, 7usize, 28u32, 2161852u32);
    emu.adr_no_count(11usize, 16usize, 11usize, 2161856u32);
    emu.adr_no_count(17usize, 16usize, 17usize, 2161860u32);
    emu.adr_no_count(5usize, 16usize, 5usize, 2161864u32);
    emu.adr_no_count(6usize, 16usize, 6usize, 2161868u32);
    emu.adr_no_count(16usize, 16usize, 7usize, 2161872u32);
    emu.sri_no_count(7usize, 14usize, 2u32, 2161876u32);
    emu.orr_no_count(14usize, 14usize, 7usize, 2161880u32);
    emu.sri_no_count(7usize, 14usize, 4u32, 2161884u32);
    emu.orr_no_count(14usize, 14usize, 7usize, 2161888u32);
    emu.sri_no_count(7usize, 14usize, 8u32, 2161892u32);
    emu.orr_no_count(14usize, 14usize, 7usize, 2161896u32);
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2161900u32;
    emu.update_insn_clock();
    emu.lbu_no_count(15usize, 15usize, 0u32, 2161904u32);
    emu.lbu_no_count(17usize, 17usize, 0u32, 2161908u32);
    emu.lbu_no_count(5usize, 5usize, 0u32, 2161912u32);
    emu.lbu_no_count(6usize, 6usize, 0u32, 2161916u32);
    emu.sh_no_count(0usize, 2usize, 12u32, 2161920u32)?;
    emu.sb_no_count(0usize, 2usize, 14u32, 2161924u32);
    emu.sb_no_count(15usize, 2usize, 15u32, 2161928u32);
    emu.sb_no_count(17usize, 2usize, 16u32, 2161932u32);
    emu.adi_no_count(15usize, 0usize, 125u32, 2161936u32);
    emu.adi_no_count(13usize, 13usize, 1365u32, 2161940u32);
    emu.lbu_no_count(16usize, 16usize, 0u32, 2161944u32);
    emu.lbu_no_count(11usize, 11usize, 0u32, 2161948u32);
    emu.sb_no_count(5usize, 2usize, 17u32, 2161952u32);
    emu.sb_no_count(6usize, 2usize, 18u32, 2161956u32);
    emu.sb_no_count(16usize, 2usize, 19u32, 2161960u32);
    emu.sri_no_count(16usize, 14usize, 16u32, 2161964u32);
    emu.orr_no_count(14usize, 14usize, 16usize, 2161968u32);
    emu.xri_no_count(14usize, 14usize, 4294967295u32, 2161972u32);
    emu.sri_no_count(16usize, 14usize, 1u32, 2161976u32);
    emu.anr_no_count(13usize, 16usize, 13usize, 2161980u32);
    emu.adi_no_count(16usize, 2usize, 12u32, 2161984u32);
    emu.adi_no_count(12usize, 12usize, 819u32, 2161988u32);
    emu.ani_no_count(14usize, 14usize, 4294967294u32, 2161992u32);
    emu.sbr_no_count(14usize, 14usize, 13usize, 2161996u32);
    emu.anr_no_count(13usize, 14usize, 12usize, 2162000u32);
    emu.sri_no_count(14usize, 14usize, 2u32, 2162004u32);
    emu.anr_no_count(12usize, 14usize, 12usize, 2162008u32);
    emu.adi_no_count(14usize, 0usize, 92u32, 2162012u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2162016u32);
    emu.sri_no_count(13usize, 12usize, 4u32, 2162020u32);
    emu.adr_no_count(12usize, 12usize, 13usize, 2162024u32);
    emu.adi_no_count(13usize, 0usize, 117u32, 2162028u32);
    emu.adi_no_count(10usize, 10usize, 4294967055u32, 2162032u32);
    emu.anr_no_count(10usize, 12usize, 10usize, 2162036u32);
    emu.adi_no_count(12usize, 0usize, 123u32, 2162040u32);
    emu.adi_no_count(17usize, 7usize, 257u32, 2162044u32);
    emu.mul_no_count(10usize, 10usize, 17usize, 2162048u32);
    emu.sri_no_count(10usize, 10usize, 26u32, 2162052u32);
    emu.adi_no_count(9usize, 10usize, 4294967294u32, 2162056u32);
    emu.adr_no_count(10usize, 16usize, 10usize, 2162060u32);
    emu.adr_no_count(16usize, 16usize, 9usize, 2162064u32);
    emu.sb_no_count(14usize, 16usize, 0u32, 2162068u32);
    emu.sb_no_count(13usize, 10usize, 4294967295u32, 2162072u32);
    emu.sb_no_count(12usize, 10usize, 0u32, 2162076u32);
    emu.sb_no_count(11usize, 2usize, 20u32, 2162080u32);
    emu.sb_no_count(15usize, 2usize, 21u32, 2162084u32);
    emu.adi_no_count(11usize, 2usize, 12u32, 2162088u32);
    emu.add_memory_rw_events(78usize);
    let return_addr = 2162092u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2162516u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ff54));
}
#[inline(always)]
pub fn block_0x0020fdac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2162096u32);
    let a = 0u32.wrapping_add(28672u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2162100u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966876u32, 2162104u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2162108u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2162552u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ff78));
}
#[inline(always)]
pub fn block_0x0020fdbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 12usize, 256u32, 2162112u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2162172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fdfc));
    } else {
        emu.pc = 2162116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fdc4));
    }
}
#[inline(always)]
pub fn block_0x0020fdc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2162120u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2162124u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1884u32, 2162128u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2162132u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2162552u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ff78));
}
#[inline(always)]
pub fn block_0x0020fdd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2162136u32);
    let a = 0u32.wrapping_add(28672u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2162140u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(3usize);
    let return_addr = 2162144u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2162548u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ff74));
}
#[inline(always)]
pub fn block_0x0020fde0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2162148u32);
    let a = 0u32.wrapping_add(28672u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2162152u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1116u32, 2162156u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2162160u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2162552u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ff78));
}
#[inline(always)]
pub fn block_0x0020fdf0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 8u32, 2162164u32);
    emu.sri_no_count(12usize, 12usize, 24u32, 2162168u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2162540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ff6c));
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
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 11usize, 0u32, 2162176u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2162180u32);
    emu.apc_no_count(1usize, 2162180u32, 8192u32, 2162184u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162188u32;
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
pub fn block_0x0020fe0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2162208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fe20));
    } else {
        emu.pc = 2162192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fe10));
    }
}
#[inline(always)]
pub fn block_0x0020fe10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(9usize, 8usize, 0u32, 2162196u32)?;
    emu.adi_no_count(18usize, 0usize, 129u32, 2162200u32);
    emu.adi_no_count(9usize, 0usize, 128u32, 2162204u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2162208u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2162572u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ff8c));
}
#[inline(never)]
pub fn block_0x0020fe20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 77u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(14usize, 9usize, 1u32, 2162212u32);
    let a = 0u32.wrapping_add(1431654400u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2162216u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(858992640u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2162220u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(252645376u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2162224u32;
    emu.update_insn_clock();
    emu.sri_no_count(15usize, 9usize, 20u32, 2162228u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2162232u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 1256u32, 2162236u32);
    emu.sli_no_count(17usize, 9usize, 12u32, 2162240u32);
    emu.sli_no_count(5usize, 9usize, 16u32, 2162244u32);
    emu.sli_no_count(6usize, 9usize, 20u32, 2162248u32);
    emu.sli_no_count(7usize, 9usize, 24u32, 2162252u32);
    emu.orr_no_count(14usize, 9usize, 14usize, 2162256u32);
    emu.ani_no_count(13usize, 9usize, 15u32, 2162260u32);
    emu.adr_no_count(15usize, 16usize, 15usize, 2162264u32);
    emu.sri_no_count(17usize, 17usize, 28u32, 2162268u32);
    emu.sri_no_count(5usize, 5usize, 28u32, 2162272u32);
    emu.sri_no_count(6usize, 6usize, 28u32, 2162276u32);
    emu.sri_no_count(7usize, 7usize, 28u32, 2162280u32);
    emu.adr_no_count(13usize, 16usize, 13usize, 2162284u32);
    emu.adr_no_count(17usize, 16usize, 17usize, 2162288u32);
    emu.adr_no_count(5usize, 16usize, 5usize, 2162292u32);
    emu.adr_no_count(6usize, 16usize, 6usize, 2162296u32);
    emu.adr_no_count(16usize, 16usize, 7usize, 2162300u32);
    emu.sri_no_count(7usize, 14usize, 2u32, 2162304u32);
    emu.orr_no_count(14usize, 14usize, 7usize, 2162308u32);
    emu.sri_no_count(7usize, 14usize, 4u32, 2162312u32);
    emu.orr_no_count(14usize, 14usize, 7usize, 2162316u32);
    emu.sri_no_count(7usize, 14usize, 8u32, 2162320u32);
    emu.orr_no_count(14usize, 14usize, 7usize, 2162324u32);
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2162328u32;
    emu.update_insn_clock();
    emu.lbu_no_count(15usize, 15usize, 0u32, 2162332u32);
    emu.lbu_no_count(17usize, 17usize, 0u32, 2162336u32);
    emu.lbu_no_count(5usize, 5usize, 0u32, 2162340u32);
    emu.lbu_no_count(6usize, 6usize, 0u32, 2162344u32);
    emu.sh_no_count(0usize, 2usize, 22u32, 2162348u32)?;
    emu.sb_no_count(0usize, 2usize, 24u32, 2162352u32);
    emu.sb_no_count(15usize, 2usize, 25u32, 2162356u32);
    emu.sb_no_count(17usize, 2usize, 26u32, 2162360u32);
    emu.adi_no_count(15usize, 0usize, 125u32, 2162364u32);
    emu.adi_no_count(12usize, 12usize, 1365u32, 2162368u32);
    emu.lbu_no_count(16usize, 16usize, 0u32, 2162372u32);
    emu.lbu_no_count(13usize, 13usize, 0u32, 2162376u32);
    emu.sb_no_count(5usize, 2usize, 27u32, 2162380u32);
    emu.sb_no_count(6usize, 2usize, 28u32, 2162384u32);
    emu.sb_no_count(16usize, 2usize, 29u32, 2162388u32);
    emu.sri_no_count(16usize, 14usize, 16u32, 2162392u32);
    emu.orr_no_count(14usize, 14usize, 16usize, 2162396u32);
    emu.xri_no_count(14usize, 14usize, 4294967295u32, 2162400u32);
    emu.sri_no_count(16usize, 14usize, 1u32, 2162404u32);
    emu.anr_no_count(12usize, 16usize, 12usize, 2162408u32);
    emu.adi_no_count(16usize, 2usize, 22u32, 2162412u32);
    emu.adi_no_count(11usize, 11usize, 819u32, 2162416u32);
    emu.ani_no_count(14usize, 14usize, 4294967294u32, 2162420u32);
    emu.sbr_no_count(14usize, 14usize, 12usize, 2162424u32);
    emu.anr_no_count(12usize, 14usize, 11usize, 2162428u32);
    emu.sri_no_count(14usize, 14usize, 2u32, 2162432u32);
    emu.anr_no_count(11usize, 14usize, 11usize, 2162436u32);
    emu.adi_no_count(14usize, 0usize, 92u32, 2162440u32);
    emu.adr_no_count(11usize, 12usize, 11usize, 2162444u32);
    emu.sri_no_count(12usize, 11usize, 4u32, 2162448u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2162452u32);
    emu.adi_no_count(12usize, 0usize, 117u32, 2162456u32);
    emu.adi_no_count(10usize, 10usize, 4294967055u32, 2162460u32);
    emu.anr_no_count(10usize, 11usize, 10usize, 2162464u32);
    emu.adi_no_count(11usize, 0usize, 123u32, 2162468u32);
    emu.adi_no_count(17usize, 7usize, 257u32, 2162472u32);
    emu.mul_no_count(10usize, 10usize, 17usize, 2162476u32);
    emu.sri_no_count(10usize, 10usize, 26u32, 2162480u32);
    emu.adi_no_count(9usize, 10usize, 4294967294u32, 2162484u32);
    emu.adr_no_count(10usize, 16usize, 10usize, 2162488u32);
    emu.adr_no_count(16usize, 16usize, 9usize, 2162492u32);
    emu.sb_no_count(14usize, 16usize, 0u32, 2162496u32);
    emu.sb_no_count(12usize, 10usize, 4294967295u32, 2162500u32);
    emu.sb_no_count(11usize, 10usize, 0u32, 2162504u32);
    emu.sb_no_count(13usize, 2usize, 30u32, 2162508u32);
    emu.sb_no_count(15usize, 2usize, 31u32, 2162512u32);
    emu.adi_no_count(11usize, 2usize, 22u32, 2162516u32);
    emu.add_memory_rw_events(77usize);
    emu.pc = 2162516u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ff54));
}
#[inline(always)]
pub fn block_0x0020ff54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 10u32, 2162520u32);
    emu.adi_no_count(18usize, 0usize, 10u32, 2162524u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2162528u32);
    emu.apc_no_count(1usize, 2162528u32, 4294914048u32, 2162532u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162536u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(276u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020ff68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2162540u32;
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
pub fn block_0x0020ff6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2162544u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2162548u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    emu.pc = 2162548u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ff74));
}
#[inline(always)]
pub fn block_0x0020ff74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 10usize, 604u32, 2162552u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2162552u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ff78));
}
#[inline(always)]
pub fn block_0x0020ff78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 8usize, 0u32, 2162556u32)?;
    emu.sh_no_count(0usize, 8usize, 4u32, 2162560u32)?;
    emu.sh_no_count(0usize, 8usize, 6u32, 2162564u32)?;
    emu.sh_no_count(0usize, 8usize, 8u32, 2162568u32)?;
    emu.adi_no_count(18usize, 0usize, 2u32, 2162572u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2162572u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ff8c));
}
#[inline(always)]
pub fn block_0x0020ff8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(9usize, 8usize, 12u32, 2162576u32);
    emu.sb_no_count(18usize, 8usize, 13u32, 2162580u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2162584u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2162588u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2162592u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2162596u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2162600u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162604u32;
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
pub fn block_0x0020ffac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 0u32, 2162608u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2162612u32)?;
    emu.adi_no_count(13usize, 10usize, 0u32, 2162616u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2162620u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2162624u32);
    emu.apc_no_count(6usize, 2162624u32, 0u32, 2162628u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2162632u32;
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
pub fn block_0x0020ffc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2162636u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2162640u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2162644u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2162648u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2162652u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2162656u32)?;
    emu.sw_no_count(20usize, 2usize, 40u32, 2162660u32)?;
    emu.sw_no_count(21usize, 2usize, 36u32, 2162664u32)?;
    emu.sw_no_count(22usize, 2usize, 32u32, 2162668u32)?;
    emu.sw_no_count(23usize, 2usize, 28u32, 2162672u32)?;
    emu.sw_no_count(24usize, 2usize, 24u32, 2162676u32)?;
    emu.sw_no_count(25usize, 2usize, 20u32, 2162680u32)?;
    emu.adi_no_count(8usize, 12usize, 0u32, 2162684u32);
    let a = 0u32.wrapping_add(3758096384u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2162688u32;
    emu.update_insn_clock();
    emu.lw_no_count(21usize, 8usize, 16u32, 2162692u32)?;
    emu.adi_no_count(12usize, 12usize, 32u32, 2162696u32);
    emu.sw_no_count(10usize, 2usize, 4u32, 2162700u32)?;
    emu.sw_no_count(11usize, 2usize, 8u32, 2162704u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2162708u32)?;
    emu.sw_no_count(0usize, 2usize, 16u32, 2162712u32)?;
    emu.add_memory_rw_events(20usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a == b {
        emu.pc = 2163008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210140));
    } else {
        emu.pc = 2162716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021001c));
    }
}
#[inline(always)]
pub fn block_0x0021001c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 8usize, 20u32, 2162720u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2163160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002101d8));
    } else {
        emu.pc = 2162724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210024));
    }
}
#[inline]
pub fn block_0x00210024(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 0u32, 2162728u32);
    emu.sli_no_count(12usize, 11usize, 3u32, 2162732u32);
    emu.sli_no_count(13usize, 11usize, 5u32, 2162736u32);
    emu.adi_no_count(10usize, 21usize, 24u32, 2162740u32);
    emu.lw_no_count(23usize, 8usize, 0u32, 2162744u32)?;
    emu.lw_no_count(19usize, 8usize, 8u32, 2162748u32)?;
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2162752u32);
    emu.adi_no_count(20usize, 0usize, 2u32, 2162756u32);
    emu.sbr_no_count(13usize, 13usize, 12usize, 2162760u32);
    emu.sli_no_count(11usize, 11usize, 3u32, 2162764u32);
    emu.adr_no_count(22usize, 21usize, 13usize, 2162768u32);
    emu.sri_no_count(11usize, 11usize, 3u32, 2162772u32);
    emu.adi_no_count(9usize, 11usize, 1u32, 2162776u32);
    emu.adi_no_count(23usize, 23usize, 4u32, 2162780u32);
    emu.adi_no_count(24usize, 0usize, 1u32, 2162784u32);
    emu.add_memory_rw_events(15usize);
    emu.pc = 2162784u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210060));
}
#[inline(always)]
pub fn block_0x00210060(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 23usize, 0u32, 2162788u32)?;
    emu.adi_no_count(25usize, 10usize, 0u32, 2162792u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2162820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210084));
    } else {
        emu.pc = 2162796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021006c));
    }
}
#[inline(always)]
pub fn block_0x0021006c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 2usize, 8u32, 2162800u32)?;
    emu.lw_no_count(10usize, 2usize, 4u32, 2162804u32)?;
    emu.lw_no_count(11usize, 23usize, 4294967292u32, 2162808u32)?;
    emu.lw_no_count(13usize, 13usize, 12u32, 2162812u32)?;
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2162816u32;
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
pub fn block_0x00210080(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2163212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021020c));
    } else {
        emu.pc = 2162820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210084));
    }
}
#[inline(always)]
pub fn block_0x00210084(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lhu_no_count(10usize, 21usize, 8u32, 2162824u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2162880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002100c0));
    } else {
        emu.pc = 2162828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021008c));
    }
}
#[inline(always)]
pub fn block_0x0021008c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2162900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002100d4));
    } else {
        emu.pc = 2162832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210090));
    }
}
#[inline(always)]
pub fn block_0x00210090(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 21usize, 12u32, 2162836u32)?;
    emu.sli_no_count(10usize, 10usize, 3u32, 2162840u32);
    emu.adr_no_count(10usize, 19usize, 10usize, 2162844u32);
    emu.lhu_no_count(11usize, 10usize, 4u32, 2162848u32)?;
    emu.lhu_no_count(10usize, 21usize, 0u32, 2162852u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2162892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002100cc));
    } else {
        emu.pc = 2162856u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002100a8));
    }
}
#[inline(always)]
pub fn block_0x002100a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2162916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002100e4));
    } else {
        emu.pc = 2162860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002100ac));
    }
}
#[inline(always)]
pub fn block_0x002100ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 21usize, 4u32, 2162864u32)?;
    emu.sli_no_count(10usize, 10usize, 3u32, 2162868u32);
    emu.adr_no_count(10usize, 19usize, 10usize, 2162872u32);
    emu.lhu_no_count(12usize, 10usize, 4u32, 2162876u32)?;
    emu.add_memory_rw_events(5usize);
    let return_addr = 2162880u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2162920u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002100e8));
}
#[inline(always)]
pub fn block_0x002100c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lhu_no_count(11usize, 21usize, 10u32, 2162884u32)?;
    emu.lhu_no_count(10usize, 21usize, 0u32, 2162888u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2162856u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002100a8));
    } else {
        emu.pc = 2162892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002100cc));
    }
}
#[inline(always)]
pub fn block_0x002100cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2162896u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2162900u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2162920u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002100e8));
}
#[inline(always)]
pub fn block_0x002100d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2162904u32);
    emu.lhu_no_count(10usize, 21usize, 0u32, 2162908u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2162856u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002100a8));
    } else {
        emu.pc = 2162912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002100e0));
    }
}
#[inline(always)]
pub fn block_0x002100e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2162916u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2162892u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002100cc));
}
#[inline(always)]
pub fn block_0x002100e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lhu_no_count(12usize, 21usize, 2u32, 2162920u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2162920u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002100e8));
}
#[inline]
pub fn block_0x002100e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 21usize, 16u32, 2162924u32)?;
    emu.lw_no_count(13usize, 21usize, 20u32, 2162928u32)?;
    emu.sli_no_count(10usize, 10usize, 3u32, 2162932u32);
    emu.adr_no_count(14usize, 19usize, 10usize, 2162936u32);
    emu.lw_no_count(10usize, 14usize, 0u32, 2162940u32)?;
    emu.lw_no_count(14usize, 14usize, 4u32, 2162944u32)?;
    emu.sw_no_count(13usize, 2usize, 12u32, 2162948u32)?;
    emu.sh_no_count(11usize, 2usize, 16u32, 2162952u32)?;
    emu.sh_no_count(12usize, 2usize, 18u32, 2162956u32)?;
    emu.adi_no_count(11usize, 2usize, 4u32, 2162960u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2162964u32;
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
pub fn block_0x00210114(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2163212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021020c));
    } else {
        emu.pc = 2162968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210118));
    }
}
#[inline]
pub fn block_0x00210118(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 18usize, 1u32, 2162972u32);
    emu.xrr_no_count(10usize, 25usize, 22usize, 2162976u32);
    emu.sltiu_no_count(10usize, 10usize, 1u32, 2162980u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2162984u32);
    emu.ani_no_count(10usize, 10usize, 24u32, 2162988u32);
    emu.adr_no_count(10usize, 25usize, 10usize, 2162992u32);
    emu.adi_no_count(23usize, 23usize, 8u32, 2162996u32);
    emu.adi_no_count(21usize, 25usize, 0u32, 2163000u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2162784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210060));
    } else {
        emu.pc = 2163004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021013c));
    }
}
#[inline(always)]
pub fn block_0x0021013c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2163008u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2163148u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002101cc));
}
#[inline(always)]
pub fn block_0x00210140(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 12u32, 2163012u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2163160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002101d8));
    } else {
        emu.pc = 2163016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210148));
    }
}
#[inline]
pub fn block_0x00210148(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 0u32, 2163020u32);
    emu.lw_no_count(20usize, 8usize, 0u32, 2163024u32)?;
    emu.lw_no_count(21usize, 8usize, 8u32, 2163028u32)?;
    emu.sli_no_count(19usize, 10usize, 3u32, 2163032u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2163036u32);
    emu.sli_no_count(10usize, 10usize, 3u32, 2163040u32);
    emu.sri_no_count(10usize, 10usize, 3u32, 2163044u32);
    emu.adi_no_count(9usize, 10usize, 1u32, 2163048u32);
    emu.adr_no_count(19usize, 21usize, 19usize, 2163052u32);
    emu.adi_no_count(10usize, 21usize, 8u32, 2163056u32);
    emu.adi_no_count(20usize, 20usize, 4u32, 2163060u32);
    emu.add_memory_rw_events(11usize);
    emu.pc = 2163060u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210174));
}
#[inline(always)]
pub fn block_0x00210174(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 20usize, 0u32, 2163064u32)?;
    emu.adi_no_count(22usize, 10usize, 0u32, 2163068u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2163096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210198));
    } else {
        emu.pc = 2163072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210180));
    }
}
#[inline(always)]
pub fn block_0x00210180(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 2usize, 8u32, 2163076u32)?;
    emu.lw_no_count(10usize, 2usize, 4u32, 2163080u32)?;
    emu.lw_no_count(11usize, 20usize, 4294967292u32, 2163084u32)?;
    emu.lw_no_count(13usize, 13usize, 12u32, 2163088u32)?;
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2163092u32;
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
pub fn block_0x00210194(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2163212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021020c));
    } else {
        emu.pc = 2163096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210198));
    }
}
#[inline(always)]
pub fn block_0x00210198(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 21usize, 0u32, 2163100u32)?;
    emu.lw_no_count(12usize, 21usize, 4u32, 2163104u32)?;
    emu.adi_no_count(11usize, 2usize, 4u32, 2163108u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2163112u32;
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
pub fn block_0x002101a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2163212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021020c));
    } else {
        emu.pc = 2163116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002101ac));
    }
}
#[inline(always)]
pub fn block_0x002101ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 18usize, 1u32, 2163120u32);
    emu.xrr_no_count(10usize, 22usize, 19usize, 2163124u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2163128u32);
    emu.sli_no_count(10usize, 10usize, 3u32, 2163132u32);
    emu.adr_no_count(10usize, 22usize, 10usize, 2163136u32);
    emu.adi_no_count(20usize, 20usize, 8u32, 2163140u32);
    emu.adi_no_count(21usize, 22usize, 0u32, 2163144u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2163060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210174));
    } else {
        emu.pc = 2163148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002101cc));
    }
}
#[inline(always)]
pub fn block_0x002101cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 4u32, 2163152u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a < b {
        emu.pc = 2163172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002101e4));
    } else {
        emu.pc = 2163156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002101d4));
    }
}
#[inline(always)]
pub fn block_0x002101d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2163160u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2163220u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210214));
}
#[inline(always)]
pub fn block_0x002101d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2163164u32);
    emu.lw_no_count(10usize, 8usize, 4u32, 2163168u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(0usize);
    if a >= b {
        emu.pc = 2163220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210214));
    } else {
        emu.pc = 2163172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002101e4));
    }
}
#[inline]
pub fn block_0x002101e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 8usize, 0u32, 2163176u32)?;
    emu.sli_no_count(9usize, 9usize, 3u32, 2163180u32);
    emu.lw_no_count(10usize, 2usize, 4u32, 2163184u32)?;
    emu.lw_no_count(13usize, 2usize, 8u32, 2163188u32)?;
    emu.adr_no_count(9usize, 11usize, 9usize, 2163192u32);
    emu.lw_no_count(11usize, 9usize, 0u32, 2163196u32)?;
    emu.lw_no_count(12usize, 9usize, 4u32, 2163200u32)?;
    emu.lw_no_count(13usize, 13usize, 12u32, 2163204u32)?;
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2163208u32;
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
pub fn block_0x00210208(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2163220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210214));
    } else {
        emu.pc = 2163212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021020c));
    }
}
#[inline(always)]
pub fn block_0x0021020c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2163216u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2163220u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2163224u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210218));
}
#[inline(always)]
pub fn block_0x00210214(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2163224u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2163224u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210218));
}
#[inline]
pub fn block_0x00210218(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2163228u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2163232u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2163236u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2163240u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2163244u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2163248u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2163252u32)?;
    emu.lw_no_count(22usize, 2usize, 32u32, 2163256u32)?;
    emu.lw_no_count(23usize, 2usize, 28u32, 2163260u32)?;
    emu.lw_no_count(24usize, 2usize, 24u32, 2163264u32)?;
    emu.lw_no_count(25usize, 2usize, 20u32, 2163268u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2163272u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2163276u32;
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
