pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2130732u32;
pub const PC_MAX: u32 = 2134332u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 128usize] = [
        block_0x0020832c,
        block_0x00208330,
        block_0x00208368,
        block_0x0020836c,
        block_0x00208370,
        block_0x00208374,
        block_0x00208398,
        block_0x002083b0,
        block_0x002083b8,
        block_0x002083c0,
        block_0x002083c4,
        block_0x002083d8,
        block_0x002083ec,
        block_0x0020843c,
        block_0x00208450,
        block_0x00208454,
        block_0x00208460,
        block_0x002084b0,
        block_0x002084c4,
        block_0x002084d8,
        block_0x002084e4,
        block_0x002084f8,
        block_0x00208500,
        block_0x00208510,
        block_0x0020851c,
        block_0x00208554,
        block_0x0020856c,
        block_0x002085f8,
        block_0x0020860c,
        block_0x00208618,
        block_0x0020862c,
        block_0x00208640,
        block_0x00208688,
        block_0x0020869c,
        block_0x002086a4,
        block_0x002087f8,
        block_0x00208808,
        block_0x00208818,
        block_0x00208828,
        block_0x00208838,
        block_0x00208848,
        block_0x00208858,
        block_0x00208868,
        block_0x00208874,
        block_0x002088c4,
        block_0x002088fc,
        block_0x00208934,
        block_0x0020896c,
        block_0x002089a4,
        block_0x002089dc,
        block_0x00208a14,
        block_0x00208a4c,
        block_0x00208a58,
        block_0x00208a74,
        block_0x00208a84,
        block_0x00208a94,
        block_0x00208abc,
        block_0x00208ac4,
        block_0x00208ad4,
        block_0x00208aec,
        block_0x00208b00,
        block_0x00208b08,
        block_0x00208b0c,
        block_0x00208b24,
        block_0x00208b54,
        block_0x00208b58,
        block_0x00208b6c,
        block_0x00208b94,
        block_0x00208ba8,
        block_0x00208bac,
        block_0x00208bcc,
        block_0x00208bdc,
        block_0x00208be4,
        block_0x00208bec,
        block_0x00208bf4,
        block_0x00208bf8,
        block_0x00208c08,
        block_0x00208c0c,
        block_0x00208c28,
        block_0x00208c34,
        block_0x00208c3c,
        block_0x00208c40,
        block_0x00208c54,
        block_0x00208c6c,
        block_0x00208c7c,
        block_0x00208cb0,
        block_0x00208cc8,
        block_0x00208cfc,
        block_0x00208d04,
        block_0x00208d30,
        block_0x00208d34,
        block_0x00208d48,
        block_0x00208d50,
        block_0x00208d84,
        block_0x00208d8c,
        block_0x00208d94,
        block_0x00208d9c,
        block_0x00208da4,
        block_0x00208dac,
        block_0x00208dd4,
        block_0x00208e34,
        block_0x00208e3c,
        block_0x00208e4c,
        block_0x00208e54,
        block_0x00208e58,
        block_0x00208e88,
        block_0x00208e90,
        block_0x00208ea8,
        block_0x00208eb0,
        block_0x00208ec4,
        block_0x00208edc,
        block_0x00208f3c,
        block_0x00208f44,
        block_0x00208f64,
        block_0x00208fc4,
        block_0x00208fc8,
        block_0x00208fcc,
        block_0x00208fd4,
        block_0x00208fdc,
        block_0x00208fe4,
        block_0x00209010,
        block_0x00209018,
        block_0x00209020,
        block_0x00209024,
        block_0x00209048,
        block_0x00209054,
        block_0x002090e8,
        block_0x0020913c,
    ];
    const IDX: [u16; 901usize] = [
        1u16, 2u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 3u16, 4u16, 5u16, 6u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 7u16, 0u16, 0u16, 0u16, 0u16, 0u16, 8u16, 0u16, 9u16, 0u16, 10u16, 11u16,
        0u16, 0u16, 0u16, 0u16, 12u16, 0u16, 0u16, 0u16, 0u16, 13u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 14u16, 0u16, 0u16, 0u16, 0u16, 15u16, 16u16, 0u16, 0u16, 17u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 18u16, 0u16, 0u16, 0u16, 0u16, 19u16, 0u16,
        0u16, 0u16, 0u16, 20u16, 0u16, 0u16, 21u16, 0u16, 0u16, 0u16, 0u16, 22u16, 0u16,
        23u16, 0u16, 0u16, 0u16, 24u16, 0u16, 0u16, 25u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 26u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 27u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 28u16, 0u16, 0u16,
        0u16, 0u16, 29u16, 0u16, 0u16, 30u16, 0u16, 0u16, 0u16, 0u16, 31u16, 0u16, 0u16,
        0u16, 0u16, 32u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 33u16, 0u16, 0u16, 0u16, 0u16, 34u16,
        0u16, 35u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 36u16, 0u16, 0u16, 0u16, 37u16,
        0u16, 0u16, 0u16, 38u16, 0u16, 0u16, 0u16, 39u16, 0u16, 0u16, 0u16, 40u16, 0u16,
        0u16, 0u16, 41u16, 0u16, 0u16, 0u16, 42u16, 0u16, 0u16, 0u16, 43u16, 0u16, 0u16,
        44u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 45u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 46u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 47u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 48u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 49u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 50u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        51u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 52u16, 0u16, 0u16, 53u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 54u16, 0u16,
        0u16, 0u16, 55u16, 0u16, 0u16, 0u16, 56u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 57u16, 0u16, 58u16, 0u16, 0u16, 0u16, 59u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 60u16, 0u16, 0u16, 0u16, 0u16, 61u16, 0u16, 62u16, 63u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 64u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 65u16, 66u16, 0u16, 0u16, 0u16, 0u16, 67u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 68u16, 0u16, 0u16, 0u16, 0u16, 69u16, 70u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 71u16, 0u16, 0u16, 0u16, 72u16, 0u16, 73u16,
        0u16, 74u16, 0u16, 75u16, 76u16, 0u16, 0u16, 0u16, 77u16, 78u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 79u16, 0u16, 0u16, 80u16, 0u16, 81u16, 82u16, 0u16, 0u16,
        0u16, 0u16, 83u16, 0u16, 0u16, 0u16, 0u16, 0u16, 84u16, 0u16, 0u16, 0u16, 85u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 86u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 87u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 88u16, 0u16, 89u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 90u16, 91u16, 0u16, 0u16, 0u16, 0u16, 92u16, 0u16,
        93u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        94u16, 0u16, 95u16, 0u16, 96u16, 0u16, 97u16, 0u16, 98u16, 0u16, 99u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 100u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 101u16, 0u16, 102u16, 0u16, 0u16, 0u16,
        103u16, 0u16, 104u16, 105u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 106u16, 0u16, 107u16, 0u16, 0u16, 0u16, 0u16, 0u16, 108u16,
        0u16, 109u16, 0u16, 0u16, 0u16, 0u16, 110u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        111u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 112u16, 0u16,
        113u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 114u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 115u16, 116u16, 117u16, 0u16, 118u16, 0u16,
        119u16, 0u16, 120u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        121u16, 0u16, 122u16, 0u16, 123u16, 124u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 125u16, 0u16, 0u16, 126u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 127u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 128u16,
    ];
    if pc < 2130732u32 || pc > 2134332u32 {
        return None;
    }
    let word_offset = ((pc - 2130732u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x0020832c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130736u32;
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
pub fn block_0x00208330(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2130740u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2130744u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2130748u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2130752u32)?;
    emu.sw_no_count(18usize, 2usize, 0u32, 2130756u32)?;
    emu.adi_no_count(8usize, 13usize, 0u32, 2130760u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2130764u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2130768u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2130772u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1452u32, 2130776u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2130780u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2130784u32);
    emu.apc_no_count(1usize, 2130784u32, 4096u32, 2130788u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130792u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966780u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208368(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2130840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208398));
    } else {
        emu.pc = 2130796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020836c));
    }
}
#[inline(always)]
pub fn block_0x0020836c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a < b {
        emu.pc = 2130804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208374));
    } else {
        emu.pc = 2130800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208370));
    }
}
#[inline(always)]
pub fn block_0x00208370(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 18usize, 0u32, 2130804u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2130804u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208374));
}
#[inline]
pub fn block_0x00208374(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 9usize, 0u32, 2130808u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2130812u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2130816u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2130820u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2130824u32)?;
    emu.lw_no_count(18usize, 2usize, 0u32, 2130828u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2130832u32);
    emu.apc_no_count(6usize, 2130832u32, 4096u32, 2130836u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2130840u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x00208398(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2130844u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2130848u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2130852u32)?;
    emu.lw_no_count(18usize, 2usize, 0u32, 2130856u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2130860u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130864u32;
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
pub fn block_0x002083b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2130864u32, 65536u32, 2130868u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2130872u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(896u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002083b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2130876u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130880u32;
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
pub fn block_0x002083c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130884u32;
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
pub fn block_0x002083c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2130888u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2130892u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2130896u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2130900u32)?;
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2130904u32;
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
pub fn block_0x002083d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(9usize, 11usize, 12u32, 2130908u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2130912u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2130916u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2130920u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(9usize);
    let return_addr = 2130924u32;
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
pub fn block_0x002083ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2130928u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2130932u32)?;
    emu.lw_no_count(12usize, 2usize, 0u32, 2130936u32)?;
    emu.lw_no_count(13usize, 2usize, 4u32, 2130940u32)?;
    let a = 0u32.wrapping_add(3112902656u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2130944u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966129u32, 2130948u32);
    emu.xrr_no_count(11usize, 11usize, 14usize, 2130952u32);
    let a = 0u32.wrapping_add(1676365824u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2130956u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 44u32, 2130960u32);
    emu.xrr_no_count(13usize, 13usize, 14usize, 2130964u32);
    let a = 0u32.wrapping_add(1470513152u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2130968u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 376u32, 2130972u32);
    emu.xrr_no_count(10usize, 10usize, 14usize, 2130976u32);
    let a = 0u32.wrapping_add(3603652608u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2130980u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966637u32, 2130984u32);
    emu.xrr_no_count(12usize, 12usize, 14usize, 2130988u32);
    emu.orr_no_count(11usize, 13usize, 11usize, 2130992u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2130996u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2131000u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2131028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208454));
    } else {
        emu.pc = 2131004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020843c));
    }
}
#[inline(always)]
pub fn block_0x0020843c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 4u32, 2131008u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2131012u32);
    emu.adr_no_count(11usize, 8usize, 11usize, 2131016u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2131020u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2131140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002084c4));
    } else {
        emu.pc = 2131024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208450));
    }
}
#[inline(always)]
pub fn block_0x00208450(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2131028u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2131160u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002084d8));
}
#[inline(always)]
pub fn block_0x00208454(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2131032u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2131036u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(9usize);
    let return_addr = 2131040u32;
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
pub fn block_0x00208460(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2131044u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2131048u32)?;
    emu.lw_no_count(12usize, 2usize, 0u32, 2131052u32)?;
    emu.lw_no_count(13usize, 2usize, 4u32, 2131056u32)?;
    let a = 0u32.wrapping_add(2330587136u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2131060u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 428u32, 2131064u32);
    emu.xrr_no_count(11usize, 11usize, 14usize, 2131068u32);
    let a = 0u32.wrapping_add(2965131264u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2131072u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965685u32, 2131076u32);
    emu.xrr_no_count(13usize, 13usize, 14usize, 2131080u32);
    let a = 0u32.wrapping_add(4112453632u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2131084u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1473u32, 2131088u32);
    emu.xrr_no_count(10usize, 10usize, 14usize, 2131092u32);
    let a = 0u32.wrapping_add(2020298752u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2131096u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965580u32, 2131100u32);
    emu.xrr_no_count(12usize, 12usize, 14usize, 2131104u32);
    emu.orr_no_count(11usize, 13usize, 11usize, 2131108u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2131112u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2131116u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2131140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002084c4));
    } else {
        emu.pc = 2131120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002084b0));
    }
}
#[inline(always)]
pub fn block_0x002084b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 4u32, 2131124u32);
    emu.adi_no_count(11usize, 0usize, 8u32, 2131128u32);
    emu.adr_no_count(11usize, 8usize, 11usize, 2131132u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2131136u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2131160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002084d8));
    } else {
        emu.pc = 2131140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002084c4));
    }
}
#[inline(always)]
pub fn block_0x002084c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2131144u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2131148u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2131152u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2131156u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131160u32;
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
pub fn block_0x002084d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2131164u32)?;
    emu.apc_no_count(1usize, 2131164u32, 0u32, 2131168u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131172u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1840u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002084e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2131176u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2131180u32)?;
    emu.lw_no_count(11usize, 11usize, 20u32, 2131184u32)?;
    emu.apc_no_count(1usize, 2131184u32, 0u32, 2131188u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131192u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966996u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002084f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2131192u32, 69632u32, 2131196u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131200u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(852u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208500(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2131204u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2131208u32)?;
    emu.apc_no_count(1usize, 2131208u32, 4096u32, 2131212u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131216u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966832u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208510(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2131220u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2131224u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131228u32;
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
pub fn block_0x0020851c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967088u32, 2131232u32);
    emu.sw_no_count(1usize, 2usize, 204u32, 2131236u32)?;
    emu.sw_no_count(8usize, 2usize, 200u32, 2131240u32)?;
    emu.sw_no_count(9usize, 2usize, 196u32, 2131244u32)?;
    emu.sw_no_count(18usize, 2usize, 192u32, 2131248u32)?;
    emu.sw_no_count(19usize, 2usize, 188u32, 2131252u32)?;
    emu.sw_no_count(20usize, 2usize, 184u32, 2131256u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2131260u32);
    let a = 0u32.wrapping_add(2244608u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2131264u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 10usize, 1008u32, 2131268u32)?;
    emu.sw_no_count(0usize, 10usize, 1012u32, 2131272u32)?;
    emu.ani_no_count(11usize, 11usize, 1u32, 2131276u32);
    emu.sw_no_count(0usize, 10usize, 1008u32, 2131280u32)?;
    emu.add_memory_rw_events(13usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2132596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208a74));
    } else {
        emu.pc = 2131284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208554));
    }
}
#[inline(always)]
pub fn block_0x00208554(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2244608u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2131288u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1016u32, 2131292u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2131296u32);
    emu.adi_no_count(12usize, 0usize, 112u32, 2131300u32);
    emu.apc_no_count(1usize, 2131300u32, 0u32, 2131304u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131308u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(2000u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0020856c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 35u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 2usize, 48u32, 2131312u32);
    emu.lbu_no_count(18usize, 2usize, 112u32, 2131316u32);
    emu.lw_no_count(10usize, 2usize, 40u32, 2131320u32)?;
    emu.lw_no_count(11usize, 2usize, 44u32, 2131324u32)?;
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2131328u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 0usize, 128u32, 2131332u32);
    emu.adi_no_count(12usize, 12usize, 4294967040u32, 2131336u32);
    emu.sli_no_count(14usize, 11usize, 9u32, 2131340u32);
    emu.sli_no_count(15usize, 10usize, 9u32, 2131344u32);
    emu.sli_no_count(16usize, 18usize, 3u32, 2131348u32);
    emu.sli_no_count(17usize, 10usize, 1u32, 2131352u32);
    emu.sli_no_count(11usize, 11usize, 1u32, 2131356u32);
    emu.orr_no_count(16usize, 15usize, 16usize, 2131360u32);
    emu.anr_no_count(17usize, 17usize, 12usize, 2131364u32);
    emu.sri_no_count(15usize, 15usize, 24u32, 2131368u32);
    emu.anr_no_count(11usize, 11usize, 12usize, 2131372u32);
    emu.orr_no_count(15usize, 17usize, 15usize, 2131376u32);
    emu.sri_no_count(17usize, 14usize, 24u32, 2131380u32);
    emu.orr_no_count(11usize, 11usize, 17usize, 2131384u32);
    emu.adi_no_count(17usize, 0usize, 63u32, 2131388u32);
    emu.sri_no_count(5usize, 10usize, 23u32, 2131392u32);
    emu.orr_no_count(10usize, 14usize, 5usize, 2131396u32);
    emu.anr_no_count(14usize, 16usize, 12usize, 2131400u32);
    emu.anr_no_count(12usize, 10usize, 12usize, 2131404u32);
    emu.sli_no_count(10usize, 18usize, 27u32, 2131408u32);
    emu.orr_no_count(15usize, 10usize, 15usize, 2131412u32);
    emu.adr_no_count(10usize, 9usize, 18usize, 2131416u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2131420u32);
    emu.sli_no_count(14usize, 14usize, 8u32, 2131424u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2131428u32);
    emu.orr_no_count(11usize, 5usize, 11usize, 2131432u32);
    emu.orr_no_count(20usize, 15usize, 14usize, 2131436u32);
    emu.orr_no_count(19usize, 11usize, 12usize, 2131440u32);
    emu.sb_no_count(13usize, 10usize, 0u32, 2131444u32);
    emu.add_memory_rw_events(34usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2131480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208618));
    } else {
        emu.pc = 2131448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002085f8));
    }
}
#[inline(always)]
pub fn block_0x002085f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 10usize, 1u32, 2131452u32);
    emu.xri_no_count(12usize, 18usize, 63u32, 2131456u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2131460u32);
    emu.apc_no_count(1usize, 2131460u32, 0u32, 2131464u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131468u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1592u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020860c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(10usize, 18usize, 56u32, 2131472u32);
    emu.adi_no_count(11usize, 0usize, 7u32, 2131476u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2131592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208688));
    } else {
        emu.pc = 2131480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208618));
    }
}
#[inline(always)]
pub fn block_0x00208618(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 8u32, 2131484u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2131488u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2131492u32);
    emu.apc_no_count(1usize, 2131492u32, 36864u32, 2131496u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131500u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1200u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020862c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 120u32, 2131504u32);
    emu.adi_no_count(12usize, 0usize, 56u32, 2131508u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2131512u32);
    emu.apc_no_count(1usize, 2131512u32, 0u32, 2131516u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131520u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1540u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00208640(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 20usize, 24u32, 2131524u32);
    emu.sri_no_count(11usize, 20usize, 16u32, 2131528u32);
    emu.sri_no_count(12usize, 20usize, 8u32, 2131532u32);
    emu.sri_no_count(13usize, 19usize, 24u32, 2131536u32);
    emu.sri_no_count(14usize, 19usize, 16u32, 2131540u32);
    emu.sb_no_count(20usize, 2usize, 180u32, 2131544u32);
    emu.sb_no_count(12usize, 2usize, 181u32, 2131548u32);
    emu.sb_no_count(11usize, 2usize, 182u32, 2131552u32);
    emu.sb_no_count(10usize, 2usize, 183u32, 2131556u32);
    emu.sri_no_count(10usize, 19usize, 8u32, 2131560u32);
    emu.sb_no_count(19usize, 2usize, 176u32, 2131564u32);
    emu.sb_no_count(10usize, 2usize, 177u32, 2131568u32);
    emu.sb_no_count(14usize, 2usize, 178u32, 2131572u32);
    emu.sb_no_count(13usize, 2usize, 179u32, 2131576u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2131580u32);
    emu.adi_no_count(11usize, 2usize, 120u32, 2131584u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2131588u32);
    emu.add_memory_rw_events(18usize);
    let return_addr = 2131592u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2131612u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020869c));
}
#[inline(always)]
pub fn block_0x00208688(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(19usize, 2usize, 104u32, 2131596u32)?;
    emu.sw_no_count(20usize, 2usize, 108u32, 2131600u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2131604u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2131608u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2131612u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2131612u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020869c));
}
#[inline(always)]
pub fn block_0x0020869c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2131612u32, 36864u32, 2131616u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131620u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1080u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x002086a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 85u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2131624u32);
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2131628u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 2usize, 8u32, 2131632u32)?;
    emu.lw_no_count(13usize, 2usize, 12u32, 2131636u32)?;
    emu.lw_no_count(15usize, 2usize, 16u32, 2131640u32)?;
    emu.lw_no_count(16usize, 2usize, 20u32, 2131644u32)?;
    emu.lw_no_count(17usize, 2usize, 24u32, 2131648u32)?;
    emu.lw_no_count(5usize, 2usize, 28u32, 2131652u32)?;
    emu.lw_no_count(6usize, 2usize, 32u32, 2131656u32)?;
    emu.lw_no_count(14usize, 2usize, 36u32, 2131660u32)?;
    emu.adi_no_count(12usize, 12usize, 4294967040u32, 2131664u32);
    emu.sri_no_count(7usize, 11usize, 8u32, 2131668u32);
    emu.sri_no_count(28usize, 11usize, 24u32, 2131672u32);
    emu.anr_no_count(29usize, 11usize, 12usize, 2131676u32);
    emu.sli_no_count(11usize, 11usize, 24u32, 2131680u32);
    emu.sri_no_count(30usize, 13usize, 8u32, 2131684u32);
    emu.sri_no_count(31usize, 13usize, 24u32, 2131688u32);
    emu.anr_no_count(9usize, 13usize, 12usize, 2131692u32);
    emu.sli_no_count(13usize, 13usize, 24u32, 2131696u32);
    emu.sri_no_count(18usize, 15usize, 8u32, 2131700u32);
    emu.sri_no_count(19usize, 15usize, 24u32, 2131704u32);
    emu.anr_no_count(20usize, 15usize, 12usize, 2131708u32);
    emu.sli_no_count(15usize, 15usize, 24u32, 2131712u32);
    emu.anr_no_count(7usize, 7usize, 12usize, 2131716u32);
    emu.orr_no_count(7usize, 7usize, 28usize, 2131720u32);
    emu.sri_no_count(28usize, 16usize, 8u32, 2131724u32);
    emu.sli_no_count(29usize, 29usize, 8u32, 2131728u32);
    emu.orr_no_count(11usize, 11usize, 29usize, 2131732u32);
    emu.sri_no_count(29usize, 16usize, 24u32, 2131736u32);
    emu.anr_no_count(30usize, 30usize, 12usize, 2131740u32);
    emu.orr_no_count(30usize, 30usize, 31usize, 2131744u32);
    emu.anr_no_count(31usize, 16usize, 12usize, 2131748u32);
    emu.sli_no_count(16usize, 16usize, 24u32, 2131752u32);
    emu.sli_no_count(9usize, 9usize, 8u32, 2131756u32);
    emu.orr_no_count(13usize, 13usize, 9usize, 2131760u32);
    emu.sri_no_count(9usize, 17usize, 8u32, 2131764u32);
    emu.anr_no_count(18usize, 18usize, 12usize, 2131768u32);
    emu.orr_no_count(18usize, 18usize, 19usize, 2131772u32);
    emu.sri_no_count(19usize, 17usize, 24u32, 2131776u32);
    emu.sli_no_count(20usize, 20usize, 8u32, 2131780u32);
    emu.orr_no_count(15usize, 15usize, 20usize, 2131784u32);
    emu.anr_no_count(20usize, 17usize, 12usize, 2131788u32);
    emu.sli_no_count(17usize, 17usize, 24u32, 2131792u32);
    emu.anr_no_count(28usize, 28usize, 12usize, 2131796u32);
    emu.orr_no_count(28usize, 28usize, 29usize, 2131800u32);
    emu.sri_no_count(29usize, 5usize, 8u32, 2131804u32);
    emu.sli_no_count(31usize, 31usize, 8u32, 2131808u32);
    emu.orr_no_count(16usize, 16usize, 31usize, 2131812u32);
    emu.sri_no_count(31usize, 5usize, 24u32, 2131816u32);
    emu.anr_no_count(9usize, 9usize, 12usize, 2131820u32);
    emu.orr_no_count(9usize, 9usize, 19usize, 2131824u32);
    emu.anr_no_count(19usize, 5usize, 12usize, 2131828u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2131832u32);
    emu.sli_no_count(20usize, 20usize, 8u32, 2131836u32);
    emu.orr_no_count(17usize, 17usize, 20usize, 2131840u32);
    emu.sri_no_count(20usize, 6usize, 8u32, 2131844u32);
    emu.anr_no_count(29usize, 29usize, 12usize, 2131848u32);
    emu.orr_no_count(29usize, 29usize, 31usize, 2131852u32);
    emu.sri_no_count(31usize, 6usize, 24u32, 2131856u32);
    emu.sli_no_count(19usize, 19usize, 8u32, 2131860u32);
    emu.orr_no_count(5usize, 5usize, 19usize, 2131864u32);
    emu.anr_no_count(19usize, 6usize, 12usize, 2131868u32);
    emu.sli_no_count(6usize, 6usize, 24u32, 2131872u32);
    emu.anr_no_count(20usize, 20usize, 12usize, 2131876u32);
    emu.orr_no_count(31usize, 20usize, 31usize, 2131880u32);
    emu.sri_no_count(20usize, 14usize, 8u32, 2131884u32);
    emu.sli_no_count(19usize, 19usize, 8u32, 2131888u32);
    emu.orr_no_count(6usize, 6usize, 19usize, 2131892u32);
    emu.sri_no_count(19usize, 14usize, 24u32, 2131896u32);
    emu.anr_no_count(20usize, 20usize, 12usize, 2131900u32);
    emu.orr_no_count(19usize, 20usize, 19usize, 2131904u32);
    emu.anr_no_count(12usize, 14usize, 12usize, 2131908u32);
    emu.sli_no_count(14usize, 14usize, 24u32, 2131912u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2131916u32);
    emu.orr_no_count(20usize, 14usize, 12usize, 2131920u32);
    emu.orr_no_count(11usize, 11usize, 7usize, 2131924u32);
    emu.orr_no_count(12usize, 13usize, 30usize, 2131928u32);
    emu.orr_no_count(13usize, 15usize, 18usize, 2131932u32);
    emu.orr_no_count(14usize, 16usize, 28usize, 2131936u32);
    emu.orr_no_count(15usize, 17usize, 9usize, 2131940u32);
    emu.orr_no_count(16usize, 5usize, 29usize, 2131944u32);
    emu.orr_no_count(17usize, 6usize, 31usize, 2131948u32);
    emu.adi_no_count(5usize, 0usize, 16u32, 2131952u32);
    emu.orr_no_count(6usize, 20usize, 19usize, 2131956u32);
    emu.add_memory_rw_events(85usize);
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
pub fn block_0x002087f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2131964u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2131968u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2131972u32);
    emu.add_memory_rw_events(4usize);
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
pub fn block_0x00208808(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2131980u32);
    emu.adi_no_count(10usize, 0usize, 2u32, 2131984u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2131988u32);
    emu.add_memory_rw_events(4usize);
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
pub fn block_0x00208818(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2131996u32);
    emu.adi_no_count(10usize, 0usize, 3u32, 2132000u32);
    emu.adi_no_count(11usize, 14usize, 0u32, 2132004u32);
    emu.add_memory_rw_events(4usize);
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
pub fn block_0x00208828(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2132012u32);
    emu.adi_no_count(10usize, 0usize, 4u32, 2132016u32);
    emu.adi_no_count(11usize, 15usize, 0u32, 2132020u32);
    emu.add_memory_rw_events(4usize);
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
pub fn block_0x00208838(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2132028u32);
    emu.adi_no_count(10usize, 0usize, 5u32, 2132032u32);
    emu.adi_no_count(11usize, 16usize, 0u32, 2132036u32);
    emu.add_memory_rw_events(4usize);
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
pub fn block_0x00208848(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2132044u32);
    emu.adi_no_count(10usize, 0usize, 6u32, 2132048u32);
    emu.adi_no_count(11usize, 17usize, 0u32, 2132052u32);
    emu.add_memory_rw_events(4usize);
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
pub fn block_0x00208858(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2132060u32);
    emu.adi_no_count(10usize, 0usize, 7u32, 2132064u32);
    emu.adi_no_count(11usize, 6usize, 0u32, 2132068u32);
    emu.add_memory_rw_events(4usize);
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
pub fn block_0x00208868(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2244608u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2132076u32;
    emu.update_insn_clock();
    emu.lw_no_count(10usize, 10usize, 968u32, 2132080u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2132612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208a84));
    } else {
        emu.pc = 2132084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208874));
    }
}
#[inline]
pub fn block_0x00208874(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2132088u32);
    let a = 0u32.wrapping_add(2244608u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2132092u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 968u32, 2132096u32);
    let a = 0u32.wrapping_add(2164260864u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2132100u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 14usize, 4u32, 2132104u32)?;
    let a = 0u32.wrapping_add(2130706432u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2132108u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1u32, 2132112u32);
    emu.adi_no_count(12usize, 12usize, 1u32, 2132116u32);
    emu.mul_no_count(15usize, 11usize, 13usize, 2132120u32);
    emu.mulhu_no_count(16usize, 15usize, 12usize, 2132124u32);
    emu.mul_no_count(15usize, 15usize, 12usize, 2132128u32);
    emu.sltru_no_count(11usize, 11usize, 15usize, 2132132u32);
    emu.sltru_no_count(15usize, 0usize, 16usize, 2132136u32);
    emu.adr_no_count(16usize, 16usize, 11usize, 2132140u32);
    emu.orr_no_count(11usize, 11usize, 15usize, 2132144u32);
    emu.sbr_no_count(11usize, 0usize, 11usize, 2132148u32);
    emu.anr_no_count(11usize, 11usize, 12usize, 2132152u32);
    emu.sbr_no_count(11usize, 11usize, 16usize, 2132156u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2132160u32);
    emu.add_memory_rw_events(20usize);
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
#[inline]
pub fn block_0x002088c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 8u32, 2132168u32)?;
    emu.mul_no_count(11usize, 10usize, 13usize, 2132172u32);
    emu.mulhu_no_count(15usize, 11usize, 12usize, 2132176u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2132180u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2132184u32);
    emu.sltru_no_count(11usize, 0usize, 15usize, 2132188u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2132192u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2132196u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2132200u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2132204u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2132208u32);
    emu.sbr_no_count(11usize, 10usize, 15usize, 2132212u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2132216u32);
    emu.add_memory_rw_events(14usize);
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
#[inline]
pub fn block_0x002088fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 12u32, 2132224u32)?;
    emu.mul_no_count(11usize, 10usize, 13usize, 2132228u32);
    emu.mulhu_no_count(15usize, 11usize, 12usize, 2132232u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2132236u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2132240u32);
    emu.sltru_no_count(11usize, 0usize, 15usize, 2132244u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2132248u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2132252u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2132256u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2132260u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2132264u32);
    emu.sbr_no_count(11usize, 10usize, 15usize, 2132268u32);
    emu.adi_no_count(10usize, 0usize, 2u32, 2132272u32);
    emu.add_memory_rw_events(14usize);
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
#[inline]
pub fn block_0x00208934(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 16u32, 2132280u32)?;
    emu.mul_no_count(11usize, 10usize, 13usize, 2132284u32);
    emu.mulhu_no_count(15usize, 11usize, 12usize, 2132288u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2132292u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2132296u32);
    emu.sltru_no_count(11usize, 0usize, 15usize, 2132300u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2132304u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2132308u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2132312u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2132316u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2132320u32);
    emu.sbr_no_count(11usize, 10usize, 15usize, 2132324u32);
    emu.adi_no_count(10usize, 0usize, 3u32, 2132328u32);
    emu.add_memory_rw_events(14usize);
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
#[inline]
pub fn block_0x0020896c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 20u32, 2132336u32)?;
    emu.mul_no_count(11usize, 10usize, 13usize, 2132340u32);
    emu.mulhu_no_count(15usize, 11usize, 12usize, 2132344u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2132348u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2132352u32);
    emu.sltru_no_count(11usize, 0usize, 15usize, 2132356u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2132360u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2132364u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2132368u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2132372u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2132376u32);
    emu.sbr_no_count(11usize, 10usize, 15usize, 2132380u32);
    emu.adi_no_count(10usize, 0usize, 4u32, 2132384u32);
    emu.add_memory_rw_events(14usize);
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
#[inline]
pub fn block_0x002089a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 24u32, 2132392u32)?;
    emu.mul_no_count(11usize, 10usize, 13usize, 2132396u32);
    emu.mulhu_no_count(15usize, 11usize, 12usize, 2132400u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2132404u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2132408u32);
    emu.sltru_no_count(11usize, 0usize, 15usize, 2132412u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2132416u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2132420u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2132424u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2132428u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2132432u32);
    emu.sbr_no_count(11usize, 10usize, 15usize, 2132436u32);
    emu.adi_no_count(10usize, 0usize, 5u32, 2132440u32);
    emu.add_memory_rw_events(14usize);
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
#[inline]
pub fn block_0x002089dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 28u32, 2132448u32)?;
    emu.mul_no_count(11usize, 10usize, 13usize, 2132452u32);
    emu.mulhu_no_count(15usize, 11usize, 12usize, 2132456u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2132460u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2132464u32);
    emu.sltru_no_count(11usize, 0usize, 15usize, 2132468u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2132472u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2132476u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2132480u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2132484u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2132488u32);
    emu.sbr_no_count(11usize, 10usize, 15usize, 2132492u32);
    emu.adi_no_count(10usize, 0usize, 6u32, 2132496u32);
    emu.add_memory_rw_events(14usize);
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
#[inline]
pub fn block_0x00208a14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 32u32, 2132504u32)?;
    emu.adi_no_count(5usize, 0usize, 26u32, 2132508u32);
    emu.mul_no_count(11usize, 10usize, 13usize, 2132512u32);
    emu.mulhu_no_count(13usize, 11usize, 12usize, 2132516u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2132520u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2132524u32);
    emu.sltru_no_count(11usize, 0usize, 13usize, 2132528u32);
    emu.adr_no_count(13usize, 13usize, 10usize, 2132532u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2132536u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2132540u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2132544u32);
    emu.sbr_no_count(11usize, 10usize, 13usize, 2132548u32);
    emu.adi_no_count(10usize, 0usize, 7u32, 2132552u32);
    emu.add_memory_rw_events(14usize);
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
pub fn block_0x00208a4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 0u32, 2132560u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2132564u32);
    emu.add_memory_rw_events(3usize);
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
pub fn block_0x00208a58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2132572u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1452u32, 2132576u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2132580u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1524u32, 2132584u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2132588u32);
    emu.apc_no_count(1usize, 2132588u32, 77824u32, 2132592u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2132596u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965964u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208a74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2132600u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1492u32, 2132604u32);
    emu.apc_no_count(1usize, 2132604u32, 81920u32, 2132608u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2132612u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966424u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208a84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2132616u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1508u32, 2132620u32);
    emu.apc_no_count(1usize, 2132620u32, 81920u32, 2132624u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2132628u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966408u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00208a94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2132632u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2132636u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2132640u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2132644u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2132648u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2132652u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2132656u32)?;
    emu.adi_no_count(8usize, 12usize, 0u32, 2132660u32);
    emu.adi_no_count(5usize, 0usize, 2u32, 2132664u32);
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
pub fn block_0x00208abc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 3u32, 2132672u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2132908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208bac));
    } else {
        emu.pc = 2132676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208ac4));
    }
}
#[inline(always)]
pub fn block_0x00208ac4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2244608u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2132680u32;
    emu.update_insn_clock();
    emu.lw_no_count(10usize, 10usize, 1008u32, 2132684u32)?;
    emu.ani_no_count(10usize, 10usize, 1u32, 2132688u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2132940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208bcc));
    } else {
        emu.pc = 2132692u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208ad4));
    }
}
#[inline(always)]
pub fn block_0x00208ad4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2244608u32);
    emu.write_reg_no_count(18usize, a);
    emu.pc = 2132696u32;
    emu.update_insn_clock();
    emu.adi_no_count(18usize, 18usize, 1008u32, 2132700u32);
    emu.lbu_no_count(19usize, 18usize, 112u32, 2132704u32);
    emu.adi_no_count(10usize, 0usize, 64u32, 2132708u32);
    emu.sbr_no_count(12usize, 10usize, 19usize, 2132712u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a >= b {
        emu.pc = 2132744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208b08));
    } else {
        emu.pc = 2132716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208aec));
    }
}
#[inline(always)]
pub fn block_0x00208aec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 18usize, 19usize, 2132720u32);
    emu.adi_no_count(10usize, 10usize, 48u32, 2132724u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2132728u32);
    emu.apc_no_count(1usize, 2132728u32, 0u32, 2132732u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2132736u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(572u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208b00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(9usize, 8usize, 19usize, 2132740u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2132744u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2132904u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208ba8));
}
#[inline(always)]
pub fn block_0x00208b08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2132824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208b58));
    } else {
        emu.pc = 2132748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208b0c));
    }
}
#[inline(always)]
pub fn block_0x00208b0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(8usize, 8usize, 12usize, 2132752u32);
    emu.adr_no_count(20usize, 11usize, 12usize, 2132756u32);
    emu.adi_no_count(9usize, 18usize, 48u32, 2132760u32);
    emu.adr_no_count(10usize, 9usize, 19usize, 2132764u32);
    emu.apc_no_count(1usize, 2132764u32, 0u32, 2132768u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2132772u32;
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
#[inline]
pub fn block_0x00208b24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 40u32, 2132776u32)?;
    emu.lw_no_count(11usize, 18usize, 44u32, 2132780u32)?;
    emu.adi_no_count(10usize, 10usize, 1u32, 2132784u32);
    emu.sltiu_no_count(12usize, 10usize, 1u32, 2132788u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2132792u32);
    emu.sw_no_count(10usize, 18usize, 40u32, 2132796u32)?;
    emu.sw_no_count(11usize, 18usize, 44u32, 2132800u32)?;
    emu.adi_no_count(10usize, 18usize, 8u32, 2132804u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2132808u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2132812u32);
    emu.apc_no_count(1usize, 2132812u32, 36864u32, 2132816u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2132820u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967176u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208b54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 20usize, 0u32, 2132824u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2132824u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208b58));
}
#[inline(always)]
pub fn block_0x00208b58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 8usize, 4294967232u32, 2132828u32);
    emu.ani_no_count(9usize, 8usize, 63u32, 2132832u32);
    emu.sri_no_count(12usize, 8usize, 6u32, 2132836u32);
    emu.adr_no_count(8usize, 11usize, 10usize, 2132840u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2132884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208b94));
    } else {
        emu.pc = 2132844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208b6c));
    }
}
#[inline]
pub fn block_0x00208b6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 40u32, 2132848u32)?;
    emu.lw_no_count(13usize, 18usize, 44u32, 2132852u32)?;
    emu.adr_no_count(14usize, 10usize, 12usize, 2132856u32);
    emu.sltru_no_count(10usize, 14usize, 10usize, 2132860u32);
    emu.adr_no_count(10usize, 13usize, 10usize, 2132864u32);
    emu.sw_no_count(14usize, 18usize, 40u32, 2132868u32)?;
    emu.sw_no_count(10usize, 18usize, 44u32, 2132872u32)?;
    emu.adi_no_count(10usize, 18usize, 8u32, 2132876u32);
    emu.apc_no_count(1usize, 2132876u32, 36864u32, 2132880u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2132884u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967112u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208b94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 48u32, 2132888u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2132892u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2132896u32);
    emu.apc_no_count(1usize, 2132896u32, 0u32, 2132900u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2132904u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(404u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208ba8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(9usize, 18usize, 112u32, 2132908u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2132908u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208bac));
}
#[inline(always)]
pub fn block_0x00208bac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2132912u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2132916u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2132920u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2132924u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2132928u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2132932u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2132936u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2132940u32;
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
pub fn block_0x00208bcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2132944u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1540u32, 2132948u32);
    emu.apc_no_count(1usize, 2132948u32, 81920u32, 2132952u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2132956u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966080u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208bdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 240u32, 2132960u32);
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
pub fn block_0x00208be4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 5usize, 0u32, 2132968u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2132972u32;
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
pub fn block_0x00208bec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 241u32, 2132976u32);
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
pub fn block_0x00208bf4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2132984u32;
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
pub fn block_0x00208bf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(3145728u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2132988u32;
    emu.update_insn_clock();
    emu.adi_no_count(5usize, 11usize, 261u32, 2132992u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2132996u32);
    emu.add_memory_rw_events(4usize);
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
pub fn block_0x00208c08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2133004u32;
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
pub fn block_0x00208c0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2133008u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2133012u32)?;
    emu.adi_no_count(12usize, 11usize, 0u32, 2133016u32);
    emu.adi_no_count(11usize, 10usize, 0u32, 2133020u32);
    emu.adi_no_count(10usize, 0usize, 2u32, 2133024u32);
    emu.apc_no_count(1usize, 2133024u32, 0u32, 2133028u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2133032u32;
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
pub fn block_0x00208c28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2133036u32);
    emu.apc_no_count(1usize, 2133036u32, 0u32, 2133040u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2133044u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965488u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208c34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2133044u32, 0u32, 2133048u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2133052u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966880u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208c3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2133296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208d30));
    } else {
        emu.pc = 2133056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208c40));
    }
}
#[inline(always)]
pub fn block_0x00208c40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 10usize, 0u32, 2133060u32);
    emu.adr_no_count(13usize, 12usize, 10usize, 2133064u32);
    emu.adi_no_count(14usize, 0usize, 3u32, 2133068u32);
    emu.sb_no_count(11usize, 13usize, 4294967295u32, 2133072u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2133296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208d30));
    } else {
        emu.pc = 2133076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208c54));
    }
}
#[inline(always)]
pub fn block_0x00208c54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 10usize, 1u32, 2133080u32);
    emu.sb_no_count(11usize, 10usize, 2u32, 2133084u32);
    emu.sb_no_count(11usize, 13usize, 4294967294u32, 2133088u32);
    emu.adi_no_count(14usize, 0usize, 7u32, 2133092u32);
    emu.sb_no_count(11usize, 13usize, 4294967293u32, 2133096u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2133296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208d30));
    } else {
        emu.pc = 2133100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208c6c));
    }
}
#[inline(always)]
pub fn block_0x00208c6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 10usize, 3u32, 2133104u32);
    emu.adi_no_count(15usize, 0usize, 9u32, 2133108u32);
    emu.sb_no_count(11usize, 13usize, 4294967292u32, 2133112u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2133296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208d30));
    } else {
        emu.pc = 2133116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208c7c));
    }
}
#[inline]
pub fn block_0x00208c7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(13usize, 0usize, 10usize, 2133120u32);
    emu.ani_no_count(14usize, 13usize, 3u32, 2133124u32);
    emu.adr_no_count(13usize, 10usize, 14usize, 2133128u32);
    emu.sbr_no_count(12usize, 12usize, 14usize, 2133132u32);
    emu.ani_no_count(12usize, 12usize, 4294967292u32, 2133136u32);
    emu.ani_no_count(11usize, 11usize, 255u32, 2133140u32);
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2133144u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 257u32, 2133148u32);
    emu.mul_no_count(11usize, 11usize, 14usize, 2133152u32);
    emu.sw_no_count(11usize, 13usize, 0u32, 2133156u32)?;
    emu.adr_no_count(14usize, 13usize, 12usize, 2133160u32);
    emu.sw_no_count(11usize, 14usize, 4294967292u32, 2133164u32)?;
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2133296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208d30));
    } else {
        emu.pc = 2133168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208cb0));
    }
}
#[inline(always)]
pub fn block_0x00208cb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 13usize, 4u32, 2133172u32)?;
    emu.sw_no_count(11usize, 13usize, 8u32, 2133176u32)?;
    emu.sw_no_count(11usize, 14usize, 4294967284u32, 2133180u32)?;
    emu.adi_no_count(15usize, 0usize, 25u32, 2133184u32);
    emu.sw_no_count(11usize, 14usize, 4294967288u32, 2133188u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2133296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208d30));
    } else {
        emu.pc = 2133192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208cc8));
    }
}
#[inline]
pub fn block_0x00208cc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 13usize, 12u32, 2133196u32)?;
    emu.sw_no_count(11usize, 13usize, 16u32, 2133200u32)?;
    emu.sw_no_count(11usize, 13usize, 20u32, 2133204u32)?;
    emu.sw_no_count(11usize, 13usize, 24u32, 2133208u32)?;
    emu.sw_no_count(11usize, 14usize, 4294967268u32, 2133212u32)?;
    emu.sw_no_count(11usize, 14usize, 4294967272u32, 2133216u32)?;
    emu.sw_no_count(11usize, 14usize, 4294967276u32, 2133220u32)?;
    emu.ani_no_count(15usize, 13usize, 4u32, 2133224u32);
    emu.ori_no_count(15usize, 15usize, 24u32, 2133228u32);
    emu.sbr_no_count(12usize, 12usize, 15usize, 2133232u32);
    emu.adi_no_count(16usize, 0usize, 32u32, 2133236u32);
    emu.sw_no_count(11usize, 14usize, 4294967280u32, 2133240u32)?;
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2133296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208d30));
    } else {
        emu.pc = 2133244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208cfc));
    }
}
#[inline(always)]
pub fn block_0x00208cfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(13usize, 13usize, 15usize, 2133248u32);
    emu.adi_no_count(14usize, 0usize, 31u32, 2133252u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2133252u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208d04));
}
#[inline]
pub fn block_0x00208d04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 13usize, 0u32, 2133256u32)?;
    emu.sw_no_count(11usize, 13usize, 4u32, 2133260u32)?;
    emu.sw_no_count(11usize, 13usize, 8u32, 2133264u32)?;
    emu.sw_no_count(11usize, 13usize, 12u32, 2133268u32)?;
    emu.sw_no_count(11usize, 13usize, 16u32, 2133272u32)?;
    emu.sw_no_count(11usize, 13usize, 20u32, 2133276u32)?;
    emu.sw_no_count(11usize, 13usize, 24u32, 2133280u32)?;
    emu.sw_no_count(11usize, 13usize, 28u32, 2133284u32)?;
    emu.adi_no_count(12usize, 12usize, 4294967264u32, 2133288u32);
    emu.adi_no_count(13usize, 13usize, 32u32, 2133292u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2133252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208d04));
    } else {
        emu.pc = 2133296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208d30));
    }
}
#[inline(always)]
pub fn block_0x00208d30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2133300u32;
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
pub fn block_0x00208d34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(13usize, 11usize, 3u32, 2133304u32);
    emu.sltiu_no_count(13usize, 13usize, 1u32, 2133308u32);
    emu.sltiu_no_count(14usize, 12usize, 1u32, 2133312u32);
    emu.orr_no_count(13usize, 13usize, 14usize, 2133316u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2133564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208e3c));
    } else {
        emu.pc = 2133320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208d48));
    }
}
#[inline(always)]
pub fn block_0x00208d48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 11usize, 1u32, 2133324u32);
    emu.adi_no_count(16usize, 10usize, 0u32, 2133328u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2133328u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208d50));
}
#[inline]
pub fn block_0x00208d50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(17usize, 11usize, 0u32, 2133332u32);
    emu.adi_no_count(14usize, 11usize, 1u32, 2133336u32);
    emu.adi_no_count(13usize, 16usize, 1u32, 2133340u32);
    emu.sb_no_count(17usize, 16usize, 0u32, 2133344u32);
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2133348u32);
    emu.ani_no_count(11usize, 15usize, 3u32, 2133352u32);
    emu.sltru_no_count(11usize, 0usize, 11usize, 2133356u32);
    emu.sltru_no_count(16usize, 0usize, 12usize, 2133360u32);
    emu.anr_no_count(17usize, 11usize, 16usize, 2133364u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2133368u32);
    emu.adi_no_count(11usize, 14usize, 0u32, 2133372u32);
    emu.adi_no_count(16usize, 13usize, 0u32, 2133376u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a != b {
        emu.pc = 2133328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208d50));
    } else {
        emu.pc = 2133380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208d84));
    }
}
#[inline(always)]
pub fn block_0x00208d84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 13usize, 3u32, 2133384u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2133580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208e4c));
    } else {
        emu.pc = 2133388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208d8c));
    }
}
#[inline(always)]
pub fn block_0x00208d8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 32u32, 2133392u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2133964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208fcc));
    } else {
        emu.pc = 2133396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208d94));
    }
}
#[inline(always)]
pub fn block_0x00208d94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 3u32, 2133400u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2133700u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208ec4));
    } else {
        emu.pc = 2133404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208d9c));
    }
}
#[inline(always)]
pub fn block_0x00208d9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 2u32, 2133408u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2133828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208f44));
    } else {
        emu.pc = 2133412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208da4));
    }
}
#[inline(always)]
pub fn block_0x00208da4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 1u32, 2133416u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2133964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208fcc));
    } else {
        emu.pc = 2133420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208dac));
    }
}
#[inline]
pub fn block_0x00208dac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 14usize, 0u32, 2133424u32)?;
    emu.sb_no_count(15usize, 13usize, 0u32, 2133428u32);
    emu.sri_no_count(11usize, 15usize, 8u32, 2133432u32);
    emu.sb_no_count(11usize, 13usize, 1u32, 2133436u32);
    emu.sri_no_count(16usize, 15usize, 16u32, 2133440u32);
    emu.adi_no_count(11usize, 13usize, 3u32, 2133444u32);
    emu.sb_no_count(16usize, 13usize, 2u32, 2133448u32);
    emu.adi_no_count(12usize, 12usize, 4294967293u32, 2133452u32);
    emu.adi_no_count(13usize, 14usize, 16u32, 2133456u32);
    emu.adi_no_count(14usize, 0usize, 16u32, 2133460u32);
    emu.add_memory_rw_events(10usize);
    emu.pc = 2133460u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208dd4));
}
#[inline]
pub fn block_0x00208dd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 13usize, 4294967284u32, 2133464u32)?;
    emu.sri_no_count(15usize, 15usize, 24u32, 2133468u32);
    emu.sli_no_count(17usize, 16usize, 8u32, 2133472u32);
    emu.lw_no_count(5usize, 13usize, 4294967288u32, 2133476u32)?;
    emu.orr_no_count(15usize, 17usize, 15usize, 2133480u32);
    emu.sw_no_count(15usize, 11usize, 0u32, 2133484u32)?;
    emu.sri_no_count(15usize, 16usize, 24u32, 2133488u32);
    emu.sli_no_count(16usize, 5usize, 8u32, 2133492u32);
    emu.lw_no_count(17usize, 13usize, 4294967292u32, 2133496u32)?;
    emu.orr_no_count(15usize, 16usize, 15usize, 2133500u32);
    emu.sw_no_count(15usize, 11usize, 4u32, 2133504u32)?;
    emu.sri_no_count(16usize, 5usize, 24u32, 2133508u32);
    emu.sli_no_count(5usize, 17usize, 8u32, 2133512u32);
    emu.lw_no_count(15usize, 13usize, 0u32, 2133516u32)?;
    emu.orr_no_count(16usize, 5usize, 16usize, 2133520u32);
    emu.sw_no_count(16usize, 11usize, 8u32, 2133524u32)?;
    emu.sri_no_count(16usize, 17usize, 24u32, 2133528u32);
    emu.sli_no_count(17usize, 15usize, 8u32, 2133532u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2133536u32);
    emu.sw_no_count(16usize, 11usize, 12u32, 2133540u32)?;
    emu.adi_no_count(11usize, 11usize, 16u32, 2133544u32);
    emu.adi_no_count(12usize, 12usize, 4294967280u32, 2133548u32);
    emu.adi_no_count(13usize, 13usize, 16u32, 2133552u32);
    emu.add_memory_rw_events(23usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2133460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208dd4));
    } else {
        emu.pc = 2133556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208e34));
    }
}
#[inline(always)]
pub fn block_0x00208e34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 13usize, 4294967283u32, 2133560u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2133564u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2133960u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208fc8));
}
#[inline(always)]
pub fn block_0x00208e3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 10usize, 0u32, 2133568u32);
    emu.adi_no_count(14usize, 11usize, 0u32, 2133572u32);
    emu.ani_no_count(11usize, 13usize, 3u32, 2133576u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2133388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208d8c));
    } else {
        emu.pc = 2133580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208e4c));
    }
}
#[inline(always)]
pub fn block_0x00208e4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 16u32, 2133584u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2133640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208e88));
    } else {
        emu.pc = 2133588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208e54));
    }
}
#[inline(always)]
pub fn block_0x00208e54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 15u32, 2133592u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2133592u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208e58));
}
#[inline]
pub fn block_0x00208e58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 14usize, 0u32, 2133596u32)?;
    emu.lw_no_count(16usize, 14usize, 4u32, 2133600u32)?;
    emu.lw_no_count(17usize, 14usize, 8u32, 2133604u32)?;
    emu.lw_no_count(5usize, 14usize, 12u32, 2133608u32)?;
    emu.sw_no_count(15usize, 13usize, 0u32, 2133612u32)?;
    emu.sw_no_count(16usize, 13usize, 4u32, 2133616u32)?;
    emu.sw_no_count(17usize, 13usize, 8u32, 2133620u32)?;
    emu.sw_no_count(5usize, 13usize, 12u32, 2133624u32)?;
    emu.adi_no_count(14usize, 14usize, 16u32, 2133628u32);
    emu.adi_no_count(12usize, 12usize, 4294967280u32, 2133632u32);
    emu.adi_no_count(13usize, 13usize, 16u32, 2133636u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2133592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208e58));
    } else {
        emu.pc = 2133640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208e88));
    }
}
#[inline(always)]
pub fn block_0x00208e88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 12usize, 8u32, 2133644u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2133672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208ea8));
    } else {
        emu.pc = 2133648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208e90));
    }
}
#[inline(always)]
pub fn block_0x00208e90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 14usize, 0u32, 2133652u32)?;
    emu.lw_no_count(15usize, 14usize, 4u32, 2133656u32)?;
    emu.sw_no_count(11usize, 13usize, 0u32, 2133660u32)?;
    emu.sw_no_count(15usize, 13usize, 4u32, 2133664u32)?;
    emu.adi_no_count(13usize, 13usize, 8u32, 2133668u32);
    emu.adi_no_count(14usize, 14usize, 8u32, 2133672u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2133672u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208ea8));
}
#[inline(always)]
pub fn block_0x00208ea8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 12usize, 4u32, 2133676u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2134032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209010));
    } else {
        emu.pc = 2133680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208eb0));
    }
}
#[inline(always)]
pub fn block_0x00208eb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 14usize, 0u32, 2133684u32)?;
    emu.sw_no_count(11usize, 13usize, 0u32, 2133688u32)?;
    emu.adi_no_count(13usize, 13usize, 4u32, 2133692u32);
    emu.adi_no_count(14usize, 14usize, 4u32, 2133696u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2133700u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2134032u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209010));
}
#[inline(always)]
pub fn block_0x00208ec4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 14usize, 0u32, 2133704u32)?;
    emu.adi_no_count(11usize, 13usize, 1u32, 2133708u32);
    emu.sb_no_count(15usize, 13usize, 0u32, 2133712u32);
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2133716u32);
    emu.adi_no_count(13usize, 14usize, 16u32, 2133720u32);
    emu.adi_no_count(14usize, 0usize, 18u32, 2133724u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2133724u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208edc));
}
#[inline]
pub fn block_0x00208edc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 13usize, 4294967284u32, 2133728u32)?;
    emu.sri_no_count(15usize, 15usize, 8u32, 2133732u32);
    emu.sli_no_count(17usize, 16usize, 24u32, 2133736u32);
    emu.lw_no_count(5usize, 13usize, 4294967288u32, 2133740u32)?;
    emu.orr_no_count(15usize, 17usize, 15usize, 2133744u32);
    emu.sw_no_count(15usize, 11usize, 0u32, 2133748u32)?;
    emu.sri_no_count(15usize, 16usize, 8u32, 2133752u32);
    emu.sli_no_count(16usize, 5usize, 24u32, 2133756u32);
    emu.lw_no_count(17usize, 13usize, 4294967292u32, 2133760u32)?;
    emu.orr_no_count(15usize, 16usize, 15usize, 2133764u32);
    emu.sw_no_count(15usize, 11usize, 4u32, 2133768u32)?;
    emu.sri_no_count(16usize, 5usize, 8u32, 2133772u32);
    emu.sli_no_count(5usize, 17usize, 24u32, 2133776u32);
    emu.lw_no_count(15usize, 13usize, 0u32, 2133780u32)?;
    emu.orr_no_count(16usize, 5usize, 16usize, 2133784u32);
    emu.sw_no_count(16usize, 11usize, 8u32, 2133788u32)?;
    emu.sri_no_count(16usize, 17usize, 8u32, 2133792u32);
    emu.sli_no_count(17usize, 15usize, 24u32, 2133796u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2133800u32);
    emu.sw_no_count(16usize, 11usize, 12u32, 2133804u32)?;
    emu.adi_no_count(11usize, 11usize, 16u32, 2133808u32);
    emu.adi_no_count(12usize, 12usize, 4294967280u32, 2133812u32);
    emu.adi_no_count(13usize, 13usize, 16u32, 2133816u32);
    emu.add_memory_rw_events(23usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2133724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208edc));
    } else {
        emu.pc = 2133820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208f3c));
    }
}
#[inline(always)]
pub fn block_0x00208f3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 13usize, 4294967281u32, 2133824u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2133828u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2133960u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208fc8));
}
#[inline(always)]
pub fn block_0x00208f44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 14usize, 0u32, 2133832u32)?;
    emu.sb_no_count(15usize, 13usize, 0u32, 2133836u32);
    emu.sri_no_count(16usize, 15usize, 8u32, 2133840u32);
    emu.adi_no_count(11usize, 13usize, 2u32, 2133844u32);
    emu.sb_no_count(16usize, 13usize, 1u32, 2133848u32);
    emu.adi_no_count(12usize, 12usize, 4294967294u32, 2133852u32);
    emu.adi_no_count(13usize, 14usize, 16u32, 2133856u32);
    emu.adi_no_count(14usize, 0usize, 17u32, 2133860u32);
    emu.add_memory_rw_events(8usize);
    emu.pc = 2133860u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208f64));
}
#[inline]
pub fn block_0x00208f64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 13usize, 4294967284u32, 2133864u32)?;
    emu.sri_no_count(15usize, 15usize, 16u32, 2133868u32);
    emu.sli_no_count(17usize, 16usize, 16u32, 2133872u32);
    emu.lw_no_count(5usize, 13usize, 4294967288u32, 2133876u32)?;
    emu.orr_no_count(15usize, 17usize, 15usize, 2133880u32);
    emu.sw_no_count(15usize, 11usize, 0u32, 2133884u32)?;
    emu.sri_no_count(15usize, 16usize, 16u32, 2133888u32);
    emu.sli_no_count(16usize, 5usize, 16u32, 2133892u32);
    emu.lw_no_count(17usize, 13usize, 4294967292u32, 2133896u32)?;
    emu.orr_no_count(15usize, 16usize, 15usize, 2133900u32);
    emu.sw_no_count(15usize, 11usize, 4u32, 2133904u32)?;
    emu.sri_no_count(16usize, 5usize, 16u32, 2133908u32);
    emu.sli_no_count(5usize, 17usize, 16u32, 2133912u32);
    emu.lw_no_count(15usize, 13usize, 0u32, 2133916u32)?;
    emu.orr_no_count(16usize, 5usize, 16usize, 2133920u32);
    emu.sw_no_count(16usize, 11usize, 8u32, 2133924u32)?;
    emu.sri_no_count(16usize, 17usize, 16u32, 2133928u32);
    emu.sli_no_count(17usize, 15usize, 16u32, 2133932u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2133936u32);
    emu.sw_no_count(16usize, 11usize, 12u32, 2133940u32)?;
    emu.adi_no_count(11usize, 11usize, 16u32, 2133944u32);
    emu.adi_no_count(12usize, 12usize, 4294967280u32, 2133948u32);
    emu.adi_no_count(13usize, 13usize, 16u32, 2133952u32);
    emu.add_memory_rw_events(23usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2133860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208f64));
    } else {
        emu.pc = 2133956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208fc4));
    }
}
#[inline(always)]
pub fn block_0x00208fc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 13usize, 4294967282u32, 2133960u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2133960u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208fc8));
}
#[inline(always)]
pub fn block_0x00208fc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 11usize, 0u32, 2133964u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2133964u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208fcc));
}
#[inline(always)]
pub fn block_0x00208fcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 12usize, 16u32, 2133968u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2134100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209054));
    } else {
        emu.pc = 2133972u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208fd4));
    }
}
#[inline(always)]
pub fn block_0x00208fd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 12usize, 8u32, 2133976u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2134248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002090e8));
    } else {
        emu.pc = 2133980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208fdc));
    }
}
#[inline(always)]
pub fn block_0x00208fdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 12usize, 4u32, 2133984u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2134032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209010));
    } else {
        emu.pc = 2133988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208fe4));
    }
}
#[inline]
pub fn block_0x00208fe4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(11usize, 14usize, 0u32, 2133992u32);
    emu.lb_no_count(15usize, 14usize, 1u32, 2133996u32);
    emu.lb_no_count(16usize, 14usize, 2u32, 2134000u32);
    emu.sb_no_count(11usize, 13usize, 0u32, 2134004u32);
    emu.sb_no_count(15usize, 13usize, 1u32, 2134008u32);
    emu.lb_no_count(11usize, 14usize, 3u32, 2134012u32);
    emu.sb_no_count(16usize, 13usize, 2u32, 2134016u32);
    emu.adi_no_count(14usize, 14usize, 4u32, 2134020u32);
    emu.adi_no_count(15usize, 13usize, 4u32, 2134024u32);
    emu.sb_no_count(11usize, 13usize, 3u32, 2134028u32);
    emu.adi_no_count(13usize, 15usize, 0u32, 2134032u32);
    emu.add_memory_rw_events(11usize);
    emu.pc = 2134032u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209010));
}
#[inline(always)]
pub fn block_0x00209010(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 12usize, 2u32, 2134036u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2134052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209024));
    } else {
        emu.pc = 2134040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209018));
    }
}
#[inline(always)]
pub fn block_0x00209018(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 12usize, 1u32, 2134044u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2134088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209048));
    } else {
        emu.pc = 2134048u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209020));
    }
}
#[inline(always)]
pub fn block_0x00209020(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134052u32;
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
pub fn block_0x00209024(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(11usize, 14usize, 0u32, 2134056u32);
    emu.lb_no_count(15usize, 14usize, 1u32, 2134060u32);
    emu.sb_no_count(11usize, 13usize, 0u32, 2134064u32);
    emu.adi_no_count(14usize, 14usize, 2u32, 2134068u32);
    emu.adi_no_count(11usize, 13usize, 2u32, 2134072u32);
    emu.sb_no_count(15usize, 13usize, 1u32, 2134076u32);
    emu.adi_no_count(13usize, 11usize, 0u32, 2134080u32);
    emu.ani_no_count(11usize, 12usize, 1u32, 2134084u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2134048u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209020));
    } else {
        emu.pc = 2134088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209048));
    }
}
#[inline(always)]
pub fn block_0x00209048(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(11usize, 14usize, 0u32, 2134092u32);
    emu.sb_no_count(11usize, 13usize, 0u32, 2134096u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134100u32;
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
pub fn block_0x00209054(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(11usize, 14usize, 0u32, 2134104u32);
    emu.lb_no_count(15usize, 14usize, 1u32, 2134108u32);
    emu.lb_no_count(16usize, 14usize, 2u32, 2134112u32);
    emu.sb_no_count(11usize, 13usize, 0u32, 2134116u32);
    emu.sb_no_count(15usize, 13usize, 1u32, 2134120u32);
    emu.lb_no_count(11usize, 14usize, 3u32, 2134124u32);
    emu.sb_no_count(16usize, 13usize, 2u32, 2134128u32);
    emu.lb_no_count(15usize, 14usize, 4u32, 2134132u32);
    emu.lb_no_count(16usize, 14usize, 5u32, 2134136u32);
    emu.sb_no_count(11usize, 13usize, 3u32, 2134140u32);
    emu.lb_no_count(11usize, 14usize, 6u32, 2134144u32);
    emu.sb_no_count(15usize, 13usize, 4u32, 2134148u32);
    emu.sb_no_count(16usize, 13usize, 5u32, 2134152u32);
    emu.lb_no_count(15usize, 14usize, 7u32, 2134156u32);
    emu.sb_no_count(11usize, 13usize, 6u32, 2134160u32);
    emu.lb_no_count(11usize, 14usize, 8u32, 2134164u32);
    emu.lb_no_count(16usize, 14usize, 9u32, 2134168u32);
    emu.sb_no_count(15usize, 13usize, 7u32, 2134172u32);
    emu.lb_no_count(15usize, 14usize, 10u32, 2134176u32);
    emu.sb_no_count(11usize, 13usize, 8u32, 2134180u32);
    emu.sb_no_count(16usize, 13usize, 9u32, 2134184u32);
    emu.lb_no_count(11usize, 14usize, 11u32, 2134188u32);
    emu.sb_no_count(15usize, 13usize, 10u32, 2134192u32);
    emu.lb_no_count(15usize, 14usize, 12u32, 2134196u32);
    emu.lb_no_count(16usize, 14usize, 13u32, 2134200u32);
    emu.sb_no_count(11usize, 13usize, 11u32, 2134204u32);
    emu.lb_no_count(11usize, 14usize, 14u32, 2134208u32);
    emu.sb_no_count(15usize, 13usize, 12u32, 2134212u32);
    emu.sb_no_count(16usize, 13usize, 13u32, 2134216u32);
    emu.lb_no_count(15usize, 14usize, 15u32, 2134220u32);
    emu.sb_no_count(11usize, 13usize, 14u32, 2134224u32);
    emu.adi_no_count(14usize, 14usize, 16u32, 2134228u32);
    emu.adi_no_count(11usize, 13usize, 16u32, 2134232u32);
    emu.sb_no_count(15usize, 13usize, 15u32, 2134236u32);
    emu.adi_no_count(13usize, 11usize, 0u32, 2134240u32);
    emu.ani_no_count(11usize, 12usize, 8u32, 2134244u32);
    emu.add_memory_rw_events(36usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2133980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208fdc));
    } else {
        emu.pc = 2134248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002090e8));
    }
}
#[inline]
pub fn block_0x002090e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(11usize, 14usize, 0u32, 2134252u32);
    emu.lb_no_count(15usize, 14usize, 1u32, 2134256u32);
    emu.lb_no_count(16usize, 14usize, 2u32, 2134260u32);
    emu.sb_no_count(11usize, 13usize, 0u32, 2134264u32);
    emu.sb_no_count(15usize, 13usize, 1u32, 2134268u32);
    emu.lb_no_count(11usize, 14usize, 3u32, 2134272u32);
    emu.sb_no_count(16usize, 13usize, 2u32, 2134276u32);
    emu.lb_no_count(15usize, 14usize, 4u32, 2134280u32);
    emu.lb_no_count(16usize, 14usize, 5u32, 2134284u32);
    emu.sb_no_count(11usize, 13usize, 3u32, 2134288u32);
    emu.lb_no_count(11usize, 14usize, 6u32, 2134292u32);
    emu.sb_no_count(15usize, 13usize, 4u32, 2134296u32);
    emu.sb_no_count(16usize, 13usize, 5u32, 2134300u32);
    emu.lb_no_count(15usize, 14usize, 7u32, 2134304u32);
    emu.sb_no_count(11usize, 13usize, 6u32, 2134308u32);
    emu.adi_no_count(14usize, 14usize, 8u32, 2134312u32);
    emu.adi_no_count(11usize, 13usize, 8u32, 2134316u32);
    emu.sb_no_count(15usize, 13usize, 7u32, 2134320u32);
    emu.adi_no_count(13usize, 11usize, 0u32, 2134324u32);
    emu.ani_no_count(11usize, 12usize, 4u32, 2134328u32);
    emu.add_memory_rw_events(20usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2133988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208fe4));
    } else {
        emu.pc = 2134332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020913c));
    }
}
#[inline(always)]
pub fn block_0x0020913c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2134336u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2134032u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209010));
}
