pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2201488u32;
pub const PC_MAX: u32 = 2204100u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 123usize] = [
        block_0x00219790,
        block_0x002197a0,
        block_0x002197f4,
        block_0x00219804,
        block_0x00219808,
        block_0x00219814,
        block_0x00219830,
        block_0x0021984c,
        block_0x00219850,
        block_0x0021986c,
        block_0x00219870,
        block_0x00219880,
        block_0x00219888,
        block_0x00219890,
        block_0x002198a0,
        block_0x002198c0,
        block_0x002198d4,
        block_0x002198e8,
        block_0x00219900,
        block_0x00219928,
        block_0x00219948,
        block_0x00219950,
        block_0x00219960,
        block_0x00219964,
        block_0x00219968,
        block_0x0021996c,
        block_0x00219978,
        block_0x00219984,
        block_0x00219990,
        block_0x00219994,
        block_0x002199a4,
        block_0x002199b8,
        block_0x002199cc,
        block_0x002199d4,
        block_0x002199e8,
        block_0x002199ec,
        block_0x002199f0,
        block_0x002199f4,
        block_0x00219a04,
        block_0x00219a20,
        block_0x00219a34,
        block_0x00219a48,
        block_0x00219a6c,
        block_0x00219a70,
        block_0x00219aa8,
        block_0x00219abc,
        block_0x00219ad0,
        block_0x00219ae4,
        block_0x00219af4,
        block_0x00219b00,
        block_0x00219b1c,
        block_0x00219b2c,
        block_0x00219b70,
        block_0x00219bc4,
        block_0x00219bd0,
        block_0x00219c08,
        block_0x00219c0c,
        block_0x00219c10,
        block_0x00219c28,
        block_0x00219c2c,
        block_0x00219c48,
        block_0x00219c5c,
        block_0x00219c60,
        block_0x00219c64,
        block_0x00219c90,
        block_0x00219cb0,
        block_0x00219cd0,
        block_0x00219cdc,
        block_0x00219cfc,
        block_0x00219d2c,
        block_0x00219d40,
        block_0x00219d58,
        block_0x00219d68,
        block_0x00219d70,
        block_0x00219d80,
        block_0x00219d90,
        block_0x00219db0,
        block_0x00219db8,
        block_0x00219de8,
        block_0x00219df8,
        block_0x00219dfc,
        block_0x00219e30,
        block_0x00219e40,
        block_0x00219e5c,
        block_0x00219e68,
        block_0x00219eac,
        block_0x00219ebc,
        block_0x00219ecc,
        block_0x00219ed4,
        block_0x00219ee4,
        block_0x00219efc,
        block_0x00219f1c,
        block_0x00219f30,
        block_0x00219f58,
        block_0x00219f64,
        block_0x00219f98,
        block_0x00219fa0,
        block_0x00219fa4,
        block_0x00219fb0,
        block_0x00219fb4,
        block_0x00219fd8,
        block_0x0021a058,
        block_0x0021a060,
        block_0x0021a06c,
        block_0x0021a084,
        block_0x0021a094,
        block_0x0021a09c,
        block_0x0021a0a0,
        block_0x0021a0a4,
        block_0x0021a0b8,
        block_0x0021a0c0,
        block_0x0021a0c8,
        block_0x0021a0d8,
        block_0x0021a0ec,
        block_0x0021a140,
        block_0x0021a14c,
        block_0x0021a150,
        block_0x0021a160,
        block_0x0021a168,
        block_0x0021a170,
        block_0x0021a180,
        block_0x0021a194,
        block_0x0021a1c4,
    ];
    const IDX: [u16; 654usize] = [
        1u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 3u16,
        0u16, 0u16, 0u16, 4u16, 5u16, 0u16, 0u16, 6u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 7u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 8u16, 9u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 10u16, 11u16, 0u16, 0u16, 0u16, 12u16, 0u16, 13u16, 0u16,
        14u16, 0u16, 0u16, 0u16, 15u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 16u16,
        0u16, 0u16, 0u16, 0u16, 17u16, 0u16, 0u16, 0u16, 0u16, 18u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 19u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 20u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 21u16, 0u16, 22u16, 0u16, 0u16, 0u16,
        23u16, 24u16, 25u16, 26u16, 0u16, 0u16, 27u16, 0u16, 0u16, 28u16, 0u16, 0u16,
        29u16, 30u16, 0u16, 0u16, 0u16, 31u16, 0u16, 0u16, 0u16, 0u16, 32u16, 0u16, 0u16,
        0u16, 0u16, 33u16, 0u16, 34u16, 0u16, 0u16, 0u16, 0u16, 35u16, 36u16, 37u16,
        38u16, 0u16, 0u16, 0u16, 39u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 40u16, 0u16,
        0u16, 0u16, 0u16, 41u16, 0u16, 0u16, 0u16, 0u16, 42u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 43u16, 44u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 45u16, 0u16, 0u16, 0u16, 0u16, 46u16, 0u16,
        0u16, 0u16, 0u16, 47u16, 0u16, 0u16, 0u16, 0u16, 48u16, 0u16, 0u16, 0u16, 49u16,
        0u16, 0u16, 50u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 51u16, 0u16, 0u16, 0u16,
        52u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 53u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 54u16,
        0u16, 0u16, 55u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 56u16, 57u16, 58u16, 0u16, 0u16, 0u16, 0u16, 0u16, 59u16,
        60u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 61u16, 0u16, 0u16, 0u16, 0u16, 62u16,
        63u16, 64u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 65u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 66u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 67u16, 0u16, 0u16, 68u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        69u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 70u16,
        0u16, 0u16, 0u16, 0u16, 71u16, 0u16, 0u16, 0u16, 0u16, 0u16, 72u16, 0u16, 0u16,
        0u16, 73u16, 0u16, 74u16, 0u16, 0u16, 0u16, 75u16, 0u16, 0u16, 0u16, 76u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 77u16, 0u16, 78u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 79u16, 0u16, 0u16, 0u16, 80u16, 81u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 82u16,
        0u16, 0u16, 0u16, 83u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 84u16, 0u16, 0u16,
        85u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 86u16, 0u16, 0u16, 0u16, 87u16, 0u16, 0u16, 0u16, 88u16,
        0u16, 89u16, 0u16, 0u16, 0u16, 90u16, 0u16, 0u16, 0u16, 0u16, 0u16, 91u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 92u16, 0u16, 0u16, 0u16, 0u16, 93u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 94u16, 0u16, 0u16, 95u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 96u16, 0u16,
        97u16, 98u16, 0u16, 0u16, 99u16, 100u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 101u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 102u16, 0u16, 103u16, 0u16, 0u16,
        104u16, 0u16, 0u16, 0u16, 0u16, 0u16, 105u16, 0u16, 0u16, 0u16, 106u16, 0u16,
        107u16, 108u16, 109u16, 0u16, 0u16, 0u16, 0u16, 110u16, 0u16, 111u16, 0u16,
        112u16, 0u16, 0u16, 0u16, 113u16, 0u16, 0u16, 0u16, 0u16, 114u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 115u16, 0u16, 0u16, 116u16, 117u16, 0u16, 0u16,
        0u16, 118u16, 0u16, 119u16, 0u16, 120u16, 0u16, 0u16, 0u16, 121u16, 0u16, 0u16,
        0u16, 0u16, 122u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 123u16,
    ];
    if pc < 2201488u32 || pc > 2204100u32 {
        return None;
    }
    let word_offset = ((pc - 2201488u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x00219790(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2201492u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2201496u32)?;
    emu.apc_no_count(1usize, 2201496u32, 4294897664u32, 2201500u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2201504u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966604u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002197a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 12u32, 2201508u32)?;
    emu.adi_no_count(10usize, 2usize, 12u32, 2201512u32);
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2201516u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966756u32, 2201520u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2201524u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 624u32, 2201528u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2201532u32);
    emu.sw_no_count(0usize, 2usize, 40u32, 2201536u32)?;
    emu.adi_no_count(14usize, 2usize, 48u32, 2201540u32);
    emu.sw_no_count(10usize, 2usize, 48u32, 2201544u32)?;
    emu.sw_no_count(11usize, 2usize, 52u32, 2201548u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2201552u32);
    emu.sw_no_count(12usize, 2usize, 24u32, 2201556u32)?;
    emu.sw_no_count(13usize, 2usize, 28u32, 2201560u32)?;
    emu.sw_no_count(14usize, 2usize, 32u32, 2201564u32)?;
    emu.sw_no_count(10usize, 2usize, 36u32, 2201568u32)?;
    emu.adi_no_count(10usize, 2usize, 16u32, 2201572u32);
    emu.adi_no_count(11usize, 2usize, 59u32, 2201576u32);
    emu.adi_no_count(12usize, 2usize, 24u32, 2201580u32);
    emu.apc_no_count(1usize, 2201580u32, 0u32, 2201584u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2201588u32;
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
pub fn block_0x002197f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 16u32, 2201592u32);
    emu.lw_no_count(11usize, 2usize, 20u32, 2201596u32)?;
    emu.apc_no_count(1usize, 2201596u32, 4294963200u32, 2201600u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2201604u32;
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
pub fn block_0x00219804(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.update_insn_clock();
    return Err(format!("Unimplemented instruction (UNIMP) at PC {:#x}", 2201604u32));
}
#[inline(always)]
pub fn block_0x00219808(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 4u32, 2201612u32);
    emu.sb_no_count(11usize, 10usize, 0u32, 2201616u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2201620u32;
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
pub fn block_0x00219814(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2201624u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2201628u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2201632u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2201636u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2201640u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2201644u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2201648u32;
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
pub fn block_0x00219830(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2201652u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2201656u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2201660u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2201664u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2201668u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2201672u32)?;
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(5usize);
    let return_addr = 2201676u32;
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
pub fn block_0x0021984c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.update_insn_clock();
    return Err(format!("Unimplemented instruction (UNIMP) at PC {:#x}", 2201676u32));
}
#[inline(always)]
pub fn block_0x00219850(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2201684u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2201688u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2201692u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2201696u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2201700u32);
    emu.adi_no_count(12usize, 12usize, 4294967288u32, 2201704u32);
    emu.sli_no_count(13usize, 13usize, 3u32, 2201708u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2201708u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021986c));
}
#[inline(always)]
pub fn block_0x0021986c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2201736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219888));
    } else {
        emu.pc = 2201712u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219870));
    }
}
#[inline(always)]
pub fn block_0x00219870(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(9usize, 12usize, 12u32, 2201716u32)?;
    emu.adi_no_count(12usize, 12usize, 8u32, 2201720u32);
    emu.adi_no_count(13usize, 13usize, 4294967288u32, 2201724u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2201708u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021986c));
    } else {
        emu.pc = 2201728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219880));
    }
}
#[inline(always)]
pub fn block_0x00219880(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 12usize, 0u32, 2201732u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2201736u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2201744u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219890));
}
#[inline(always)]
pub fn block_0x00219888(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2201740u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2201744u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2201744u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219890));
}
#[inline(always)]
pub fn block_0x00219890(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2201748u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2201752u32);
    emu.apc_no_count(1usize, 2201752u32, 4294897664u32, 2201756u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2201760u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(924u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002198a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2201764u32);
    emu.sb_no_count(10usize, 8usize, 0u32, 2201768u32);
    emu.sw_no_count(9usize, 8usize, 4u32, 2201772u32)?;
    emu.lw_no_count(1usize, 2usize, 12u32, 2201776u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2201780u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2201784u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2201788u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2201792u32;
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
pub fn block_0x002198c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2201796u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2201800u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2201804u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2201808u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2201832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002198e8));
    } else {
        emu.pc = 2201812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002198d4));
    }
}
#[inline(always)]
pub fn block_0x002198d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2201816u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2201820u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2201824u32);
    emu.apc_no_count(1usize, 2201824u32, 4294897664u32, 2201828u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2201832u32;
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
pub fn block_0x002198e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2201836u32);
    emu.sb_no_count(10usize, 8usize, 0u32, 2201840u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2201844u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2201848u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2201852u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2201856u32;
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
pub fn block_0x00219900(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2201860u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2201864u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2201868u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2201872u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2201876u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2201880u32)?;
    emu.sw_no_count(20usize, 2usize, 40u32, 2201884u32)?;
    emu.sw_no_count(21usize, 2usize, 36u32, 2201888u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2201892u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2201964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021996c));
    } else {
        emu.pc = 2201896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219928));
    }
}
#[inline(always)]
pub fn block_0x00219928(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2201900u32);
    let a = 0u32.wrapping_add(536870912u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2201904u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 13usize, 4294967295u32, 2201908u32);
    emu.sli_no_count(11usize, 13usize, 3u32, 2201912u32);
    emu.adi_no_count(19usize, 19usize, 4294967295u32, 2201916u32);
    emu.anr_no_count(14usize, 14usize, 19usize, 2201920u32);
    emu.adi_no_count(14usize, 14usize, 1u32, 2201924u32);
    emu.adi_no_count(15usize, 12usize, 4u32, 2201928u32);
    emu.add_memory_rw_events(8usize);
    emu.pc = 2201928u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219948));
}
#[inline(always)]
pub fn block_0x00219948(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 15usize, 0u32, 2201932u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a != b {
        emu.pc = 2201956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219964));
    } else {
        emu.pc = 2201936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219950));
    }
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
    emu.adi_no_count(10usize, 10usize, 1u32, 2201940u32);
    emu.adi_no_count(11usize, 11usize, 4294967288u32, 2201944u32);
    emu.adi_no_count(15usize, 15usize, 8u32, 2201948u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2201928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219948));
    } else {
        emu.pc = 2201952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219960));
    }
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
    emu.adi_no_count(10usize, 14usize, 0u32, 2201956u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2201956u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219964));
}
#[inline(always)]
pub fn block_0x00219964(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2202280u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219aa8));
    } else {
        emu.pc = 2201960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219968));
    }
}
#[inline(always)]
pub fn block_0x00219968(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2201976u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219978));
    } else {
        emu.pc = 2201964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021996c));
    }
}
#[inline(always)]
pub fn block_0x0021996c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2201968u32);
    emu.sb_no_count(10usize, 8usize, 0u32, 2201972u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2201976u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2202184u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219a48));
}
#[inline(always)]
pub fn block_0x00219978(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(20usize, 10usize, 3u32, 2201980u32);
    emu.adr_no_count(20usize, 12usize, 20usize, 2201984u32);
    emu.sbr_no_count(9usize, 13usize, 10usize, 2201988u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2201988u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219984));
}
#[inline(always)]
pub fn block_0x00219984(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(21usize, 9usize, 3u32, 2201992u32);
    emu.adi_no_count(10usize, 20usize, 4294967288u32, 2201996u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2202000u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2202000u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219990));
}
#[inline(always)]
pub fn block_0x00219990(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2202144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219a20));
    } else {
        emu.pc = 2202004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219994));
    }
}
#[inline(always)]
pub fn block_0x00219994(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 10usize, 12u32, 2202008u32)?;
    emu.adi_no_count(10usize, 10usize, 8u32, 2202012u32);
    emu.adi_no_count(11usize, 11usize, 4294967288u32, 2202016u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2202000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219990));
    } else {
        emu.pc = 2202020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002199a4));
    }
}
#[inline(always)]
pub fn block_0x002199a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2202024u32)?;
    emu.adi_no_count(10usize, 0usize, 2u32, 2202028u32);
    emu.adi_no_count(12usize, 18usize, 0u32, 2202032u32);
    emu.apc_no_count(1usize, 2202032u32, 4294897664u32, 2202036u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2202040u32;
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
pub fn block_0x002199b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2202044u32);
    emu.adr_no_count(11usize, 9usize, 19usize, 2202048u32);
    emu.anr_no_count(11usize, 11usize, 19usize, 2202052u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2202056u32);
    emu.adi_no_count(12usize, 20usize, 4u32, 2202060u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2202060u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002199cc));
}
#[inline(always)]
pub fn block_0x002199cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 12usize, 0u32, 2202064u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a < b {
        emu.pc = 2202092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002199ec));
    } else {
        emu.pc = 2202068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002199d4));
    }
}
#[inline(always)]
pub fn block_0x002199d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(18usize, 18usize, 13usize, 2202072u32);
    emu.adi_no_count(10usize, 10usize, 1u32, 2202076u32);
    emu.adi_no_count(21usize, 21usize, 4294967288u32, 2202080u32);
    emu.adi_no_count(12usize, 12usize, 8u32, 2202084u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a != b {
        emu.pc = 2202060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002199cc));
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
    emu.adi_no_count(10usize, 11usize, 0u32, 2202092u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2202092u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002199ec));
}
#[inline(always)]
pub fn block_0x002199ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2202300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219abc));
    } else {
        emu.pc = 2202096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002199f0));
    }
}
#[inline(always)]
pub fn block_0x002199f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2202220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219a6c));
    } else {
        emu.pc = 2202100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002199f4));
    }
}
#[inline(always)]
pub fn block_0x002199f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 10usize, 3u32, 2202104u32);
    emu.adr_no_count(20usize, 20usize, 11usize, 2202108u32);
    emu.lw_no_count(11usize, 20usize, 4u32, 2202112u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2202320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219ad0));
    } else {
        emu.pc = 2202116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219a04));
    }
}
#[inline(always)]
pub fn block_0x00219a04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 20usize, 0u32, 2202120u32)?;
    emu.sbr_no_count(9usize, 9usize, 10usize, 2202124u32);
    emu.sbr_no_count(10usize, 11usize, 18usize, 2202128u32);
    emu.adr_no_count(12usize, 12usize, 18usize, 2202132u32);
    emu.sw_no_count(12usize, 20usize, 0u32, 2202136u32)?;
    emu.sw_no_count(10usize, 20usize, 4u32, 2202140u32)?;
    emu.add_memory_rw_events(7usize);
    let return_addr = 2202144u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2201988u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219984));
}
#[inline(always)]
pub fn block_0x00219a20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2202148u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2202152u32);
    emu.adi_no_count(12usize, 0usize, 0u32, 2202156u32);
    emu.apc_no_count(1usize, 2202156u32, 4294897664u32, 2202160u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2202164u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(520u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00219a34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2202168u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 10usize, 848u32, 2202172u32)?;
    emu.lw_no_count(10usize, 10usize, 852u32, 2202176u32)?;
    emu.sw_no_count(11usize, 8usize, 0u32, 2202180u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2202184u32)?;
    emu.add_memory_rw_events(5usize);
    emu.pc = 2202184u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219a48));
}
#[inline]
pub fn block_0x00219a48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2202188u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2202192u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2202196u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2202200u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2202204u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2202208u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2202212u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2202216u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2202220u32;
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
pub fn block_0x00219a6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2201964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021996c));
    } else {
        emu.pc = 2202224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219a70));
    }
}
#[inline]
pub fn block_0x00219a70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2202228u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 748u32, 2202232u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2202236u32);
    emu.sw_no_count(0usize, 2usize, 28u32, 2202240u32)?;
    emu.adi_no_count(12usize, 0usize, 4u32, 2202244u32);
    emu.sw_no_count(10usize, 2usize, 12u32, 2202248u32)?;
    emu.sw_no_count(11usize, 2usize, 16u32, 2202252u32)?;
    emu.sw_no_count(12usize, 2usize, 20u32, 2202256u32)?;
    emu.sw_no_count(0usize, 2usize, 24u32, 2202260u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2202264u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 756u32, 2202268u32);
    emu.adi_no_count(10usize, 2usize, 12u32, 2202272u32);
    emu.apc_no_count(1usize, 2202272u32, 8192u32, 2202276u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2202280u32;
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
pub fn block_0x00219aa8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2202284u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 788u32, 2202288u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2202292u32);
    emu.apc_no_count(1usize, 2202292u32, 20480u32, 2202296u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2202300u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965844u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00219abc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2202304u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 788u32, 2202308u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2202312u32);
    emu.apc_no_count(1usize, 2202312u32, 20480u32, 2202316u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2202320u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965824u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00219ad0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2202324u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 772u32, 2202328u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2202332u32);
    emu.apc_no_count(1usize, 2202332u32, 20480u32, 2202336u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2202340u32;
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
#[inline(always)]
pub fn block_0x00219ae4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2202344u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2202348u32)?;
    emu.apc_no_count(1usize, 2202348u32, 4294959104u32, 2202352u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2202356u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966932u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00219af4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2202360u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2202364u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2202368u32;
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
pub fn block_0x00219b00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2202372u32);
    let a = 0u32.wrapping_add(2244608u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2202376u32;
    emu.update_insn_clock();
    emu.lbu_no_count(12usize, 11usize, 1153u32, 2202380u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2202384u32);
    emu.sb_no_count(12usize, 2usize, 7u32, 2202388u32);
    emu.sb_no_count(10usize, 11usize, 1153u32, 2202392u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2202412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219b2c));
    } else {
        emu.pc = 2202396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219b1c));
    }
}
#[inline(always)]
pub fn block_0x00219b1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2244608u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2202400u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1153u32, 2202404u32);
    emu.adi_no_count(2usize, 2usize, 32u32, 2202408u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2202412u32;
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
pub fn block_0x00219b2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2202416u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 980u32, 2202420u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2202424u32);
    emu.sw_no_count(11usize, 2usize, 8u32, 2202428u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2202432u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2202436u32)?;
    emu.sw_no_count(0usize, 2usize, 20u32, 2202440u32)?;
    emu.sw_no_count(0usize, 2usize, 24u32, 2202444u32)?;
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2202448u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967158u32, 2202452u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2202456u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 988u32, 2202460u32);
    emu.adi_no_count(11usize, 2usize, 7u32, 2202464u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2202468u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2202472u32);
    emu.apc_no_count(1usize, 2202472u32, 0u32, 2202476u32);
    emu.add_memory_rw_events(17usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2202480u32;
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
#[inline]
pub fn block_0x00219b70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2202484u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2202488u32)?;
    emu.adi_no_count(11usize, 12usize, 0u32, 2202492u32);
    emu.sb_no_count(14usize, 2usize, 43u32, 2202496u32);
    emu.adi_no_count(12usize, 2usize, 43u32, 2202500u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2202504u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966224u32, 2202508u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2202512u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 4294966100u32, 2202516u32);
    emu.adi_no_count(16usize, 0usize, 1u32, 2202520u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2202524u32)?;
    emu.sw_no_count(12usize, 2usize, 32u32, 2202528u32)?;
    emu.sw_no_count(14usize, 2usize, 36u32, 2202532u32)?;
    emu.adi_no_count(12usize, 2usize, 32u32, 2202536u32);
    emu.lw_no_count(13usize, 13usize, 36u32, 2202540u32)?;
    emu.sw_no_count(15usize, 2usize, 8u32, 2202544u32)?;
    emu.sw_no_count(16usize, 2usize, 12u32, 2202548u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2202552u32)?;
    emu.sw_no_count(16usize, 2usize, 20u32, 2202556u32)?;
    emu.adi_no_count(12usize, 2usize, 8u32, 2202560u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2202564u32;
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
pub fn block_0x00219bc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2202568u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2202572u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2202576u32;
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
pub fn block_0x00219bd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2202580u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2202584u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2202588u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2202592u32)?;
    emu.sw_no_count(18usize, 2usize, 0u32, 2202596u32)?;
    emu.lw_no_count(12usize, 11usize, 4u32, 2202600u32)?;
    emu.lw_no_count(8usize, 11usize, 0u32, 2202604u32)?;
    emu.lbu_no_count(18usize, 10usize, 0u32, 2202608u32);
    emu.lw_no_count(9usize, 12usize, 12u32, 2202612u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2202616u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 856u32, 2202620u32);
    emu.adi_no_count(12usize, 0usize, 17u32, 2202624u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2202628u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(9usize);
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
        emu.pc = 2202664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219c28));
    } else {
        emu.pc = 2202636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219c0c));
    }
}
#[inline(always)]
pub fn block_0x00219c0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2202640u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2202640u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219c10));
}
#[inline(always)]
pub fn block_0x00219c10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2202644u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2202648u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2202652u32)?;
    emu.lw_no_count(18usize, 2usize, 0u32, 2202656u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2202660u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2202664u32;
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
pub fn block_0x00219c28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2202696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219c48));
    } else {
        emu.pc = 2202668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219c2c));
    }
}
#[inline(always)]
pub fn block_0x00219c2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2202672u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2202676u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2202680u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2202684u32)?;
    emu.lw_no_count(18usize, 2usize, 0u32, 2202688u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2202692u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2202696u32;
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
pub fn block_0x00219c48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2202700u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 873u32, 2202704u32);
    emu.adi_no_count(12usize, 0usize, 88u32, 2202708u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2202712u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(9usize);
    let return_addr = 2202716u32;
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
pub fn block_0x00219c5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2202636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219c0c));
    } else {
        emu.pc = 2202720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219c60));
    }
}
#[inline(always)]
pub fn block_0x00219c60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2202724u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2202640u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219c10));
}
#[inline]
pub fn block_0x00219c64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2202728u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2202732u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2202736u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2202740u32)?;
    emu.adi_no_count(8usize, 13usize, 0u32, 2202744u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2202748u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2202752u32);
    emu.adi_no_count(10usize, 0usize, 2u32, 2202756u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2202760u32);
    emu.apc_no_count(1usize, 2202760u32, 4294897664u32, 2202764u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2202768u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967212u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00219c90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2202772u32);
    emu.sb_no_count(10usize, 9usize, 0u32, 2202776u32);
    emu.sw_no_count(8usize, 9usize, 4u32, 2202780u32)?;
    emu.lw_no_count(1usize, 2usize, 12u32, 2202784u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2202788u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2202792u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2202796u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2202800u32;
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
pub fn block_0x00219cb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2202804u32)?;
    emu.lw_no_count(13usize, 10usize, 4u32, 2202808u32)?;
    emu.adi_no_count(14usize, 11usize, 0u32, 2202812u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2202816u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2202820u32);
    emu.adi_no_count(12usize, 14usize, 0u32, 2202824u32);
    emu.apc_no_count(6usize, 2202824u32, 16384u32, 2202828u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2202832u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966836u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00219cd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2202836u32)?;
    emu.apc_no_count(6usize, 2202836u32, 16384u32, 2202840u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2202844u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966772u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00219cdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2202848u32)?;
    emu.lw_no_count(13usize, 10usize, 4u32, 2202852u32)?;
    emu.adi_no_count(14usize, 11usize, 0u32, 2202856u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2202860u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2202864u32);
    emu.adi_no_count(12usize, 14usize, 0u32, 2202868u32);
    emu.apc_no_count(6usize, 2202868u32, 16384u32, 2202872u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2202876u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(488u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00219cfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2202880u32);
    emu.adi_no_count(16usize, 14usize, 0u32, 2202884u32);
    emu.adi_no_count(15usize, 13usize, 0u32, 2202888u32);
    emu.sw_no_count(11usize, 2usize, 8u32, 2202892u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2202896u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2202900u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 964u32, 2202904u32);
    emu.adi_no_count(11usize, 2usize, 8u32, 2202908u32);
    emu.adi_no_count(13usize, 2usize, 12u32, 2202912u32);
    emu.adi_no_count(14usize, 12usize, 0u32, 2202916u32);
    emu.apc_no_count(1usize, 2202916u32, 8192u32, 2202920u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2202924u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965424u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00219d2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2202928u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2202932u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2202936u32)?;
    emu.lw_no_count(6usize, 12usize, 12u32, 2202940u32)?;
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2202944u32;
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
pub fn block_0x00219d40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2202948u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2202952u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2202956u32)?;
    emu.lw_no_count(8usize, 10usize, 0u32, 2202960u32)?;
    emu.lw_no_count(11usize, 8usize, 12u32, 2202964u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2202984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219d68));
    } else {
        emu.pc = 2202968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219d58));
    }
}
#[inline(always)]
pub fn block_0x00219d58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 16u32, 2202972u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2202976u32);
    emu.apc_no_count(1usize, 2202976u32, 4294893568u32, 2202980u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2202984u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1484u32);
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
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4294967295u32, 2202988u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2203008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219d80));
    } else {
        emu.pc = 2202992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219d70));
    }
}
#[inline(always)]
pub fn block_0x00219d70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 4u32, 2202996u32)?;
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2203000u32);
    emu.sw_no_count(10usize, 8usize, 4u32, 2203004u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2203024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219d90));
    } else {
        emu.pc = 2203008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219d80));
    }
}
#[inline(always)]
pub fn block_0x00219d80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2203012u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2203016u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2203020u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2203024u32;
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
pub fn block_0x00219d90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 24u32, 2203028u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2203032u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2203036u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2203040u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2203044u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2203048u32);
    emu.apc_no_count(6usize, 2203048u32, 4294893568u32, 2203052u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2203056u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x00219db0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2203056u32, 4294959104u32, 2203060u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2203064u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1340u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00219db8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2203068u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2203072u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2203076u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2203080u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2203084u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2203088u32)?;
    emu.adi_no_count(9usize, 13usize, 0u32, 2203092u32);
    emu.adi_no_count(19usize, 12usize, 0u32, 2203096u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2203100u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2203104u32);
    emu.apc_no_count(1usize, 2203104u32, 4294893568u32, 2203108u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2203112u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1504u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00219de8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 12u32, 2203116u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2203120u32);
    emu.apc_no_count(1usize, 2203120u32, 4294893568u32, 2203124u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2203128u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1312u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00219df8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2203184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219e30));
    } else {
        emu.pc = 2203132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219dfc));
    }
}
#[inline]
pub fn block_0x00219dfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(19usize, 10usize, 0u32, 2203136u32)?;
    emu.sw_no_count(9usize, 10usize, 4u32, 2203140u32)?;
    emu.sb_no_count(18usize, 10usize, 8u32, 2203144u32);
    emu.adi_no_count(11usize, 0usize, 3u32, 2203148u32);
    emu.sb_no_count(11usize, 8usize, 0u32, 2203152u32);
    emu.sw_no_count(10usize, 8usize, 4u32, 2203156u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2203160u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2203164u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2203168u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2203172u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2203176u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2203180u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2203184u32;
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
pub fn block_0x00219e30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2203188u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2203192u32);
    emu.apc_no_count(1usize, 2203192u32, 0u32, 2203196u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2203200u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1680u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00219e40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2203204u32);
    emu.adi_no_count(11usize, 10usize, 0u32, 2203208u32);
    emu.lbu_no_count(12usize, 10usize, 0u32, 2203212u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2203216u32);
    emu.sb_no_count(12usize, 2usize, 7u32, 2203220u32);
    emu.sb_no_count(10usize, 11usize, 0u32, 2203224u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2203240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219e68));
    } else {
        emu.pc = 2203228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219e5c));
    }
}
#[inline(always)]
pub fn block_0x00219e5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2203232u32);
    emu.adi_no_count(2usize, 2usize, 32u32, 2203236u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2203240u32;
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
pub fn block_0x00219e68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2203244u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 980u32, 2203248u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2203252u32);
    emu.sw_no_count(11usize, 2usize, 8u32, 2203256u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2203260u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2203264u32)?;
    emu.sw_no_count(0usize, 2usize, 20u32, 2203268u32)?;
    emu.sw_no_count(0usize, 2usize, 24u32, 2203272u32)?;
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2203276u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967158u32, 2203280u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2203284u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 988u32, 2203288u32);
    emu.adi_no_count(11usize, 2usize, 7u32, 2203292u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2203296u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2203300u32);
    emu.apc_no_count(1usize, 2203300u32, 0u32, 2203304u32);
    emu.add_memory_rw_events(17usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2203308u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966872u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00219eac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 4u32, 2203312u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2203316u32)?;
    emu.lw_no_count(6usize, 12usize, 12u32, 2203320u32)?;
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2203324u32;
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
pub fn block_0x00219ebc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2244608u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2203328u32;
    emu.update_insn_clock();
    emu.lw_no_count(12usize, 11usize, 1168u32, 2203332u32)?;
    emu.adi_no_count(11usize, 0usize, 2u32, 2203336u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2203364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219ee4));
    } else {
        emu.pc = 2203340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219ecc));
    }
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
    emu.lw_no_count(11usize, 12usize, 8u32, 2203344u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2203440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219f30));
    } else {
        emu.pc = 2203348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219ed4));
    }
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
    emu.lw_no_count(12usize, 12usize, 12u32, 2203352u32)?;
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2203356u32);
    emu.apc_no_count(6usize, 2203356u32, 0u32, 2203360u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2203364u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(136u32);
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
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2244608u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2203368u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1168u32, 2203372u32);
    emu.lw_no_count(11usize, 12usize, 8u32, 2203376u32)?;
    emu.lw_no_count(12usize, 12usize, 12u32, 2203380u32)?;
    emu.orr_no_count(13usize, 11usize, 12usize, 2203384u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2203480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219f58));
    } else {
        emu.pc = 2203388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219efc));
    }
}
#[inline(always)]
pub fn block_0x00219efc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2244608u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2203392u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2244608u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2203396u32;
    emu.update_insn_clock();
    emu.lw_no_count(14usize, 14usize, 1160u32, 2203400u32)?;
    emu.lw_no_count(13usize, 13usize, 1156u32, 2203404u32)?;
    emu.xrr_no_count(12usize, 14usize, 12usize, 2203408u32);
    emu.xrr_no_count(11usize, 13usize, 11usize, 2203412u32);
    emu.orr_no_count(11usize, 11usize, 12usize, 2203416u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2203480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219f58));
    } else {
        emu.pc = 2203420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219f1c));
    }
}
#[inline(always)]
pub fn block_0x00219f1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2203424u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966044u32, 2203428u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2203432u32);
    emu.apc_no_count(6usize, 2203432u32, 0u32, 2203436u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2203440u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(60u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00219f30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 12usize, 0u32, 2203444u32)?;
    emu.lw_no_count(12usize, 12usize, 4u32, 2203448u32)?;
    let a = 0u32.wrapping_add(2244608u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2203452u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1168u32, 2203456u32);
    emu.lw_no_count(14usize, 13usize, 12u32, 2203460u32)?;
    emu.lw_no_count(13usize, 13usize, 8u32, 2203464u32)?;
    emu.xrr_no_count(12usize, 12usize, 14usize, 2203468u32);
    emu.xrr_no_count(11usize, 11usize, 13usize, 2203472u32);
    emu.orr_no_count(11usize, 11usize, 12usize, 2203476u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2203420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219f1c));
    } else {
        emu.pc = 2203480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219f58));
    }
}
#[inline(always)]
pub fn block_0x00219f58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2203484u32);
    emu.apc_no_count(6usize, 2203484u32, 0u32, 2203488u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2203492u32;
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
pub fn block_0x00219f64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294966656u32, 2203496u32);
    emu.sw_no_count(1usize, 2usize, 636u32, 2203500u32)?;
    emu.sw_no_count(8usize, 2usize, 632u32, 2203504u32)?;
    emu.sw_no_count(9usize, 2usize, 628u32, 2203508u32)?;
    emu.sw_no_count(18usize, 2usize, 624u32, 2203512u32)?;
    emu.sw_no_count(19usize, 2usize, 620u32, 2203516u32)?;
    emu.sw_no_count(20usize, 2usize, 616u32, 2203520u32)?;
    emu.sw_no_count(21usize, 2usize, 612u32, 2203524u32)?;
    emu.sw_no_count(22usize, 2usize, 608u32, 2203528u32)?;
    emu.sw_no_count(23usize, 2usize, 604u32, 2203532u32)?;
    emu.sw_no_count(24usize, 2usize, 600u32, 2203536u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2203540u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2203556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219fa4));
    } else {
        emu.pc = 2203544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219f98));
    }
}
#[inline(always)]
pub fn block_0x00219f98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 11usize, 0u32, 2203548u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2203568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219fb0));
    } else {
        emu.pc = 2203552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219fa0));
    }
}
#[inline(always)]
pub fn block_0x00219fa0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2203556u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2203572u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219fb4));
}
#[inline(always)]
pub fn block_0x00219fa4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2203560u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1004u32, 2203564u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2203572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219fb4));
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
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 9u32, 2203572u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2203572u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219fb4));
}
#[inline]
pub fn block_0x00219fb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 8u32, 2203576u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2203580u32)?;
    emu.adi_no_count(9usize, 2usize, 16u32, 2203584u32);
    emu.adi_no_count(10usize, 2usize, 16u32, 2203588u32);
    emu.adi_no_count(12usize, 0usize, 512u32, 2203592u32);
    emu.adi_no_count(18usize, 0usize, 512u32, 2203596u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2203600u32);
    emu.apc_no_count(1usize, 2203600u32, 4294897664u32, 2203604u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2203608u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966380u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00219fd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 32u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(9usize, 2usize, 528u32, 2203612u32)?;
    emu.sw_no_count(18usize, 2usize, 532u32, 2203616u32)?;
    emu.sw_no_count(0usize, 2usize, 536u32, 2203620u32)?;
    emu.sw_no_count(0usize, 2usize, 540u32, 2203624u32)?;
    emu.lw_no_count(21usize, 8usize, 0u32, 2203628u32)?;
    emu.lw_no_count(19usize, 8usize, 4u32, 2203632u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2203636u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(20usize, a);
    emu.pc = 2203640u32;
    emu.update_insn_clock();
    emu.adi_no_count(20usize, 20usize, 4294966492u32, 2203644u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(23usize, a);
    emu.pc = 2203648u32;
    emu.update_insn_clock();
    emu.adi_no_count(23usize, 23usize, 4294967204u32, 2203652u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(22usize, a);
    emu.pc = 2203656u32;
    emu.update_insn_clock();
    emu.adi_no_count(22usize, 22usize, 1060u32, 2203660u32);
    emu.adi_no_count(18usize, 0usize, 4u32, 2203664u32);
    emu.sw_no_count(0usize, 2usize, 568u32, 2203668u32)?;
    emu.adi_no_count(11usize, 2usize, 576u32, 2203672u32);
    emu.adi_no_count(9usize, 0usize, 3u32, 2203676u32);
    emu.sw_no_count(10usize, 2usize, 576u32, 2203680u32)?;
    emu.sw_no_count(20usize, 2usize, 580u32, 2203684u32)?;
    emu.sw_no_count(21usize, 2usize, 584u32, 2203688u32)?;
    emu.sw_no_count(23usize, 2usize, 588u32, 2203692u32)?;
    emu.sw_no_count(19usize, 2usize, 592u32, 2203696u32)?;
    emu.sw_no_count(20usize, 2usize, 596u32, 2203700u32)?;
    emu.sw_no_count(22usize, 2usize, 552u32, 2203704u32)?;
    emu.sw_no_count(18usize, 2usize, 556u32, 2203708u32)?;
    emu.sw_no_count(11usize, 2usize, 560u32, 2203712u32)?;
    emu.sw_no_count(9usize, 2usize, 564u32, 2203716u32)?;
    emu.adi_no_count(10usize, 2usize, 544u32, 2203720u32);
    emu.adi_no_count(11usize, 2usize, 528u32, 2203724u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2203728u32);
    emu.apc_no_count(1usize, 2203728u32, 4294959104u32, 2203732u32);
    emu.add_memory_rw_events(32usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2203736u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965272u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021a058(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 544u32, 2203740u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2203808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a0a0));
    } else {
        emu.pc = 2203744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a060));
    }
}
#[inline(always)]
pub fn block_0x0021a060(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 2usize, 536u32, 2203748u32)?;
    emu.adi_no_count(10usize, 0usize, 513u32, 2203752u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2204100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a1c4));
    } else {
        emu.pc = 2203756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a06c));
    }
}
#[inline(always)]
pub fn block_0x0021a06c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 12u32, 2203760u32)?;
    emu.lw_no_count(11usize, 8usize, 8u32, 2203764u32)?;
    emu.lw_no_count(14usize, 10usize, 28u32, 2203768u32)?;
    emu.adi_no_count(10usize, 2usize, 576u32, 2203772u32);
    emu.adi_no_count(12usize, 2usize, 16u32, 2203776u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2203780u32;
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
pub fn block_0x0021a084(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 576u32, 2203784u32);
    emu.lw_no_count(8usize, 2usize, 580u32, 2203788u32)?;
    emu.adi_no_count(11usize, 0usize, 4u32, 2203792u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2203984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a150));
    } else {
        emu.pc = 2203796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a094));
    }
}
#[inline(always)]
pub fn block_0x0021a094(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 3u32, 2203800u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2204052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a194));
    } else {
        emu.pc = 2203804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a09c));
    }
}
#[inline(always)]
pub fn block_0x0021a09c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2203808u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2203984u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021a150));
}
#[inline(always)]
pub fn block_0x0021a0a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2203884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a0ec));
    } else {
        emu.pc = 2203812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a0a4));
    }
}
#[inline(always)]
pub fn block_0x0021a0a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(9usize, 2usize, 548u32, 2203816u32)?;
    emu.lw_no_count(24usize, 9usize, 4u32, 2203820u32)?;
    emu.lw_no_count(11usize, 24usize, 0u32, 2203824u32)?;
    emu.lw_no_count(18usize, 9usize, 0u32, 2203828u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2203840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a0c0));
    } else {
        emu.pc = 2203832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a0b8));
    }
}
#[inline(always)]
pub fn block_0x0021a0b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2203836u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2203840u32;
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
pub fn block_0x0021a0c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 24usize, 4u32, 2203844u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2203864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a0d8));
    } else {
        emu.pc = 2203848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a0c8));
    }
}
#[inline(always)]
pub fn block_0x0021a0c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 24usize, 8u32, 2203852u32)?;
    emu.adi_no_count(10usize, 18usize, 0u32, 2203856u32);
    emu.apc_no_count(1usize, 2203856u32, 4294893568u32, 2203860u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2203864u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(604u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021a0d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 12u32, 2203868u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2203872u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2203876u32);
    emu.apc_no_count(1usize, 2203876u32, 4294893568u32, 2203880u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2203884u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(584u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021a0ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 8usize, 8u32, 2203888u32)?;
    emu.lw_no_count(10usize, 8usize, 12u32, 2203892u32)?;
    emu.adi_no_count(12usize, 2usize, 8u32, 2203896u32);
    emu.adi_no_count(18usize, 0usize, 4u32, 2203900u32);
    emu.sw_no_count(0usize, 2usize, 568u32, 2203904u32)?;
    emu.adi_no_count(13usize, 2usize, 576u32, 2203908u32);
    emu.lw_no_count(14usize, 10usize, 36u32, 2203912u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2203916u32)?;
    emu.sw_no_count(20usize, 2usize, 580u32, 2203920u32)?;
    emu.sw_no_count(21usize, 2usize, 584u32, 2203924u32)?;
    emu.sw_no_count(23usize, 2usize, 588u32, 2203928u32)?;
    emu.adi_no_count(9usize, 0usize, 3u32, 2203932u32);
    emu.sw_no_count(19usize, 2usize, 592u32, 2203936u32)?;
    emu.sw_no_count(20usize, 2usize, 596u32, 2203940u32)?;
    emu.sw_no_count(22usize, 2usize, 552u32, 2203944u32)?;
    emu.sw_no_count(18usize, 2usize, 556u32, 2203948u32)?;
    emu.sw_no_count(13usize, 2usize, 560u32, 2203952u32)?;
    emu.sw_no_count(9usize, 2usize, 564u32, 2203956u32)?;
    emu.adi_no_count(10usize, 2usize, 544u32, 2203960u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2203964u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2203968u32;
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
pub fn block_0x0021a140(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 544u32, 2203972u32);
    emu.lw_no_count(8usize, 2usize, 548u32, 2203976u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a < b {
        emu.pc = 2203984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a150));
    } else {
        emu.pc = 2203980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a14c));
    }
}
#[inline(always)]
pub fn block_0x0021a14c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2204052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a194));
    } else {
        emu.pc = 2203984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a150));
    }
}
#[inline(always)]
pub fn block_0x0021a150(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 8usize, 4u32, 2203988u32)?;
    emu.lw_no_count(11usize, 18usize, 0u32, 2203992u32)?;
    emu.lw_no_count(9usize, 8usize, 0u32, 2203996u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2204008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a168));
    } else {
        emu.pc = 2204000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a160));
    }
}
#[inline(always)]
pub fn block_0x0021a160(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2204004u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2204008u32;
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
pub fn block_0x0021a168(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 18usize, 4u32, 2204012u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2204032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a180));
    } else {
        emu.pc = 2204016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a170));
    }
}
#[inline(always)]
pub fn block_0x0021a170(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 8u32, 2204020u32)?;
    emu.adi_no_count(10usize, 9usize, 0u32, 2204024u32);
    emu.apc_no_count(1usize, 2204024u32, 4294893568u32, 2204028u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2204032u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(436u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021a180(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 12u32, 2204036u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2204040u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2204044u32);
    emu.apc_no_count(1usize, 2204044u32, 4294893568u32, 2204048u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2204052u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(416u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021a194(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 636u32, 2204056u32)?;
    emu.lw_no_count(8usize, 2usize, 632u32, 2204060u32)?;
    emu.lw_no_count(9usize, 2usize, 628u32, 2204064u32)?;
    emu.lw_no_count(18usize, 2usize, 624u32, 2204068u32)?;
    emu.lw_no_count(19usize, 2usize, 620u32, 2204072u32)?;
    emu.lw_no_count(20usize, 2usize, 616u32, 2204076u32)?;
    emu.lw_no_count(21usize, 2usize, 612u32, 2204080u32)?;
    emu.lw_no_count(22usize, 2usize, 608u32, 2204084u32)?;
    emu.lw_no_count(23usize, 2usize, 604u32, 2204088u32)?;
    emu.lw_no_count(24usize, 2usize, 600u32, 2204092u32)?;
    emu.adi_no_count(2usize, 2usize, 640u32, 2204096u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2204100u32;
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
pub fn block_0x0021a1c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2204104u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1016u32, 2204108u32);
    emu.adi_no_count(11usize, 0usize, 512u32, 2204112u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2204116u32);
    emu.apc_no_count(1usize, 2204116u32, 16384u32, 2204120u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2204124u32;
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
