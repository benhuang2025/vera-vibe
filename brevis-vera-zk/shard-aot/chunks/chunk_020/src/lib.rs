pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2168380u32;
pub const PC_MAX: u32 = 2171076u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 153usize] = [
        block_0x0021163c,
        block_0x00211654,
        block_0x00211688,
        block_0x0021168c,
        block_0x00211694,
        block_0x002116ac,
        block_0x002116b8,
        block_0x002116c4,
        block_0x002116c8,
        block_0x002116cc,
        block_0x002116e4,
        block_0x002116e8,
        block_0x002116f4,
        block_0x00211710,
        block_0x00211724,
        block_0x00211738,
        block_0x00211750,
        block_0x0021175c,
        block_0x0021176c,
        block_0x00211774,
        block_0x002117a0,
        block_0x002117ac,
        block_0x002117b4,
        block_0x002117f0,
        block_0x002117fc,
        block_0x00211848,
        block_0x00211854,
        block_0x0021187c,
        block_0x00211884,
        block_0x002118a4,
        block_0x002118c0,
        block_0x002118d8,
        block_0x002118dc,
        block_0x00211900,
        block_0x00211908,
        block_0x00211910,
        block_0x00211928,
        block_0x0021192c,
        block_0x00211950,
        block_0x00211958,
        block_0x00211960,
        block_0x00211980,
        block_0x00211988,
        block_0x002119a0,
        block_0x002119b0,
        block_0x002119c8,
        block_0x002119dc,
        block_0x002119f0,
        block_0x002119f4,
        block_0x002119f8,
        block_0x002119fc,
        block_0x00211a00,
        block_0x00211a10,
        block_0x00211a14,
        block_0x00211a18,
        block_0x00211a20,
        block_0x00211a28,
        block_0x00211a38,
        block_0x00211a44,
        block_0x00211a48,
        block_0x00211a4c,
        block_0x00211a68,
        block_0x00211a70,
        block_0x00211a78,
        block_0x00211a80,
        block_0x00211a98,
        block_0x00211ab0,
        block_0x00211ac0,
        block_0x00211ac8,
        block_0x00211ad0,
        block_0x00211ad8,
        block_0x00211ae0,
        block_0x00211ae8,
        block_0x00211b1c,
        block_0x00211b24,
        block_0x00211b58,
        block_0x00211c38,
        block_0x00211c5c,
        block_0x00211c68,
        block_0x00211c6c,
        block_0x00211c90,
        block_0x00211cb8,
        block_0x00211cbc,
        block_0x00211cc4,
        block_0x00211ccc,
        block_0x00211cd4,
        block_0x00211cdc,
        block_0x00211ce8,
        block_0x00211cec,
        block_0x00211d14,
        block_0x00211d3c,
        block_0x00211d40,
        block_0x00211d48,
        block_0x00211d50,
        block_0x00211d54,
        block_0x00211d5c,
        block_0x00211d68,
        block_0x00211d6c,
        block_0x00211d84,
        block_0x00211dac,
        block_0x00211db0,
        block_0x00211db8,
        block_0x00211dc0,
        block_0x00211dc4,
        block_0x00211dcc,
        block_0x00211de0,
        block_0x00211dec,
        block_0x00211e08,
        block_0x00211e10,
        block_0x00211e28,
        block_0x00211e30,
        block_0x00211e48,
        block_0x00211e4c,
        block_0x00211e54,
        block_0x00211e68,
        block_0x00211e74,
        block_0x00211e90,
        block_0x00211e98,
        block_0x00211eb0,
        block_0x00211eb8,
        block_0x00211ed0,
        block_0x00211ed4,
        block_0x00211edc,
        block_0x00211ef0,
        block_0x00211efc,
        block_0x00211f18,
        block_0x00211f20,
        block_0x00211f38,
        block_0x00211f40,
        block_0x00211f58,
        block_0x00211f5c,
        block_0x00211f64,
        block_0x00211f78,
        block_0x00211f84,
        block_0x00211fa0,
        block_0x00211fa8,
        block_0x00211fc0,
        block_0x00211fc8,
        block_0x00211fe0,
        block_0x00211fe4,
        block_0x00211fec,
        block_0x00212000,
        block_0x0021200c,
        block_0x00212028,
        block_0x00212030,
        block_0x00212048,
        block_0x00212050,
        block_0x00212068,
        block_0x0021206c,
        block_0x0021207c,
        block_0x00212098,
        block_0x002120ac,
        block_0x002120c4,
    ];
    const IDX: [u16; 675usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 3u16, 4u16, 0u16, 5u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 6u16, 0u16, 0u16, 7u16, 0u16, 0u16, 8u16, 9u16, 10u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 11u16, 12u16, 0u16, 0u16, 13u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 14u16, 0u16, 0u16, 0u16, 0u16, 15u16, 0u16, 0u16, 0u16, 0u16, 16u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 17u16, 0u16, 0u16, 18u16, 0u16, 0u16, 0u16, 19u16, 0u16,
        20u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 21u16, 0u16,
        0u16, 22u16, 0u16, 23u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 24u16, 0u16, 0u16, 25u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 26u16, 0u16, 0u16, 27u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 28u16, 0u16, 29u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 30u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 31u16, 0u16, 0u16, 0u16, 0u16, 0u16, 32u16, 33u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 34u16, 0u16, 35u16, 0u16, 36u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 37u16, 38u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 39u16, 0u16, 40u16, 0u16, 41u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 42u16, 0u16, 43u16, 0u16, 0u16, 0u16, 0u16, 0u16, 44u16, 0u16, 0u16, 0u16,
        45u16, 0u16, 0u16, 0u16, 0u16, 0u16, 46u16, 0u16, 0u16, 0u16, 0u16, 47u16, 0u16,
        0u16, 0u16, 0u16, 48u16, 49u16, 50u16, 51u16, 52u16, 0u16, 0u16, 0u16, 53u16,
        54u16, 55u16, 0u16, 56u16, 0u16, 57u16, 0u16, 0u16, 0u16, 58u16, 0u16, 0u16,
        59u16, 60u16, 61u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 62u16, 0u16, 63u16,
        0u16, 64u16, 0u16, 65u16, 0u16, 0u16, 0u16, 0u16, 0u16, 66u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 67u16, 0u16, 0u16, 0u16, 68u16, 0u16, 69u16, 0u16, 70u16, 0u16,
        71u16, 0u16, 72u16, 0u16, 73u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 74u16, 0u16, 75u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 76u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 77u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 78u16, 0u16, 0u16, 79u16, 80u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 81u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 82u16, 83u16, 0u16, 84u16, 0u16, 85u16, 0u16, 86u16, 0u16,
        87u16, 0u16, 0u16, 88u16, 89u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 90u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 91u16, 92u16,
        0u16, 93u16, 0u16, 94u16, 95u16, 0u16, 96u16, 0u16, 0u16, 97u16, 98u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 99u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 100u16, 101u16, 0u16, 102u16, 0u16, 103u16, 104u16, 0u16, 105u16, 0u16,
        0u16, 0u16, 0u16, 106u16, 0u16, 0u16, 107u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        108u16, 0u16, 109u16, 0u16, 0u16, 0u16, 0u16, 0u16, 110u16, 0u16, 111u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 112u16, 113u16, 0u16, 114u16, 0u16, 0u16, 0u16, 0u16,
        115u16, 0u16, 0u16, 116u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 117u16, 0u16,
        118u16, 0u16, 0u16, 0u16, 0u16, 0u16, 119u16, 0u16, 120u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 121u16, 122u16, 0u16, 123u16, 0u16, 0u16, 0u16, 0u16, 124u16, 0u16,
        0u16, 125u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 126u16, 0u16, 127u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 128u16, 0u16, 129u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        130u16, 131u16, 0u16, 132u16, 0u16, 0u16, 0u16, 0u16, 133u16, 0u16, 0u16, 134u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 135u16, 0u16, 136u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 137u16, 0u16, 138u16, 0u16, 0u16, 0u16, 0u16, 0u16, 139u16, 140u16, 0u16,
        141u16, 0u16, 0u16, 0u16, 0u16, 142u16, 0u16, 0u16, 143u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 144u16, 0u16, 145u16, 0u16, 0u16, 0u16, 0u16, 0u16, 146u16,
        0u16, 147u16, 0u16, 0u16, 0u16, 0u16, 0u16, 148u16, 149u16, 0u16, 0u16, 0u16,
        150u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 151u16, 0u16, 0u16, 0u16, 0u16,
        152u16, 0u16, 0u16, 0u16, 0u16, 0u16, 153u16,
    ];
    if pc < 2168380u32 || pc > 2171076u32 {
        return None;
    }
    let word_offset = ((pc - 2168380u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x0021163c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 11usize, 0u32, 2168384u32);
    emu.adi_no_count(11usize, 10usize, 0u32, 2168388u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2168392u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2168396u32);
    emu.apc_no_count(6usize, 2168396u32, 4294963200u32, 2168400u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2168404u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294967284u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00211654(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2168408u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2168412u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2168416u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2168420u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2168424u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2168428u32)?;
    emu.lw_no_count(19usize, 11usize, 4u32, 2168432u32)?;
    emu.lw_no_count(8usize, 11usize, 0u32, 2168436u32)?;
    emu.lw_no_count(18usize, 19usize, 16u32, 2168440u32)?;
    emu.adi_no_count(9usize, 10usize, 0u32, 2168444u32);
    emu.adi_no_count(11usize, 0usize, 39u32, 2168448u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2168452u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(18usize);
    let return_addr = 2168456u32;
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
pub fn block_0x00211688(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2168468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211694));
    } else {
        emu.pc = 2168460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021168c));
    }
}
#[inline(always)]
pub fn block_0x0021168c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2168464u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2168468u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2168564u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002116f4));
}
#[inline(always)]
pub fn block_0x00211694(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 9usize, 0u32, 2168472u32)?;
    emu.adi_no_count(10usize, 2usize, 12u32, 2168476u32);
    emu.adi_no_count(12usize, 0usize, 257u32, 2168480u32);
    emu.adi_no_count(9usize, 2usize, 12u32, 2168484u32);
    emu.apc_no_count(1usize, 2168484u32, 4294959104u32, 2168488u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2168492u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1352u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002116ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 25u32, 2168496u32);
    emu.adi_no_count(11usize, 0usize, 129u32, 2168500u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2168524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002116cc));
    } else {
        emu.pc = 2168504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002116b8));
    }
}
#[inline(always)]
pub fn block_0x002116b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 12u32, 2168508u32)?;
    emu.adi_no_count(10usize, 8usize, 0u32, 2168512u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(18usize);
    let return_addr = 2168516u32;
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
pub fn block_0x002116c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2168460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021168c));
    } else {
        emu.pc = 2168520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002116c8));
    }
}
#[inline(always)]
pub fn block_0x002116c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2168524u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2168552u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002116e8));
}
#[inline(always)]
pub fn block_0x002116cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 2usize, 24u32, 2168528u32);
    emu.lw_no_count(13usize, 19usize, 12u32, 2168532u32)?;
    emu.sbr_no_count(12usize, 10usize, 11usize, 2168536u32);
    emu.adr_no_count(11usize, 9usize, 11usize, 2168540u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2168544u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2168548u32;
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
pub fn block_0x002116e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2168460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021168c));
    } else {
        emu.pc = 2168552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002116e8));
    }
}
#[inline(always)]
pub fn block_0x002116e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 39u32, 2168556u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2168560u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(18usize);
    let return_addr = 2168564u32;
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
pub fn block_0x002116f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2168568u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2168572u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2168576u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2168580u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2168584u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2168588u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2168592u32;
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
pub fn block_0x00211710(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 11usize, 0u32, 2168596u32);
    emu.lbu_no_count(12usize, 11usize, 11u32, 2168600u32);
    emu.lw_no_count(11usize, 10usize, 0u32, 2168604u32)?;
    emu.ani_no_count(12usize, 12usize, 24u32, 2168608u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2168668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021175c));
    } else {
        emu.pc = 2168612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211724));
    }
}
#[inline(always)]
pub fn block_0x00211724(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2168616u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2168620u32)?;
    emu.adi_no_count(10usize, 0usize, 128u32, 2168624u32);
    emu.sw_no_count(0usize, 2usize, 8u32, 2168628u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2168684u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021176c));
    } else {
        emu.pc = 2168632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211738));
    }
}
#[inline(always)]
pub fn block_0x00211738(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 2usize, 8u32, 2168636u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2168640u32);
    emu.adi_no_count(11usize, 2usize, 8u32, 2168644u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2168648u32);
    emu.apc_no_count(1usize, 2168648u32, 4294963200u32, 2168652u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2168656u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967032u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00211750(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2168660u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2168664u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2168668u32;
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
pub fn block_0x0021175c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 13usize, 4u32, 2168672u32)?;
    emu.lw_no_count(10usize, 13usize, 0u32, 2168676u32)?;
    emu.lw_no_count(6usize, 12usize, 16u32, 2168680u32)?;
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2168684u32;
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
pub fn block_0x0021176c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 11u32, 2168688u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2168748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002117ac));
    } else {
        emu.pc = 2168692u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211774));
    }
}
#[inline]
pub fn block_0x00211774(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 6u32, 2168696u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2168700u32);
    emu.ori_no_count(10usize, 10usize, 192u32, 2168704u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2168708u32);
    emu.sb_no_count(10usize, 2usize, 8u32, 2168712u32);
    emu.sb_no_count(11usize, 2usize, 9u32, 2168716u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2168720u32);
    emu.adi_no_count(11usize, 2usize, 8u32, 2168724u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2168728u32);
    emu.apc_no_count(1usize, 2168728u32, 4294963200u32, 2168732u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2168736u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966952u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002117a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2168740u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2168744u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2168748u32;
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
pub fn block_0x002117ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 16u32, 2168752u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2168828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002117fc));
    } else {
        emu.pc = 2168756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002117b4));
    }
}
#[inline]
pub fn block_0x002117b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 12u32, 2168760u32);
    emu.sli_no_count(12usize, 11usize, 20u32, 2168764u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2168768u32);
    emu.ori_no_count(10usize, 10usize, 224u32, 2168772u32);
    emu.sri_no_count(12usize, 12usize, 26u32, 2168776u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2168780u32);
    emu.adi_no_count(12usize, 12usize, 128u32, 2168784u32);
    emu.sb_no_count(10usize, 2usize, 8u32, 2168788u32);
    emu.sb_no_count(12usize, 2usize, 9u32, 2168792u32);
    emu.sb_no_count(11usize, 2usize, 10u32, 2168796u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2168800u32);
    emu.adi_no_count(11usize, 2usize, 8u32, 2168804u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2168808u32);
    emu.apc_no_count(1usize, 2168808u32, 4294963200u32, 2168812u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2168816u32;
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
pub fn block_0x002117f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2168820u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2168824u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2168828u32;
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
pub fn block_0x002117fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 18u32, 2168832u32);
    emu.sli_no_count(12usize, 11usize, 14u32, 2168836u32);
    emu.sli_no_count(14usize, 11usize, 20u32, 2168840u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2168844u32);
    emu.adi_no_count(10usize, 10usize, 240u32, 2168848u32);
    emu.sri_no_count(12usize, 12usize, 26u32, 2168852u32);
    emu.sri_no_count(14usize, 14usize, 26u32, 2168856u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2168860u32);
    emu.adi_no_count(12usize, 12usize, 128u32, 2168864u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2168868u32);
    emu.sb_no_count(10usize, 2usize, 8u32, 2168872u32);
    emu.sb_no_count(12usize, 2usize, 9u32, 2168876u32);
    emu.sb_no_count(14usize, 2usize, 10u32, 2168880u32);
    emu.sb_no_count(11usize, 2usize, 11u32, 2168884u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2168888u32);
    emu.adi_no_count(11usize, 2usize, 8u32, 2168892u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2168896u32);
    emu.apc_no_count(1usize, 2168896u32, 4294963200u32, 2168900u32);
    emu.add_memory_rw_events(19usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2168904u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966784u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00211848(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2168908u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2168912u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2168916u32;
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
pub fn block_0x00211854(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967152u32, 2168920u32);
    emu.sw_no_count(1usize, 2usize, 140u32, 2168924u32)?;
    emu.sw_no_count(8usize, 2usize, 136u32, 2168928u32)?;
    emu.sw_no_count(9usize, 2usize, 132u32, 2168932u32)?;
    emu.sw_no_count(18usize, 2usize, 128u32, 2168936u32)?;
    emu.adi_no_count(8usize, 11usize, 0u32, 2168940u32);
    emu.lw_no_count(12usize, 11usize, 8u32, 2168944u32)?;
    emu.lw_no_count(11usize, 10usize, 0u32, 2168948u32)?;
    emu.sli_no_count(10usize, 12usize, 6u32, 2168952u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2169024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002118c0));
    } else {
        emu.pc = 2168956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021187c));
    }
}
#[inline(always)]
pub fn block_0x0021187c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 5u32, 2168960u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2169104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211910));
    } else {
        emu.pc = 2168964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211884));
    }
}
#[inline(always)]
pub fn block_0x00211884(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 11usize, 0u32, 2168968u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2168972u32)?;
    emu.adi_no_count(9usize, 2usize, 0u32, 2168976u32);
    emu.adi_no_count(12usize, 2usize, 0u32, 2168980u32);
    emu.adi_no_count(13usize, 0usize, 20u32, 2168984u32);
    emu.adi_no_count(18usize, 0usize, 20u32, 2168988u32);
    emu.apc_no_count(1usize, 2168988u32, 4294946816u32, 2168992u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2168996u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965660u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002118a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(15usize, 18usize, 10usize, 2169000u32);
    emu.adr_no_count(14usize, 9usize, 10usize, 2169004u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2169008u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2169012u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2169016u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2169020u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2169024u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2169216u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211980));
}
#[inline(always)]
pub fn block_0x002118c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 0u32, 2169028u32);
    emu.lw_no_count(13usize, 11usize, 0u32, 2169032u32)?;
    emu.lw_no_count(10usize, 11usize, 4u32, 2169036u32)?;
    emu.adi_no_count(11usize, 2usize, 127u32, 2169040u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2169044u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2169048u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2169088u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211900));
}
#[inline(always)]
pub fn block_0x002118d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 14usize, 87u32, 2169052u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2169052u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002118dc));
}
#[inline]
pub fn block_0x002118dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 13usize, 4u32, 2169056u32);
    emu.sli_no_count(16usize, 10usize, 28u32, 2169060u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2169064u32);
    emu.sb_no_count(14usize, 11usize, 0u32, 2169068u32);
    emu.orr_no_count(13usize, 13usize, 16usize, 2169072u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2169076u32);
    emu.orr_no_count(14usize, 13usize, 10usize, 2169080u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2169084u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2169184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211960));
    } else {
        emu.pc = 2169088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211900));
    }
}
#[inline(always)]
pub fn block_0x00211900(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(14usize, 13usize, 15u32, 2169092u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a >= b {
        emu.pc = 2169048u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002118d8));
    } else {
        emu.pc = 2169096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211908));
    }
}
#[inline(always)]
pub fn block_0x00211908(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 14usize, 48u32, 2169100u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2169104u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2169052u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002118dc));
}
#[inline(always)]
pub fn block_0x00211910(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 0u32, 2169108u32);
    emu.lw_no_count(13usize, 11usize, 0u32, 2169112u32)?;
    emu.lw_no_count(10usize, 11usize, 4u32, 2169116u32)?;
    emu.adi_no_count(11usize, 2usize, 127u32, 2169120u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2169124u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2169128u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2169168u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211950));
}
#[inline(always)]
pub fn block_0x00211928(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 14usize, 55u32, 2169132u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2169132u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021192c));
}
#[inline]
pub fn block_0x0021192c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 13usize, 4u32, 2169136u32);
    emu.sli_no_count(16usize, 10usize, 28u32, 2169140u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2169144u32);
    emu.sb_no_count(14usize, 11usize, 0u32, 2169148u32);
    emu.orr_no_count(13usize, 13usize, 16usize, 2169152u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2169156u32);
    emu.orr_no_count(14usize, 13usize, 10usize, 2169160u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2169164u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2169184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211960));
    } else {
        emu.pc = 2169168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211950));
    }
}
#[inline(always)]
pub fn block_0x00211950(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(14usize, 13usize, 15u32, 2169172u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a >= b {
        emu.pc = 2169128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211928));
    } else {
        emu.pc = 2169176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211958));
    }
}
#[inline(always)]
pub fn block_0x00211958(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 14usize, 48u32, 2169180u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2169184u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2169132u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021192c));
}
#[inline(always)]
pub fn block_0x00211960(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2169188u32);
    emu.sbr_no_count(10usize, 10usize, 15usize, 2169192u32);
    emu.adi_no_count(14usize, 10usize, 128u32, 2169196u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2169200u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 336u32, 2169204u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2169208u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2169212u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2169216u32);
    emu.add_memory_rw_events(8usize);
    emu.pc = 2169216u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211980));
}
#[inline(always)]
pub fn block_0x00211980(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2169216u32, 4294963200u32, 2169220u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2169224u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965452u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00211988(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 140u32, 2169228u32)?;
    emu.lw_no_count(8usize, 2usize, 136u32, 2169232u32)?;
    emu.lw_no_count(9usize, 2usize, 132u32, 2169236u32)?;
    emu.lw_no_count(18usize, 2usize, 128u32, 2169240u32)?;
    emu.adi_no_count(2usize, 2usize, 144u32, 2169244u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2169248u32;
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
pub fn block_0x002119a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 4u32, 2169252u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2169256u32)?;
    emu.lw_no_count(6usize, 12usize, 12u32, 2169260u32)?;
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2169264u32;
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
pub fn block_0x002119b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 0u32, 2169268u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2169272u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2169276u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2169280u32);
    emu.apc_no_count(6usize, 2169280u32, 4294963200u32, 2169284u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2169288u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966400u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002119c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 0usize, 0u32, 2169292u32);
    emu.sri_no_count(6usize, 10usize, 8u32, 2169296u32);
    emu.sli_no_count(12usize, 12usize, 1u32, 2169300u32);
    emu.adr_no_count(12usize, 11usize, 12usize, 2169304u32);
    emu.ani_no_count(7usize, 10usize, 255u32, 2169308u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2169308u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002119dc));
}
#[inline(always)]
pub fn block_0x002119dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(29usize, 11usize, 0u32, 2169312u32);
    emu.lbu_no_count(28usize, 11usize, 1u32, 2169316u32);
    emu.adi_no_count(11usize, 11usize, 2u32, 2169320u32);
    emu.adr_no_count(5usize, 17usize, 28usize, 2169324u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(29usize);
    if a != b {
        emu.pc = 2169364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211a14));
    } else {
        emu.pc = 2169328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002119f0));
    }
}
#[inline(always)]
pub fn block_0x002119f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a < b {
        emu.pc = 2169472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211a80));
    } else {
        emu.pc = 2169332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002119f4));
    }
}
#[inline(always)]
pub fn block_0x002119f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2169496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211a98));
    } else {
        emu.pc = 2169336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002119f8));
    }
}
#[inline(always)]
pub fn block_0x002119f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(17usize, 13usize, 17usize, 2169340u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2169340u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002119fc));
}
#[inline(always)]
pub fn block_0x002119fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2169368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211a18));
    } else {
        emu.pc = 2169344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211a00));
    }
}
#[inline(always)]
pub fn block_0x00211a00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(29usize, 17usize, 0u32, 2169348u32);
    emu.adi_no_count(17usize, 17usize, 1u32, 2169352u32);
    emu.adi_no_count(28usize, 28usize, 4294967295u32, 2169356u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(29usize);
    if a != b {
        emu.pc = 2169340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002119fc));
    } else {
        emu.pc = 2169360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211a10));
    }
}
#[inline(always)]
pub fn block_0x00211a10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2169364u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2169464u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211a78));
}
#[inline(always)]
pub fn block_0x00211a14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a < b {
        emu.pc = 2169376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211a20));
    } else {
        emu.pc = 2169368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211a18));
    }
}
#[inline(always)]
pub fn block_0x00211a18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 5usize, 0u32, 2169372u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2169308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002119dc));
    } else {
        emu.pc = 2169376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211a20));
    }
}
#[inline(always)]
pub fn block_0x00211a20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(16usize, 15usize, 16usize, 2169380u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2169384u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2169384u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211a28));
}
#[inline(always)]
pub fn block_0x00211a28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(14usize, 15usize, 0u32, 2169388u32);
    emu.adi_no_count(13usize, 15usize, 1u32, 2169392u32);
    emu.ani_no_count(12usize, 14usize, 255u32, 2169396u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2169416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211a48));
    } else {
        emu.pc = 2169400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211a38));
    }
}
#[inline(always)]
pub fn block_0x00211a38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 13usize, 0u32, 2169404u32);
    emu.sbr_no_count(10usize, 10usize, 12usize, 2169408u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2169448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211a68));
    } else {
        emu.pc = 2169412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211a44));
    }
}
#[inline(always)]
pub fn block_0x00211a44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2169416u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2169456u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211a70));
}
#[inline(always)]
pub fn block_0x00211a48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2169520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211ab0));
    } else {
        emu.pc = 2169420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211a4c));
    }
}
#[inline(always)]
pub fn block_0x00211a4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(13usize, 15usize, 1u32, 2169424u32);
    emu.adi_no_count(15usize, 15usize, 2u32, 2169428u32);
    emu.ani_no_count(12usize, 12usize, 127u32, 2169432u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2169436u32);
    emu.orr_no_count(12usize, 12usize, 13usize, 2169440u32);
    emu.sbr_no_count(10usize, 10usize, 12usize, 2169444u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2169456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211a70));
    } else {
        emu.pc = 2169448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211a68));
    }
}
#[inline(always)]
pub fn block_0x00211a68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(11usize, 11usize, 1u32, 2169452u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2169384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211a28));
    } else {
        emu.pc = 2169456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211a70));
    }
}
#[inline(always)]
pub fn block_0x00211a70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 11usize, 1u32, 2169460u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2169464u32;
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
pub fn block_0x00211a78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 0usize, 1u32, 2169468u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2169472u32;
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
pub fn block_0x00211a80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2169476u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 600u32, 2169480u32);
    emu.adi_no_count(10usize, 17usize, 0u32, 2169484u32);
    emu.adi_no_count(11usize, 5usize, 0u32, 2169488u32);
    emu.apc_no_count(1usize, 2169488u32, 8192u32, 2169492u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2169496u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(488u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00211a98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2169500u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 600u32, 2169504u32);
    emu.adi_no_count(10usize, 5usize, 0u32, 2169508u32);
    emu.adi_no_count(11usize, 14usize, 0u32, 2169512u32);
    emu.apc_no_count(1usize, 2169512u32, 8192u32, 2169516u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2169520u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(456u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00211ab0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2169524u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 616u32, 2169528u32);
    emu.apc_no_count(1usize, 2169528u32, 4294950912u32, 2169532u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2169536u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1928u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00211ac0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 32u32, 2169540u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2169552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211ad0));
    } else {
        emu.pc = 2169544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211ac8));
    }
}
#[inline(always)]
pub fn block_0x00211ac8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2169548u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2169552u32;
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
pub fn block_0x00211ad0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 127u32, 2169556u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2169568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211ae0));
    } else {
        emu.pc = 2169560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211ad8));
    }
}
#[inline(always)]
pub fn block_0x00211ad8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2169564u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2169568u32;
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
pub fn block_0x00211ae0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 10usize, 16u32, 2169572u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2169628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211b1c));
    } else {
        emu.pc = 2169576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211ae8));
    }
}
#[inline]
pub fn block_0x00211ae8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 10usize, 16u32, 2169580u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2169584u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1414u32, 2169588u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2169592u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1494u32, 2169596u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2169600u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 1784u32, 2169604u32);
    emu.adi_no_count(12usize, 0usize, 40u32, 2169608u32);
    emu.adi_no_count(14usize, 0usize, 290u32, 2169612u32);
    emu.sri_no_count(10usize, 10usize, 16u32, 2169616u32);
    emu.adi_no_count(16usize, 0usize, 297u32, 2169620u32);
    emu.apc_no_count(6usize, 2169620u32, 0u32, 2169624u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2169628u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966964u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00211b1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 10usize, 17u32, 2169632u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2169688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211b58));
    } else {
        emu.pc = 2169636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211b24));
    }
}
#[inline]
pub fn block_0x00211b24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 10usize, 16u32, 2169640u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2169644u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 632u32, 2169648u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2169652u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 720u32, 2169656u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2169660u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 928u32, 2169664u32);
    emu.adi_no_count(12usize, 0usize, 44u32, 2169668u32);
    emu.adi_no_count(14usize, 0usize, 208u32, 2169672u32);
    emu.sri_no_count(10usize, 10usize, 16u32, 2169676u32);
    emu.adi_no_count(16usize, 0usize, 486u32, 2169680u32);
    emu.apc_no_count(6usize, 2169680u32, 0u32, 2169684u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2169688u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966904u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00211b58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 56u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2097152u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2169692u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(172032u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2169696u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4294791168u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2169700u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4294782976u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2169704u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4294774784u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2169708u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4294770688u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2169712u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4294766592u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2169716u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4294049792u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2169720u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(917504u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2169724u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967264u32, 2169728u32);
    emu.adi_no_count(12usize, 12usize, 1760u32, 2169732u32);
    emu.adi_no_count(13usize, 13usize, 4294965440u32, 2169736u32);
    emu.adi_no_count(14usize, 14usize, 336u32, 2169740u32);
    emu.anr_no_count(7usize, 10usize, 11usize, 2169744u32);
    emu.xrr_no_count(12usize, 7usize, 12usize, 2169748u32);
    emu.adi_no_count(7usize, 15usize, 1040u32, 2169752u32);
    emu.adi_no_count(15usize, 15usize, 4294965248u32, 2169756u32);
    emu.adr_no_count(16usize, 10usize, 16usize, 2169760u32);
    emu.adi_no_count(17usize, 17usize, 4294966448u32, 2169764u32);
    emu.adi_no_count(5usize, 5usize, 4294967040u32, 2169768u32);
    emu.adi_no_count(6usize, 6usize, 496u32, 2169772u32);
    emu.adr_no_count(13usize, 10usize, 13usize, 2169776u32);
    emu.adi_no_count(11usize, 11usize, 30u32, 2169780u32);
    emu.adr_no_count(14usize, 10usize, 14usize, 2169784u32);
    emu.adr_no_count(7usize, 10usize, 7usize, 2169788u32);
    emu.adr_no_count(15usize, 10usize, 15usize, 2169792u32);
    emu.adr_no_count(17usize, 10usize, 17usize, 2169796u32);
    emu.adr_no_count(5usize, 10usize, 5usize, 2169800u32);
    emu.sltru_no_count(6usize, 10usize, 6usize, 2169804u32);
    emu.anr_no_count(10usize, 10usize, 11usize, 2169808u32);
    let a = 0u32.wrapping_add(4294963200u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2169812u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1630u32, 2169816u32);
    emu.sltru_no_count(11usize, 15usize, 11usize, 2169820u32);
    let a = 0u32.wrapping_add(4294254592u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2169824u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 688u32, 2169828u32);
    emu.sltru_no_count(15usize, 5usize, 15usize, 2169832u32);
    let a = 0u32.wrapping_add(180224u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2169836u32;
    emu.update_insn_clock();
    emu.adi_no_count(5usize, 5usize, 4294965278u32, 2169840u32);
    emu.xrr_no_count(10usize, 10usize, 5usize, 2169844u32);
    emu.sltiu_no_count(14usize, 14usize, 4294967282u32, 2169848u32);
    emu.sltiu_no_count(5usize, 7usize, 4294967281u32, 2169852u32);
    emu.anr_no_count(14usize, 14usize, 5usize, 2169856u32);
    emu.sltiu_no_count(17usize, 17usize, 4294967291u32, 2169860u32);
    emu.anr_no_count(15usize, 17usize, 15usize, 2169864u32);
    emu.sltiu_no_count(13usize, 13usize, 4294967290u32, 2169868u32);
    emu.sltru_no_count(12usize, 0usize, 12usize, 2169872u32);
    emu.anr_no_count(12usize, 12usize, 13usize, 2169876u32);
    emu.anr_no_count(11usize, 14usize, 11usize, 2169880u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2169884u32);
    emu.anr_no_count(10usize, 12usize, 10usize, 2169888u32);
    emu.sltiu_no_count(12usize, 16usize, 4294965790u32, 2169892u32);
    emu.anr_no_count(11usize, 11usize, 12usize, 2169896u32);
    emu.anr_no_count(10usize, 10usize, 11usize, 2169900u32);
    emu.anr_no_count(11usize, 15usize, 6usize, 2169904u32);
    emu.anr_no_count(10usize, 10usize, 11usize, 2169908u32);
    emu.add_memory_rw_events(56usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2169912u32;
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
pub fn block_0x00211c38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967120u32, 2169916u32);
    emu.sw_no_count(1usize, 2usize, 172u32, 2169920u32)?;
    emu.sw_no_count(8usize, 2usize, 168u32, 2169924u32)?;
    emu.sw_no_count(9usize, 2usize, 164u32, 2169928u32)?;
    emu.sw_no_count(18usize, 2usize, 160u32, 2169932u32)?;
    emu.adi_no_count(9usize, 11usize, 0u32, 2169936u32);
    emu.adi_no_count(11usize, 0usize, 8u32, 2169940u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2169944u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a >= b {
        emu.pc = 2170068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211cd4));
    } else {
        emu.pc = 2169948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211c5c));
    }
}
#[inline(always)]
pub fn block_0x00211c5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 160u32, 2169952u32)?;
    emu.adi_no_count(11usize, 0usize, 41u32, 2169956u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2171032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212098));
    } else {
        emu.pc = 2169960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211c68));
    }
}
#[inline(always)]
pub fn block_0x00211c68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2170060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211ccc));
    } else {
        emu.pc = 2169964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211c6c));
    }
}
#[inline]
pub fn block_0x00211c6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2169968u32);
    emu.sli_no_count(9usize, 9usize, 2u32, 2169972u32);
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2169976u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965284u32, 2169980u32);
    emu.adr_no_count(11usize, 11usize, 9usize, 2169984u32);
    emu.lw_no_count(13usize, 11usize, 0u32, 2169988u32)?;
    emu.sli_no_count(14usize, 10usize, 2u32, 2169992u32);
    emu.adr_no_count(11usize, 8usize, 14usize, 2169996u32);
    emu.adi_no_count(15usize, 8usize, 0u32, 2170000u32);
    emu.add_memory_rw_events(9usize);
    emu.pc = 2170000u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211c90));
}
#[inline]
pub fn block_0x00211c90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 15usize, 0u32, 2170004u32)?;
    emu.mulhu_no_count(17usize, 16usize, 13usize, 2170008u32);
    emu.mul_no_count(16usize, 16usize, 13usize, 2170012u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2170016u32);
    emu.sw_no_count(12usize, 15usize, 0u32, 2170020u32)?;
    emu.adi_no_count(15usize, 15usize, 4u32, 2170024u32);
    emu.sltru_no_count(12usize, 12usize, 16usize, 2170028u32);
    emu.adi_no_count(14usize, 14usize, 4294967292u32, 2170032u32);
    emu.adr_no_count(12usize, 17usize, 12usize, 2170036u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a != b {
        emu.pc = 2170000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211c90));
    } else {
        emu.pc = 2170040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211cb8));
    }
}
#[inline(always)]
pub fn block_0x00211cb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2170060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211ccc));
    } else {
        emu.pc = 2170044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211cbc));
    }
}
#[inline(always)]
pub fn block_0x00211cbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 40u32, 2170048u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2171076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002120c4));
    } else {
        emu.pc = 2170052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211cc4));
    }
}
#[inline(always)]
pub fn block_0x00211cc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(12usize, 11usize, 0u32, 2170056u32)?;
    emu.adi_no_count(10usize, 10usize, 1u32, 2170060u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2170060u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211ccc));
}
#[inline(always)]
pub fn block_0x00211ccc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 8usize, 160u32, 2170064u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2170068u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2171004u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021207c));
}
#[inline(always)]
pub fn block_0x00211cd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(12usize, 9usize, 7u32, 2170072u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2170196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211d54));
    } else {
        emu.pc = 2170076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211cdc));
    }
}
#[inline(always)]
pub fn block_0x00211cdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 160u32, 2170080u32)?;
    emu.adi_no_count(11usize, 0usize, 41u32, 2170084u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2171032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212098));
    } else {
        emu.pc = 2170088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211ce8));
    }
}
#[inline(always)]
pub fn block_0x00211ce8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2170192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211d50));
    } else {
        emu.pc = 2170092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211cec));
    }
}
#[inline]
pub fn block_0x00211cec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2170096u32);
    emu.sli_no_count(13usize, 12usize, 2u32, 2170100u32);
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2170104u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965284u32, 2170108u32);
    emu.adr_no_count(13usize, 14usize, 13usize, 2170112u32);
    emu.lw_no_count(14usize, 13usize, 0u32, 2170116u32)?;
    emu.sli_no_count(13usize, 10usize, 2u32, 2170120u32);
    emu.srr_no_count(14usize, 14usize, 12usize, 2170124u32);
    emu.adr_no_count(12usize, 8usize, 13usize, 2170128u32);
    emu.adi_no_count(15usize, 8usize, 0u32, 2170132u32);
    emu.add_memory_rw_events(10usize);
    emu.pc = 2170132u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211d14));
}
#[inline]
pub fn block_0x00211d14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 15usize, 0u32, 2170136u32)?;
    emu.mulhu_no_count(17usize, 16usize, 14usize, 2170140u32);
    emu.mul_no_count(16usize, 16usize, 14usize, 2170144u32);
    emu.adr_no_count(11usize, 16usize, 11usize, 2170148u32);
    emu.sw_no_count(11usize, 15usize, 0u32, 2170152u32)?;
    emu.adi_no_count(15usize, 15usize, 4u32, 2170156u32);
    emu.sltru_no_count(11usize, 11usize, 16usize, 2170160u32);
    emu.adi_no_count(13usize, 13usize, 4294967292u32, 2170164u32);
    emu.adr_no_count(11usize, 17usize, 11usize, 2170168u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2170132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211d14));
    } else {
        emu.pc = 2170172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211d3c));
    }
}
#[inline(always)]
pub fn block_0x00211d3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2170192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211d50));
    } else {
        emu.pc = 2170176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211d40));
    }
}
#[inline(always)]
pub fn block_0x00211d40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 40u32, 2170180u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2171076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002120c4));
    } else {
        emu.pc = 2170184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211d48));
    }
}
#[inline(always)]
pub fn block_0x00211d48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 12usize, 0u32, 2170188u32)?;
    emu.adi_no_count(10usize, 10usize, 1u32, 2170192u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2170192u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211d50));
}
#[inline(always)]
pub fn block_0x00211d50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 8usize, 160u32, 2170196u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2170196u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211d54));
}
#[inline(always)]
pub fn block_0x00211d54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 9usize, 8u32, 2170200u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2170308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211dc4));
    } else {
        emu.pc = 2170204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211d5c));
    }
}
#[inline(always)]
pub fn block_0x00211d5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 160u32, 2170208u32)?;
    emu.adi_no_count(11usize, 0usize, 41u32, 2170212u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2171032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212098));
    } else {
        emu.pc = 2170216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211d68));
    }
}
#[inline(always)]
pub fn block_0x00211d68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2170304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211dc0));
    } else {
        emu.pc = 2170220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211d6c));
    }
}
#[inline(always)]
pub fn block_0x00211d6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2170224u32);
    emu.sli_no_count(13usize, 10usize, 2u32, 2170228u32);
    let a = 0u32.wrapping_add(389120u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2170232u32;
    emu.update_insn_clock();
    emu.adr_no_count(11usize, 8usize, 13usize, 2170236u32);
    emu.adi_no_count(14usize, 14usize, 1505u32, 2170240u32);
    emu.adi_no_count(15usize, 8usize, 0u32, 2170244u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2170244u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211d84));
}
#[inline]
pub fn block_0x00211d84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 15usize, 0u32, 2170248u32)?;
    emu.mulhu_no_count(17usize, 16usize, 14usize, 2170252u32);
    emu.mul_no_count(16usize, 16usize, 14usize, 2170256u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2170260u32);
    emu.sw_no_count(12usize, 15usize, 0u32, 2170264u32)?;
    emu.adi_no_count(15usize, 15usize, 4u32, 2170268u32);
    emu.sltru_no_count(12usize, 12usize, 16usize, 2170272u32);
    emu.adi_no_count(13usize, 13usize, 4294967292u32, 2170276u32);
    emu.adr_no_count(12usize, 17usize, 12usize, 2170280u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2170244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211d84));
    } else {
        emu.pc = 2170284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211dac));
    }
}
#[inline(always)]
pub fn block_0x00211dac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2170304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211dc0));
    } else {
        emu.pc = 2170288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211db0));
    }
}
#[inline(always)]
pub fn block_0x00211db0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 40u32, 2170292u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2171076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002120c4));
    } else {
        emu.pc = 2170296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211db8));
    }
}
#[inline(always)]
pub fn block_0x00211db8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(12usize, 11usize, 0u32, 2170300u32)?;
    emu.adi_no_count(10usize, 10usize, 1u32, 2170304u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2170304u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211dc0));
}
#[inline(always)]
pub fn block_0x00211dc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 8usize, 160u32, 2170308u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2170308u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211dc4));
}
#[inline(always)]
pub fn block_0x00211dc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 9usize, 16u32, 2170312u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2170444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211e4c));
    } else {
        emu.pc = 2170316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211dcc));
    }
}
#[inline(always)]
pub fn block_0x00211dcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2170320u32);
    emu.adi_no_count(12usize, 0usize, 160u32, 2170324u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2170328u32);
    emu.apc_no_count(1usize, 2170328u32, 4294905856u32, 2170332u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2170336u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(420u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00211de0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 8usize, 160u32, 2170340u32)?;
    emu.adi_no_count(10usize, 0usize, 2u32, 2170344u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2170376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211e08));
    } else {
        emu.pc = 2170348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211dec));
    }
}
#[inline(always)]
pub fn block_0x00211dec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2170352u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294965324u32, 2170356u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2170360u32);
    emu.adi_no_count(14usize, 0usize, 2u32, 2170364u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2170368u32);
    emu.adi_no_count(12usize, 15usize, 0u32, 2170372u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2170376u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2170408u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211e28));
}
#[inline(always)]
pub fn block_0x00211e08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 41u32, 2170380u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2171052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002120ac));
    } else {
        emu.pc = 2170384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211e10));
    }
}
#[inline(always)]
pub fn block_0x00211e10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2170388u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965324u32, 2170392u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2170396u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2170400u32);
    emu.adi_no_count(13usize, 8usize, 0u32, 2170404u32);
    emu.adi_no_count(14usize, 15usize, 0u32, 2170408u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2170408u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211e28));
}
#[inline(always)]
pub fn block_0x00211e28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2170408u32, 4294946816u32, 2170412u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2170416u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966596u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00211e30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2170420u32);
    emu.adi_no_count(11usize, 2usize, 0u32, 2170424u32);
    emu.adi_no_count(12usize, 0usize, 160u32, 2170428u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2170432u32);
    emu.apc_no_count(1usize, 2170432u32, 4294905856u32, 2170436u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2170440u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(564u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00211e48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 8usize, 160u32, 2170444u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2170444u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211e4c));
}
#[inline(always)]
pub fn block_0x00211e4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 9usize, 32u32, 2170448u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2170580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211ed4));
    } else {
        emu.pc = 2170452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211e54));
    }
}
#[inline(always)]
pub fn block_0x00211e54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2170456u32);
    emu.adi_no_count(12usize, 0usize, 160u32, 2170460u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2170464u32);
    emu.apc_no_count(1usize, 2170464u32, 4294905856u32, 2170468u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2170472u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(284u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00211e68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 8usize, 160u32, 2170476u32)?;
    emu.adi_no_count(10usize, 0usize, 3u32, 2170480u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2170512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211e90));
    } else {
        emu.pc = 2170484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211e74));
    }
}
#[inline(always)]
pub fn block_0x00211e74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2170488u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294965332u32, 2170492u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2170496u32);
    emu.adi_no_count(14usize, 0usize, 3u32, 2170500u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2170504u32);
    emu.adi_no_count(12usize, 15usize, 0u32, 2170508u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2170512u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2170544u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211eb0));
}
#[inline(always)]
pub fn block_0x00211e90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 41u32, 2170516u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2171052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002120ac));
    } else {
        emu.pc = 2170520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211e98));
    }
}
#[inline(always)]
pub fn block_0x00211e98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2170524u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965332u32, 2170528u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2170532u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2170536u32);
    emu.adi_no_count(13usize, 8usize, 0u32, 2170540u32);
    emu.adi_no_count(14usize, 15usize, 0u32, 2170544u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2170544u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211eb0));
}
#[inline(always)]
pub fn block_0x00211eb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2170544u32, 4294946816u32, 2170548u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2170552u32;
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
pub fn block_0x00211eb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2170556u32);
    emu.adi_no_count(11usize, 2usize, 0u32, 2170560u32);
    emu.adi_no_count(12usize, 0usize, 160u32, 2170564u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2170568u32);
    emu.apc_no_count(1usize, 2170568u32, 4294905856u32, 2170572u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2170576u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(428u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00211ed0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 8usize, 160u32, 2170580u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2170580u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211ed4));
}
#[inline(always)]
pub fn block_0x00211ed4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 9usize, 64u32, 2170584u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2170716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211f5c));
    } else {
        emu.pc = 2170588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211edc));
    }
}
#[inline(always)]
pub fn block_0x00211edc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2170592u32);
    emu.adi_no_count(12usize, 0usize, 160u32, 2170596u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2170600u32);
    emu.apc_no_count(1usize, 2170600u32, 4294905856u32, 2170604u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2170608u32;
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
pub fn block_0x00211ef0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 8usize, 160u32, 2170612u32)?;
    emu.adi_no_count(10usize, 0usize, 5u32, 2170616u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2170648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211f18));
    } else {
        emu.pc = 2170620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211efc));
    }
}
#[inline(always)]
pub fn block_0x00211efc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2170624u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294965344u32, 2170628u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2170632u32);
    emu.adi_no_count(14usize, 0usize, 5u32, 2170636u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2170640u32);
    emu.adi_no_count(12usize, 15usize, 0u32, 2170644u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2170648u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2170680u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211f38));
}
#[inline(always)]
pub fn block_0x00211f18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 41u32, 2170652u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2171052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002120ac));
    } else {
        emu.pc = 2170656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211f20));
    }
}
#[inline(always)]
pub fn block_0x00211f20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2170660u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965344u32, 2170664u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2170668u32);
    emu.adi_no_count(12usize, 0usize, 5u32, 2170672u32);
    emu.adi_no_count(13usize, 8usize, 0u32, 2170676u32);
    emu.adi_no_count(14usize, 15usize, 0u32, 2170680u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2170680u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211f38));
}
#[inline(always)]
pub fn block_0x00211f38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2170680u32, 4294946816u32, 2170684u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2170688u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966324u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00211f40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2170692u32);
    emu.adi_no_count(11usize, 2usize, 0u32, 2170696u32);
    emu.adi_no_count(12usize, 0usize, 160u32, 2170700u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2170704u32);
    emu.apc_no_count(1usize, 2170704u32, 4294905856u32, 2170708u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2170712u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(292u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00211f58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 8usize, 160u32, 2170716u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2170716u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211f5c));
}
#[inline(always)]
pub fn block_0x00211f5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 9usize, 128u32, 2170720u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2170852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211fe4));
    } else {
        emu.pc = 2170724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211f64));
    }
}
#[inline(always)]
pub fn block_0x00211f64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2170728u32);
    emu.adi_no_count(12usize, 0usize, 160u32, 2170732u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2170736u32);
    emu.apc_no_count(1usize, 2170736u32, 4294905856u32, 2170740u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2170744u32;
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
pub fn block_0x00211f78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 8usize, 160u32, 2170748u32)?;
    emu.adi_no_count(10usize, 0usize, 10u32, 2170752u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2170784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211fa0));
    } else {
        emu.pc = 2170756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211f84));
    }
}
#[inline(always)]
pub fn block_0x00211f84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2170760u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294965364u32, 2170764u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2170768u32);
    emu.adi_no_count(14usize, 0usize, 10u32, 2170772u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2170776u32);
    emu.adi_no_count(12usize, 15usize, 0u32, 2170780u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2170784u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2170816u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211fc0));
}
#[inline(always)]
pub fn block_0x00211fa0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 41u32, 2170788u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2171052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002120ac));
    } else {
        emu.pc = 2170792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211fa8));
    }
}
#[inline(always)]
pub fn block_0x00211fa8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2170796u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965364u32, 2170800u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2170804u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2170808u32);
    emu.adi_no_count(13usize, 8usize, 0u32, 2170812u32);
    emu.adi_no_count(14usize, 15usize, 0u32, 2170816u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2170816u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211fc0));
}
#[inline(always)]
pub fn block_0x00211fc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2170816u32, 4294946816u32, 2170820u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2170824u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966188u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00211fc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2170828u32);
    emu.adi_no_count(11usize, 2usize, 0u32, 2170832u32);
    emu.adi_no_count(12usize, 0usize, 160u32, 2170836u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2170840u32);
    emu.apc_no_count(1usize, 2170840u32, 4294905856u32, 2170844u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2170848u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x00211fe0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 8usize, 160u32, 2170852u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2170852u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211fe4));
}
#[inline(always)]
pub fn block_0x00211fe4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 9usize, 256u32, 2170856u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2170988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021206c));
    } else {
        emu.pc = 2170860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211fec));
    }
}
#[inline(always)]
pub fn block_0x00211fec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2170864u32);
    emu.adi_no_count(12usize, 0usize, 160u32, 2170868u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2170872u32);
    emu.apc_no_count(1usize, 2170872u32, 4294905856u32, 2170876u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2170880u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967172u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00212000(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 8usize, 160u32, 2170884u32)?;
    emu.adi_no_count(10usize, 0usize, 19u32, 2170888u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2170920u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212028));
    } else {
        emu.pc = 2170892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021200c));
    }
}
#[inline(always)]
pub fn block_0x0021200c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2170896u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294965404u32, 2170900u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2170904u32);
    emu.adi_no_count(14usize, 0usize, 19u32, 2170908u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2170912u32);
    emu.adi_no_count(12usize, 15usize, 0u32, 2170916u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2170920u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2170952u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212048));
}
#[inline(always)]
pub fn block_0x00212028(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 41u32, 2170924u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2171052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002120ac));
    } else {
        emu.pc = 2170928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212030));
    }
}
#[inline(always)]
pub fn block_0x00212030(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2170932u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965404u32, 2170936u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2170940u32);
    emu.adi_no_count(12usize, 0usize, 19u32, 2170944u32);
    emu.adi_no_count(13usize, 8usize, 0u32, 2170948u32);
    emu.adi_no_count(14usize, 15usize, 0u32, 2170952u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2170952u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212048));
}
#[inline(always)]
pub fn block_0x00212048(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2170952u32, 4294946816u32, 2170956u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2170960u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966052u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00212050(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2170964u32);
    emu.adi_no_count(11usize, 2usize, 0u32, 2170968u32);
    emu.adi_no_count(12usize, 0usize, 160u32, 2170972u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2170976u32);
    emu.apc_no_count(1usize, 2170976u32, 4294905856u32, 2170980u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2170984u32;
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
pub fn block_0x00212068(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 8usize, 160u32, 2170988u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2170988u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021206c));
}
#[inline(always)]
pub fn block_0x0021206c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 0u32, 2170992u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2170996u32);
    emu.apc_no_count(1usize, 2170996u32, 4294946816u32, 2171000u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2171004u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965540u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021207c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 0u32, 2171008u32);
    emu.lw_no_count(1usize, 2usize, 172u32, 2171012u32)?;
    emu.lw_no_count(8usize, 2usize, 168u32, 2171016u32)?;
    emu.lw_no_count(9usize, 2usize, 164u32, 2171020u32)?;
    emu.lw_no_count(18usize, 2usize, 160u32, 2171024u32)?;
    emu.adi_no_count(2usize, 2usize, 176u32, 2171028u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2171032u32;
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
pub fn block_0x00212098(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2171036u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 920u32, 2171040u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2171044u32);
    emu.apc_no_count(1usize, 2171044u32, 8192u32, 2171048u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2171052u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966220u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002120ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2171056u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 920u32, 2171060u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2171064u32);
    emu.adi_no_count(10usize, 15usize, 0u32, 2171068u32);
    emu.apc_no_count(1usize, 2171068u32, 8192u32, 2171072u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2171076u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966196u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002120c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2171080u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 920u32, 2171084u32);
    emu.adi_no_count(10usize, 0usize, 40u32, 2171088u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2171092u32);
    emu.apc_no_count(1usize, 2171092u32, 4294942720u32, 2171096u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2171100u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(988u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
