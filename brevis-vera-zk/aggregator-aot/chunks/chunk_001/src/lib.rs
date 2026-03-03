pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2102012u32;
pub const PC_MAX: u32 = 2105340u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 112usize] = [
        block_0x002012fc,
        block_0x0020132c,
        block_0x0020134c,
        block_0x0020135c,
        block_0x00201360,
        block_0x002013cc,
        block_0x002013d8,
        block_0x002013dc,
        block_0x002013f4,
        block_0x00201404,
        block_0x0020141c,
        block_0x00201420,
        block_0x00201428,
        block_0x00201434,
        block_0x00201440,
        block_0x00201460,
        block_0x00201478,
        block_0x0020147c,
        block_0x00201494,
        block_0x00201498,
        block_0x002014a0,
        block_0x002014bc,
        block_0x002014c0,
        block_0x002014cc,
        block_0x002014d4,
        block_0x002014e8,
        block_0x002014fc,
        block_0x00201504,
        block_0x00201514,
        block_0x0020152c,
        block_0x00201530,
        block_0x00201540,
        block_0x00201550,
        block_0x00201560,
        block_0x002015a0,
        block_0x002015b8,
        block_0x002015c0,
        block_0x00201660,
        block_0x00201678,
        block_0x00201680,
        block_0x00201684,
        block_0x00201690,
        block_0x002016cc,
        block_0x0020176c,
        block_0x0020177c,
        block_0x0020179c,
        block_0x002017ac,
        block_0x002017b0,
        block_0x0020181c,
        block_0x00201828,
        block_0x00201840,
        block_0x00201850,
        block_0x00201860,
        block_0x00201878,
        block_0x00201880,
        block_0x00201894,
        block_0x002018b4,
        block_0x002018c4,
        block_0x002018c8,
        block_0x002018dc,
        block_0x002018e4,
        block_0x002018e8,
        block_0x00201954,
        block_0x00201960,
        block_0x00201970,
        block_0x00201980,
        block_0x00201990,
        block_0x002019ac,
        block_0x002019b4,
        block_0x002019bc,
        block_0x002019e0,
        block_0x002019e8,
        block_0x00201a88,
        block_0x00201ab0,
        block_0x00201ab8,
        block_0x00201b50,
        block_0x00201b80,
        block_0x00201b94,
        block_0x00201ba8,
        block_0x00201c30,
        block_0x00201c4c,
        block_0x00201c54,
        block_0x00201c6c,
        block_0x00201c84,
        block_0x00201cc4,
        block_0x00201cd8,
        block_0x00201ce8,
        block_0x00201cf4,
        block_0x00201cfc,
        block_0x00201d08,
        block_0x00201d1c,
        block_0x00201d5c,
        block_0x00201d64,
        block_0x00201d78,
        block_0x00201da0,
        block_0x00201da8,
        block_0x00201db8,
        block_0x00201dc4,
        block_0x00201dc8,
        block_0x00201de0,
        block_0x00201e04,
        block_0x00201e40,
        block_0x00201e48,
        block_0x00201e50,
        block_0x00201e68,
        block_0x00201f50,
        block_0x00201fa4,
        block_0x00201fbc,
        block_0x00201fd4,
        block_0x00201fe8,
        block_0x00201ff0,
        block_0x00201ffc,
    ];
    const IDX: [u16; 833usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 3u16, 0u16, 0u16, 0u16, 4u16, 5u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        6u16, 0u16, 0u16, 7u16, 8u16, 0u16, 0u16, 0u16, 0u16, 0u16, 9u16, 0u16, 0u16,
        0u16, 10u16, 0u16, 0u16, 0u16, 0u16, 0u16, 11u16, 12u16, 0u16, 13u16, 0u16, 0u16,
        14u16, 0u16, 0u16, 15u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 16u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 17u16, 18u16, 0u16, 0u16, 0u16, 0u16, 0u16, 19u16, 20u16,
        0u16, 21u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 22u16, 23u16, 0u16, 0u16, 24u16,
        0u16, 25u16, 0u16, 0u16, 0u16, 0u16, 26u16, 0u16, 0u16, 0u16, 0u16, 27u16, 0u16,
        28u16, 0u16, 0u16, 0u16, 29u16, 0u16, 0u16, 0u16, 0u16, 0u16, 30u16, 31u16, 0u16,
        0u16, 0u16, 32u16, 0u16, 0u16, 0u16, 33u16, 0u16, 0u16, 0u16, 34u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        35u16, 0u16, 0u16, 0u16, 0u16, 0u16, 36u16, 0u16, 37u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 38u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 39u16, 0u16, 40u16, 41u16, 0u16, 0u16, 42u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 43u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 44u16, 0u16,
        0u16, 0u16, 45u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 46u16, 0u16, 0u16,
        0u16, 47u16, 48u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 49u16, 0u16, 0u16, 50u16, 0u16, 0u16, 0u16, 0u16, 0u16, 51u16,
        0u16, 0u16, 0u16, 52u16, 0u16, 0u16, 0u16, 53u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        54u16, 0u16, 55u16, 0u16, 0u16, 0u16, 0u16, 56u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 57u16, 0u16, 0u16, 0u16, 58u16, 59u16, 0u16, 0u16, 0u16, 0u16, 60u16,
        0u16, 61u16, 62u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 63u16, 0u16, 0u16, 64u16, 0u16, 0u16, 0u16, 65u16, 0u16, 0u16,
        0u16, 66u16, 0u16, 0u16, 0u16, 67u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 68u16,
        0u16, 69u16, 0u16, 70u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 71u16,
        0u16, 72u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 73u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 74u16,
        0u16, 75u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        76u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 77u16,
        0u16, 0u16, 0u16, 0u16, 78u16, 0u16, 0u16, 0u16, 0u16, 79u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 80u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 81u16, 0u16,
        82u16, 0u16, 0u16, 0u16, 0u16, 0u16, 83u16, 0u16, 0u16, 0u16, 0u16, 0u16, 84u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 85u16, 0u16, 0u16, 0u16, 0u16, 86u16, 0u16, 0u16, 0u16, 87u16, 0u16,
        0u16, 88u16, 0u16, 89u16, 0u16, 0u16, 90u16, 0u16, 0u16, 0u16, 0u16, 91u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 92u16, 0u16, 93u16, 0u16, 0u16, 0u16, 0u16, 94u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 95u16, 0u16, 96u16, 0u16, 0u16, 0u16, 97u16, 0u16,
        0u16, 98u16, 99u16, 0u16, 0u16, 0u16, 0u16, 0u16, 100u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 101u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 102u16, 0u16, 103u16, 0u16, 104u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 105u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 106u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 107u16, 0u16, 0u16, 0u16, 0u16, 0u16, 108u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 109u16, 0u16, 0u16, 0u16, 0u16, 110u16, 0u16, 111u16,
        0u16, 0u16, 112u16,
    ];
    if pc < 2102012u32 || pc > 2105340u32 {
        return None;
    }
    let word_offset = ((pc - 2102012u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x002012fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2102016u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2102020u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2102024u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2102028u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2102032u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2102036u32)?;
    emu.sw_no_count(20usize, 2usize, 24u32, 2102040u32)?;
    emu.adi_no_count(18usize, 11usize, 0u32, 2102044u32);
    emu.lw_no_count(20usize, 11usize, 4u32, 2102048u32)?;
    emu.adi_no_count(11usize, 0usize, 7u32, 2102052u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2102056u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2102112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201360));
    } else {
        emu.pc = 2102060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020132c));
    }
}
#[inline(always)]
pub fn block_0x0020132c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 8u32, 2102064u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2102068u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1281u32, 2102072u32);
    emu.sw_no_count(11usize, 2usize, 8u32, 2102076u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2102080u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2102084u32);
    emu.apc_no_count(1usize, 2102084u32, 32768u32, 2102088u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102092u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(424u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020134c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2102096u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2102100u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2102104u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2102232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002013d8));
    } else {
        emu.pc = 2102108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020135c));
    }
}
#[inline(always)]
pub fn block_0x0020135c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2102112u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2102324u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201434));
}
#[inline(never)]
pub fn block_0x00201360(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 0u32, 2102116u32)?;
    emu.lbu_no_count(11usize, 10usize, 1u32, 2102120u32);
    emu.lbu_no_count(12usize, 10usize, 2u32, 2102124u32);
    emu.lbu_no_count(13usize, 10usize, 3u32, 2102128u32);
    emu.lbu_no_count(14usize, 10usize, 0u32, 2102132u32);
    emu.sli_no_count(11usize, 11usize, 8u32, 2102136u32);
    emu.sli_no_count(12usize, 12usize, 16u32, 2102140u32);
    emu.sli_no_count(13usize, 13usize, 24u32, 2102144u32);
    emu.orr_no_count(11usize, 11usize, 14usize, 2102148u32);
    emu.lbu_no_count(14usize, 10usize, 4u32, 2102152u32);
    emu.lbu_no_count(15usize, 10usize, 5u32, 2102156u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2102160u32);
    emu.lbu_no_count(13usize, 10usize, 6u32, 2102164u32);
    emu.lbu_no_count(16usize, 10usize, 7u32, 2102168u32);
    emu.sli_no_count(15usize, 15usize, 8u32, 2102172u32);
    emu.orr_no_count(14usize, 15usize, 14usize, 2102176u32);
    emu.sli_no_count(13usize, 13usize, 16u32, 2102180u32);
    emu.sli_no_count(16usize, 16usize, 24u32, 2102184u32);
    emu.orr_no_count(13usize, 16usize, 13usize, 2102188u32);
    emu.adi_no_count(20usize, 20usize, 4294967288u32, 2102192u32);
    emu.adi_no_count(15usize, 10usize, 8u32, 2102196u32);
    emu.orr_no_count(10usize, 12usize, 11usize, 2102200u32);
    emu.orr_no_count(11usize, 13usize, 14usize, 2102204u32);
    emu.sw_no_count(15usize, 18usize, 0u32, 2102208u32)?;
    emu.sw_no_count(20usize, 18usize, 4u32, 2102212u32)?;
    emu.apc_no_count(1usize, 2102212u32, 32768u32, 2102216u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102220u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(60u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002013cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 11usize, 0u32, 2102224u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2102228u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2102324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201434));
    } else {
        emu.pc = 2102232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002013d8));
    }
}
#[inline(always)]
pub fn block_0x002013d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a >= b {
        emu.pc = 2102368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201460));
    } else {
        emu.pc = 2102236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002013dc));
    }
}
#[inline(always)]
pub fn block_0x002013dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 8u32, 2102240u32);
    emu.adi_no_count(11usize, 0usize, 37u32, 2102244u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2102248u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2102252u32);
    emu.apc_no_count(1usize, 2102252u32, 32768u32, 2102256u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102260u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(360u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002013f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 2usize, 8u32, 2102264u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2102268u32)?;
    emu.apc_no_count(1usize, 2102268u32, 28672u32, 2102272u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102276u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967236u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201404(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2102280u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1452u32, 2102284u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2102288u32);
    emu.adi_no_count(12usize, 0usize, 12u32, 2102292u32);
    emu.apc_no_count(1usize, 2102292u32, 32768u32, 2102296u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102300u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966600u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020141c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2102608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201550));
    } else {
        emu.pc = 2102304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201420));
    }
}
#[inline(always)]
pub fn block_0x00201420(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2102308u32);
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2102312u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    emu.pc = 2102312u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201428));
}
#[inline(always)]
pub fn block_0x00201428(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 9usize, 0u32, 2102316u32)?;
    emu.sw_no_count(18usize, 9usize, 4u32, 2102320u32)?;
    emu.sw_no_count(19usize, 9usize, 8u32, 2102324u32)?;
    emu.add_memory_rw_events(3usize);
    emu.pc = 2102324u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201434));
}
#[inline(always)]
pub fn block_0x00201434(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2102328u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 8usize, 0u32, 2102332u32)?;
    emu.sw_no_count(9usize, 8usize, 4u32, 2102336u32)?;
    emu.add_memory_rw_events(3usize);
    emu.pc = 2102336u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201440));
}
#[inline(always)]
pub fn block_0x00201440(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2102340u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2102344u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2102348u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2102352u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2102356u32)?;
    emu.lw_no_count(20usize, 2usize, 24u32, 2102360u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2102364u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102368u32;
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
pub fn block_0x00201460(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(19usize, 18usize, 0u32, 2102372u32)?;
    emu.sbr_no_count(10usize, 20usize, 9usize, 2102376u32);
    emu.adr_no_count(11usize, 19usize, 9usize, 2102380u32);
    emu.sw_no_count(11usize, 18usize, 0u32, 2102384u32)?;
    emu.sw_no_count(10usize, 18usize, 4u32, 2102388u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2102420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201494));
    } else {
        emu.pc = 2102392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201478));
    }
}
#[inline(always)]
pub fn block_0x00201478(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 0u32, 2102396u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2102396u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020147c));
}
#[inline(always)]
pub fn block_0x0020147c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2102400u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 2004u32, 2102404u32);
    emu.adi_no_count(10usize, 20usize, 0u32, 2102408u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2102412u32);
    emu.apc_no_count(1usize, 2102412u32, 102400u32, 2102416u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102420u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(36u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201494(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2102476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002014cc));
    } else {
        emu.pc = 2102424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201498));
    }
}
#[inline(always)]
pub fn block_0x00201498(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2102424u32, 28672u32, 2102428u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102432u32;
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
pub fn block_0x002014a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2102436u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1452u32, 2102440u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2102444u32);
    emu.adi_no_count(20usize, 0usize, 1u32, 2102448u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2102452u32);
    emu.apc_no_count(1usize, 2102452u32, 32768u32, 2102456u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102460u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966440u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002014bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2102396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020147c));
    } else {
        emu.pc = 2102464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002014c0));
    }
}
#[inline(always)]
pub fn block_0x002014c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2102468u32);
    emu.adi_no_count(20usize, 9usize, 0u32, 2102472u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2102476u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2102484u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002014d4));
}
#[inline(always)]
pub fn block_0x002014cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 0u32, 2102480u32);
    emu.adi_no_count(18usize, 0usize, 1u32, 2102484u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2102484u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002014d4));
}
#[inline(always)]
pub fn block_0x002014d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2102488u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2102492u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2102496u32);
    emu.apc_no_count(1usize, 2102496u32, 32768u32, 2102500u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102504u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965332u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002014e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 8u32, 2102508u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2102512u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2102516u32);
    emu.apc_no_count(1usize, 2102516u32, 118784u32, 2102520u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102524u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965988u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002014fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2102528u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2102592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201540));
    } else {
        emu.pc = 2102532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201504));
    }
}
#[inline(always)]
pub fn block_0x00201504(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 2usize, 12u32, 2102536u32)?;
    emu.lw_no_count(19usize, 2usize, 16u32, 2102540u32)?;
    emu.apc_no_count(1usize, 2102540u32, 28672u32, 2102544u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102548u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x00201514(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2102552u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1452u32, 2102556u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2102560u32);
    emu.adi_no_count(12usize, 0usize, 12u32, 2102564u32);
    emu.apc_no_count(1usize, 2102564u32, 32768u32, 2102568u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102572u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966328u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020152c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2102608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201550));
    } else {
        emu.pc = 2102576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201530));
    }
}
#[inline(always)]
pub fn block_0x00201530(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2102580u32);
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2102584u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1u32, 2102588u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2102592u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2102312u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201428));
}
#[inline(always)]
pub fn block_0x00201540(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(20usize, 8usize, 0u32, 2102596u32)?;
    emu.sw_no_count(18usize, 8usize, 4u32, 2102600u32)?;
    emu.sw_no_count(9usize, 8usize, 8u32, 2102604u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2102608u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2102336u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201440));
}
#[inline(always)]
pub fn block_0x00201550(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2102612u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2102616u32);
    emu.apc_no_count(1usize, 2102616u32, 102400u32, 2102620u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102624u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967152u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00201560(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967040u32, 2102628u32);
    emu.sw_no_count(1usize, 2usize, 252u32, 2102632u32)?;
    emu.sw_no_count(8usize, 2usize, 248u32, 2102636u32)?;
    emu.sw_no_count(9usize, 2usize, 244u32, 2102640u32)?;
    emu.sw_no_count(18usize, 2usize, 240u32, 2102644u32)?;
    emu.sw_no_count(19usize, 2usize, 236u32, 2102648u32)?;
    emu.sw_no_count(20usize, 2usize, 232u32, 2102652u32)?;
    emu.sw_no_count(21usize, 2usize, 228u32, 2102656u32)?;
    emu.sw_no_count(22usize, 2usize, 224u32, 2102660u32)?;
    emu.sw_no_count(23usize, 2usize, 220u32, 2102664u32)?;
    emu.sw_no_count(24usize, 2usize, 216u32, 2102668u32)?;
    emu.sw_no_count(25usize, 2usize, 212u32, 2102672u32)?;
    emu.sw_no_count(26usize, 2usize, 208u32, 2102676u32)?;
    emu.sw_no_count(27usize, 2usize, 204u32, 2102680u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2102684u32);
    emu.add_memory_rw_events(15usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2103392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201860));
    } else {
        emu.pc = 2102688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002015a0));
    }
}
#[inline(always)]
pub fn block_0x002015a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 15usize, 0u32, 2102692u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2102696u32);
    emu.adi_no_count(10usize, 2usize, 168u32, 2102700u32);
    emu.adi_no_count(12usize, 0usize, 32u32, 2102704u32);
    emu.apc_no_count(1usize, 2102704u32, 4294963200u32, 2102708u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102712u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1064u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002015b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 168u32, 2102716u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2102912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201680));
    } else {
        emu.pc = 2102720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002015c0));
    }
}
#[inline(never)]
pub fn block_0x002015c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 40u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(22usize, 2usize, 169u32, 2102724u32);
    emu.lbu_no_count(23usize, 2usize, 170u32, 2102728u32);
    emu.lbu_no_count(10usize, 2usize, 171u32, 2102732u32);
    emu.lw_no_count(20usize, 2usize, 172u32, 2102736u32)?;
    emu.lw_no_count(11usize, 2usize, 176u32, 2102740u32)?;
    emu.lw_no_count(12usize, 2usize, 180u32, 2102744u32)?;
    emu.lw_no_count(13usize, 2usize, 184u32, 2102748u32)?;
    emu.lw_no_count(14usize, 2usize, 188u32, 2102752u32)?;
    emu.lw_no_count(15usize, 2usize, 192u32, 2102756u32)?;
    emu.lw_no_count(16usize, 2usize, 196u32, 2102760u32)?;
    emu.lbu_no_count(17usize, 2usize, 200u32, 2102764u32);
    emu.adi_no_count(5usize, 0usize, 1u32, 2102768u32);
    emu.sw_no_count(11usize, 2usize, 136u32, 2102772u32)?;
    emu.sw_no_count(12usize, 2usize, 140u32, 2102776u32)?;
    emu.sw_no_count(13usize, 2usize, 144u32, 2102780u32)?;
    emu.sw_no_count(14usize, 2usize, 148u32, 2102784u32)?;
    emu.sw_no_count(15usize, 2usize, 152u32, 2102788u32)?;
    emu.sw_no_count(16usize, 2usize, 156u32, 2102792u32)?;
    emu.sb_no_count(17usize, 2usize, 160u32, 2102796u32);
    emu.lw_no_count(11usize, 2usize, 136u32, 2102800u32)?;
    emu.lw_no_count(12usize, 2usize, 140u32, 2102804u32)?;
    emu.lw_no_count(13usize, 2usize, 144u32, 2102808u32)?;
    emu.lw_no_count(14usize, 2usize, 148u32, 2102812u32)?;
    emu.lw_no_count(15usize, 2usize, 152u32, 2102816u32)?;
    emu.lw_no_count(16usize, 2usize, 156u32, 2102820u32)?;
    emu.lbu_no_count(17usize, 2usize, 160u32, 2102824u32);
    emu.sw_no_count(11usize, 2usize, 44u32, 2102828u32)?;
    emu.sw_no_count(12usize, 2usize, 48u32, 2102832u32)?;
    emu.sw_no_count(13usize, 2usize, 52u32, 2102836u32)?;
    emu.sw_no_count(14usize, 2usize, 56u32, 2102840u32)?;
    emu.sri_no_count(11usize, 20usize, 16u32, 2102844u32);
    emu.sw_no_count(15usize, 2usize, 60u32, 2102848u32)?;
    emu.sw_no_count(16usize, 2usize, 64u32, 2102852u32)?;
    emu.sb_no_count(17usize, 2usize, 68u32, 2102856u32);
    emu.sri_no_count(12usize, 20usize, 8u32, 2102860u32);
    emu.sb_no_count(10usize, 2usize, 100u32, 2102864u32);
    emu.sb_no_count(20usize, 2usize, 101u32, 2102868u32);
    emu.sb_no_count(12usize, 2usize, 102u32, 2102872u32);
    emu.sb_no_count(11usize, 2usize, 103u32, 2102876u32);
    emu.add_memory_rw_events(39usize);
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2103496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002018c8));
    } else {
        emu.pc = 2102880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201660));
    }
}
#[inline(always)]
pub fn block_0x00201660(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(21usize, 2usize, 100u32, 2102884u32)?;
    emu.adi_no_count(10usize, 2usize, 168u32, 2102888u32);
    emu.adi_no_count(12usize, 0usize, 32u32, 2102892u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2102896u32);
    emu.apc_no_count(1usize, 2102896u32, 4294963200u32, 2102900u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102904u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(872u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201678(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 168u32, 2102908u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2102988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002016cc));
    } else {
        emu.pc = 2102912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201680));
    }
}
#[inline(always)]
pub fn block_0x00201680(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 172u32, 2102916u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2102916u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201684));
}
#[inline(always)]
pub fn block_0x00201684(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2102920u32;
    emu.update_insn_clock();
    emu.sw_no_count(11usize, 8usize, 0u32, 2102924u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2102928u32)?;
    emu.add_memory_rw_events(3usize);
    emu.pc = 2102928u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201690));
}
#[inline]
pub fn block_0x00201690(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 252u32, 2102932u32)?;
    emu.lw_no_count(8usize, 2usize, 248u32, 2102936u32)?;
    emu.lw_no_count(9usize, 2usize, 244u32, 2102940u32)?;
    emu.lw_no_count(18usize, 2usize, 240u32, 2102944u32)?;
    emu.lw_no_count(19usize, 2usize, 236u32, 2102948u32)?;
    emu.lw_no_count(20usize, 2usize, 232u32, 2102952u32)?;
    emu.lw_no_count(21usize, 2usize, 228u32, 2102956u32)?;
    emu.lw_no_count(22usize, 2usize, 224u32, 2102960u32)?;
    emu.lw_no_count(23usize, 2usize, 220u32, 2102964u32)?;
    emu.lw_no_count(24usize, 2usize, 216u32, 2102968u32)?;
    emu.lw_no_count(25usize, 2usize, 212u32, 2102972u32)?;
    emu.lw_no_count(26usize, 2usize, 208u32, 2102976u32)?;
    emu.lw_no_count(27usize, 2usize, 204u32, 2102980u32)?;
    emu.adi_no_count(2usize, 2usize, 256u32, 2102984u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102988u32;
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
pub fn block_0x002016cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 40u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(24usize, 18usize, 4294967294u32, 2102992u32);
    emu.lbu_no_count(26usize, 2usize, 169u32, 2102996u32);
    emu.lbu_no_count(27usize, 2usize, 170u32, 2103000u32);
    emu.lbu_no_count(10usize, 2usize, 171u32, 2103004u32);
    emu.lw_no_count(25usize, 2usize, 172u32, 2103008u32)?;
    emu.lw_no_count(11usize, 2usize, 176u32, 2103012u32)?;
    emu.lw_no_count(12usize, 2usize, 180u32, 2103016u32)?;
    emu.lw_no_count(13usize, 2usize, 184u32, 2103020u32)?;
    emu.lw_no_count(14usize, 2usize, 188u32, 2103024u32)?;
    emu.lw_no_count(15usize, 2usize, 192u32, 2103028u32)?;
    emu.lw_no_count(16usize, 2usize, 196u32, 2103032u32)?;
    emu.lbu_no_count(17usize, 2usize, 200u32, 2103036u32);
    emu.sw_no_count(11usize, 2usize, 136u32, 2103040u32)?;
    emu.sw_no_count(12usize, 2usize, 140u32, 2103044u32)?;
    emu.sw_no_count(13usize, 2usize, 144u32, 2103048u32)?;
    emu.sw_no_count(14usize, 2usize, 148u32, 2103052u32)?;
    emu.sw_no_count(15usize, 2usize, 152u32, 2103056u32)?;
    emu.sw_no_count(16usize, 2usize, 156u32, 2103060u32)?;
    emu.sb_no_count(17usize, 2usize, 160u32, 2103064u32);
    emu.lw_no_count(11usize, 2usize, 136u32, 2103068u32)?;
    emu.lw_no_count(12usize, 2usize, 140u32, 2103072u32)?;
    emu.lw_no_count(13usize, 2usize, 144u32, 2103076u32)?;
    emu.lw_no_count(14usize, 2usize, 148u32, 2103080u32)?;
    emu.lw_no_count(15usize, 2usize, 152u32, 2103084u32)?;
    emu.lw_no_count(16usize, 2usize, 156u32, 2103088u32)?;
    emu.lbu_no_count(17usize, 2usize, 160u32, 2103092u32);
    emu.sw_no_count(11usize, 2usize, 72u32, 2103096u32)?;
    emu.sw_no_count(12usize, 2usize, 76u32, 2103100u32)?;
    emu.sw_no_count(13usize, 2usize, 80u32, 2103104u32)?;
    emu.sw_no_count(14usize, 2usize, 84u32, 2103108u32)?;
    emu.sri_no_count(11usize, 25usize, 16u32, 2103112u32);
    emu.sw_no_count(15usize, 2usize, 88u32, 2103116u32)?;
    emu.sw_no_count(16usize, 2usize, 92u32, 2103120u32)?;
    emu.sb_no_count(17usize, 2usize, 96u32, 2103124u32);
    emu.sri_no_count(12usize, 25usize, 8u32, 2103128u32);
    emu.sb_no_count(10usize, 2usize, 100u32, 2103132u32);
    emu.sb_no_count(25usize, 2usize, 101u32, 2103136u32);
    emu.sb_no_count(12usize, 2usize, 102u32, 2103140u32);
    emu.sb_no_count(11usize, 2usize, 103u32, 2103144u32);
    emu.add_memory_rw_events(39usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a == b {
        emu.pc = 2103696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201990));
    } else {
        emu.pc = 2103148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020176c));
    }
}
#[inline(always)]
pub fn block_0x0020176c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 4u32, 2103152u32)?;
    emu.lw_no_count(19usize, 2usize, 100u32, 2103156u32)?;
    emu.adi_no_count(11usize, 0usize, 7u32, 2103160u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2103216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002017b0));
    } else {
        emu.pc = 2103164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020177c));
    }
}
#[inline(always)]
pub fn block_0x0020177c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 8u32, 2103168u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2103172u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1281u32, 2103176u32);
    emu.sw_no_count(11usize, 2usize, 136u32, 2103180u32)?;
    emu.sw_no_count(10usize, 2usize, 140u32, 2103184u32)?;
    emu.adi_no_count(10usize, 2usize, 136u32, 2103188u32);
    emu.apc_no_count(1usize, 2103188u32, 32768u32, 2103192u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103196u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966616u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020179c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2103200u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2103204u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2103208u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2103336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201828));
    } else {
        emu.pc = 2103212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002017ac));
    }
}
#[inline(always)]
pub fn block_0x002017ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2103216u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2103376u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201850));
}
#[inline(never)]
pub fn block_0x002017b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 9usize, 0u32, 2103220u32)?;
    emu.lbu_no_count(12usize, 11usize, 1u32, 2103224u32);
    emu.lbu_no_count(13usize, 11usize, 2u32, 2103228u32);
    emu.lbu_no_count(14usize, 11usize, 3u32, 2103232u32);
    emu.lbu_no_count(15usize, 11usize, 0u32, 2103236u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2103240u32);
    emu.sli_no_count(13usize, 13usize, 16u32, 2103244u32);
    emu.sli_no_count(14usize, 14usize, 24u32, 2103248u32);
    emu.orr_no_count(12usize, 12usize, 15usize, 2103252u32);
    emu.lbu_no_count(15usize, 11usize, 4u32, 2103256u32);
    emu.lbu_no_count(16usize, 11usize, 5u32, 2103260u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2103264u32);
    emu.lbu_no_count(14usize, 11usize, 6u32, 2103268u32);
    emu.lbu_no_count(17usize, 11usize, 7u32, 2103272u32);
    emu.sli_no_count(16usize, 16usize, 8u32, 2103276u32);
    emu.orr_no_count(15usize, 16usize, 15usize, 2103280u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2103284u32);
    emu.sli_no_count(17usize, 17usize, 24u32, 2103288u32);
    emu.orr_no_count(14usize, 17usize, 14usize, 2103292u32);
    emu.adi_no_count(16usize, 10usize, 4294967288u32, 2103296u32);
    emu.adi_no_count(17usize, 11usize, 8u32, 2103300u32);
    emu.orr_no_count(10usize, 13usize, 12usize, 2103304u32);
    emu.orr_no_count(11usize, 14usize, 15usize, 2103308u32);
    emu.sw_no_count(17usize, 9usize, 0u32, 2103312u32)?;
    emu.sw_no_count(16usize, 9usize, 4u32, 2103316u32)?;
    emu.apc_no_count(1usize, 2103316u32, 32768u32, 2103320u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103324u32;
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
pub fn block_0x0020181c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 11usize, 0u32, 2103328u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2103332u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2103376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201850));
    } else {
        emu.pc = 2103336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201828));
    }
}
#[inline(always)]
pub fn block_0x00201828(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(19usize, 2usize, 40u32, 2103340u32)?;
    emu.adi_no_count(10usize, 2usize, 168u32, 2103344u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2103348u32);
    emu.adi_no_count(12usize, 18usize, 0u32, 2103352u32);
    emu.apc_no_count(1usize, 2103352u32, 20480u32, 2103356u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103360u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(2036u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201840(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(19usize, 2usize, 168u32, 2103364u32)?;
    emu.lw_no_count(18usize, 2usize, 172u32, 2103368u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2103372u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a != b {
        emu.pc = 2103416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201878));
    } else {
        emu.pc = 2103376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201850));
    }
}
#[inline(always)]
pub fn block_0x00201850(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2103380u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 8usize, 0u32, 2103384u32)?;
    emu.sw_no_count(18usize, 8usize, 4u32, 2103388u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2103392u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2102928u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201690));
}
#[inline(always)]
pub fn block_0x00201860(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2103396u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966856u32, 2103400u32);
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2103404u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966864u32, 2103408u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2103412u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2103416u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2103516u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002018dc));
}
#[inline(always)]
pub fn block_0x00201878(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2103420u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a == b {
        emu.pc = 2104368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201c30));
    } else {
        emu.pc = 2103424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201880));
    }
}
#[inline(always)]
pub fn block_0x00201880(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 4u32, 2103428u32)?;
    emu.lw_no_count(11usize, 2usize, 176u32, 2103432u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2103436u32)?;
    emu.adi_no_count(11usize, 0usize, 7u32, 2103440u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2103528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002018e8));
    } else {
        emu.pc = 2103444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201894));
    }
}
#[inline(always)]
pub fn block_0x00201894(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 8u32, 2103448u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2103452u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1281u32, 2103456u32);
    emu.sw_no_count(11usize, 2usize, 136u32, 2103460u32)?;
    emu.sw_no_count(10usize, 2usize, 140u32, 2103464u32)?;
    emu.adi_no_count(10usize, 2usize, 136u32, 2103468u32);
    emu.apc_no_count(1usize, 2103468u32, 32768u32, 2103472u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103476u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966336u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002018b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2103480u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2103484u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2103488u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2103648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201960));
    } else {
        emu.pc = 2103492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002018c4));
    }
}
#[inline(always)]
pub fn block_0x002018c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2103496u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2103680u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201980));
}
#[inline(always)]
pub fn block_0x002018c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2103500u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966856u32, 2103504u32);
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2103508u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966864u32, 2103512u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2103516u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2103516u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002018dc));
}
#[inline(always)]
pub fn block_0x002018dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2103516u32, 12288u32, 2103520u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103524u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965652u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002018e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2103528u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2102916u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201684));
}
#[inline(never)]
pub fn block_0x002018e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 9usize, 0u32, 2103532u32)?;
    emu.lbu_no_count(12usize, 11usize, 1u32, 2103536u32);
    emu.lbu_no_count(13usize, 11usize, 2u32, 2103540u32);
    emu.lbu_no_count(14usize, 11usize, 3u32, 2103544u32);
    emu.lbu_no_count(15usize, 11usize, 0u32, 2103548u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2103552u32);
    emu.sli_no_count(13usize, 13usize, 16u32, 2103556u32);
    emu.sli_no_count(14usize, 14usize, 24u32, 2103560u32);
    emu.orr_no_count(12usize, 12usize, 15usize, 2103564u32);
    emu.lbu_no_count(15usize, 11usize, 4u32, 2103568u32);
    emu.lbu_no_count(16usize, 11usize, 5u32, 2103572u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2103576u32);
    emu.lbu_no_count(14usize, 11usize, 6u32, 2103580u32);
    emu.lbu_no_count(17usize, 11usize, 7u32, 2103584u32);
    emu.sli_no_count(16usize, 16usize, 8u32, 2103588u32);
    emu.orr_no_count(15usize, 16usize, 15usize, 2103592u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2103596u32);
    emu.sli_no_count(17usize, 17usize, 24u32, 2103600u32);
    emu.orr_no_count(14usize, 17usize, 14usize, 2103604u32);
    emu.adi_no_count(16usize, 10usize, 4294967288u32, 2103608u32);
    emu.adi_no_count(17usize, 11usize, 8u32, 2103612u32);
    emu.orr_no_count(10usize, 13usize, 12usize, 2103616u32);
    emu.orr_no_count(11usize, 14usize, 15usize, 2103620u32);
    emu.sw_no_count(17usize, 9usize, 0u32, 2103624u32)?;
    emu.sw_no_count(16usize, 9usize, 4u32, 2103628u32)?;
    emu.apc_no_count(1usize, 2103628u32, 32768u32, 2103632u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103636u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965940u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201954(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 0u32, 2103640u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2103644u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2103680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201980));
    } else {
        emu.pc = 2103648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201960));
    }
}
#[inline(always)]
pub fn block_0x00201960(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 168u32, 2103652u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2103656u32);
    emu.apc_no_count(1usize, 2103656u32, 20480u32, 2103660u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103664u32;
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
pub fn block_0x00201970(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 168u32, 2103668u32)?;
    emu.lw_no_count(12usize, 2usize, 172u32, 2103672u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2103676u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2103732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002019b4));
    } else {
        emu.pc = 2103680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201980));
    }
}
#[inline(always)]
pub fn block_0x00201980(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2103684u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 8usize, 0u32, 2103688u32)?;
    emu.sw_no_count(12usize, 8usize, 4u32, 2103692u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2103696u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2102928u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201690));
}
#[inline(always)]
pub fn block_0x00201990(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2103700u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966856u32, 2103704u32);
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2103708u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966864u32, 2103712u32);
    emu.adi_no_count(10usize, 0usize, 2u32, 2103716u32);
    emu.apc_no_count(1usize, 2103716u32, 12288u32, 2103720u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103724u32;
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
pub fn block_0x002019ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2103728u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2103732u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2103376u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201850));
}
#[inline(always)]
pub fn block_0x002019b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2103736u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a == b {
        emu.pc = 2104404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201c54));
    } else {
        emu.pc = 2103740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002019bc));
    }
}
#[inline]
pub fn block_0x002019bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 2usize, 28u32, 2103744u32)?;
    emu.sw_no_count(12usize, 2usize, 32u32, 2103748u32)?;
    emu.lw_no_count(10usize, 2usize, 176u32, 2103752u32)?;
    emu.sw_no_count(10usize, 2usize, 24u32, 2103756u32)?;
    emu.adi_no_count(10usize, 2usize, 168u32, 2103760u32);
    emu.adi_no_count(12usize, 0usize, 32u32, 2103764u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2103768u32);
    emu.apc_no_count(1usize, 2103768u32, 4294963200u32, 2103772u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103776u32;
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
pub fn block_0x002019e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 168u32, 2103780u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2102912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201680));
    } else {
        emu.pc = 2103784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002019e8));
    }
}
#[inline(never)]
pub fn block_0x002019e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 40u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(7usize, 2usize, 169u32, 2103788u32);
    emu.lbu_no_count(28usize, 2usize, 170u32, 2103792u32);
    emu.lbu_no_count(10usize, 2usize, 171u32, 2103796u32);
    emu.lw_no_count(6usize, 2usize, 172u32, 2103800u32)?;
    emu.lw_no_count(11usize, 2usize, 176u32, 2103804u32)?;
    emu.lw_no_count(12usize, 2usize, 180u32, 2103808u32)?;
    emu.lw_no_count(13usize, 2usize, 184u32, 2103812u32)?;
    emu.lw_no_count(14usize, 2usize, 188u32, 2103816u32)?;
    emu.lw_no_count(15usize, 2usize, 192u32, 2103820u32)?;
    emu.lw_no_count(16usize, 2usize, 196u32, 2103824u32)?;
    emu.lbu_no_count(17usize, 2usize, 200u32, 2103828u32);
    emu.adi_no_count(5usize, 0usize, 3u32, 2103832u32);
    emu.sw_no_count(11usize, 2usize, 136u32, 2103836u32)?;
    emu.sw_no_count(12usize, 2usize, 140u32, 2103840u32)?;
    emu.sw_no_count(13usize, 2usize, 144u32, 2103844u32)?;
    emu.sw_no_count(14usize, 2usize, 148u32, 2103848u32)?;
    emu.sw_no_count(15usize, 2usize, 152u32, 2103852u32)?;
    emu.sw_no_count(16usize, 2usize, 156u32, 2103856u32)?;
    emu.sb_no_count(17usize, 2usize, 160u32, 2103860u32);
    emu.lw_no_count(11usize, 2usize, 136u32, 2103864u32)?;
    emu.lw_no_count(12usize, 2usize, 140u32, 2103868u32)?;
    emu.lw_no_count(13usize, 2usize, 144u32, 2103872u32)?;
    emu.lw_no_count(14usize, 2usize, 148u32, 2103876u32)?;
    emu.lw_no_count(15usize, 2usize, 152u32, 2103880u32)?;
    emu.lw_no_count(16usize, 2usize, 156u32, 2103884u32)?;
    emu.lbu_no_count(17usize, 2usize, 160u32, 2103888u32);
    emu.sw_no_count(11usize, 2usize, 100u32, 2103892u32)?;
    emu.sw_no_count(12usize, 2usize, 104u32, 2103896u32)?;
    emu.sw_no_count(13usize, 2usize, 108u32, 2103900u32)?;
    emu.sw_no_count(14usize, 2usize, 112u32, 2103904u32)?;
    emu.sri_no_count(11usize, 6usize, 16u32, 2103908u32);
    emu.sw_no_count(15usize, 2usize, 116u32, 2103912u32)?;
    emu.sw_no_count(16usize, 2usize, 120u32, 2103916u32)?;
    emu.sb_no_count(17usize, 2usize, 124u32, 2103920u32);
    emu.sri_no_count(12usize, 6usize, 8u32, 2103924u32);
    emu.sb_no_count(10usize, 2usize, 128u32, 2103928u32);
    emu.sb_no_count(6usize, 2usize, 129u32, 2103932u32);
    emu.sb_no_count(12usize, 2usize, 130u32, 2103936u32);
    emu.sb_no_count(11usize, 2usize, 131u32, 2103940u32);
    emu.add_memory_rw_events(39usize);
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a == b {
        emu.pc = 2104428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201c6c));
    } else {
        emu.pc = 2103944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201a88));
    }
}
#[inline]
pub fn block_0x00201a88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(24usize, 28usize, 0u32, 2103948u32);
    emu.sw_no_count(7usize, 2usize, 12u32, 2103952u32)?;
    emu.sw_no_count(6usize, 2usize, 16u32, 2103956u32)?;
    emu.lw_no_count(10usize, 2usize, 128u32, 2103960u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2103964u32)?;
    emu.adi_no_count(10usize, 2usize, 168u32, 2103968u32);
    emu.adi_no_count(12usize, 0usize, 32u32, 2103972u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2103976u32);
    emu.apc_no_count(1usize, 2103976u32, 4294963200u32, 2103980u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103984u32;
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
pub fn block_0x00201ab0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 168u32, 2103988u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2102912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201680));
    } else {
        emu.pc = 2103992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201ab8));
    }
}
#[inline(never)]
pub fn block_0x00201ab8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 38u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(23usize, 23usize, 8u32, 2103996u32);
    emu.sli_no_count(27usize, 27usize, 8u32, 2104000u32);
    emu.sli_no_count(10usize, 24usize, 8u32, 2104004u32);
    emu.lbu_no_count(11usize, 2usize, 169u32, 2104008u32);
    emu.lbu_no_count(12usize, 2usize, 170u32, 2104012u32);
    emu.lbu_no_count(13usize, 2usize, 171u32, 2104016u32);
    emu.lw_no_count(9usize, 2usize, 172u32, 2104020u32)?;
    emu.orr_no_count(23usize, 23usize, 22usize, 2104024u32);
    emu.lw_no_count(14usize, 2usize, 176u32, 2104028u32)?;
    emu.lw_no_count(15usize, 2usize, 180u32, 2104032u32)?;
    emu.lw_no_count(16usize, 2usize, 184u32, 2104036u32)?;
    emu.lw_no_count(17usize, 2usize, 188u32, 2104040u32)?;
    emu.orr_no_count(5usize, 27usize, 26usize, 2104044u32);
    emu.sw_no_count(5usize, 2usize, 8u32, 2104048u32)?;
    emu.lw_no_count(5usize, 2usize, 12u32, 2104052u32)?;
    emu.orr_no_count(24usize, 10usize, 5usize, 2104056u32);
    emu.sb_no_count(13usize, 2usize, 128u32, 2104060u32);
    emu.lw_no_count(10usize, 2usize, 192u32, 2104064u32)?;
    emu.lw_no_count(13usize, 2usize, 196u32, 2104068u32)?;
    emu.lbu_no_count(5usize, 2usize, 200u32, 2104072u32);
    emu.sw_no_count(14usize, 2usize, 136u32, 2104076u32)?;
    emu.sw_no_count(15usize, 2usize, 140u32, 2104080u32)?;
    emu.sw_no_count(16usize, 2usize, 144u32, 2104084u32)?;
    emu.sw_no_count(17usize, 2usize, 148u32, 2104088u32)?;
    emu.sw_no_count(10usize, 2usize, 152u32, 2104092u32)?;
    emu.sw_no_count(13usize, 2usize, 156u32, 2104096u32)?;
    emu.sb_no_count(5usize, 2usize, 160u32, 2104100u32);
    emu.sri_no_count(27usize, 20usize, 24u32, 2104104u32);
    emu.sri_no_count(26usize, 25usize, 24u32, 2104108u32);
    emu.lw_no_count(10usize, 2usize, 16u32, 2104112u32)?;
    emu.sri_no_count(20usize, 10usize, 24u32, 2104116u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2104120u32);
    emu.orr_no_count(25usize, 12usize, 11usize, 2104124u32);
    emu.adi_no_count(10usize, 8usize, 127u32, 2104128u32);
    emu.adi_no_count(11usize, 2usize, 136u32, 2104132u32);
    emu.adi_no_count(12usize, 0usize, 25u32, 2104136u32);
    emu.apc_no_count(1usize, 2104136u32, 28672u32, 2104140u32);
    emu.add_memory_rw_events(38usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2104144u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(492u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00201b50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 9usize, 16u32, 2104148u32);
    emu.sri_no_count(11usize, 9usize, 8u32, 2104152u32);
    emu.sb_no_count(9usize, 2usize, 129u32, 2104156u32);
    emu.sb_no_count(11usize, 2usize, 130u32, 2104160u32);
    emu.sb_no_count(10usize, 2usize, 131u32, 2104164u32);
    emu.lw_no_count(22usize, 2usize, 128u32, 2104168u32)?;
    emu.sri_no_count(9usize, 9usize, 24u32, 2104172u32);
    emu.adi_no_count(10usize, 8usize, 31u32, 2104176u32);
    emu.adi_no_count(11usize, 2usize, 44u32, 2104180u32);
    emu.adi_no_count(12usize, 0usize, 25u32, 2104184u32);
    emu.apc_no_count(1usize, 2104184u32, 28672u32, 2104188u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2104192u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(444u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201b80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 63u32, 2104196u32);
    emu.adi_no_count(11usize, 2usize, 72u32, 2104200u32);
    emu.adi_no_count(12usize, 0usize, 25u32, 2104204u32);
    emu.apc_no_count(1usize, 2104204u32, 28672u32, 2104208u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2104212u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(424u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201b94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 95u32, 2104216u32);
    emu.adi_no_count(11usize, 2usize, 100u32, 2104220u32);
    emu.adi_no_count(12usize, 0usize, 25u32, 2104224u32);
    emu.apc_no_count(1usize, 2104224u32, 28672u32, 2104228u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2104232u32;
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
#[inline(never)]
pub fn block_0x00201ba8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 34u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(19usize, 8usize, 0u32, 2104236u32)?;
    emu.sw_no_count(18usize, 8usize, 4u32, 2104240u32)?;
    emu.lw_no_count(10usize, 2usize, 36u32, 2104244u32)?;
    emu.sw_no_count(10usize, 8usize, 8u32, 2104248u32)?;
    emu.lw_no_count(10usize, 2usize, 28u32, 2104252u32)?;
    emu.sw_no_count(10usize, 8usize, 12u32, 2104256u32)?;
    emu.sri_no_count(10usize, 21usize, 16u32, 2104260u32);
    emu.lw_no_count(11usize, 2usize, 32u32, 2104264u32)?;
    emu.sw_no_count(11usize, 8usize, 16u32, 2104268u32)?;
    emu.lw_no_count(11usize, 2usize, 24u32, 2104272u32)?;
    emu.sw_no_count(11usize, 8usize, 20u32, 2104276u32)?;
    emu.sh_no_count(23usize, 8usize, 24u32, 2104280u32)?;
    emu.sh_no_count(21usize, 8usize, 26u32, 2104284u32)?;
    emu.lw_no_count(14usize, 2usize, 40u32, 2104288u32)?;
    emu.sri_no_count(11usize, 14usize, 16u32, 2104292u32);
    emu.lw_no_count(15usize, 2usize, 20u32, 2104296u32)?;
    emu.sri_no_count(12usize, 15usize, 16u32, 2104300u32);
    emu.sri_no_count(13usize, 22usize, 16u32, 2104304u32);
    emu.sh_no_count(10usize, 8usize, 28u32, 2104308u32)?;
    emu.sb_no_count(27usize, 8usize, 30u32, 2104312u32);
    emu.lw_no_count(10usize, 2usize, 8u32, 2104316u32)?;
    emu.sh_no_count(10usize, 8usize, 56u32, 2104320u32)?;
    emu.sh_no_count(14usize, 8usize, 58u32, 2104324u32)?;
    emu.sh_no_count(11usize, 8usize, 60u32, 2104328u32)?;
    emu.sb_no_count(26usize, 8usize, 62u32, 2104332u32);
    emu.sh_no_count(24usize, 8usize, 88u32, 2104336u32)?;
    emu.sh_no_count(15usize, 8usize, 90u32, 2104340u32)?;
    emu.sh_no_count(12usize, 8usize, 92u32, 2104344u32)?;
    emu.sb_no_count(20usize, 8usize, 94u32, 2104348u32);
    emu.sh_no_count(25usize, 8usize, 120u32, 2104352u32)?;
    emu.sh_no_count(22usize, 8usize, 122u32, 2104356u32)?;
    emu.sh_no_count(13usize, 8usize, 124u32, 2104360u32)?;
    emu.sb_no_count(9usize, 8usize, 126u32, 2104364u32);
    emu.add_memory_rw_events(34usize);
    let return_addr = 2104368u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2102928u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201690));
}
#[inline(always)]
pub fn block_0x00201c30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2104372u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966856u32, 2104376u32);
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2104380u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966864u32, 2104384u32);
    emu.adi_no_count(10usize, 0usize, 3u32, 2104388u32);
    emu.apc_no_count(1usize, 2104388u32, 8192u32, 2104392u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2104396u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1580u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201c4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2104400u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2104404u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2103680u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201980));
}
#[inline(always)]
pub fn block_0x00201c54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2104408u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966856u32, 2104412u32);
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2104416u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966864u32, 2104420u32);
    emu.adi_no_count(10usize, 0usize, 4u32, 2104424u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2104428u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2103516u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002018dc));
}
#[inline(always)]
pub fn block_0x00201c6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2104432u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966856u32, 2104436u32);
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2104440u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966864u32, 2104444u32);
    emu.adi_no_count(10usize, 0usize, 5u32, 2104448u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2104452u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2103516u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002018dc));
}
#[inline]
pub fn block_0x00201c84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967136u32, 2104456u32);
    emu.sw_no_count(1usize, 2usize, 156u32, 2104460u32)?;
    emu.sw_no_count(8usize, 2usize, 152u32, 2104464u32)?;
    emu.sw_no_count(9usize, 2usize, 148u32, 2104468u32)?;
    emu.sw_no_count(18usize, 2usize, 144u32, 2104472u32)?;
    emu.sw_no_count(19usize, 2usize, 140u32, 2104476u32)?;
    emu.sw_no_count(20usize, 2usize, 136u32, 2104480u32)?;
    emu.sw_no_count(21usize, 2usize, 132u32, 2104484u32)?;
    emu.sw_no_count(22usize, 2usize, 128u32, 2104488u32)?;
    emu.sw_no_count(23usize, 2usize, 124u32, 2104492u32)?;
    emu.sw_no_count(24usize, 2usize, 120u32, 2104496u32)?;
    emu.sw_no_count(25usize, 2usize, 116u32, 2104500u32)?;
    emu.sw_no_count(26usize, 2usize, 112u32, 2104504u32)?;
    emu.sw_no_count(27usize, 2usize, 108u32, 2104508u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2104512u32);
    emu.add_memory_rw_events(15usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2104776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201dc8));
    } else {
        emu.pc = 2104516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201cc4));
    }
}
#[inline(always)]
pub fn block_0x00201cc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 15usize, 0u32, 2104520u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2104524u32);
    emu.adi_no_count(10usize, 2usize, 72u32, 2104528u32);
    emu.apc_no_count(1usize, 2104528u32, 4294963200u32, 2104532u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2104536u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1580u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201cd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(20usize, 2usize, 72u32, 2104540u32)?;
    emu.lw_no_count(19usize, 2usize, 76u32, 2104544u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2104548u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a != b {
        emu.pc = 2104564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201cf4));
    } else {
        emu.pc = 2104552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201ce8));
    }
}
#[inline(always)]
pub fn block_0x00201ce8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(19usize, 8usize, 0u32, 2104556u32)?;
    emu.sw_no_count(10usize, 8usize, 48u32, 2104560u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2104564u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2105340u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201ffc));
}
#[inline(always)]
pub fn block_0x00201cf4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2104568u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2104912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201e50));
    } else {
        emu.pc = 2104572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201cfc));
    }
}
#[inline(always)]
pub fn block_0x00201cfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 4u32, 2104576u32)?;
    emu.adi_no_count(11usize, 0usize, 8u32, 2104580u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2104604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201d1c));
    } else {
        emu.pc = 2104584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201d08));
    }
}
#[inline(always)]
pub fn block_0x00201d08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2104588u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1281u32, 2104592u32);
    emu.sw_no_count(10usize, 2usize, 72u32, 2104596u32)?;
    emu.sw_no_count(11usize, 2usize, 76u32, 2104600u32)?;
    emu.add_memory_rw_events(5usize);
    let return_addr = 2104604u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2104760u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201db8));
}
#[inline]
pub fn block_0x00201d1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 9usize, 0u32, 2104608u32)?;
    emu.lw_no_count(15usize, 2usize, 80u32, 2104612u32)?;
    emu.adi_no_count(12usize, 10usize, 4294967288u32, 2104616u32);
    emu.adi_no_count(13usize, 11usize, 8u32, 2104620u32);
    emu.lbu_no_count(16usize, 11usize, 0u32, 2104624u32);
    emu.lbu_no_count(21usize, 11usize, 1u32, 2104628u32);
    emu.lbu_no_count(25usize, 11usize, 2u32, 2104632u32);
    emu.lbu_no_count(26usize, 11usize, 3u32, 2104636u32);
    emu.lbu_no_count(23usize, 11usize, 4u32, 2104640u32);
    emu.lbu_no_count(22usize, 11usize, 5u32, 2104644u32);
    emu.lbu_no_count(24usize, 11usize, 6u32, 2104648u32);
    emu.lbu_no_count(27usize, 11usize, 7u32, 2104652u32);
    emu.adi_no_count(14usize, 0usize, 2u32, 2104656u32);
    emu.sw_no_count(13usize, 9usize, 0u32, 2104660u32)?;
    emu.sw_no_count(12usize, 9usize, 4u32, 2104664u32)?;
    emu.add_memory_rw_events(15usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2105252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201fa4));
    } else {
        emu.pc = 2104668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201d5c));
    }
}
#[inline(always)]
pub fn block_0x00201d5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 4u32, 2104672u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a >= b {
        emu.pc = 2104696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201d78));
    } else {
        emu.pc = 2104676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201d64));
    }
}
#[inline(always)]
pub fn block_0x00201d64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2104680u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1281u32, 2104684u32);
    emu.sw_no_count(10usize, 2usize, 72u32, 2104688u32)?;
    emu.sw_no_count(13usize, 2usize, 76u32, 2104692u32)?;
    emu.add_memory_rw_events(5usize);
    let return_addr = 2104696u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2104760u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201db8));
}
#[inline]
pub fn block_0x00201d78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 12u32, 2104700u32);
    emu.adi_no_count(13usize, 10usize, 4294967284u32, 2104704u32);
    emu.lbu_no_count(17usize, 11usize, 8u32, 2104708u32);
    emu.lbu_no_count(7usize, 11usize, 9u32, 2104712u32);
    emu.lbu_no_count(5usize, 11usize, 10u32, 2104716u32);
    emu.lbu_no_count(6usize, 11usize, 11u32, 2104720u32);
    emu.adi_no_count(14usize, 0usize, 3u32, 2104724u32);
    emu.sw_no_count(12usize, 9usize, 0u32, 2104728u32)?;
    emu.sw_no_count(13usize, 9usize, 4u32, 2104732u32)?;
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2105276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201fbc));
    } else {
        emu.pc = 2104736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201da0));
    }
}
#[inline(always)]
pub fn block_0x00201da0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 4u32, 2104740u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2104800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201de0));
    } else {
        emu.pc = 2104744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201da8));
    }
}
#[inline(always)]
pub fn block_0x00201da8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2104748u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1281u32, 2104752u32);
    emu.sw_no_count(10usize, 2usize, 72u32, 2104756u32)?;
    emu.sw_no_count(12usize, 2usize, 76u32, 2104760u32)?;
    emu.add_memory_rw_events(4usize);
    emu.pc = 2104760u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201db8));
}
#[inline(always)]
pub fn block_0x00201db8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 72u32, 2104764u32);
    emu.apc_no_count(1usize, 2104764u32, 28672u32, 2104768u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2104772u32;
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
pub fn block_0x00201dc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2104776u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2105328u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201ff0));
}
#[inline(always)]
pub fn block_0x00201dc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2104780u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966916u32, 2104784u32);
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2104788u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966864u32, 2104792u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2104796u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2104800u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2105320u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201fe8));
}
#[inline]
pub fn block_0x00201de0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 11usize, 16u32, 2104804u32);
    emu.lbu_no_count(14usize, 11usize, 12u32, 2104808u32);
    emu.lbu_no_count(29usize, 11usize, 13u32, 2104812u32);
    emu.lbu_no_count(28usize, 11usize, 14u32, 2104816u32);
    emu.lbu_no_count(11usize, 11usize, 15u32, 2104820u32);
    emu.adi_no_count(10usize, 10usize, 4294967280u32, 2104824u32);
    emu.sw_no_count(13usize, 9usize, 0u32, 2104828u32)?;
    emu.sw_no_count(10usize, 9usize, 4u32, 2104832u32)?;
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2105300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201fd4));
    } else {
        emu.pc = 2104836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201e04));
    }
}
#[inline]
pub fn block_0x00201e04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(29usize, 2usize, 0u32, 2104840u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2104844u32)?;
    emu.sw_no_count(28usize, 2usize, 8u32, 2104848u32)?;
    emu.adi_no_count(18usize, 7usize, 0u32, 2104852u32);
    emu.sw_no_count(6usize, 2usize, 16u32, 2104856u32)?;
    emu.sw_no_count(5usize, 2usize, 12u32, 2104860u32)?;
    emu.sw_no_count(14usize, 2usize, 24u32, 2104864u32)?;
    emu.sw_no_count(17usize, 2usize, 28u32, 2104868u32)?;
    emu.sw_no_count(16usize, 2usize, 20u32, 2104872u32)?;
    emu.sw_no_count(15usize, 2usize, 32u32, 2104876u32)?;
    emu.adi_no_count(10usize, 2usize, 72u32, 2104880u32);
    emu.adi_no_count(12usize, 0usize, 32u32, 2104884u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2104888u32);
    emu.apc_no_count(1usize, 2104888u32, 4294963200u32, 2104892u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2104896u32;
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
pub fn block_0x00201e40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 72u32, 2104900u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2104936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201e68));
    } else {
        emu.pc = 2104904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201e48));
    }
}
#[inline(always)]
pub fn block_0x00201e48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 76u32, 2104908u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2104912u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2105328u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201ff0));
}
#[inline(always)]
pub fn block_0x00201e50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2104916u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966916u32, 2104920u32);
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2104924u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966864u32, 2104928u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2104932u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2104936u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2105320u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201fe8));
}
#[inline(never)]
pub fn block_0x00201e68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 58u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(21usize, 21usize, 8u32, 2104940u32);
    emu.sli_no_count(25usize, 25usize, 16u32, 2104944u32);
    emu.sli_no_count(26usize, 26usize, 24u32, 2104948u32);
    emu.sli_no_count(22usize, 22usize, 8u32, 2104952u32);
    emu.sli_no_count(24usize, 24usize, 16u32, 2104956u32);
    emu.sli_no_count(27usize, 27usize, 24u32, 2104960u32);
    emu.sli_no_count(10usize, 18usize, 8u32, 2104964u32);
    emu.lw_no_count(11usize, 2usize, 12u32, 2104968u32)?;
    emu.sli_no_count(11usize, 11usize, 16u32, 2104972u32);
    emu.lw_no_count(12usize, 2usize, 16u32, 2104976u32)?;
    emu.sli_no_count(12usize, 12usize, 24u32, 2104980u32);
    emu.lw_no_count(13usize, 2usize, 0u32, 2104984u32)?;
    emu.sli_no_count(13usize, 13usize, 8u32, 2104988u32);
    emu.lw_no_count(14usize, 2usize, 8u32, 2104992u32)?;
    emu.sli_no_count(14usize, 14usize, 16u32, 2104996u32);
    emu.lw_no_count(15usize, 2usize, 4u32, 2105000u32)?;
    emu.sli_no_count(15usize, 15usize, 24u32, 2105004u32);
    emu.lw_no_count(16usize, 2usize, 20u32, 2105008u32)?;
    emu.orr_no_count(16usize, 21usize, 16usize, 2105012u32);
    emu.orr_no_count(17usize, 26usize, 25usize, 2105016u32);
    emu.orr_no_count(5usize, 22usize, 23usize, 2105020u32);
    emu.orr_no_count(6usize, 27usize, 24usize, 2105024u32);
    emu.lbu_no_count(7usize, 2usize, 73u32, 2105028u32);
    emu.lbu_no_count(28usize, 2usize, 74u32, 2105032u32);
    emu.lbu_no_count(29usize, 2usize, 75u32, 2105036u32);
    emu.lw_no_count(9usize, 2usize, 76u32, 2105040u32)?;
    emu.lw_no_count(30usize, 2usize, 28u32, 2105044u32)?;
    emu.orr_no_count(10usize, 10usize, 30usize, 2105048u32);
    emu.orr_no_count(11usize, 12usize, 11usize, 2105052u32);
    emu.lw_no_count(12usize, 2usize, 24u32, 2105056u32)?;
    emu.orr_no_count(12usize, 13usize, 12usize, 2105060u32);
    emu.orr_no_count(14usize, 15usize, 14usize, 2105064u32);
    emu.lw_no_count(13usize, 2usize, 80u32, 2105068u32)?;
    emu.lw_no_count(15usize, 2usize, 84u32, 2105072u32)?;
    emu.lw_no_count(30usize, 2usize, 88u32, 2105076u32)?;
    emu.lw_no_count(31usize, 2usize, 92u32, 2105080u32)?;
    emu.orr_no_count(18usize, 17usize, 16usize, 2105084u32);
    emu.orr_no_count(21usize, 6usize, 5usize, 2105088u32);
    emu.orr_no_count(22usize, 11usize, 10usize, 2105092u32);
    emu.lw_no_count(10usize, 2usize, 96u32, 2105096u32)?;
    emu.lw_no_count(11usize, 2usize, 100u32, 2105100u32)?;
    emu.lbu_no_count(16usize, 2usize, 104u32, 2105104u32);
    emu.orr_no_count(23usize, 14usize, 12usize, 2105108u32);
    emu.sli_no_count(28usize, 28usize, 8u32, 2105112u32);
    emu.sb_no_count(29usize, 2usize, 36u32, 2105116u32);
    emu.sw_no_count(13usize, 2usize, 44u32, 2105120u32)?;
    emu.sw_no_count(15usize, 2usize, 48u32, 2105124u32)?;
    emu.sw_no_count(30usize, 2usize, 52u32, 2105128u32)?;
    emu.sw_no_count(31usize, 2usize, 56u32, 2105132u32)?;
    emu.sw_no_count(10usize, 2usize, 60u32, 2105136u32)?;
    emu.sw_no_count(11usize, 2usize, 64u32, 2105140u32)?;
    emu.sb_no_count(16usize, 2usize, 68u32, 2105144u32);
    emu.orr_no_count(24usize, 28usize, 7usize, 2105148u32);
    emu.adi_no_count(10usize, 8usize, 7u32, 2105152u32);
    emu.adi_no_count(11usize, 2usize, 44u32, 2105156u32);
    emu.adi_no_count(12usize, 0usize, 25u32, 2105160u32);
    emu.apc_no_count(1usize, 2105160u32, 28672u32, 2105164u32);
    emu.add_memory_rw_events(58usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2105168u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966764u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00201f50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 9usize, 16u32, 2105172u32);
    emu.sri_no_count(11usize, 9usize, 8u32, 2105176u32);
    emu.sri_no_count(12usize, 9usize, 24u32, 2105180u32);
    emu.sb_no_count(9usize, 2usize, 37u32, 2105184u32);
    emu.sb_no_count(11usize, 2usize, 38u32, 2105188u32);
    emu.sb_no_count(10usize, 2usize, 39u32, 2105192u32);
    emu.lw_no_count(10usize, 2usize, 36u32, 2105196u32)?;
    emu.sw_no_count(20usize, 8usize, 48u32, 2105200u32)?;
    emu.sw_no_count(19usize, 8usize, 52u32, 2105204u32)?;
    emu.lw_no_count(11usize, 2usize, 32u32, 2105208u32)?;
    emu.sw_no_count(11usize, 8usize, 56u32, 2105212u32)?;
    emu.sri_no_count(11usize, 10usize, 16u32, 2105216u32);
    emu.sh_no_count(24usize, 8usize, 0u32, 2105220u32)?;
    emu.sh_no_count(10usize, 8usize, 2u32, 2105224u32)?;
    emu.sh_no_count(11usize, 8usize, 4u32, 2105228u32)?;
    emu.sb_no_count(12usize, 8usize, 6u32, 2105232u32);
    emu.sw_no_count(18usize, 8usize, 32u32, 2105236u32)?;
    emu.sw_no_count(21usize, 8usize, 36u32, 2105240u32)?;
    emu.sw_no_count(22usize, 8usize, 40u32, 2105244u32)?;
    emu.sw_no_count(23usize, 8usize, 44u32, 2105248u32)?;
    emu.add_memory_rw_events(21usize);
    let return_addr = 2105252u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2105340u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201ffc));
}
#[inline(always)]
pub fn block_0x00201fa4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2105256u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966916u32, 2105260u32);
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2105264u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966864u32, 2105268u32);
    emu.adi_no_count(10usize, 0usize, 2u32, 2105272u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2105276u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2105320u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201fe8));
}
#[inline(always)]
pub fn block_0x00201fbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2105280u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966916u32, 2105284u32);
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2105288u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966864u32, 2105292u32);
    emu.adi_no_count(10usize, 0usize, 3u32, 2105296u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2105300u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2105320u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201fe8));
}
#[inline(always)]
pub fn block_0x00201fd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2105304u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966916u32, 2105308u32);
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2105312u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966864u32, 2105316u32);
    emu.adi_no_count(10usize, 0usize, 4u32, 2105320u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2105320u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201fe8));
}
#[inline(always)]
pub fn block_0x00201fe8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2105320u32, 8192u32, 2105324u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2105328u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(648u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201ff0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2105332u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 8usize, 0u32, 2105336u32)?;
    emu.sw_no_count(11usize, 8usize, 48u32, 2105340u32)?;
    emu.add_memory_rw_events(3usize);
    emu.pc = 2105340u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201ffc));
}
#[inline]
pub fn block_0x00201ffc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 156u32, 2105344u32)?;
    emu.lw_no_count(8usize, 2usize, 152u32, 2105348u32)?;
    emu.lw_no_count(9usize, 2usize, 148u32, 2105352u32)?;
    emu.lw_no_count(18usize, 2usize, 144u32, 2105356u32)?;
    emu.lw_no_count(19usize, 2usize, 140u32, 2105360u32)?;
    emu.lw_no_count(20usize, 2usize, 136u32, 2105364u32)?;
    emu.lw_no_count(21usize, 2usize, 132u32, 2105368u32)?;
    emu.lw_no_count(22usize, 2usize, 128u32, 2105372u32)?;
    emu.lw_no_count(23usize, 2usize, 124u32, 2105376u32)?;
    emu.lw_no_count(24usize, 2usize, 120u32, 2105380u32)?;
    emu.lw_no_count(25usize, 2usize, 116u32, 2105384u32)?;
    emu.lw_no_count(26usize, 2usize, 112u32, 2105388u32)?;
    emu.lw_no_count(27usize, 2usize, 108u32, 2105392u32)?;
    emu.adi_no_count(2usize, 2usize, 160u32, 2105396u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2105400u32;
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
