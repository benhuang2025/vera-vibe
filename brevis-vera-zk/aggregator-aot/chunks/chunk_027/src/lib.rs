pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2216696u32;
pub const PC_MAX: u32 = 2219780u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 173usize] = [
        block_0x0021d2f8,
        block_0x0021d324,
        block_0x0021d334,
        block_0x0021d340,
        block_0x0021d348,
        block_0x0021d34c,
        block_0x0021d374,
        block_0x0021d378,
        block_0x0021d394,
        block_0x0021d3dc,
        block_0x0021d3e4,
        block_0x0021d3ec,
        block_0x0021d3f4,
        block_0x0021d3fc,
        block_0x0021d414,
        block_0x0021d41c,
        block_0x0021d438,
        block_0x0021d43c,
        block_0x0021d444,
        block_0x0021d44c,
        block_0x0021d468,
        block_0x0021d46c,
        block_0x0021d480,
        block_0x0021d484,
        block_0x0021d48c,
        block_0x0021d494,
        block_0x0021d498,
        block_0x0021d4a0,
        block_0x0021d4a8,
        block_0x0021d4b0,
        block_0x0021d4bc,
        block_0x0021d504,
        block_0x0021d518,
        block_0x0021d528,
        block_0x0021d52c,
        block_0x0021d534,
        block_0x0021d53c,
        block_0x0021d558,
        block_0x0021d560,
        block_0x0021d574,
        block_0x0021d578,
        block_0x0021d57c,
        block_0x0021d590,
        block_0x0021d594,
        block_0x0021d598,
        block_0x0021d5d0,
        block_0x0021d5e0,
        block_0x0021d5e4,
        block_0x0021d5f8,
        block_0x0021d604,
        block_0x0021d618,
        block_0x0021d61c,
        block_0x0021d620,
        block_0x0021d630,
        block_0x0021d664,
        block_0x0021d684,
        block_0x0021d6c4,
        block_0x0021d6ec,
        block_0x0021d6fc,
        block_0x0021d704,
        block_0x0021d714,
        block_0x0021d730,
        block_0x0021d748,
        block_0x0021d74c,
        block_0x0021d770,
        block_0x0021d7d0,
        block_0x0021d7f8,
        block_0x0021d814,
        block_0x0021d824,
        block_0x0021d82c,
        block_0x0021d83c,
        block_0x0021d858,
        block_0x0021d870,
        block_0x0021d874,
        block_0x0021d8a8,
        block_0x0021d8e4,
        block_0x0021d910,
        block_0x0021d95c,
        block_0x0021d964,
        block_0x0021d994,
        block_0x0021d9a0,
        block_0x0021d9b8,
        block_0x0021d9bc,
        block_0x0021d9cc,
        block_0x0021d9d0,
        block_0x0021d9d4,
        block_0x0021d9e8,
        block_0x0021d9ec,
        block_0x0021da38,
        block_0x0021da3c,
        block_0x0021da58,
        block_0x0021da5c,
        block_0x0021da60,
        block_0x0021da6c,
        block_0x0021da8c,
        block_0x0021da90,
        block_0x0021daac,
        block_0x0021dab4,
        block_0x0021daec,
        block_0x0021db0c,
        block_0x0021db18,
        block_0x0021db2c,
        block_0x0021db40,
        block_0x0021db98,
        block_0x0021dba0,
        block_0x0021dba8,
        block_0x0021dbe8,
        block_0x0021dbec,
        block_0x0021dbfc,
        block_0x0021dc08,
        block_0x0021dc18,
        block_0x0021dc1c,
        block_0x0021dc20,
        block_0x0021dc2c,
        block_0x0021dc30,
        block_0x0021dc40,
        block_0x0021dc58,
        block_0x0021dc74,
        block_0x0021dc98,
        block_0x0021dca4,
        block_0x0021dcb0,
        block_0x0021dcc4,
        block_0x0021dcd8,
        block_0x0021dce0,
        block_0x0021dce8,
        block_0x0021dcec,
        block_0x0021dcf8,
        block_0x0021dcfc,
        block_0x0021dd00,
        block_0x0021dd04,
        block_0x0021dd08,
        block_0x0021dd1c,
        block_0x0021dd20,
        block_0x0021dd2c,
        block_0x0021dd4c,
        block_0x0021dd50,
        block_0x0021dd5c,
        block_0x0021dd6c,
        block_0x0021dd70,
        block_0x0021dd74,
        block_0x0021dd8c,
        block_0x0021dd90,
        block_0x0021dd98,
        block_0x0021dda4,
        block_0x0021ddb0,
        block_0x0021ddb8,
        block_0x0021ddc4,
        block_0x0021ddd0,
        block_0x0021ddd8,
        block_0x0021dde0,
        block_0x0021dde8,
        block_0x0021ddf8,
        block_0x0021de0c,
        block_0x0021de10,
        block_0x0021de14,
        block_0x0021de24,
        block_0x0021de28,
        block_0x0021de30,
        block_0x0021de34,
        block_0x0021de38,
        block_0x0021de3c,
        block_0x0021de4c,
        block_0x0021de54,
        block_0x0021de5c,
        block_0x0021de60,
        block_0x0021de78,
        block_0x0021de7c,
        block_0x0021de88,
        block_0x0021de8c,
        block_0x0021decc,
        block_0x0021ded4,
        block_0x0021dee8,
        block_0x0021df04,
    ];
    const IDX: [u16; 772usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16,
        0u16, 0u16, 3u16, 0u16, 0u16, 4u16, 0u16, 5u16, 6u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 7u16, 8u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        9u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 10u16, 0u16, 11u16, 0u16, 12u16, 0u16, 13u16, 0u16,
        14u16, 0u16, 0u16, 0u16, 0u16, 0u16, 15u16, 0u16, 16u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 17u16, 18u16, 0u16, 19u16, 0u16, 20u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 21u16, 22u16, 0u16, 0u16, 0u16, 0u16, 23u16, 24u16, 0u16, 25u16, 0u16,
        26u16, 27u16, 0u16, 28u16, 0u16, 29u16, 0u16, 30u16, 0u16, 0u16, 31u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 32u16, 0u16, 0u16, 0u16, 0u16, 33u16, 0u16, 0u16, 0u16, 34u16,
        35u16, 0u16, 36u16, 0u16, 37u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 38u16, 0u16,
        39u16, 0u16, 0u16, 0u16, 0u16, 40u16, 41u16, 42u16, 0u16, 0u16, 0u16, 0u16,
        43u16, 44u16, 45u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 46u16, 0u16, 0u16, 0u16, 47u16, 48u16, 0u16, 0u16, 0u16, 0u16,
        49u16, 0u16, 0u16, 50u16, 0u16, 0u16, 0u16, 0u16, 51u16, 52u16, 53u16, 0u16,
        0u16, 0u16, 54u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 55u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 56u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        57u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 58u16, 0u16, 0u16,
        0u16, 59u16, 0u16, 60u16, 0u16, 0u16, 0u16, 61u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 62u16, 0u16, 0u16, 0u16, 0u16, 0u16, 63u16, 64u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 65u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 66u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 67u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 68u16, 0u16, 0u16, 0u16, 69u16, 0u16, 70u16,
        0u16, 0u16, 0u16, 71u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 72u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 73u16, 74u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 75u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 76u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 77u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 78u16, 0u16, 79u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 80u16, 0u16,
        0u16, 81u16, 0u16, 0u16, 0u16, 0u16, 0u16, 82u16, 83u16, 0u16, 0u16, 0u16, 84u16,
        85u16, 86u16, 0u16, 0u16, 0u16, 0u16, 87u16, 88u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        89u16, 90u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 91u16, 92u16, 93u16, 0u16,
        0u16, 94u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 95u16, 96u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 97u16, 0u16, 98u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 99u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 100u16, 0u16, 0u16, 101u16, 0u16, 0u16, 0u16, 0u16, 102u16, 0u16,
        0u16, 0u16, 0u16, 103u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 104u16,
        0u16, 105u16, 0u16, 106u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 107u16, 108u16, 0u16, 0u16, 0u16, 109u16,
        0u16, 0u16, 110u16, 0u16, 0u16, 0u16, 111u16, 112u16, 113u16, 0u16, 0u16, 114u16,
        115u16, 0u16, 0u16, 0u16, 116u16, 0u16, 0u16, 0u16, 0u16, 0u16, 117u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 118u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 119u16, 0u16, 0u16, 120u16, 0u16, 0u16, 121u16, 0u16, 0u16, 0u16, 0u16,
        122u16, 0u16, 0u16, 0u16, 0u16, 123u16, 0u16, 124u16, 0u16, 125u16, 126u16, 0u16,
        0u16, 127u16, 128u16, 129u16, 130u16, 131u16, 0u16, 0u16, 0u16, 0u16, 132u16,
        133u16, 0u16, 0u16, 134u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 135u16,
        136u16, 0u16, 0u16, 137u16, 0u16, 0u16, 0u16, 138u16, 139u16, 140u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 141u16, 142u16, 0u16, 143u16, 0u16, 0u16, 144u16, 0u16, 0u16,
        145u16, 0u16, 146u16, 0u16, 0u16, 147u16, 0u16, 0u16, 148u16, 0u16, 149u16, 0u16,
        150u16, 0u16, 151u16, 0u16, 0u16, 0u16, 152u16, 0u16, 0u16, 0u16, 0u16, 153u16,
        154u16, 155u16, 0u16, 0u16, 0u16, 156u16, 157u16, 0u16, 158u16, 159u16, 160u16,
        161u16, 0u16, 0u16, 0u16, 162u16, 0u16, 163u16, 0u16, 164u16, 165u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 166u16, 167u16, 0u16, 0u16, 168u16, 169u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 170u16,
        0u16, 171u16, 0u16, 0u16, 0u16, 0u16, 172u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        173u16,
    ];
    if pc < 2216696u32 || pc > 2219780u32 {
        return None;
    }
    let word_offset = ((pc - 2216696u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x0021d2f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2216700u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2216704u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2216708u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2216712u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2216716u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2216720u32)?;
    emu.adi_no_count(8usize, 14usize, 0u32, 2216724u32);
    emu.adi_no_count(9usize, 13usize, 0u32, 2216728u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2216732u32);
    let a = 0u32.wrapping_add(1114112u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2216736u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2216776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d348));
    } else {
        emu.pc = 2216740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d324));
    }
}
#[inline(always)]
pub fn block_0x0021d324(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 18usize, 16u32, 2216744u32)?;
    emu.adi_no_count(19usize, 10usize, 0u32, 2216748u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2216752u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2216756u32;
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
pub fn block_0x0021d334(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 10usize, 0u32, 2216760u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2216764u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2216776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d348));
    } else {
        emu.pc = 2216768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d340));
    }
}
#[inline(always)]
pub fn block_0x0021d340(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2216772u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2216776u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2216824u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d378));
}
#[inline(always)]
pub fn block_0x0021d348(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2216820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d374));
    } else {
        emu.pc = 2216780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d34c));
    }
}
#[inline]
pub fn block_0x0021d34c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(6usize, 18usize, 12u32, 2216784u32)?;
    emu.adi_no_count(11usize, 9usize, 0u32, 2216788u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2216792u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2216796u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2216800u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2216804u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2216808u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2216812u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2216816u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2216820u32;
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
pub fn block_0x0021d374(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2216824u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2216824u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d378));
}
#[inline(always)]
pub fn block_0x0021d378(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2216828u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2216832u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2216836u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2216840u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2216844u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2216848u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2216852u32;
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
pub fn block_0x0021d394(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2216856u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2216860u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2216864u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2216868u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2216872u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2216876u32)?;
    emu.sw_no_count(20usize, 2usize, 24u32, 2216880u32)?;
    emu.sw_no_count(21usize, 2usize, 20u32, 2216884u32)?;
    emu.sw_no_count(22usize, 2usize, 16u32, 2216888u32)?;
    emu.sw_no_count(23usize, 2usize, 12u32, 2216892u32)?;
    emu.sw_no_count(24usize, 2usize, 8u32, 2216896u32)?;
    emu.sw_no_count(25usize, 2usize, 4u32, 2216900u32)?;
    emu.adi_no_count(9usize, 12usize, 0u32, 2216904u32);
    emu.lw_no_count(18usize, 10usize, 8u32, 2216908u32)?;
    let a = 0u32.wrapping_add(402653184u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2216912u32;
    emu.update_insn_clock();
    emu.anr_no_count(12usize, 18usize, 12usize, 2216916u32);
    emu.adi_no_count(8usize, 11usize, 0u32, 2216920u32);
    emu.add_memory_rw_events(17usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2217148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d4bc));
    } else {
        emu.pc = 2216924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d3dc));
    }
}
#[inline(always)]
pub fn block_0x0021d3dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 18usize, 3u32, 2216928u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2217028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d444));
    } else {
        emu.pc = 2216932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d3e4));
    }
}
#[inline(always)]
pub fn block_0x0021d3e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 16u32, 2216936u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a >= b {
        emu.pc = 2217220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d504));
    } else {
        emu.pc = 2216940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d3ec));
    }
}
#[inline(always)]
pub fn block_0x0021d3ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2216944u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2216980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d414));
    } else {
        emu.pc = 2216948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d3f4));
    }
}
#[inline(always)]
pub fn block_0x0021d3f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(12usize, 8usize, 9usize, 2216952u32);
    emu.adi_no_count(13usize, 8usize, 0u32, 2216956u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2216956u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d3fc));
}
#[inline(always)]
pub fn block_0x0021d3fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(14usize, 13usize, 0u32, 2216960u32);
    emu.adi_no_count(13usize, 13usize, 1u32, 2216964u32);
    emu.slti_no_count(14usize, 14usize, 4294967232u32, 2216968u32);
    emu.xri_no_count(14usize, 14usize, 1u32, 2216972u32);
    emu.adr_no_count(11usize, 11usize, 14usize, 2216976u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2216956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d3fc));
    } else {
        emu.pc = 2216980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d414));
    }
}
#[inline(always)]
pub fn block_0x0021d414(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lhu_no_count(12usize, 10usize, 12u32, 2216984u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2217148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d4bc));
    } else {
        emu.pc = 2216988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d41c));
    }
}
#[inline(always)]
pub fn block_0x0021d41c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(21usize, 0usize, 0u32, 2216992u32);
    emu.sbr_no_count(20usize, 12usize, 11usize, 2216996u32);
    emu.sli_no_count(11usize, 18usize, 1u32, 2217000u32);
    emu.sri_no_count(11usize, 11usize, 30u32, 2217004u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2217008u32);
    emu.sli_no_count(18usize, 18usize, 11u32, 2217012u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2217260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d52c));
    } else {
        emu.pc = 2217016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d438));
    }
}
#[inline(always)]
pub fn block_0x0021d438(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2217276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d53c));
    } else {
        emu.pc = 2217020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d43c));
    }
}
#[inline(always)]
pub fn block_0x0021d43c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(21usize, 20usize, 0u32, 2217024u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2217028u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2217276u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d53c));
}
#[inline(always)]
pub fn block_0x0021d444(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lhu_no_count(11usize, 10usize, 14u32, 2217032u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2217424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d5d0));
    } else {
        emu.pc = 2217036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d44c));
    }
}
#[inline(always)]
pub fn block_0x0021d44c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(13usize, 8usize, 9usize, 2217040u32);
    emu.adi_no_count(14usize, 0usize, 224u32, 2217044u32);
    emu.adi_no_count(15usize, 0usize, 240u32, 2217048u32);
    emu.adi_no_count(16usize, 8usize, 0u32, 2217052u32);
    emu.adi_no_count(12usize, 11usize, 0u32, 2217056u32);
    emu.adi_no_count(9usize, 0usize, 0u32, 2217060u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2217064u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2217088u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d480));
}
#[inline(always)]
pub fn block_0x0021d468(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 16usize, 1u32, 2217068u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2217068u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d46c));
}
#[inline(always)]
pub fn block_0x0021d46c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(16usize, 16usize, 9usize, 2217072u32);
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2217076u32);
    emu.sbr_no_count(9usize, 17usize, 16usize, 2217080u32);
    emu.adi_no_count(16usize, 17usize, 0u32, 2217084u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2217136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d4b0));
    } else {
        emu.pc = 2217088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d480));
    }
}
#[inline(always)]
pub fn block_0x0021d480(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2217136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d4b0));
    } else {
        emu.pc = 2217092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d484));
    }
}
#[inline(always)]
pub fn block_0x0021d484(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(17usize, 16usize, 0u32, 2217096u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2217064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d468));
    } else {
        emu.pc = 2217100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d48c));
    }
}
#[inline(always)]
pub fn block_0x0021d48c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(17usize, 17usize, 255u32, 2217104u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a < b {
        emu.pc = 2217120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d4a0));
    } else {
        emu.pc = 2217108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d494));
    }
}
#[inline(always)]
pub fn block_0x0021d494(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a < b {
        emu.pc = 2217128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d4a8));
    } else {
        emu.pc = 2217112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d498));
    }
}
#[inline(always)]
pub fn block_0x0021d498(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 16usize, 4u32, 2217116u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2217120u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2217068u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d46c));
}
#[inline(always)]
pub fn block_0x0021d4a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 16usize, 2u32, 2217124u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2217128u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2217068u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d46c));
}
#[inline(always)]
pub fn block_0x0021d4a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 16usize, 3u32, 2217132u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2217136u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2217068u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d46c));
}
#[inline(always)]
pub fn block_0x0021d4b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 11usize, 12usize, 2217140u32);
    emu.lhu_no_count(12usize, 10usize, 12u32, 2217144u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2216988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d41c));
    } else {
        emu.pc = 2217148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d4bc));
    }
}
#[inline]
pub fn block_0x0021d4bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 4u32, 2217152u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2217156u32)?;
    emu.lw_no_count(6usize, 11usize, 12u32, 2217160u32)?;
    emu.adi_no_count(11usize, 8usize, 0u32, 2217164u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2217168u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2217172u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2217176u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2217180u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2217184u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2217188u32)?;
    emu.lw_no_count(20usize, 2usize, 24u32, 2217192u32)?;
    emu.lw_no_count(21usize, 2usize, 20u32, 2217196u32)?;
    emu.lw_no_count(22usize, 2usize, 16u32, 2217200u32)?;
    emu.lw_no_count(23usize, 2usize, 12u32, 2217204u32)?;
    emu.lw_no_count(24usize, 2usize, 8u32, 2217208u32)?;
    emu.lw_no_count(25usize, 2usize, 4u32, 2217212u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2217216u32);
    emu.add_memory_rw_events(18usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2217220u32;
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
pub fn block_0x0021d504(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 10usize, 0u32, 2217224u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2217228u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2217232u32);
    emu.apc_no_count(1usize, 2217232u32, 4096u32, 2217236u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2217240u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(84u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021d518(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 10usize, 0u32, 2217244u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2217248u32);
    emu.lhu_no_count(12usize, 19usize, 12u32, 2217252u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2217148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d4bc));
    } else {
        emu.pc = 2217256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d528));
    }
}
#[inline(always)]
pub fn block_0x0021d528(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2217260u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2216988u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d41c));
}
#[inline(always)]
pub fn block_0x0021d52c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 2u32, 2217264u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2217276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d53c));
    } else {
        emu.pc = 2217268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d534));
    }
}
#[inline(always)]
pub fn block_0x0021d534(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 20usize, 16u32, 2217272u32);
    emu.sri_no_count(21usize, 11usize, 17u32, 2217276u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2217276u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d53c));
}
#[inline(always)]
pub fn block_0x0021d53c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(23usize, 0usize, 0u32, 2217280u32);
    emu.sri_no_count(18usize, 18usize, 11u32, 2217284u32);
    emu.lw_no_count(19usize, 10usize, 0u32, 2217288u32)?;
    emu.lw_no_count(22usize, 10usize, 4u32, 2217292u32)?;
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(24usize, a);
    emu.pc = 2217296u32;
    emu.update_insn_clock();
    emu.adi_no_count(24usize, 24usize, 4294967295u32, 2217300u32);
    emu.anr_no_count(25usize, 21usize, 24usize, 2217304u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2217304u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d558));
}
#[inline(always)]
pub fn block_0x0021d558(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(10usize, 23usize, 24usize, 2217308u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(25usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2217340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d57c));
    } else {
        emu.pc = 2217312u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d560));
    }
}
#[inline(always)]
pub fn block_0x0021d560(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 22usize, 16u32, 2217316u32)?;
    emu.adi_no_count(23usize, 23usize, 1u32, 2217320u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2217324u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2217328u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2217332u32;
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
pub fn block_0x0021d574(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2217304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d558));
    } else {
        emu.pc = 2217336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d578));
    }
}
#[inline(always)]
pub fn block_0x0021d578(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2217340u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2217364u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d594));
}
#[inline(always)]
pub fn block_0x0021d57c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 22usize, 12u32, 2217344u32)?;
    emu.adi_no_count(10usize, 19usize, 0u32, 2217348u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2217352u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2217356u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2217360u32;
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
pub fn block_0x0021d590(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2217444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d5e4));
    } else {
        emu.pc = 2217364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d594));
    }
}
#[inline(always)]
pub fn block_0x0021d594(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 0usize, 1u32, 2217368u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2217368u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d598));
}
#[inline]
pub fn block_0x0021d598(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 0u32, 2217372u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2217376u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2217380u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2217384u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2217388u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2217392u32)?;
    emu.lw_no_count(20usize, 2usize, 24u32, 2217396u32)?;
    emu.lw_no_count(21usize, 2usize, 20u32, 2217400u32)?;
    emu.lw_no_count(22usize, 2usize, 16u32, 2217404u32)?;
    emu.lw_no_count(23usize, 2usize, 12u32, 2217408u32)?;
    emu.lw_no_count(24usize, 2usize, 8u32, 2217412u32)?;
    emu.lw_no_count(25usize, 2usize, 4u32, 2217416u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2217420u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2217424u32;
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
pub fn block_0x0021d5d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2217428u32);
    emu.sbr_no_count(11usize, 11usize, 0usize, 2217432u32);
    emu.lhu_no_count(12usize, 10usize, 12u32, 2217436u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2216988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d41c));
    } else {
        emu.pc = 2217440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d5e0));
    }
}
#[inline(always)]
pub fn block_0x0021d5e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2217444u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2217148u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d4bc));
}
#[inline(always)]
pub fn block_0x0021d5e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2217448u32);
    emu.sbr_no_count(10usize, 20usize, 21usize, 2217452u32);
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(20usize, a);
    emu.pc = 2217456u32;
    emu.update_insn_clock();
    emu.adi_no_count(20usize, 20usize, 4294967295u32, 2217460u32);
    emu.anr_no_count(21usize, 10usize, 20usize, 2217464u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2217464u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d5f8));
}
#[inline(always)]
pub fn block_0x0021d5f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(10usize, 9usize, 20usize, 2217468u32);
    emu.sltru_no_count(8usize, 10usize, 21usize, 2217472u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2217368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d598));
    } else {
        emu.pc = 2217476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d604));
    }
}
#[inline(always)]
pub fn block_0x0021d604(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 22usize, 16u32, 2217480u32)?;
    emu.adi_no_count(9usize, 9usize, 1u32, 2217484u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2217488u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2217492u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2217496u32;
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
pub fn block_0x0021d618(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2217464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d5f8));
    } else {
        emu.pc = 2217500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d61c));
    }
}
#[inline(always)]
pub fn block_0x0021d61c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2217504u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2217368u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d598));
}
#[inline(always)]
pub fn block_0x0021d620(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 4u32, 2217508u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2217512u32)?;
    emu.lw_no_count(6usize, 13usize, 12u32, 2217516u32)?;
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2217520u32;
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
pub fn block_0x0021d630(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2217524u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2217528u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2217532u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2217536u32)?;
    emu.adi_no_count(8usize, 11usize, 0u32, 2217540u32);
    emu.lw_no_count(14usize, 11usize, 4u32, 2217544u32)?;
    emu.lw_no_count(11usize, 11usize, 0u32, 2217548u32)?;
    emu.lw_no_count(14usize, 14usize, 12u32, 2217552u32)?;
    emu.adi_no_count(9usize, 10usize, 0u32, 2217556u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2217560u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2217564u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2217568u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2217572u32;
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
pub fn block_0x0021d664(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(8usize, 9usize, 0u32, 2217576u32)?;
    emu.sb_no_count(10usize, 9usize, 4u32, 2217580u32);
    emu.sb_no_count(0usize, 9usize, 5u32, 2217584u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2217588u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2217592u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2217596u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2217600u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2217604u32;
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
pub fn block_0x0021d684(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2217608u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2217612u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2217616u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2217620u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2217624u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2217628u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2217632u32)?;
    emu.adi_no_count(8usize, 16usize, 0u32, 2217636u32);
    emu.adi_no_count(9usize, 15usize, 0u32, 2217640u32);
    emu.adi_no_count(18usize, 14usize, 0u32, 2217644u32);
    emu.adi_no_count(19usize, 13usize, 0u32, 2217648u32);
    emu.adi_no_count(20usize, 10usize, 0u32, 2217652u32);
    emu.lw_no_count(13usize, 10usize, 4u32, 2217656u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2217660u32)?;
    emu.lw_no_count(13usize, 13usize, 12u32, 2217664u32)?;
    emu.add_memory_rw_events(16usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2217668u32;
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
pub fn block_0x0021d6c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(20usize, 2usize, 0u32, 2217672u32)?;
    emu.sb_no_count(10usize, 2usize, 4u32, 2217676u32);
    emu.sb_no_count(0usize, 2usize, 5u32, 2217680u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2217684u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2217688u32);
    emu.adi_no_count(12usize, 18usize, 0u32, 2217692u32);
    emu.adi_no_count(13usize, 9usize, 0u32, 2217696u32);
    emu.adi_no_count(14usize, 8usize, 0u32, 2217700u32);
    emu.apc_no_count(1usize, 2217700u32, 4294959104u32, 2217704u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2217708u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1760u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021d6ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 2usize, 4u32, 2217712u32);
    emu.lbu_no_count(12usize, 2usize, 5u32, 2217716u32);
    emu.orr_no_count(10usize, 12usize, 11usize, 2217720u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2217804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d74c));
    } else {
        emu.pc = 2217724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d6fc));
    }
}
#[inline(always)]
pub fn block_0x0021d6fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 11usize, 1u32, 2217728u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2217804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d74c));
    } else {
        emu.pc = 2217732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d704));
    }
}
#[inline(always)]
pub fn block_0x0021d704(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 0u32, 2217736u32)?;
    emu.lbu_no_count(11usize, 10usize, 10u32, 2217740u32);
    emu.ani_no_count(11usize, 11usize, 128u32, 2217744u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2217776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d730));
    } else {
        emu.pc = 2217748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d714));
    }
}
#[inline(always)]
pub fn block_0x0021d714(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 4u32, 2217752u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2217756u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2217760u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2217764u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 2005u32, 2217768u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2217772u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2217776u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2217800u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d748));
}
#[inline(always)]
pub fn block_0x0021d730(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 4u32, 2217780u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2217784u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2217788u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2217792u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 2004u32, 2217796u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2217800u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2217800u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d748));
}
#[inline(always)]
pub fn block_0x0021d748(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2217804u32;
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
pub fn block_0x0021d74c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 1u32, 2217808u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2217812u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2217816u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2217820u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2217824u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2217828u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2217832u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2217836u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2217840u32;
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
pub fn block_0x0021d770(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2217844u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2217848u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2217852u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2217856u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2217860u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2217864u32)?;
    emu.sw_no_count(20usize, 2usize, 24u32, 2217868u32)?;
    emu.sw_no_count(21usize, 2usize, 20u32, 2217872u32)?;
    emu.sw_no_count(22usize, 2usize, 16u32, 2217876u32)?;
    emu.sw_no_count(23usize, 2usize, 12u32, 2217880u32)?;
    emu.sw_no_count(24usize, 2usize, 8u32, 2217884u32)?;
    emu.adi_no_count(8usize, 17usize, 0u32, 2217888u32);
    emu.adi_no_count(9usize, 16usize, 0u32, 2217892u32);
    emu.adi_no_count(18usize, 15usize, 0u32, 2217896u32);
    emu.adi_no_count(19usize, 14usize, 0u32, 2217900u32);
    emu.adi_no_count(20usize, 13usize, 0u32, 2217904u32);
    emu.adi_no_count(21usize, 10usize, 0u32, 2217908u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2217912u32)?;
    emu.lw_no_count(13usize, 21usize, 4u32, 2217916u32)?;
    emu.lw_no_count(22usize, 2usize, 48u32, 2217920u32)?;
    emu.lw_no_count(23usize, 2usize, 52u32, 2217924u32)?;
    emu.lw_no_count(24usize, 2usize, 56u32, 2217928u32)?;
    emu.lw_no_count(13usize, 13usize, 12u32, 2217932u32)?;
    emu.add_memory_rw_events(24usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2217936u32;
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
pub fn block_0x0021d7d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(21usize, 2usize, 0u32, 2217940u32)?;
    emu.sb_no_count(10usize, 2usize, 4u32, 2217944u32);
    emu.sb_no_count(0usize, 2usize, 5u32, 2217948u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2217952u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2217956u32);
    emu.adi_no_count(12usize, 19usize, 0u32, 2217960u32);
    emu.adi_no_count(13usize, 18usize, 0u32, 2217964u32);
    emu.adi_no_count(14usize, 9usize, 0u32, 2217968u32);
    emu.apc_no_count(1usize, 2217968u32, 4294959104u32, 2217972u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2217976u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1492u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021d7f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2217980u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2217984u32);
    emu.adi_no_count(12usize, 22usize, 0u32, 2217988u32);
    emu.adi_no_count(13usize, 23usize, 0u32, 2217992u32);
    emu.adi_no_count(14usize, 24usize, 0u32, 2217996u32);
    emu.apc_no_count(1usize, 2217996u32, 4294959104u32, 2218000u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2218004u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1464u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021d814(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 2usize, 4u32, 2218008u32);
    emu.lbu_no_count(12usize, 2usize, 5u32, 2218012u32);
    emu.orr_no_count(10usize, 12usize, 11usize, 2218016u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2218100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d874));
    } else {
        emu.pc = 2218020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d824));
    }
}
#[inline(always)]
pub fn block_0x0021d824(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 11usize, 1u32, 2218024u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2218100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d874));
    } else {
        emu.pc = 2218028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d82c));
    }
}
#[inline(always)]
pub fn block_0x0021d82c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 0u32, 2218032u32)?;
    emu.lbu_no_count(11usize, 10usize, 10u32, 2218036u32);
    emu.ani_no_count(11usize, 11usize, 128u32, 2218040u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2218072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d858));
    } else {
        emu.pc = 2218044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d83c));
    }
}
#[inline(always)]
pub fn block_0x0021d83c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 4u32, 2218048u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2218052u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2218056u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2218060u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 2005u32, 2218064u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2218068u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2218072u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2218096u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d870));
}
#[inline(always)]
pub fn block_0x0021d858(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 4u32, 2218076u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2218080u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2218084u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2218088u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 2004u32, 2218092u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2218096u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2218096u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d870));
}
#[inline(always)]
pub fn block_0x0021d870(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2218100u32;
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
pub fn block_0x0021d874(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 1u32, 2218104u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2218108u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2218112u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2218116u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2218120u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2218124u32)?;
    emu.lw_no_count(20usize, 2usize, 24u32, 2218128u32)?;
    emu.lw_no_count(21usize, 2usize, 20u32, 2218132u32)?;
    emu.lw_no_count(22usize, 2usize, 16u32, 2218136u32)?;
    emu.lw_no_count(23usize, 2usize, 12u32, 2218140u32)?;
    emu.lw_no_count(24usize, 2usize, 8u32, 2218144u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2218148u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2218152u32;
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
pub fn block_0x0021d8a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2218156u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2218160u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2218164u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2218168u32)?;
    emu.sw_no_count(18usize, 2usize, 0u32, 2218172u32)?;
    emu.adi_no_count(8usize, 13usize, 0u32, 2218176u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2218180u32);
    emu.lw_no_count(13usize, 11usize, 4u32, 2218184u32)?;
    emu.lw_no_count(11usize, 11usize, 0u32, 2218188u32)?;
    emu.lw_no_count(13usize, 13usize, 12u32, 2218192u32)?;
    emu.adi_no_count(18usize, 10usize, 0u32, 2218196u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2218200u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2218204u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2218208u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2218212u32;
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
pub fn block_0x0021d8e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(11usize, 8usize, 1u32, 2218216u32);
    emu.sw_no_count(0usize, 18usize, 0u32, 2218220u32)?;
    emu.sw_no_count(9usize, 18usize, 4u32, 2218224u32)?;
    emu.sb_no_count(10usize, 18usize, 8u32, 2218228u32);
    emu.sb_no_count(11usize, 18usize, 9u32, 2218232u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2218236u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2218240u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2218244u32)?;
    emu.lw_no_count(18usize, 2usize, 0u32, 2218248u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2218252u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2218256u32;
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
pub fn block_0x0021d910(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967216u32, 2218260u32);
    emu.sw_no_count(1usize, 2usize, 76u32, 2218264u32)?;
    emu.sw_no_count(8usize, 2usize, 72u32, 2218268u32)?;
    emu.sw_no_count(9usize, 2usize, 68u32, 2218272u32)?;
    emu.sw_no_count(18usize, 2usize, 64u32, 2218276u32)?;
    emu.sw_no_count(19usize, 2usize, 60u32, 2218280u32)?;
    emu.sw_no_count(20usize, 2usize, 56u32, 2218284u32)?;
    emu.sw_no_count(21usize, 2usize, 52u32, 2218288u32)?;
    emu.sw_no_count(22usize, 2usize, 48u32, 2218292u32)?;
    emu.sw_no_count(23usize, 2usize, 44u32, 2218296u32)?;
    emu.adi_no_count(20usize, 14usize, 0u32, 2218300u32);
    emu.adi_no_count(19usize, 13usize, 0u32, 2218304u32);
    emu.adi_no_count(9usize, 12usize, 0u32, 2218308u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2218312u32);
    emu.lw_no_count(22usize, 10usize, 4u32, 2218316u32)?;
    emu.lw_no_count(21usize, 10usize, 0u32, 2218320u32)?;
    emu.lw_no_count(23usize, 22usize, 12u32, 2218324u32)?;
    emu.adi_no_count(10usize, 21usize, 0u32, 2218328u32);
    emu.add_memory_rw_events(19usize);
    let base = emu.read_reg_b_tracked(23usize);
    let return_addr = 2218332u32;
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
pub fn block_0x0021d95c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 1u32, 2218336u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2218388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d994));
    } else {
        emu.pc = 2218340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d964));
    }
}
#[inline]
pub fn block_0x0021d964(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2218344u32);
    emu.lw_no_count(1usize, 2usize, 76u32, 2218348u32)?;
    emu.lw_no_count(8usize, 2usize, 72u32, 2218352u32)?;
    emu.lw_no_count(9usize, 2usize, 68u32, 2218356u32)?;
    emu.lw_no_count(18usize, 2usize, 64u32, 2218360u32)?;
    emu.lw_no_count(19usize, 2usize, 60u32, 2218364u32)?;
    emu.lw_no_count(20usize, 2usize, 56u32, 2218368u32)?;
    emu.lw_no_count(21usize, 2usize, 52u32, 2218372u32)?;
    emu.lw_no_count(22usize, 2usize, 48u32, 2218376u32)?;
    emu.lw_no_count(23usize, 2usize, 44u32, 2218380u32)?;
    emu.adi_no_count(2usize, 2usize, 80u32, 2218384u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2218388u32;
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
pub fn block_0x0021d994(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 8usize, 10u32, 2218392u32);
    emu.ani_no_count(10usize, 10usize, 128u32, 2218396u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2218452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d9d4));
    } else {
        emu.pc = 2218400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d9a0));
    }
}
#[inline(always)]
pub fn block_0x0021d9a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2218404u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 2007u32, 2218408u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2218412u32);
    emu.adi_no_count(18usize, 0usize, 1u32, 2218416u32);
    emu.adi_no_count(10usize, 21usize, 0u32, 2218420u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(23usize);
    let return_addr = 2218424u32;
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
pub fn block_0x0021d9b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2218340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d964));
    } else {
        emu.pc = 2218428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d9bc));
    }
}
#[inline(always)]
pub fn block_0x0021d9bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 20usize, 12u32, 2218432u32)?;
    emu.adi_no_count(10usize, 19usize, 0u32, 2218436u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2218440u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2218444u32;
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
pub fn block_0x0021d9cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2218340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d964));
    } else {
        emu.pc = 2218448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d9d0));
    }
}
#[inline(always)]
pub fn block_0x0021d9d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2218452u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2218588u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021da5c));
}
#[inline(always)]
pub fn block_0x0021d9d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2218456u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 2008u32, 2218460u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2218464u32);
    emu.adi_no_count(10usize, 21usize, 0u32, 2218468u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(23usize);
    let return_addr = 2218472u32;
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
pub fn block_0x0021d9e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2218340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d964));
    } else {
        emu.pc = 2218476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d9ec));
    }
}
#[inline]
pub fn block_0x0021d9ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 1u32, 2218480u32);
    emu.adi_no_count(10usize, 2usize, 27u32, 2218484u32);
    emu.lw_no_count(11usize, 8usize, 8u32, 2218488u32)?;
    emu.lw_no_count(12usize, 8usize, 12u32, 2218492u32)?;
    emu.adi_no_count(13usize, 2usize, 12u32, 2218496u32);
    emu.sw_no_count(21usize, 2usize, 12u32, 2218500u32)?;
    emu.sw_no_count(22usize, 2usize, 16u32, 2218504u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2218508u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2218512u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1968u32, 2218516u32);
    emu.lw_no_count(14usize, 20usize, 12u32, 2218520u32)?;
    emu.sb_no_count(18usize, 2usize, 27u32, 2218524u32);
    emu.sw_no_count(13usize, 2usize, 28u32, 2218528u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2218532u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2218536u32)?;
    emu.sw_no_count(12usize, 2usize, 40u32, 2218540u32)?;
    emu.adi_no_count(11usize, 2usize, 28u32, 2218544u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2218548u32);
    emu.add_memory_rw_events(19usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2218552u32;
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
pub fn block_0x0021da38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2218340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d964));
    } else {
        emu.pc = 2218556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021da3c));
    }
}
#[inline(always)]
pub fn block_0x0021da3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 32u32, 2218560u32)?;
    emu.lw_no_count(10usize, 2usize, 28u32, 2218564u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2218568u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2218572u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 2002u32, 2218576u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2218580u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2218584u32;
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
pub fn block_0x0021da58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2218340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d964));
    } else {
        emu.pc = 2218588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021da5c));
    }
}
#[inline(always)]
pub fn block_0x0021da5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2218640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021da90));
    } else {
        emu.pc = 2218592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021da60));
    }
}
#[inline(always)]
pub fn block_0x0021da60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 8usize, 10u32, 2218596u32);
    emu.ani_no_count(10usize, 10usize, 128u32, 2218600u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2218640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021da90));
    } else {
        emu.pc = 2218604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021da6c));
    }
}
#[inline(always)]
pub fn block_0x0021da6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 8usize, 4u32, 2218608u32)?;
    emu.lw_no_count(10usize, 8usize, 0u32, 2218612u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2218616u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2218620u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 2011u32, 2218624u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2218628u32);
    emu.adi_no_count(18usize, 0usize, 1u32, 2218632u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2218636u32;
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
pub fn block_0x0021da8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2218340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d964));
    } else {
        emu.pc = 2218640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021da90));
    }
}
#[inline(always)]
pub fn block_0x0021da90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 8usize, 4u32, 2218644u32)?;
    emu.lw_no_count(10usize, 8usize, 0u32, 2218648u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2218652u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2218656u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 2010u32, 2218660u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2218664u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2218668u32;
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
pub fn block_0x0021daac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2218672u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2218676u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2218340u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d964));
}
#[inline]
pub fn block_0x0021dab4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2218680u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2218684u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2218688u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2218692u32)?;
    emu.adi_no_count(8usize, 11usize, 0u32, 2218696u32);
    emu.lw_no_count(11usize, 11usize, 4u32, 2218700u32)?;
    emu.lw_no_count(13usize, 8usize, 0u32, 2218704u32)?;
    emu.lw_no_count(14usize, 11usize, 12u32, 2218708u32)?;
    emu.adi_no_count(9usize, 10usize, 0u32, 2218712u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2218716u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 2013u32, 2218720u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2218724u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2218728u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2218732u32;
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
pub fn block_0x0021daec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(8usize, 9usize, 0u32, 2218736u32)?;
    emu.sb_no_count(10usize, 9usize, 4u32, 2218740u32);
    emu.sb_no_count(0usize, 9usize, 5u32, 2218744u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2218748u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2218752u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2218756u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2218760u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2218764u32;
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
pub fn block_0x0021db0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(12usize, 10usize, 0u32, 2218768u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2218772u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2218796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021db2c));
    } else {
        emu.pc = 2218776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021db18));
    }
}
#[inline(always)]
pub fn block_0x0021db18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2218780u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966112u32, 2218784u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2218788u32);
    emu.apc_no_count(6usize, 2218788u32, 0u32, 2218792u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2218796u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x0021db2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2218800u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966328u32, 2218804u32);
    emu.adi_no_count(12usize, 0usize, 5u32, 2218808u32);
    emu.apc_no_count(6usize, 2218808u32, 0u32, 2218812u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2218816u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965340u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021db40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967200u32, 2218820u32);
    emu.sw_no_count(1usize, 2usize, 92u32, 2218824u32)?;
    emu.sw_no_count(8usize, 2usize, 88u32, 2218828u32)?;
    emu.sw_no_count(9usize, 2usize, 84u32, 2218832u32)?;
    emu.sw_no_count(18usize, 2usize, 80u32, 2218836u32)?;
    emu.sw_no_count(19usize, 2usize, 76u32, 2218840u32)?;
    emu.sw_no_count(20usize, 2usize, 72u32, 2218844u32)?;
    emu.sw_no_count(21usize, 2usize, 68u32, 2218848u32)?;
    emu.sw_no_count(22usize, 2usize, 64u32, 2218852u32)?;
    emu.sw_no_count(23usize, 2usize, 60u32, 2218856u32)?;
    emu.sw_no_count(24usize, 2usize, 56u32, 2218860u32)?;
    emu.sw_no_count(25usize, 2usize, 52u32, 2218864u32)?;
    emu.sw_no_count(26usize, 2usize, 48u32, 2218868u32)?;
    emu.sw_no_count(27usize, 2usize, 44u32, 2218872u32)?;
    emu.adi_no_count(19usize, 11usize, 0u32, 2218876u32);
    emu.lw_no_count(21usize, 12usize, 4u32, 2218880u32)?;
    emu.lw_no_count(8usize, 12usize, 0u32, 2218884u32)?;
    emu.lw_no_count(9usize, 21usize, 16u32, 2218888u32)?;
    emu.adi_no_count(23usize, 10usize, 0u32, 2218892u32);
    emu.adi_no_count(11usize, 0usize, 34u32, 2218896u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2218900u32);
    emu.add_memory_rw_events(22usize);
    let base = emu.read_reg_b_tracked(9usize);
    let return_addr = 2218904u32;
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
pub fn block_0x0021db98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 1u32, 2218908u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2219660u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021de8c));
    } else {
        emu.pc = 2218912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dba0));
    }
}
#[inline(always)]
pub fn block_0x0021dba0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(8usize, 2usize, 24u32, 2218916u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2219560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021de28));
    } else {
        emu.pc = 2218920u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dba8));
    }
}
#[inline]
pub fn block_0x0021dba8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(21usize, 2usize, 16u32, 2218924u32)?;
    emu.sw_no_count(9usize, 2usize, 12u32, 2218928u32)?;
    emu.adi_no_count(20usize, 0usize, 0u32, 2218932u32);
    emu.adi_no_count(9usize, 0usize, 0u32, 2218936u32);
    emu.sbr_no_count(10usize, 0usize, 19usize, 2218940u32);
    emu.sw_no_count(10usize, 2usize, 8u32, 2218944u32)?;
    emu.adi_no_count(27usize, 0usize, 4294967201u32, 2218948u32);
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2218952u32;
    emu.update_insn_clock();
    emu.adi_no_count(24usize, 0usize, 1u32, 2218956u32);
    emu.adi_no_count(26usize, 0usize, 34u32, 2218960u32);
    emu.adi_no_count(10usize, 10usize, 1u32, 2218964u32);
    emu.sw_no_count(10usize, 2usize, 20u32, 2218968u32)?;
    emu.adi_no_count(21usize, 0usize, 92u32, 2218972u32);
    emu.adi_no_count(18usize, 23usize, 0u32, 2218976u32);
    emu.adi_no_count(13usize, 19usize, 0u32, 2218980u32);
    emu.add_memory_rw_events(16usize);
    let return_addr = 2218984u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2219004u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021dbfc));
}
#[inline(always)]
pub fn block_0x0021dbe8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2218988u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2218988u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021dbec));
}
#[inline(always)]
pub fn block_0x0021dbec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 10usize, 9usize, 2218992u32);
    emu.sbr_no_count(13usize, 8usize, 18usize, 2218996u32);
    emu.adr_no_count(9usize, 10usize, 25usize, 2219000u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2219732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ded4));
    } else {
        emu.pc = 2219004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dbfc));
    }
}
#[inline(always)]
pub fn block_0x0021dbfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 0usize, 0u32, 2219008u32);
    emu.adr_no_count(8usize, 18usize, 13usize, 2219012u32);
    emu.sbr_no_count(11usize, 0usize, 13usize, 2219016u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2219016u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021dc08));
}
#[inline(always)]
pub fn block_0x0021dc08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 18usize, 25usize, 2219020u32);
    emu.lbu_no_count(12usize, 10usize, 0u32, 2219024u32);
    emu.adi_no_count(14usize, 12usize, 4294967169u32, 2219028u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2219056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dc30));
    } else {
        emu.pc = 2219032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dc18));
    }
}
#[inline(always)]
pub fn block_0x0021dc18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2219056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dc30));
    } else {
        emu.pc = 2219036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dc1c));
    }
}
#[inline(always)]
pub fn block_0x0021dc1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2219056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dc30));
    } else {
        emu.pc = 2219040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dc20));
    }
}
#[inline(always)]
pub fn block_0x0021dc20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 25usize, 1u32, 2219044u32);
    emu.adr_no_count(10usize, 11usize, 25usize, 2219048u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2219016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dc08));
    } else {
        emu.pc = 2219052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dc2c));
    }
}
#[inline(always)]
pub fn block_0x0021dc2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2219056u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2219512u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ddf8));
}
#[inline(always)]
pub fn block_0x0021dc30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(11usize, 10usize, 0u32, 2219060u32);
    emu.adi_no_count(18usize, 10usize, 1u32, 2219064u32);
    emu.ani_no_count(22usize, 11usize, 255u32, 2219068u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2219184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dcb0));
    } else {
        emu.pc = 2219072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dc40));
    }
}
#[inline(always)]
pub fn block_0x0021dc40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(12usize, 18usize, 0u32, 2219076u32);
    emu.ani_no_count(11usize, 22usize, 31u32, 2219080u32);
    emu.adi_no_count(18usize, 10usize, 2u32, 2219084u32);
    emu.ani_no_count(13usize, 12usize, 63u32, 2219088u32);
    emu.adi_no_count(12usize, 0usize, 224u32, 2219092u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a < b {
        emu.pc = 2219160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dc98));
    } else {
        emu.pc = 2219096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dc58));
    }
}
#[inline(always)]
pub fn block_0x0021dc58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(14usize, 18usize, 0u32, 2219100u32);
    emu.adi_no_count(12usize, 10usize, 3u32, 2219104u32);
    emu.sli_no_count(13usize, 13usize, 6u32, 2219108u32);
    emu.ani_no_count(14usize, 14usize, 63u32, 2219112u32);
    emu.orr_no_count(13usize, 13usize, 14usize, 2219116u32);
    emu.adi_no_count(14usize, 0usize, 240u32, 2219120u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a < b {
        emu.pc = 2219172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dca4));
    } else {
        emu.pc = 2219124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dc74));
    }
}
#[inline]
pub fn block_0x0021dc74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 4u32, 2219128u32);
    emu.lbu_no_count(10usize, 12usize, 0u32, 2219132u32);
    emu.sli_no_count(11usize, 11usize, 29u32, 2219136u32);
    emu.sri_no_count(11usize, 11usize, 11u32, 2219140u32);
    emu.sli_no_count(13usize, 13usize, 6u32, 2219144u32);
    emu.ani_no_count(10usize, 10usize, 63u32, 2219148u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2219152u32);
    emu.orr_no_count(22usize, 13usize, 10usize, 2219156u32);
    emu.add_memory_rw_events(9usize);
    let return_addr = 2219160u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2219184u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021dcb0));
}
#[inline(always)]
pub fn block_0x0021dc98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 11usize, 6u32, 2219164u32);
    emu.orr_no_count(22usize, 11usize, 13usize, 2219168u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2219172u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2219184u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021dcb0));
}
#[inline(always)]
pub fn block_0x0021dca4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 11usize, 12u32, 2219176u32);
    emu.orr_no_count(22usize, 13usize, 11usize, 2219180u32);
    emu.adi_no_count(18usize, 12usize, 0u32, 2219184u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2219184u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021dcb0));
}
#[inline(always)]
pub fn block_0x0021dcb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 28u32, 2219188u32);
    emu.adi_no_count(11usize, 22usize, 0u32, 2219192u32);
    emu.lw_no_count(12usize, 2usize, 20u32, 2219196u32)?;
    emu.apc_no_count(1usize, 2219196u32, 4294963200u32, 2219200u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2219204u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966404u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021dcc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 40u32, 2219208u32);
    emu.lbu_no_count(11usize, 2usize, 41u32, 2219212u32);
    emu.sbr_no_count(11usize, 11usize, 10usize, 2219216u32);
    emu.ani_no_count(10usize, 11usize, 255u32, 2219220u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2219472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ddd0));
    } else {
        emu.pc = 2219224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dcd8));
    }
}
#[inline(always)]
pub fn block_0x0021dcd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(13usize, 9usize, 25usize, 2219228u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2219780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021df04));
    } else {
        emu.pc = 2219232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dce0));
    }
}
#[inline(always)]
pub fn block_0x0021dce0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 23usize, 20usize, 2219236u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2219264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dd00));
    } else {
        emu.pc = 2219240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dce8));
    }
}
#[inline(always)]
pub fn block_0x0021dce8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a >= b {
        emu.pc = 2219260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dcfc));
    } else {
        emu.pc = 2219244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dcec));
    }
}
#[inline(always)]
pub fn block_0x0021dcec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(10usize, 11usize, 0u32, 2219248u32);
    emu.adi_no_count(12usize, 0usize, 4294967231u32, 2219252u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2219264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dd00));
    } else {
        emu.pc = 2219256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dcf8));
    }
}
#[inline(always)]
pub fn block_0x0021dcf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2219260u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2219780u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021df04));
}
#[inline(always)]
pub fn block_0x0021dcfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a != b {
        emu.pc = 2219780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021df04));
    } else {
        emu.pc = 2219264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dd00));
    }
}
#[inline(always)]
pub fn block_0x0021dd00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2219308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dd2c));
    } else {
        emu.pc = 2219268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dd04));
    }
}
#[inline(always)]
pub fn block_0x0021dd04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2219296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dd20));
    } else {
        emu.pc = 2219272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dd08));
    }
}
#[inline(always)]
pub fn block_0x0021dd08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 23usize, 9usize, 2219276u32);
    emu.adr_no_count(10usize, 10usize, 25usize, 2219280u32);
    emu.lb_no_count(10usize, 10usize, 0u32, 2219284u32);
    emu.adi_no_count(12usize, 0usize, 4294967231u32, 2219288u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2219308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dd2c));
    } else {
        emu.pc = 2219292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dd1c));
    }
}
#[inline(always)]
pub fn block_0x0021dd1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2219296u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2219780u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021df04));
}
#[inline(always)]
pub fn block_0x0021dd20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2219300u32)?;
    emu.adr_no_count(10usize, 13usize, 10usize, 2219304u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2219780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021df04));
    } else {
        emu.pc = 2219308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dd2c));
    }
}
#[inline(always)]
pub fn block_0x0021dd2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(24usize, 23usize, 0u32, 2219312u32);
    emu.lw_no_count(10usize, 2usize, 16u32, 2219316u32)?;
    emu.lw_no_count(23usize, 10usize, 12u32, 2219320u32)?;
    emu.sbr_no_count(12usize, 9usize, 20usize, 2219324u32);
    emu.adr_no_count(12usize, 12usize, 25usize, 2219328u32);
    emu.lw_no_count(20usize, 2usize, 24u32, 2219332u32)?;
    emu.adi_no_count(10usize, 20usize, 0u32, 2219336u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(23usize);
    let return_addr = 2219340u32;
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
pub fn block_0x0021dd4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2219724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021decc));
    } else {
        emu.pc = 2219344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dd50));
    }
}
#[inline(always)]
pub fn block_0x0021dd50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 41u32, 2219348u32);
    emu.adi_no_count(11usize, 0usize, 129u32, 2219352u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2219380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dd74));
    } else {
        emu.pc = 2219356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dd5c));
    }
}
#[inline(always)]
pub fn block_0x0021dd5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 28u32, 2219360u32)?;
    emu.adi_no_count(10usize, 20usize, 0u32, 2219364u32);
    emu.lw_no_count(12usize, 2usize, 12u32, 2219368u32)?;
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2219372u32;
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
pub fn block_0x0021dd6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2219408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dd90));
    } else {
        emu.pc = 2219376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dd70));
    }
}
#[inline(always)]
pub fn block_0x0021dd70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2219380u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2219724u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021decc));
}
#[inline(always)]
pub fn block_0x0021dd74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 2usize, 40u32, 2219384u32);
    emu.sbr_no_count(12usize, 10usize, 11usize, 2219388u32);
    emu.adi_no_count(10usize, 2usize, 28u32, 2219392u32);
    emu.adr_no_count(11usize, 10usize, 11usize, 2219396u32);
    emu.adi_no_count(10usize, 20usize, 0u32, 2219400u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(23usize);
    let return_addr = 2219404u32;
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
pub fn block_0x0021dd8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2219724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021decc));
    } else {
        emu.pc = 2219408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dd90));
    }
}
#[inline(always)]
pub fn block_0x0021dd90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 128u32, 2219412u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a >= b {
        emu.pc = 2219428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dda4));
    } else {
        emu.pc = 2219416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dd98));
    }
}
#[inline(always)]
pub fn block_0x0021dd98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 1u32, 2219420u32);
    emu.adi_no_count(23usize, 24usize, 0u32, 2219424u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2219428u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2219460u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ddc4));
}
#[inline(always)]
pub fn block_0x0021dda4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 22usize, 11u32, 2219432u32);
    emu.adi_no_count(23usize, 24usize, 0u32, 2219436u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2219448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ddb8));
    } else {
        emu.pc = 2219440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ddb0));
    }
}
#[inline(always)]
pub fn block_0x0021ddb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 2u32, 2219444u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2219448u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2219460u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ddc4));
}
#[inline(always)]
pub fn block_0x0021ddb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 22usize, 16u32, 2219452u32);
    emu.sltru_no_count(20usize, 0usize, 10usize, 2219456u32);
    emu.adi_no_count(20usize, 20usize, 3u32, 2219460u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2219460u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ddc4));
}
#[inline(always)]
pub fn block_0x0021ddc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(24usize, 0usize, 1u32, 2219464u32);
    emu.adr_no_count(10usize, 9usize, 25usize, 2219468u32);
    emu.adr_no_count(20usize, 20usize, 10usize, 2219472u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2219472u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ddd0));
}
#[inline(always)]
pub fn block_0x0021ddd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 128u32, 2219476u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a < b {
        emu.pc = 2218984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dbe8));
    } else {
        emu.pc = 2219480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ddd8));
    }
}
#[inline(always)]
pub fn block_0x0021ddd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 22usize, 11u32, 2219484u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2219496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dde8));
    } else {
        emu.pc = 2219488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dde0));
    }
}
#[inline(always)]
pub fn block_0x0021dde0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2219492u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2219496u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2218988u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021dbec));
}
#[inline(always)]
pub fn block_0x0021dde8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 22usize, 16u32, 2219500u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2219504u32);
    emu.adi_no_count(10usize, 10usize, 3u32, 2219508u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2219512u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2218988u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021dbec));
}
#[inline(always)]
pub fn block_0x0021ddf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(13usize, 13usize, 9usize, 2219516u32);
    emu.lw_no_count(21usize, 2usize, 16u32, 2219520u32)?;
    emu.lw_no_count(9usize, 2usize, 12u32, 2219524u32)?;
    emu.adi_no_count(18usize, 0usize, 1u32, 2219528u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2219752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dee8));
    } else {
        emu.pc = 2219532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021de0c));
    }
}
#[inline(always)]
pub fn block_0x0021de0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2219572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021de34));
    } else {
        emu.pc = 2219536u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021de10));
    }
}
#[inline(always)]
pub fn block_0x0021de10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a >= b {
        emu.pc = 2219568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021de30));
    } else {
        emu.pc = 2219540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021de14));
    }
}
#[inline(always)]
pub fn block_0x0021de14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 23usize, 20usize, 2219544u32);
    emu.lb_no_count(10usize, 10usize, 0u32, 2219548u32);
    emu.adi_no_count(11usize, 0usize, 4294967231u32, 2219552u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2219572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021de34));
    } else {
        emu.pc = 2219556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021de24));
    }
}
#[inline(always)]
pub fn block_0x0021de24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2219560u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2219752u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021dee8));
}
#[inline(always)]
pub fn block_0x0021de28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 0u32, 2219564u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2219568u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2219616u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021de60));
}
#[inline(always)]
pub fn block_0x0021de30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a != b {
        emu.pc = 2219752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dee8));
    } else {
        emu.pc = 2219572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021de34));
    }
}
#[inline(always)]
pub fn block_0x0021de34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2219604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021de54));
    } else {
        emu.pc = 2219576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021de38));
    }
}
#[inline(always)]
pub fn block_0x0021de38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2219612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021de5c));
    } else {
        emu.pc = 2219580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021de3c));
    }
}
#[inline(always)]
pub fn block_0x0021de3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 23usize, 13usize, 2219584u32);
    emu.lb_no_count(10usize, 10usize, 0u32, 2219588u32);
    emu.adi_no_count(11usize, 0usize, 4294967231u32, 2219592u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2219752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dee8));
    } else {
        emu.pc = 2219596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021de4c));
    }
}
#[inline(always)]
pub fn block_0x0021de4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 13usize, 0u32, 2219600u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2219604u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2219616u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021de60));
}
#[inline(always)]
pub fn block_0x0021de54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 0u32, 2219608u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2219612u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2219616u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021de60));
}
#[inline(always)]
pub fn block_0x0021de5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2219752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dee8));
    } else {
        emu.pc = 2219616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021de60));
    }
}
#[inline(always)]
pub fn block_0x0021de60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 21usize, 12u32, 2219620u32)?;
    emu.sbr_no_count(12usize, 19usize, 20usize, 2219624u32);
    emu.adr_no_count(11usize, 23usize, 20usize, 2219628u32);
    emu.lw_no_count(8usize, 2usize, 24u32, 2219632u32)?;
    emu.adi_no_count(10usize, 8usize, 0u32, 2219636u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2219640u32;
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
pub fn block_0x0021de78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2219660u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021de8c));
    } else {
        emu.pc = 2219644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021de7c));
    }
}
#[inline(always)]
pub fn block_0x0021de7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 34u32, 2219648u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2219652u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(9usize);
    let return_addr = 2219656u32;
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
pub fn block_0x0021de88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2219660u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2219660u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021de8c));
}
#[inline]
pub fn block_0x0021de8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2219664u32);
    emu.lw_no_count(1usize, 2usize, 92u32, 2219668u32)?;
    emu.lw_no_count(8usize, 2usize, 88u32, 2219672u32)?;
    emu.lw_no_count(9usize, 2usize, 84u32, 2219676u32)?;
    emu.lw_no_count(18usize, 2usize, 80u32, 2219680u32)?;
    emu.lw_no_count(19usize, 2usize, 76u32, 2219684u32)?;
    emu.lw_no_count(20usize, 2usize, 72u32, 2219688u32)?;
    emu.lw_no_count(21usize, 2usize, 68u32, 2219692u32)?;
    emu.lw_no_count(22usize, 2usize, 64u32, 2219696u32)?;
    emu.lw_no_count(23usize, 2usize, 60u32, 2219700u32)?;
    emu.lw_no_count(24usize, 2usize, 56u32, 2219704u32)?;
    emu.lw_no_count(25usize, 2usize, 52u32, 2219708u32)?;
    emu.lw_no_count(26usize, 2usize, 48u32, 2219712u32)?;
    emu.lw_no_count(27usize, 2usize, 44u32, 2219716u32)?;
    emu.adi_no_count(2usize, 2usize, 96u32, 2219720u32);
    emu.add_memory_rw_events(16usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2219724u32;
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
pub fn block_0x0021decc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 1u32, 2219728u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2219732u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2219660u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021de8c));
}
#[inline(always)]
pub fn block_0x0021ded4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(13usize, 10usize, 25usize, 2219736u32);
    emu.lw_no_count(21usize, 2usize, 16u32, 2219740u32)?;
    emu.lw_no_count(9usize, 2usize, 12u32, 2219744u32)?;
    emu.adi_no_count(18usize, 0usize, 1u32, 2219748u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2219532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021de0c));
    } else {
        emu.pc = 2219752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dee8));
    }
}
#[inline(always)]
pub fn block_0x0021dee8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2219756u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966352u32, 2219760u32);
    emu.adi_no_count(10usize, 23usize, 0u32, 2219764u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2219768u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2219772u32);
    emu.apc_no_count(1usize, 2219772u32, 4294959104u32, 2219776u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2219780u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965344u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021df04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2219784u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966336u32, 2219788u32);
    emu.adi_no_count(10usize, 23usize, 0u32, 2219792u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2219796u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2219800u32);
    emu.apc_no_count(1usize, 2219800u32, 4294959104u32, 2219804u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2219808u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965316u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
