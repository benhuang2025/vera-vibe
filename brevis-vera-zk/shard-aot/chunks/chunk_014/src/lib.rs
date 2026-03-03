pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2153040u32;
pub const PC_MAX: u32 = 2155060u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 112usize] = [
        block_0x0020da50,
        block_0x0020da88,
        block_0x0020da94,
        block_0x0020dab0,
        block_0x0020dae0,
        block_0x0020dae8,
        block_0x0020daf0,
        block_0x0020daf8,
        block_0x0020db0c,
        block_0x0020db34,
        block_0x0020db3c,
        block_0x0020db44,
        block_0x0020db5c,
        block_0x0020db64,
        block_0x0020db70,
        block_0x0020db90,
        block_0x0020db94,
        block_0x0020dbc4,
        block_0x0020dbcc,
        block_0x0020dbe4,
        block_0x0020dbe8,
        block_0x0020dbf8,
        block_0x0020dbfc,
        block_0x0020dc04,
        block_0x0020dc14,
        block_0x0020dc18,
        block_0x0020dc28,
        block_0x0020dc30,
        block_0x0020dc50,
        block_0x0020dc58,
        block_0x0020dc74,
        block_0x0020dc8c,
        block_0x0020dcac,
        block_0x0020dcb4,
        block_0x0020dcc4,
        block_0x0020dccc,
        block_0x0020dcd4,
        block_0x0020dce0,
        block_0x0020dcfc,
        block_0x0020dd0c,
        block_0x0020dd1c,
        block_0x0020dd58,
        block_0x0020dd70,
        block_0x0020dd8c,
        block_0x0020dda8,
        block_0x0020dde4,
        block_0x0020de04,
        block_0x0020de34,
        block_0x0020de3c,
        block_0x0020de44,
        block_0x0020de4c,
        block_0x0020de60,
        block_0x0020de88,
        block_0x0020de90,
        block_0x0020de98,
        block_0x0020deb0,
        block_0x0020deb8,
        block_0x0020dec4,
        block_0x0020dee4,
        block_0x0020dee8,
        block_0x0020df18,
        block_0x0020df20,
        block_0x0020df38,
        block_0x0020df3c,
        block_0x0020df4c,
        block_0x0020df50,
        block_0x0020df58,
        block_0x0020df68,
        block_0x0020df6c,
        block_0x0020df7c,
        block_0x0020df84,
        block_0x0020dfa8,
        block_0x0020dfb0,
        block_0x0020dfb8,
        block_0x0020dfbc,
        block_0x0020dfe4,
        block_0x0020dfec,
        block_0x0020e00c,
        block_0x0020e010,
        block_0x0020e02c,
        block_0x0020e034,
        block_0x0020e050,
        block_0x0020e058,
        block_0x0020e068,
        block_0x0020e070,
        block_0x0020e078,
        block_0x0020e084,
        block_0x0020e0a0,
        block_0x0020e0b8,
        block_0x0020e0c8,
        block_0x0020e0d8,
        block_0x0020e118,
        block_0x0020e134,
        block_0x0020e150,
        block_0x0020e15c,
        block_0x0020e168,
        block_0x0020e174,
        block_0x0020e178,
        block_0x0020e17c,
        block_0x0020e188,
        block_0x0020e190,
        block_0x0020e19c,
        block_0x0020e1a4,
        block_0x0020e1c0,
        block_0x0020e1f4,
        block_0x0020e1fc,
        block_0x0020e200,
        block_0x0020e20c,
        block_0x0020e214,
        block_0x0020e220,
        block_0x0020e228,
        block_0x0020e234,
    ];
    const IDX: [u16; 506usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 2u16, 0u16, 0u16, 3u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 4u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 5u16, 0u16, 6u16,
        0u16, 7u16, 0u16, 8u16, 0u16, 0u16, 0u16, 0u16, 9u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 10u16, 0u16, 11u16, 0u16, 12u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 13u16, 0u16, 14u16, 0u16, 0u16, 15u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 16u16, 17u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 18u16, 0u16, 19u16, 0u16, 0u16, 0u16, 0u16, 0u16, 20u16, 21u16, 0u16,
        0u16, 0u16, 22u16, 23u16, 0u16, 24u16, 0u16, 0u16, 0u16, 25u16, 26u16, 0u16,
        0u16, 0u16, 27u16, 0u16, 28u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 29u16,
        0u16, 30u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 31u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 32u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 33u16, 0u16, 34u16, 0u16,
        0u16, 0u16, 35u16, 0u16, 36u16, 0u16, 37u16, 0u16, 0u16, 38u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 39u16, 0u16, 0u16, 0u16, 40u16, 0u16, 0u16, 0u16, 41u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        42u16, 0u16, 0u16, 0u16, 0u16, 0u16, 43u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        44u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 45u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 46u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 47u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 48u16, 0u16, 49u16, 0u16, 50u16, 0u16, 51u16, 0u16, 0u16, 0u16,
        0u16, 52u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 53u16, 0u16,
        54u16, 0u16, 55u16, 0u16, 0u16, 0u16, 0u16, 0u16, 56u16, 0u16, 57u16, 0u16, 0u16,
        58u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 59u16, 60u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 61u16, 0u16, 62u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 63u16, 64u16, 0u16, 0u16, 0u16, 65u16, 66u16, 0u16, 67u16,
        0u16, 0u16, 0u16, 68u16, 69u16, 0u16, 0u16, 0u16, 70u16, 0u16, 71u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 72u16, 0u16, 73u16, 0u16, 74u16, 75u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 76u16, 0u16, 77u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 78u16, 79u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        80u16, 0u16, 81u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 82u16, 0u16, 83u16, 0u16,
        0u16, 0u16, 84u16, 0u16, 85u16, 0u16, 86u16, 0u16, 0u16, 87u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 88u16, 0u16, 0u16, 0u16, 0u16, 0u16, 89u16, 0u16, 0u16, 0u16,
        90u16, 0u16, 0u16, 0u16, 91u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 92u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 93u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 94u16, 0u16, 0u16, 95u16, 0u16,
        0u16, 96u16, 0u16, 0u16, 97u16, 98u16, 99u16, 0u16, 0u16, 100u16, 0u16, 101u16,
        0u16, 0u16, 102u16, 0u16, 103u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 104u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 105u16,
        0u16, 106u16, 107u16, 0u16, 0u16, 108u16, 0u16, 109u16, 0u16, 0u16, 110u16, 0u16,
        111u16, 0u16, 0u16, 112u16,
    ];
    if pc < 2153040u32 || pc > 2155060u32 {
        return None;
    }
    let word_offset = ((pc - 2153040u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x0020da50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967200u32, 2153044u32);
    emu.sw_no_count(1usize, 2usize, 92u32, 2153048u32)?;
    emu.sw_no_count(8usize, 2usize, 88u32, 2153052u32)?;
    emu.sw_no_count(9usize, 2usize, 84u32, 2153056u32)?;
    emu.sw_no_count(18usize, 2usize, 80u32, 2153060u32)?;
    emu.sw_no_count(19usize, 2usize, 76u32, 2153064u32)?;
    emu.sw_no_count(20usize, 2usize, 72u32, 2153068u32)?;
    emu.sw_no_count(21usize, 2usize, 68u32, 2153072u32)?;
    emu.sw_no_count(22usize, 2usize, 64u32, 2153076u32)?;
    emu.sw_no_count(23usize, 2usize, 60u32, 2153080u32)?;
    emu.adi_no_count(18usize, 14usize, 0u32, 2153084u32);
    emu.lw_no_count(19usize, 2usize, 96u32, 2153088u32)?;
    emu.adi_no_count(14usize, 0usize, 3u32, 2153092u32);
    emu.add_memory_rw_events(13usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a >= b {
        emu.pc = 2153840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dd70));
    } else {
        emu.pc = 2153096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020da88));
    }
}
#[inline(always)]
pub fn block_0x0020da88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2153100u32);
    emu.adi_no_count(10usize, 0usize, 16u32, 2153104u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2153868u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dd8c));
    } else {
        emu.pc = 2153108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020da94));
    }
}
#[inline(always)]
pub fn block_0x0020da94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 17usize, 0u32, 2153112u32);
    emu.sli_no_count(6usize, 12usize, 1u32, 2153116u32);
    let a = 0u32.wrapping_add(1048576u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2153120u32;
    emu.update_insn_clock();
    emu.sri_no_count(17usize, 6usize, 21u32, 2153124u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2153128u32);
    emu.anr_no_count(5usize, 12usize, 10usize, 2153132u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2153228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020db0c));
    } else {
        emu.pc = 2153136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dab0));
    }
}
#[inline]
pub fn block_0x0020dab0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(14usize, 0usize, 11usize, 2153140u32);
    emu.adi_no_count(7usize, 11usize, 4294967295u32, 2153144u32);
    emu.adr_no_count(28usize, 5usize, 14usize, 2153148u32);
    emu.adi_no_count(14usize, 7usize, 1u32, 2153152u32);
    emu.sltiu_no_count(7usize, 14usize, 1u32, 2153156u32);
    emu.adr_no_count(10usize, 7usize, 10usize, 2153160u32);
    emu.adr_no_count(10usize, 28usize, 10usize, 2153164u32);
    emu.sri_no_count(7usize, 6usize, 1u32, 2153168u32);
    let a = 0u32.wrapping_add(2146435072u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2153172u32;
    emu.update_insn_clock();
    emu.xrr_no_count(7usize, 7usize, 6usize, 2153176u32);
    emu.orr_no_count(7usize, 11usize, 7usize, 2153180u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2153268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020db34));
    } else {
        emu.pc = 2153184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dae0));
    }
}
#[inline(always)]
pub fn block_0x0020dae0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(28usize, 12usize, 6usize, 2153188u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2153276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020db3c));
    } else {
        emu.pc = 2153192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dae8));
    }
}
#[inline(always)]
pub fn block_0x0020dae8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(7usize, 14usize, 1u32, 2153196u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a != b {
        emu.pc = 2153284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020db44));
    } else {
        emu.pc = 2153200u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020daf0));
    }
}
#[inline(always)]
pub fn block_0x0020daf0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.orr_no_count(11usize, 11usize, 5usize, 2153204u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2153360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020db90));
    } else {
        emu.pc = 2153208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020daf8));
    }
}
#[inline(always)]
pub fn block_0x0020daf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(6usize, 0usize, 0u32, 2153212u32);
    emu.adi_no_count(17usize, 17usize, 4294966221u32, 2153216u32);
    emu.xri_no_count(11usize, 7usize, 1u32, 2153220u32);
    emu.adi_no_count(5usize, 0usize, 1u32, 2153224u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2153228u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2153364u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020db94));
}
#[inline]
pub fn block_0x0020db0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 31u32, 2153232u32);
    emu.orr_no_count(10usize, 6usize, 10usize, 2153236u32);
    emu.sli_no_count(14usize, 11usize, 1u32, 2153240u32);
    emu.sli_no_count(10usize, 10usize, 11u32, 2153244u32);
    emu.sri_no_count(10usize, 10usize, 11u32, 2153248u32);
    emu.sri_no_count(7usize, 6usize, 1u32, 2153252u32);
    let a = 0u32.wrapping_add(2146435072u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2153256u32;
    emu.update_insn_clock();
    emu.xrr_no_count(7usize, 7usize, 6usize, 2153260u32);
    emu.orr_no_count(7usize, 11usize, 7usize, 2153264u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a != b {
        emu.pc = 2153184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dae0));
    } else {
        emu.pc = 2153268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020db34));
    }
}
#[inline(always)]
pub fn block_0x0020db34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 3u32, 2153272u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2153276u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2153364u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020db94));
}
#[inline(always)]
pub fn block_0x0020db3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 2u32, 2153280u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2153284u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2153364u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020db94));
}
#[inline(always)]
pub fn block_0x0020db44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(1048576u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2153288u32;
    emu.update_insn_clock();
    emu.xrr_no_count(11usize, 10usize, 11usize, 2153292u32);
    emu.orr_no_count(5usize, 14usize, 11usize, 2153296u32);
    emu.sltiu_no_count(11usize, 5usize, 1u32, 2153300u32);
    emu.sli_no_count(6usize, 14usize, 1u32, 2153304u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a != b {
        emu.pc = 2153316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020db64));
    } else {
        emu.pc = 2153308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020db5c));
    }
}
#[inline(always)]
pub fn block_0x0020db5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(4194304u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2153312u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    let return_addr = 2153316u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2153328u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020db70));
}
#[inline(always)]
pub fn block_0x0020db64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(14usize, 14usize, 31u32, 2153320u32);
    emu.sli_no_count(10usize, 10usize, 1u32, 2153324u32);
    emu.orr_no_count(10usize, 10usize, 14usize, 2153328u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2153328u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020db70));
}
#[inline(always)]
pub fn block_0x0020db70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 11usize, 4294967295u32, 2153332u32);
    emu.adi_no_count(5usize, 11usize, 1u32, 2153336u32);
    emu.sbr_no_count(11usize, 17usize, 11usize, 2153340u32);
    emu.anr_no_count(14usize, 14usize, 6usize, 2153344u32);
    emu.sltiu_no_count(6usize, 5usize, 1u32, 2153348u32);
    emu.adi_no_count(17usize, 11usize, 4294966220u32, 2153352u32);
    emu.xri_no_count(11usize, 7usize, 1u32, 2153356u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2153360u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2153364u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020db94));
}
#[inline(always)]
pub fn block_0x0020db90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 4u32, 2153364u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2153364u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020db94));
}
#[inline]
pub fn block_0x0020db94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(7usize, 11usize, 255u32, 2153368u32);
    emu.sri_no_count(12usize, 12usize, 31u32, 2153372u32);
    emu.adi_no_count(22usize, 0usize, 1u32, 2153376u32);
    emu.sw_no_count(14usize, 2usize, 0u32, 2153380u32)?;
    emu.sw_no_count(10usize, 2usize, 4u32, 2153384u32)?;
    emu.sw_no_count(22usize, 2usize, 8u32, 2153388u32)?;
    emu.sw_no_count(0usize, 2usize, 12u32, 2153392u32)?;
    emu.sw_no_count(5usize, 2usize, 16u32, 2153396u32)?;
    emu.sw_no_count(6usize, 2usize, 20u32, 2153400u32)?;
    emu.sh_no_count(17usize, 2usize, 24u32, 2153404u32)?;
    emu.sb_no_count(11usize, 2usize, 26u32, 2153408u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a >= b {
        emu.pc = 2153444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dbe4));
    } else {
        emu.pc = 2153412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dbc4));
    }
}
#[inline(always)]
pub fn block_0x0020dbc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2153416u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a != b {
        emu.pc = 2153468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dbfc));
    } else {
        emu.pc = 2153420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dbcc));
    }
}
#[inline(always)]
pub fn block_0x0020dbcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(22usize, 0usize, 0u32, 2153424u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2153428u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1322u32, 2153432u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2153436u32);
    emu.adi_no_count(23usize, 0usize, 1u32, 2153440u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2153444u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2153740u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020dd0c));
}
#[inline(always)]
pub fn block_0x0020dbe4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2153496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dc18));
    } else {
        emu.pc = 2153448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dbe8));
    }
}
#[inline(always)]
pub fn block_0x0020dbe8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(23usize, a);
    emu.pc = 2153452u32;
    emu.update_insn_clock();
    emu.adi_no_count(23usize, 23usize, 1321u32, 2153456u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2153460u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2153512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dc28));
    } else {
        emu.pc = 2153464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dbf8));
    }
}
#[inline(always)]
pub fn block_0x0020dbf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2153468u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2153520u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020dc30));
}
#[inline(always)]
pub fn block_0x0020dbfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(22usize, 0usize, 1u32, 2153472u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2153652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dcb4));
    } else {
        emu.pc = 2153476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dc04));
    }
}
#[inline(always)]
pub fn block_0x0020dc04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(23usize, a);
    emu.pc = 2153480u32;
    emu.update_insn_clock();
    emu.adi_no_count(23usize, 23usize, 1321u32, 2153484u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2153488u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2153668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dcc4));
    } else {
        emu.pc = 2153492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dc14));
    }
}
#[inline(always)]
pub fn block_0x0020dc14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2153496u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2153676u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020dccc));
}
#[inline(always)]
pub fn block_0x0020dc18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(23usize, a);
    emu.pc = 2153500u32;
    emu.update_insn_clock();
    emu.adi_no_count(23usize, 23usize, 1320u32, 2153504u32);
    emu.adi_no_count(10usize, 23usize, 0u32, 2153508u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2153520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dc30));
    } else {
        emu.pc = 2153512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dc28));
    }
}
#[inline(always)]
pub fn block_0x0020dc28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(22usize, 12usize, 0u32, 2153516u32);
    emu.adi_no_count(23usize, 10usize, 0u32, 2153520u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2153520u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020dc30));
}
#[inline(always)]
pub fn block_0x0020dc30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 48u32, 2153524u32);
    emu.adi_no_count(11usize, 2usize, 0u32, 2153528u32);
    emu.adi_no_count(21usize, 15usize, 0u32, 2153532u32);
    emu.adi_no_count(12usize, 15usize, 0u32, 2153536u32);
    emu.adi_no_count(20usize, 16usize, 0u32, 2153540u32);
    emu.adi_no_count(13usize, 16usize, 0u32, 2153544u32);
    emu.apc_no_count(1usize, 2153544u32, 0u32, 2153548u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2153552u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1904u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020dc50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 48u32, 2153556u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2153588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dc74));
    } else {
        emu.pc = 2153560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dc58));
    }
}
#[inline(always)]
pub fn block_0x0020dc58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 48u32, 2153564u32)?;
    emu.lw_no_count(11usize, 2usize, 52u32, 2153568u32)?;
    emu.lw_no_count(12usize, 2usize, 56u32, 2153572u32)?;
    emu.sw_no_count(10usize, 2usize, 36u32, 2153576u32)?;
    emu.sw_no_count(11usize, 2usize, 40u32, 2153580u32)?;
    emu.sw_no_count(12usize, 2usize, 44u32, 2153584u32)?;
    emu.add_memory_rw_events(7usize);
    let return_addr = 2153588u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2153612u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020dc8c));
}
#[inline(always)]
pub fn block_0x0020dc74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 36u32, 2153592u32);
    emu.adi_no_count(11usize, 2usize, 0u32, 2153596u32);
    emu.adi_no_count(12usize, 21usize, 0u32, 2153600u32);
    emu.adi_no_count(13usize, 20usize, 0u32, 2153604u32);
    emu.apc_no_count(1usize, 2153604u32, 16384u32, 2153608u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2153612u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1112u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020dc8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 36u32, 2153616u32)?;
    emu.lw_no_count(11usize, 2usize, 40u32, 2153620u32)?;
    emu.lh_no_count(12usize, 2usize, 44u32, 2153624u32)?;
    emu.adi_no_count(13usize, 18usize, 0u32, 2153628u32);
    emu.adi_no_count(14usize, 8usize, 0u32, 2153632u32);
    emu.adi_no_count(15usize, 19usize, 0u32, 2153636u32);
    emu.apc_no_count(1usize, 2153636u32, 0u32, 2153640u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2153644u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966352u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020dcac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 10usize, 0u32, 2153648u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2153652u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2153756u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020dd1c));
}
#[inline(always)]
pub fn block_0x0020dcb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(23usize, a);
    emu.pc = 2153656u32;
    emu.update_insn_clock();
    emu.adi_no_count(23usize, 23usize, 1320u32, 2153660u32);
    emu.adi_no_count(10usize, 23usize, 0u32, 2153664u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2153676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dccc));
    } else {
        emu.pc = 2153668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dcc4));
    }
}
#[inline(always)]
pub fn block_0x0020dcc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(22usize, 12usize, 0u32, 2153672u32);
    emu.adi_no_count(23usize, 10usize, 0u32, 2153676u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2153676u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020dccc));
}
#[inline(always)]
pub fn block_0x0020dccc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2153680u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a != b {
        emu.pc = 2153724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dcfc));
    } else {
        emu.pc = 2153684u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dcd4));
    }
}
#[inline(always)]
pub fn block_0x0020dcd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 2u32, 2153688u32);
    emu.sh_no_count(11usize, 8usize, 0u32, 2153692u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2153816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dd58));
    } else {
        emu.pc = 2153696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dce0));
    }
}
#[inline(always)]
pub fn block_0x0020dce0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2153700u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1237u32, 2153704u32);
    emu.sw_no_count(10usize, 8usize, 4u32, 2153708u32)?;
    emu.sw_no_count(11usize, 8usize, 8u32, 2153712u32)?;
    emu.sh_no_count(0usize, 8usize, 12u32, 2153716u32)?;
    emu.sw_no_count(18usize, 8usize, 16u32, 2153720u32)?;
    emu.add_memory_rw_events(7usize);
    let return_addr = 2153724u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2153756u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020dd1c));
}
#[inline(always)]
pub fn block_0x0020dcfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2153728u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2153732u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1325u32, 2153736u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2153740u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2153740u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020dd0c));
}
#[inline(always)]
pub fn block_0x0020dd0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sh_no_count(10usize, 8usize, 0u32, 2153744u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2153748u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2153752u32)?;
    emu.adi_no_count(11usize, 0usize, 1u32, 2153756u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2153756u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020dd1c));
}
#[inline]
pub fn block_0x0020dd1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(23usize, 9usize, 0u32, 2153760u32)?;
    emu.sw_no_count(22usize, 9usize, 4u32, 2153764u32)?;
    emu.sw_no_count(8usize, 9usize, 8u32, 2153768u32)?;
    emu.sw_no_count(11usize, 9usize, 12u32, 2153772u32)?;
    emu.lw_no_count(1usize, 2usize, 92u32, 2153776u32)?;
    emu.lw_no_count(8usize, 2usize, 88u32, 2153780u32)?;
    emu.lw_no_count(9usize, 2usize, 84u32, 2153784u32)?;
    emu.lw_no_count(18usize, 2usize, 80u32, 2153788u32)?;
    emu.lw_no_count(19usize, 2usize, 76u32, 2153792u32)?;
    emu.lw_no_count(20usize, 2usize, 72u32, 2153796u32)?;
    emu.lw_no_count(21usize, 2usize, 68u32, 2153800u32)?;
    emu.lw_no_count(22usize, 2usize, 64u32, 2153804u32)?;
    emu.lw_no_count(23usize, 2usize, 60u32, 2153808u32)?;
    emu.adi_no_count(2usize, 2usize, 96u32, 2153812u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2153816u32;
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
pub fn block_0x0020dd58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2153820u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1328u32, 2153824u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2153828u32);
    emu.sw_no_count(10usize, 8usize, 4u32, 2153832u32)?;
    emu.sw_no_count(11usize, 8usize, 8u32, 2153836u32)?;
    emu.add_memory_rw_events(6usize);
    let return_addr = 2153840u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2153756u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020dd1c));
}
#[inline(always)]
pub fn block_0x0020dd70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2153844u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1184u32, 2153848u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2153852u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1240u32, 2153856u32);
    emu.adi_no_count(11usize, 0usize, 34u32, 2153860u32);
    emu.apc_no_count(1usize, 2153860u32, 4294959104u32, 2153864u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2153868u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1776u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020dd8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2153872u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1256u32, 2153876u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2153880u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1304u32, 2153884u32);
    emu.adi_no_count(11usize, 0usize, 45u32, 2153888u32);
    emu.apc_no_count(1usize, 2153888u32, 4294959104u32, 2153892u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2153896u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1748u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020dda8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967200u32, 2153900u32);
    emu.sw_no_count(1usize, 2usize, 92u32, 2153904u32)?;
    emu.sw_no_count(8usize, 2usize, 88u32, 2153908u32)?;
    emu.sw_no_count(9usize, 2usize, 84u32, 2153912u32)?;
    emu.sw_no_count(18usize, 2usize, 80u32, 2153916u32)?;
    emu.sw_no_count(19usize, 2usize, 76u32, 2153920u32)?;
    emu.sw_no_count(20usize, 2usize, 72u32, 2153924u32)?;
    emu.sw_no_count(21usize, 2usize, 68u32, 2153928u32)?;
    emu.sw_no_count(22usize, 2usize, 64u32, 2153932u32)?;
    emu.sw_no_count(23usize, 2usize, 60u32, 2153936u32)?;
    emu.sw_no_count(24usize, 2usize, 56u32, 2153940u32)?;
    emu.adi_no_count(18usize, 14usize, 0u32, 2153944u32);
    emu.lw_no_count(19usize, 2usize, 96u32, 2153948u32)?;
    emu.adi_no_count(14usize, 0usize, 3u32, 2153952u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a >= b {
        emu.pc = 2154776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e118));
    } else {
        emu.pc = 2153956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dde4));
    }
}
#[inline(always)]
pub fn block_0x0020dde4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 17usize, 0u32, 2153960u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2153964u32);
    emu.sli_no_count(6usize, 12usize, 1u32, 2153968u32);
    let a = 0u32.wrapping_add(1048576u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2153972u32;
    emu.update_insn_clock();
    emu.sri_no_count(17usize, 6usize, 21u32, 2153976u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2153980u32);
    emu.anr_no_count(5usize, 12usize, 10usize, 2153984u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2154080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020de60));
    } else {
        emu.pc = 2153988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020de04));
    }
}
#[inline]
pub fn block_0x0020de04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(14usize, 0usize, 11usize, 2153992u32);
    emu.adi_no_count(7usize, 11usize, 4294967295u32, 2153996u32);
    emu.adr_no_count(28usize, 5usize, 14usize, 2154000u32);
    emu.adi_no_count(14usize, 7usize, 1u32, 2154004u32);
    emu.sltiu_no_count(7usize, 14usize, 1u32, 2154008u32);
    emu.adr_no_count(10usize, 7usize, 10usize, 2154012u32);
    emu.adr_no_count(10usize, 28usize, 10usize, 2154016u32);
    emu.sri_no_count(7usize, 6usize, 1u32, 2154020u32);
    let a = 0u32.wrapping_add(2146435072u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2154024u32;
    emu.update_insn_clock();
    emu.xrr_no_count(7usize, 7usize, 6usize, 2154028u32);
    emu.orr_no_count(7usize, 11usize, 7usize, 2154032u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2154120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020de88));
    } else {
        emu.pc = 2154036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020de34));
    }
}
#[inline(always)]
pub fn block_0x0020de34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(28usize, 12usize, 6usize, 2154040u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2154128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020de90));
    } else {
        emu.pc = 2154044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020de3c));
    }
}
#[inline(always)]
pub fn block_0x0020de3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(7usize, 14usize, 1u32, 2154048u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a != b {
        emu.pc = 2154136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020de98));
    } else {
        emu.pc = 2154052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020de44));
    }
}
#[inline(always)]
pub fn block_0x0020de44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.orr_no_count(11usize, 11usize, 5usize, 2154056u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2154212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dee4));
    } else {
        emu.pc = 2154060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020de4c));
    }
}
#[inline(always)]
pub fn block_0x0020de4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(6usize, 0usize, 0u32, 2154064u32);
    emu.adi_no_count(11usize, 17usize, 4294966221u32, 2154068u32);
    emu.xri_no_count(17usize, 7usize, 1u32, 2154072u32);
    emu.adi_no_count(5usize, 0usize, 1u32, 2154076u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2154080u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2154216u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020dee8));
}
#[inline]
pub fn block_0x0020de60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 31u32, 2154084u32);
    emu.orr_no_count(10usize, 6usize, 10usize, 2154088u32);
    emu.sli_no_count(14usize, 11usize, 1u32, 2154092u32);
    emu.sli_no_count(10usize, 10usize, 11u32, 2154096u32);
    emu.sri_no_count(10usize, 10usize, 11u32, 2154100u32);
    emu.sri_no_count(7usize, 6usize, 1u32, 2154104u32);
    let a = 0u32.wrapping_add(2146435072u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2154108u32;
    emu.update_insn_clock();
    emu.xrr_no_count(7usize, 7usize, 6usize, 2154112u32);
    emu.orr_no_count(7usize, 11usize, 7usize, 2154116u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a != b {
        emu.pc = 2154036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020de34));
    } else {
        emu.pc = 2154120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020de88));
    }
}
#[inline(always)]
pub fn block_0x0020de88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 0usize, 3u32, 2154124u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2154128u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2154216u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020dee8));
}
#[inline(always)]
pub fn block_0x0020de90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 0usize, 2u32, 2154132u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2154136u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2154216u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020dee8));
}
#[inline(always)]
pub fn block_0x0020de98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(1048576u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2154140u32;
    emu.update_insn_clock();
    emu.xrr_no_count(11usize, 10usize, 11usize, 2154144u32);
    emu.orr_no_count(5usize, 14usize, 11usize, 2154148u32);
    emu.sltiu_no_count(11usize, 5usize, 1u32, 2154152u32);
    emu.sli_no_count(6usize, 14usize, 1u32, 2154156u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a != b {
        emu.pc = 2154168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020deb8));
    } else {
        emu.pc = 2154160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020deb0));
    }
}
#[inline(always)]
pub fn block_0x0020deb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(4194304u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2154164u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    let return_addr = 2154168u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2154180u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020dec4));
}
#[inline(always)]
pub fn block_0x0020deb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(14usize, 14usize, 31u32, 2154172u32);
    emu.sli_no_count(10usize, 10usize, 1u32, 2154176u32);
    emu.orr_no_count(10usize, 10usize, 14usize, 2154180u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2154180u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020dec4));
}
#[inline(always)]
pub fn block_0x0020dec4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 11usize, 4294967295u32, 2154184u32);
    emu.adi_no_count(5usize, 11usize, 1u32, 2154188u32);
    emu.sbr_no_count(11usize, 17usize, 11usize, 2154192u32);
    emu.anr_no_count(14usize, 14usize, 6usize, 2154196u32);
    emu.sltiu_no_count(6usize, 5usize, 1u32, 2154200u32);
    emu.adi_no_count(11usize, 11usize, 4294966220u32, 2154204u32);
    emu.xri_no_count(17usize, 7usize, 1u32, 2154208u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2154212u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2154216u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020dee8));
}
#[inline(always)]
pub fn block_0x0020dee4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 0usize, 4u32, 2154216u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2154216u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020dee8));
}
#[inline]
pub fn block_0x0020dee8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(7usize, 17usize, 255u32, 2154220u32);
    emu.sri_no_count(12usize, 12usize, 31u32, 2154224u32);
    emu.adi_no_count(23usize, 0usize, 1u32, 2154228u32);
    emu.sw_no_count(14usize, 2usize, 0u32, 2154232u32)?;
    emu.sw_no_count(10usize, 2usize, 4u32, 2154236u32)?;
    emu.sw_no_count(23usize, 2usize, 8u32, 2154240u32)?;
    emu.sw_no_count(0usize, 2usize, 12u32, 2154244u32)?;
    emu.sw_no_count(5usize, 2usize, 16u32, 2154248u32)?;
    emu.sw_no_count(6usize, 2usize, 20u32, 2154252u32)?;
    emu.sh_no_count(11usize, 2usize, 24u32, 2154256u32)?;
    emu.sb_no_count(17usize, 2usize, 26u32, 2154260u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a >= b {
        emu.pc = 2154296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020df38));
    } else {
        emu.pc = 2154264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020df18));
    }
}
#[inline(always)]
pub fn block_0x0020df18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2154268u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a != b {
        emu.pc = 2154320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020df50));
    } else {
        emu.pc = 2154272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020df20));
    }
}
#[inline(always)]
pub fn block_0x0020df20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(23usize, 0usize, 0u32, 2154276u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2154280u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1322u32, 2154284u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2154288u32);
    emu.adi_no_count(24usize, 0usize, 1u32, 2154292u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2154296u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2154696u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e0c8));
}
#[inline(always)]
pub fn block_0x0020df38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2154348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020df6c));
    } else {
        emu.pc = 2154300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020df3c));
    }
}
#[inline(always)]
pub fn block_0x0020df3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(24usize, a);
    emu.pc = 2154304u32;
    emu.update_insn_clock();
    emu.adi_no_count(24usize, 24usize, 1321u32, 2154308u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2154312u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2154364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020df7c));
    } else {
        emu.pc = 2154316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020df4c));
    }
}
#[inline(always)]
pub fn block_0x0020df4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2154320u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2154372u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020df84));
}
#[inline(always)]
pub fn block_0x0020df50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(23usize, 0usize, 1u32, 2154324u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2154584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e058));
    } else {
        emu.pc = 2154328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020df58));
    }
}
#[inline(always)]
pub fn block_0x0020df58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(24usize, a);
    emu.pc = 2154332u32;
    emu.update_insn_clock();
    emu.adi_no_count(24usize, 24usize, 1321u32, 2154336u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2154340u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2154600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e068));
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
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2154348u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2154608u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e070));
}
#[inline(always)]
pub fn block_0x0020df6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(24usize, a);
    emu.pc = 2154352u32;
    emu.update_insn_clock();
    emu.adi_no_count(24usize, 24usize, 1320u32, 2154356u32);
    emu.adi_no_count(10usize, 24usize, 0u32, 2154360u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2154372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020df84));
    } else {
        emu.pc = 2154364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020df7c));
    }
}
#[inline(always)]
pub fn block_0x0020df7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(23usize, 12usize, 0u32, 2154368u32);
    emu.adi_no_count(24usize, 10usize, 0u32, 2154372u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2154372u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020df84));
}
#[inline]
pub fn block_0x0020df84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 11usize, 16u32, 2154376u32);
    emu.sai_no_count(10usize, 11usize, 1055u32, 2154380u32);
    emu.ani_no_count(10usize, 10usize, 4294967279u32, 2154384u32);
    emu.adi_no_count(10usize, 10usize, 5u32, 2154388u32);
    emu.sai_no_count(11usize, 11usize, 1040u32, 2154392u32);
    emu.mul_no_count(10usize, 10usize, 11usize, 2154396u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2154400u32);
    emu.adi_no_count(21usize, 10usize, 21u32, 2154404u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a < b {
        emu.pc = 2154804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e134));
    } else {
        emu.pc = 2154408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dfa8));
    }
}
#[inline(always)]
pub fn block_0x0020dfa8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 18usize, 15u32, 2154412u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2154424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dfb8));
    } else {
        emu.pc = 2154416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dfb0));
    }
}
#[inline(always)]
pub fn block_0x0020dfb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(4294934528u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2154420u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    let return_addr = 2154424u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2154428u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020dfbc));
}
#[inline(always)]
pub fn block_0x0020dfb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 0usize, 18usize, 2154428u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2154428u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020dfbc));
}
#[inline]
pub fn block_0x0020dfbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 10usize, 16u32, 2154432u32);
    emu.sai_no_count(20usize, 10usize, 1040u32, 2154436u32);
    emu.adi_no_count(10usize, 2usize, 44u32, 2154440u32);
    emu.adi_no_count(11usize, 2usize, 0u32, 2154444u32);
    emu.adi_no_count(22usize, 15usize, 0u32, 2154448u32);
    emu.adi_no_count(12usize, 15usize, 0u32, 2154452u32);
    emu.adi_no_count(13usize, 21usize, 0u32, 2154456u32);
    emu.adi_no_count(14usize, 20usize, 0u32, 2154460u32);
    emu.apc_no_count(1usize, 2154460u32, 4096u32, 2154464u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2154468u32;
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
pub fn block_0x0020dfe4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 44u32, 2154472u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2154512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e010));
    } else {
        emu.pc = 2154476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dfec));
    }
}
#[inline(always)]
pub fn block_0x0020dfec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 44u32, 2154480u32)?;
    emu.lw_no_count(11usize, 2usize, 48u32, 2154484u32)?;
    emu.lw_no_count(12usize, 2usize, 52u32, 2154488u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2154492u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2154496u32)?;
    emu.sw_no_count(12usize, 2usize, 40u32, 2154500u32)?;
    emu.lh_no_count(12usize, 2usize, 40u32, 2154504u32)?;
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(20usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2154548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e034));
    } else {
        emu.pc = 2154508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e00c));
    }
}
#[inline(always)]
pub fn block_0x0020e00c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2154512u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2154616u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e078));
}
#[inline(always)]
pub fn block_0x0020e010(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 32u32, 2154516u32);
    emu.adi_no_count(11usize, 2usize, 0u32, 2154520u32);
    emu.adi_no_count(12usize, 22usize, 0u32, 2154524u32);
    emu.adi_no_count(13usize, 21usize, 0u32, 2154528u32);
    emu.adi_no_count(14usize, 20usize, 0u32, 2154532u32);
    emu.apc_no_count(1usize, 2154532u32, 20480u32, 2154536u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2154540u32;
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
pub fn block_0x0020e02c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lh_no_count(12usize, 2usize, 40u32, 2154544u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(20usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2154616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e078));
    } else {
        emu.pc = 2154548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e034));
    }
}
#[inline(always)]
pub fn block_0x0020e034(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 32u32, 2154552u32)?;
    emu.lw_no_count(11usize, 2usize, 36u32, 2154556u32)?;
    emu.adi_no_count(13usize, 18usize, 0u32, 2154560u32);
    emu.adi_no_count(14usize, 8usize, 0u32, 2154564u32);
    emu.adi_no_count(15usize, 19usize, 0u32, 2154568u32);
    emu.apc_no_count(1usize, 2154568u32, 0u32, 2154572u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2154576u32;
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
pub fn block_0x0020e050(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 10usize, 0u32, 2154580u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2154584u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2154712u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e0d8));
}
#[inline(always)]
pub fn block_0x0020e058(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(24usize, a);
    emu.pc = 2154588u32;
    emu.update_insn_clock();
    emu.adi_no_count(24usize, 24usize, 1320u32, 2154592u32);
    emu.adi_no_count(10usize, 24usize, 0u32, 2154596u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2154608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e070));
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
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(23usize, 12usize, 0u32, 2154604u32);
    emu.adi_no_count(24usize, 10usize, 0u32, 2154608u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2154608u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e070));
}
#[inline(always)]
pub fn block_0x0020e070(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2154612u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a != b {
        emu.pc = 2154680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e0b8));
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
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 2u32, 2154620u32);
    emu.sh_no_count(11usize, 8usize, 0u32, 2154624u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2154656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e0a0));
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
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2154632u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1237u32, 2154636u32);
    emu.sw_no_count(10usize, 8usize, 4u32, 2154640u32)?;
    emu.sw_no_count(11usize, 8usize, 8u32, 2154644u32)?;
    emu.sh_no_count(0usize, 8usize, 12u32, 2154648u32)?;
    emu.sw_no_count(18usize, 8usize, 16u32, 2154652u32)?;
    emu.add_memory_rw_events(7usize);
    let return_addr = 2154656u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2154712u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e0d8));
}
#[inline(always)]
pub fn block_0x0020e0a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2154660u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1328u32, 2154664u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2154668u32);
    emu.sw_no_count(10usize, 8usize, 4u32, 2154672u32)?;
    emu.sw_no_count(11usize, 8usize, 8u32, 2154676u32)?;
    emu.add_memory_rw_events(6usize);
    let return_addr = 2154680u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2154712u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e0d8));
}
#[inline(always)]
pub fn block_0x0020e0b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2154684u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2154688u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1325u32, 2154692u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2154696u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2154696u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e0c8));
}
#[inline(always)]
pub fn block_0x0020e0c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sh_no_count(10usize, 8usize, 0u32, 2154700u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2154704u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2154708u32)?;
    emu.adi_no_count(11usize, 0usize, 1u32, 2154712u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2154712u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e0d8));
}
#[inline]
pub fn block_0x0020e0d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(24usize, 9usize, 0u32, 2154716u32)?;
    emu.sw_no_count(23usize, 9usize, 4u32, 2154720u32)?;
    emu.sw_no_count(8usize, 9usize, 8u32, 2154724u32)?;
    emu.sw_no_count(11usize, 9usize, 12u32, 2154728u32)?;
    emu.lw_no_count(1usize, 2usize, 92u32, 2154732u32)?;
    emu.lw_no_count(8usize, 2usize, 88u32, 2154736u32)?;
    emu.lw_no_count(9usize, 2usize, 84u32, 2154740u32)?;
    emu.lw_no_count(18usize, 2usize, 80u32, 2154744u32)?;
    emu.lw_no_count(19usize, 2usize, 76u32, 2154748u32)?;
    emu.lw_no_count(20usize, 2usize, 72u32, 2154752u32)?;
    emu.lw_no_count(21usize, 2usize, 68u32, 2154756u32)?;
    emu.lw_no_count(22usize, 2usize, 64u32, 2154760u32)?;
    emu.lw_no_count(23usize, 2usize, 60u32, 2154764u32)?;
    emu.lw_no_count(24usize, 2usize, 56u32, 2154768u32)?;
    emu.adi_no_count(2usize, 2usize, 96u32, 2154772u32);
    emu.add_memory_rw_events(16usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2154776u32;
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
pub fn block_0x0020e118(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2154780u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1184u32, 2154784u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2154788u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1332u32, 2154792u32);
    emu.adi_no_count(11usize, 0usize, 34u32, 2154796u32);
    emu.apc_no_count(1usize, 2154796u32, 4294959104u32, 2154800u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2154804u32;
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
pub fn block_0x0020e134(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2154808u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1348u32, 2154812u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2154816u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1388u32, 2154820u32);
    emu.adi_no_count(11usize, 0usize, 37u32, 2154824u32);
    emu.apc_no_count(1usize, 2154824u32, 4294959104u32, 2154828u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2154832u32;
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
pub fn block_0x0020e150(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 11usize, 3u32, 2154836u32);
    emu.ani_no_count(13usize, 13usize, 4294967292u32, 2154840u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2154856u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e168));
    } else {
        emu.pc = 2154844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e15c));
    }
}
#[inline(always)]
pub fn block_0x0020e15c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 0u32, 2154848u32);
    emu.adi_no_count(14usize, 12usize, 4294967288u32, 2154852u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2154856u32;
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
pub fn block_0x0020e168(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(14usize, 13usize, 11usize, 2154860u32);
    emu.adi_no_count(13usize, 12usize, 0u32, 2154864u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2154872u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e178));
    } else {
        emu.pc = 2154868u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e174));
    }
}
#[inline(always)]
pub fn block_0x0020e174(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 14usize, 0u32, 2154872u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2154872u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e178));
}
#[inline(always)]
pub fn block_0x0020e178(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2154908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e19c));
    } else {
        emu.pc = 2154876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e17c));
    }
}
#[inline(always)]
pub fn block_0x0020e17c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 0u32, 2154880u32);
    emu.sbr_no_count(15usize, 0usize, 13usize, 2154884u32);
    emu.adi_no_count(16usize, 11usize, 0u32, 2154888u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2154888u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e188));
}
#[inline(always)]
pub fn block_0x0020e188(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(17usize, 16usize, 0u32, 2154892u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2155060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e234));
    } else {
        emu.pc = 2154896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e190));
    }
}
#[inline(always)]
pub fn block_0x0020e190(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2154900u32);
    emu.adi_no_count(16usize, 16usize, 1u32, 2154904u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2154888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e188));
    } else {
        emu.pc = 2154908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e19c));
    }
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
    emu.adi_no_count(14usize, 12usize, 4294967288u32, 2154912u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2155004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e1fc));
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
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2154920u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 11usize, 4u32, 2154924u32);
    let a = 0u32.wrapping_add(2155905024u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2154928u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 256u32, 2154932u32);
    emu.adi_no_count(17usize, 16usize, 1u32, 2154936u32);
    emu.mul_no_count(17usize, 10usize, 17usize, 2154940u32);
    emu.adi_no_count(5usize, 5usize, 128u32, 2154944u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2154944u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e1c0));
}
#[inline]
pub fn block_0x0020e1c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(6usize, 11usize, 13usize, 2154948u32);
    emu.adr_no_count(7usize, 15usize, 13usize, 2154952u32);
    emu.lw_no_count(6usize, 6usize, 0u32, 2154956u32)?;
    emu.lw_no_count(7usize, 7usize, 0u32, 2154960u32)?;
    emu.xrr_no_count(6usize, 6usize, 17usize, 2154964u32);
    emu.xrr_no_count(7usize, 7usize, 17usize, 2154968u32);
    emu.sbr_no_count(28usize, 16usize, 6usize, 2154972u32);
    emu.orr_no_count(6usize, 28usize, 6usize, 2154976u32);
    emu.sbr_no_count(28usize, 16usize, 7usize, 2154980u32);
    emu.orr_no_count(7usize, 28usize, 7usize, 2154984u32);
    emu.anr_no_count(6usize, 6usize, 7usize, 2154988u32);
    emu.anr_no_count(6usize, 6usize, 5usize, 2154992u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a != b {
        emu.pc = 2155004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e1fc));
    } else {
        emu.pc = 2154996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e1f4));
    }
}
#[inline(always)]
pub fn block_0x0020e1f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 8u32, 2155000u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a >= b {
        emu.pc = 2154944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e1c0));
    } else {
        emu.pc = 2155004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e1fc));
    }
}
#[inline(always)]
pub fn block_0x0020e1fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2155040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e220));
    } else {
        emu.pc = 2155008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e200));
    }
}
#[inline(always)]
pub fn block_0x0020e200(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(14usize, 11usize, 13usize, 2155012u32);
    emu.sbr_no_count(11usize, 0usize, 13usize, 2155016u32);
    emu.sbr_no_count(12usize, 0usize, 12usize, 2155020u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2155020u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e20c));
}
#[inline(always)]
pub fn block_0x0020e20c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(13usize, 14usize, 0u32, 2155024u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2155048u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e228));
    } else {
        emu.pc = 2155028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e214));
    }
}
#[inline(always)]
pub fn block_0x0020e214(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2155032u32);
    emu.adi_no_count(14usize, 14usize, 1u32, 2155036u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2155020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e20c));
    } else {
        emu.pc = 2155040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e220));
    }
}
#[inline(always)]
pub fn block_0x0020e220(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2155044u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2155048u32;
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
pub fn block_0x0020e228(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 0usize, 11usize, 2155052u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2155056u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2155060u32;
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
pub fn block_0x0020e234(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 0usize, 14usize, 2155064u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2155068u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2155072u32;
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
