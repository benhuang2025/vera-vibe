pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2157364u32;
pub const PC_MAX: u32 = 2160452u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 171usize] = [
        block_0x0020eb34,
        block_0x0020eb38,
        block_0x0020eb40,
        block_0x0020eb4c,
        block_0x0020eb58,
        block_0x0020eb5c,
        block_0x0020eb64,
        block_0x0020eb68,
        block_0x0020eb78,
        block_0x0020eb8c,
        block_0x0020eb94,
        block_0x0020eb98,
        block_0x0020eba0,
        block_0x0020ebbc,
        block_0x0020ebc4,
        block_0x0020ebcc,
        block_0x0020ebd8,
        block_0x0020ebf0,
        block_0x0020ebf8,
        block_0x0020ebfc,
        block_0x0020ec14,
        block_0x0020ec1c,
        block_0x0020ec20,
        block_0x0020ec30,
        block_0x0020ec34,
        block_0x0020ec40,
        block_0x0020ec44,
        block_0x0020ec58,
        block_0x0020ec60,
        block_0x0020ec64,
        block_0x0020ec6c,
        block_0x0020ec74,
        block_0x0020ec84,
        block_0x0020ec98,
        block_0x0020eca0,
        block_0x0020eca4,
        block_0x0020ecac,
        block_0x0020ecc8,
        block_0x0020ecd0,
        block_0x0020ecd8,
        block_0x0020ece4,
        block_0x0020ecfc,
        block_0x0020ed04,
        block_0x0020ed08,
        block_0x0020ed20,
        block_0x0020ed28,
        block_0x0020ed2c,
        block_0x0020ed3c,
        block_0x0020ed40,
        block_0x0020ed4c,
        block_0x0020ed50,
        block_0x0020ed64,
        block_0x0020ed6c,
        block_0x0020ed70,
        block_0x0020ed78,
        block_0x0020ed94,
        block_0x0020ed9c,
        block_0x0020eda4,
        block_0x0020edb4,
        block_0x0020edc8,
        block_0x0020edd0,
        block_0x0020ede8,
        block_0x0020edf0,
        block_0x0020ee0c,
        block_0x0020ee14,
        block_0x0020ee1c,
        block_0x0020ee30,
        block_0x0020ee38,
        block_0x0020ee3c,
        block_0x0020ee48,
        block_0x0020ee74,
        block_0x0020ee7c,
        block_0x0020ee90,
        block_0x0020ee98,
        block_0x0020eea0,
        block_0x0020eeb0,
        block_0x0020eeb4,
        block_0x0020eec4,
        block_0x0020eec8,
        block_0x0020eed0,
        block_0x0020eed4,
        block_0x0020ef10,
        block_0x0020ef34,
        block_0x0020ef50,
        block_0x0020ef6c,
        block_0x0020ef88,
        block_0x0020efa4,
        block_0x0020efc0,
        block_0x0020efdc,
        block_0x0020eff8,
        block_0x0020f010,
        block_0x0020f028,
        block_0x0020f040,
        block_0x0020f088,
        block_0x0020f090,
        block_0x0020f094,
        block_0x0020f0c0,
        block_0x0020f120,
        block_0x0020f17c,
        block_0x0020f1b8,
        block_0x0020f1c4,
        block_0x0020f1cc,
        block_0x0020f1dc,
        block_0x0020f27c,
        block_0x0020f284,
        block_0x0020f294,
        block_0x0020f2d4,
        block_0x0020f2e0,
        block_0x0020f2e8,
        block_0x0020f2f4,
        block_0x0020f310,
        block_0x0020f314,
        block_0x0020f31c,
        block_0x0020f334,
        block_0x0020f33c,
        block_0x0020f348,
        block_0x0020f358,
        block_0x0020f360,
        block_0x0020f378,
        block_0x0020f384,
        block_0x0020f38c,
        block_0x0020f398,
        block_0x0020f39c,
        block_0x0020f3b4,
        block_0x0020f41c,
        block_0x0020f424,
        block_0x0020f430,
        block_0x0020f440,
        block_0x0020f448,
        block_0x0020f460,
        block_0x0020f464,
        block_0x0020f46c,
        block_0x0020f488,
        block_0x0020f490,
        block_0x0020f4b0,
        block_0x0020f4b8,
        block_0x0020f4cc,
        block_0x0020f4dc,
        block_0x0020f4e4,
        block_0x0020f4ec,
        block_0x0020f4f4,
        block_0x0020f50c,
        block_0x0020f534,
        block_0x0020f538,
        block_0x0020f564,
        block_0x0020f56c,
        block_0x0020f57c,
        block_0x0020f580,
        block_0x0020f59c,
        block_0x0020f5a0,
        block_0x0020f5b4,
        block_0x0020f5c8,
        block_0x0020f5d0,
        block_0x0020f5f0,
        block_0x0020f5f8,
        block_0x0020f600,
        block_0x0020f614,
        block_0x0020f630,
        block_0x0020f634,
        block_0x0020f644,
        block_0x0020f648,
        block_0x0020f660,
        block_0x0020f664,
        block_0x0020f680,
        block_0x0020f688,
        block_0x0020f6c0,
        block_0x0020f6dc,
        block_0x0020f6f8,
        block_0x0020f714,
        block_0x0020f72c,
        block_0x0020f744,
    ];
    const IDX: [u16; 773usize] = [
        1u16, 2u16, 0u16, 3u16, 0u16, 0u16, 4u16, 0u16, 0u16, 5u16, 6u16, 0u16, 7u16,
        8u16, 0u16, 0u16, 0u16, 9u16, 0u16, 0u16, 0u16, 0u16, 10u16, 0u16, 11u16, 12u16,
        0u16, 13u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 14u16, 0u16, 15u16, 0u16, 16u16,
        0u16, 0u16, 17u16, 0u16, 0u16, 0u16, 0u16, 0u16, 18u16, 0u16, 19u16, 20u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 21u16, 0u16, 22u16, 23u16, 0u16, 0u16, 0u16, 24u16,
        25u16, 0u16, 0u16, 26u16, 27u16, 0u16, 0u16, 0u16, 0u16, 28u16, 0u16, 29u16,
        30u16, 0u16, 31u16, 0u16, 32u16, 0u16, 0u16, 0u16, 33u16, 0u16, 0u16, 0u16, 0u16,
        34u16, 0u16, 35u16, 36u16, 0u16, 37u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        38u16, 0u16, 39u16, 0u16, 40u16, 0u16, 0u16, 41u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        42u16, 0u16, 43u16, 44u16, 0u16, 0u16, 0u16, 0u16, 0u16, 45u16, 0u16, 46u16,
        47u16, 0u16, 0u16, 0u16, 48u16, 49u16, 0u16, 0u16, 50u16, 51u16, 0u16, 0u16,
        0u16, 0u16, 52u16, 0u16, 53u16, 54u16, 0u16, 55u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 56u16, 0u16, 57u16, 0u16, 58u16, 0u16, 0u16, 0u16, 59u16, 0u16, 0u16, 0u16,
        0u16, 60u16, 0u16, 61u16, 0u16, 0u16, 0u16, 0u16, 0u16, 62u16, 0u16, 63u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 64u16, 0u16, 65u16, 0u16, 66u16, 0u16, 0u16, 0u16,
        0u16, 67u16, 0u16, 68u16, 69u16, 0u16, 0u16, 70u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 71u16, 0u16, 72u16, 0u16, 0u16, 0u16, 0u16, 73u16,
        0u16, 74u16, 0u16, 75u16, 0u16, 0u16, 0u16, 76u16, 77u16, 0u16, 0u16, 0u16,
        78u16, 79u16, 0u16, 80u16, 81u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 82u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 83u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 84u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 85u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 86u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 87u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 88u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 89u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 90u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 91u16, 0u16, 0u16, 0u16, 0u16, 0u16, 92u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 93u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 94u16, 0u16, 95u16, 96u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 97u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 98u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 99u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 100u16, 0u16, 0u16, 101u16, 0u16,
        102u16, 0u16, 0u16, 0u16, 103u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 104u16, 0u16, 105u16, 0u16, 0u16, 0u16, 106u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 107u16, 0u16, 0u16, 108u16, 0u16, 109u16, 0u16, 0u16, 110u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 111u16, 112u16, 0u16, 113u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 114u16, 0u16, 115u16, 0u16, 0u16, 116u16, 0u16, 0u16, 0u16, 117u16,
        0u16, 118u16, 0u16, 0u16, 0u16, 0u16, 0u16, 119u16, 0u16, 0u16, 120u16, 0u16,
        121u16, 0u16, 0u16, 122u16, 123u16, 0u16, 0u16, 0u16, 0u16, 0u16, 124u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 125u16, 0u16,
        126u16, 0u16, 0u16, 127u16, 0u16, 0u16, 0u16, 128u16, 0u16, 129u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 130u16, 131u16, 0u16, 132u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 133u16, 0u16, 134u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 135u16,
        0u16, 136u16, 0u16, 0u16, 0u16, 0u16, 137u16, 0u16, 0u16, 0u16, 138u16, 0u16,
        139u16, 0u16, 140u16, 0u16, 141u16, 0u16, 0u16, 0u16, 0u16, 0u16, 142u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 143u16, 144u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 145u16, 0u16, 146u16, 0u16, 0u16, 0u16,
        147u16, 148u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 149u16, 150u16, 0u16, 0u16,
        0u16, 0u16, 151u16, 0u16, 0u16, 0u16, 0u16, 152u16, 0u16, 153u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 154u16, 0u16, 155u16, 0u16, 156u16, 0u16, 0u16,
        0u16, 0u16, 157u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 158u16, 159u16, 0u16,
        0u16, 0u16, 160u16, 161u16, 0u16, 0u16, 0u16, 0u16, 0u16, 162u16, 163u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 164u16, 0u16, 165u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 166u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 167u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 168u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 169u16, 0u16, 0u16, 0u16, 0u16, 0u16, 170u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 171u16,
    ];
    if pc < 2157364u32 || pc > 2160452u32 {
        return None;
    }
    let word_offset = ((pc - 2157364u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x0020eb34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2157668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec64));
    } else {
        emu.pc = 2157368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb38));
    }
}
#[inline(always)]
pub fn block_0x0020eb38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 14usize, 15usize, 2157372u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2157676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec6c));
    } else {
        emu.pc = 2157376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb40));
    }
}
#[inline(always)]
pub fn block_0x0020eb40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(6usize, 21usize, 0u32, 2157380u32);
    emu.adi_no_count(7usize, 18usize, 0u32, 2157384u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a != b {
        emu.pc = 2157888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed40));
    } else {
        emu.pc = 2157388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb4c));
    }
}
#[inline(always)]
pub fn block_0x0020eb4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 6usize, 13usize, 2157392u32);
    emu.lw_no_count(29usize, 2usize, 16u32, 2157396u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2157900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed4c));
    } else {
        emu.pc = 2157400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb58));
    }
}
#[inline(always)]
pub fn block_0x0020eb58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2157404u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2158108u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ee1c));
}
#[inline(always)]
pub fn block_0x0020eb5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 5usize, 6usize, 2157408u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2157432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb78));
    } else {
        emu.pc = 2157412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb64));
    }
}
#[inline(always)]
pub fn block_0x0020eb64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2157416u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157276u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(2157276u32));
}
#[inline(always)]
pub fn block_0x0020eb68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 29usize, 15usize, 2157420u32);
    emu.adi_no_count(26usize, 16usize, 0u32, 2157424u32);
    emu.adi_no_count(27usize, 5usize, 0u32, 2157428u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2157616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec30));
    } else {
        emu.pc = 2157432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb78));
    }
}
#[inline(always)]
pub fn block_0x0020eb78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(16usize, 26usize, 6usize, 2157436u32);
    emu.sltru_no_count(5usize, 16usize, 26usize, 2157440u32);
    emu.adr_no_count(10usize, 27usize, 15usize, 2157444u32);
    emu.adr_no_count(5usize, 10usize, 5usize, 2157448u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2157464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb98));
    } else {
        emu.pc = 2157452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb8c));
    }
}
#[inline(always)]
pub fn block_0x0020eb8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 5usize, 7usize, 2157456u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2157472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eba0));
    } else {
        emu.pc = 2157460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb94));
    }
}
#[inline(always)]
pub fn block_0x0020eb94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2157464u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157516u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ebcc));
}
#[inline(always)]
pub fn block_0x0020eb98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 16usize, 28usize, 2157468u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2157516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ebcc));
    } else {
        emu.pc = 2157472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eba0));
    }
}
#[inline(always)]
pub fn block_0x0020eba0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 28usize, 26usize, 2157476u32);
    emu.sbr_no_count(12usize, 7usize, 27usize, 2157480u32);
    emu.sltru_no_count(14usize, 16usize, 28usize, 2157484u32);
    emu.sbr_no_count(10usize, 12usize, 10usize, 2157488u32);
    emu.sbr_no_count(12usize, 5usize, 7usize, 2157492u32);
    emu.sbr_no_count(12usize, 12usize, 14usize, 2157496u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2157508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ebc4));
    } else {
        emu.pc = 2157500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ebbc));
    }
}
#[inline(always)]
pub fn block_0x0020ebbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 28usize, 26usize, 2157504u32);
    emu.sbr_no_count(12usize, 16usize, 28usize, 2157508u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2157508u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ebc4));
}
#[inline(always)]
pub fn block_0x0020ebc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 10usize, 12usize, 2157512u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2158240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eea0));
    } else {
        emu.pc = 2157516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ebcc));
    }
}
#[inline(always)]
pub fn block_0x0020ebcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(30usize, 30usize, 4294967295u32, 2157520u32);
    emu.sb_no_count(30usize, 23usize, 0u32, 2157524u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2157564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ebfc));
    } else {
        emu.pc = 2157528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ebd8));
    }
}
#[inline(always)]
pub fn block_0x0020ebd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 5usize, 7usize, 2157532u32);
    emu.sltru_no_count(12usize, 8usize, 16usize, 2157536u32);
    emu.sbr_no_count(14usize, 9usize, 5usize, 2157540u32);
    emu.sbr_no_count(29usize, 14usize, 12usize, 2157544u32);
    emu.sbr_no_count(31usize, 8usize, 16usize, 2157548u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(29usize);
    if a != b {
        emu.pc = 2157588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec14));
    } else {
        emu.pc = 2157552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ebf0));
    }
}
#[inline(always)]
pub fn block_0x0020ebf0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(14usize, 31usize, 6usize, 2157556u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2157596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec1c));
    } else {
        emu.pc = 2157560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ebf8));
    }
}
#[inline(always)]
pub fn block_0x0020ebf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2157564u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157616u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ec30));
}
#[inline(always)]
pub fn block_0x0020ebfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 16usize, 28usize, 2157568u32);
    emu.sltru_no_count(12usize, 8usize, 16usize, 2157572u32);
    emu.sbr_no_count(14usize, 9usize, 5usize, 2157576u32);
    emu.sbr_no_count(29usize, 14usize, 12usize, 2157580u32);
    emu.sbr_no_count(31usize, 8usize, 16usize, 2157584u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(29usize);
    if a == b {
        emu.pc = 2157552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ebf0));
    } else {
        emu.pc = 2157588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec14));
    }
}
#[inline(always)]
pub fn block_0x0020ec14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(14usize, 29usize, 15usize, 2157592u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2157616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec30));
    } else {
        emu.pc = 2157596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec1c));
    }
}
#[inline(always)]
pub fn block_0x0020ec1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(29usize);
    if a != b {
        emu.pc = 2157416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb68));
    } else {
        emu.pc = 2157600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec20));
    }
}
#[inline(always)]
pub fn block_0x0020ec20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 31usize, 6usize, 2157604u32);
    emu.adi_no_count(26usize, 16usize, 0u32, 2157608u32);
    emu.adi_no_count(27usize, 5usize, 0u32, 2157612u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2157432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb78));
    } else {
        emu.pc = 2157616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec30));
    }
}
#[inline(always)]
pub fn block_0x0020ec30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2157288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2157288u32));
    } else {
        emu.pc = 2157620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec34));
    }
}
#[inline(always)]
pub fn block_0x0020ec34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 5usize, 17usize, 2157624u32);
    emu.lw_no_count(7usize, 2usize, 20u32, 2157628u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2157988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eda4));
    } else {
        emu.pc = 2157632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec40));
    }
}
#[inline(always)]
pub fn block_0x0020ec40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2157988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eda4));
    } else {
        emu.pc = 2157636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec44));
    }
}
#[inline(always)]
pub fn block_0x0020ec44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(6usize, 16usize, 6usize, 2157640u32);
    emu.sltru_no_count(10usize, 6usize, 16usize, 2157644u32);
    emu.adr_no_count(15usize, 5usize, 15usize, 2157648u32);
    emu.adr_no_count(10usize, 15usize, 10usize, 2157652u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2157936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed70));
    } else {
        emu.pc = 2157656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec58));
    }
}
#[inline(always)]
pub fn block_0x0020ec58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 10usize, 17usize, 2157660u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2157944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed78));
    } else {
        emu.pc = 2157664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec60));
    }
}
#[inline(always)]
pub fn block_0x0020ec60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2157668u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2158232u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ee98));
}
#[inline(always)]
pub fn block_0x0020ec64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 6usize, 16usize, 2157672u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2157376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb40));
    } else {
        emu.pc = 2157676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec6c));
    }
}
#[inline(always)]
pub fn block_0x0020ec6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(14usize, 23usize, 11usize, 2157680u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2157684u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157700u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ec84));
}
#[inline(always)]
pub fn block_0x0020ec74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 18usize, 15usize, 2157688u32);
    emu.adi_no_count(21usize, 6usize, 0u32, 2157692u32);
    emu.adi_no_count(18usize, 7usize, 0u32, 2157696u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2157884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed3c));
    } else {
        emu.pc = 2157700u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec84));
    }
}
#[inline(always)]
pub fn block_0x0020ec84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(6usize, 21usize, 16usize, 2157704u32);
    emu.sltru_no_count(7usize, 6usize, 21usize, 2157708u32);
    emu.adr_no_count(10usize, 18usize, 15usize, 2157712u32);
    emu.adr_no_count(7usize, 10usize, 7usize, 2157716u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2157732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eca4));
    } else {
        emu.pc = 2157720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec98));
    }
}
#[inline(always)]
pub fn block_0x0020ec98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 7usize, 29usize, 2157724u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2157740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ecac));
    } else {
        emu.pc = 2157728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eca0));
    }
}
#[inline(always)]
pub fn block_0x0020eca0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2157732u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157784u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ecd8));
}
#[inline(always)]
pub fn block_0x0020eca4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 6usize, 30usize, 2157736u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2157784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ecd8));
    } else {
        emu.pc = 2157740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ecac));
    }
}
#[inline(always)]
pub fn block_0x0020ecac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 30usize, 21usize, 2157744u32);
    emu.sbr_no_count(12usize, 29usize, 18usize, 2157748u32);
    emu.sltru_no_count(31usize, 6usize, 30usize, 2157752u32);
    emu.sbr_no_count(10usize, 12usize, 10usize, 2157756u32);
    emu.sbr_no_count(12usize, 7usize, 29usize, 2157760u32);
    emu.sbr_no_count(12usize, 12usize, 31usize, 2157764u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2157776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ecd0));
    } else {
        emu.pc = 2157768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ecc8));
    }
}
#[inline(always)]
pub fn block_0x0020ecc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 30usize, 21usize, 2157772u32);
    emu.sbr_no_count(12usize, 6usize, 30usize, 2157776u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2157776u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ecd0));
}
#[inline(always)]
pub fn block_0x0020ecd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 10usize, 12usize, 2157780u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2158260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eeb4));
    } else {
        emu.pc = 2157784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ecd8));
    }
}
#[inline(always)]
pub fn block_0x0020ecd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(22usize, 22usize, 4294967295u32, 2157788u32);
    emu.sb_no_count(22usize, 14usize, 4294967295u32, 2157792u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2157832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed08));
    } else {
        emu.pc = 2157796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ece4));
    }
}
#[inline(always)]
pub fn block_0x0020ece4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 7usize, 29usize, 2157800u32);
    emu.sltru_no_count(12usize, 17usize, 6usize, 2157804u32);
    emu.sbr_no_count(31usize, 5usize, 7usize, 2157808u32);
    emu.sbr_no_count(18usize, 31usize, 12usize, 2157812u32);
    emu.sbr_no_count(19usize, 17usize, 6usize, 2157816u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2157856u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed20));
    } else {
        emu.pc = 2157820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ecfc));
    }
}
#[inline(always)]
pub fn block_0x0020ecfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(31usize, 19usize, 16usize, 2157824u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2157864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed28));
    } else {
        emu.pc = 2157828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed04));
    }
}
#[inline(always)]
pub fn block_0x0020ed04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2157832u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157884u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ed3c));
}
#[inline(always)]
pub fn block_0x0020ed08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 6usize, 30usize, 2157836u32);
    emu.sltru_no_count(12usize, 17usize, 6usize, 2157840u32);
    emu.sbr_no_count(31usize, 5usize, 7usize, 2157844u32);
    emu.sbr_no_count(18usize, 31usize, 12usize, 2157848u32);
    emu.sbr_no_count(19usize, 17usize, 6usize, 2157852u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2157820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ecfc));
    } else {
        emu.pc = 2157856u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed20));
    }
}
#[inline(always)]
pub fn block_0x0020ed20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(31usize, 18usize, 15usize, 2157860u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2157884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed3c));
    } else {
        emu.pc = 2157864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed28));
    }
}
#[inline(always)]
pub fn block_0x0020ed28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2157684u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec74));
    } else {
        emu.pc = 2157868u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed2c));
    }
}
#[inline(always)]
pub fn block_0x0020ed2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 19usize, 16usize, 2157872u32);
    emu.adi_no_count(21usize, 6usize, 0u32, 2157876u32);
    emu.adi_no_count(18usize, 7usize, 0u32, 2157880u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2157700u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec84));
    } else {
        emu.pc = 2157884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed3c));
    }
}
#[inline(always)]
pub fn block_0x0020ed3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2157388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb4c));
    } else {
        emu.pc = 2157888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed40));
    }
}
#[inline(always)]
pub fn block_0x0020ed40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 7usize, 28usize, 2157892u32);
    emu.lw_no_count(29usize, 2usize, 16u32, 2157896u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2158108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee1c));
    } else {
        emu.pc = 2157900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed4c));
    }
}
#[inline(always)]
pub fn block_0x0020ed4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(31usize);
    if a != b {
        emu.pc = 2158108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee1c));
    } else {
        emu.pc = 2157904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed50));
    }
}
#[inline(always)]
pub fn block_0x0020ed50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(16usize, 6usize, 16usize, 2157908u32);
    emu.sltru_no_count(10usize, 16usize, 6usize, 2157912u32);
    emu.adr_no_count(15usize, 7usize, 15usize, 2157916u32);
    emu.adr_no_count(10usize, 15usize, 10usize, 2157920u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2158056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ede8));
    } else {
        emu.pc = 2157924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed64));
    }
}
#[inline(always)]
pub fn block_0x0020ed64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 10usize, 28usize, 2157928u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2158064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020edf0));
    } else {
        emu.pc = 2157932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed6c));
    }
}
#[inline(always)]
pub fn block_0x0020ed6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2157936u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2158288u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020eed0));
}
#[inline(always)]
pub fn block_0x0020ed70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 6usize, 13usize, 2157940u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2158232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee98));
    } else {
        emu.pc = 2157944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed78));
    }
}
#[inline(always)]
pub fn block_0x0020ed78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 13usize, 16usize, 2157948u32);
    emu.sbr_no_count(14usize, 17usize, 5usize, 2157952u32);
    emu.sbr_no_count(15usize, 10usize, 17usize, 2157956u32);
    emu.sltru_no_count(17usize, 6usize, 13usize, 2157960u32);
    emu.sbr_no_count(10usize, 14usize, 12usize, 2157964u32);
    emu.sbr_no_count(12usize, 15usize, 17usize, 2157968u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2157980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed9c));
    } else {
        emu.pc = 2157972u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed94));
    }
}
#[inline(always)]
pub fn block_0x0020ed94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 13usize, 16usize, 2157976u32);
    emu.sbr_no_count(12usize, 6usize, 13usize, 2157980u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2157980u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ed9c));
}
#[inline(always)]
pub fn block_0x0020ed9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 10usize, 12usize, 2157984u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2158232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee98));
    } else {
        emu.pc = 2157988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eda4));
    }
}
#[inline(always)]
pub fn block_0x0020eda4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(10usize, 16usize, 2u32, 2157992u32);
    emu.sltiu_no_count(12usize, 5usize, 1u32, 2157996u32);
    emu.anr_no_count(10usize, 12usize, 10usize, 2158000u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2158232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee98));
    } else {
        emu.pc = 2158004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020edb4));
    }
}
#[inline(always)]
pub fn block_0x0020edb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 4294967292u32, 2158008u32);
    emu.sltru_no_count(12usize, 10usize, 8usize, 2158012u32);
    emu.adr_no_count(12usize, 9usize, 12usize, 2158016u32);
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2158020u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2158224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee90));
    } else {
        emu.pc = 2158024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020edc8));
    }
}
#[inline(always)]
pub fn block_0x0020edc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 12usize, 5usize, 2158028u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2158232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee98));
    } else {
        emu.pc = 2158032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020edd0));
    }
}
#[inline(always)]
pub fn block_0x0020edd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 0usize, 11usize, 2158036u32);
    emu.sw_no_count(7usize, 18usize, 0u32, 2158040u32)?;
    emu.sw_no_count(10usize, 18usize, 4u32, 2158044u32)?;
    emu.lw_no_count(10usize, 2usize, 0u32, 2158048u32)?;
    emu.sh_no_count(10usize, 18usize, 8u32, 2158052u32)?;
    emu.add_memory_rw_events(6usize);
    let return_addr = 2158056u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2158292u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020eed4));
}
#[inline(always)]
pub fn block_0x0020ede8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 16usize, 13usize, 2158060u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2158288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eed0));
    } else {
        emu.pc = 2158064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020edf0));
    }
}
#[inline(always)]
pub fn block_0x0020edf0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 13usize, 6usize, 2158068u32);
    emu.sbr_no_count(14usize, 28usize, 7usize, 2158072u32);
    emu.sbr_no_count(15usize, 10usize, 28usize, 2158076u32);
    emu.sltru_no_count(28usize, 16usize, 13usize, 2158080u32);
    emu.sbr_no_count(10usize, 14usize, 12usize, 2158084u32);
    emu.sbr_no_count(12usize, 15usize, 28usize, 2158088u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2158100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee14));
    } else {
        emu.pc = 2158092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee0c));
    }
}
#[inline(always)]
pub fn block_0x0020ee0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 13usize, 6usize, 2158096u32);
    emu.sbr_no_count(12usize, 16usize, 13usize, 2158100u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2158100u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ee14));
}
#[inline(always)]
pub fn block_0x0020ee14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 10usize, 12usize, 2158104u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2158288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eed0));
    } else {
        emu.pc = 2158108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee1c));
    }
}
#[inline(always)]
pub fn block_0x0020ee1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 20u32, 2158112u32);
    emu.mulhu_no_count(12usize, 8usize, 10usize, 2158116u32);
    emu.mul_no_count(13usize, 9usize, 10usize, 2158120u32);
    emu.adr_no_count(12usize, 12usize, 13usize, 2158124u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2158140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee3c));
    } else {
        emu.pc = 2158128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee30));
    }
}
#[inline(always)]
pub fn block_0x0020ee30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 7usize, 12usize, 2158132u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2158152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee48));
    } else {
        emu.pc = 2158136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee38));
    }
}
#[inline(always)]
pub fn block_0x0020ee38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2158140u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2158288u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020eed0));
}
#[inline(always)]
pub fn block_0x0020ee3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.mul_no_count(10usize, 8usize, 10usize, 2158144u32);
    emu.sltru_no_count(10usize, 6usize, 10usize, 2158148u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2158288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eed0));
    } else {
        emu.pc = 2158152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee48));
    }
}
#[inline]
pub fn block_0x0020ee48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4294967256u32, 2158156u32);
    emu.mul_no_count(12usize, 9usize, 10usize, 2158160u32);
    emu.mulhu_no_count(13usize, 8usize, 10usize, 2158164u32);
    emu.mul_no_count(10usize, 8usize, 10usize, 2158168u32);
    emu.sbr_no_count(13usize, 13usize, 8usize, 2158172u32);
    emu.adr_no_count(10usize, 17usize, 10usize, 2158176u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2158180u32);
    emu.adr_no_count(12usize, 5usize, 12usize, 2158184u32);
    emu.sltru_no_count(13usize, 10usize, 17usize, 2158188u32);
    emu.adr_no_count(12usize, 12usize, 13usize, 2158192u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2158280u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eec8));
    } else {
        emu.pc = 2158196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee74));
    }
}
#[inline(always)]
pub fn block_0x0020ee74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 12usize, 7usize, 2158200u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2158288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eed0));
    } else {
        emu.pc = 2158204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee7c));
    }
}
#[inline(always)]
pub fn block_0x0020ee7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(23usize, 29usize, 0u32, 2158208u32)?;
    emu.sw_no_count(11usize, 29usize, 4u32, 2158212u32)?;
    emu.lw_no_count(10usize, 2usize, 0u32, 2158216u32)?;
    emu.sh_no_count(10usize, 29usize, 8u32, 2158220u32)?;
    emu.add_memory_rw_events(5usize);
    let return_addr = 2158224u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2158292u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020eed4));
}
#[inline(always)]
pub fn block_0x0020ee90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 10usize, 16usize, 2158228u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2158032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020edd0));
    } else {
        emu.pc = 2158232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee98));
    }
}
#[inline(always)]
pub fn block_0x0020ee98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 18usize, 0u32, 2158236u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2158240u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2158292u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020eed4));
}
#[inline(always)]
pub fn block_0x0020eea0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 0u32, 2158244u32);
    emu.adi_no_count(16usize, 26usize, 0u32, 2158248u32);
    emu.adi_no_count(5usize, 27usize, 0u32, 2158252u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a != b {
        emu.pc = 2157620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec34));
    } else {
        emu.pc = 2158256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eeb0));
    }
}
#[inline(always)]
pub fn block_0x0020eeb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2158260u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157288u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(2157288u32));
}
#[inline(always)]
pub fn block_0x0020eeb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(31usize, 0usize, 0u32, 2158264u32);
    emu.adi_no_count(6usize, 21usize, 0u32, 2158268u32);
    emu.adi_no_count(7usize, 18usize, 0u32, 2158272u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a != b {
        emu.pc = 2157888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed40));
    } else {
        emu.pc = 2158276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eec4));
    }
}
#[inline(always)]
pub fn block_0x0020eec4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2158280u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157388u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020eb4c));
}
#[inline(always)]
pub fn block_0x0020eec8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 10usize, 6usize, 2158284u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2158204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee7c));
    } else {
        emu.pc = 2158288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eed0));
    }
}
#[inline(always)]
pub fn block_0x0020eed0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 29usize, 0u32, 2158292u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2158292u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020eed4));
}
#[inline]
pub fn block_0x0020eed4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 124u32, 2158296u32)?;
    emu.lw_no_count(8usize, 2usize, 120u32, 2158300u32)?;
    emu.lw_no_count(9usize, 2usize, 116u32, 2158304u32)?;
    emu.lw_no_count(18usize, 2usize, 112u32, 2158308u32)?;
    emu.lw_no_count(19usize, 2usize, 108u32, 2158312u32)?;
    emu.lw_no_count(20usize, 2usize, 104u32, 2158316u32)?;
    emu.lw_no_count(21usize, 2usize, 100u32, 2158320u32)?;
    emu.lw_no_count(22usize, 2usize, 96u32, 2158324u32)?;
    emu.lw_no_count(23usize, 2usize, 92u32, 2158328u32)?;
    emu.lw_no_count(24usize, 2usize, 88u32, 2158332u32)?;
    emu.lw_no_count(25usize, 2usize, 84u32, 2158336u32)?;
    emu.lw_no_count(26usize, 2usize, 80u32, 2158340u32)?;
    emu.lw_no_count(27usize, 2usize, 76u32, 2158344u32)?;
    emu.adi_no_count(2usize, 2usize, 128u32, 2158348u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158352u32;
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
pub fn block_0x0020ef10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 2usize, 52u32, 2158356u32)?;
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2158360u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965928u32, 2158364u32);
    emu.adi_no_count(11usize, 2usize, 40u32, 2158368u32);
    emu.adi_no_count(12usize, 2usize, 24u32, 2158372u32);
    emu.adi_no_count(13usize, 2usize, 52u32, 2158376u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2158380u32);
    emu.apc_no_count(1usize, 2158380u32, 4294955008u32, 2158384u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158388u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1264u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020ef34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2158392u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966608u32, 2158396u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2158400u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966636u32, 2158404u32);
    emu.adi_no_count(11usize, 0usize, 28u32, 2158408u32);
    emu.apc_no_count(1usize, 2158408u32, 4294955008u32, 2158412u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158416u32;
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
#[inline(always)]
pub fn block_0x0020ef50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2158420u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966652u32, 2158424u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2158428u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966684u32, 2158432u32);
    emu.adi_no_count(11usize, 0usize, 29u32, 2158436u32);
    emu.apc_no_count(1usize, 2158436u32, 4294955008u32, 2158440u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158444u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1052u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020ef6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2158448u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966700u32, 2158452u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2158456u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966728u32, 2158460u32);
    emu.adi_no_count(11usize, 0usize, 28u32, 2158464u32);
    emu.apc_no_count(1usize, 2158464u32, 4294955008u32, 2158468u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158472u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1024u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020ef88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2158476u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966992u32, 2158480u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2158484u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967048u32, 2158488u32);
    emu.adi_no_count(11usize, 0usize, 54u32, 2158492u32);
    emu.apc_no_count(1usize, 2158492u32, 4294955008u32, 2158496u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158500u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(996u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020efa4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2158504u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966920u32, 2158508u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2158512u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966976u32, 2158516u32);
    emu.adi_no_count(11usize, 0usize, 55u32, 2158520u32);
    emu.apc_no_count(1usize, 2158520u32, 4294955008u32, 2158524u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158528u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(968u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020efc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2158532u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966744u32, 2158536u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2158540u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966792u32, 2158544u32);
    emu.adi_no_count(11usize, 0usize, 45u32, 2158548u32);
    emu.apc_no_count(1usize, 2158548u32, 4294955008u32, 2158552u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158556u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(940u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020efdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2158560u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966808u32, 2158564u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2158568u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966856u32, 2158572u32);
    emu.adi_no_count(11usize, 0usize, 45u32, 2158576u32);
    emu.apc_no_count(1usize, 2158576u32, 4294955008u32, 2158580u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158584u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(912u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020eff8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2158588u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966888u32, 2158592u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2158596u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2158600u32);
    emu.apc_no_count(1usize, 2158600u32, 4294955008u32, 2158604u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158608u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(948u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020f010(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2158612u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966904u32, 2158616u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2158620u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2158624u32);
    emu.apc_no_count(1usize, 2158624u32, 4294955008u32, 2158628u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158632u32;
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
pub fn block_0x0020f028(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2158636u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966592u32, 2158640u32);
    emu.adi_no_count(11usize, 0usize, 81u32, 2158644u32);
    emu.adi_no_count(10usize, 14usize, 0u32, 2158648u32);
    emu.apc_no_count(1usize, 2158648u32, 4294955008u32, 2158652u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158656u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(900u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f040(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2158660u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2158664u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2158668u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2158672u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2158676u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2158680u32)?;
    emu.sw_no_count(20usize, 2usize, 24u32, 2158684u32)?;
    emu.sw_no_count(21usize, 2usize, 20u32, 2158688u32)?;
    emu.sw_no_count(22usize, 2usize, 16u32, 2158692u32)?;
    emu.sw_no_count(23usize, 2usize, 12u32, 2158696u32)?;
    emu.sw_no_count(24usize, 2usize, 8u32, 2158700u32)?;
    emu.sw_no_count(25usize, 2usize, 4u32, 2158704u32)?;
    emu.sw_no_count(26usize, 2usize, 0u32, 2158708u32)?;
    emu.adi_no_count(15usize, 14usize, 0u32, 2158712u32);
    emu.lw_no_count(17usize, 11usize, 0u32, 2158716u32)?;
    emu.lw_no_count(16usize, 11usize, 4u32, 2158720u32)?;
    emu.orr_no_count(14usize, 17usize, 16usize, 2158724u32);
    emu.add_memory_rw_events(17usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2160320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f6c0));
    } else {
        emu.pc = 2158728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f088));
    }
}
#[inline(always)]
pub fn block_0x0020f088(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(14usize, 16usize, 29u32, 2158732u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a != b {
        emu.pc = 2160348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f6dc));
    } else {
        emu.pc = 2158736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f090));
    }
}
#[inline(always)]
pub fn block_0x0020f090(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2160376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f6f8));
    } else {
        emu.pc = 2158740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f094));
    }
}
#[inline]
pub fn block_0x0020f094(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lh_no_count(14usize, 11usize, 24u32, 2158744u32)?;
    emu.sri_no_count(11usize, 17usize, 1u32, 2158748u32);
    let a = 0u32.wrapping_add(1431654400u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2158752u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(858992640u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2158756u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(252645376u32);
    emu.write_reg_no_count(29usize, a);
    emu.pc = 2158760u32;
    emu.update_insn_clock();
    emu.adi_no_count(28usize, 5usize, 1365u32, 2158764u32);
    emu.adi_no_count(7usize, 6usize, 819u32, 2158768u32);
    emu.adi_no_count(5usize, 29usize, 4294967055u32, 2158772u32);
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2158776u32;
    emu.update_insn_clock();
    emu.adi_no_count(6usize, 6usize, 257u32, 2158780u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a != b {
        emu.pc = 2158880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f120));
    } else {
        emu.pc = 2158784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f0c0));
    }
}
#[inline]
pub fn block_0x0020f0c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.orr_no_count(29usize, 17usize, 11usize, 2158788u32);
    emu.sri_no_count(30usize, 29usize, 2u32, 2158792u32);
    emu.orr_no_count(29usize, 29usize, 30usize, 2158796u32);
    emu.sri_no_count(30usize, 29usize, 4u32, 2158800u32);
    emu.orr_no_count(29usize, 29usize, 30usize, 2158804u32);
    emu.sri_no_count(30usize, 29usize, 8u32, 2158808u32);
    emu.orr_no_count(29usize, 29usize, 30usize, 2158812u32);
    emu.sri_no_count(30usize, 29usize, 16u32, 2158816u32);
    emu.orr_no_count(29usize, 29usize, 30usize, 2158820u32);
    emu.xri_no_count(29usize, 29usize, 4294967295u32, 2158824u32);
    emu.sri_no_count(30usize, 29usize, 1u32, 2158828u32);
    emu.anr_no_count(28usize, 30usize, 28usize, 2158832u32);
    emu.sbr_no_count(28usize, 29usize, 28usize, 2158836u32);
    emu.anr_no_count(29usize, 28usize, 7usize, 2158840u32);
    emu.sri_no_count(28usize, 28usize, 2u32, 2158844u32);
    emu.anr_no_count(7usize, 28usize, 7usize, 2158848u32);
    emu.adr_no_count(7usize, 29usize, 7usize, 2158852u32);
    emu.sri_no_count(28usize, 7usize, 4u32, 2158856u32);
    emu.adr_no_count(7usize, 7usize, 28usize, 2158860u32);
    emu.anr_no_count(5usize, 7usize, 5usize, 2158864u32);
    emu.mul_no_count(5usize, 5usize, 6usize, 2158868u32);
    emu.sri_no_count(5usize, 5usize, 24u32, 2158872u32);
    emu.adi_no_count(6usize, 5usize, 32u32, 2158876u32);
    emu.add_memory_rw_events(24usize);
    let return_addr = 2158880u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2158972u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f17c));
}
#[inline]
pub fn block_0x0020f120(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 23u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(29usize, 16usize, 1u32, 2158884u32);
    emu.orr_no_count(29usize, 16usize, 29usize, 2158888u32);
    emu.sri_no_count(30usize, 29usize, 2u32, 2158892u32);
    emu.orr_no_count(29usize, 29usize, 30usize, 2158896u32);
    emu.sri_no_count(30usize, 29usize, 4u32, 2158900u32);
    emu.orr_no_count(29usize, 29usize, 30usize, 2158904u32);
    emu.sri_no_count(30usize, 29usize, 8u32, 2158908u32);
    emu.orr_no_count(29usize, 29usize, 30usize, 2158912u32);
    emu.sri_no_count(30usize, 29usize, 16u32, 2158916u32);
    emu.orr_no_count(29usize, 29usize, 30usize, 2158920u32);
    emu.xri_no_count(29usize, 29usize, 4294967295u32, 2158924u32);
    emu.sri_no_count(30usize, 29usize, 1u32, 2158928u32);
    emu.anr_no_count(28usize, 30usize, 28usize, 2158932u32);
    emu.sbr_no_count(28usize, 29usize, 28usize, 2158936u32);
    emu.anr_no_count(29usize, 28usize, 7usize, 2158940u32);
    emu.sri_no_count(28usize, 28usize, 2u32, 2158944u32);
    emu.anr_no_count(7usize, 28usize, 7usize, 2158948u32);
    emu.adr_no_count(7usize, 29usize, 7usize, 2158952u32);
    emu.sri_no_count(28usize, 7usize, 4u32, 2158956u32);
    emu.adr_no_count(7usize, 7usize, 28usize, 2158960u32);
    emu.anr_no_count(5usize, 7usize, 5usize, 2158964u32);
    emu.mul_no_count(5usize, 5usize, 6usize, 2158968u32);
    emu.sri_no_count(6usize, 5usize, 24u32, 2158972u32);
    emu.add_memory_rw_events(23usize);
    emu.pc = 2158972u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f17c));
}
#[inline]
pub fn block_0x0020f17c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(5usize, 14usize, 6usize, 2158976u32);
    emu.adi_no_count(14usize, 0usize, 4294967200u32, 2158980u32);
    emu.adi_no_count(7usize, 0usize, 80u32, 2158984u32);
    let a = 0u32.wrapping_add(2068697088u32);
    emu.write_reg_no_count(28usize, a);
    emu.pc = 2158988u32;
    emu.update_insn_clock();
    emu.sbr_no_count(14usize, 14usize, 5usize, 2158992u32);
    emu.adi_no_count(28usize, 28usize, 4294965651u32, 2158996u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2159000u32);
    emu.sai_no_count(14usize, 14usize, 1040u32, 2159004u32);
    emu.adi_no_count(14usize, 14usize, 1087u32, 2159008u32);
    emu.mul_no_count(14usize, 14usize, 7usize, 2159012u32);
    emu.mulh_no_count(14usize, 14usize, 28usize, 2159016u32);
    emu.sri_no_count(28usize, 14usize, 31u32, 2159020u32);
    emu.sai_no_count(14usize, 14usize, 1034u32, 2159024u32);
    emu.adr_no_count(14usize, 14usize, 28usize, 2159028u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a < b {
        emu.pc = 2160452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f744));
    } else {
        emu.pc = 2159032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f1b8));
    }
}
#[inline(always)]
pub fn block_0x0020f1b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(7usize, 6usize, 4294967264u32, 2159036u32);
    emu.slr_no_count(17usize, 17usize, 6usize, 2159040u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(7usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2159052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f1cc));
    } else {
        emu.pc = 2159044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f1c4));
    }
}
#[inline(always)]
pub fn block_0x0020f1c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 17usize, 0u32, 2159048u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2159052u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2159068u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f1dc));
}
#[inline(always)]
pub fn block_0x0020f1cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(28usize, 6usize, 4294967295u32, 2159056u32);
    emu.srr_no_count(11usize, 11usize, 28usize, 2159060u32);
    emu.slr_no_count(16usize, 16usize, 6usize, 2159064u32);
    emu.orr_no_count(11usize, 16usize, 11usize, 2159068u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2159068u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f1dc));
}
#[inline(never)]
pub fn block_0x0020f1dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 40u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sai_no_count(16usize, 7usize, 1055u32, 2159072u32);
    emu.sli_no_count(14usize, 14usize, 4u32, 2159076u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2159080u32;
    emu.update_insn_clock();
    emu.adi_no_count(6usize, 6usize, 4294965296u32, 2159084u32);
    emu.adr_no_count(14usize, 6usize, 14usize, 2159088u32);
    emu.lw_no_count(6usize, 14usize, 0u32, 2159092u32)?;
    emu.lw_no_count(7usize, 14usize, 4u32, 2159096u32)?;
    emu.anr_no_count(16usize, 16usize, 17usize, 2159100u32);
    emu.lh_no_count(17usize, 14usize, 8u32, 2159104u32)?;
    emu.mulhu_no_count(28usize, 6usize, 16usize, 2159108u32);
    emu.mul_no_count(29usize, 7usize, 16usize, 2159112u32);
    emu.mulhu_no_count(30usize, 7usize, 16usize, 2159116u32);
    emu.mul_no_count(31usize, 6usize, 11usize, 2159120u32);
    emu.mulhu_no_count(6usize, 6usize, 11usize, 2159124u32);
    emu.mul_no_count(8usize, 7usize, 11usize, 2159128u32);
    emu.mulhu_no_count(11usize, 7usize, 11usize, 2159132u32);
    emu.adr_no_count(17usize, 5usize, 17usize, 2159136u32);
    emu.adi_no_count(16usize, 0usize, 4294967232u32, 2159140u32);
    emu.adr_no_count(28usize, 29usize, 28usize, 2159144u32);
    emu.sbr_no_count(5usize, 16usize, 17usize, 2159148u32);
    emu.sbr_no_count(16usize, 0usize, 17usize, 2159152u32);
    emu.sltru_no_count(17usize, 28usize, 29usize, 2159156u32);
    emu.adr_no_count(28usize, 31usize, 28usize, 2159160u32);
    emu.adr_no_count(17usize, 30usize, 17usize, 2159164u32);
    emu.ani_no_count(29usize, 5usize, 63u32, 2159168u32);
    emu.sltru_no_count(7usize, 28usize, 31usize, 2159172u32);
    emu.sri_no_count(30usize, 28usize, 31u32, 2159176u32);
    emu.adr_no_count(6usize, 6usize, 7usize, 2159180u32);
    emu.adi_no_count(28usize, 29usize, 4294967264u32, 2159184u32);
    emu.adr_no_count(6usize, 17usize, 6usize, 2159188u32);
    emu.sltru_no_count(17usize, 6usize, 17usize, 2159192u32);
    emu.adr_no_count(6usize, 8usize, 6usize, 2159196u32);
    emu.sltru_no_count(7usize, 6usize, 8usize, 2159200u32);
    emu.adr_no_count(11usize, 11usize, 17usize, 2159204u32);
    emu.adr_no_count(6usize, 30usize, 6usize, 2159208u32);
    emu.adr_no_count(11usize, 11usize, 7usize, 2159212u32);
    emu.sltru_no_count(7usize, 6usize, 30usize, 2159216u32);
    emu.adr_no_count(7usize, 11usize, 7usize, 2159220u32);
    emu.xri_no_count(8usize, 29usize, 4294967295u32, 2159224u32);
    emu.add_memory_rw_events(39usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2159236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f284));
    } else {
        emu.pc = 2159228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f27c));
    }
}
#[inline(always)]
pub fn block_0x0020f27c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.srr_no_count(11usize, 7usize, 29usize, 2159232u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2159236u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2159252u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f294));
}
#[inline(always)]
pub fn block_0x0020f284(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 7usize, 1u32, 2159240u32);
    emu.slr_no_count(11usize, 11usize, 8usize, 2159244u32);
    emu.srr_no_count(17usize, 6usize, 16usize, 2159248u32);
    emu.orr_no_count(11usize, 17usize, 11usize, 2159252u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2159252u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f294));
}
#[inline]
pub fn block_0x0020f294(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lhu_no_count(14usize, 14usize, 10u32, 2159256u32)?;
    emu.slti_no_count(17usize, 28usize, 0u32, 2159260u32);
    emu.adi_no_count(30usize, 0usize, 1u32, 2159264u32);
    emu.slr_no_count(31usize, 30usize, 29usize, 2159268u32);
    emu.adi_no_count(9usize, 17usize, 4294967295u32, 2159272u32);
    emu.sbr_no_count(17usize, 0usize, 17usize, 2159276u32);
    emu.slr_no_count(16usize, 30usize, 16usize, 2159280u32);
    emu.anr_no_count(31usize, 9usize, 31usize, 2159284u32);
    emu.anr_no_count(30usize, 17usize, 16usize, 2159288u32);
    emu.sltiu_no_count(16usize, 30usize, 1u32, 2159292u32);
    emu.adi_no_count(9usize, 30usize, 4294967295u32, 2159296u32);
    emu.sbr_no_count(18usize, 31usize, 16usize, 2159300u32);
    emu.anr_no_count(17usize, 18usize, 7usize, 2159304u32);
    emu.anr_no_count(16usize, 9usize, 6usize, 2159308u32);
    emu.orr_no_count(19usize, 16usize, 17usize, 2159312u32);
    emu.add_memory_rw_events(15usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2159380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f314));
    } else {
        emu.pc = 2159316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f2d4));
    }
}
#[inline(always)]
pub fn block_0x0020f2d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(19usize, 11usize, 4u32, 2159320u32);
    emu.adi_no_count(20usize, 0usize, 625u32, 2159324u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a >= b {
        emu.pc = 2159420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f33c));
    } else {
        emu.pc = 2159328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f2e0));
    }
}
#[inline(always)]
pub fn block_0x0020f2e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 100u32, 2159332u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2159500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f38c));
    } else {
        emu.pc = 2159336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f2e8));
    }
}
#[inline(always)]
pub fn block_0x0020f2e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 9u32, 2159340u32);
    emu.sltiu_no_count(20usize, 11usize, 10u32, 2159344u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a < b {
        emu.pc = 2160148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f614));
    } else {
        emu.pc = 2159348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f2f4));
    }
}
#[inline(always)]
pub fn block_0x0020f2f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 1u32, 2159352u32);
    emu.xri_no_count(20usize, 20usize, 1u32, 2159356u32);
    emu.sbr_no_count(14usize, 20usize, 14usize, 2159360u32);
    emu.adi_no_count(14usize, 14usize, 1u32, 2159364u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2159368u32);
    emu.sai_no_count(14usize, 14usize, 1040u32, 2159372u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(15usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2159540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f3b4));
    } else {
        emu.pc = 2159376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f310));
    }
}
#[inline(always)]
pub fn block_0x0020f310(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2159380u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2159480u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f378));
}
#[inline(always)]
pub fn block_0x0020f314(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 10u32, 2159384u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a < b {
        emu.pc = 2159412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f334));
    } else {
        emu.pc = 2159388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f31c));
    }
}
#[inline(always)]
pub fn block_0x0020f31c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(19usize, 13usize, 2u32, 2159392u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(20usize, a);
    emu.pc = 2159396u32;
    emu.update_insn_clock();
    emu.adi_no_count(20usize, 20usize, 4294967184u32, 2159400u32);
    emu.adr_no_count(19usize, 20usize, 19usize, 2159404u32);
    emu.lw_no_count(19usize, 19usize, 4294967292u32, 2159408u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2159316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f2d4));
    } else {
        emu.pc = 2159412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f334));
    }
}
#[inline(always)]
pub fn block_0x0020f334(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 10usize, 0u32, 2159416u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2159420u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2160264u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f688));
}
#[inline(always)]
pub fn block_0x0020f33c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(999424u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2159424u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 19usize, 576u32, 2159428u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2159652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f424));
    } else {
        emu.pc = 2159432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f348));
    }
}
#[inline(always)]
pub fn block_0x0020f348(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(98304u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2159436u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 19usize, 1696u32, 2159440u32);
    emu.sltru_no_count(20usize, 11usize, 19usize, 2159444u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2159456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f360));
    } else {
        emu.pc = 2159448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f358));
    }
}
#[inline(always)]
pub fn block_0x0020f358(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2159452u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 19usize, 1808u32, 2159456u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2159456u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f360));
}
#[inline(always)]
pub fn block_0x0020f360(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(20usize, 20usize, 5u32, 2159460u32);
    emu.sbr_no_count(14usize, 20usize, 14usize, 2159464u32);
    emu.adi_no_count(14usize, 14usize, 1u32, 2159468u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2159472u32);
    emu.sai_no_count(14usize, 14usize, 1040u32, 2159476u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(15usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2159540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f3b4));
    } else {
        emu.pc = 2159480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f378));
    }
}
#[inline(always)]
pub fn block_0x0020f378(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(7usize, 14usize, 15usize, 2159484u32);
    emu.sli_no_count(6usize, 5usize, 16u32, 2159488u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a < b {
        emu.pc = 2159716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f464));
    } else {
        emu.pc = 2159492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f384));
    }
}
#[inline(always)]
pub fn block_0x0020f384(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 13usize, 0u32, 2159496u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2159500u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2159724u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f46c));
}
#[inline(always)]
pub fn block_0x0020f38c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(21usize, 0usize, 1000u32, 2159504u32);
    emu.sltiu_no_count(20usize, 11usize, 1000u32, 2159508u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2159516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f39c));
    } else {
        emu.pc = 2159512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f398));
    }
}
#[inline(always)]
pub fn block_0x0020f398(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 1000u32, 2159516u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2159516u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f39c));
}
#[inline(always)]
pub fn block_0x0020f39c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(20usize, 20usize, 3u32, 2159520u32);
    emu.sbr_no_count(14usize, 20usize, 14usize, 2159524u32);
    emu.adi_no_count(14usize, 14usize, 1u32, 2159528u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2159532u32);
    emu.sai_no_count(14usize, 14usize, 1040u32, 2159536u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(15usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2159480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f378));
    } else {
        emu.pc = 2159540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f3b4));
    }
}
#[inline(never)]
pub fn block_0x0020f3b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 26u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 6usize, 1u32, 2159544u32);
    emu.sli_no_count(16usize, 7usize, 31u32, 2159548u32);
    emu.sri_no_count(17usize, 7usize, 1u32, 2159552u32);
    let a = 0u32.wrapping_add(3435974656u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2159556u32;
    emu.update_insn_clock();
    emu.orr_no_count(11usize, 11usize, 16usize, 2159560u32);
    emu.adi_no_count(6usize, 5usize, 4294966477u32, 2159564u32);
    emu.adi_no_count(16usize, 5usize, 4294966476u32, 2159568u32);
    emu.adr_no_count(5usize, 11usize, 17usize, 2159572u32);
    emu.sltru_no_count(7usize, 5usize, 11usize, 2159576u32);
    emu.adr_no_count(5usize, 5usize, 7usize, 2159580u32);
    emu.mulhu_no_count(7usize, 5usize, 6usize, 2159584u32);
    emu.sri_no_count(9usize, 7usize, 2u32, 2159588u32);
    emu.ani_no_count(7usize, 7usize, 4294967292u32, 2159592u32);
    emu.adr_no_count(7usize, 7usize, 9usize, 2159596u32);
    emu.sbr_no_count(5usize, 5usize, 7usize, 2159600u32);
    emu.sbr_no_count(7usize, 11usize, 5usize, 2159604u32);
    emu.sltru_no_count(11usize, 11usize, 5usize, 2159608u32);
    emu.mul_no_count(5usize, 7usize, 16usize, 2159612u32);
    emu.mulhu_no_count(9usize, 7usize, 6usize, 2159616u32);
    emu.sbr_no_count(11usize, 17usize, 11usize, 2159620u32);
    emu.mul_no_count(16usize, 7usize, 6usize, 2159624u32);
    emu.adr_no_count(17usize, 9usize, 5usize, 2159628u32);
    emu.mul_no_count(11usize, 11usize, 6usize, 2159632u32);
    emu.adr_no_count(17usize, 17usize, 11usize, 2159636u32);
    emu.slr_no_count(11usize, 19usize, 29usize, 2159640u32);
    emu.add_memory_rw_events(25usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2159852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f4ec));
    } else {
        emu.pc = 2159644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f41c));
    }
}
#[inline(always)]
pub fn block_0x0020f41c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(29usize, 11usize, 0u32, 2159648u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2159652u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2159860u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f4f4));
}
#[inline(always)]
pub fn block_0x0020f424(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(99999744u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2159656u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 19usize, 256u32, 2159660u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2160180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f634));
    } else {
        emu.pc = 2159664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f430));
    }
}
#[inline(always)]
pub fn block_0x0020f430(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(9998336u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2159668u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 19usize, 1664u32, 2159672u32);
    emu.sltru_no_count(20usize, 11usize, 19usize, 2159676u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2159688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f448));
    } else {
        emu.pc = 2159680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f440));
    }
}
#[inline(always)]
pub fn block_0x0020f440(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(999424u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2159684u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 19usize, 576u32, 2159688u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2159688u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f448));
}
#[inline(always)]
pub fn block_0x0020f448(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(20usize, 20usize, 7u32, 2159692u32);
    emu.sbr_no_count(14usize, 20usize, 14usize, 2159696u32);
    emu.adi_no_count(14usize, 14usize, 1u32, 2159700u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2159704u32);
    emu.sai_no_count(14usize, 14usize, 1040u32, 2159708u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(15usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2159480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f378));
    } else {
        emu.pc = 2159712u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f460));
    }
}
#[inline(always)]
pub fn block_0x0020f460(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2159716u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2159540u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f3b4));
}
#[inline(always)]
pub fn block_0x0020f464(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(7usize, 7usize, 16u32, 2159720u32);
    emu.sai_no_count(5usize, 7usize, 1040u32, 2159724u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2159724u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f46c));
}
#[inline(always)]
pub fn block_0x0020f46c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(7usize, 6usize, 16u32, 2159728u32);
    emu.adi_no_count(22usize, 0usize, 4294967295u32, 2159732u32);
    let a = 0u32.wrapping_add(3435974656u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2159736u32;
    emu.update_insn_clock();
    emu.adi_no_count(23usize, 0usize, 10u32, 2159740u32);
    emu.adi_no_count(24usize, 6usize, 4294966477u32, 2159744u32);
    emu.adi_no_count(21usize, 0usize, 4294967295u32, 2159748u32);
    emu.adi_no_count(25usize, 12usize, 0u32, 2159752u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2159752u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f488));
}
#[inline(always)]
pub fn block_0x0020f488(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(26usize, 13usize, 21usize, 2159756u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a == b {
        emu.pc = 2160404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f714));
    } else {
        emu.pc = 2159760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f490));
    }
}
#[inline(always)]
pub fn block_0x0020f490(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(6usize, 19usize, 0u32, 2159764u32);
    emu.divu_no_count(19usize, 11usize, 19usize, 2159768u32);
    emu.mul_no_count(26usize, 19usize, 6usize, 2159772u32);
    emu.sbr_no_count(11usize, 11usize, 26usize, 2159776u32);
    emu.adr_no_count(26usize, 5usize, 21usize, 2159780u32);
    emu.adi_no_count(19usize, 19usize, 48u32, 2159784u32);
    emu.sb_no_count(19usize, 25usize, 0u32, 2159788u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a == b {
        emu.pc = 2159836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f4dc));
    } else {
        emu.pc = 2159792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f4b0));
    }
}
#[inline(always)]
pub fn block_0x0020f4b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(19usize, 20usize, 21usize, 2159796u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2159884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f50c));
    } else {
        emu.pc = 2159800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f4b8));
    }
}
#[inline(always)]
pub fn block_0x0020f4b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.mulhu_no_count(19usize, 6usize, 24usize, 2159804u32);
    emu.adi_no_count(25usize, 25usize, 1u32, 2159808u32);
    emu.sri_no_count(19usize, 19usize, 3u32, 2159812u32);
    emu.adi_no_count(21usize, 21usize, 4294967295u32, 2159816u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a >= b {
        emu.pc = 2159752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f488));
    } else {
        emu.pc = 2159820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f4cc));
    }
}
#[inline(always)]
pub fn block_0x0020f4cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2159824u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294967224u32, 2159828u32);
    emu.apc_no_count(1usize, 2159828u32, 0u32, 2159832u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2159836u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1516u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020f4dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.slr_no_count(7usize, 11usize, 29usize, 2159840u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2160072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f5c8));
    } else {
        emu.pc = 2159844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f4e4));
    }
}
#[inline(always)]
pub fn block_0x0020f4e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 7usize, 0u32, 2159848u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2159852u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2160080u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f5d0));
}
#[inline(always)]
pub fn block_0x0020f4ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(5usize, 19usize, 1u32, 2159856u32);
    emu.srr_no_count(29usize, 5usize, 8usize, 2159860u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2159860u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f4f4));
}
#[inline(always)]
pub fn block_0x0020f4f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sai_no_count(5usize, 28usize, 1055u32, 2159864u32);
    emu.anr_no_count(28usize, 5usize, 11usize, 2159868u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2159872u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2159876u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2159880u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2159884u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2160256u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f680));
}
#[inline]
pub fn block_0x0020f50c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(6usize, 0usize, 0u32, 2159888u32);
    emu.adi_no_count(19usize, 7usize, 4294967295u32, 2159892u32);
    emu.sbr_no_count(11usize, 0usize, 21usize, 2159896u32);
    emu.adi_no_count(7usize, 0usize, 1u32, 2159900u32);
    emu.ani_no_count(19usize, 19usize, 63u32, 2159904u32);
    emu.xri_no_count(20usize, 19usize, 4294967295u32, 2159908u32);
    emu.adi_no_count(21usize, 19usize, 4294967264u32, 2159912u32);
    emu.sai_no_count(22usize, 21usize, 1055u32, 2159916u32);
    emu.adi_no_count(23usize, 0usize, 10u32, 2159920u32);
    emu.add_memory_rw_events(10usize);
    let return_addr = 2159924u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2159972u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f564));
}
#[inline(always)]
pub fn block_0x0020f534(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.srr_no_count(24usize, 17usize, 29usize, 2159928u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2159928u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f538));
}
#[inline]
pub fn block_0x0020f538(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(17usize, 17usize, 18usize, 2159932u32);
    emu.anr_no_count(16usize, 16usize, 9usize, 2159936u32);
    emu.mulhu_no_count(25usize, 7usize, 23usize, 2159940u32);
    emu.mul_no_count(6usize, 6usize, 23usize, 2159944u32);
    emu.mul_no_count(7usize, 7usize, 23usize, 2159948u32);
    emu.adi_no_count(24usize, 24usize, 48u32, 2159952u32);
    emu.adr_no_count(6usize, 25usize, 6usize, 2159956u32);
    emu.adr_no_count(25usize, 12usize, 11usize, 2159960u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2159964u32);
    emu.sb_no_count(24usize, 25usize, 0u32, 2159968u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2160228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f664));
    } else {
        emu.pc = 2159972u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f564));
    }
}
#[inline(always)]
pub fn block_0x0020f564(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.srr_no_count(24usize, 6usize, 19usize, 2159976u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(21usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2160000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f580));
    } else {
        emu.pc = 2159980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f56c));
    }
}
#[inline(always)]
pub fn block_0x0020f56c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 24usize, 0u32, 2159984u32);
    emu.anr_no_count(24usize, 22usize, 24usize, 2159988u32);
    emu.orr_no_count(24usize, 25usize, 24usize, 2159992u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a == b {
        emu.pc = 2160028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f59c));
    } else {
        emu.pc = 2159996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f57c));
    }
}
#[inline(always)]
pub fn block_0x0020f57c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2160000u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2159412u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f334));
}
#[inline(always)]
pub fn block_0x0020f580(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.srr_no_count(25usize, 7usize, 19usize, 2160004u32);
    emu.sli_no_count(26usize, 6usize, 1u32, 2160008u32);
    emu.slr_no_count(26usize, 26usize, 20usize, 2160012u32);
    emu.orr_no_count(25usize, 25usize, 26usize, 2160016u32);
    emu.anr_no_count(24usize, 22usize, 24usize, 2160020u32);
    emu.orr_no_count(24usize, 25usize, 24usize, 2160024u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a != b {
        emu.pc = 2159412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f334));
    } else {
        emu.pc = 2160028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f59c));
    }
}
#[inline(always)]
pub fn block_0x0020f59c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2160428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f72c));
    } else {
        emu.pc = 2160032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f5a0));
    }
}
#[inline(always)]
pub fn block_0x0020f5a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.mulhu_no_count(24usize, 16usize, 23usize, 2160036u32);
    emu.mul_no_count(17usize, 17usize, 23usize, 2160040u32);
    emu.adr_no_count(17usize, 24usize, 17usize, 2160044u32);
    emu.mul_no_count(16usize, 16usize, 23usize, 2160048u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2159924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f534));
    } else {
        emu.pc = 2160052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f5b4));
    }
}
#[inline(always)]
pub fn block_0x0020f5b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(24usize, 17usize, 1u32, 2160056u32);
    emu.slr_no_count(24usize, 24usize, 8usize, 2160060u32);
    emu.srr_no_count(25usize, 16usize, 29usize, 2160064u32);
    emu.orr_no_count(24usize, 25usize, 24usize, 2160068u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2160072u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2159928u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f538));
}
#[inline(always)]
pub fn block_0x0020f5c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 11usize, 1u32, 2160076u32);
    emu.srr_no_count(9usize, 11usize, 8usize, 2160080u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2160080u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f5d0));
}
#[inline(always)]
pub fn block_0x0020f5d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sai_no_count(11usize, 28usize, 1055u32, 2160084u32);
    emu.adr_no_count(17usize, 9usize, 17usize, 2160088u32);
    emu.anr_no_count(7usize, 11usize, 7usize, 2160092u32);
    emu.adr_no_count(16usize, 7usize, 16usize, 2160096u32);
    emu.sltru_no_count(7usize, 16usize, 7usize, 2160100u32);
    emu.adr_no_count(17usize, 17usize, 7usize, 2160104u32);
    emu.slr_no_count(7usize, 6usize, 29usize, 2160108u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2160120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f5f8));
    } else {
        emu.pc = 2160112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f5f0));
    }
}
#[inline(always)]
pub fn block_0x0020f5f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(29usize, 7usize, 0u32, 2160116u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2160120u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2160128u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f600));
}
#[inline(always)]
pub fn block_0x0020f5f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(6usize, 6usize, 1u32, 2160124u32);
    emu.srr_no_count(29usize, 6usize, 8usize, 2160128u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2160128u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f600));
}
#[inline(always)]
pub fn block_0x0020f600(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(28usize, 11usize, 7usize, 2160132u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2160136u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2160140u32);
    emu.adi_no_count(13usize, 5usize, 0u32, 2160144u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2160148u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2160256u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f680));
}
#[inline(always)]
pub fn block_0x0020f614(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 10u32, 2160152u32);
    emu.xri_no_count(20usize, 20usize, 1u32, 2160156u32);
    emu.sbr_no_count(14usize, 20usize, 14usize, 2160160u32);
    emu.adi_no_count(14usize, 14usize, 1u32, 2160164u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2160168u32);
    emu.sai_no_count(14usize, 14usize, 1040u32, 2160172u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(15usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2159540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f3b4));
    } else {
        emu.pc = 2160176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f630));
    }
}
#[inline(always)]
pub fn block_0x0020f630(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2160180u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2159480u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f378));
}
#[inline(always)]
pub fn block_0x0020f634(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(1000001536u32);
    emu.write_reg_no_count(20usize, a);
    emu.pc = 2160184u32;
    emu.update_insn_clock();
    emu.adi_no_count(21usize, 20usize, 4294965760u32, 2160188u32);
    emu.sltru_no_count(20usize, 11usize, 21usize, 2160192u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2160200u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f648));
    } else {
        emu.pc = 2160196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f644));
    }
}
#[inline(always)]
pub fn block_0x0020f644(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 21usize, 0u32, 2160200u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2160200u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f648));
}
#[inline(always)]
pub fn block_0x0020f648(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(20usize, 20usize, 9u32, 2160204u32);
    emu.sbr_no_count(14usize, 20usize, 14usize, 2160208u32);
    emu.adi_no_count(14usize, 14usize, 1u32, 2160212u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2160216u32);
    emu.sai_no_count(14usize, 14usize, 1040u32, 2160220u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(15usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2159480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f378));
    } else {
        emu.pc = 2160224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f660));
    }
}
#[inline(always)]
pub fn block_0x0020f660(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2160228u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2159540u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f3b4));
}
#[inline(always)]
pub fn block_0x0020f664(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 12usize, 0u32, 2160232u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2160236u32);
    emu.adi_no_count(13usize, 5usize, 0u32, 2160240u32);
    emu.adi_no_count(28usize, 30usize, 0u32, 2160244u32);
    emu.adi_no_count(29usize, 31usize, 0u32, 2160248u32);
    emu.adi_no_count(30usize, 7usize, 0u32, 2160252u32);
    emu.adi_no_count(31usize, 6usize, 0u32, 2160256u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2160256u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f680));
}
#[inline(always)]
pub fn block_0x0020f680(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2160256u32, 0u32, 2160260u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160264u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(220u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f688(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2160268u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2160272u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2160276u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2160280u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2160284u32)?;
    emu.lw_no_count(20usize, 2usize, 24u32, 2160288u32)?;
    emu.lw_no_count(21usize, 2usize, 20u32, 2160292u32)?;
    emu.lw_no_count(22usize, 2usize, 16u32, 2160296u32)?;
    emu.lw_no_count(23usize, 2usize, 12u32, 2160300u32)?;
    emu.lw_no_count(24usize, 2usize, 8u32, 2160304u32)?;
    emu.lw_no_count(25usize, 2usize, 4u32, 2160308u32)?;
    emu.lw_no_count(26usize, 2usize, 0u32, 2160312u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2160316u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160320u32;
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
pub fn block_0x0020f6c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2160324u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966608u32, 2160328u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2160332u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967100u32, 2160336u32);
    emu.adi_no_count(11usize, 0usize, 28u32, 2160340u32);
    emu.apc_no_count(1usize, 2160340u32, 4294955008u32, 2160344u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160348u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966444u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020f6dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2160352u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294967116u32, 2160356u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2160360u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967152u32, 2160364u32);
    emu.adi_no_count(11usize, 0usize, 36u32, 2160368u32);
    emu.apc_no_count(1usize, 2160368u32, 4294955008u32, 2160372u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160376u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966416u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020f6f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2160380u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294967064u32, 2160384u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2160388u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967168u32, 2160392u32);
    emu.adi_no_count(11usize, 0usize, 33u32, 2160396u32);
    emu.apc_no_count(1usize, 2160396u32, 4294955008u32, 2160400u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160404u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966388u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020f714(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2160408u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967240u32, 2160412u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2160416u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2160420u32);
    emu.apc_no_count(1usize, 2160420u32, 4294955008u32, 2160424u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160428u32;
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
pub fn block_0x0020f72c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2160432u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967256u32, 2160436u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2160440u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2160444u32);
    emu.apc_no_count(1usize, 2160444u32, 4294955008u32, 2160448u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160452u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x0020f744(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2160456u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966592u32, 2160460u32);
    emu.adi_no_count(11usize, 0usize, 81u32, 2160464u32);
    emu.adi_no_count(10usize, 14usize, 0u32, 2160468u32);
    emu.apc_no_count(1usize, 2160468u32, 4294955008u32, 2160472u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160476u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966376u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
