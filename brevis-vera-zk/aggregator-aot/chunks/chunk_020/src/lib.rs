pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2200996u32;
pub const PC_MAX: u32 = 2203924u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 127usize] = [
        block_0x002195a4,
        block_0x002195ac,
        block_0x002195c0,
        block_0x002195e8,
        block_0x002195f4,
        block_0x002195fc,
        block_0x00219604,
        block_0x00219618,
        block_0x0021964c,
        block_0x00219658,
        block_0x00219670,
        block_0x00219684,
        block_0x0021968c,
        block_0x00219694,
        block_0x0021969c,
        block_0x002196a8,
        block_0x002196cc,
        block_0x002196e8,
        block_0x00219748,
        block_0x0021976c,
        block_0x0021979c,
        block_0x002197ac,
        block_0x002197b0,
        block_0x002197e0,
        block_0x002197f0,
        block_0x002197fc,
        block_0x0021986c,
        block_0x002198a0,
        block_0x002198ac,
        block_0x002198b8,
        block_0x0021990c,
        block_0x00219918,
        block_0x00219930,
        block_0x00219950,
        block_0x00219960,
        block_0x00219964,
        block_0x00219988,
        block_0x00219998,
        block_0x002199a4,
        block_0x002199b4,
        block_0x002199cc,
        block_0x002199e4,
        block_0x002199e8,
        block_0x002199ec,
        block_0x002199f8,
        block_0x002199fc,
        block_0x00219a08,
        block_0x00219a34,
        block_0x00219a64,
        block_0x00219a88,
        block_0x00219ad8,
        block_0x00219ae4,
        block_0x00219af0,
        block_0x00219b40,
        block_0x00219b48,
        block_0x00219b68,
        block_0x00219b88,
        block_0x00219bd0,
        block_0x00219bdc,
        block_0x00219be8,
        block_0x00219c04,
        block_0x00219c08,
        block_0x00219c28,
        block_0x00219c2c,
        block_0x00219c34,
        block_0x00219c40,
        block_0x00219ca8,
        block_0x00219cf4,
        block_0x00219d0c,
        block_0x00219d18,
        block_0x00219d1c,
        block_0x00219d50,
        block_0x00219d68,
        block_0x00219d6c,
        block_0x00219d7c,
        block_0x00219d84,
        block_0x00219db4,
        block_0x00219dbc,
        block_0x00219dc4,
        block_0x00219dc8,
        block_0x00219dd4,
        block_0x00219de4,
        block_0x00219e38,
        block_0x00219e48,
        block_0x00219e4c,
        block_0x00219e58,
        block_0x00219e74,
        block_0x00219e90,
        block_0x00219e94,
        block_0x00219eb0,
        block_0x00219eb4,
        block_0x00219ec4,
        block_0x00219ecc,
        block_0x00219ed4,
        block_0x00219ee4,
        block_0x00219f04,
        block_0x00219f18,
        block_0x00219f2c,
        block_0x00219f44,
        block_0x00219f6c,
        block_0x00219f8c,
        block_0x00219f94,
        block_0x00219fa4,
        block_0x00219fa8,
        block_0x00219fac,
        block_0x00219fb0,
        block_0x00219fbc,
        block_0x00219fc8,
        block_0x00219fd4,
        block_0x00219fd8,
        block_0x00219fe8,
        block_0x00219ffc,
        block_0x0021a010,
        block_0x0021a018,
        block_0x0021a02c,
        block_0x0021a030,
        block_0x0021a034,
        block_0x0021a038,
        block_0x0021a048,
        block_0x0021a064,
        block_0x0021a078,
        block_0x0021a08c,
        block_0x0021a0b0,
        block_0x0021a0b4,
        block_0x0021a0ec,
        block_0x0021a100,
        block_0x0021a114,
    ];
    const IDX: [u16; 733usize] = [
        1u16, 0u16, 2u16, 0u16, 0u16, 0u16, 0u16, 3u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 4u16, 0u16, 0u16, 5u16, 0u16, 6u16, 0u16, 7u16, 0u16,
        0u16, 0u16, 0u16, 8u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 9u16, 0u16, 0u16, 10u16, 0u16, 0u16, 0u16, 0u16, 0u16, 11u16,
        0u16, 0u16, 0u16, 0u16, 12u16, 0u16, 13u16, 0u16, 14u16, 0u16, 15u16, 0u16, 0u16,
        16u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 17u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 18u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 19u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 20u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 21u16, 0u16, 0u16, 0u16,
        22u16, 23u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        24u16, 0u16, 0u16, 0u16, 25u16, 0u16, 0u16, 26u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 27u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 28u16, 0u16, 0u16, 29u16,
        0u16, 0u16, 30u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 31u16, 0u16, 0u16,
        32u16, 0u16, 0u16, 0u16, 0u16, 0u16, 33u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 34u16, 0u16, 0u16, 0u16, 35u16, 36u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 37u16, 0u16, 0u16, 0u16, 38u16, 0u16, 0u16, 39u16, 0u16, 0u16, 0u16,
        40u16, 0u16, 0u16, 0u16, 0u16, 0u16, 41u16, 0u16, 0u16, 0u16, 0u16, 0u16, 42u16,
        43u16, 44u16, 0u16, 0u16, 45u16, 46u16, 0u16, 0u16, 47u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 48u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 49u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 50u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 51u16, 0u16, 0u16, 52u16,
        0u16, 0u16, 53u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 54u16, 0u16, 55u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 56u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 57u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 58u16, 0u16, 0u16, 59u16, 0u16, 0u16, 60u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 61u16, 62u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 63u16, 64u16, 0u16, 65u16, 0u16, 0u16, 66u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 67u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 68u16, 0u16, 0u16, 0u16, 0u16, 0u16, 69u16, 0u16, 0u16, 70u16, 71u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 72u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 73u16, 74u16, 0u16, 0u16, 0u16, 75u16, 0u16, 76u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 77u16, 0u16, 78u16,
        0u16, 79u16, 80u16, 0u16, 0u16, 81u16, 0u16, 0u16, 0u16, 82u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 83u16, 0u16, 0u16, 0u16, 84u16, 85u16, 0u16, 0u16, 86u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 87u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        88u16, 89u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 90u16, 91u16, 0u16, 0u16, 0u16,
        92u16, 0u16, 93u16, 0u16, 94u16, 0u16, 0u16, 0u16, 95u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 96u16, 0u16, 0u16, 0u16, 0u16, 97u16, 0u16, 0u16, 0u16, 0u16,
        98u16, 0u16, 0u16, 0u16, 0u16, 0u16, 99u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 100u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 101u16, 0u16,
        102u16, 0u16, 0u16, 0u16, 103u16, 104u16, 105u16, 106u16, 0u16, 0u16, 107u16,
        0u16, 0u16, 108u16, 0u16, 0u16, 109u16, 110u16, 0u16, 0u16, 0u16, 111u16, 0u16,
        0u16, 0u16, 0u16, 112u16, 0u16, 0u16, 0u16, 0u16, 113u16, 0u16, 114u16, 0u16,
        0u16, 0u16, 0u16, 115u16, 116u16, 117u16, 118u16, 0u16, 0u16, 0u16, 119u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 120u16, 0u16, 0u16, 0u16, 0u16, 121u16, 0u16, 0u16,
        0u16, 0u16, 122u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 123u16,
        124u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 125u16, 0u16, 0u16, 0u16, 0u16, 126u16, 0u16, 0u16, 0u16, 0u16, 127u16,
    ];
    if pc < 2200996u32 || pc > 2203924u32 {
        return None;
    }
    let word_offset = ((pc - 2200996u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x002195a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(5usize, 2200996u32, 4096u32, 2201000u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(5usize);
    let return_addr = 2201004u32;
    emu.write_reg_no_count(5usize, return_addr);
    let target = base.wrapping_add(4294965456u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002195ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 12usize, 0u32, 2201008u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2201012u32);
    emu.adi_no_count(19usize, 10usize, 0u32, 2201016u32);
    emu.apc_no_count(1usize, 2201016u32, 4096u32, 2201020u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2201024u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966156u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002195c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 10usize, 0u32, 2201028u32);
    emu.lw_no_count(10usize, 19usize, 0u32, 2201032u32)?;
    emu.lw_no_count(11usize, 19usize, 4u32, 2201036u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2201040u32)?;
    emu.sw_no_count(11usize, 2usize, 24u32, 2201044u32)?;
    emu.sw_no_count(9usize, 2usize, 28u32, 2201048u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2201052u32)?;
    emu.adi_no_count(10usize, 2usize, 20u32, 2201056u32);
    emu.apc_no_count(1usize, 2201056u32, 4096u32, 2201060u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2201064u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967072u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002195e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 19usize, 8u32, 2201068u32)?;
    emu.lbu_no_count(10usize, 10usize, 0u32, 2201072u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2201200u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219670));
    } else {
        emu.pc = 2201076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002195f4));
    }
}
#[inline(always)]
pub fn block_0x002195f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2201080u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2201176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219658));
    } else {
        emu.pc = 2201084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002195fc));
    }
}
#[inline(always)]
pub fn block_0x002195fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 2u32, 2201088u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2201244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021969c));
    } else {
        emu.pc = 2201092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219604));
    }
}
#[inline(always)]
pub fn block_0x00219604(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2248704u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2201096u32;
    emu.update_insn_clock();
    emu.lbu_no_count(12usize, 11usize, 4294965824u32, 2201100u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2201104u32);
    emu.sb_no_count(10usize, 11usize, 4294965824u32, 2201108u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2201244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021969c));
    } else {
        emu.pc = 2201112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219618));
    }
}
#[inline]
pub fn block_0x00219618(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2201116u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1688u32, 2201120u32);
    emu.sw_no_count(0usize, 2usize, 36u32, 2201124u32)?;
    emu.adi_no_count(12usize, 0usize, 4u32, 2201128u32);
    emu.lw_no_count(13usize, 18usize, 36u32, 2201132u32)?;
    emu.sw_no_count(11usize, 2usize, 20u32, 2201136u32)?;
    emu.sw_no_count(10usize, 2usize, 24u32, 2201140u32)?;
    emu.sw_no_count(12usize, 2usize, 28u32, 2201144u32)?;
    emu.sw_no_count(0usize, 2usize, 32u32, 2201148u32)?;
    emu.adi_no_count(10usize, 2usize, 12u32, 2201152u32);
    emu.adi_no_count(12usize, 2usize, 20u32, 2201156u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2201160u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2201164u32;
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
pub fn block_0x0021964c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 12u32, 2201168u32);
    emu.lw_no_count(11usize, 2usize, 16u32, 2201172u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2201176u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2201236u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219694));
}
#[inline(always)]
pub fn block_0x00219658(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 20u32, 2201180u32);
    emu.adi_no_count(11usize, 2usize, 12u32, 2201184u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2201188u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2201192u32);
    emu.adi_no_count(13usize, 18usize, 0u32, 2201196u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2201200u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2201220u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219684));
}
#[inline(always)]
pub fn block_0x00219670(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 20u32, 2201204u32);
    emu.adi_no_count(11usize, 2usize, 12u32, 2201208u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2201212u32);
    emu.adi_no_count(13usize, 18usize, 0u32, 2201216u32);
    emu.adi_no_count(14usize, 0usize, 0u32, 2201220u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2201220u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219684));
}
#[inline(always)]
pub fn block_0x00219684(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2201220u32, 4096u32, 2201224u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2201228u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966064u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021968c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 20u32, 2201232u32);
    emu.lw_no_count(11usize, 2usize, 24u32, 2201236u32)?;
    emu.add_memory_rw_events(2usize);
    emu.pc = 2201236u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219694));
}
#[inline(always)]
pub fn block_0x00219694(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2201236u32, 4294963200u32, 2201240u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2201244u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1976u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021969c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(0usize, 8usize, 0u32, 2201248u32);
    emu.apc_no_count(6usize, 2201248u32, 0u32, 2201252u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2201256u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1976u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002196a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2201260u32);
    emu.lw_no_count(11usize, 10usize, 0u32, 2201264u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2201268u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2201272u32)?;
    emu.sw_no_count(12usize, 2usize, 8u32, 2201276u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2201280u32)?;
    emu.adi_no_count(10usize, 2usize, 4u32, 2201284u32);
    emu.apc_no_count(1usize, 2201284u32, 0u32, 2201288u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2201292u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(776u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002196cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2201296u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2201300u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2201304u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2201308u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2201312u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2201316u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2201452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021976c));
    } else {
        emu.pc = 2201320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002196e8));
    }
}
#[inline]
pub fn block_0x002196e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 12u32, 2201324u32)?;
    emu.adi_no_count(11usize, 0usize, 1u32, 2201328u32);
    emu.sw_no_count(0usize, 2usize, 20u32, 2201332u32)?;
    emu.sw_no_count(11usize, 2usize, 24u32, 2201336u32)?;
    emu.sw_no_count(0usize, 2usize, 28u32, 2201340u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2201344u32)?;
    emu.lw_no_count(11usize, 10usize, 0u32, 2201348u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2201352u32)?;
    emu.lw_no_count(13usize, 10usize, 8u32, 2201356u32)?;
    emu.lw_no_count(14usize, 10usize, 12u32, 2201360u32)?;
    emu.lw_no_count(15usize, 10usize, 16u32, 2201364u32)?;
    emu.lw_no_count(10usize, 10usize, 20u32, 2201368u32)?;
    emu.sw_no_count(11usize, 2usize, 32u32, 2201372u32)?;
    emu.sw_no_count(12usize, 2usize, 36u32, 2201376u32)?;
    emu.sw_no_count(13usize, 2usize, 40u32, 2201380u32)?;
    emu.sw_no_count(14usize, 2usize, 44u32, 2201384u32)?;
    emu.sw_no_count(15usize, 2usize, 48u32, 2201388u32)?;
    emu.sw_no_count(10usize, 2usize, 52u32, 2201392u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2201396u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1696u32, 2201400u32);
    emu.adi_no_count(10usize, 2usize, 20u32, 2201404u32);
    emu.adi_no_count(12usize, 2usize, 32u32, 2201408u32);
    emu.apc_no_count(1usize, 2201408u32, 16384u32, 2201412u32);
    emu.add_memory_rw_events(24usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2201416u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965788u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00219748(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 20u32, 2201420u32)?;
    emu.lw_no_count(11usize, 2usize, 24u32, 2201424u32)?;
    emu.lw_no_count(12usize, 2usize, 28u32, 2201428u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2201432u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2201436u32)?;
    emu.sw_no_count(12usize, 2usize, 40u32, 2201440u32)?;
    emu.sw_no_count(10usize, 8usize, 0u32, 2201444u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2201448u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2201452u32)?;
    emu.add_memory_rw_events(9usize);
    emu.pc = 2201452u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021976c));
}
#[inline]
pub fn block_0x0021976c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 0u32, 2201456u32)?;
    emu.lw_no_count(11usize, 8usize, 4u32, 2201460u32)?;
    emu.lw_no_count(12usize, 8usize, 8u32, 2201464u32)?;
    emu.sw_no_count(10usize, 2usize, 8u32, 2201468u32)?;
    emu.sw_no_count(11usize, 2usize, 12u32, 2201472u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2201476u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2201480u32);
    emu.sw_no_count(0usize, 8usize, 0u32, 2201484u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2201488u32)?;
    emu.sw_no_count(0usize, 8usize, 8u32, 2201492u32)?;
    emu.apc_no_count(1usize, 2201492u32, 4294897664u32, 2201496u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2201500u32;
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
pub fn block_0x0021979c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 12u32, 2201504u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2201508u32);
    emu.apc_no_count(1usize, 2201508u32, 4294897664u32, 2201512u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2201516u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967080u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002197ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2201568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002197e0));
    } else {
        emu.pc = 2201520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002197b0));
    }
}
#[inline]
pub fn block_0x002197b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 2usize, 8u32, 2201524u32)?;
    emu.lw_no_count(13usize, 2usize, 12u32, 2201528u32)?;
    emu.lw_no_count(14usize, 2usize, 16u32, 2201532u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2201536u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1720u32, 2201540u32);
    emu.sw_no_count(12usize, 10usize, 0u32, 2201544u32)?;
    emu.sw_no_count(13usize, 10usize, 4u32, 2201548u32)?;
    emu.sw_no_count(14usize, 10usize, 8u32, 2201552u32)?;
    emu.lw_no_count(1usize, 2usize, 60u32, 2201556u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2201560u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2201564u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2201568u32;
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
pub fn block_0x002197e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2201572u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2201576u32);
    emu.apc_no_count(1usize, 2201576u32, 4096u32, 2201580u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2201584u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(804u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002197f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2201588u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2201592u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2201760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002198a0));
    } else {
        emu.pc = 2201596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002197fc));
    }
}
#[inline(never)]
pub fn block_0x002197fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2201600u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2201604u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2201608u32)?;
    emu.lw_no_count(11usize, 10usize, 12u32, 2201612u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2201616u32);
    emu.sw_no_count(0usize, 2usize, 4u32, 2201620u32)?;
    emu.sw_no_count(12usize, 2usize, 8u32, 2201624u32)?;
    emu.sw_no_count(0usize, 2usize, 12u32, 2201628u32)?;
    emu.lw_no_count(11usize, 11usize, 0u32, 2201632u32)?;
    emu.lw_no_count(12usize, 11usize, 0u32, 2201636u32)?;
    emu.lw_no_count(13usize, 11usize, 4u32, 2201640u32)?;
    emu.lw_no_count(14usize, 11usize, 8u32, 2201644u32)?;
    emu.lw_no_count(15usize, 11usize, 12u32, 2201648u32)?;
    emu.lw_no_count(16usize, 11usize, 16u32, 2201652u32)?;
    emu.lw_no_count(11usize, 11usize, 20u32, 2201656u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2201660u32)?;
    emu.sw_no_count(13usize, 2usize, 20u32, 2201664u32)?;
    emu.sw_no_count(14usize, 2usize, 24u32, 2201668u32)?;
    emu.sw_no_count(15usize, 2usize, 28u32, 2201672u32)?;
    emu.sw_no_count(16usize, 2usize, 32u32, 2201676u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2201680u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2201684u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1696u32, 2201688u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2201692u32);
    emu.adi_no_count(10usize, 2usize, 4u32, 2201696u32);
    emu.adi_no_count(12usize, 2usize, 16u32, 2201700u32);
    emu.apc_no_count(1usize, 2201700u32, 16384u32, 2201704u32);
    emu.add_memory_rw_events(28usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2201708u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965496u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021986c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 0u32, 2201712u32);
    emu.lw_no_count(11usize, 2usize, 4u32, 2201716u32)?;
    emu.lw_no_count(12usize, 2usize, 8u32, 2201720u32)?;
    emu.lw_no_count(13usize, 2usize, 12u32, 2201724u32)?;
    emu.sw_no_count(11usize, 2usize, 16u32, 2201728u32)?;
    emu.sw_no_count(12usize, 2usize, 20u32, 2201732u32)?;
    emu.sw_no_count(13usize, 2usize, 24u32, 2201736u32)?;
    emu.sw_no_count(11usize, 8usize, 0u32, 2201740u32)?;
    emu.sw_no_count(12usize, 8usize, 4u32, 2201744u32)?;
    emu.sw_no_count(13usize, 8usize, 8u32, 2201748u32)?;
    emu.lw_no_count(1usize, 2usize, 44u32, 2201752u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2201756u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2201760u32);
    emu.add_memory_rw_events(13usize);
    emu.pc = 2201760u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002198a0));
}
#[inline(always)]
pub fn block_0x002198a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2201764u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1720u32, 2201768u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2201772u32;
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
pub fn block_0x002198ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2201776u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2201780u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2201880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219918));
    } else {
        emu.pc = 2201784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002198b8));
    }
}
#[inline]
pub fn block_0x002198b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2201788u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2201792u32)?;
    emu.lw_no_count(10usize, 10usize, 12u32, 2201796u32)?;
    emu.lw_no_count(12usize, 10usize, 0u32, 2201800u32)?;
    emu.lw_no_count(10usize, 11usize, 0u32, 2201804u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2201808u32)?;
    emu.lw_no_count(13usize, 12usize, 0u32, 2201812u32)?;
    emu.lw_no_count(14usize, 12usize, 4u32, 2201816u32)?;
    emu.lw_no_count(15usize, 12usize, 8u32, 2201820u32)?;
    emu.lw_no_count(16usize, 12usize, 12u32, 2201824u32)?;
    emu.lw_no_count(17usize, 12usize, 16u32, 2201828u32)?;
    emu.lw_no_count(12usize, 12usize, 20u32, 2201832u32)?;
    emu.sw_no_count(13usize, 2usize, 4u32, 2201836u32)?;
    emu.sw_no_count(14usize, 2usize, 8u32, 2201840u32)?;
    emu.sw_no_count(15usize, 2usize, 12u32, 2201844u32)?;
    emu.sw_no_count(16usize, 2usize, 16u32, 2201848u32)?;
    emu.sw_no_count(17usize, 2usize, 20u32, 2201852u32)?;
    emu.sw_no_count(12usize, 2usize, 24u32, 2201856u32)?;
    emu.adi_no_count(12usize, 2usize, 4u32, 2201860u32);
    emu.apc_no_count(1usize, 2201860u32, 16384u32, 2201864u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2201868u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965336u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021990c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2201872u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2201876u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2201880u32;
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
pub fn block_0x00219918(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 4u32, 2201884u32)?;
    emu.lw_no_count(12usize, 10usize, 8u32, 2201888u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2201892u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2201896u32);
    emu.apc_no_count(6usize, 2201896u32, 16384u32, 2201900u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2201904u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(312u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00219930(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2201908u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2201912u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2201916u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2201920u32)?;
    emu.lw_no_count(8usize, 10usize, 0u32, 2201924u32)?;
    emu.lw_no_count(9usize, 10usize, 4u32, 2201928u32)?;
    emu.apc_no_count(1usize, 2201928u32, 4294897664u32, 2201932u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2201936u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1468u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00219950(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 8u32, 2201940u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2201944u32);
    emu.apc_no_count(1usize, 2201944u32, 4294897664u32, 2201948u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2201952u32;
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
pub fn block_0x00219960(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2201992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219988));
    } else {
        emu.pc = 2201956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219964));
    }
}
#[inline]
pub fn block_0x00219964(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2201960u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1736u32, 2201964u32);
    emu.sw_no_count(8usize, 10usize, 0u32, 2201968u32)?;
    emu.sw_no_count(9usize, 10usize, 4u32, 2201972u32)?;
    emu.lw_no_count(1usize, 2usize, 12u32, 2201976u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2201980u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2201984u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2201988u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2201992u32;
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
pub fn block_0x00219988(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2201996u32);
    emu.adi_no_count(11usize, 0usize, 8u32, 2202000u32);
    emu.apc_no_count(1usize, 2202000u32, 4096u32, 2202004u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2202008u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(380u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00219998(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2202012u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1736u32, 2202016u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2202020u32;
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
pub fn block_0x002199a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2202024u32)?;
    emu.lw_no_count(11usize, 10usize, 4u32, 2202028u32)?;
    emu.adi_no_count(10usize, 12usize, 0u32, 2202032u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2202036u32;
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
pub fn block_0x002199b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 0u32, 2202040u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2202044u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2202048u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2202052u32);
    emu.apc_no_count(6usize, 2202052u32, 16384u32, 2202056u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2202060u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x002199cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2202064u32);
    emu.lw_no_count(11usize, 10usize, 0u32, 2202068u32)?;
    emu.lw_no_count(13usize, 11usize, 4u32, 2202072u32)?;
    emu.lw_no_count(12usize, 11usize, 12u32, 2202076u32)?;
    emu.adi_no_count(14usize, 0usize, 1u32, 2202080u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2202104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002199f8));
    } else {
        emu.pc = 2202084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002199e4));
    }
}
#[inline(always)]
pub fn block_0x002199e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2202164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219a34));
    } else {
        emu.pc = 2202088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002199e8));
    }
}
#[inline(always)]
pub fn block_0x002199e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2202164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219a34));
    } else {
        emu.pc = 2202092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002199ec));
    }
}
#[inline(always)]
pub fn block_0x002199ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2202096u32);
    emu.adi_no_count(15usize, 0usize, 1u32, 2202100u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2202104u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2202120u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219a08));
}
#[inline(always)]
pub fn block_0x002199f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2202164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219a34));
    } else {
        emu.pc = 2202108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002199fc));
    }
}
#[inline(always)]
pub fn block_0x002199fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 11usize, 0u32, 2202112u32)?;
    emu.lw_no_count(15usize, 11usize, 0u32, 2202116u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2202120u32)?;
    emu.add_memory_rw_events(3usize);
    emu.pc = 2202120u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219a08));
}
#[inline]
pub fn block_0x00219a08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 10usize, 8u32, 2202124u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2202128u32)?;
    emu.lbu_no_count(13usize, 14usize, 8u32, 2202132u32);
    emu.lbu_no_count(14usize, 14usize, 9u32, 2202136u32);
    emu.sw_no_count(15usize, 2usize, 0u32, 2202140u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2202144u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2202148u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1752u32, 2202152u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2202156u32);
    emu.apc_no_count(1usize, 2202156u32, 0u32, 2202160u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2202164u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(348u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00219a34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 8u32, 2202168u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2202172u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2202176u32)?;
    emu.lbu_no_count(13usize, 11usize, 8u32, 2202180u32);
    emu.lbu_no_count(14usize, 11usize, 9u32, 2202184u32);
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2202188u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 2usize, 0u32, 2202192u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2202196u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1780u32, 2202200u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2202204u32);
    emu.apc_no_count(1usize, 2202204u32, 0u32, 2202208u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2202212u32;
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
#[inline]
pub fn block_0x00219a64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2202216u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2202220u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2202224u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2202228u32)?;
    emu.lw_no_count(9usize, 11usize, 12u32, 2202232u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2202236u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2202240u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2202244u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(9usize);
    let return_addr = 2202248u32;
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
#[inline]
pub fn block_0x00219a88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2202252u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2202256u32)?;
    emu.lw_no_count(12usize, 2usize, 0u32, 2202260u32)?;
    emu.lw_no_count(13usize, 2usize, 4u32, 2202264u32)?;
    let a = 0u32.wrapping_add(3112902656u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2202268u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966129u32, 2202272u32);
    emu.xrr_no_count(11usize, 11usize, 14usize, 2202276u32);
    let a = 0u32.wrapping_add(1676365824u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2202280u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 44u32, 2202284u32);
    emu.xrr_no_count(13usize, 13usize, 14usize, 2202288u32);
    let a = 0u32.wrapping_add(1470513152u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2202292u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 376u32, 2202296u32);
    emu.xrr_no_count(10usize, 10usize, 14usize, 2202300u32);
    let a = 0u32.wrapping_add(3603652608u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2202304u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966637u32, 2202308u32);
    emu.xrr_no_count(12usize, 12usize, 14usize, 2202312u32);
    emu.orr_no_count(11usize, 13usize, 11usize, 2202316u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2202320u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2202324u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2202340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219ae4));
    } else {
        emu.pc = 2202328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219ad8));
    }
}
#[inline(always)]
pub fn block_0x00219ad8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 4u32, 2202332u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2202336u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2202340u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2202440u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219b48));
}
#[inline(always)]
pub fn block_0x00219ae4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2202344u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2202348u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(9usize);
    let return_addr = 2202352u32;
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
#[inline]
pub fn block_0x00219af0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2202356u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2202360u32)?;
    emu.lw_no_count(12usize, 2usize, 0u32, 2202364u32)?;
    emu.lw_no_count(13usize, 2usize, 4u32, 2202368u32)?;
    let a = 0u32.wrapping_add(2330587136u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2202372u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 428u32, 2202376u32);
    emu.xrr_no_count(11usize, 11usize, 14usize, 2202380u32);
    let a = 0u32.wrapping_add(2965131264u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2202384u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965685u32, 2202388u32);
    emu.xrr_no_count(13usize, 13usize, 14usize, 2202392u32);
    let a = 0u32.wrapping_add(4112453632u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2202396u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1473u32, 2202400u32);
    emu.xrr_no_count(10usize, 10usize, 14usize, 2202404u32);
    let a = 0u32.wrapping_add(2020298752u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2202408u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965580u32, 2202412u32);
    emu.xrr_no_count(12usize, 12usize, 14usize, 2202416u32);
    emu.orr_no_count(11usize, 13usize, 11usize, 2202420u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2202424u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2202428u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2202472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219b68));
    } else {
        emu.pc = 2202432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219b40));
    }
}
#[inline(always)]
pub fn block_0x00219b40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 4u32, 2202436u32);
    emu.adi_no_count(11usize, 0usize, 8u32, 2202440u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2202440u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219b48));
}
#[inline(always)]
pub fn block_0x00219b48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2202444u32)?;
    emu.adr_no_count(11usize, 8usize, 11usize, 2202448u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2202452u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2202456u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2202460u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2202464u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2202468u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2202472u32;
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
pub fn block_0x00219b68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2202476u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1808u32, 2202480u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2202484u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2202488u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2202492u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2202496u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2202500u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2202504u32;
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
pub fn block_0x00219b88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967200u32, 2202508u32);
    emu.sw_no_count(1usize, 2usize, 92u32, 2202512u32)?;
    emu.sw_no_count(8usize, 2usize, 88u32, 2202516u32)?;
    emu.sw_no_count(9usize, 2usize, 84u32, 2202520u32)?;
    emu.sw_no_count(18usize, 2usize, 80u32, 2202524u32)?;
    emu.sw_no_count(19usize, 2usize, 76u32, 2202528u32)?;
    emu.sw_no_count(20usize, 2usize, 72u32, 2202532u32)?;
    emu.adi_no_count(19usize, 14usize, 0u32, 2202536u32);
    emu.adi_no_count(18usize, 13usize, 0u32, 2202540u32);
    emu.adi_no_count(20usize, 12usize, 0u32, 2202544u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2202548u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2202552u32);
    emu.sw_no_count(10usize, 2usize, 4u32, 2202556u32)?;
    emu.sw_no_count(11usize, 2usize, 8u32, 2202560u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2202564u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2202568u32);
    emu.apc_no_count(1usize, 2202568u32, 4096u32, 2202572u32);
    emu.add_memory_rw_events(18usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2202576u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966360u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00219bd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 255u32, 2202580u32);
    emu.adi_no_count(11usize, 0usize, 2u32, 2202584u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2202668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219c2c));
    } else {
        emu.pc = 2202588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219bdc));
    }
}
#[inline(always)]
pub fn block_0x00219bdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2248704u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2202592u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 10usize, 4294965812u32, 2202596u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2203080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219dc8));
    } else {
        emu.pc = 2202600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219be8));
    }
}
#[inline(always)]
pub fn block_0x00219be8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 4294965812u32, 2202604u32);
    emu.lw_no_count(12usize, 12usize, 4u32, 2202608u32)?;
    emu.adi_no_count(11usize, 11usize, 1u32, 2202612u32);
    emu.sw_no_count(11usize, 10usize, 4294965812u32, 2202616u32)?;
    emu.lw_no_count(11usize, 9usize, 20u32, 2202620u32)?;
    emu.adi_no_count(10usize, 8usize, 0u32, 2202624u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2202904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219d18));
    } else {
        emu.pc = 2202628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219c04));
    }
}
#[inline(always)]
pub fn block_0x00219c04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2202632u32;
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
pub fn block_0x00219c08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 48u32, 2202636u32)?;
    emu.sw_no_count(11usize, 2usize, 52u32, 2202640u32)?;
    emu.sw_no_count(20usize, 2usize, 56u32, 2202644u32)?;
    emu.sb_no_count(18usize, 2usize, 60u32, 2202648u32);
    emu.sb_no_count(19usize, 2usize, 61u32, 2202652u32);
    emu.adi_no_count(10usize, 2usize, 48u32, 2202656u32);
    emu.apc_no_count(1usize, 2202656u32, 0u32, 2202660u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2202664u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965372u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00219c28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2202668u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2202960u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219d50));
}
#[inline(always)]
pub fn block_0x00219c2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 1u32, 2202672u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2202792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219ca8));
    } else {
        emu.pc = 2202676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219c34));
    }
}
#[inline(always)]
pub fn block_0x00219c34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 9usize, 24u32, 2202680u32)?;
    emu.adi_no_count(10usize, 8usize, 0u32, 2202684u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2202688u32;
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
#[inline(never)]
pub fn block_0x00219c40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 26u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(12usize, 10usize, 1u32, 2202692u32);
    emu.adi_no_count(13usize, 2usize, 12u32, 2202696u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2202700u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1512u32, 2202704u32);
    emu.adi_no_count(15usize, 2usize, 16u32, 2202708u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2202712u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 800u32, 2202716u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2202720u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 1936u32, 2202724u32);
    emu.sw_no_count(13usize, 2usize, 32u32, 2202728u32)?;
    emu.sw_no_count(14usize, 2usize, 36u32, 2202732u32)?;
    emu.sw_no_count(15usize, 2usize, 40u32, 2202736u32)?;
    emu.sw_no_count(16usize, 2usize, 44u32, 2202740u32)?;
    emu.adi_no_count(13usize, 0usize, 3u32, 2202744u32);
    emu.sw_no_count(0usize, 2usize, 64u32, 2202748u32)?;
    emu.adr_no_count(10usize, 10usize, 12usize, 2202752u32);
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2202756u32);
    emu.anr_no_count(11usize, 12usize, 11usize, 2202760u32);
    emu.adi_no_count(12usize, 2usize, 32u32, 2202764u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2202768u32)?;
    emu.sw_no_count(11usize, 2usize, 20u32, 2202772u32)?;
    emu.adi_no_count(10usize, 0usize, 2u32, 2202776u32);
    emu.sw_no_count(17usize, 2usize, 48u32, 2202780u32)?;
    emu.sw_no_count(13usize, 2usize, 52u32, 2202784u32)?;
    emu.sw_no_count(12usize, 2usize, 56u32, 2202788u32)?;
    emu.add_memory_rw_events(26usize);
    let return_addr = 2202792u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2202868u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219cf4));
}
#[inline]
pub fn block_0x00219ca8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2202796u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2202800u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1512u32, 2202804u32);
    emu.adi_no_count(12usize, 2usize, 4u32, 2202808u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2202812u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1264u32, 2202816u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2202820u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1848u32, 2202824u32);
    emu.adi_no_count(15usize, 0usize, 3u32, 2202828u32);
    emu.sw_no_count(0usize, 2usize, 64u32, 2202832u32)?;
    emu.adi_no_count(16usize, 2usize, 32u32, 2202836u32);
    emu.sw_no_count(10usize, 2usize, 32u32, 2202840u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2202844u32)?;
    emu.sw_no_count(12usize, 2usize, 40u32, 2202848u32)?;
    emu.sw_no_count(13usize, 2usize, 44u32, 2202852u32)?;
    emu.adi_no_count(10usize, 0usize, 2u32, 2202856u32);
    emu.sw_no_count(14usize, 2usize, 48u32, 2202860u32)?;
    emu.sw_no_count(15usize, 2usize, 52u32, 2202864u32)?;
    emu.sw_no_count(16usize, 2usize, 56u32, 2202868u32)?;
    emu.add_memory_rw_events(19usize);
    emu.pc = 2202868u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219cf4));
}
#[inline(always)]
pub fn block_0x00219cf4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 60u32, 2202872u32)?;
    emu.adi_no_count(10usize, 2usize, 24u32, 2202876u32);
    emu.adi_no_count(11usize, 2usize, 24u32, 2202880u32);
    emu.adi_no_count(12usize, 2usize, 48u32, 2202884u32);
    emu.apc_no_count(1usize, 2202884u32, 0u32, 2202888u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2202892u32;
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
pub fn block_0x00219d0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 24u32, 2202896u32);
    emu.lw_no_count(11usize, 2usize, 28u32, 2202900u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2202904u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2203068u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219dbc));
}
#[inline(always)]
pub fn block_0x00219d18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2202908u32;
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
#[inline]
pub fn block_0x00219d1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2248704u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2202912u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965812u32, 2202916u32);
    emu.lw_no_count(13usize, 12usize, 8u32, 2202920u32)?;
    emu.lw_no_count(12usize, 12usize, 4u32, 2202924u32)?;
    emu.lw_no_count(13usize, 13usize, 20u32, 2202928u32)?;
    emu.sw_no_count(10usize, 2usize, 48u32, 2202932u32)?;
    emu.sw_no_count(11usize, 2usize, 52u32, 2202936u32)?;
    emu.sw_no_count(20usize, 2usize, 56u32, 2202940u32)?;
    emu.sb_no_count(18usize, 2usize, 60u32, 2202944u32);
    emu.sb_no_count(19usize, 2usize, 61u32, 2202948u32);
    emu.adi_no_count(11usize, 2usize, 48u32, 2202952u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2202956u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2202960u32;
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
pub fn block_0x00219d50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2248704u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2202964u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 10usize, 4294965812u32, 2202968u32)?;
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2202972u32);
    emu.sw_no_count(11usize, 10usize, 4294965812u32, 2202976u32)?;
    emu.apc_no_count(1usize, 2202976u32, 4096u32, 2202980u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2202984u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966028u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00219d68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2203004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219d7c));
    } else {
        emu.pc = 2202988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219d6c));
    }
}
#[inline(always)]
pub fn block_0x00219d6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 0u32, 2202992u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2202996u32);
    emu.apc_no_count(1usize, 2202996u32, 0u32, 2203000u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2203004u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(96u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00219d7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2203008u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 2008u32, 2203012u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2203012u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219d84));
}
#[inline]
pub fn block_0x00219d84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2203016u32);
    emu.sw_no_count(0usize, 2usize, 64u32, 2203020u32)?;
    emu.adi_no_count(12usize, 0usize, 4u32, 2203024u32);
    emu.sw_no_count(10usize, 2usize, 48u32, 2203028u32)?;
    emu.sw_no_count(11usize, 2usize, 52u32, 2203032u32)?;
    emu.sw_no_count(12usize, 2usize, 56u32, 2203036u32)?;
    emu.sw_no_count(0usize, 2usize, 60u32, 2203040u32)?;
    emu.adi_no_count(10usize, 2usize, 32u32, 2203044u32);
    emu.adi_no_count(11usize, 2usize, 24u32, 2203048u32);
    emu.adi_no_count(12usize, 2usize, 48u32, 2203052u32);
    emu.apc_no_count(1usize, 2203052u32, 0u32, 2203056u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2203060u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(892u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00219db4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 32u32, 2203064u32);
    emu.lw_no_count(11usize, 2usize, 36u32, 2203068u32)?;
    emu.add_memory_rw_events(2usize);
    emu.pc = 2203068u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219dbc));
}
#[inline(always)]
pub fn block_0x00219dbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2203068u32, 4294963200u32, 2203072u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2203076u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(144u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00219dc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.update_insn_clock();
    return Err(format!("Unimplemented instruction (UNIMP) at PC {:#x}", 2203076u32));
}
#[inline(always)]
pub fn block_0x00219dc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2203084u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965356u32, 2203088u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2203092u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2203012u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219d84));
}
#[inline(always)]
pub fn block_0x00219dd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2203096u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2203100u32)?;
    emu.apc_no_count(1usize, 2203100u32, 4294897664u32, 2203104u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2203108u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(588u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00219de4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 12u32, 2203112u32)?;
    emu.adi_no_count(10usize, 2usize, 12u32, 2203116u32);
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2203120u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1064u32, 2203124u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2203128u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965280u32, 2203132u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2203136u32);
    emu.sw_no_count(0usize, 2usize, 40u32, 2203140u32)?;
    emu.adi_no_count(14usize, 2usize, 48u32, 2203144u32);
    emu.sw_no_count(10usize, 2usize, 48u32, 2203148u32)?;
    emu.sw_no_count(11usize, 2usize, 52u32, 2203152u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2203156u32);
    emu.sw_no_count(12usize, 2usize, 24u32, 2203160u32)?;
    emu.sw_no_count(13usize, 2usize, 28u32, 2203164u32)?;
    emu.sw_no_count(14usize, 2usize, 32u32, 2203168u32)?;
    emu.sw_no_count(10usize, 2usize, 36u32, 2203172u32)?;
    emu.adi_no_count(10usize, 2usize, 16u32, 2203176u32);
    emu.adi_no_count(11usize, 2usize, 59u32, 2203180u32);
    emu.adi_no_count(12usize, 2usize, 24u32, 2203184u32);
    emu.apc_no_count(1usize, 2203184u32, 0u32, 2203188u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2203192u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(760u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00219e38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 16u32, 2203196u32);
    emu.lw_no_count(11usize, 2usize, 20u32, 2203200u32)?;
    emu.apc_no_count(1usize, 2203200u32, 4294963200u32, 2203204u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2203208u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(12u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00219e48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.update_insn_clock();
    return Err(format!("Unimplemented instruction (UNIMP) at PC {:#x}", 2203208u32));
}
#[inline(always)]
pub fn block_0x00219e4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 4u32, 2203216u32);
    emu.sb_no_count(11usize, 10usize, 0u32, 2203220u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2203224u32;
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
pub fn block_0x00219e58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2203228u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2203232u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2203236u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2203240u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2203244u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2203248u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2203252u32;
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
pub fn block_0x00219e74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2203256u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2203260u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2203264u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2203268u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2203272u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2203276u32)?;
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(5usize);
    let return_addr = 2203280u32;
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
pub fn block_0x00219e90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.update_insn_clock();
    return Err(format!("Unimplemented instruction (UNIMP) at PC {:#x}", 2203280u32));
}
#[inline(always)]
pub fn block_0x00219e94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2203288u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2203292u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2203296u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2203300u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2203304u32);
    emu.adi_no_count(12usize, 12usize, 4294967288u32, 2203308u32);
    emu.sli_no_count(13usize, 13usize, 3u32, 2203312u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2203312u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219eb0));
}
#[inline(always)]
pub fn block_0x00219eb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2203340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219ecc));
    } else {
        emu.pc = 2203316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219eb4));
    }
}
#[inline(always)]
pub fn block_0x00219eb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(9usize, 12usize, 12u32, 2203320u32)?;
    emu.adi_no_count(12usize, 12usize, 8u32, 2203324u32);
    emu.adi_no_count(13usize, 13usize, 4294967288u32, 2203328u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2203312u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219eb0));
    } else {
        emu.pc = 2203332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219ec4));
    }
}
#[inline(always)]
pub fn block_0x00219ec4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 12usize, 0u32, 2203336u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2203340u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2203348u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219ed4));
}
#[inline(always)]
pub fn block_0x00219ecc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2203344u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2203348u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2203348u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219ed4));
}
#[inline(always)]
pub fn block_0x00219ed4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2203352u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2203356u32);
    emu.apc_no_count(1usize, 2203356u32, 4294901760u32, 2203360u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2203364u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965384u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00219ee4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2203368u32);
    emu.sb_no_count(10usize, 8usize, 0u32, 2203372u32);
    emu.sw_no_count(9usize, 8usize, 4u32, 2203376u32)?;
    emu.lw_no_count(1usize, 2usize, 12u32, 2203380u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2203384u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2203388u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2203392u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2203396u32;
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
pub fn block_0x00219f04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2203400u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2203404u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2203408u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2203412u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2203436u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219f2c));
    } else {
        emu.pc = 2203416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219f18));
    }
}
#[inline(always)]
pub fn block_0x00219f18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2203420u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2203424u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2203428u32);
    emu.apc_no_count(1usize, 2203428u32, 4294901760u32, 2203432u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2203436u32;
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
#[inline(always)]
pub fn block_0x00219f2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2203440u32);
    emu.sb_no_count(10usize, 8usize, 0u32, 2203444u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2203448u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2203452u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2203456u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2203460u32;
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
pub fn block_0x00219f44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2203464u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2203468u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2203472u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2203476u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2203480u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2203484u32)?;
    emu.sw_no_count(20usize, 2usize, 40u32, 2203488u32)?;
    emu.sw_no_count(21usize, 2usize, 36u32, 2203492u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2203496u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2203568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219fb0));
    } else {
        emu.pc = 2203500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219f6c));
    }
}
#[inline(always)]
pub fn block_0x00219f6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2203504u32);
    let a = 0u32.wrapping_add(536870912u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2203508u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 13usize, 4294967295u32, 2203512u32);
    emu.sli_no_count(11usize, 13usize, 3u32, 2203516u32);
    emu.adi_no_count(19usize, 19usize, 4294967295u32, 2203520u32);
    emu.anr_no_count(14usize, 14usize, 19usize, 2203524u32);
    emu.adi_no_count(14usize, 14usize, 1u32, 2203528u32);
    emu.adi_no_count(15usize, 12usize, 4u32, 2203532u32);
    emu.add_memory_rw_events(8usize);
    emu.pc = 2203532u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219f8c));
}
#[inline(always)]
pub fn block_0x00219f8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 15usize, 0u32, 2203536u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a != b {
        emu.pc = 2203560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219fa8));
    } else {
        emu.pc = 2203540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219f94));
    }
}
#[inline(always)]
pub fn block_0x00219f94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 10usize, 1u32, 2203544u32);
    emu.adi_no_count(11usize, 11usize, 4294967288u32, 2203548u32);
    emu.adi_no_count(15usize, 15usize, 8u32, 2203552u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2203532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219f8c));
    } else {
        emu.pc = 2203556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219fa4));
    }
}
#[inline(always)]
pub fn block_0x00219fa4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 14usize, 0u32, 2203560u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2203560u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219fa8));
}
#[inline(always)]
pub fn block_0x00219fa8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2203884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a0ec));
    } else {
        emu.pc = 2203564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219fac));
    }
}
#[inline(always)]
pub fn block_0x00219fac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2203580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219fbc));
    } else {
        emu.pc = 2203568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219fb0));
    }
}
#[inline(always)]
pub fn block_0x00219fb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2203572u32);
    emu.sb_no_count(10usize, 8usize, 0u32, 2203576u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2203580u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2203788u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021a08c));
}
#[inline(always)]
pub fn block_0x00219fbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(20usize, 10usize, 3u32, 2203584u32);
    emu.adr_no_count(20usize, 12usize, 20usize, 2203588u32);
    emu.sbr_no_count(9usize, 13usize, 10usize, 2203592u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2203592u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219fc8));
}
#[inline(always)]
pub fn block_0x00219fc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(21usize, 9usize, 3u32, 2203596u32);
    emu.adi_no_count(10usize, 20usize, 4294967288u32, 2203600u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2203604u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2203604u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219fd4));
}
#[inline(always)]
pub fn block_0x00219fd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2203748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a064));
    } else {
        emu.pc = 2203608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219fd8));
    }
}
#[inline(always)]
pub fn block_0x00219fd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 10usize, 12u32, 2203612u32)?;
    emu.adi_no_count(10usize, 10usize, 8u32, 2203616u32);
    emu.adi_no_count(11usize, 11usize, 4294967288u32, 2203620u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2203604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219fd4));
    } else {
        emu.pc = 2203624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219fe8));
    }
}
#[inline(always)]
pub fn block_0x00219fe8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2203628u32)?;
    emu.adi_no_count(10usize, 0usize, 2u32, 2203632u32);
    emu.adi_no_count(12usize, 18usize, 0u32, 2203636u32);
    emu.apc_no_count(1usize, 2203636u32, 4294897664u32, 2203640u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2203644u32;
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
pub fn block_0x00219ffc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2203648u32);
    emu.adr_no_count(11usize, 9usize, 19usize, 2203652u32);
    emu.anr_no_count(11usize, 11usize, 19usize, 2203656u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2203660u32);
    emu.adi_no_count(12usize, 20usize, 4u32, 2203664u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2203664u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021a010));
}
#[inline(always)]
pub fn block_0x0021a010(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 12usize, 0u32, 2203668u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a < b {
        emu.pc = 2203696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a030));
    } else {
        emu.pc = 2203672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a018));
    }
}
#[inline(always)]
pub fn block_0x0021a018(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(18usize, 18usize, 13usize, 2203676u32);
    emu.adi_no_count(10usize, 10usize, 1u32, 2203680u32);
    emu.adi_no_count(21usize, 21usize, 4294967288u32, 2203684u32);
    emu.adi_no_count(12usize, 12usize, 8u32, 2203688u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a != b {
        emu.pc = 2203664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a010));
    } else {
        emu.pc = 2203692u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a02c));
    }
}
#[inline(always)]
pub fn block_0x0021a02c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 11usize, 0u32, 2203696u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2203696u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021a030));
}
#[inline(always)]
pub fn block_0x0021a030(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a < b {
        emu.pc = 2203904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a100));
    } else {
        emu.pc = 2203700u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a034));
    }
}
#[inline(always)]
pub fn block_0x0021a034(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2203824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a0b0));
    } else {
        emu.pc = 2203704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a038));
    }
}
#[inline(always)]
pub fn block_0x0021a038(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 10usize, 3u32, 2203708u32);
    emu.adr_no_count(20usize, 20usize, 11usize, 2203712u32);
    emu.lw_no_count(11usize, 20usize, 4u32, 2203716u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2203924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a114));
    } else {
        emu.pc = 2203720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a048));
    }
}
#[inline(always)]
pub fn block_0x0021a048(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 20usize, 0u32, 2203724u32)?;
    emu.sbr_no_count(9usize, 9usize, 10usize, 2203728u32);
    emu.sbr_no_count(10usize, 11usize, 18usize, 2203732u32);
    emu.adr_no_count(12usize, 12usize, 18usize, 2203736u32);
    emu.sw_no_count(12usize, 20usize, 0u32, 2203740u32)?;
    emu.sw_no_count(10usize, 20usize, 4u32, 2203744u32)?;
    emu.add_memory_rw_events(7usize);
    let return_addr = 2203748u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2203592u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219fc8));
}
#[inline(always)]
pub fn block_0x0021a064(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2203752u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2203756u32);
    emu.adi_no_count(12usize, 0usize, 0u32, 2203760u32);
    emu.apc_no_count(1usize, 2203760u32, 4294897664u32, 2203764u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2203768u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1780u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021a078(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2203772u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 10usize, 4294965504u32, 2203776u32)?;
    emu.lw_no_count(10usize, 10usize, 4294965508u32, 2203780u32)?;
    emu.sw_no_count(11usize, 8usize, 0u32, 2203784u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2203788u32)?;
    emu.add_memory_rw_events(5usize);
    emu.pc = 2203788u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021a08c));
}
#[inline]
pub fn block_0x0021a08c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2203792u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2203796u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2203800u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2203804u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2203808u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2203812u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2203816u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2203820u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2203824u32;
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
pub fn block_0x0021a0b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2203568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219fb0));
    } else {
        emu.pc = 2203828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a0b4));
    }
}
#[inline]
pub fn block_0x0021a0b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2203832u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965404u32, 2203836u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2203840u32);
    emu.sw_no_count(0usize, 2usize, 28u32, 2203844u32)?;
    emu.adi_no_count(12usize, 0usize, 4u32, 2203848u32);
    emu.sw_no_count(10usize, 2usize, 12u32, 2203852u32)?;
    emu.sw_no_count(11usize, 2usize, 16u32, 2203856u32)?;
    emu.sw_no_count(12usize, 2usize, 20u32, 2203860u32)?;
    emu.sw_no_count(0usize, 2usize, 24u32, 2203864u32)?;
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2203868u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965412u32, 2203872u32);
    emu.adi_no_count(10usize, 2usize, 12u32, 2203876u32);
    emu.apc_no_count(1usize, 2203876u32, 8192u32, 2203880u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2203884u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965880u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021a0ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2203888u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965444u32, 2203892u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2203896u32);
    emu.apc_no_count(1usize, 2203896u32, 20480u32, 2203900u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2203904u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965632u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021a100(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2203908u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965444u32, 2203912u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2203916u32);
    emu.apc_no_count(1usize, 2203916u32, 20480u32, 2203920u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2203924u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965612u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021a114(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2203928u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965428u32, 2203932u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2203936u32);
    emu.apc_no_count(1usize, 2203936u32, 20480u32, 2203940u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2203944u32;
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
