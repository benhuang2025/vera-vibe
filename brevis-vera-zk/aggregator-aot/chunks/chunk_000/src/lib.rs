pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2099200u32;
pub const PC_MAX: u32 = 2101996u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 167usize] = [
        block_0x00200800,
        block_0x00200818,
        block_0x0020082c,
        block_0x00200868,
        block_0x00200884,
        block_0x002008a8,
        block_0x002008c8,
        block_0x002008ec,
        block_0x0020090c,
        block_0x0020091c,
        block_0x0020092c,
        block_0x00200940,
        block_0x002009ac,
        block_0x002009b8,
        block_0x002009d8,
        block_0x00200a1c,
        block_0x00200a24,
        block_0x00200a44,
        block_0x00200a50,
        block_0x00200a68,
        block_0x00200a6c,
        block_0x00200a88,
        block_0x00200a8c,
        block_0x00200aa8,
        block_0x00200aac,
        block_0x00200ac8,
        block_0x00200acc,
        block_0x00200ae8,
        block_0x00200aec,
        block_0x00200b08,
        block_0x00200b14,
        block_0x00200b38,
        block_0x00200b40,
        block_0x00200b48,
        block_0x00200b68,
        block_0x00200b6c,
        block_0x00200b70,
        block_0x00200bac,
        block_0x00200bc0,
        block_0x00200bd8,
        block_0x00200be0,
        block_0x00200bf0,
        block_0x00200c00,
        block_0x00200c08,
        block_0x00200c1c,
        block_0x00200c24,
        block_0x00200c2c,
        block_0x00200c3c,
        block_0x00200c4c,
        block_0x00200c54,
        block_0x00200c68,
        block_0x00200c70,
        block_0x00200c78,
        block_0x00200c88,
        block_0x00200c90,
        block_0x00200ca4,
        block_0x00200cac,
        block_0x00200cb4,
        block_0x00200cc0,
        block_0x00200cc4,
        block_0x00200ccc,
        block_0x00200cd8,
        block_0x00200ce0,
        block_0x00200cf8,
        block_0x00200d00,
        block_0x00200d08,
        block_0x00200d10,
        block_0x00200d28,
        block_0x00200d30,
        block_0x00200d38,
        block_0x00200d40,
        block_0x00200d58,
        block_0x00200d60,
        block_0x00200d68,
        block_0x00200d70,
        block_0x00200d88,
        block_0x00200d90,
        block_0x00200d98,
        block_0x00200da0,
        block_0x00200db8,
        block_0x00200dc0,
        block_0x00200dc8,
        block_0x00200dd0,
        block_0x00200de8,
        block_0x00200df0,
        block_0x00200df8,
        block_0x00200e00,
        block_0x00200e18,
        block_0x00200e20,
        block_0x00200e28,
        block_0x00200e30,
        block_0x00200e48,
        block_0x00200e50,
        block_0x00200e58,
        block_0x00200e68,
        block_0x00200e70,
        block_0x00200e88,
        block_0x00200e90,
        block_0x00200e98,
        block_0x00200ea8,
        block_0x00200eb0,
        block_0x00200ec8,
        block_0x00200ed0,
        block_0x00200ed8,
        block_0x00200ee8,
        block_0x00200ef0,
        block_0x00200f08,
        block_0x00200f10,
        block_0x00200f18,
        block_0x00200f28,
        block_0x00200f30,
        block_0x00200f48,
        block_0x00200f50,
        block_0x00200f58,
        block_0x00200f68,
        block_0x00200f70,
        block_0x00200f88,
        block_0x00200f90,
        block_0x00200f98,
        block_0x00200fa8,
        block_0x00200fb0,
        block_0x00200fc8,
        block_0x00200fd0,
        block_0x00200fd8,
        block_0x00200fe8,
        block_0x00200ff0,
        block_0x00201008,
        block_0x00201010,
        block_0x00201018,
        block_0x00201028,
        block_0x00201030,
        block_0x00201048,
        block_0x00201050,
        block_0x00201058,
        block_0x00201068,
        block_0x00201070,
        block_0x00201088,
        block_0x00201090,
        block_0x00201098,
        block_0x002010a8,
        block_0x002010b0,
        block_0x002010c8,
        block_0x002010d0,
        block_0x002010d8,
        block_0x002010e8,
        block_0x002010f0,
        block_0x00201108,
        block_0x00201110,
        block_0x00201118,
        block_0x00201128,
        block_0x00201130,
        block_0x00201148,
        block_0x00201150,
        block_0x00201158,
        block_0x00201168,
        block_0x00201170,
        block_0x0020124c,
        block_0x0020125c,
        block_0x0020126c,
        block_0x0020127c,
        block_0x0020128c,
        block_0x0020129c,
        block_0x002012ac,
        block_0x002012bc,
        block_0x002012cc,
        block_0x002012dc,
        block_0x002012ec,
    ];
    const IDX: [u16; 700usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 0u16, 0u16, 3u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        4u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 5u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 6u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 7u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 8u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 9u16, 0u16, 0u16, 0u16, 10u16, 0u16, 0u16, 0u16, 11u16, 0u16, 0u16,
        0u16, 0u16, 12u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 13u16, 0u16, 0u16, 14u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 15u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 16u16, 0u16, 17u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 18u16, 0u16, 0u16, 19u16, 0u16, 0u16, 0u16, 0u16, 0u16, 20u16, 21u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 22u16, 23u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 24u16, 25u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 26u16, 27u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 28u16, 29u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 30u16,
        0u16, 0u16, 31u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 32u16, 0u16,
        33u16, 0u16, 34u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 35u16, 36u16,
        37u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 38u16, 0u16, 0u16, 0u16, 0u16, 39u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        40u16, 0u16, 41u16, 0u16, 0u16, 0u16, 42u16, 0u16, 0u16, 0u16, 43u16, 0u16,
        44u16, 0u16, 0u16, 0u16, 0u16, 45u16, 0u16, 46u16, 0u16, 47u16, 0u16, 0u16, 0u16,
        48u16, 0u16, 0u16, 0u16, 49u16, 0u16, 50u16, 0u16, 0u16, 0u16, 0u16, 51u16, 0u16,
        52u16, 0u16, 53u16, 0u16, 0u16, 0u16, 54u16, 0u16, 55u16, 0u16, 0u16, 0u16, 0u16,
        56u16, 0u16, 57u16, 0u16, 58u16, 0u16, 0u16, 59u16, 60u16, 0u16, 61u16, 0u16,
        0u16, 62u16, 0u16, 63u16, 0u16, 0u16, 0u16, 0u16, 0u16, 64u16, 0u16, 65u16, 0u16,
        66u16, 0u16, 67u16, 0u16, 0u16, 0u16, 0u16, 0u16, 68u16, 0u16, 69u16, 0u16,
        70u16, 0u16, 71u16, 0u16, 0u16, 0u16, 0u16, 0u16, 72u16, 0u16, 73u16, 0u16,
        74u16, 0u16, 75u16, 0u16, 0u16, 0u16, 0u16, 0u16, 76u16, 0u16, 77u16, 0u16,
        78u16, 0u16, 79u16, 0u16, 0u16, 0u16, 0u16, 0u16, 80u16, 0u16, 81u16, 0u16,
        82u16, 0u16, 83u16, 0u16, 0u16, 0u16, 0u16, 0u16, 84u16, 0u16, 85u16, 0u16,
        86u16, 0u16, 87u16, 0u16, 0u16, 0u16, 0u16, 0u16, 88u16, 0u16, 89u16, 0u16,
        90u16, 0u16, 91u16, 0u16, 0u16, 0u16, 0u16, 0u16, 92u16, 0u16, 93u16, 0u16,
        94u16, 0u16, 0u16, 0u16, 95u16, 0u16, 96u16, 0u16, 0u16, 0u16, 0u16, 0u16, 97u16,
        0u16, 98u16, 0u16, 99u16, 0u16, 0u16, 0u16, 100u16, 0u16, 101u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 102u16, 0u16, 103u16, 0u16, 104u16, 0u16, 0u16, 0u16, 105u16,
        0u16, 106u16, 0u16, 0u16, 0u16, 0u16, 0u16, 107u16, 0u16, 108u16, 0u16, 109u16,
        0u16, 0u16, 0u16, 110u16, 0u16, 111u16, 0u16, 0u16, 0u16, 0u16, 0u16, 112u16,
        0u16, 113u16, 0u16, 114u16, 0u16, 0u16, 0u16, 115u16, 0u16, 116u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 117u16, 0u16, 118u16, 0u16, 119u16, 0u16, 0u16, 0u16, 120u16,
        0u16, 121u16, 0u16, 0u16, 0u16, 0u16, 0u16, 122u16, 0u16, 123u16, 0u16, 124u16,
        0u16, 0u16, 0u16, 125u16, 0u16, 126u16, 0u16, 0u16, 0u16, 0u16, 0u16, 127u16,
        0u16, 128u16, 0u16, 129u16, 0u16, 0u16, 0u16, 130u16, 0u16, 131u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 132u16, 0u16, 133u16, 0u16, 134u16, 0u16, 0u16, 0u16, 135u16,
        0u16, 136u16, 0u16, 0u16, 0u16, 0u16, 0u16, 137u16, 0u16, 138u16, 0u16, 139u16,
        0u16, 0u16, 0u16, 140u16, 0u16, 141u16, 0u16, 0u16, 0u16, 0u16, 0u16, 142u16,
        0u16, 143u16, 0u16, 144u16, 0u16, 0u16, 0u16, 145u16, 0u16, 146u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 147u16, 0u16, 148u16, 0u16, 149u16, 0u16, 0u16, 0u16, 150u16,
        0u16, 151u16, 0u16, 0u16, 0u16, 0u16, 0u16, 152u16, 0u16, 153u16, 0u16, 154u16,
        0u16, 0u16, 0u16, 155u16, 0u16, 156u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 157u16, 0u16, 0u16, 0u16, 158u16,
        0u16, 0u16, 0u16, 159u16, 0u16, 0u16, 0u16, 160u16, 0u16, 0u16, 0u16, 161u16,
        0u16, 0u16, 0u16, 162u16, 0u16, 0u16, 0u16, 163u16, 0u16, 0u16, 0u16, 164u16,
        0u16, 0u16, 0u16, 165u16, 0u16, 0u16, 0u16, 166u16, 0u16, 0u16, 0u16, 167u16,
    ];
    if pc < 2099200u32 || pc > 2101996u32 {
        return None;
    }
    let word_offset = ((pc - 2099200u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x00200800(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2099204u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2099208u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2099212u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2099216u32)?;
    emu.lw_no_count(13usize, 11usize, 4u32, 2099220u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2099304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200868));
    } else {
        emu.pc = 2099224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200818));
    }
}
#[inline(always)]
pub fn block_0x00200818(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 0u32, 2099228u32)?;
    emu.lw_no_count(14usize, 12usize, 4u32, 2099232u32)?;
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2099236u32);
    emu.sw_no_count(13usize, 11usize, 4u32, 2099240u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2099332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200884));
    } else {
        emu.pc = 2099244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020082c));
    }
}
#[inline]
pub fn block_0x0020082c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 12usize, 0u32, 2099248u32)?;
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2099252u32);
    emu.adi_no_count(13usize, 11usize, 1u32, 2099256u32);
    emu.lbu_no_count(11usize, 11usize, 0u32, 2099260u32);
    emu.sw_no_count(13usize, 12usize, 0u32, 2099264u32)?;
    emu.sw_no_count(14usize, 12usize, 4u32, 2099268u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2099272u32);
    emu.sb_no_count(12usize, 10usize, 1u32, 2099276u32);
    emu.sb_no_count(11usize, 10usize, 2u32, 2099280u32);
    emu.sb_no_count(0usize, 10usize, 0u32, 2099284u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2099288u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2099292u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2099296u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2099300u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2099304u32;
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
pub fn block_0x00200868(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(0usize, 10usize, 1u32, 2099308u32);
    emu.sb_no_count(0usize, 10usize, 0u32, 2099312u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2099316u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2099320u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2099324u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2099328u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2099332u32;
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
pub fn block_0x00200884(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 0usize, 1u32, 2099336u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2099340u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1281u32, 2099344u32);
    emu.sw_no_count(11usize, 2usize, 8u32, 2099348u32)?;
    emu.sw_no_count(8usize, 2usize, 12u32, 2099352u32)?;
    emu.adi_no_count(9usize, 10usize, 0u32, 2099356u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2099360u32);
    emu.apc_no_count(1usize, 2099360u32, 36864u32, 2099364u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2099368u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966348u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002008a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 10usize, 0u32, 2099372u32);
    emu.sw_no_count(11usize, 9usize, 4u32, 2099376u32)?;
    emu.sb_no_count(8usize, 9usize, 0u32, 2099380u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2099384u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2099388u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2099392u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2099396u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2099400u32;
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
pub fn block_0x002008c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2099404u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2099408u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2099412u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2099416u32)?;
    emu.adi_no_count(8usize, 11usize, 0u32, 2099420u32);
    emu.lw_no_count(11usize, 11usize, 4u32, 2099424u32)?;
    emu.adi_no_count(12usize, 0usize, 7u32, 2099428u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2099432u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2099520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200940));
    } else {
        emu.pc = 2099436u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002008ec));
    }
}
#[inline(always)]
pub fn block_0x002008ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 8u32, 2099440u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2099444u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1281u32, 2099448u32);
    emu.sw_no_count(11usize, 2usize, 8u32, 2099452u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2099456u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2099460u32);
    emu.apc_no_count(1usize, 2099460u32, 36864u32, 2099464u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2099468u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966248u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020090c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2099472u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2099476u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2099480u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2099640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002009b8));
    } else {
        emu.pc = 2099484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020091c));
    }
}
#[inline(always)]
pub fn block_0x0020091c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2099488u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2099492u32);
    emu.apc_no_count(1usize, 2099492u32, 24576u32, 2099496u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2099500u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1228u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020092c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2099504u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2099508u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2099512u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2099516u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2099520u32;
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
pub fn block_0x00200940(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 0u32, 2099524u32)?;
    emu.lbu_no_count(12usize, 10usize, 1u32, 2099528u32);
    emu.lbu_no_count(13usize, 10usize, 2u32, 2099532u32);
    emu.lbu_no_count(14usize, 10usize, 3u32, 2099536u32);
    emu.lbu_no_count(15usize, 10usize, 0u32, 2099540u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2099544u32);
    emu.sli_no_count(13usize, 13usize, 16u32, 2099548u32);
    emu.sli_no_count(14usize, 14usize, 24u32, 2099552u32);
    emu.orr_no_count(12usize, 12usize, 15usize, 2099556u32);
    emu.lbu_no_count(15usize, 10usize, 4u32, 2099560u32);
    emu.lbu_no_count(16usize, 10usize, 5u32, 2099564u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2099568u32);
    emu.lbu_no_count(14usize, 10usize, 6u32, 2099572u32);
    emu.lbu_no_count(17usize, 10usize, 7u32, 2099576u32);
    emu.sli_no_count(16usize, 16usize, 8u32, 2099580u32);
    emu.orr_no_count(15usize, 16usize, 15usize, 2099584u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2099588u32);
    emu.sli_no_count(17usize, 17usize, 24u32, 2099592u32);
    emu.orr_no_count(14usize, 17usize, 14usize, 2099596u32);
    emu.adi_no_count(16usize, 11usize, 4294967288u32, 2099600u32);
    emu.adi_no_count(17usize, 10usize, 8u32, 2099604u32);
    emu.orr_no_count(10usize, 13usize, 12usize, 2099608u32);
    emu.orr_no_count(11usize, 14usize, 15usize, 2099612u32);
    emu.sw_no_count(17usize, 8usize, 0u32, 2099616u32)?;
    emu.sw_no_count(16usize, 8usize, 4u32, 2099620u32)?;
    emu.apc_no_count(1usize, 2099620u32, 36864u32, 2099624u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2099628u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965852u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002009ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 0u32, 2099632u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2099636u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2099484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020091c));
    } else {
        emu.pc = 2099640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002009b8));
    }
}
#[inline(always)]
pub fn block_0x002009b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2099644u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 9usize, 0u32, 2099648u32)?;
    emu.sw_no_count(12usize, 9usize, 4u32, 2099652u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2099656u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2099660u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2099664u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2099668u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2099672u32;
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
pub fn block_0x002009d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294966960u32, 2099676u32);
    emu.sw_no_count(1usize, 2usize, 332u32, 2099680u32)?;
    emu.sw_no_count(8usize, 2usize, 328u32, 2099684u32)?;
    emu.sw_no_count(9usize, 2usize, 324u32, 2099688u32)?;
    emu.sw_no_count(18usize, 2usize, 320u32, 2099692u32)?;
    emu.sw_no_count(19usize, 2usize, 316u32, 2099696u32)?;
    emu.sw_no_count(20usize, 2usize, 312u32, 2099700u32)?;
    emu.sw_no_count(21usize, 2usize, 308u32, 2099704u32)?;
    emu.sw_no_count(22usize, 2usize, 304u32, 2099708u32)?;
    emu.sw_no_count(23usize, 2usize, 300u32, 2099712u32)?;
    emu.sw_no_count(24usize, 2usize, 296u32, 2099716u32)?;
    emu.sw_no_count(25usize, 2usize, 292u32, 2099720u32)?;
    emu.sw_no_count(26usize, 2usize, 288u32, 2099724u32)?;
    emu.sw_no_count(27usize, 2usize, 284u32, 2099728u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2099732u32);
    emu.sw_no_count(11usize, 2usize, 80u32, 2099736u32)?;
    emu.add_memory_rw_events(16usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2100140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200bac));
    } else {
        emu.pc = 2099740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200a1c));
    }
}
#[inline(always)]
pub fn block_0x00200a1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 11usize, 4u32, 2099744u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2100040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200b48));
    } else {
        emu.pc = 2099748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200a24));
    }
}
#[inline(always)]
pub fn block_0x00200a24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 11usize, 0u32, 2099752u32)?;
    emu.adi_no_count(14usize, 10usize, 4294967295u32, 2099756u32);
    emu.adi_no_count(15usize, 13usize, 1u32, 2099760u32);
    emu.lbu_no_count(9usize, 13usize, 0u32, 2099764u32);
    emu.adi_no_count(18usize, 0usize, 1u32, 2099768u32);
    emu.sw_no_count(15usize, 11usize, 0u32, 2099772u32)?;
    emu.sw_no_count(14usize, 11usize, 4u32, 2099776u32)?;
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2100160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200bc0));
    } else {
        emu.pc = 2099780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200a44));
    }
}
#[inline(always)]
pub fn block_0x00200a44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 12usize, 4294967294u32, 2099784u32);
    emu.sw_no_count(15usize, 2usize, 84u32, 2099788u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2100040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200b48));
    } else {
        emu.pc = 2099792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200a50));
    }
}
#[inline(always)]
pub fn block_0x00200a50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 13usize, 2u32, 2099796u32);
    emu.lbu_no_count(18usize, 13usize, 1u32, 2099800u32);
    emu.adi_no_count(14usize, 10usize, 4294967294u32, 2099804u32);
    emu.sw_no_count(16usize, 11usize, 0u32, 2099808u32)?;
    emu.sw_no_count(14usize, 11usize, 4u32, 2099812u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2100192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200be0));
    } else {
        emu.pc = 2099816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200a68));
    }
}
#[inline(always)]
pub fn block_0x00200a68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2100040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200b48));
    } else {
        emu.pc = 2099820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200a6c));
    }
}
#[inline(always)]
pub fn block_0x00200a6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 13usize, 3u32, 2099824u32);
    emu.adi_no_count(14usize, 10usize, 4294967293u32, 2099828u32);
    emu.lbu_no_count(19usize, 13usize, 2u32, 2099832u32);
    emu.adi_no_count(16usize, 0usize, 3u32, 2099836u32);
    emu.sw_no_count(15usize, 11usize, 0u32, 2099840u32)?;
    emu.sw_no_count(14usize, 11usize, 4u32, 2099844u32)?;
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2100208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200bf0));
    } else {
        emu.pc = 2099848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200a88));
    }
}
#[inline(always)]
pub fn block_0x00200a88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2100040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200b48));
    } else {
        emu.pc = 2099852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200a8c));
    }
}
#[inline(always)]
pub fn block_0x00200a8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 13usize, 4u32, 2099856u32);
    emu.adi_no_count(14usize, 10usize, 4294967292u32, 2099860u32);
    emu.lbu_no_count(20usize, 13usize, 3u32, 2099864u32);
    emu.adi_no_count(16usize, 0usize, 4u32, 2099868u32);
    emu.sw_no_count(15usize, 11usize, 0u32, 2099872u32)?;
    emu.sw_no_count(14usize, 11usize, 4u32, 2099876u32)?;
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2100268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200c2c));
    } else {
        emu.pc = 2099880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200aa8));
    }
}
#[inline(always)]
pub fn block_0x00200aa8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2100040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200b48));
    } else {
        emu.pc = 2099884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200aac));
    }
}
#[inline(always)]
pub fn block_0x00200aac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 13usize, 5u32, 2099888u32);
    emu.adi_no_count(14usize, 10usize, 4294967291u32, 2099892u32);
    emu.lbu_no_count(21usize, 13usize, 4u32, 2099896u32);
    emu.adi_no_count(16usize, 0usize, 5u32, 2099900u32);
    emu.sw_no_count(15usize, 11usize, 0u32, 2099904u32)?;
    emu.sw_no_count(14usize, 11usize, 4u32, 2099908u32)?;
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2100284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200c3c));
    } else {
        emu.pc = 2099912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200ac8));
    }
}
#[inline(always)]
pub fn block_0x00200ac8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2100040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200b48));
    } else {
        emu.pc = 2099916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200acc));
    }
}
#[inline(always)]
pub fn block_0x00200acc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 13usize, 6u32, 2099920u32);
    emu.adi_no_count(14usize, 10usize, 4294967290u32, 2099924u32);
    emu.lbu_no_count(22usize, 13usize, 5u32, 2099928u32);
    emu.adi_no_count(16usize, 0usize, 6u32, 2099932u32);
    emu.sw_no_count(15usize, 11usize, 0u32, 2099936u32)?;
    emu.sw_no_count(14usize, 11usize, 4u32, 2099940u32)?;
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2100344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200c78));
    } else {
        emu.pc = 2099944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200ae8));
    }
}
#[inline(always)]
pub fn block_0x00200ae8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2100040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200b48));
    } else {
        emu.pc = 2099948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200aec));
    }
}
#[inline(always)]
pub fn block_0x00200aec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 13usize, 7u32, 2099952u32);
    emu.adi_no_count(14usize, 10usize, 4294967289u32, 2099956u32);
    emu.lbu_no_count(23usize, 13usize, 6u32, 2099960u32);
    emu.adi_no_count(16usize, 0usize, 7u32, 2099964u32);
    emu.sw_no_count(15usize, 11usize, 0u32, 2099968u32)?;
    emu.sw_no_count(14usize, 11usize, 4u32, 2099972u32)?;
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2100404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200cb4));
    } else {
        emu.pc = 2099976u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200b08));
    }
}
#[inline(always)]
pub fn block_0x00200b08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4294967288u32, 2099980u32);
    emu.sw_no_count(12usize, 2usize, 84u32, 2099984u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2100040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200b48));
    } else {
        emu.pc = 2099988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200b14));
    }
}
#[inline]
pub fn block_0x00200b14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 13usize, 8u32, 2099992u32);
    emu.lbu_no_count(24usize, 13usize, 7u32, 2099996u32);
    emu.adi_no_count(10usize, 10usize, 4294967288u32, 2100000u32);
    emu.sw_no_count(12usize, 11usize, 0u32, 2100004u32)?;
    emu.sw_no_count(10usize, 11usize, 4u32, 2100008u32)?;
    emu.adi_no_count(10usize, 2usize, 88u32, 2100012u32);
    emu.adi_no_count(11usize, 2usize, 80u32, 2100016u32);
    emu.apc_no_count(1usize, 2100016u32, 0u32, 2100020u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100024u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966480u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200b38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 88u32, 2100028u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2100224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200c00));
    } else {
        emu.pc = 2100032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200b40));
    }
}
#[inline(always)]
pub fn block_0x00200b40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 92u32, 2100036u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2100040u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100428u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200ccc));
}
#[inline(always)]
pub fn block_0x00200b48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 1u32, 2100044u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2100048u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1281u32, 2100052u32);
    emu.sw_no_count(10usize, 2usize, 272u32, 2100056u32)?;
    emu.sw_no_count(9usize, 2usize, 276u32, 2100060u32)?;
    emu.adi_no_count(10usize, 2usize, 272u32, 2100064u32);
    emu.apc_no_count(1usize, 2100064u32, 36864u32, 2100068u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100072u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965644u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200b68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(9usize, 8usize, 0u32, 2100076u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2100076u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200b6c));
}
#[inline(always)]
pub fn block_0x00200b6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 8usize, 4u32, 2100080u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2100080u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200b70));
}
#[inline]
pub fn block_0x00200b70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 332u32, 2100084u32)?;
    emu.lw_no_count(8usize, 2usize, 328u32, 2100088u32)?;
    emu.lw_no_count(9usize, 2usize, 324u32, 2100092u32)?;
    emu.lw_no_count(18usize, 2usize, 320u32, 2100096u32)?;
    emu.lw_no_count(19usize, 2usize, 316u32, 2100100u32)?;
    emu.lw_no_count(20usize, 2usize, 312u32, 2100104u32)?;
    emu.lw_no_count(21usize, 2usize, 308u32, 2100108u32)?;
    emu.lw_no_count(22usize, 2usize, 304u32, 2100112u32)?;
    emu.lw_no_count(23usize, 2usize, 300u32, 2100116u32)?;
    emu.lw_no_count(24usize, 2usize, 296u32, 2100120u32)?;
    emu.lw_no_count(25usize, 2usize, 292u32, 2100124u32)?;
    emu.lw_no_count(26usize, 2usize, 288u32, 2100128u32)?;
    emu.lw_no_count(27usize, 2usize, 284u32, 2100132u32)?;
    emu.adi_no_count(2usize, 2usize, 336u32, 2100136u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100140u32;
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
pub fn block_0x00200bac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2100144u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966584u32, 2100148u32);
    emu.adi_no_count(11usize, 2usize, 283u32, 2100152u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2100156u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2100160u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100420u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200cc4));
}
#[inline(always)]
pub fn block_0x00200bc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2100164u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966584u32, 2100168u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2100172u32);
    emu.adi_no_count(11usize, 2usize, 283u32, 2100176u32);
    emu.apc_no_count(1usize, 2100176u32, 12288u32, 2100180u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100184u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1696u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200bd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(18usize, 8usize, 0u32, 2100188u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2100192u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100076u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200b6c));
}
#[inline(always)]
pub fn block_0x00200be0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2100196u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966584u32, 2100200u32);
    emu.adi_no_count(10usize, 0usize, 2u32, 2100204u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2100208u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100416u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200cc0));
}
#[inline(always)]
pub fn block_0x00200bf0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2100212u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966584u32, 2100216u32);
    emu.adi_no_count(10usize, 0usize, 3u32, 2100220u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2100224u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100416u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200cc0));
}
#[inline(always)]
pub fn block_0x00200c00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 89u32, 2100228u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2100824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200e58));
    } else {
        emu.pc = 2100232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200c08));
    }
}
#[inline(always)]
pub fn block_0x00200c08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(25usize, 2usize, 90u32, 2100236u32);
    emu.adi_no_count(10usize, 2usize, 96u32, 2100240u32);
    emu.adi_no_count(11usize, 2usize, 80u32, 2100244u32);
    emu.apc_no_count(1usize, 2100244u32, 0u32, 2100248u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100252u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966252u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200c1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 96u32, 2100256u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2100300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200c4c));
    } else {
        emu.pc = 2100260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200c24));
    }
}
#[inline(always)]
pub fn block_0x00200c24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 100u32, 2100264u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2100268u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100428u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200ccc));
}
#[inline(always)]
pub fn block_0x00200c2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2100272u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966584u32, 2100276u32);
    emu.adi_no_count(10usize, 0usize, 4u32, 2100280u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2100284u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100416u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200cc0));
}
#[inline(always)]
pub fn block_0x00200c3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2100288u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966584u32, 2100292u32);
    emu.adi_no_count(10usize, 0usize, 5u32, 2100296u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2100300u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100416u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200cc0));
}
#[inline(always)]
pub fn block_0x00200c4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 97u32, 2100304u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2100888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200e98));
    } else {
        emu.pc = 2100308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200c54));
    }
}
#[inline(always)]
pub fn block_0x00200c54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(26usize, 2usize, 98u32, 2100312u32);
    emu.adi_no_count(10usize, 2usize, 104u32, 2100316u32);
    emu.adi_no_count(11usize, 2usize, 80u32, 2100320u32);
    emu.apc_no_count(1usize, 2100320u32, 0u32, 2100324u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100328u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966176u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200c68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 104u32, 2100332u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2100360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200c88));
    } else {
        emu.pc = 2100336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200c70));
    }
}
#[inline(always)]
pub fn block_0x00200c70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 108u32, 2100340u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2100344u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100428u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200ccc));
}
#[inline(always)]
pub fn block_0x00200c78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2100348u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966584u32, 2100352u32);
    emu.adi_no_count(10usize, 0usize, 6u32, 2100356u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2100360u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100416u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200cc0));
}
#[inline(always)]
pub fn block_0x00200c88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 105u32, 2100364u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2100952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200ed8));
    } else {
        emu.pc = 2100368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200c90));
    }
}
#[inline(always)]
pub fn block_0x00200c90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(27usize, 2usize, 106u32, 2100372u32);
    emu.adi_no_count(10usize, 2usize, 112u32, 2100376u32);
    emu.adi_no_count(11usize, 2usize, 80u32, 2100380u32);
    emu.apc_no_count(1usize, 2100380u32, 0u32, 2100384u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100388u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966116u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200ca4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 112u32, 2100392u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2100440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200cd8));
    } else {
        emu.pc = 2100396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200cac));
    }
}
#[inline(always)]
pub fn block_0x00200cac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 116u32, 2100400u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2100404u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100428u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200ccc));
}
#[inline(always)]
pub fn block_0x00200cb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2100408u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966584u32, 2100412u32);
    emu.adi_no_count(10usize, 0usize, 7u32, 2100416u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2100416u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200cc0));
}
#[inline(always)]
pub fn block_0x00200cc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 2usize, 283u32, 2100420u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2100420u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200cc4));
}
#[inline(always)]
pub fn block_0x00200cc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2100420u32, 12288u32, 2100424u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100428u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1452u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200ccc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2100432u32);
    emu.sb_no_count(11usize, 8usize, 0u32, 2100436u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2100440u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100076u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200b6c));
}
#[inline(always)]
pub fn block_0x00200cd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 113u32, 2100444u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2101016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200f18));
    } else {
        emu.pc = 2100448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200ce0));
    }
}
#[inline(always)]
pub fn block_0x00200ce0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 114u32, 2100452u32);
    emu.sw_no_count(10usize, 2usize, 76u32, 2100456u32)?;
    emu.adi_no_count(10usize, 2usize, 120u32, 2100460u32);
    emu.adi_no_count(11usize, 2usize, 80u32, 2100464u32);
    emu.apc_no_count(1usize, 2100464u32, 0u32, 2100468u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100472u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966032u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200cf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 120u32, 2100476u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2100488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200d08));
    } else {
        emu.pc = 2100480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200d00));
    }
}
#[inline(always)]
pub fn block_0x00200d00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 124u32, 2100484u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2100488u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100428u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200ccc));
}
#[inline(always)]
pub fn block_0x00200d08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 121u32, 2100492u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2101080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200f58));
    } else {
        emu.pc = 2100496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200d10));
    }
}
#[inline(always)]
pub fn block_0x00200d10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 122u32, 2100500u32);
    emu.sw_no_count(10usize, 2usize, 72u32, 2100504u32)?;
    emu.adi_no_count(10usize, 2usize, 128u32, 2100508u32);
    emu.adi_no_count(11usize, 2usize, 80u32, 2100512u32);
    emu.apc_no_count(1usize, 2100512u32, 0u32, 2100516u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100520u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965984u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200d28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 128u32, 2100524u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2100536u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200d38));
    } else {
        emu.pc = 2100528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200d30));
    }
}
#[inline(always)]
pub fn block_0x00200d30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 132u32, 2100532u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2100536u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100428u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200ccc));
}
#[inline(always)]
pub fn block_0x00200d38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 129u32, 2100540u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2101144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200f98));
    } else {
        emu.pc = 2100544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200d40));
    }
}
#[inline(always)]
pub fn block_0x00200d40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 130u32, 2100548u32);
    emu.sw_no_count(10usize, 2usize, 68u32, 2100552u32)?;
    emu.adi_no_count(10usize, 2usize, 136u32, 2100556u32);
    emu.adi_no_count(11usize, 2usize, 80u32, 2100560u32);
    emu.apc_no_count(1usize, 2100560u32, 0u32, 2100564u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100568u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965936u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200d58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 136u32, 2100572u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2100584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200d68));
    } else {
        emu.pc = 2100576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200d60));
    }
}
#[inline(always)]
pub fn block_0x00200d60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 140u32, 2100580u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2100584u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100428u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200ccc));
}
#[inline(always)]
pub fn block_0x00200d68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 137u32, 2100588u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2101208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200fd8));
    } else {
        emu.pc = 2100592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200d70));
    }
}
#[inline(always)]
pub fn block_0x00200d70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 138u32, 2100596u32);
    emu.sw_no_count(10usize, 2usize, 64u32, 2100600u32)?;
    emu.adi_no_count(10usize, 2usize, 144u32, 2100604u32);
    emu.adi_no_count(11usize, 2usize, 80u32, 2100608u32);
    emu.apc_no_count(1usize, 2100608u32, 0u32, 2100612u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100616u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965888u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200d88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 144u32, 2100620u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2100632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200d98));
    } else {
        emu.pc = 2100624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200d90));
    }
}
#[inline(always)]
pub fn block_0x00200d90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 148u32, 2100628u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2100632u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100428u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200ccc));
}
#[inline(always)]
pub fn block_0x00200d98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 145u32, 2100636u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2101272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201018));
    } else {
        emu.pc = 2100640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200da0));
    }
}
#[inline(always)]
pub fn block_0x00200da0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 146u32, 2100644u32);
    emu.sw_no_count(10usize, 2usize, 60u32, 2100648u32)?;
    emu.adi_no_count(10usize, 2usize, 152u32, 2100652u32);
    emu.adi_no_count(11usize, 2usize, 80u32, 2100656u32);
    emu.apc_no_count(1usize, 2100656u32, 0u32, 2100660u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100664u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965840u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200db8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 152u32, 2100668u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2100680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200dc8));
    } else {
        emu.pc = 2100672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200dc0));
    }
}
#[inline(always)]
pub fn block_0x00200dc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 156u32, 2100676u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2100680u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100428u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200ccc));
}
#[inline(always)]
pub fn block_0x00200dc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 153u32, 2100684u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2101336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201058));
    } else {
        emu.pc = 2100688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200dd0));
    }
}
#[inline(always)]
pub fn block_0x00200dd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 154u32, 2100692u32);
    emu.sw_no_count(10usize, 2usize, 56u32, 2100696u32)?;
    emu.adi_no_count(10usize, 2usize, 160u32, 2100700u32);
    emu.adi_no_count(11usize, 2usize, 80u32, 2100704u32);
    emu.apc_no_count(1usize, 2100704u32, 0u32, 2100708u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100712u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965792u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200de8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 160u32, 2100716u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2100728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200df8));
    } else {
        emu.pc = 2100720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200df0));
    }
}
#[inline(always)]
pub fn block_0x00200df0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 164u32, 2100724u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2100728u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100428u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200ccc));
}
#[inline(always)]
pub fn block_0x00200df8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 161u32, 2100732u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2101400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201098));
    } else {
        emu.pc = 2100736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200e00));
    }
}
#[inline(always)]
pub fn block_0x00200e00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 162u32, 2100740u32);
    emu.sw_no_count(10usize, 2usize, 52u32, 2100744u32)?;
    emu.adi_no_count(10usize, 2usize, 168u32, 2100748u32);
    emu.adi_no_count(11usize, 2usize, 80u32, 2100752u32);
    emu.apc_no_count(1usize, 2100752u32, 0u32, 2100756u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100760u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965744u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200e18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 168u32, 2100764u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2100776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200e28));
    } else {
        emu.pc = 2100768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200e20));
    }
}
#[inline(always)]
pub fn block_0x00200e20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 172u32, 2100772u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2100776u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100428u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200ccc));
}
#[inline(always)]
pub fn block_0x00200e28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 169u32, 2100780u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2101464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002010d8));
    } else {
        emu.pc = 2100784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200e30));
    }
}
#[inline(always)]
pub fn block_0x00200e30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 170u32, 2100788u32);
    emu.sw_no_count(10usize, 2usize, 48u32, 2100792u32)?;
    emu.adi_no_count(10usize, 2usize, 176u32, 2100796u32);
    emu.adi_no_count(11usize, 2usize, 80u32, 2100800u32);
    emu.apc_no_count(1usize, 2100800u32, 0u32, 2100804u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100808u32;
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
#[inline(always)]
pub fn block_0x00200e48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 176u32, 2100812u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2100840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200e68));
    } else {
        emu.pc = 2100816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200e50));
    }
}
#[inline(always)]
pub fn block_0x00200e50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 180u32, 2100820u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2100824u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100428u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200ccc));
}
#[inline(always)]
pub fn block_0x00200e58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2100828u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966584u32, 2100832u32);
    emu.adi_no_count(10usize, 0usize, 8u32, 2100836u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2100840u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100416u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200cc0));
}
#[inline(always)]
pub fn block_0x00200e68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 177u32, 2100844u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2101528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201118));
    } else {
        emu.pc = 2100848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200e70));
    }
}
#[inline(always)]
pub fn block_0x00200e70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 178u32, 2100852u32);
    emu.sw_no_count(10usize, 2usize, 44u32, 2100856u32)?;
    emu.adi_no_count(10usize, 2usize, 184u32, 2100860u32);
    emu.adi_no_count(11usize, 2usize, 80u32, 2100864u32);
    emu.apc_no_count(1usize, 2100864u32, 0u32, 2100868u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100872u32;
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
pub fn block_0x00200e88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 184u32, 2100876u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2100904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200ea8));
    } else {
        emu.pc = 2100880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200e90));
    }
}
#[inline(always)]
pub fn block_0x00200e90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 188u32, 2100884u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2100888u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100428u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200ccc));
}
#[inline(always)]
pub fn block_0x00200e98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2100892u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966584u32, 2100896u32);
    emu.adi_no_count(10usize, 0usize, 9u32, 2100900u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2100904u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100416u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200cc0));
}
#[inline(always)]
pub fn block_0x00200ea8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 185u32, 2100908u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2101592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201158));
    } else {
        emu.pc = 2100912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200eb0));
    }
}
#[inline(always)]
pub fn block_0x00200eb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 186u32, 2100916u32);
    emu.sw_no_count(10usize, 2usize, 40u32, 2100920u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2100924u32);
    emu.adi_no_count(11usize, 2usize, 80u32, 2100928u32);
    emu.apc_no_count(1usize, 2100928u32, 0u32, 2100932u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100936u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965568u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200ec8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 192u32, 2100940u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2100968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200ee8));
    } else {
        emu.pc = 2100944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200ed0));
    }
}
#[inline(always)]
pub fn block_0x00200ed0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 196u32, 2100948u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2100952u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100428u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200ccc));
}
#[inline(always)]
pub fn block_0x00200ed8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2100956u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966584u32, 2100960u32);
    emu.adi_no_count(10usize, 0usize, 10u32, 2100964u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2100968u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100416u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200cc0));
}
#[inline(always)]
pub fn block_0x00200ee8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 193u32, 2100972u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2101836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020124c));
    } else {
        emu.pc = 2100976u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200ef0));
    }
}
#[inline(always)]
pub fn block_0x00200ef0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 194u32, 2100980u32);
    emu.sw_no_count(10usize, 2usize, 36u32, 2100984u32)?;
    emu.adi_no_count(10usize, 2usize, 200u32, 2100988u32);
    emu.adi_no_count(11usize, 2usize, 80u32, 2100992u32);
    emu.apc_no_count(1usize, 2100992u32, 0u32, 2100996u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101000u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965504u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200f08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 200u32, 2101004u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2101032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200f28));
    } else {
        emu.pc = 2101008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200f10));
    }
}
#[inline(always)]
pub fn block_0x00200f10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 204u32, 2101012u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2101016u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100428u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200ccc));
}
#[inline(always)]
pub fn block_0x00200f18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2101020u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966584u32, 2101024u32);
    emu.adi_no_count(10usize, 0usize, 11u32, 2101028u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2101032u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100416u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200cc0));
}
#[inline(always)]
pub fn block_0x00200f28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 201u32, 2101036u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2101852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020125c));
    } else {
        emu.pc = 2101040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200f30));
    }
}
#[inline(always)]
pub fn block_0x00200f30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 202u32, 2101044u32);
    emu.sw_no_count(10usize, 2usize, 32u32, 2101048u32)?;
    emu.adi_no_count(10usize, 2usize, 208u32, 2101052u32);
    emu.adi_no_count(11usize, 2usize, 80u32, 2101056u32);
    emu.apc_no_count(1usize, 2101056u32, 0u32, 2101060u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101064u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965440u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200f48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 208u32, 2101068u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2101096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200f68));
    } else {
        emu.pc = 2101072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200f50));
    }
}
#[inline(always)]
pub fn block_0x00200f50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 212u32, 2101076u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2101080u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100428u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200ccc));
}
#[inline(always)]
pub fn block_0x00200f58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2101084u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966584u32, 2101088u32);
    emu.adi_no_count(10usize, 0usize, 12u32, 2101092u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2101096u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100416u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200cc0));
}
#[inline(always)]
pub fn block_0x00200f68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 209u32, 2101100u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2101868u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020126c));
    } else {
        emu.pc = 2101104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200f70));
    }
}
#[inline(always)]
pub fn block_0x00200f70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 210u32, 2101108u32);
    emu.sw_no_count(10usize, 2usize, 28u32, 2101112u32)?;
    emu.adi_no_count(10usize, 2usize, 216u32, 2101116u32);
    emu.adi_no_count(11usize, 2usize, 80u32, 2101120u32);
    emu.apc_no_count(1usize, 2101120u32, 0u32, 2101124u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101128u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965376u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200f88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 216u32, 2101132u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2101160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200fa8));
    } else {
        emu.pc = 2101136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200f90));
    }
}
#[inline(always)]
pub fn block_0x00200f90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 220u32, 2101140u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2101144u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100428u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200ccc));
}
#[inline(always)]
pub fn block_0x00200f98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2101148u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966584u32, 2101152u32);
    emu.adi_no_count(10usize, 0usize, 13u32, 2101156u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2101160u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100416u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200cc0));
}
#[inline(always)]
pub fn block_0x00200fa8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 217u32, 2101164u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2101884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020127c));
    } else {
        emu.pc = 2101168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200fb0));
    }
}
#[inline(always)]
pub fn block_0x00200fb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 218u32, 2101172u32);
    emu.sw_no_count(10usize, 2usize, 24u32, 2101176u32)?;
    emu.adi_no_count(10usize, 2usize, 224u32, 2101180u32);
    emu.adi_no_count(11usize, 2usize, 80u32, 2101184u32);
    emu.apc_no_count(1usize, 2101184u32, 0u32, 2101188u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101192u32;
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
pub fn block_0x00200fc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 224u32, 2101196u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2101224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200fe8));
    } else {
        emu.pc = 2101200u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200fd0));
    }
}
#[inline(always)]
pub fn block_0x00200fd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 228u32, 2101204u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2101208u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100428u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200ccc));
}
#[inline(always)]
pub fn block_0x00200fd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2101212u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966584u32, 2101216u32);
    emu.adi_no_count(10usize, 0usize, 14u32, 2101220u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2101224u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100416u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200cc0));
}
#[inline(always)]
pub fn block_0x00200fe8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 225u32, 2101228u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2101900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020128c));
    } else {
        emu.pc = 2101232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200ff0));
    }
}
#[inline(always)]
pub fn block_0x00200ff0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 226u32, 2101236u32);
    emu.sw_no_count(10usize, 2usize, 20u32, 2101240u32)?;
    emu.adi_no_count(10usize, 2usize, 232u32, 2101244u32);
    emu.adi_no_count(11usize, 2usize, 80u32, 2101248u32);
    emu.apc_no_count(1usize, 2101248u32, 0u32, 2101252u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101256u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965248u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201008(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 232u32, 2101260u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2101288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201028));
    } else {
        emu.pc = 2101264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201010));
    }
}
#[inline(always)]
pub fn block_0x00201010(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 236u32, 2101268u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2101272u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100428u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200ccc));
}
#[inline(always)]
pub fn block_0x00201018(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2101276u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966584u32, 2101280u32);
    emu.adi_no_count(10usize, 0usize, 15u32, 2101284u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2101288u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100416u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200cc0));
}
#[inline(always)]
pub fn block_0x00201028(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 233u32, 2101292u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2101916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020129c));
    } else {
        emu.pc = 2101296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201030));
    }
}
#[inline(always)]
pub fn block_0x00201030(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 234u32, 2101300u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2101304u32)?;
    emu.adi_no_count(10usize, 2usize, 240u32, 2101308u32);
    emu.adi_no_count(11usize, 2usize, 80u32, 2101312u32);
    emu.apc_no_count(1usize, 2101312u32, 4294963200u32, 2101316u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101320u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1984u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201048(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 240u32, 2101324u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2101352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201068));
    } else {
        emu.pc = 2101328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201050));
    }
}
#[inline(always)]
pub fn block_0x00201050(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 244u32, 2101332u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2101336u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100428u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200ccc));
}
#[inline(always)]
pub fn block_0x00201058(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2101340u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966584u32, 2101344u32);
    emu.adi_no_count(10usize, 0usize, 16u32, 2101348u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2101352u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100416u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200cc0));
}
#[inline(always)]
pub fn block_0x00201068(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 241u32, 2101356u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2101932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002012ac));
    } else {
        emu.pc = 2101360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201070));
    }
}
#[inline(always)]
pub fn block_0x00201070(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 242u32, 2101364u32);
    emu.sw_no_count(10usize, 2usize, 12u32, 2101368u32)?;
    emu.adi_no_count(10usize, 2usize, 248u32, 2101372u32);
    emu.adi_no_count(11usize, 2usize, 80u32, 2101376u32);
    emu.apc_no_count(1usize, 2101376u32, 4294963200u32, 2101380u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101384u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1920u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201088(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 248u32, 2101388u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2101416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002010a8));
    } else {
        emu.pc = 2101392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201090));
    }
}
#[inline(always)]
pub fn block_0x00201090(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 252u32, 2101396u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2101400u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100428u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200ccc));
}
#[inline(always)]
pub fn block_0x00201098(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2101404u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966584u32, 2101408u32);
    emu.adi_no_count(10usize, 0usize, 17u32, 2101412u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2101416u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100416u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200cc0));
}
#[inline(always)]
pub fn block_0x002010a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 249u32, 2101420u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2101948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002012bc));
    } else {
        emu.pc = 2101424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002010b0));
    }
}
#[inline(always)]
pub fn block_0x002010b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 250u32, 2101428u32);
    emu.sw_no_count(10usize, 2usize, 8u32, 2101432u32)?;
    emu.adi_no_count(10usize, 2usize, 256u32, 2101436u32);
    emu.adi_no_count(11usize, 2usize, 80u32, 2101440u32);
    emu.apc_no_count(1usize, 2101440u32, 4294963200u32, 2101444u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101448u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1856u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002010c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 256u32, 2101452u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2101480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002010e8));
    } else {
        emu.pc = 2101456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002010d0));
    }
}
#[inline(always)]
pub fn block_0x002010d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 260u32, 2101460u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2101464u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100428u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200ccc));
}
#[inline(always)]
pub fn block_0x002010d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2101468u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966584u32, 2101472u32);
    emu.adi_no_count(10usize, 0usize, 18u32, 2101476u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2101480u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100416u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200cc0));
}
#[inline(always)]
pub fn block_0x002010e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 257u32, 2101484u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2101964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002012cc));
    } else {
        emu.pc = 2101488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002010f0));
    }
}
#[inline(always)]
pub fn block_0x002010f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 258u32, 2101492u32);
    emu.sw_no_count(10usize, 2usize, 4u32, 2101496u32)?;
    emu.adi_no_count(10usize, 2usize, 264u32, 2101500u32);
    emu.adi_no_count(11usize, 2usize, 80u32, 2101504u32);
    emu.apc_no_count(1usize, 2101504u32, 4294963200u32, 2101508u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101512u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1792u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201108(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 264u32, 2101516u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2101544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201128));
    } else {
        emu.pc = 2101520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201110));
    }
}
#[inline(always)]
pub fn block_0x00201110(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 268u32, 2101524u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2101528u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100428u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200ccc));
}
#[inline(always)]
pub fn block_0x00201118(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2101532u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966584u32, 2101536u32);
    emu.adi_no_count(10usize, 0usize, 19u32, 2101540u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2101544u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100416u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200cc0));
}
#[inline(always)]
pub fn block_0x00201128(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 265u32, 2101548u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2101980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002012dc));
    } else {
        emu.pc = 2101552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201130));
    }
}
#[inline(always)]
pub fn block_0x00201130(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 266u32, 2101556u32);
    emu.sw_no_count(10usize, 2usize, 0u32, 2101560u32)?;
    emu.adi_no_count(10usize, 2usize, 272u32, 2101564u32);
    emu.adi_no_count(11usize, 2usize, 80u32, 2101568u32);
    emu.apc_no_count(1usize, 2101568u32, 4294963200u32, 2101572u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101576u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1728u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201148(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 272u32, 2101580u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2101608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201168));
    } else {
        emu.pc = 2101584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201150));
    }
}
#[inline(always)]
pub fn block_0x00201150(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 276u32, 2101588u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2101592u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100428u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200ccc));
}
#[inline(always)]
pub fn block_0x00201158(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2101596u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966584u32, 2101600u32);
    emu.adi_no_count(10usize, 0usize, 20u32, 2101604u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2101608u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100416u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200cc0));
}
#[inline(always)]
pub fn block_0x00201168(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 273u32, 2101612u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2101996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002012ec));
    } else {
        emu.pc = 2101616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201170));
    }
}
#[inline(never)]
pub fn block_0x00201170(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 55u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(20usize, 8usize, 4u32, 2101620u32);
    emu.sb_no_count(21usize, 8usize, 5u32, 2101624u32);
    emu.sb_no_count(22usize, 8usize, 6u32, 2101628u32);
    emu.sb_no_count(23usize, 8usize, 7u32, 2101632u32);
    emu.sb_no_count(9usize, 8usize, 1u32, 2101636u32);
    emu.lbu_no_count(10usize, 2usize, 274u32, 2101640u32);
    emu.sb_no_count(18usize, 8usize, 2u32, 2101644u32);
    emu.sb_no_count(19usize, 8usize, 3u32, 2101648u32);
    emu.sb_no_count(24usize, 8usize, 8u32, 2101652u32);
    emu.sb_no_count(25usize, 8usize, 9u32, 2101656u32);
    emu.sb_no_count(26usize, 8usize, 10u32, 2101660u32);
    emu.sb_no_count(27usize, 8usize, 11u32, 2101664u32);
    emu.lw_no_count(11usize, 2usize, 76u32, 2101668u32)?;
    emu.sb_no_count(11usize, 8usize, 12u32, 2101672u32);
    emu.lw_no_count(11usize, 2usize, 72u32, 2101676u32)?;
    emu.sb_no_count(11usize, 8usize, 13u32, 2101680u32);
    emu.lw_no_count(11usize, 2usize, 68u32, 2101684u32)?;
    emu.sb_no_count(11usize, 8usize, 14u32, 2101688u32);
    emu.lw_no_count(11usize, 2usize, 64u32, 2101692u32)?;
    emu.sb_no_count(11usize, 8usize, 15u32, 2101696u32);
    emu.lw_no_count(11usize, 2usize, 60u32, 2101700u32)?;
    emu.sb_no_count(11usize, 8usize, 16u32, 2101704u32);
    emu.lw_no_count(11usize, 2usize, 56u32, 2101708u32)?;
    emu.sb_no_count(11usize, 8usize, 17u32, 2101712u32);
    emu.lw_no_count(11usize, 2usize, 52u32, 2101716u32)?;
    emu.sb_no_count(11usize, 8usize, 18u32, 2101720u32);
    emu.lw_no_count(11usize, 2usize, 48u32, 2101724u32)?;
    emu.sb_no_count(11usize, 8usize, 19u32, 2101728u32);
    emu.lw_no_count(11usize, 2usize, 44u32, 2101732u32)?;
    emu.sb_no_count(11usize, 8usize, 20u32, 2101736u32);
    emu.lw_no_count(11usize, 2usize, 40u32, 2101740u32)?;
    emu.sb_no_count(11usize, 8usize, 21u32, 2101744u32);
    emu.lw_no_count(11usize, 2usize, 36u32, 2101748u32)?;
    emu.sb_no_count(11usize, 8usize, 22u32, 2101752u32);
    emu.lw_no_count(11usize, 2usize, 32u32, 2101756u32)?;
    emu.sb_no_count(11usize, 8usize, 23u32, 2101760u32);
    emu.lw_no_count(11usize, 2usize, 28u32, 2101764u32)?;
    emu.sb_no_count(11usize, 8usize, 24u32, 2101768u32);
    emu.lw_no_count(11usize, 2usize, 24u32, 2101772u32)?;
    emu.sb_no_count(11usize, 8usize, 25u32, 2101776u32);
    emu.lw_no_count(11usize, 2usize, 20u32, 2101780u32)?;
    emu.sb_no_count(11usize, 8usize, 26u32, 2101784u32);
    emu.lw_no_count(11usize, 2usize, 16u32, 2101788u32)?;
    emu.sb_no_count(11usize, 8usize, 27u32, 2101792u32);
    emu.lw_no_count(11usize, 2usize, 12u32, 2101796u32)?;
    emu.sb_no_count(11usize, 8usize, 28u32, 2101800u32);
    emu.lw_no_count(11usize, 2usize, 8u32, 2101804u32)?;
    emu.sb_no_count(11usize, 8usize, 29u32, 2101808u32);
    emu.lw_no_count(11usize, 2usize, 4u32, 2101812u32)?;
    emu.sb_no_count(11usize, 8usize, 30u32, 2101816u32);
    emu.lw_no_count(11usize, 2usize, 0u32, 2101820u32)?;
    emu.sb_no_count(11usize, 8usize, 31u32, 2101824u32);
    emu.sb_no_count(10usize, 8usize, 32u32, 2101828u32);
    emu.sb_no_count(0usize, 8usize, 0u32, 2101832u32);
    emu.add_memory_rw_events(55usize);
    let return_addr = 2101836u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100080u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200b70));
}
#[inline(always)]
pub fn block_0x0020124c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2101840u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966584u32, 2101844u32);
    emu.adi_no_count(10usize, 0usize, 21u32, 2101848u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2101852u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100416u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200cc0));
}
#[inline(always)]
pub fn block_0x0020125c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2101856u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966584u32, 2101860u32);
    emu.adi_no_count(10usize, 0usize, 22u32, 2101864u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2101868u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100416u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200cc0));
}
#[inline(always)]
pub fn block_0x0020126c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2101872u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966584u32, 2101876u32);
    emu.adi_no_count(10usize, 0usize, 23u32, 2101880u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2101884u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100416u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200cc0));
}
#[inline(always)]
pub fn block_0x0020127c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2101888u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966584u32, 2101892u32);
    emu.adi_no_count(10usize, 0usize, 24u32, 2101896u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2101900u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100416u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200cc0));
}
#[inline(always)]
pub fn block_0x0020128c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2101904u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966584u32, 2101908u32);
    emu.adi_no_count(10usize, 0usize, 25u32, 2101912u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2101916u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100416u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200cc0));
}
#[inline(always)]
pub fn block_0x0020129c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2101920u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966584u32, 2101924u32);
    emu.adi_no_count(10usize, 0usize, 26u32, 2101928u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2101932u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100416u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200cc0));
}
#[inline(always)]
pub fn block_0x002012ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2101936u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966584u32, 2101940u32);
    emu.adi_no_count(10usize, 0usize, 27u32, 2101944u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2101948u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100416u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200cc0));
}
#[inline(always)]
pub fn block_0x002012bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2101952u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966584u32, 2101956u32);
    emu.adi_no_count(10usize, 0usize, 28u32, 2101960u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2101964u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100416u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200cc0));
}
#[inline(always)]
pub fn block_0x002012cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2101968u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966584u32, 2101972u32);
    emu.adi_no_count(10usize, 0usize, 29u32, 2101976u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2101980u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100416u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200cc0));
}
#[inline(always)]
pub fn block_0x002012dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2101984u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966584u32, 2101988u32);
    emu.adi_no_count(10usize, 0usize, 30u32, 2101992u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2101996u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100416u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200cc0));
}
#[inline(always)]
pub fn block_0x002012ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2102000u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966584u32, 2102004u32);
    emu.adi_no_count(10usize, 0usize, 31u32, 2102008u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2102012u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100416u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200cc0));
}
