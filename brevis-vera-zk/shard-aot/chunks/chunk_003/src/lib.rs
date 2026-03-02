pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2108512u32;
pub const PC_MAX: u32 = 2111212u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 105usize] = [
        block_0x00202c60,
        block_0x00202c88,
        block_0x00202c90,
        block_0x00202ca0,
        block_0x00202cb8,
        block_0x00202ccc,
        block_0x00202cd4,
        block_0x00202cd8,
        block_0x00202cf0,
        block_0x00202d20,
        block_0x00202d24,
        block_0x00202d38,
        block_0x00202d60,
        block_0x00202d74,
        block_0x00202d78,
        block_0x00202d98,
        block_0x00202da8,
        block_0x00202db0,
        block_0x00202db8,
        block_0x00202dc0,
        block_0x00202dc4,
        block_0x00202de0,
        block_0x00202dec,
        block_0x00202df4,
        block_0x00202df8,
        block_0x00202e0c,
        block_0x00202e24,
        block_0x00202e34,
        block_0x00202e68,
        block_0x00202e80,
        block_0x00202eb4,
        block_0x00202ebc,
        block_0x00202ee8,
        block_0x00202eec,
        block_0x00202f00,
        block_0x00202f08,
        block_0x00202f3c,
        block_0x00202f44,
        block_0x00202f4c,
        block_0x00202f54,
        block_0x00202f5c,
        block_0x00202f64,
        block_0x00202f8c,
        block_0x00202fec,
        block_0x00202ff4,
        block_0x00203004,
        block_0x0020300c,
        block_0x00203010,
        block_0x00203040,
        block_0x00203048,
        block_0x00203060,
        block_0x00203068,
        block_0x0020307c,
        block_0x00203094,
        block_0x002030f4,
        block_0x002030fc,
        block_0x0020311c,
        block_0x0020317c,
        block_0x00203180,
        block_0x00203184,
        block_0x0020318c,
        block_0x00203194,
        block_0x0020319c,
        block_0x002031c8,
        block_0x002031d0,
        block_0x002031d8,
        block_0x002031dc,
        block_0x00203200,
        block_0x0020320c,
        block_0x002032a0,
        block_0x002032f4,
        block_0x002032f8,
        block_0x00203314,
        block_0x00203320,
        block_0x00203328,
        block_0x0020334c,
        block_0x00203358,
        block_0x00203364,
        block_0x002033a0,
        block_0x0020343c,
        block_0x00203468,
        block_0x00203474,
        block_0x00203490,
        block_0x002034c0,
        block_0x002034e0,
        block_0x00203504,
        block_0x0020352c,
        block_0x00203530,
        block_0x00203538,
        block_0x00203548,
        block_0x00203558,
        block_0x00203580,
        block_0x002035a8,
        block_0x002035bc,
        block_0x002035d4,
        block_0x00203638,
        block_0x00203640,
        block_0x00203650,
        block_0x00203654,
        block_0x00203684,
        block_0x00203694,
        block_0x002036b4,
        block_0x002036c4,
        block_0x002036c8,
        block_0x002036ec,
    ];
    const IDX: [u16; 676usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 3u16,
        0u16, 0u16, 0u16, 4u16, 0u16, 0u16, 0u16, 0u16, 0u16, 5u16, 0u16, 0u16, 0u16,
        0u16, 6u16, 0u16, 7u16, 8u16, 0u16, 0u16, 0u16, 0u16, 0u16, 9u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 10u16, 11u16, 0u16, 0u16,
        0u16, 0u16, 12u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 13u16,
        0u16, 0u16, 0u16, 0u16, 14u16, 15u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        16u16, 0u16, 0u16, 0u16, 17u16, 0u16, 18u16, 0u16, 19u16, 0u16, 20u16, 21u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 22u16, 0u16, 0u16, 23u16, 0u16, 24u16, 25u16,
        0u16, 0u16, 0u16, 0u16, 26u16, 0u16, 0u16, 0u16, 0u16, 0u16, 27u16, 0u16, 0u16,
        0u16, 28u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 29u16, 0u16, 0u16, 0u16, 0u16, 0u16, 30u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 31u16, 0u16, 32u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 33u16, 34u16, 0u16, 0u16, 0u16, 0u16,
        35u16, 0u16, 36u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 37u16, 0u16, 38u16, 0u16, 39u16, 0u16, 40u16, 0u16, 41u16, 0u16,
        42u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 43u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 44u16, 0u16, 45u16, 0u16, 0u16,
        0u16, 46u16, 0u16, 47u16, 48u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 49u16, 0u16, 50u16, 0u16, 0u16, 0u16, 0u16, 0u16, 51u16, 0u16,
        52u16, 0u16, 0u16, 0u16, 0u16, 53u16, 0u16, 0u16, 0u16, 0u16, 0u16, 54u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 55u16, 0u16, 56u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 57u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 58u16, 59u16, 60u16, 0u16, 61u16, 0u16, 62u16, 0u16,
        63u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 64u16, 0u16,
        65u16, 0u16, 66u16, 67u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 68u16,
        0u16, 0u16, 69u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        70u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 71u16, 72u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 73u16, 0u16, 0u16, 74u16, 0u16, 75u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 76u16, 0u16, 0u16, 77u16, 0u16, 0u16, 78u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 79u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 80u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 81u16, 0u16, 0u16,
        82u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 83u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 84u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 85u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 86u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 87u16, 88u16, 0u16, 89u16, 0u16, 0u16,
        0u16, 90u16, 0u16, 0u16, 0u16, 91u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 92u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 93u16,
        0u16, 0u16, 0u16, 0u16, 94u16, 0u16, 0u16, 0u16, 0u16, 0u16, 95u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 96u16, 0u16, 97u16, 0u16,
        0u16, 0u16, 98u16, 99u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 100u16, 0u16, 0u16, 0u16, 101u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 102u16, 0u16, 0u16, 0u16, 103u16, 104u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 105u16,
    ];
    if pc < 2108512u32 || pc > 2111212u32 {
        return None;
    }
    let word_offset = ((pc - 2108512u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x00202c60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2108516u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2108520u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2108524u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2108528u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2108532u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2108536u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2108540u32)?;
    emu.adi_no_count(8usize, 12usize, 0u32, 2108544u32);
    emu.adi_no_count(5usize, 0usize, 2u32, 2108548u32);
    emu.add_memory_rw_events(10usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x00202c88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 3u32, 2108556u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2108792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202d78));
    } else {
        emu.pc = 2108560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202c90));
    }
}
#[inline(always)]
pub fn block_0x00202c90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2108564u32;
    emu.update_insn_clock();
    emu.lw_no_count(10usize, 10usize, 4294965520u32, 2108568u32)?;
    emu.ani_no_count(10usize, 10usize, 1u32, 2108572u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2108824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202d98));
    } else {
        emu.pc = 2108576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202ca0));
    }
}
#[inline(always)]
pub fn block_0x00202ca0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(18usize, a);
    emu.pc = 2108580u32;
    emu.update_insn_clock();
    emu.adi_no_count(18usize, 18usize, 4294965520u32, 2108584u32);
    emu.lbu_no_count(19usize, 18usize, 112u32, 2108588u32);
    emu.adi_no_count(10usize, 0usize, 64u32, 2108592u32);
    emu.sbr_no_count(12usize, 10usize, 19usize, 2108596u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a >= b {
        emu.pc = 2108628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202cd4));
    } else {
        emu.pc = 2108600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202cb8));
    }
}
#[inline(always)]
pub fn block_0x00202cb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 18usize, 19usize, 2108604u32);
    emu.adi_no_count(10usize, 10usize, 48u32, 2108608u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2108612u32);
    emu.apc_no_count(1usize, 2108612u32, 0u32, 2108616u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108620u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(552u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202ccc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(9usize, 8usize, 19usize, 2108624u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2108628u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2108788u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202d74));
}
#[inline(always)]
pub fn block_0x00202cd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2108708u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202d24));
    } else {
        emu.pc = 2108632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202cd8));
    }
}
#[inline(always)]
pub fn block_0x00202cd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(8usize, 8usize, 12usize, 2108636u32);
    emu.adr_no_count(20usize, 11usize, 12usize, 2108640u32);
    emu.adi_no_count(9usize, 18usize, 48u32, 2108644u32);
    emu.adr_no_count(10usize, 9usize, 19usize, 2108648u32);
    emu.apc_no_count(1usize, 2108648u32, 0u32, 2108652u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108656u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(516u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00202cf0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 40u32, 2108660u32)?;
    emu.lw_no_count(11usize, 18usize, 44u32, 2108664u32)?;
    emu.adi_no_count(10usize, 10usize, 1u32, 2108668u32);
    emu.sltiu_no_count(12usize, 10usize, 1u32, 2108672u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2108676u32);
    emu.sw_no_count(10usize, 18usize, 40u32, 2108680u32)?;
    emu.sw_no_count(11usize, 18usize, 44u32, 2108684u32)?;
    emu.adi_no_count(10usize, 18usize, 8u32, 2108688u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2108692u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2108696u32);
    emu.apc_no_count(1usize, 2108696u32, 4096u32, 2108700u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108704u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965732u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202d20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 20usize, 0u32, 2108708u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2108708u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202d24));
}
#[inline(always)]
pub fn block_0x00202d24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 8usize, 4294967232u32, 2108712u32);
    emu.ani_no_count(9usize, 8usize, 63u32, 2108716u32);
    emu.sri_no_count(12usize, 8usize, 6u32, 2108720u32);
    emu.adr_no_count(8usize, 11usize, 10usize, 2108724u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2108768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202d60));
    } else {
        emu.pc = 2108728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202d38));
    }
}
#[inline]
pub fn block_0x00202d38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 40u32, 2108732u32)?;
    emu.lw_no_count(13usize, 18usize, 44u32, 2108736u32)?;
    emu.adr_no_count(14usize, 10usize, 12usize, 2108740u32);
    emu.sltru_no_count(10usize, 14usize, 10usize, 2108744u32);
    emu.adr_no_count(10usize, 13usize, 10usize, 2108748u32);
    emu.sw_no_count(14usize, 18usize, 40u32, 2108752u32)?;
    emu.sw_no_count(10usize, 18usize, 44u32, 2108756u32)?;
    emu.adi_no_count(10usize, 18usize, 8u32, 2108760u32);
    emu.apc_no_count(1usize, 2108760u32, 4096u32, 2108764u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108768u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965668u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202d60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 48u32, 2108772u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2108776u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2108780u32);
    emu.apc_no_count(1usize, 2108780u32, 0u32, 2108784u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108788u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(384u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202d74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(9usize, 18usize, 112u32, 2108792u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2108792u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202d78));
}
#[inline(always)]
pub fn block_0x00202d78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2108796u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2108800u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2108804u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2108808u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2108812u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2108816u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2108820u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108824u32;
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
pub fn block_0x00202d98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2108828u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 220u32, 2108832u32);
    emu.apc_no_count(1usize, 2108832u32, 45056u32, 2108836u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108840u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(536u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202da8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 240u32, 2108844u32);
    emu.add_memory_rw_events(2usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x00202db0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 5usize, 0u32, 2108852u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108856u32;
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
pub fn block_0x00202db8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 241u32, 2108860u32);
    emu.add_memory_rw_events(2usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x00202dc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108868u32;
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
pub fn block_0x00202dc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2108872u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2108876u32)?;
    emu.adi_no_count(12usize, 11usize, 0u32, 2108880u32);
    emu.adi_no_count(11usize, 10usize, 0u32, 2108884u32);
    emu.adi_no_count(10usize, 0usize, 2u32, 2108888u32);
    emu.apc_no_count(1usize, 2108888u32, 0u32, 2108892u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108896u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(20u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202de0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2108900u32);
    emu.apc_no_count(1usize, 2108900u32, 0u32, 2108904u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108908u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965508u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202dec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2108908u32, 0u32, 2108912u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2108916u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966900u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202df4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2109160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202ee8));
    } else {
        emu.pc = 2108920u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202df8));
    }
}
#[inline(always)]
pub fn block_0x00202df8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 10usize, 0u32, 2108924u32);
    emu.adr_no_count(13usize, 12usize, 10usize, 2108928u32);
    emu.adi_no_count(14usize, 0usize, 3u32, 2108932u32);
    emu.sb_no_count(11usize, 13usize, 4294967295u32, 2108936u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2109160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202ee8));
    } else {
        emu.pc = 2108940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202e0c));
    }
}
#[inline(always)]
pub fn block_0x00202e0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 10usize, 1u32, 2108944u32);
    emu.sb_no_count(11usize, 10usize, 2u32, 2108948u32);
    emu.sb_no_count(11usize, 13usize, 4294967294u32, 2108952u32);
    emu.adi_no_count(14usize, 0usize, 7u32, 2108956u32);
    emu.sb_no_count(11usize, 13usize, 4294967293u32, 2108960u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2109160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202ee8));
    } else {
        emu.pc = 2108964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202e24));
    }
}
#[inline(always)]
pub fn block_0x00202e24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 10usize, 3u32, 2108968u32);
    emu.adi_no_count(15usize, 0usize, 9u32, 2108972u32);
    emu.sb_no_count(11usize, 13usize, 4294967292u32, 2108976u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2109160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202ee8));
    } else {
        emu.pc = 2108980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202e34));
    }
}
#[inline]
pub fn block_0x00202e34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(13usize, 0usize, 10usize, 2108984u32);
    emu.ani_no_count(14usize, 13usize, 3u32, 2108988u32);
    emu.adr_no_count(13usize, 10usize, 14usize, 2108992u32);
    emu.sbr_no_count(12usize, 12usize, 14usize, 2108996u32);
    emu.ani_no_count(12usize, 12usize, 4294967292u32, 2109000u32);
    emu.ani_no_count(11usize, 11usize, 255u32, 2109004u32);
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2109008u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 257u32, 2109012u32);
    emu.mul_no_count(11usize, 11usize, 14usize, 2109016u32);
    emu.sw_no_count(11usize, 13usize, 0u32, 2109020u32)?;
    emu.adr_no_count(14usize, 13usize, 12usize, 2109024u32);
    emu.sw_no_count(11usize, 14usize, 4294967292u32, 2109028u32)?;
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2109160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202ee8));
    } else {
        emu.pc = 2109032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202e68));
    }
}
#[inline(always)]
pub fn block_0x00202e68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 13usize, 4u32, 2109036u32)?;
    emu.sw_no_count(11usize, 13usize, 8u32, 2109040u32)?;
    emu.sw_no_count(11usize, 14usize, 4294967284u32, 2109044u32)?;
    emu.adi_no_count(15usize, 0usize, 25u32, 2109048u32);
    emu.sw_no_count(11usize, 14usize, 4294967288u32, 2109052u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2109160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202ee8));
    } else {
        emu.pc = 2109056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202e80));
    }
}
#[inline]
pub fn block_0x00202e80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 13usize, 12u32, 2109060u32)?;
    emu.sw_no_count(11usize, 13usize, 16u32, 2109064u32)?;
    emu.sw_no_count(11usize, 13usize, 20u32, 2109068u32)?;
    emu.sw_no_count(11usize, 13usize, 24u32, 2109072u32)?;
    emu.sw_no_count(11usize, 14usize, 4294967268u32, 2109076u32)?;
    emu.sw_no_count(11usize, 14usize, 4294967272u32, 2109080u32)?;
    emu.sw_no_count(11usize, 14usize, 4294967276u32, 2109084u32)?;
    emu.ani_no_count(15usize, 13usize, 4u32, 2109088u32);
    emu.ori_no_count(15usize, 15usize, 24u32, 2109092u32);
    emu.sbr_no_count(12usize, 12usize, 15usize, 2109096u32);
    emu.adi_no_count(16usize, 0usize, 32u32, 2109100u32);
    emu.sw_no_count(11usize, 14usize, 4294967280u32, 2109104u32)?;
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2109160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202ee8));
    } else {
        emu.pc = 2109108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202eb4));
    }
}
#[inline(always)]
pub fn block_0x00202eb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(13usize, 13usize, 15usize, 2109112u32);
    emu.adi_no_count(14usize, 0usize, 31u32, 2109116u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2109116u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202ebc));
}
#[inline]
pub fn block_0x00202ebc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 13usize, 0u32, 2109120u32)?;
    emu.sw_no_count(11usize, 13usize, 4u32, 2109124u32)?;
    emu.sw_no_count(11usize, 13usize, 8u32, 2109128u32)?;
    emu.sw_no_count(11usize, 13usize, 12u32, 2109132u32)?;
    emu.sw_no_count(11usize, 13usize, 16u32, 2109136u32)?;
    emu.sw_no_count(11usize, 13usize, 20u32, 2109140u32)?;
    emu.sw_no_count(11usize, 13usize, 24u32, 2109144u32)?;
    emu.sw_no_count(11usize, 13usize, 28u32, 2109148u32)?;
    emu.adi_no_count(12usize, 12usize, 4294967264u32, 2109152u32);
    emu.adi_no_count(13usize, 13usize, 32u32, 2109156u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2109116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202ebc));
    } else {
        emu.pc = 2109160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202ee8));
    }
}
#[inline(always)]
pub fn block_0x00202ee8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109164u32;
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
pub fn block_0x00202eec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(13usize, 11usize, 3u32, 2109168u32);
    emu.sltiu_no_count(13usize, 13usize, 1u32, 2109172u32);
    emu.sltiu_no_count(14usize, 12usize, 1u32, 2109176u32);
    emu.orr_no_count(13usize, 13usize, 14usize, 2109180u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2109428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202ff4));
    } else {
        emu.pc = 2109184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202f00));
    }
}
#[inline(always)]
pub fn block_0x00202f00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 11usize, 1u32, 2109188u32);
    emu.adi_no_count(16usize, 10usize, 0u32, 2109192u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2109192u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202f08));
}
#[inline]
pub fn block_0x00202f08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(17usize, 11usize, 0u32, 2109196u32);
    emu.adi_no_count(14usize, 11usize, 1u32, 2109200u32);
    emu.adi_no_count(13usize, 16usize, 1u32, 2109204u32);
    emu.sb_no_count(17usize, 16usize, 0u32, 2109208u32);
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2109212u32);
    emu.ani_no_count(11usize, 15usize, 3u32, 2109216u32);
    emu.sltru_no_count(11usize, 0usize, 11usize, 2109220u32);
    emu.sltru_no_count(16usize, 0usize, 12usize, 2109224u32);
    emu.anr_no_count(17usize, 11usize, 16usize, 2109228u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2109232u32);
    emu.adi_no_count(11usize, 14usize, 0u32, 2109236u32);
    emu.adi_no_count(16usize, 13usize, 0u32, 2109240u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a != b {
        emu.pc = 2109192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202f08));
    } else {
        emu.pc = 2109244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202f3c));
    }
}
#[inline(always)]
pub fn block_0x00202f3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 13usize, 3u32, 2109248u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2109444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203004));
    } else {
        emu.pc = 2109252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202f44));
    }
}
#[inline(always)]
pub fn block_0x00202f44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 32u32, 2109256u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2109828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203184));
    } else {
        emu.pc = 2109260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202f4c));
    }
}
#[inline(always)]
pub fn block_0x00202f4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 3u32, 2109264u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2109564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020307c));
    } else {
        emu.pc = 2109268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202f54));
    }
}
#[inline(always)]
pub fn block_0x00202f54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 2u32, 2109272u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2109692u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002030fc));
    } else {
        emu.pc = 2109276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202f5c));
    }
}
#[inline(always)]
pub fn block_0x00202f5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 1u32, 2109280u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2109828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203184));
    } else {
        emu.pc = 2109284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202f64));
    }
}
#[inline]
pub fn block_0x00202f64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 14usize, 0u32, 2109288u32)?;
    emu.sb_no_count(15usize, 13usize, 0u32, 2109292u32);
    emu.sri_no_count(11usize, 15usize, 8u32, 2109296u32);
    emu.sb_no_count(11usize, 13usize, 1u32, 2109300u32);
    emu.sri_no_count(16usize, 15usize, 16u32, 2109304u32);
    emu.adi_no_count(11usize, 13usize, 3u32, 2109308u32);
    emu.sb_no_count(16usize, 13usize, 2u32, 2109312u32);
    emu.adi_no_count(12usize, 12usize, 4294967293u32, 2109316u32);
    emu.adi_no_count(13usize, 14usize, 16u32, 2109320u32);
    emu.adi_no_count(14usize, 0usize, 16u32, 2109324u32);
    emu.add_memory_rw_events(10usize);
    emu.pc = 2109324u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202f8c));
}
#[inline]
pub fn block_0x00202f8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 13usize, 4294967284u32, 2109328u32)?;
    emu.sri_no_count(15usize, 15usize, 24u32, 2109332u32);
    emu.sli_no_count(17usize, 16usize, 8u32, 2109336u32);
    emu.lw_no_count(5usize, 13usize, 4294967288u32, 2109340u32)?;
    emu.orr_no_count(15usize, 17usize, 15usize, 2109344u32);
    emu.sw_no_count(15usize, 11usize, 0u32, 2109348u32)?;
    emu.sri_no_count(15usize, 16usize, 24u32, 2109352u32);
    emu.sli_no_count(16usize, 5usize, 8u32, 2109356u32);
    emu.lw_no_count(17usize, 13usize, 4294967292u32, 2109360u32)?;
    emu.orr_no_count(15usize, 16usize, 15usize, 2109364u32);
    emu.sw_no_count(15usize, 11usize, 4u32, 2109368u32)?;
    emu.sri_no_count(16usize, 5usize, 24u32, 2109372u32);
    emu.sli_no_count(5usize, 17usize, 8u32, 2109376u32);
    emu.lw_no_count(15usize, 13usize, 0u32, 2109380u32)?;
    emu.orr_no_count(16usize, 5usize, 16usize, 2109384u32);
    emu.sw_no_count(16usize, 11usize, 8u32, 2109388u32)?;
    emu.sri_no_count(16usize, 17usize, 24u32, 2109392u32);
    emu.sli_no_count(17usize, 15usize, 8u32, 2109396u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2109400u32);
    emu.sw_no_count(16usize, 11usize, 12u32, 2109404u32)?;
    emu.adi_no_count(11usize, 11usize, 16u32, 2109408u32);
    emu.adi_no_count(12usize, 12usize, 4294967280u32, 2109412u32);
    emu.adi_no_count(13usize, 13usize, 16u32, 2109416u32);
    emu.add_memory_rw_events(23usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2109324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202f8c));
    } else {
        emu.pc = 2109420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202fec));
    }
}
#[inline(always)]
pub fn block_0x00202fec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 13usize, 4294967283u32, 2109424u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2109428u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2109824u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203180));
}
#[inline(always)]
pub fn block_0x00202ff4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 10usize, 0u32, 2109432u32);
    emu.adi_no_count(14usize, 11usize, 0u32, 2109436u32);
    emu.ani_no_count(11usize, 13usize, 3u32, 2109440u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2109252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202f44));
    } else {
        emu.pc = 2109444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203004));
    }
}
#[inline(always)]
pub fn block_0x00203004(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 16u32, 2109448u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2109504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203040));
    } else {
        emu.pc = 2109452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020300c));
    }
}
#[inline(always)]
pub fn block_0x0020300c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 15u32, 2109456u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2109456u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203010));
}
#[inline]
pub fn block_0x00203010(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 14usize, 0u32, 2109460u32)?;
    emu.lw_no_count(16usize, 14usize, 4u32, 2109464u32)?;
    emu.lw_no_count(17usize, 14usize, 8u32, 2109468u32)?;
    emu.lw_no_count(5usize, 14usize, 12u32, 2109472u32)?;
    emu.sw_no_count(15usize, 13usize, 0u32, 2109476u32)?;
    emu.sw_no_count(16usize, 13usize, 4u32, 2109480u32)?;
    emu.sw_no_count(17usize, 13usize, 8u32, 2109484u32)?;
    emu.sw_no_count(5usize, 13usize, 12u32, 2109488u32)?;
    emu.adi_no_count(14usize, 14usize, 16u32, 2109492u32);
    emu.adi_no_count(12usize, 12usize, 4294967280u32, 2109496u32);
    emu.adi_no_count(13usize, 13usize, 16u32, 2109500u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2109456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203010));
    } else {
        emu.pc = 2109504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203040));
    }
}
#[inline(always)]
pub fn block_0x00203040(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 12usize, 8u32, 2109508u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2109536u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203060));
    } else {
        emu.pc = 2109512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203048));
    }
}
#[inline(always)]
pub fn block_0x00203048(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 14usize, 0u32, 2109516u32)?;
    emu.lw_no_count(15usize, 14usize, 4u32, 2109520u32)?;
    emu.sw_no_count(11usize, 13usize, 0u32, 2109524u32)?;
    emu.sw_no_count(15usize, 13usize, 4u32, 2109528u32)?;
    emu.adi_no_count(13usize, 13usize, 8u32, 2109532u32);
    emu.adi_no_count(14usize, 14usize, 8u32, 2109536u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2109536u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203060));
}
#[inline(always)]
pub fn block_0x00203060(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 12usize, 4u32, 2109540u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2109896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002031c8));
    } else {
        emu.pc = 2109544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203068));
    }
}
#[inline(always)]
pub fn block_0x00203068(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 14usize, 0u32, 2109548u32)?;
    emu.sw_no_count(11usize, 13usize, 0u32, 2109552u32)?;
    emu.adi_no_count(13usize, 13usize, 4u32, 2109556u32);
    emu.adi_no_count(14usize, 14usize, 4u32, 2109560u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2109564u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2109896u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002031c8));
}
#[inline(always)]
pub fn block_0x0020307c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 14usize, 0u32, 2109568u32)?;
    emu.adi_no_count(11usize, 13usize, 1u32, 2109572u32);
    emu.sb_no_count(15usize, 13usize, 0u32, 2109576u32);
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2109580u32);
    emu.adi_no_count(13usize, 14usize, 16u32, 2109584u32);
    emu.adi_no_count(14usize, 0usize, 18u32, 2109588u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2109588u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203094));
}
#[inline]
pub fn block_0x00203094(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 13usize, 4294967284u32, 2109592u32)?;
    emu.sri_no_count(15usize, 15usize, 8u32, 2109596u32);
    emu.sli_no_count(17usize, 16usize, 24u32, 2109600u32);
    emu.lw_no_count(5usize, 13usize, 4294967288u32, 2109604u32)?;
    emu.orr_no_count(15usize, 17usize, 15usize, 2109608u32);
    emu.sw_no_count(15usize, 11usize, 0u32, 2109612u32)?;
    emu.sri_no_count(15usize, 16usize, 8u32, 2109616u32);
    emu.sli_no_count(16usize, 5usize, 24u32, 2109620u32);
    emu.lw_no_count(17usize, 13usize, 4294967292u32, 2109624u32)?;
    emu.orr_no_count(15usize, 16usize, 15usize, 2109628u32);
    emu.sw_no_count(15usize, 11usize, 4u32, 2109632u32)?;
    emu.sri_no_count(16usize, 5usize, 8u32, 2109636u32);
    emu.sli_no_count(5usize, 17usize, 24u32, 2109640u32);
    emu.lw_no_count(15usize, 13usize, 0u32, 2109644u32)?;
    emu.orr_no_count(16usize, 5usize, 16usize, 2109648u32);
    emu.sw_no_count(16usize, 11usize, 8u32, 2109652u32)?;
    emu.sri_no_count(16usize, 17usize, 8u32, 2109656u32);
    emu.sli_no_count(17usize, 15usize, 24u32, 2109660u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2109664u32);
    emu.sw_no_count(16usize, 11usize, 12u32, 2109668u32)?;
    emu.adi_no_count(11usize, 11usize, 16u32, 2109672u32);
    emu.adi_no_count(12usize, 12usize, 4294967280u32, 2109676u32);
    emu.adi_no_count(13usize, 13usize, 16u32, 2109680u32);
    emu.add_memory_rw_events(23usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2109588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203094));
    } else {
        emu.pc = 2109684u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002030f4));
    }
}
#[inline(always)]
pub fn block_0x002030f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 13usize, 4294967281u32, 2109688u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2109692u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2109824u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203180));
}
#[inline(always)]
pub fn block_0x002030fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 14usize, 0u32, 2109696u32)?;
    emu.sb_no_count(15usize, 13usize, 0u32, 2109700u32);
    emu.sri_no_count(16usize, 15usize, 8u32, 2109704u32);
    emu.adi_no_count(11usize, 13usize, 2u32, 2109708u32);
    emu.sb_no_count(16usize, 13usize, 1u32, 2109712u32);
    emu.adi_no_count(12usize, 12usize, 4294967294u32, 2109716u32);
    emu.adi_no_count(13usize, 14usize, 16u32, 2109720u32);
    emu.adi_no_count(14usize, 0usize, 17u32, 2109724u32);
    emu.add_memory_rw_events(8usize);
    emu.pc = 2109724u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020311c));
}
#[inline]
pub fn block_0x0020311c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 13usize, 4294967284u32, 2109728u32)?;
    emu.sri_no_count(15usize, 15usize, 16u32, 2109732u32);
    emu.sli_no_count(17usize, 16usize, 16u32, 2109736u32);
    emu.lw_no_count(5usize, 13usize, 4294967288u32, 2109740u32)?;
    emu.orr_no_count(15usize, 17usize, 15usize, 2109744u32);
    emu.sw_no_count(15usize, 11usize, 0u32, 2109748u32)?;
    emu.sri_no_count(15usize, 16usize, 16u32, 2109752u32);
    emu.sli_no_count(16usize, 5usize, 16u32, 2109756u32);
    emu.lw_no_count(17usize, 13usize, 4294967292u32, 2109760u32)?;
    emu.orr_no_count(15usize, 16usize, 15usize, 2109764u32);
    emu.sw_no_count(15usize, 11usize, 4u32, 2109768u32)?;
    emu.sri_no_count(16usize, 5usize, 16u32, 2109772u32);
    emu.sli_no_count(5usize, 17usize, 16u32, 2109776u32);
    emu.lw_no_count(15usize, 13usize, 0u32, 2109780u32)?;
    emu.orr_no_count(16usize, 5usize, 16usize, 2109784u32);
    emu.sw_no_count(16usize, 11usize, 8u32, 2109788u32)?;
    emu.sri_no_count(16usize, 17usize, 16u32, 2109792u32);
    emu.sli_no_count(17usize, 15usize, 16u32, 2109796u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2109800u32);
    emu.sw_no_count(16usize, 11usize, 12u32, 2109804u32)?;
    emu.adi_no_count(11usize, 11usize, 16u32, 2109808u32);
    emu.adi_no_count(12usize, 12usize, 4294967280u32, 2109812u32);
    emu.adi_no_count(13usize, 13usize, 16u32, 2109816u32);
    emu.add_memory_rw_events(23usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2109724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020311c));
    } else {
        emu.pc = 2109820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020317c));
    }
}
#[inline(always)]
pub fn block_0x0020317c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 13usize, 4294967282u32, 2109824u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2109824u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203180));
}
#[inline(always)]
pub fn block_0x00203180(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 11usize, 0u32, 2109828u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2109828u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203184));
}
#[inline(always)]
pub fn block_0x00203184(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 12usize, 16u32, 2109832u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2109964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020320c));
    } else {
        emu.pc = 2109836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020318c));
    }
}
#[inline(always)]
pub fn block_0x0020318c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 12usize, 8u32, 2109840u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2110112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002032a0));
    } else {
        emu.pc = 2109844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203194));
    }
}
#[inline(always)]
pub fn block_0x00203194(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 12usize, 4u32, 2109848u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2109896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002031c8));
    } else {
        emu.pc = 2109852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020319c));
    }
}
#[inline]
pub fn block_0x0020319c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(11usize, 14usize, 0u32, 2109856u32);
    emu.lb_no_count(15usize, 14usize, 1u32, 2109860u32);
    emu.lb_no_count(16usize, 14usize, 2u32, 2109864u32);
    emu.sb_no_count(11usize, 13usize, 0u32, 2109868u32);
    emu.sb_no_count(15usize, 13usize, 1u32, 2109872u32);
    emu.lb_no_count(11usize, 14usize, 3u32, 2109876u32);
    emu.sb_no_count(16usize, 13usize, 2u32, 2109880u32);
    emu.adi_no_count(14usize, 14usize, 4u32, 2109884u32);
    emu.adi_no_count(15usize, 13usize, 4u32, 2109888u32);
    emu.sb_no_count(11usize, 13usize, 3u32, 2109892u32);
    emu.adi_no_count(13usize, 15usize, 0u32, 2109896u32);
    emu.add_memory_rw_events(11usize);
    emu.pc = 2109896u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002031c8));
}
#[inline(always)]
pub fn block_0x002031c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 12usize, 2u32, 2109900u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2109916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002031dc));
    } else {
        emu.pc = 2109904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002031d0));
    }
}
#[inline(always)]
pub fn block_0x002031d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 12usize, 1u32, 2109908u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2109952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203200));
    } else {
        emu.pc = 2109912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002031d8));
    }
}
#[inline(always)]
pub fn block_0x002031d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109916u32;
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
pub fn block_0x002031dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(11usize, 14usize, 0u32, 2109920u32);
    emu.lb_no_count(15usize, 14usize, 1u32, 2109924u32);
    emu.sb_no_count(11usize, 13usize, 0u32, 2109928u32);
    emu.adi_no_count(14usize, 14usize, 2u32, 2109932u32);
    emu.adi_no_count(11usize, 13usize, 2u32, 2109936u32);
    emu.sb_no_count(15usize, 13usize, 1u32, 2109940u32);
    emu.adi_no_count(13usize, 11usize, 0u32, 2109944u32);
    emu.ani_no_count(11usize, 12usize, 1u32, 2109948u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2109912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002031d8));
    } else {
        emu.pc = 2109952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203200));
    }
}
#[inline(always)]
pub fn block_0x00203200(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(11usize, 14usize, 0u32, 2109956u32);
    emu.sb_no_count(11usize, 13usize, 0u32, 2109960u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109964u32;
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
#[inline(never)]
pub fn block_0x0020320c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(11usize, 14usize, 0u32, 2109968u32);
    emu.lb_no_count(15usize, 14usize, 1u32, 2109972u32);
    emu.lb_no_count(16usize, 14usize, 2u32, 2109976u32);
    emu.sb_no_count(11usize, 13usize, 0u32, 2109980u32);
    emu.sb_no_count(15usize, 13usize, 1u32, 2109984u32);
    emu.lb_no_count(11usize, 14usize, 3u32, 2109988u32);
    emu.sb_no_count(16usize, 13usize, 2u32, 2109992u32);
    emu.lb_no_count(15usize, 14usize, 4u32, 2109996u32);
    emu.lb_no_count(16usize, 14usize, 5u32, 2110000u32);
    emu.sb_no_count(11usize, 13usize, 3u32, 2110004u32);
    emu.lb_no_count(11usize, 14usize, 6u32, 2110008u32);
    emu.sb_no_count(15usize, 13usize, 4u32, 2110012u32);
    emu.sb_no_count(16usize, 13usize, 5u32, 2110016u32);
    emu.lb_no_count(15usize, 14usize, 7u32, 2110020u32);
    emu.sb_no_count(11usize, 13usize, 6u32, 2110024u32);
    emu.lb_no_count(11usize, 14usize, 8u32, 2110028u32);
    emu.lb_no_count(16usize, 14usize, 9u32, 2110032u32);
    emu.sb_no_count(15usize, 13usize, 7u32, 2110036u32);
    emu.lb_no_count(15usize, 14usize, 10u32, 2110040u32);
    emu.sb_no_count(11usize, 13usize, 8u32, 2110044u32);
    emu.sb_no_count(16usize, 13usize, 9u32, 2110048u32);
    emu.lb_no_count(11usize, 14usize, 11u32, 2110052u32);
    emu.sb_no_count(15usize, 13usize, 10u32, 2110056u32);
    emu.lb_no_count(15usize, 14usize, 12u32, 2110060u32);
    emu.lb_no_count(16usize, 14usize, 13u32, 2110064u32);
    emu.sb_no_count(11usize, 13usize, 11u32, 2110068u32);
    emu.lb_no_count(11usize, 14usize, 14u32, 2110072u32);
    emu.sb_no_count(15usize, 13usize, 12u32, 2110076u32);
    emu.sb_no_count(16usize, 13usize, 13u32, 2110080u32);
    emu.lb_no_count(15usize, 14usize, 15u32, 2110084u32);
    emu.sb_no_count(11usize, 13usize, 14u32, 2110088u32);
    emu.adi_no_count(14usize, 14usize, 16u32, 2110092u32);
    emu.adi_no_count(11usize, 13usize, 16u32, 2110096u32);
    emu.sb_no_count(15usize, 13usize, 15u32, 2110100u32);
    emu.adi_no_count(13usize, 11usize, 0u32, 2110104u32);
    emu.ani_no_count(11usize, 12usize, 8u32, 2110108u32);
    emu.add_memory_rw_events(36usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2109844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203194));
    } else {
        emu.pc = 2110112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002032a0));
    }
}
#[inline]
pub fn block_0x002032a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(11usize, 14usize, 0u32, 2110116u32);
    emu.lb_no_count(15usize, 14usize, 1u32, 2110120u32);
    emu.lb_no_count(16usize, 14usize, 2u32, 2110124u32);
    emu.sb_no_count(11usize, 13usize, 0u32, 2110128u32);
    emu.sb_no_count(15usize, 13usize, 1u32, 2110132u32);
    emu.lb_no_count(11usize, 14usize, 3u32, 2110136u32);
    emu.sb_no_count(16usize, 13usize, 2u32, 2110140u32);
    emu.lb_no_count(15usize, 14usize, 4u32, 2110144u32);
    emu.lb_no_count(16usize, 14usize, 5u32, 2110148u32);
    emu.sb_no_count(11usize, 13usize, 3u32, 2110152u32);
    emu.lb_no_count(11usize, 14usize, 6u32, 2110156u32);
    emu.sb_no_count(15usize, 13usize, 4u32, 2110160u32);
    emu.sb_no_count(16usize, 13usize, 5u32, 2110164u32);
    emu.lb_no_count(15usize, 14usize, 7u32, 2110168u32);
    emu.sb_no_count(11usize, 13usize, 6u32, 2110172u32);
    emu.adi_no_count(14usize, 14usize, 8u32, 2110176u32);
    emu.adi_no_count(11usize, 13usize, 8u32, 2110180u32);
    emu.sb_no_count(15usize, 13usize, 7u32, 2110184u32);
    emu.adi_no_count(13usize, 11usize, 0u32, 2110188u32);
    emu.ani_no_count(11usize, 12usize, 4u32, 2110192u32);
    emu.add_memory_rw_events(20usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2109852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020319c));
    } else {
        emu.pc = 2110196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002032f4));
    }
}
#[inline(always)]
pub fn block_0x002032f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2110200u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2109896u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002031c8));
}
#[inline(always)]
pub fn block_0x002032f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(3usize, 2110200u32, 4292923392u32, 2110204u32);
    emu.adi_no_count(3usize, 3usize, 1288u32, 2110208u32);
    emu.apc_no_count(2usize, 2110208u32, 77824u32, 2110212u32);
    emu.adi_no_count(2usize, 2usize, 4294966824u32, 2110216u32);
    emu.lw_no_count(2usize, 2usize, 0u32, 2110220u32)?;
    emu.apc_no_count(1usize, 2110220u32, 0u32, 2110224u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110228u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(148u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203314(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2110232u32;
    emu.update_insn_clock();
    emu.lw_no_count(10usize, 10usize, 4294965472u32, 2110236u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2110248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203328));
    } else {
        emu.pc = 2110240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203320));
    }
}
#[inline(always)]
pub fn block_0x00203320(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2110244u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965708u32, 2110248u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2110248u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203328));
}
#[inline]
pub fn block_0x00203328(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 11usize, 4294967295u32, 2110252u32);
    emu.anr_no_count(13usize, 10usize, 13usize, 2110256u32);
    emu.sltiu_no_count(14usize, 13usize, 1u32, 2110260u32);
    emu.sbr_no_count(11usize, 11usize, 13usize, 2110264u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2110268u32);
    emu.anr_no_count(11usize, 14usize, 11usize, 2110272u32);
    emu.adr_no_count(10usize, 11usize, 10usize, 2110276u32);
    emu.adr_no_count(12usize, 10usize, 12usize, 2110280u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2110308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203364));
    } else {
        emu.pc = 2110284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020334c));
    }
}
#[inline(always)]
pub fn block_0x0020334c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2013265920u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2110288u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1u32, 2110292u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a >= b {
        emu.pc = 2110308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203364));
    } else {
        emu.pc = 2110296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203358));
    }
}
#[inline(always)]
pub fn block_0x00203358(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2110300u32;
    emu.update_insn_clock();
    emu.sw_no_count(12usize, 11usize, 4294965472u32, 2110304u32)?;
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110308u32;
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
pub fn block_0x00203364(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2110312u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2110316u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 272u32, 2110320u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2110324u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2110328u32)?;
    emu.adi_no_count(12usize, 0usize, 4u32, 2110332u32);
    emu.sw_no_count(10usize, 2usize, 8u32, 2110336u32)?;
    emu.sw_no_count(11usize, 2usize, 12u32, 2110340u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2110344u32)?;
    emu.sw_no_count(0usize, 2usize, 20u32, 2110348u32)?;
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2110352u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 280u32, 2110356u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2110360u32);
    emu.apc_no_count(1usize, 2110360u32, 36864u32, 2110364u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110368u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966836u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x002033a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 39u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2110372u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2110376u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2110380u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2110384u32)?;
    emu.sw_no_count(18usize, 2usize, 0u32, 2110388u32)?;
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(8usize, a);
    emu.pc = 2110392u32;
    emu.update_insn_clock();
    emu.adi_no_count(9usize, 0usize, 1u32, 2110396u32);
    let a = 0u32.wrapping_add(1779032064u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2110400u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(3144134656u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2110404u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1013903360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2110408u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2773479424u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2110412u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1359892480u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2110416u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2600824832u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2110420u32;
    emu.update_insn_clock();
    emu.adi_no_count(18usize, 8usize, 4294965480u32, 2110424u32);
    emu.adi_no_count(10usize, 10usize, 1639u32, 2110428u32);
    emu.adi_no_count(11usize, 11usize, 4294966917u32, 2110432u32);
    emu.sw_no_count(9usize, 18usize, 40u32, 2110436u32)?;
    emu.sw_no_count(0usize, 18usize, 44u32, 2110440u32)?;
    emu.sw_no_count(10usize, 18usize, 48u32, 2110444u32)?;
    emu.sw_no_count(11usize, 18usize, 52u32, 2110448u32)?;
    let a = 0u32.wrapping_add(528736256u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2110452u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 12usize, 882u32, 2110456u32);
    emu.adi_no_count(12usize, 13usize, 1338u32, 2110460u32);
    emu.adi_no_count(13usize, 14usize, 639u32, 2110464u32);
    emu.adi_no_count(14usize, 15usize, 4294965388u32, 2110468u32);
    emu.sw_no_count(11usize, 18usize, 56u32, 2110472u32)?;
    emu.sw_no_count(12usize, 18usize, 60u32, 2110476u32)?;
    emu.sw_no_count(13usize, 18usize, 64u32, 2110480u32)?;
    emu.sw_no_count(14usize, 18usize, 68u32, 2110484u32)?;
    let a = 0u32.wrapping_add(1541459968u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2110488u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965675u32, 2110492u32);
    emu.adi_no_count(11usize, 11usize, 4294966553u32, 2110496u32);
    emu.sw_no_count(10usize, 18usize, 72u32, 2110500u32)?;
    emu.sw_no_count(11usize, 18usize, 76u32, 2110504u32)?;
    emu.adi_no_count(10usize, 18usize, 80u32, 2110508u32);
    emu.adi_no_count(12usize, 0usize, 73u32, 2110512u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2110516u32);
    emu.apc_no_count(1usize, 2110516u32, 0u32, 2110520u32);
    emu.add_memory_rw_events(39usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110524u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965696u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020343c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(9usize, 8usize, 4294965480u32, 2110528u32)?;
    emu.sw_no_count(0usize, 8usize, 4294965484u32, 2110532u32)?;
    emu.sw_no_count(0usize, 18usize, 8u32, 2110536u32)?;
    emu.sw_no_count(0usize, 18usize, 12u32, 2110540u32)?;
    emu.sw_no_count(0usize, 18usize, 16u32, 2110544u32)?;
    emu.sw_no_count(0usize, 18usize, 20u32, 2110548u32)?;
    emu.sw_no_count(0usize, 18usize, 24u32, 2110552u32)?;
    emu.sw_no_count(0usize, 18usize, 28u32, 2110556u32)?;
    emu.sw_no_count(0usize, 18usize, 32u32, 2110560u32)?;
    emu.apc_no_count(1usize, 2110560u32, 4294959104u32, 2110564u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110568u32;
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
pub fn block_0x00203468(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2110572u32);
    emu.apc_no_count(1usize, 2110572u32, 4294963200u32, 2110576u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110580u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(636u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203474(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2110584u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 300u32, 2110588u32);
    emu.adi_no_count(12usize, 0usize, 11u32, 2110592u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2110596u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2110600u32);
    emu.apc_no_count(6usize, 2110600u32, 53248u32, 2110604u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2110608u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1888u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00203490(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2110612u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2110616u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2110620u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2110624u32)?;
    emu.adi_no_count(8usize, 13usize, 0u32, 2110628u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2110632u32)?;
    emu.adi_no_count(9usize, 10usize, 0u32, 2110636u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2110640u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2110644u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2110648u32);
    emu.apc_no_count(1usize, 2110648u32, 4294963200u32, 2110652u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110656u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1960u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002034c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2110660u32);
    emu.sb_no_count(10usize, 9usize, 0u32, 2110664u32);
    emu.sw_no_count(8usize, 9usize, 4u32, 2110668u32)?;
    emu.lw_no_count(1usize, 2usize, 12u32, 2110672u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2110676u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2110680u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2110684u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110688u32;
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
pub fn block_0x002034e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2110692u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2110696u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2110700u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2110704u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2110708u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2110712u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2110716u32);
    emu.apc_no_count(1usize, 2110716u32, 0u32, 2110720u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110724u32;
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
#[inline]
pub fn block_0x00203504(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2110728u32);
    emu.ani_no_count(10usize, 10usize, 3u32, 2110732u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2110736u32);
    emu.sli_no_count(10usize, 10usize, 2u32, 2110740u32);
    emu.adr_no_count(10usize, 10usize, 9usize, 2110744u32);
    emu.ani_no_count(18usize, 10usize, 4294967292u32, 2110748u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2110752u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2110756u32);
    emu.apc_no_count(1usize, 2110756u32, 32768u32, 2110760u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110764u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1004u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020352c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2110848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203580));
    } else {
        emu.pc = 2110768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203530));
    }
}
#[inline(always)]
pub fn block_0x00203530(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2110768u32, 4294963200u32, 2110772u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110776u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(92u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203538(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 4u32, 2110780u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2110784u32);
    emu.apc_no_count(1usize, 2110784u32, 4294959104u32, 2110788u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110792u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(448u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203548(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 10usize, 0u32, 2110796u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2110800u32);
    emu.apc_no_count(1usize, 2110800u32, 0u32, 2110804u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110808u32;
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
pub fn block_0x00203558(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 8usize, 0u32, 2110812u32)?;
    emu.sw_no_count(19usize, 8usize, 4u32, 2110816u32)?;
    emu.sw_no_count(9usize, 8usize, 8u32, 2110820u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2110824u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2110828u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2110832u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2110836u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2110840u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2110844u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110848u32;
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
pub fn block_0x00203580(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2110852u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 328u32, 2110856u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2110860u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 312u32, 2110864u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2110868u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 472u32, 2110872u32);
    emu.adi_no_count(11usize, 0usize, 16u32, 2110876u32);
    emu.adi_no_count(12usize, 2usize, 11u32, 2110880u32);
    emu.apc_no_count(1usize, 2110880u32, 49152u32, 2110884u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110888u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(796u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002035a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2110892u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2110896u32)?;
    emu.sw_no_count(10usize, 2usize, 0u32, 2110900u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2110904u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2110932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002035d4));
    } else {
        emu.pc = 2110908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002035bc));
    }
}
#[inline(always)]
pub fn block_0x002035bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2110912u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2110916u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2110920u32);
    emu.lw_no_count(1usize, 2usize, 60u32, 2110924u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2110928u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110932u32;
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
#[inline(never)]
pub fn block_0x002035d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 25u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2110936u32);
    let a = 0u32.wrapping_add(2146304u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2110940u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966940u32, 2110944u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2110948u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 72u32, 2110952u32);
    let a = 0u32.wrapping_add(2146304u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2110956u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294965756u32, 2110960u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2110964u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 536u32, 2110968u32);
    emu.adi_no_count(15usize, 0usize, 3u32, 2110972u32);
    emu.sw_no_count(0usize, 2usize, 36u32, 2110976u32)?;
    emu.adi_no_count(16usize, 2usize, 44u32, 2110980u32);
    emu.sw_no_count(10usize, 2usize, 44u32, 2110984u32)?;
    emu.sw_no_count(11usize, 2usize, 48u32, 2110988u32)?;
    emu.sw_no_count(12usize, 2usize, 52u32, 2110992u32)?;
    emu.sw_no_count(13usize, 2usize, 56u32, 2110996u32)?;
    emu.adi_no_count(10usize, 0usize, 2u32, 2111000u32);
    emu.sw_no_count(14usize, 2usize, 20u32, 2111004u32)?;
    emu.sw_no_count(15usize, 2usize, 24u32, 2111008u32)?;
    emu.sw_no_count(16usize, 2usize, 28u32, 2111012u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2111016u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2111020u32);
    emu.adi_no_count(11usize, 2usize, 20u32, 2111024u32);
    emu.apc_no_count(1usize, 2111024u32, 32768u32, 2111028u32);
    emu.add_memory_rw_events(25usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111032u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966636u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203638(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2111032u32, 4294963200u32, 2111036u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111040u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967124u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203640(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 12u32, 2111044u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2111048u32);
    emu.apc_no_count(1usize, 2111048u32, 4294959104u32, 2111052u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111056u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(184u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203650(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2111108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203684));
    } else {
        emu.pc = 2111060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203654));
    }
}
#[inline]
pub fn block_0x00203654(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2111064u32);
    emu.lw_no_count(10usize, 2usize, 8u32, 2111068u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2111072u32)?;
    emu.lw_no_count(13usize, 2usize, 16u32, 2111076u32)?;
    emu.sw_no_count(10usize, 12usize, 0u32, 2111080u32)?;
    emu.sw_no_count(11usize, 12usize, 4u32, 2111084u32)?;
    emu.sw_no_count(13usize, 12usize, 8u32, 2111088u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2111092u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2111096u32);
    emu.lw_no_count(1usize, 2usize, 60u32, 2111100u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2111104u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111108u32;
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
pub fn block_0x00203684(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2111112u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2111116u32);
    emu.apc_no_count(1usize, 2111116u32, 32768u32, 2111120u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111124u32;
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
pub fn block_0x00203694(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2111128u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2111132u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2111136u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2111140u32)?;
    emu.lw_no_count(8usize, 10usize, 0u32, 2111144u32)?;
    emu.lw_no_count(9usize, 10usize, 4u32, 2111148u32)?;
    emu.apc_no_count(1usize, 2111148u32, 4294963200u32, 2111152u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111156u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967008u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002036b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 12u32, 2111160u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2111164u32);
    emu.apc_no_count(1usize, 2111164u32, 4294959104u32, 2111168u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111172u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(68u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002036c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2111212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002036ec));
    } else {
        emu.pc = 2111176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002036c8));
    }
}
#[inline]
pub fn block_0x002036c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2111180u32;
    emu.update_insn_clock();
    emu.sw_no_count(11usize, 10usize, 0u32, 2111184u32)?;
    emu.sw_no_count(8usize, 10usize, 4u32, 2111188u32)?;
    emu.sw_no_count(9usize, 10usize, 8u32, 2111192u32)?;
    emu.lw_no_count(1usize, 2usize, 12u32, 2111196u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2111200u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2111204u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2111208u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111212u32;
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
pub fn block_0x002036ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2111216u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2111220u32);
    emu.apc_no_count(1usize, 2111220u32, 32768u32, 2111224u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111228u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965820u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
