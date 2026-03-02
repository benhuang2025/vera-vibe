pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2177504u32;
pub const PC_MAX: u32 = 2179516u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 97usize] = [
        block_0x002139e0,
        block_0x002139e8,
        block_0x002139f0,
        block_0x002139f8,
        block_0x00213a0c,
        block_0x00213a14,
        block_0x00213a18,
        block_0x00213a30,
        block_0x00213a34,
        block_0x00213a40,
        block_0x00213a4c,
        block_0x00213a54,
        block_0x00213a58,
        block_0x00213a70,
        block_0x00213a78,
        block_0x00213a80,
        block_0x00213a9c,
        block_0x00213ab8,
        block_0x00213aec,
        block_0x00213af0,
        block_0x00213b04,
        block_0x00213b08,
        block_0x00213b10,
        block_0x00213b1c,
        block_0x00213b98,
        block_0x00213b9c,
        block_0x00213bc0,
        block_0x00213be8,
        block_0x00213c0c,
        block_0x00213c6c,
        block_0x00213ccc,
        block_0x00213d2c,
        block_0x00213d50,
        block_0x00213d64,
        block_0x00213d68,
        block_0x00213d6c,
        block_0x00213d70,
        block_0x00213d74,
        block_0x00213d88,
        block_0x00213d9c,
        block_0x00213db4,
        block_0x00213dcc,
        block_0x00213dd0,
        block_0x00213dec,
        block_0x00213dfc,
        block_0x00213e14,
        block_0x00213e18,
        block_0x00213e24,
        block_0x00213e28,
        block_0x00213e34,
        block_0x00213e38,
        block_0x00213e4c,
        block_0x00213e60,
        block_0x00213e78,
        block_0x00213e90,
        block_0x00213e94,
        block_0x00213ed4,
        block_0x00213ee8,
        block_0x00213ef8,
        block_0x00213f00,
        block_0x00213f0c,
        block_0x00213f10,
        block_0x00213f1c,
        block_0x00213f24,
        block_0x00213f38,
        block_0x00213f4c,
        block_0x00213f6c,
        block_0x00213f70,
        block_0x00213fa4,
        block_0x00213fbc,
        block_0x00213fd8,
        block_0x00213ff4,
        block_0x00213ff8,
        block_0x00214010,
        block_0x0021402c,
        block_0x00214048,
        block_0x0021404c,
        block_0x00214060,
        block_0x00214064,
        block_0x0021407c,
        block_0x00214090,
        block_0x002140a0,
        block_0x002140ac,
        block_0x002140b4,
        block_0x002140d8,
        block_0x002140ec,
        block_0x00214144,
        block_0x00214148,
        block_0x00214150,
        block_0x00214154,
        block_0x00214158,
        block_0x0021416c,
        block_0x00214184,
        block_0x00214198,
        block_0x002141a8,
        block_0x002141b4,
        block_0x002141bc,
    ];
    const IDX: [u16; 504usize] = [
        1u16, 0u16, 2u16, 0u16, 3u16, 0u16, 4u16, 0u16, 0u16, 0u16, 0u16, 5u16, 0u16,
        6u16, 7u16, 0u16, 0u16, 0u16, 0u16, 0u16, 8u16, 9u16, 0u16, 0u16, 10u16, 0u16,
        0u16, 11u16, 0u16, 12u16, 13u16, 0u16, 0u16, 0u16, 0u16, 0u16, 14u16, 0u16,
        15u16, 0u16, 16u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 17u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 18u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 19u16, 20u16, 0u16, 0u16, 0u16, 0u16, 21u16, 22u16, 0u16,
        23u16, 0u16, 0u16, 24u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 25u16, 26u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 27u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 28u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 29u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 30u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 31u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 32u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 33u16, 0u16, 0u16, 0u16, 0u16, 34u16, 35u16, 36u16, 37u16, 38u16, 0u16,
        0u16, 0u16, 0u16, 39u16, 0u16, 0u16, 0u16, 0u16, 40u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 41u16, 0u16, 0u16, 0u16, 0u16, 0u16, 42u16, 43u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 44u16, 0u16, 0u16, 0u16, 45u16, 0u16, 0u16, 0u16, 0u16, 0u16, 46u16,
        47u16, 0u16, 0u16, 48u16, 49u16, 0u16, 0u16, 50u16, 51u16, 0u16, 0u16, 0u16,
        0u16, 52u16, 0u16, 0u16, 0u16, 0u16, 53u16, 0u16, 0u16, 0u16, 0u16, 0u16, 54u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 55u16, 56u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 57u16, 0u16, 0u16, 0u16,
        0u16, 58u16, 0u16, 0u16, 0u16, 59u16, 0u16, 60u16, 0u16, 0u16, 61u16, 62u16,
        0u16, 0u16, 63u16, 0u16, 64u16, 0u16, 0u16, 0u16, 0u16, 65u16, 0u16, 0u16, 0u16,
        0u16, 66u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 67u16, 68u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 69u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 70u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 71u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 72u16, 73u16, 0u16, 0u16, 0u16, 0u16, 0u16, 74u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 75u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 76u16,
        77u16, 0u16, 0u16, 0u16, 0u16, 78u16, 79u16, 0u16, 0u16, 0u16, 0u16, 0u16, 80u16,
        0u16, 0u16, 0u16, 0u16, 81u16, 0u16, 0u16, 0u16, 82u16, 0u16, 0u16, 83u16, 0u16,
        84u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 85u16, 0u16, 0u16, 0u16,
        0u16, 86u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 87u16, 88u16, 0u16,
        89u16, 90u16, 91u16, 0u16, 0u16, 0u16, 0u16, 92u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        93u16, 0u16, 0u16, 0u16, 0u16, 94u16, 0u16, 0u16, 0u16, 95u16, 0u16, 0u16, 96u16,
        0u16, 97u16,
    ];
    if pc < 2177504u32 || pc > 2179516u32 {
        return None;
    }
    let word_offset = ((pc - 2177504u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x002139e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2177504u32, 0u32, 2177508u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2177512u32;
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
pub fn block_0x002139e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2177512u32, 0u32, 2177516u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2177520u32;
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
pub fn block_0x002139f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2177520u32, 0u32, 2177524u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2177528u32;
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
pub fn block_0x002139f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2177532u32);
    emu.adi_no_count(10usize, 10usize, 3u32, 2177536u32);
    emu.ani_no_count(10usize, 10usize, 4294967292u32, 2177540u32);
    emu.sbr_no_count(5usize, 10usize, 12usize, 2177544u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2177588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213a34));
    } else {
        emu.pc = 2177548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213a0c));
    }
}
#[inline(always)]
pub fn block_0x00213a0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2177552u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2177584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213a30));
    } else {
        emu.pc = 2177556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213a14));
    }
}
#[inline(always)]
pub fn block_0x00213a14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 12usize, 11usize, 2177560u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2177560u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213a18));
}
#[inline(always)]
pub fn block_0x00213a18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(13usize, 12usize, 0u32, 2177564u32);
    emu.adi_no_count(12usize, 12usize, 1u32, 2177568u32);
    emu.slti_no_count(13usize, 13usize, 4294967232u32, 2177572u32);
    emu.xri_no_count(13usize, 13usize, 1u32, 2177576u32);
    emu.adr_no_count(10usize, 10usize, 13usize, 2177580u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2177560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213a18));
    } else {
        emu.pc = 2177584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213a30));
    }
}
#[inline(always)]
pub fn block_0x00213a30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2177588u32;
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
pub fn block_0x00213a34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(13usize, 11usize, 5usize, 2177592u32);
    emu.sri_no_count(17usize, 13usize, 2u32, 2177596u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2177548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213a0c));
    } else {
        emu.pc = 2177600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213a40));
    }
}
#[inline(always)]
pub fn block_0x00213a40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(5usize, 12usize, 5usize, 2177604u32);
    emu.ani_no_count(11usize, 13usize, 3u32, 2177608u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2177620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213a54));
    } else {
        emu.pc = 2177612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213a4c));
    }
}
#[inline(always)]
pub fn block_0x00213a4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2177616u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2177620u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2177648u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213a70));
}
#[inline(always)]
pub fn block_0x00213a54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2177624u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2177624u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213a58));
}
#[inline(always)]
pub fn block_0x00213a58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(14usize, 12usize, 0u32, 2177628u32);
    emu.adi_no_count(12usize, 12usize, 1u32, 2177632u32);
    emu.slti_no_count(14usize, 14usize, 4294967232u32, 2177636u32);
    emu.xri_no_count(14usize, 14usize, 1u32, 2177640u32);
    emu.adr_no_count(10usize, 10usize, 14usize, 2177644u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2177624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213a58));
    } else {
        emu.pc = 2177648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213a70));
    }
}
#[inline(always)]
pub fn block_0x00213a70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2177652u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2177692u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213a9c));
    } else {
        emu.pc = 2177656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213a78));
    }
}
#[inline(always)]
pub fn block_0x00213a78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(13usize, 13usize, 4294967292u32, 2177660u32);
    emu.adr_no_count(13usize, 5usize, 13usize, 2177664u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2177664u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213a80));
}
#[inline(always)]
pub fn block_0x00213a80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(14usize, 13usize, 0u32, 2177668u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2177672u32);
    emu.slti_no_count(14usize, 14usize, 4294967232u32, 2177676u32);
    emu.xri_no_count(14usize, 14usize, 1u32, 2177680u32);
    emu.adr_no_count(12usize, 12usize, 14usize, 2177684u32);
    emu.adi_no_count(13usize, 13usize, 1u32, 2177688u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2177664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213a80));
    } else {
        emu.pc = 2177692u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213a9c));
    }
}
#[inline(always)]
pub fn block_0x00213a9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2177696u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(16711680u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2177700u32;
    emu.update_insn_clock();
    emu.adr_no_count(10usize, 12usize, 10usize, 2177704u32);
    emu.adi_no_count(12usize, 11usize, 257u32, 2177708u32);
    emu.adi_no_count(11usize, 13usize, 255u32, 2177712u32);
    emu.adi_no_count(14usize, 0usize, 4u32, 2177716u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2177720u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2177772u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213aec));
}
#[inline]
pub fn block_0x00213ab8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(6usize, 16usize, 2u32, 2177724u32);
    emu.sbr_no_count(17usize, 13usize, 16usize, 2177728u32);
    emu.ani_no_count(7usize, 16usize, 3u32, 2177732u32);
    emu.anr_no_count(28usize, 5usize, 11usize, 2177736u32);
    emu.sri_no_count(29usize, 5usize, 8u32, 2177740u32);
    emu.adr_no_count(5usize, 15usize, 6usize, 2177744u32);
    emu.anr_no_count(6usize, 29usize, 11usize, 2177748u32);
    emu.adr_no_count(6usize, 6usize, 28usize, 2177752u32);
    emu.sli_no_count(28usize, 6usize, 16u32, 2177756u32);
    emu.adr_no_count(6usize, 28usize, 6usize, 2177760u32);
    emu.sri_no_count(6usize, 6usize, 16u32, 2177764u32);
    emu.adr_no_count(10usize, 6usize, 10usize, 2177768u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a != b {
        emu.pc = 2177948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213b9c));
    } else {
        emu.pc = 2177772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213aec));
    }
}
#[inline(always)]
pub fn block_0x00213aec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2177584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213a30));
    } else {
        emu.pc = 2177776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213af0));
    }
}
#[inline(always)]
pub fn block_0x00213af0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 17usize, 0u32, 2177780u32);
    emu.adi_no_count(15usize, 5usize, 0u32, 2177784u32);
    emu.adi_no_count(17usize, 0usize, 192u32, 2177788u32);
    emu.adi_no_count(16usize, 13usize, 0u32, 2177792u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2177800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213b08));
    } else {
        emu.pc = 2177796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213b04));
    }
}
#[inline(always)]
pub fn block_0x00213b04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 192u32, 2177800u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2177800u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213b08));
}
#[inline(always)]
pub fn block_0x00213b08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 0u32, 2177804u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2177720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213ab8));
    } else {
        emu.pc = 2177808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213b10));
    }
}
#[inline(always)]
pub fn block_0x00213b10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(17usize, 16usize, 2u32, 2177812u32);
    emu.sli_no_count(17usize, 17usize, 4u32, 2177816u32);
    emu.adi_no_count(6usize, 15usize, 0u32, 2177820u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2177820u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213b1c));
}
#[inline(never)]
pub fn block_0x00213b1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 31u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(7usize, 6usize, 0u32, 2177824u32)?;
    emu.lw_no_count(28usize, 6usize, 4u32, 2177828u32)?;
    emu.lw_no_count(29usize, 6usize, 8u32, 2177832u32)?;
    emu.lw_no_count(30usize, 6usize, 12u32, 2177836u32)?;
    emu.xri_no_count(31usize, 7usize, 4294967295u32, 2177840u32);
    emu.sri_no_count(7usize, 7usize, 6u32, 2177844u32);
    emu.sri_no_count(31usize, 31usize, 7u32, 2177848u32);
    emu.orr_no_count(7usize, 31usize, 7usize, 2177852u32);
    emu.xri_no_count(31usize, 28usize, 4294967295u32, 2177856u32);
    emu.sri_no_count(28usize, 28usize, 6u32, 2177860u32);
    emu.sri_no_count(31usize, 31usize, 7u32, 2177864u32);
    emu.orr_no_count(28usize, 31usize, 28usize, 2177868u32);
    emu.xri_no_count(31usize, 29usize, 4294967295u32, 2177872u32);
    emu.sri_no_count(29usize, 29usize, 6u32, 2177876u32);
    emu.sri_no_count(31usize, 31usize, 7u32, 2177880u32);
    emu.orr_no_count(29usize, 31usize, 29usize, 2177884u32);
    emu.xri_no_count(31usize, 30usize, 4294967295u32, 2177888u32);
    emu.sri_no_count(30usize, 30usize, 6u32, 2177892u32);
    emu.sri_no_count(31usize, 31usize, 7u32, 2177896u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2177900u32);
    emu.anr_no_count(7usize, 7usize, 12usize, 2177904u32);
    emu.adr_no_count(5usize, 7usize, 5usize, 2177908u32);
    emu.adi_no_count(17usize, 17usize, 4294967280u32, 2177912u32);
    emu.anr_no_count(7usize, 28usize, 12usize, 2177916u32);
    emu.anr_no_count(28usize, 29usize, 12usize, 2177920u32);
    emu.anr_no_count(29usize, 30usize, 12usize, 2177924u32);
    emu.adr_no_count(7usize, 7usize, 28usize, 2177928u32);
    emu.adr_no_count(5usize, 5usize, 7usize, 2177932u32);
    emu.adr_no_count(5usize, 5usize, 29usize, 2177936u32);
    emu.adi_no_count(6usize, 6usize, 16u32, 2177940u32);
    emu.add_memory_rw_events(30usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a != b {
        emu.pc = 2177820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213b1c));
    } else {
        emu.pc = 2177944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213b98));
    }
}
#[inline(always)]
pub fn block_0x00213b98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2177948u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2177720u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213ab8));
}
#[inline]
pub fn block_0x00213b9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 0u32, 2177952u32);
    emu.ani_no_count(16usize, 16usize, 252u32, 2177956u32);
    emu.sli_no_count(16usize, 16usize, 2u32, 2177960u32);
    emu.adr_no_count(15usize, 15usize, 16usize, 2177964u32);
    emu.sltiu_no_count(16usize, 13usize, 192u32, 2177968u32);
    emu.sbr_no_count(16usize, 0usize, 16usize, 2177972u32);
    emu.anr_no_count(13usize, 13usize, 16usize, 2177976u32);
    emu.ani_no_count(13usize, 13usize, 3u32, 2177980u32);
    emu.sli_no_count(13usize, 13usize, 2u32, 2177984u32);
    emu.add_memory_rw_events(9usize);
    emu.pc = 2177984u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213bc0));
}
#[inline]
pub fn block_0x00213bc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 15usize, 0u32, 2177988u32)?;
    emu.adi_no_count(15usize, 15usize, 4u32, 2177992u32);
    emu.xri_no_count(17usize, 16usize, 4294967295u32, 2177996u32);
    emu.sri_no_count(16usize, 16usize, 6u32, 2178000u32);
    emu.sri_no_count(17usize, 17usize, 7u32, 2178004u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2178008u32);
    emu.anr_no_count(16usize, 16usize, 12usize, 2178012u32);
    emu.adi_no_count(13usize, 13usize, 4294967292u32, 2178016u32);
    emu.adr_no_count(14usize, 16usize, 14usize, 2178020u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2177984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213bc0));
    } else {
        emu.pc = 2178024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213be8));
    }
}
#[inline]
pub fn block_0x00213be8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(12usize, 14usize, 11usize, 2178028u32);
    emu.sri_no_count(14usize, 14usize, 8u32, 2178032u32);
    emu.anr_no_count(11usize, 14usize, 11usize, 2178036u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2178040u32);
    emu.sli_no_count(12usize, 11usize, 16u32, 2178044u32);
    emu.adr_no_count(11usize, 12usize, 11usize, 2178048u32);
    emu.sri_no_count(11usize, 11usize, 16u32, 2178052u32);
    emu.adr_no_count(10usize, 11usize, 10usize, 2178056u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2178060u32;
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
pub fn block_0x00213c0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2178064u32);
    emu.sw_no_count(10usize, 2usize, 0u32, 2178068u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2178072u32)?;
    emu.adi_no_count(10usize, 2usize, 0u32, 2178076u32);
    let a = 0u32.wrapping_add(2146304u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2178080u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965756u32, 2178084u32);
    emu.adi_no_count(13usize, 2usize, 4u32, 2178088u32);
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2178092u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965284u32, 2178096u32);
    emu.adi_no_count(15usize, 0usize, 2u32, 2178100u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2178104u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2178108u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2178112u32)?;
    emu.sw_no_count(13usize, 2usize, 40u32, 2178116u32)?;
    emu.sw_no_count(11usize, 2usize, 44u32, 2178120u32)?;
    emu.adi_no_count(10usize, 2usize, 32u32, 2178124u32);
    emu.sw_no_count(14usize, 2usize, 8u32, 2178128u32)?;
    emu.sw_no_count(15usize, 2usize, 12u32, 2178132u32)?;
    emu.sw_no_count(10usize, 2usize, 16u32, 2178136u32)?;
    emu.sw_no_count(15usize, 2usize, 20u32, 2178140u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2178144u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2178148u32);
    emu.apc_no_count(1usize, 2178148u32, 4294934528u32, 2178152u32);
    emu.add_memory_rw_events(24usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2178156u32;
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
#[inline]
pub fn block_0x00213c6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2178160u32);
    emu.sw_no_count(10usize, 2usize, 0u32, 2178164u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2178168u32)?;
    emu.adi_no_count(10usize, 2usize, 0u32, 2178172u32);
    let a = 0u32.wrapping_add(2146304u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2178176u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965756u32, 2178180u32);
    emu.adi_no_count(13usize, 2usize, 4u32, 2178184u32);
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2178188u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965300u32, 2178192u32);
    emu.adi_no_count(15usize, 0usize, 2u32, 2178196u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2178200u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2178204u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2178208u32)?;
    emu.sw_no_count(13usize, 2usize, 40u32, 2178212u32)?;
    emu.sw_no_count(11usize, 2usize, 44u32, 2178216u32)?;
    emu.adi_no_count(10usize, 2usize, 32u32, 2178220u32);
    emu.sw_no_count(14usize, 2usize, 8u32, 2178224u32)?;
    emu.sw_no_count(15usize, 2usize, 12u32, 2178228u32)?;
    emu.sw_no_count(10usize, 2usize, 16u32, 2178232u32)?;
    emu.sw_no_count(15usize, 2usize, 20u32, 2178236u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2178240u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2178244u32);
    emu.apc_no_count(1usize, 2178244u32, 4294934528u32, 2178248u32);
    emu.add_memory_rw_events(24usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2178252u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1288u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00213ccc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2178256u32);
    emu.sw_no_count(10usize, 2usize, 0u32, 2178260u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2178264u32)?;
    emu.adi_no_count(10usize, 2usize, 0u32, 2178268u32);
    let a = 0u32.wrapping_add(2146304u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2178272u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965756u32, 2178276u32);
    emu.adi_no_count(13usize, 2usize, 4u32, 2178280u32);
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2178284u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965352u32, 2178288u32);
    emu.adi_no_count(15usize, 0usize, 2u32, 2178292u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2178296u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2178300u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2178304u32)?;
    emu.sw_no_count(13usize, 2usize, 40u32, 2178308u32)?;
    emu.sw_no_count(11usize, 2usize, 44u32, 2178312u32)?;
    emu.adi_no_count(10usize, 2usize, 32u32, 2178316u32);
    emu.sw_no_count(14usize, 2usize, 8u32, 2178320u32)?;
    emu.sw_no_count(15usize, 2usize, 12u32, 2178324u32)?;
    emu.sw_no_count(10usize, 2usize, 16u32, 2178328u32)?;
    emu.sw_no_count(15usize, 2usize, 20u32, 2178332u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2178336u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2178340u32);
    emu.apc_no_count(1usize, 2178340u32, 4294934528u32, 2178344u32);
    emu.add_memory_rw_events(24usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2178348u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1192u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00213d2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2178352u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2178356u32)?;
    emu.adi_no_count(14usize, 13usize, 0u32, 2178360u32);
    emu.adi_no_count(13usize, 12usize, 0u32, 2178364u32);
    emu.adi_no_count(12usize, 11usize, 0u32, 2178368u32);
    emu.adi_no_count(11usize, 10usize, 0u32, 2178372u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2178376u32);
    emu.apc_no_count(1usize, 2178376u32, 0u32, 2178380u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2178384u32;
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
pub fn block_0x00213d50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2178388u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2178392u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2178396u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2178400u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2178404u32;
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
pub fn block_0x00213d64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2178580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213e14));
    } else {
        emu.pc = 2178408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213d68));
    }
}
#[inline(always)]
pub fn block_0x00213d68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2178580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213e14));
    } else {
        emu.pc = 2178412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213d6c));
    }
}
#[inline(always)]
pub fn block_0x00213d6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2178892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213f4c));
    } else {
        emu.pc = 2178416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213d70));
    }
}
#[inline(always)]
pub fn block_0x00213d70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2178924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213f6c));
    } else {
        emu.pc = 2178420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213d74));
    }
}
#[inline(always)]
pub fn block_0x00213d74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(17usize, 13usize, 16u32, 2178424u32);
    emu.sltru_no_count(15usize, 17usize, 12usize, 2178428u32);
    emu.xri_no_count(16usize, 15usize, 1u32, 2178432u32);
    emu.adi_no_count(15usize, 13usize, 0u32, 2178436u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2179064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213ff8));
    } else {
        emu.pc = 2178440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213d88));
    }
}
#[inline(always)]
pub fn block_0x00213d88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(16usize, 16usize, 4u32, 2178444u32);
    emu.sri_no_count(5usize, 15usize, 8u32, 2178448u32);
    emu.sltru_no_count(17usize, 5usize, 12usize, 2178452u32);
    emu.xri_no_count(17usize, 17usize, 1u32, 2178456u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a >= b {
        emu.pc = 2179088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00214010));
    } else {
        emu.pc = 2178460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213d9c));
    }
}
#[inline(always)]
pub fn block_0x00213d9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(5usize, 17usize, 3u32, 2178464u32);
    emu.sri_no_count(17usize, 15usize, 4u32, 2178468u32);
    emu.orr_no_count(16usize, 5usize, 16usize, 2178472u32);
    emu.sltru_no_count(5usize, 17usize, 12usize, 2178476u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2178480u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2179116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021402c));
    } else {
        emu.pc = 2178484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213db4));
    }
}
#[inline(always)]
pub fn block_0x00213db4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(5usize, 5usize, 2u32, 2178488u32);
    emu.sri_no_count(17usize, 15usize, 2u32, 2178492u32);
    emu.orr_no_count(16usize, 16usize, 5usize, 2178496u32);
    emu.sltru_no_count(5usize, 17usize, 12usize, 2178500u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2178504u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a < b {
        emu.pc = 2178512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213dd0));
    } else {
        emu.pc = 2178508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213dcc));
    }
}
#[inline(always)]
pub fn block_0x00213dcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 17usize, 0u32, 2178512u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2178512u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213dd0));
}
#[inline(always)]
pub fn block_0x00213dd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(5usize, 5usize, 1u32, 2178516u32);
    emu.sri_no_count(15usize, 15usize, 1u32, 2178520u32);
    emu.sltru_no_count(15usize, 15usize, 12usize, 2178524u32);
    emu.xri_no_count(15usize, 15usize, 1u32, 2178528u32);
    emu.orr_no_count(15usize, 5usize, 15usize, 2178532u32);
    emu.orr_no_count(15usize, 16usize, 15usize, 2178536u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2179148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021404c));
    } else {
        emu.pc = 2178540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213dec));
    }
}
#[inline(always)]
pub fn block_0x00213dec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 31u32, 2178544u32);
    emu.adi_no_count(5usize, 16usize, 4294967264u32, 2178548u32);
    emu.slr_no_count(17usize, 13usize, 16usize, 2178552u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2179168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00214060));
    } else {
        emu.pc = 2178556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213dfc));
    }
}
#[inline(always)]
pub fn block_0x00213dfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.slr_no_count(14usize, 14usize, 16usize, 2178560u32);
    emu.xri_no_count(15usize, 16usize, 4294967295u32, 2178564u32);
    emu.sri_no_count(6usize, 13usize, 1u32, 2178568u32);
    emu.srr_no_count(15usize, 6usize, 15usize, 2178572u32);
    emu.orr_no_count(15usize, 14usize, 15usize, 2178576u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2178580u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2179172u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00214064));
}
#[inline(always)]
pub fn block_0x00213e14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2178600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213e28));
    } else {
        emu.pc = 2178584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213e18));
    }
}
#[inline(always)]
pub fn block_0x00213e18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 12usize, 14usize, 2178588u32);
    emu.adi_no_count(17usize, 0usize, 0u32, 2178592u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2178612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213e34));
    } else {
        emu.pc = 2178596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213e24));
    }
}
#[inline(always)]
pub fn block_0x00213e24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2178600u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2178852u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213f24));
}
#[inline(always)]
pub fn block_0x00213e28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 11usize, 13usize, 2178604u32);
    emu.adi_no_count(17usize, 0usize, 0u32, 2178608u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2178852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213f24));
    } else {
        emu.pc = 2178612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213e34));
    }
}
#[inline(always)]
pub fn block_0x00213e34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2178852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213f24));
    } else {
        emu.pc = 2178616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213e38));
    }
}
#[inline(always)]
pub fn block_0x00213e38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(17usize, 12usize, 16u32, 2178620u32);
    emu.sltru_no_count(15usize, 17usize, 14usize, 2178624u32);
    emu.xri_no_count(16usize, 15usize, 1u32, 2178628u32);
    emu.adi_no_count(15usize, 12usize, 0u32, 2178632u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2178980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213fa4));
    } else {
        emu.pc = 2178636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213e4c));
    }
}
#[inline(always)]
pub fn block_0x00213e4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(16usize, 16usize, 4u32, 2178640u32);
    emu.sri_no_count(5usize, 15usize, 8u32, 2178644u32);
    emu.sltru_no_count(17usize, 5usize, 14usize, 2178648u32);
    emu.xri_no_count(17usize, 17usize, 1u32, 2178652u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a >= b {
        emu.pc = 2179004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213fbc));
    } else {
        emu.pc = 2178656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213e60));
    }
}
#[inline(always)]
pub fn block_0x00213e60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(5usize, 17usize, 3u32, 2178660u32);
    emu.sri_no_count(17usize, 15usize, 4u32, 2178664u32);
    emu.orr_no_count(16usize, 5usize, 16usize, 2178668u32);
    emu.sltru_no_count(5usize, 17usize, 14usize, 2178672u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2178676u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2179032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213fd8));
    } else {
        emu.pc = 2178680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213e78));
    }
}
#[inline(always)]
pub fn block_0x00213e78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(5usize, 5usize, 2u32, 2178684u32);
    emu.sri_no_count(17usize, 15usize, 2u32, 2178688u32);
    emu.orr_no_count(16usize, 16usize, 5usize, 2178692u32);
    emu.sltru_no_count(5usize, 17usize, 14usize, 2178696u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2178700u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a < b {
        emu.pc = 2178708u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213e94));
    } else {
        emu.pc = 2178704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213e90));
    }
}
#[inline(always)]
pub fn block_0x00213e90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 17usize, 0u32, 2178708u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2178708u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213e94));
}
#[inline]
pub fn block_0x00213e94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 0usize, 0u32, 2178712u32);
    emu.sli_no_count(5usize, 5usize, 1u32, 2178716u32);
    emu.sri_no_count(15usize, 15usize, 1u32, 2178720u32);
    emu.sltru_no_count(15usize, 15usize, 14usize, 2178724u32);
    emu.xri_no_count(15usize, 15usize, 1u32, 2178728u32);
    emu.orr_no_count(15usize, 5usize, 15usize, 2178732u32);
    emu.sri_no_count(5usize, 13usize, 1u32, 2178736u32);
    emu.orr_no_count(16usize, 16usize, 15usize, 2178740u32);
    emu.xri_no_count(15usize, 16usize, 31u32, 2178744u32);
    emu.srr_no_count(15usize, 5usize, 15usize, 2178748u32);
    emu.slr_no_count(5usize, 14usize, 16usize, 2178752u32);
    emu.orr_no_count(15usize, 5usize, 15usize, 2178756u32);
    emu.adi_no_count(5usize, 0usize, 1u32, 2178760u32);
    emu.slr_no_count(5usize, 5usize, 16usize, 2178764u32);
    emu.slr_no_count(6usize, 13usize, 16usize, 2178768u32);
    emu.add_memory_rw_events(16usize);
    let return_addr = 2178772u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2178792u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213ee8));
}
#[inline(always)]
pub fn block_0x00213ed4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(16usize, 6usize, 1u32, 2178776u32);
    emu.sli_no_count(6usize, 15usize, 31u32, 2178780u32);
    emu.sri_no_count(15usize, 15usize, 1u32, 2178784u32);
    emu.orr_no_count(6usize, 16usize, 6usize, 2178788u32);
    emu.sri_no_count(5usize, 5usize, 1u32, 2178792u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2178792u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213ee8));
}
#[inline(always)]
pub fn block_0x00213ee8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 11usize, 6usize, 2178796u32);
    emu.sbr_no_count(7usize, 12usize, 15usize, 2178800u32);
    emu.sbr_no_count(16usize, 7usize, 16usize, 2178804u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2178772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213ed4));
    } else {
        emu.pc = 2178808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213ef8));
    }
}
#[inline(always)]
pub fn block_0x00213ef8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 11usize, 6usize, 2178812u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2178832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213f10));
    } else {
        emu.pc = 2178816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213f00));
    }
}
#[inline(always)]
pub fn block_0x00213f00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 16usize, 14usize, 2178820u32);
    emu.orr_no_count(17usize, 5usize, 17usize, 2178824u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2178844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213f1c));
    } else {
        emu.pc = 2178828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213f0c));
    }
}
#[inline(always)]
pub fn block_0x00213f0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2178832u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2178872u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213f38));
}
#[inline(always)]
pub fn block_0x00213f10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 11usize, 13usize, 2178836u32);
    emu.orr_no_count(17usize, 5usize, 17usize, 2178840u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2178872u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213f38));
    } else {
        emu.pc = 2178844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213f1c));
    }
}
#[inline(always)]
pub fn block_0x00213f1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 16usize, 0u32, 2178848u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2178852u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2178772u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213ed4));
}
#[inline(always)]
pub fn block_0x00213f24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(17usize, 10usize, 0u32, 2178856u32)?;
    emu.sw_no_count(17usize, 10usize, 4u32, 2178860u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2178864u32)?;
    emu.sw_no_count(12usize, 10usize, 12u32, 2178868u32)?;
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2178872u32;
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
pub fn block_0x00213f38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(17usize, 10usize, 0u32, 2178876u32)?;
    emu.sw_no_count(0usize, 10usize, 4u32, 2178880u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2178884u32)?;
    emu.sw_no_count(16usize, 10usize, 12u32, 2178888u32)?;
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2178892u32;
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
pub fn block_0x00213f4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.divu_no_count(17usize, 11usize, 13usize, 2178896u32);
    emu.mul_no_count(12usize, 17usize, 13usize, 2178900u32);
    emu.sbr_no_count(11usize, 11usize, 12usize, 2178904u32);
    emu.sw_no_count(17usize, 10usize, 0u32, 2178908u32)?;
    emu.sw_no_count(0usize, 10usize, 4u32, 2178912u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2178916u32)?;
    emu.sw_no_count(0usize, 10usize, 12u32, 2178920u32)?;
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2178924u32;
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
pub fn block_0x00213f6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2179288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002140d8));
    } else {
        emu.pc = 2178928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213f70));
    }
}
#[inline]
pub fn block_0x00213f70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.divu_no_count(13usize, 11usize, 12usize, 2178932u32);
    emu.mul_no_count(12usize, 13usize, 12usize, 2178936u32);
    emu.sltru_no_count(15usize, 0usize, 13usize, 2178940u32);
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2178944u32);
    emu.sbr_no_count(11usize, 11usize, 12usize, 2178948u32);
    emu.adi_no_count(17usize, 13usize, 1u32, 2178952u32);
    emu.sltiu_no_count(12usize, 17usize, 1u32, 2178956u32);
    emu.adr_no_count(15usize, 15usize, 12usize, 2178960u32);
    emu.sw_no_count(17usize, 10usize, 0u32, 2178964u32)?;
    emu.sw_no_count(15usize, 10usize, 4u32, 2178968u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2178972u32)?;
    emu.sw_no_count(0usize, 10usize, 12u32, 2178976u32)?;
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2178980u32;
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
pub fn block_0x00213fa4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 17usize, 0u32, 2178984u32);
    emu.sli_no_count(16usize, 16usize, 4u32, 2178988u32);
    emu.sri_no_count(5usize, 17usize, 8u32, 2178992u32);
    emu.sltru_no_count(17usize, 5usize, 14usize, 2178996u32);
    emu.xri_no_count(17usize, 17usize, 1u32, 2179000u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a < b {
        emu.pc = 2178656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213e60));
    } else {
        emu.pc = 2179004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213fbc));
    }
}
#[inline(always)]
pub fn block_0x00213fbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 5usize, 0u32, 2179008u32);
    emu.sli_no_count(5usize, 17usize, 3u32, 2179012u32);
    emu.sri_no_count(17usize, 15usize, 4u32, 2179016u32);
    emu.orr_no_count(16usize, 5usize, 16usize, 2179020u32);
    emu.sltru_no_count(5usize, 17usize, 14usize, 2179024u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2179028u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a < b {
        emu.pc = 2178680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213e78));
    } else {
        emu.pc = 2179032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213fd8));
    }
}
#[inline(always)]
pub fn block_0x00213fd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 17usize, 0u32, 2179036u32);
    emu.sli_no_count(5usize, 5usize, 2u32, 2179040u32);
    emu.sri_no_count(17usize, 17usize, 2u32, 2179044u32);
    emu.orr_no_count(16usize, 16usize, 5usize, 2179048u32);
    emu.sltru_no_count(5usize, 17usize, 14usize, 2179052u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2179056u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2178704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213e90));
    } else {
        emu.pc = 2179060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213ff4));
    }
}
#[inline(always)]
pub fn block_0x00213ff4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2179064u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2178708u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213e94));
}
#[inline(always)]
pub fn block_0x00213ff8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 17usize, 0u32, 2179068u32);
    emu.sli_no_count(16usize, 16usize, 4u32, 2179072u32);
    emu.sri_no_count(5usize, 17usize, 8u32, 2179076u32);
    emu.sltru_no_count(17usize, 5usize, 12usize, 2179080u32);
    emu.xri_no_count(17usize, 17usize, 1u32, 2179084u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a < b {
        emu.pc = 2178460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213d9c));
    } else {
        emu.pc = 2179088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00214010));
    }
}
#[inline(always)]
pub fn block_0x00214010(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 5usize, 0u32, 2179092u32);
    emu.sli_no_count(5usize, 17usize, 3u32, 2179096u32);
    emu.sri_no_count(17usize, 15usize, 4u32, 2179100u32);
    emu.orr_no_count(16usize, 5usize, 16usize, 2179104u32);
    emu.sltru_no_count(5usize, 17usize, 12usize, 2179108u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2179112u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a < b {
        emu.pc = 2178484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213db4));
    } else {
        emu.pc = 2179116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021402c));
    }
}
#[inline(always)]
pub fn block_0x0021402c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 17usize, 0u32, 2179120u32);
    emu.sli_no_count(5usize, 5usize, 2u32, 2179124u32);
    emu.sri_no_count(17usize, 17usize, 2u32, 2179128u32);
    emu.orr_no_count(16usize, 16usize, 5usize, 2179132u32);
    emu.sltru_no_count(5usize, 17usize, 12usize, 2179136u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2179140u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2178508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213dcc));
    } else {
        emu.pc = 2179144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00214048));
    }
}
#[inline(always)]
pub fn block_0x00214048(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2179148u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2178512u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213dd0));
}
#[inline(always)]
pub fn block_0x0021404c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 32u32, 2179152u32);
    emu.sbr_no_count(16usize, 16usize, 15usize, 2179156u32);
    emu.adi_no_count(5usize, 16usize, 4294967264u32, 2179160u32);
    emu.slr_no_count(17usize, 13usize, 16usize, 2179164u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2178556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213dfc));
    } else {
        emu.pc = 2179168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00214060));
    }
}
#[inline(always)]
pub fn block_0x00214060(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 17usize, 0u32, 2179172u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2179172u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00214064));
}
#[inline(always)]
pub fn block_0x00214064(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 0u32, 2179176u32);
    emu.sai_no_count(5usize, 5usize, 1055u32, 2179180u32);
    emu.anr_no_count(17usize, 5usize, 17usize, 2179184u32);
    emu.adi_no_count(5usize, 0usize, 1u32, 2179188u32);
    emu.slr_no_count(16usize, 5usize, 16usize, 2179192u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2179196u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2179216u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00214090));
}
#[inline(always)]
pub fn block_0x0021407c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(17usize, 17usize, 1u32, 2179200u32);
    emu.sli_no_count(5usize, 15usize, 31u32, 2179204u32);
    emu.sri_no_count(15usize, 15usize, 1u32, 2179208u32);
    emu.orr_no_count(17usize, 17usize, 5usize, 2179212u32);
    emu.sri_no_count(16usize, 16usize, 1u32, 2179216u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2179216u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00214090));
}
#[inline(always)]
pub fn block_0x00214090(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 11usize, 17usize, 2179220u32);
    emu.sbr_no_count(6usize, 12usize, 15usize, 2179224u32);
    emu.sbr_no_count(5usize, 6usize, 5usize, 2179228u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2179196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021407c));
    } else {
        emu.pc = 2179232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002140a0));
    }
}
#[inline(always)]
pub fn block_0x002140a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 11usize, 17usize, 2179236u32);
    emu.orr_no_count(14usize, 16usize, 14usize, 2179240u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2179252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002140b4));
    } else {
        emu.pc = 2179244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002140ac));
    }
}
#[inline(always)]
pub fn block_0x002140ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 5usize, 0u32, 2179248u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2179252u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2179196u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021407c));
}
#[inline]
pub fn block_0x002140b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.divu_no_count(12usize, 11usize, 13usize, 2179256u32);
    emu.mul_no_count(13usize, 12usize, 13usize, 2179260u32);
    emu.sbr_no_count(11usize, 11usize, 13usize, 2179264u32);
    emu.orr_no_count(17usize, 12usize, 14usize, 2179268u32);
    emu.sw_no_count(17usize, 10usize, 0u32, 2179272u32)?;
    emu.sw_no_count(0usize, 10usize, 4u32, 2179276u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2179280u32)?;
    emu.sw_no_count(0usize, 10usize, 12u32, 2179284u32)?;
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2179288u32;
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
pub fn block_0x002140d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(17usize, 13usize, 16u32, 2179292u32);
    emu.divu_no_count(15usize, 12usize, 13usize, 2179296u32);
    emu.mul_no_count(16usize, 15usize, 13usize, 2179300u32);
    emu.sbr_no_count(16usize, 12usize, 16usize, 2179304u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a != b {
        emu.pc = 2179396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00214144));
    } else {
        emu.pc = 2179308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002140ec));
    }
}
#[inline]
pub fn block_0x002140ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 11usize, 16u32, 2179312u32);
    emu.sli_no_count(16usize, 16usize, 16u32, 2179316u32);
    emu.sli_no_count(11usize, 11usize, 16u32, 2179320u32);
    emu.orr_no_count(14usize, 16usize, 12usize, 2179324u32);
    emu.sri_no_count(11usize, 11usize, 16u32, 2179328u32);
    emu.divu_no_count(14usize, 14usize, 13usize, 2179332u32);
    emu.mul_no_count(16usize, 14usize, 13usize, 2179336u32);
    emu.sbr_no_count(12usize, 12usize, 16usize, 2179340u32);
    emu.sli_no_count(16usize, 14usize, 16u32, 2179344u32);
    emu.sri_no_count(14usize, 14usize, 16u32, 2179348u32);
    emu.orr_no_count(15usize, 14usize, 15usize, 2179352u32);
    emu.sli_no_count(12usize, 12usize, 16u32, 2179356u32);
    emu.orr_no_count(11usize, 12usize, 11usize, 2179360u32);
    emu.divu_no_count(12usize, 11usize, 13usize, 2179364u32);
    emu.mul_no_count(13usize, 12usize, 13usize, 2179368u32);
    emu.sbr_no_count(11usize, 11usize, 13usize, 2179372u32);
    emu.orr_no_count(17usize, 16usize, 12usize, 2179376u32);
    emu.sw_no_count(17usize, 10usize, 0u32, 2179380u32)?;
    emu.sw_no_count(15usize, 10usize, 4u32, 2179384u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2179388u32)?;
    emu.sw_no_count(0usize, 10usize, 12u32, 2179392u32)?;
    emu.add_memory_rw_events(22usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2179396u32;
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
pub fn block_0x00214144(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2179408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00214150));
    } else {
        emu.pc = 2179400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00214148));
    }
}
#[inline(always)]
pub fn block_0x00214148(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 16usize, 14usize, 2179404u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2179408u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2179412u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00214154));
}
#[inline(always)]
pub fn block_0x00214150(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 11usize, 13usize, 2179412u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2179412u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00214154));
}
#[inline(always)]
pub fn block_0x00214154(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2179436u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021416c));
    } else {
        emu.pc = 2179416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00214158));
    }
}
#[inline(always)]
pub fn block_0x00214158(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 10usize, 0u32, 2179420u32)?;
    emu.sw_no_count(15usize, 10usize, 4u32, 2179424u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2179428u32)?;
    emu.sw_no_count(16usize, 10usize, 12u32, 2179432u32)?;
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2179436u32;
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
pub fn block_0x0021416c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(17usize, 13usize, 1u32, 2179440u32);
    emu.sli_no_count(14usize, 14usize, 31u32, 2179444u32);
    emu.sli_no_count(5usize, 13usize, 31u32, 2179448u32);
    emu.orr_no_count(14usize, 14usize, 17usize, 2179452u32);
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2179456u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(6usize);
    let return_addr = 2179460u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2179480u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00214198));
}
#[inline(always)]
pub fn block_0x00214184(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(5usize, 5usize, 1u32, 2179464u32);
    emu.sli_no_count(6usize, 14usize, 31u32, 2179468u32);
    emu.sri_no_count(14usize, 14usize, 1u32, 2179472u32);
    emu.orr_no_count(5usize, 5usize, 6usize, 2179476u32);
    emu.sri_no_count(17usize, 17usize, 1u32, 2179480u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2179480u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00214198));
}
#[inline(always)]
pub fn block_0x00214198(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(6usize, 11usize, 5usize, 2179484u32);
    emu.sbr_no_count(7usize, 16usize, 14usize, 2179488u32);
    emu.sbr_no_count(6usize, 7usize, 6usize, 2179492u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(6usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2179460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00214184));
    } else {
        emu.pc = 2179496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002141a8));
    }
}
#[inline(always)]
pub fn block_0x002141a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 11usize, 5usize, 2179500u32);
    emu.orr_no_count(12usize, 17usize, 12usize, 2179504u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2179516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002141bc));
    } else {
        emu.pc = 2179508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002141b4));
    }
}
#[inline(always)]
pub fn block_0x002141b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 6usize, 0u32, 2179512u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2179516u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2179460u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00214184));
}
#[inline]
pub fn block_0x002141bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.divu_no_count(14usize, 11usize, 13usize, 2179520u32);
    emu.mul_no_count(13usize, 14usize, 13usize, 2179524u32);
    emu.sbr_no_count(11usize, 11usize, 13usize, 2179528u32);
    emu.orr_no_count(17usize, 14usize, 12usize, 2179532u32);
    emu.sw_no_count(17usize, 10usize, 0u32, 2179536u32)?;
    emu.sw_no_count(15usize, 10usize, 4u32, 2179540u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2179544u32)?;
    emu.sw_no_count(0usize, 10usize, 12u32, 2179548u32)?;
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2179552u32;
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
