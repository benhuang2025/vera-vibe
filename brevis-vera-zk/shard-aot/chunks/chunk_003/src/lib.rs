pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2107432u32;
pub const PC_MAX: u32 = 2110588u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 108usize] = [
        block_0x00202828,
        block_0x0020283c,
        block_0x00202844,
        block_0x00202854,
        block_0x00202860,
        block_0x00202898,
        block_0x002028b0,
        block_0x0020293c,
        block_0x00202950,
        block_0x0020295c,
        block_0x00202970,
        block_0x00202984,
        block_0x002029cc,
        block_0x002029e0,
        block_0x002029e8,
        block_0x00202b3c,
        block_0x00202b4c,
        block_0x00202b5c,
        block_0x00202b6c,
        block_0x00202b7c,
        block_0x00202b8c,
        block_0x00202b9c,
        block_0x00202bac,
        block_0x00202bb8,
        block_0x00202c08,
        block_0x00202c40,
        block_0x00202c78,
        block_0x00202cb0,
        block_0x00202ce8,
        block_0x00202d20,
        block_0x00202d58,
        block_0x00202d90,
        block_0x00202d9c,
        block_0x00202db8,
        block_0x00202dc8,
        block_0x00202dd8,
        block_0x00202e00,
        block_0x00202e08,
        block_0x00202e18,
        block_0x00202e30,
        block_0x00202e44,
        block_0x00202e4c,
        block_0x00202e50,
        block_0x00202e68,
        block_0x00202e98,
        block_0x00202e9c,
        block_0x00202eb0,
        block_0x00202ed8,
        block_0x00202eec,
        block_0x00202ef0,
        block_0x00202f10,
        block_0x00202f20,
        block_0x00202f28,
        block_0x00202f30,
        block_0x00202f38,
        block_0x00202f3c,
        block_0x00202f58,
        block_0x00202f64,
        block_0x00202f6c,
        block_0x00202f78,
        block_0x00202f7c,
        block_0x00202f80,
        block_0x00202f94,
        block_0x00202fac,
        block_0x00202fbc,
        block_0x00202ff0,
        block_0x00203008,
        block_0x0020303c,
        block_0x00203044,
        block_0x00203070,
        block_0x00203074,
        block_0x00203088,
        block_0x00203090,
        block_0x002030c4,
        block_0x002030cc,
        block_0x002030d4,
        block_0x002030dc,
        block_0x002030e4,
        block_0x002030ec,
        block_0x00203114,
        block_0x00203174,
        block_0x0020317c,
        block_0x0020318c,
        block_0x00203194,
        block_0x00203198,
        block_0x002031c8,
        block_0x002031d0,
        block_0x002031e8,
        block_0x002031f0,
        block_0x00203204,
        block_0x0020321c,
        block_0x0020327c,
        block_0x00203284,
        block_0x002032a4,
        block_0x00203304,
        block_0x00203308,
        block_0x0020330c,
        block_0x00203314,
        block_0x0020331c,
        block_0x00203324,
        block_0x00203350,
        block_0x00203358,
        block_0x00203360,
        block_0x00203364,
        block_0x00203388,
        block_0x00203394,
        block_0x00203428,
        block_0x0020347c,
    ];
    const IDX: [u16; 790usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 3u16, 0u16, 0u16, 0u16, 4u16, 0u16,
        0u16, 5u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 6u16, 0u16, 0u16, 0u16, 0u16, 0u16, 7u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 8u16, 0u16, 0u16, 0u16, 0u16, 9u16, 0u16, 0u16, 10u16,
        0u16, 0u16, 0u16, 0u16, 11u16, 0u16, 0u16, 0u16, 0u16, 12u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 13u16, 0u16, 0u16, 0u16, 0u16, 14u16, 0u16, 15u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 16u16, 0u16, 0u16, 0u16, 17u16, 0u16, 0u16, 0u16, 18u16, 0u16, 0u16,
        0u16, 19u16, 0u16, 0u16, 0u16, 20u16, 0u16, 0u16, 0u16, 21u16, 0u16, 0u16, 0u16,
        22u16, 0u16, 0u16, 0u16, 23u16, 0u16, 0u16, 24u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 25u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 26u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 27u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 28u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 29u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 30u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 31u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 32u16, 0u16, 0u16, 33u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 34u16, 0u16, 0u16, 0u16, 35u16, 0u16, 0u16, 0u16,
        36u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 37u16, 0u16, 38u16,
        0u16, 0u16, 0u16, 39u16, 0u16, 0u16, 0u16, 0u16, 0u16, 40u16, 0u16, 0u16, 0u16,
        0u16, 41u16, 0u16, 42u16, 43u16, 0u16, 0u16, 0u16, 0u16, 0u16, 44u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 45u16, 46u16, 0u16, 0u16,
        0u16, 0u16, 47u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 48u16,
        0u16, 0u16, 0u16, 0u16, 49u16, 50u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        51u16, 0u16, 0u16, 0u16, 52u16, 0u16, 53u16, 0u16, 54u16, 0u16, 55u16, 56u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 57u16, 0u16, 0u16, 58u16, 0u16, 59u16, 0u16,
        0u16, 60u16, 61u16, 62u16, 0u16, 0u16, 0u16, 0u16, 63u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 64u16, 0u16, 0u16, 0u16, 65u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 66u16, 0u16, 0u16, 0u16, 0u16, 0u16, 67u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 68u16, 0u16,
        69u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 70u16, 71u16,
        0u16, 0u16, 0u16, 0u16, 72u16, 0u16, 73u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 74u16, 0u16, 75u16, 0u16, 76u16, 0u16, 77u16,
        0u16, 78u16, 0u16, 79u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        80u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 81u16, 0u16,
        82u16, 0u16, 0u16, 0u16, 83u16, 0u16, 84u16, 85u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 86u16, 0u16, 87u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 88u16, 0u16, 89u16, 0u16, 0u16, 0u16, 0u16, 90u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 91u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 92u16,
        0u16, 93u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 94u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 95u16, 96u16, 97u16, 0u16, 98u16, 0u16,
        99u16, 0u16, 100u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        101u16, 0u16, 102u16, 0u16, 103u16, 104u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 105u16, 0u16, 0u16, 106u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 107u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 108u16,
    ];
    if pc < 2107432u32 || pc > 2110588u32 {
        return None;
    }
    let word_offset = ((pc - 2107432u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x00202828(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2107436u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2107440u32)?;
    emu.lw_no_count(11usize, 11usize, 20u32, 2107444u32)?;
    emu.apc_no_count(1usize, 2107444u32, 0u32, 2107448u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107452u32;
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
pub fn block_0x0020283c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2107452u32, 32768u32, 2107456u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107460u32;
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
pub fn block_0x00202844(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2107464u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2107468u32)?;
    emu.apc_no_count(1usize, 2107468u32, 4096u32, 2107472u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107476u32;
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
pub fn block_0x00202854(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2107480u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2107484u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107488u32;
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
pub fn block_0x00202860(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967088u32, 2107492u32);
    emu.sw_no_count(1usize, 2usize, 204u32, 2107496u32)?;
    emu.sw_no_count(8usize, 2usize, 200u32, 2107500u32)?;
    emu.sw_no_count(9usize, 2usize, 196u32, 2107504u32)?;
    emu.sw_no_count(18usize, 2usize, 192u32, 2107508u32)?;
    emu.sw_no_count(19usize, 2usize, 188u32, 2107512u32)?;
    emu.sw_no_count(20usize, 2usize, 184u32, 2107516u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2107520u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2107524u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 10usize, 4294966320u32, 2107528u32)?;
    emu.sw_no_count(0usize, 10usize, 4294966324u32, 2107532u32)?;
    emu.ani_no_count(11usize, 11usize, 1u32, 2107536u32);
    emu.sw_no_count(0usize, 10usize, 4294966320u32, 2107540u32)?;
    emu.add_memory_rw_events(13usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2108856u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202db8));
    } else {
        emu.pc = 2107544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202898));
    }
}
#[inline(always)]
pub fn block_0x00202898(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2107548u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966328u32, 2107552u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2107556u32);
    emu.adi_no_count(12usize, 0usize, 112u32, 2107560u32);
    emu.apc_no_count(1usize, 2107560u32, 0u32, 2107564u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107568u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1996u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x002028b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 35u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 2usize, 48u32, 2107572u32);
    emu.lbu_no_count(18usize, 2usize, 112u32, 2107576u32);
    emu.lw_no_count(10usize, 2usize, 40u32, 2107580u32)?;
    emu.lw_no_count(11usize, 2usize, 44u32, 2107584u32)?;
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2107588u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 0usize, 128u32, 2107592u32);
    emu.adi_no_count(12usize, 12usize, 4294967040u32, 2107596u32);
    emu.sli_no_count(14usize, 11usize, 9u32, 2107600u32);
    emu.sli_no_count(15usize, 10usize, 9u32, 2107604u32);
    emu.sli_no_count(16usize, 18usize, 3u32, 2107608u32);
    emu.sli_no_count(17usize, 10usize, 1u32, 2107612u32);
    emu.sli_no_count(11usize, 11usize, 1u32, 2107616u32);
    emu.orr_no_count(16usize, 15usize, 16usize, 2107620u32);
    emu.anr_no_count(17usize, 17usize, 12usize, 2107624u32);
    emu.sri_no_count(15usize, 15usize, 24u32, 2107628u32);
    emu.anr_no_count(11usize, 11usize, 12usize, 2107632u32);
    emu.orr_no_count(15usize, 17usize, 15usize, 2107636u32);
    emu.sri_no_count(17usize, 14usize, 24u32, 2107640u32);
    emu.orr_no_count(11usize, 11usize, 17usize, 2107644u32);
    emu.adi_no_count(17usize, 0usize, 63u32, 2107648u32);
    emu.sri_no_count(5usize, 10usize, 23u32, 2107652u32);
    emu.orr_no_count(10usize, 14usize, 5usize, 2107656u32);
    emu.anr_no_count(14usize, 16usize, 12usize, 2107660u32);
    emu.anr_no_count(12usize, 10usize, 12usize, 2107664u32);
    emu.sli_no_count(10usize, 18usize, 27u32, 2107668u32);
    emu.orr_no_count(15usize, 10usize, 15usize, 2107672u32);
    emu.adr_no_count(10usize, 9usize, 18usize, 2107676u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2107680u32);
    emu.sli_no_count(14usize, 14usize, 8u32, 2107684u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2107688u32);
    emu.orr_no_count(11usize, 5usize, 11usize, 2107692u32);
    emu.orr_no_count(20usize, 15usize, 14usize, 2107696u32);
    emu.orr_no_count(19usize, 11usize, 12usize, 2107700u32);
    emu.sb_no_count(13usize, 10usize, 0u32, 2107704u32);
    emu.add_memory_rw_events(34usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2107740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020295c));
    } else {
        emu.pc = 2107708u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020293c));
    }
}
#[inline(always)]
pub fn block_0x0020293c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 10usize, 1u32, 2107712u32);
    emu.xri_no_count(12usize, 18usize, 63u32, 2107716u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2107720u32);
    emu.apc_no_count(1usize, 2107720u32, 0u32, 2107724u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107728u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1588u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202950(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(10usize, 18usize, 56u32, 2107732u32);
    emu.adi_no_count(11usize, 0usize, 7u32, 2107736u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2107852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002029cc));
    } else {
        emu.pc = 2107740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020295c));
    }
}
#[inline(always)]
pub fn block_0x0020295c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 8u32, 2107744u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2107748u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2107752u32);
    emu.apc_no_count(1usize, 2107752u32, 4096u32, 2107756u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107760u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967088u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202970(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 120u32, 2107764u32);
    emu.adi_no_count(12usize, 0usize, 56u32, 2107768u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2107772u32);
    emu.apc_no_count(1usize, 2107772u32, 0u32, 2107776u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107780u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1536u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00202984(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 20usize, 24u32, 2107784u32);
    emu.sri_no_count(11usize, 20usize, 16u32, 2107788u32);
    emu.sri_no_count(12usize, 20usize, 8u32, 2107792u32);
    emu.sri_no_count(13usize, 19usize, 24u32, 2107796u32);
    emu.sri_no_count(14usize, 19usize, 16u32, 2107800u32);
    emu.sb_no_count(20usize, 2usize, 180u32, 2107804u32);
    emu.sb_no_count(12usize, 2usize, 181u32, 2107808u32);
    emu.sb_no_count(11usize, 2usize, 182u32, 2107812u32);
    emu.sb_no_count(10usize, 2usize, 183u32, 2107816u32);
    emu.sri_no_count(10usize, 19usize, 8u32, 2107820u32);
    emu.sb_no_count(19usize, 2usize, 176u32, 2107824u32);
    emu.sb_no_count(10usize, 2usize, 177u32, 2107828u32);
    emu.sb_no_count(14usize, 2usize, 178u32, 2107832u32);
    emu.sb_no_count(13usize, 2usize, 179u32, 2107836u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2107840u32);
    emu.adi_no_count(11usize, 2usize, 120u32, 2107844u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2107848u32);
    emu.add_memory_rw_events(18usize);
    let return_addr = 2107852u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2107872u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002029e0));
}
#[inline(always)]
pub fn block_0x002029cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(19usize, 2usize, 104u32, 2107856u32)?;
    emu.sw_no_count(20usize, 2usize, 108u32, 2107860u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2107864u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2107868u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2107872u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2107872u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002029e0));
}
#[inline(always)]
pub fn block_0x002029e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2107872u32, 4096u32, 2107876u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107880u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966968u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x002029e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 85u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2107884u32);
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2107888u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 2usize, 8u32, 2107892u32)?;
    emu.lw_no_count(13usize, 2usize, 12u32, 2107896u32)?;
    emu.lw_no_count(15usize, 2usize, 16u32, 2107900u32)?;
    emu.lw_no_count(16usize, 2usize, 20u32, 2107904u32)?;
    emu.lw_no_count(17usize, 2usize, 24u32, 2107908u32)?;
    emu.lw_no_count(5usize, 2usize, 28u32, 2107912u32)?;
    emu.lw_no_count(6usize, 2usize, 32u32, 2107916u32)?;
    emu.lw_no_count(14usize, 2usize, 36u32, 2107920u32)?;
    emu.adi_no_count(12usize, 12usize, 4294967040u32, 2107924u32);
    emu.sri_no_count(7usize, 11usize, 8u32, 2107928u32);
    emu.sri_no_count(28usize, 11usize, 24u32, 2107932u32);
    emu.anr_no_count(29usize, 11usize, 12usize, 2107936u32);
    emu.sli_no_count(11usize, 11usize, 24u32, 2107940u32);
    emu.sri_no_count(30usize, 13usize, 8u32, 2107944u32);
    emu.sri_no_count(31usize, 13usize, 24u32, 2107948u32);
    emu.anr_no_count(9usize, 13usize, 12usize, 2107952u32);
    emu.sli_no_count(13usize, 13usize, 24u32, 2107956u32);
    emu.sri_no_count(18usize, 15usize, 8u32, 2107960u32);
    emu.sri_no_count(19usize, 15usize, 24u32, 2107964u32);
    emu.anr_no_count(20usize, 15usize, 12usize, 2107968u32);
    emu.sli_no_count(15usize, 15usize, 24u32, 2107972u32);
    emu.anr_no_count(7usize, 7usize, 12usize, 2107976u32);
    emu.orr_no_count(7usize, 7usize, 28usize, 2107980u32);
    emu.sri_no_count(28usize, 16usize, 8u32, 2107984u32);
    emu.sli_no_count(29usize, 29usize, 8u32, 2107988u32);
    emu.orr_no_count(11usize, 11usize, 29usize, 2107992u32);
    emu.sri_no_count(29usize, 16usize, 24u32, 2107996u32);
    emu.anr_no_count(30usize, 30usize, 12usize, 2108000u32);
    emu.orr_no_count(30usize, 30usize, 31usize, 2108004u32);
    emu.anr_no_count(31usize, 16usize, 12usize, 2108008u32);
    emu.sli_no_count(16usize, 16usize, 24u32, 2108012u32);
    emu.sli_no_count(9usize, 9usize, 8u32, 2108016u32);
    emu.orr_no_count(13usize, 13usize, 9usize, 2108020u32);
    emu.sri_no_count(9usize, 17usize, 8u32, 2108024u32);
    emu.anr_no_count(18usize, 18usize, 12usize, 2108028u32);
    emu.orr_no_count(18usize, 18usize, 19usize, 2108032u32);
    emu.sri_no_count(19usize, 17usize, 24u32, 2108036u32);
    emu.sli_no_count(20usize, 20usize, 8u32, 2108040u32);
    emu.orr_no_count(15usize, 15usize, 20usize, 2108044u32);
    emu.anr_no_count(20usize, 17usize, 12usize, 2108048u32);
    emu.sli_no_count(17usize, 17usize, 24u32, 2108052u32);
    emu.anr_no_count(28usize, 28usize, 12usize, 2108056u32);
    emu.orr_no_count(28usize, 28usize, 29usize, 2108060u32);
    emu.sri_no_count(29usize, 5usize, 8u32, 2108064u32);
    emu.sli_no_count(31usize, 31usize, 8u32, 2108068u32);
    emu.orr_no_count(16usize, 16usize, 31usize, 2108072u32);
    emu.sri_no_count(31usize, 5usize, 24u32, 2108076u32);
    emu.anr_no_count(9usize, 9usize, 12usize, 2108080u32);
    emu.orr_no_count(9usize, 9usize, 19usize, 2108084u32);
    emu.anr_no_count(19usize, 5usize, 12usize, 2108088u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2108092u32);
    emu.sli_no_count(20usize, 20usize, 8u32, 2108096u32);
    emu.orr_no_count(17usize, 17usize, 20usize, 2108100u32);
    emu.sri_no_count(20usize, 6usize, 8u32, 2108104u32);
    emu.anr_no_count(29usize, 29usize, 12usize, 2108108u32);
    emu.orr_no_count(29usize, 29usize, 31usize, 2108112u32);
    emu.sri_no_count(31usize, 6usize, 24u32, 2108116u32);
    emu.sli_no_count(19usize, 19usize, 8u32, 2108120u32);
    emu.orr_no_count(5usize, 5usize, 19usize, 2108124u32);
    emu.anr_no_count(19usize, 6usize, 12usize, 2108128u32);
    emu.sli_no_count(6usize, 6usize, 24u32, 2108132u32);
    emu.anr_no_count(20usize, 20usize, 12usize, 2108136u32);
    emu.orr_no_count(31usize, 20usize, 31usize, 2108140u32);
    emu.sri_no_count(20usize, 14usize, 8u32, 2108144u32);
    emu.sli_no_count(19usize, 19usize, 8u32, 2108148u32);
    emu.orr_no_count(6usize, 6usize, 19usize, 2108152u32);
    emu.sri_no_count(19usize, 14usize, 24u32, 2108156u32);
    emu.anr_no_count(20usize, 20usize, 12usize, 2108160u32);
    emu.orr_no_count(19usize, 20usize, 19usize, 2108164u32);
    emu.anr_no_count(12usize, 14usize, 12usize, 2108168u32);
    emu.sli_no_count(14usize, 14usize, 24u32, 2108172u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2108176u32);
    emu.orr_no_count(20usize, 14usize, 12usize, 2108180u32);
    emu.orr_no_count(11usize, 11usize, 7usize, 2108184u32);
    emu.orr_no_count(12usize, 13usize, 30usize, 2108188u32);
    emu.orr_no_count(13usize, 15usize, 18usize, 2108192u32);
    emu.orr_no_count(14usize, 16usize, 28usize, 2108196u32);
    emu.orr_no_count(15usize, 17usize, 9usize, 2108200u32);
    emu.orr_no_count(16usize, 5usize, 29usize, 2108204u32);
    emu.orr_no_count(17usize, 6usize, 31usize, 2108208u32);
    emu.adi_no_count(5usize, 0usize, 16u32, 2108212u32);
    emu.orr_no_count(6usize, 20usize, 19usize, 2108216u32);
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
pub fn block_0x00202b3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2108224u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2108228u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2108232u32);
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
pub fn block_0x00202b4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2108240u32);
    emu.adi_no_count(10usize, 0usize, 2u32, 2108244u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2108248u32);
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
pub fn block_0x00202b5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2108256u32);
    emu.adi_no_count(10usize, 0usize, 3u32, 2108260u32);
    emu.adi_no_count(11usize, 14usize, 0u32, 2108264u32);
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
pub fn block_0x00202b6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2108272u32);
    emu.adi_no_count(10usize, 0usize, 4u32, 2108276u32);
    emu.adi_no_count(11usize, 15usize, 0u32, 2108280u32);
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
pub fn block_0x00202b7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2108288u32);
    emu.adi_no_count(10usize, 0usize, 5u32, 2108292u32);
    emu.adi_no_count(11usize, 16usize, 0u32, 2108296u32);
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
pub fn block_0x00202b8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2108304u32);
    emu.adi_no_count(10usize, 0usize, 6u32, 2108308u32);
    emu.adi_no_count(11usize, 17usize, 0u32, 2108312u32);
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
pub fn block_0x00202b9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2108320u32);
    emu.adi_no_count(10usize, 0usize, 7u32, 2108324u32);
    emu.adi_no_count(11usize, 6usize, 0u32, 2108328u32);
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
pub fn block_0x00202bac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2108336u32;
    emu.update_insn_clock();
    emu.lw_no_count(10usize, 10usize, 4294966280u32, 2108340u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2108872u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202dc8));
    } else {
        emu.pc = 2108344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202bb8));
    }
}
#[inline]
pub fn block_0x00202bb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2108348u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2108352u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966280u32, 2108356u32);
    let a = 0u32.wrapping_add(2164260864u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2108360u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 14usize, 4u32, 2108364u32)?;
    let a = 0u32.wrapping_add(2130706432u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2108368u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1u32, 2108372u32);
    emu.adi_no_count(12usize, 12usize, 1u32, 2108376u32);
    emu.mul_no_count(15usize, 11usize, 13usize, 2108380u32);
    emu.mulhu_no_count(16usize, 15usize, 12usize, 2108384u32);
    emu.mul_no_count(15usize, 15usize, 12usize, 2108388u32);
    emu.sltru_no_count(11usize, 11usize, 15usize, 2108392u32);
    emu.sltru_no_count(15usize, 0usize, 16usize, 2108396u32);
    emu.adr_no_count(16usize, 16usize, 11usize, 2108400u32);
    emu.orr_no_count(11usize, 11usize, 15usize, 2108404u32);
    emu.sbr_no_count(11usize, 0usize, 11usize, 2108408u32);
    emu.anr_no_count(11usize, 11usize, 12usize, 2108412u32);
    emu.sbr_no_count(11usize, 11usize, 16usize, 2108416u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2108420u32);
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
pub fn block_0x00202c08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 8u32, 2108428u32)?;
    emu.mul_no_count(11usize, 10usize, 13usize, 2108432u32);
    emu.mulhu_no_count(15usize, 11usize, 12usize, 2108436u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2108440u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2108444u32);
    emu.sltru_no_count(11usize, 0usize, 15usize, 2108448u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2108452u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2108456u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2108460u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2108464u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2108468u32);
    emu.sbr_no_count(11usize, 10usize, 15usize, 2108472u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2108476u32);
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
pub fn block_0x00202c40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 12u32, 2108484u32)?;
    emu.mul_no_count(11usize, 10usize, 13usize, 2108488u32);
    emu.mulhu_no_count(15usize, 11usize, 12usize, 2108492u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2108496u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2108500u32);
    emu.sltru_no_count(11usize, 0usize, 15usize, 2108504u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2108508u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2108512u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2108516u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2108520u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2108524u32);
    emu.sbr_no_count(11usize, 10usize, 15usize, 2108528u32);
    emu.adi_no_count(10usize, 0usize, 2u32, 2108532u32);
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
pub fn block_0x00202c78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 16u32, 2108540u32)?;
    emu.mul_no_count(11usize, 10usize, 13usize, 2108544u32);
    emu.mulhu_no_count(15usize, 11usize, 12usize, 2108548u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2108552u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2108556u32);
    emu.sltru_no_count(11usize, 0usize, 15usize, 2108560u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2108564u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2108568u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2108572u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2108576u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2108580u32);
    emu.sbr_no_count(11usize, 10usize, 15usize, 2108584u32);
    emu.adi_no_count(10usize, 0usize, 3u32, 2108588u32);
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
pub fn block_0x00202cb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 20u32, 2108596u32)?;
    emu.mul_no_count(11usize, 10usize, 13usize, 2108600u32);
    emu.mulhu_no_count(15usize, 11usize, 12usize, 2108604u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2108608u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2108612u32);
    emu.sltru_no_count(11usize, 0usize, 15usize, 2108616u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2108620u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2108624u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2108628u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2108632u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2108636u32);
    emu.sbr_no_count(11usize, 10usize, 15usize, 2108640u32);
    emu.adi_no_count(10usize, 0usize, 4u32, 2108644u32);
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
pub fn block_0x00202ce8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 24u32, 2108652u32)?;
    emu.mul_no_count(11usize, 10usize, 13usize, 2108656u32);
    emu.mulhu_no_count(15usize, 11usize, 12usize, 2108660u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2108664u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2108668u32);
    emu.sltru_no_count(11usize, 0usize, 15usize, 2108672u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2108676u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2108680u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2108684u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2108688u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2108692u32);
    emu.sbr_no_count(11usize, 10usize, 15usize, 2108696u32);
    emu.adi_no_count(10usize, 0usize, 5u32, 2108700u32);
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
pub fn block_0x00202d20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 28u32, 2108708u32)?;
    emu.mul_no_count(11usize, 10usize, 13usize, 2108712u32);
    emu.mulhu_no_count(15usize, 11usize, 12usize, 2108716u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2108720u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2108724u32);
    emu.sltru_no_count(11usize, 0usize, 15usize, 2108728u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2108732u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2108736u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2108740u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2108744u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2108748u32);
    emu.sbr_no_count(11usize, 10usize, 15usize, 2108752u32);
    emu.adi_no_count(10usize, 0usize, 6u32, 2108756u32);
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
pub fn block_0x00202d58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 32u32, 2108764u32)?;
    emu.adi_no_count(5usize, 0usize, 26u32, 2108768u32);
    emu.mul_no_count(11usize, 10usize, 13usize, 2108772u32);
    emu.mulhu_no_count(13usize, 11usize, 12usize, 2108776u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2108780u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2108784u32);
    emu.sltru_no_count(11usize, 0usize, 13usize, 2108788u32);
    emu.adr_no_count(13usize, 13usize, 10usize, 2108792u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2108796u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2108800u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2108804u32);
    emu.sbr_no_count(11usize, 10usize, 13usize, 2108808u32);
    emu.adi_no_count(10usize, 0usize, 7u32, 2108812u32);
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
pub fn block_0x00202d90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 0u32, 2108820u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2108824u32);
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
pub fn block_0x00202d9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2108832u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 932u32, 2108836u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2108840u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1004u32, 2108844u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2108848u32);
    emu.apc_no_count(1usize, 2108848u32, 36864u32, 2108852u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108856u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1732u32);
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
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2108860u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 972u32, 2108864u32);
    emu.apc_no_count(1usize, 2108864u32, 45056u32, 2108868u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108872u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1152u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202dc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2108876u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 988u32, 2108880u32);
    emu.apc_no_count(1usize, 2108880u32, 45056u32, 2108884u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108888u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1136u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00202dd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2108892u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2108896u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2108900u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2108904u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2108908u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2108912u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2108916u32)?;
    emu.adi_no_count(8usize, 12usize, 0u32, 2108920u32);
    emu.adi_no_count(5usize, 0usize, 2u32, 2108924u32);
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
pub fn block_0x00202e00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 3u32, 2108932u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2109168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202ef0));
    } else {
        emu.pc = 2108936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202e08));
    }
}
#[inline(always)]
pub fn block_0x00202e08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2108940u32;
    emu.update_insn_clock();
    emu.lw_no_count(10usize, 10usize, 4294966320u32, 2108944u32)?;
    emu.ani_no_count(10usize, 10usize, 1u32, 2108948u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2109200u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202f10));
    } else {
        emu.pc = 2108952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202e18));
    }
}
#[inline(always)]
pub fn block_0x00202e18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(18usize, a);
    emu.pc = 2108956u32;
    emu.update_insn_clock();
    emu.adi_no_count(18usize, 18usize, 4294966320u32, 2108960u32);
    emu.lbu_no_count(19usize, 18usize, 112u32, 2108964u32);
    emu.adi_no_count(10usize, 0usize, 64u32, 2108968u32);
    emu.sbr_no_count(12usize, 10usize, 19usize, 2108972u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a >= b {
        emu.pc = 2109004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202e4c));
    } else {
        emu.pc = 2108976u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202e30));
    }
}
#[inline(always)]
pub fn block_0x00202e30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 18usize, 19usize, 2108980u32);
    emu.adi_no_count(10usize, 10usize, 48u32, 2108984u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2108988u32);
    emu.apc_no_count(1usize, 2108988u32, 0u32, 2108992u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108996u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(568u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202e44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(9usize, 8usize, 19usize, 2109000u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2109004u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2109164u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202eec));
}
#[inline(always)]
pub fn block_0x00202e4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2109084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202e9c));
    } else {
        emu.pc = 2109008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202e50));
    }
}
#[inline(always)]
pub fn block_0x00202e50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(8usize, 8usize, 12usize, 2109012u32);
    emu.adr_no_count(20usize, 11usize, 12usize, 2109016u32);
    emu.adi_no_count(9usize, 18usize, 48u32, 2109020u32);
    emu.adr_no_count(10usize, 9usize, 19usize, 2109024u32);
    emu.apc_no_count(1usize, 2109024u32, 0u32, 2109028u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109032u32;
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
#[inline]
pub fn block_0x00202e68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 40u32, 2109036u32)?;
    emu.lw_no_count(11usize, 18usize, 44u32, 2109040u32)?;
    emu.adi_no_count(10usize, 10usize, 1u32, 2109044u32);
    emu.sltiu_no_count(12usize, 10usize, 1u32, 2109048u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2109052u32);
    emu.sw_no_count(10usize, 18usize, 40u32, 2109056u32)?;
    emu.sw_no_count(11usize, 18usize, 44u32, 2109060u32)?;
    emu.adi_no_count(10usize, 18usize, 8u32, 2109064u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2109068u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2109072u32);
    emu.apc_no_count(1usize, 2109072u32, 4096u32, 2109076u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109080u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965768u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202e98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 20usize, 0u32, 2109084u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2109084u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202e9c));
}
#[inline(always)]
pub fn block_0x00202e9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 8usize, 4294967232u32, 2109088u32);
    emu.ani_no_count(9usize, 8usize, 63u32, 2109092u32);
    emu.sri_no_count(12usize, 8usize, 6u32, 2109096u32);
    emu.adr_no_count(8usize, 11usize, 10usize, 2109100u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2109144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202ed8));
    } else {
        emu.pc = 2109104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202eb0));
    }
}
#[inline]
pub fn block_0x00202eb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 40u32, 2109108u32)?;
    emu.lw_no_count(13usize, 18usize, 44u32, 2109112u32)?;
    emu.adr_no_count(14usize, 10usize, 12usize, 2109116u32);
    emu.sltru_no_count(10usize, 14usize, 10usize, 2109120u32);
    emu.adr_no_count(10usize, 13usize, 10usize, 2109124u32);
    emu.sw_no_count(14usize, 18usize, 40u32, 2109128u32)?;
    emu.sw_no_count(10usize, 18usize, 44u32, 2109132u32)?;
    emu.adi_no_count(10usize, 18usize, 8u32, 2109136u32);
    emu.apc_no_count(1usize, 2109136u32, 4096u32, 2109140u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109144u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x00202ed8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 48u32, 2109148u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2109152u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2109156u32);
    emu.apc_no_count(1usize, 2109156u32, 0u32, 2109160u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109164u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(400u32);
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
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(9usize, 18usize, 112u32, 2109168u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2109168u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202ef0));
}
#[inline(always)]
pub fn block_0x00202ef0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2109172u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2109176u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2109180u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2109184u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2109188u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2109192u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2109196u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109200u32;
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
pub fn block_0x00202f10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2109204u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1020u32, 2109208u32);
    emu.apc_no_count(1usize, 2109208u32, 45056u32, 2109212u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109216u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(808u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202f20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 240u32, 2109220u32);
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
pub fn block_0x00202f28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 5usize, 0u32, 2109228u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109232u32;
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
pub fn block_0x00202f30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 241u32, 2109236u32);
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
pub fn block_0x00202f38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109244u32;
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
pub fn block_0x00202f3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2109248u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2109252u32)?;
    emu.adi_no_count(12usize, 11usize, 0u32, 2109256u32);
    emu.adi_no_count(11usize, 10usize, 0u32, 2109260u32);
    emu.adi_no_count(10usize, 0usize, 2u32, 2109264u32);
    emu.apc_no_count(1usize, 2109264u32, 0u32, 2109268u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109272u32;
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
pub fn block_0x00202f58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2109276u32);
    emu.apc_no_count(1usize, 2109276u32, 0u32, 2109280u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109284u32;
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
pub fn block_0x00202f64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2109284u32, 0u32, 2109288u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2109292u32;
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
pub fn block_0x00202f6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2109296u32;
    emu.update_insn_clock();
    emu.adi_no_count(5usize, 12usize, 262u32, 2109300u32);
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
pub fn block_0x00202f78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109308u32;
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
pub fn block_0x00202f7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2109552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203070));
    } else {
        emu.pc = 2109312u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202f80));
    }
}
#[inline(always)]
pub fn block_0x00202f80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 10usize, 0u32, 2109316u32);
    emu.adr_no_count(13usize, 12usize, 10usize, 2109320u32);
    emu.adi_no_count(14usize, 0usize, 3u32, 2109324u32);
    emu.sb_no_count(11usize, 13usize, 4294967295u32, 2109328u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2109552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203070));
    } else {
        emu.pc = 2109332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202f94));
    }
}
#[inline(always)]
pub fn block_0x00202f94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 10usize, 1u32, 2109336u32);
    emu.sb_no_count(11usize, 10usize, 2u32, 2109340u32);
    emu.sb_no_count(11usize, 13usize, 4294967294u32, 2109344u32);
    emu.adi_no_count(14usize, 0usize, 7u32, 2109348u32);
    emu.sb_no_count(11usize, 13usize, 4294967293u32, 2109352u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2109552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203070));
    } else {
        emu.pc = 2109356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202fac));
    }
}
#[inline(always)]
pub fn block_0x00202fac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 10usize, 3u32, 2109360u32);
    emu.adi_no_count(15usize, 0usize, 9u32, 2109364u32);
    emu.sb_no_count(11usize, 13usize, 4294967292u32, 2109368u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2109552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203070));
    } else {
        emu.pc = 2109372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202fbc));
    }
}
#[inline]
pub fn block_0x00202fbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(13usize, 0usize, 10usize, 2109376u32);
    emu.ani_no_count(14usize, 13usize, 3u32, 2109380u32);
    emu.adr_no_count(13usize, 10usize, 14usize, 2109384u32);
    emu.sbr_no_count(12usize, 12usize, 14usize, 2109388u32);
    emu.ani_no_count(12usize, 12usize, 4294967292u32, 2109392u32);
    emu.ani_no_count(11usize, 11usize, 255u32, 2109396u32);
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2109400u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 257u32, 2109404u32);
    emu.mul_no_count(11usize, 11usize, 14usize, 2109408u32);
    emu.sw_no_count(11usize, 13usize, 0u32, 2109412u32)?;
    emu.adr_no_count(14usize, 13usize, 12usize, 2109416u32);
    emu.sw_no_count(11usize, 14usize, 4294967292u32, 2109420u32)?;
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2109552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203070));
    } else {
        emu.pc = 2109424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202ff0));
    }
}
#[inline(always)]
pub fn block_0x00202ff0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 13usize, 4u32, 2109428u32)?;
    emu.sw_no_count(11usize, 13usize, 8u32, 2109432u32)?;
    emu.sw_no_count(11usize, 14usize, 4294967284u32, 2109436u32)?;
    emu.adi_no_count(15usize, 0usize, 25u32, 2109440u32);
    emu.sw_no_count(11usize, 14usize, 4294967288u32, 2109444u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2109552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203070));
    } else {
        emu.pc = 2109448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203008));
    }
}
#[inline]
pub fn block_0x00203008(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 13usize, 12u32, 2109452u32)?;
    emu.sw_no_count(11usize, 13usize, 16u32, 2109456u32)?;
    emu.sw_no_count(11usize, 13usize, 20u32, 2109460u32)?;
    emu.sw_no_count(11usize, 13usize, 24u32, 2109464u32)?;
    emu.sw_no_count(11usize, 14usize, 4294967268u32, 2109468u32)?;
    emu.sw_no_count(11usize, 14usize, 4294967272u32, 2109472u32)?;
    emu.sw_no_count(11usize, 14usize, 4294967276u32, 2109476u32)?;
    emu.ani_no_count(15usize, 13usize, 4u32, 2109480u32);
    emu.ori_no_count(15usize, 15usize, 24u32, 2109484u32);
    emu.sbr_no_count(12usize, 12usize, 15usize, 2109488u32);
    emu.adi_no_count(16usize, 0usize, 32u32, 2109492u32);
    emu.sw_no_count(11usize, 14usize, 4294967280u32, 2109496u32)?;
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2109552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203070));
    } else {
        emu.pc = 2109500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020303c));
    }
}
#[inline(always)]
pub fn block_0x0020303c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(13usize, 13usize, 15usize, 2109504u32);
    emu.adi_no_count(14usize, 0usize, 31u32, 2109508u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2109508u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203044));
}
#[inline]
pub fn block_0x00203044(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 13usize, 0u32, 2109512u32)?;
    emu.sw_no_count(11usize, 13usize, 4u32, 2109516u32)?;
    emu.sw_no_count(11usize, 13usize, 8u32, 2109520u32)?;
    emu.sw_no_count(11usize, 13usize, 12u32, 2109524u32)?;
    emu.sw_no_count(11usize, 13usize, 16u32, 2109528u32)?;
    emu.sw_no_count(11usize, 13usize, 20u32, 2109532u32)?;
    emu.sw_no_count(11usize, 13usize, 24u32, 2109536u32)?;
    emu.sw_no_count(11usize, 13usize, 28u32, 2109540u32)?;
    emu.adi_no_count(12usize, 12usize, 4294967264u32, 2109544u32);
    emu.adi_no_count(13usize, 13usize, 32u32, 2109548u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2109508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203044));
    } else {
        emu.pc = 2109552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203070));
    }
}
#[inline(always)]
pub fn block_0x00203070(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109556u32;
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
pub fn block_0x00203074(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(13usize, 11usize, 3u32, 2109560u32);
    emu.sltiu_no_count(13usize, 13usize, 1u32, 2109564u32);
    emu.sltiu_no_count(14usize, 12usize, 1u32, 2109568u32);
    emu.orr_no_count(13usize, 13usize, 14usize, 2109572u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2109820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020317c));
    } else {
        emu.pc = 2109576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203088));
    }
}
#[inline(always)]
pub fn block_0x00203088(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 11usize, 1u32, 2109580u32);
    emu.adi_no_count(16usize, 10usize, 0u32, 2109584u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2109584u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203090));
}
#[inline]
pub fn block_0x00203090(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(17usize, 11usize, 0u32, 2109588u32);
    emu.adi_no_count(14usize, 11usize, 1u32, 2109592u32);
    emu.adi_no_count(13usize, 16usize, 1u32, 2109596u32);
    emu.sb_no_count(17usize, 16usize, 0u32, 2109600u32);
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2109604u32);
    emu.ani_no_count(11usize, 15usize, 3u32, 2109608u32);
    emu.sltru_no_count(11usize, 0usize, 11usize, 2109612u32);
    emu.sltru_no_count(16usize, 0usize, 12usize, 2109616u32);
    emu.anr_no_count(17usize, 11usize, 16usize, 2109620u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2109624u32);
    emu.adi_no_count(11usize, 14usize, 0u32, 2109628u32);
    emu.adi_no_count(16usize, 13usize, 0u32, 2109632u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a != b {
        emu.pc = 2109584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203090));
    } else {
        emu.pc = 2109636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002030c4));
    }
}
#[inline(always)]
pub fn block_0x002030c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 13usize, 3u32, 2109640u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2109836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020318c));
    } else {
        emu.pc = 2109644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002030cc));
    }
}
#[inline(always)]
pub fn block_0x002030cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 32u32, 2109648u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2110220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020330c));
    } else {
        emu.pc = 2109652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002030d4));
    }
}
#[inline(always)]
pub fn block_0x002030d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 3u32, 2109656u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2109956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203204));
    } else {
        emu.pc = 2109660u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002030dc));
    }
}
#[inline(always)]
pub fn block_0x002030dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 2u32, 2109664u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2110084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203284));
    } else {
        emu.pc = 2109668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002030e4));
    }
}
#[inline(always)]
pub fn block_0x002030e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 1u32, 2109672u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2110220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020330c));
    } else {
        emu.pc = 2109676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002030ec));
    }
}
#[inline]
pub fn block_0x002030ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 14usize, 0u32, 2109680u32)?;
    emu.sb_no_count(15usize, 13usize, 0u32, 2109684u32);
    emu.sri_no_count(11usize, 15usize, 8u32, 2109688u32);
    emu.sb_no_count(11usize, 13usize, 1u32, 2109692u32);
    emu.sri_no_count(16usize, 15usize, 16u32, 2109696u32);
    emu.adi_no_count(11usize, 13usize, 3u32, 2109700u32);
    emu.sb_no_count(16usize, 13usize, 2u32, 2109704u32);
    emu.adi_no_count(12usize, 12usize, 4294967293u32, 2109708u32);
    emu.adi_no_count(13usize, 14usize, 16u32, 2109712u32);
    emu.adi_no_count(14usize, 0usize, 16u32, 2109716u32);
    emu.add_memory_rw_events(10usize);
    emu.pc = 2109716u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203114));
}
#[inline]
pub fn block_0x00203114(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 13usize, 4294967284u32, 2109720u32)?;
    emu.sri_no_count(15usize, 15usize, 24u32, 2109724u32);
    emu.sli_no_count(17usize, 16usize, 8u32, 2109728u32);
    emu.lw_no_count(5usize, 13usize, 4294967288u32, 2109732u32)?;
    emu.orr_no_count(15usize, 17usize, 15usize, 2109736u32);
    emu.sw_no_count(15usize, 11usize, 0u32, 2109740u32)?;
    emu.sri_no_count(15usize, 16usize, 24u32, 2109744u32);
    emu.sli_no_count(16usize, 5usize, 8u32, 2109748u32);
    emu.lw_no_count(17usize, 13usize, 4294967292u32, 2109752u32)?;
    emu.orr_no_count(15usize, 16usize, 15usize, 2109756u32);
    emu.sw_no_count(15usize, 11usize, 4u32, 2109760u32)?;
    emu.sri_no_count(16usize, 5usize, 24u32, 2109764u32);
    emu.sli_no_count(5usize, 17usize, 8u32, 2109768u32);
    emu.lw_no_count(15usize, 13usize, 0u32, 2109772u32)?;
    emu.orr_no_count(16usize, 5usize, 16usize, 2109776u32);
    emu.sw_no_count(16usize, 11usize, 8u32, 2109780u32)?;
    emu.sri_no_count(16usize, 17usize, 24u32, 2109784u32);
    emu.sli_no_count(17usize, 15usize, 8u32, 2109788u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2109792u32);
    emu.sw_no_count(16usize, 11usize, 12u32, 2109796u32)?;
    emu.adi_no_count(11usize, 11usize, 16u32, 2109800u32);
    emu.adi_no_count(12usize, 12usize, 4294967280u32, 2109804u32);
    emu.adi_no_count(13usize, 13usize, 16u32, 2109808u32);
    emu.add_memory_rw_events(23usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2109716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203114));
    } else {
        emu.pc = 2109812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203174));
    }
}
#[inline(always)]
pub fn block_0x00203174(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 13usize, 4294967283u32, 2109816u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2109820u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2110216u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203308));
}
#[inline(always)]
pub fn block_0x0020317c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 10usize, 0u32, 2109824u32);
    emu.adi_no_count(14usize, 11usize, 0u32, 2109828u32);
    emu.ani_no_count(11usize, 13usize, 3u32, 2109832u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2109644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002030cc));
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
    emu.adi_no_count(11usize, 0usize, 16u32, 2109840u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2109896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002031c8));
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
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 15u32, 2109848u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2109848u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203198));
}
#[inline]
pub fn block_0x00203198(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 14usize, 0u32, 2109852u32)?;
    emu.lw_no_count(16usize, 14usize, 4u32, 2109856u32)?;
    emu.lw_no_count(17usize, 14usize, 8u32, 2109860u32)?;
    emu.lw_no_count(5usize, 14usize, 12u32, 2109864u32)?;
    emu.sw_no_count(15usize, 13usize, 0u32, 2109868u32)?;
    emu.sw_no_count(16usize, 13usize, 4u32, 2109872u32)?;
    emu.sw_no_count(17usize, 13usize, 8u32, 2109876u32)?;
    emu.sw_no_count(5usize, 13usize, 12u32, 2109880u32)?;
    emu.adi_no_count(14usize, 14usize, 16u32, 2109884u32);
    emu.adi_no_count(12usize, 12usize, 4294967280u32, 2109888u32);
    emu.adi_no_count(13usize, 13usize, 16u32, 2109892u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2109848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203198));
    } else {
        emu.pc = 2109896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002031c8));
    }
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
    emu.ani_no_count(11usize, 12usize, 8u32, 2109900u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2109928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002031e8));
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
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 14usize, 0u32, 2109908u32)?;
    emu.lw_no_count(15usize, 14usize, 4u32, 2109912u32)?;
    emu.sw_no_count(11usize, 13usize, 0u32, 2109916u32)?;
    emu.sw_no_count(15usize, 13usize, 4u32, 2109920u32)?;
    emu.adi_no_count(13usize, 13usize, 8u32, 2109924u32);
    emu.adi_no_count(14usize, 14usize, 8u32, 2109928u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2109928u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002031e8));
}
#[inline(always)]
pub fn block_0x002031e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 12usize, 4u32, 2109932u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2110288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203350));
    } else {
        emu.pc = 2109936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002031f0));
    }
}
#[inline(always)]
pub fn block_0x002031f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 14usize, 0u32, 2109940u32)?;
    emu.sw_no_count(11usize, 13usize, 0u32, 2109944u32)?;
    emu.adi_no_count(13usize, 13usize, 4u32, 2109948u32);
    emu.adi_no_count(14usize, 14usize, 4u32, 2109952u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2109956u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2110288u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203350));
}
#[inline(always)]
pub fn block_0x00203204(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 14usize, 0u32, 2109960u32)?;
    emu.adi_no_count(11usize, 13usize, 1u32, 2109964u32);
    emu.sb_no_count(15usize, 13usize, 0u32, 2109968u32);
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2109972u32);
    emu.adi_no_count(13usize, 14usize, 16u32, 2109976u32);
    emu.adi_no_count(14usize, 0usize, 18u32, 2109980u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2109980u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020321c));
}
#[inline]
pub fn block_0x0020321c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 13usize, 4294967284u32, 2109984u32)?;
    emu.sri_no_count(15usize, 15usize, 8u32, 2109988u32);
    emu.sli_no_count(17usize, 16usize, 24u32, 2109992u32);
    emu.lw_no_count(5usize, 13usize, 4294967288u32, 2109996u32)?;
    emu.orr_no_count(15usize, 17usize, 15usize, 2110000u32);
    emu.sw_no_count(15usize, 11usize, 0u32, 2110004u32)?;
    emu.sri_no_count(15usize, 16usize, 8u32, 2110008u32);
    emu.sli_no_count(16usize, 5usize, 24u32, 2110012u32);
    emu.lw_no_count(17usize, 13usize, 4294967292u32, 2110016u32)?;
    emu.orr_no_count(15usize, 16usize, 15usize, 2110020u32);
    emu.sw_no_count(15usize, 11usize, 4u32, 2110024u32)?;
    emu.sri_no_count(16usize, 5usize, 8u32, 2110028u32);
    emu.sli_no_count(5usize, 17usize, 24u32, 2110032u32);
    emu.lw_no_count(15usize, 13usize, 0u32, 2110036u32)?;
    emu.orr_no_count(16usize, 5usize, 16usize, 2110040u32);
    emu.sw_no_count(16usize, 11usize, 8u32, 2110044u32)?;
    emu.sri_no_count(16usize, 17usize, 8u32, 2110048u32);
    emu.sli_no_count(17usize, 15usize, 24u32, 2110052u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2110056u32);
    emu.sw_no_count(16usize, 11usize, 12u32, 2110060u32)?;
    emu.adi_no_count(11usize, 11usize, 16u32, 2110064u32);
    emu.adi_no_count(12usize, 12usize, 4294967280u32, 2110068u32);
    emu.adi_no_count(13usize, 13usize, 16u32, 2110072u32);
    emu.add_memory_rw_events(23usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2109980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020321c));
    } else {
        emu.pc = 2110076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020327c));
    }
}
#[inline(always)]
pub fn block_0x0020327c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 13usize, 4294967281u32, 2110080u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2110084u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2110216u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203308));
}
#[inline(always)]
pub fn block_0x00203284(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 14usize, 0u32, 2110088u32)?;
    emu.sb_no_count(15usize, 13usize, 0u32, 2110092u32);
    emu.sri_no_count(16usize, 15usize, 8u32, 2110096u32);
    emu.adi_no_count(11usize, 13usize, 2u32, 2110100u32);
    emu.sb_no_count(16usize, 13usize, 1u32, 2110104u32);
    emu.adi_no_count(12usize, 12usize, 4294967294u32, 2110108u32);
    emu.adi_no_count(13usize, 14usize, 16u32, 2110112u32);
    emu.adi_no_count(14usize, 0usize, 17u32, 2110116u32);
    emu.add_memory_rw_events(8usize);
    emu.pc = 2110116u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002032a4));
}
#[inline]
pub fn block_0x002032a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 13usize, 4294967284u32, 2110120u32)?;
    emu.sri_no_count(15usize, 15usize, 16u32, 2110124u32);
    emu.sli_no_count(17usize, 16usize, 16u32, 2110128u32);
    emu.lw_no_count(5usize, 13usize, 4294967288u32, 2110132u32)?;
    emu.orr_no_count(15usize, 17usize, 15usize, 2110136u32);
    emu.sw_no_count(15usize, 11usize, 0u32, 2110140u32)?;
    emu.sri_no_count(15usize, 16usize, 16u32, 2110144u32);
    emu.sli_no_count(16usize, 5usize, 16u32, 2110148u32);
    emu.lw_no_count(17usize, 13usize, 4294967292u32, 2110152u32)?;
    emu.orr_no_count(15usize, 16usize, 15usize, 2110156u32);
    emu.sw_no_count(15usize, 11usize, 4u32, 2110160u32)?;
    emu.sri_no_count(16usize, 5usize, 16u32, 2110164u32);
    emu.sli_no_count(5usize, 17usize, 16u32, 2110168u32);
    emu.lw_no_count(15usize, 13usize, 0u32, 2110172u32)?;
    emu.orr_no_count(16usize, 5usize, 16usize, 2110176u32);
    emu.sw_no_count(16usize, 11usize, 8u32, 2110180u32)?;
    emu.sri_no_count(16usize, 17usize, 16u32, 2110184u32);
    emu.sli_no_count(17usize, 15usize, 16u32, 2110188u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2110192u32);
    emu.sw_no_count(16usize, 11usize, 12u32, 2110196u32)?;
    emu.adi_no_count(11usize, 11usize, 16u32, 2110200u32);
    emu.adi_no_count(12usize, 12usize, 4294967280u32, 2110204u32);
    emu.adi_no_count(13usize, 13usize, 16u32, 2110208u32);
    emu.add_memory_rw_events(23usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2110116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002032a4));
    } else {
        emu.pc = 2110212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203304));
    }
}
#[inline(always)]
pub fn block_0x00203304(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 13usize, 4294967282u32, 2110216u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2110216u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203308));
}
#[inline(always)]
pub fn block_0x00203308(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 11usize, 0u32, 2110220u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2110220u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020330c));
}
#[inline(always)]
pub fn block_0x0020330c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 12usize, 16u32, 2110224u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2110356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203394));
    } else {
        emu.pc = 2110228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203314));
    }
}
#[inline(always)]
pub fn block_0x00203314(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 12usize, 8u32, 2110232u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2110504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203428));
    } else {
        emu.pc = 2110236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020331c));
    }
}
#[inline(always)]
pub fn block_0x0020331c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 12usize, 4u32, 2110240u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2110288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203350));
    } else {
        emu.pc = 2110244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203324));
    }
}
#[inline]
pub fn block_0x00203324(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(11usize, 14usize, 0u32, 2110248u32);
    emu.lb_no_count(15usize, 14usize, 1u32, 2110252u32);
    emu.lb_no_count(16usize, 14usize, 2u32, 2110256u32);
    emu.sb_no_count(11usize, 13usize, 0u32, 2110260u32);
    emu.sb_no_count(15usize, 13usize, 1u32, 2110264u32);
    emu.lb_no_count(11usize, 14usize, 3u32, 2110268u32);
    emu.sb_no_count(16usize, 13usize, 2u32, 2110272u32);
    emu.adi_no_count(14usize, 14usize, 4u32, 2110276u32);
    emu.adi_no_count(15usize, 13usize, 4u32, 2110280u32);
    emu.sb_no_count(11usize, 13usize, 3u32, 2110284u32);
    emu.adi_no_count(13usize, 15usize, 0u32, 2110288u32);
    emu.add_memory_rw_events(11usize);
    emu.pc = 2110288u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203350));
}
#[inline(always)]
pub fn block_0x00203350(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 12usize, 2u32, 2110292u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
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
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 12usize, 1u32, 2110300u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2110344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203388));
    } else {
        emu.pc = 2110304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203360));
    }
}
#[inline(always)]
pub fn block_0x00203360(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
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
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(11usize, 14usize, 0u32, 2110312u32);
    emu.lb_no_count(15usize, 14usize, 1u32, 2110316u32);
    emu.sb_no_count(11usize, 13usize, 0u32, 2110320u32);
    emu.adi_no_count(14usize, 14usize, 2u32, 2110324u32);
    emu.adi_no_count(11usize, 13usize, 2u32, 2110328u32);
    emu.sb_no_count(15usize, 13usize, 1u32, 2110332u32);
    emu.adi_no_count(13usize, 11usize, 0u32, 2110336u32);
    emu.ani_no_count(11usize, 12usize, 1u32, 2110340u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2110304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203360));
    } else {
        emu.pc = 2110344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203388));
    }
}
#[inline(always)]
pub fn block_0x00203388(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(11usize, 14usize, 0u32, 2110348u32);
    emu.sb_no_count(11usize, 13usize, 0u32, 2110352u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110356u32;
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
pub fn block_0x00203394(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(11usize, 14usize, 0u32, 2110360u32);
    emu.lb_no_count(15usize, 14usize, 1u32, 2110364u32);
    emu.lb_no_count(16usize, 14usize, 2u32, 2110368u32);
    emu.sb_no_count(11usize, 13usize, 0u32, 2110372u32);
    emu.sb_no_count(15usize, 13usize, 1u32, 2110376u32);
    emu.lb_no_count(11usize, 14usize, 3u32, 2110380u32);
    emu.sb_no_count(16usize, 13usize, 2u32, 2110384u32);
    emu.lb_no_count(15usize, 14usize, 4u32, 2110388u32);
    emu.lb_no_count(16usize, 14usize, 5u32, 2110392u32);
    emu.sb_no_count(11usize, 13usize, 3u32, 2110396u32);
    emu.lb_no_count(11usize, 14usize, 6u32, 2110400u32);
    emu.sb_no_count(15usize, 13usize, 4u32, 2110404u32);
    emu.sb_no_count(16usize, 13usize, 5u32, 2110408u32);
    emu.lb_no_count(15usize, 14usize, 7u32, 2110412u32);
    emu.sb_no_count(11usize, 13usize, 6u32, 2110416u32);
    emu.lb_no_count(11usize, 14usize, 8u32, 2110420u32);
    emu.lb_no_count(16usize, 14usize, 9u32, 2110424u32);
    emu.sb_no_count(15usize, 13usize, 7u32, 2110428u32);
    emu.lb_no_count(15usize, 14usize, 10u32, 2110432u32);
    emu.sb_no_count(11usize, 13usize, 8u32, 2110436u32);
    emu.sb_no_count(16usize, 13usize, 9u32, 2110440u32);
    emu.lb_no_count(11usize, 14usize, 11u32, 2110444u32);
    emu.sb_no_count(15usize, 13usize, 10u32, 2110448u32);
    emu.lb_no_count(15usize, 14usize, 12u32, 2110452u32);
    emu.lb_no_count(16usize, 14usize, 13u32, 2110456u32);
    emu.sb_no_count(11usize, 13usize, 11u32, 2110460u32);
    emu.lb_no_count(11usize, 14usize, 14u32, 2110464u32);
    emu.sb_no_count(15usize, 13usize, 12u32, 2110468u32);
    emu.sb_no_count(16usize, 13usize, 13u32, 2110472u32);
    emu.lb_no_count(15usize, 14usize, 15u32, 2110476u32);
    emu.sb_no_count(11usize, 13usize, 14u32, 2110480u32);
    emu.adi_no_count(14usize, 14usize, 16u32, 2110484u32);
    emu.adi_no_count(11usize, 13usize, 16u32, 2110488u32);
    emu.sb_no_count(15usize, 13usize, 15u32, 2110492u32);
    emu.adi_no_count(13usize, 11usize, 0u32, 2110496u32);
    emu.ani_no_count(11usize, 12usize, 8u32, 2110500u32);
    emu.add_memory_rw_events(36usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2110236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020331c));
    } else {
        emu.pc = 2110504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203428));
    }
}
#[inline]
pub fn block_0x00203428(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(11usize, 14usize, 0u32, 2110508u32);
    emu.lb_no_count(15usize, 14usize, 1u32, 2110512u32);
    emu.lb_no_count(16usize, 14usize, 2u32, 2110516u32);
    emu.sb_no_count(11usize, 13usize, 0u32, 2110520u32);
    emu.sb_no_count(15usize, 13usize, 1u32, 2110524u32);
    emu.lb_no_count(11usize, 14usize, 3u32, 2110528u32);
    emu.sb_no_count(16usize, 13usize, 2u32, 2110532u32);
    emu.lb_no_count(15usize, 14usize, 4u32, 2110536u32);
    emu.lb_no_count(16usize, 14usize, 5u32, 2110540u32);
    emu.sb_no_count(11usize, 13usize, 3u32, 2110544u32);
    emu.lb_no_count(11usize, 14usize, 6u32, 2110548u32);
    emu.sb_no_count(15usize, 13usize, 4u32, 2110552u32);
    emu.sb_no_count(16usize, 13usize, 5u32, 2110556u32);
    emu.lb_no_count(15usize, 14usize, 7u32, 2110560u32);
    emu.sb_no_count(11usize, 13usize, 6u32, 2110564u32);
    emu.adi_no_count(14usize, 14usize, 8u32, 2110568u32);
    emu.adi_no_count(11usize, 13usize, 8u32, 2110572u32);
    emu.sb_no_count(15usize, 13usize, 7u32, 2110576u32);
    emu.adi_no_count(13usize, 11usize, 0u32, 2110580u32);
    emu.ani_no_count(11usize, 12usize, 4u32, 2110584u32);
    emu.add_memory_rw_events(20usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2110244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203324));
    } else {
        emu.pc = 2110588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020347c));
    }
}
#[inline(always)]
pub fn block_0x0020347c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2110592u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2110288u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203350));
}
