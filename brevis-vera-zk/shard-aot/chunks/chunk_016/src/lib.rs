pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2156948u32;
pub const PC_MAX: u32 = 2160048u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 172usize] = [
        block_0x0020e994,
        block_0x0020e9a0,
        block_0x0020e9a4,
        block_0x0020e9ac,
        block_0x0020e9b8,
        block_0x0020e9c4,
        block_0x0020e9c8,
        block_0x0020e9d0,
        block_0x0020e9d4,
        block_0x0020e9e4,
        block_0x0020e9f8,
        block_0x0020ea00,
        block_0x0020ea04,
        block_0x0020ea0c,
        block_0x0020ea28,
        block_0x0020ea30,
        block_0x0020ea38,
        block_0x0020ea44,
        block_0x0020ea5c,
        block_0x0020ea64,
        block_0x0020ea68,
        block_0x0020ea80,
        block_0x0020ea88,
        block_0x0020ea8c,
        block_0x0020ea9c,
        block_0x0020eaa0,
        block_0x0020eaac,
        block_0x0020eab0,
        block_0x0020eac4,
        block_0x0020eacc,
        block_0x0020ead0,
        block_0x0020ead8,
        block_0x0020eae0,
        block_0x0020eaf0,
        block_0x0020eb04,
        block_0x0020eb0c,
        block_0x0020eb10,
        block_0x0020eb18,
        block_0x0020eb34,
        block_0x0020eb3c,
        block_0x0020eb44,
        block_0x0020eb50,
        block_0x0020eb68,
        block_0x0020eb70,
        block_0x0020eb74,
        block_0x0020eb8c,
        block_0x0020eb94,
        block_0x0020eb98,
        block_0x0020eba8,
        block_0x0020ebac,
        block_0x0020ebb8,
        block_0x0020ebbc,
        block_0x0020ebd0,
        block_0x0020ebd8,
        block_0x0020ebdc,
        block_0x0020ebe4,
        block_0x0020ec00,
        block_0x0020ec08,
        block_0x0020ec10,
        block_0x0020ec20,
        block_0x0020ec34,
        block_0x0020ec3c,
        block_0x0020ec54,
        block_0x0020ec5c,
        block_0x0020ec78,
        block_0x0020ec80,
        block_0x0020ec88,
        block_0x0020ec9c,
        block_0x0020eca4,
        block_0x0020eca8,
        block_0x0020ecb4,
        block_0x0020ece0,
        block_0x0020ece8,
        block_0x0020ecfc,
        block_0x0020ed04,
        block_0x0020ed0c,
        block_0x0020ed1c,
        block_0x0020ed20,
        block_0x0020ed30,
        block_0x0020ed34,
        block_0x0020ed3c,
        block_0x0020ed40,
        block_0x0020ed7c,
        block_0x0020eda0,
        block_0x0020edbc,
        block_0x0020edd8,
        block_0x0020edf4,
        block_0x0020ee10,
        block_0x0020ee2c,
        block_0x0020ee48,
        block_0x0020ee64,
        block_0x0020ee7c,
        block_0x0020ee94,
        block_0x0020eeac,
        block_0x0020eef4,
        block_0x0020eefc,
        block_0x0020ef00,
        block_0x0020ef2c,
        block_0x0020ef8c,
        block_0x0020efe8,
        block_0x0020f024,
        block_0x0020f030,
        block_0x0020f038,
        block_0x0020f048,
        block_0x0020f0e8,
        block_0x0020f0f0,
        block_0x0020f100,
        block_0x0020f140,
        block_0x0020f14c,
        block_0x0020f154,
        block_0x0020f160,
        block_0x0020f17c,
        block_0x0020f180,
        block_0x0020f188,
        block_0x0020f1a0,
        block_0x0020f1a8,
        block_0x0020f1b4,
        block_0x0020f1c4,
        block_0x0020f1cc,
        block_0x0020f1e4,
        block_0x0020f1f0,
        block_0x0020f1f8,
        block_0x0020f204,
        block_0x0020f208,
        block_0x0020f220,
        block_0x0020f288,
        block_0x0020f290,
        block_0x0020f29c,
        block_0x0020f2ac,
        block_0x0020f2b4,
        block_0x0020f2cc,
        block_0x0020f2d0,
        block_0x0020f2d8,
        block_0x0020f2f4,
        block_0x0020f2fc,
        block_0x0020f31c,
        block_0x0020f324,
        block_0x0020f338,
        block_0x0020f348,
        block_0x0020f350,
        block_0x0020f358,
        block_0x0020f360,
        block_0x0020f378,
        block_0x0020f3a0,
        block_0x0020f3a4,
        block_0x0020f3d0,
        block_0x0020f3d8,
        block_0x0020f3e8,
        block_0x0020f3ec,
        block_0x0020f408,
        block_0x0020f40c,
        block_0x0020f420,
        block_0x0020f434,
        block_0x0020f43c,
        block_0x0020f45c,
        block_0x0020f464,
        block_0x0020f46c,
        block_0x0020f480,
        block_0x0020f49c,
        block_0x0020f4a0,
        block_0x0020f4b0,
        block_0x0020f4b4,
        block_0x0020f4cc,
        block_0x0020f4d0,
        block_0x0020f4ec,
        block_0x0020f4f4,
        block_0x0020f52c,
        block_0x0020f548,
        block_0x0020f564,
        block_0x0020f580,
        block_0x0020f598,
        block_0x0020f5b0,
    ];
    const IDX: [u16; 776usize] = [
        1u16, 0u16, 0u16, 2u16, 3u16, 0u16, 4u16, 0u16, 0u16, 5u16, 0u16, 0u16, 6u16,
        7u16, 0u16, 8u16, 9u16, 0u16, 0u16, 0u16, 10u16, 0u16, 0u16, 0u16, 0u16, 11u16,
        0u16, 12u16, 13u16, 0u16, 14u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 15u16, 0u16,
        16u16, 0u16, 17u16, 0u16, 0u16, 18u16, 0u16, 0u16, 0u16, 0u16, 0u16, 19u16, 0u16,
        20u16, 21u16, 0u16, 0u16, 0u16, 0u16, 0u16, 22u16, 0u16, 23u16, 24u16, 0u16,
        0u16, 0u16, 25u16, 26u16, 0u16, 0u16, 27u16, 28u16, 0u16, 0u16, 0u16, 0u16,
        29u16, 0u16, 30u16, 31u16, 0u16, 32u16, 0u16, 33u16, 0u16, 0u16, 0u16, 34u16,
        0u16, 0u16, 0u16, 0u16, 35u16, 0u16, 36u16, 37u16, 0u16, 38u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 39u16, 0u16, 40u16, 0u16, 41u16, 0u16, 0u16, 42u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 43u16, 0u16, 44u16, 45u16, 0u16, 0u16, 0u16, 0u16, 0u16, 46u16,
        0u16, 47u16, 48u16, 0u16, 0u16, 0u16, 49u16, 50u16, 0u16, 0u16, 51u16, 52u16,
        0u16, 0u16, 0u16, 0u16, 53u16, 0u16, 54u16, 55u16, 0u16, 56u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 57u16, 0u16, 58u16, 0u16, 59u16, 0u16, 0u16, 0u16, 60u16, 0u16,
        0u16, 0u16, 0u16, 61u16, 0u16, 62u16, 0u16, 0u16, 0u16, 0u16, 0u16, 63u16, 0u16,
        64u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 65u16, 0u16, 66u16, 0u16, 67u16, 0u16,
        0u16, 0u16, 0u16, 68u16, 0u16, 69u16, 70u16, 0u16, 0u16, 71u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 72u16, 0u16, 73u16, 0u16, 0u16, 0u16,
        0u16, 74u16, 0u16, 75u16, 0u16, 76u16, 0u16, 0u16, 0u16, 77u16, 78u16, 0u16,
        0u16, 0u16, 79u16, 80u16, 0u16, 81u16, 82u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 83u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 84u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 85u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 86u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 87u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 88u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        89u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 90u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 91u16, 0u16, 0u16, 0u16, 0u16, 0u16, 92u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        93u16, 0u16, 0u16, 0u16, 0u16, 0u16, 94u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 95u16, 0u16,
        96u16, 97u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 98u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 99u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 100u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 101u16, 0u16, 0u16, 102u16,
        0u16, 103u16, 0u16, 0u16, 0u16, 104u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 105u16, 0u16, 106u16, 0u16, 0u16, 0u16,
        107u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 108u16, 0u16, 0u16, 109u16, 0u16, 110u16, 0u16, 0u16, 111u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 112u16, 113u16, 0u16, 114u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 115u16, 0u16, 116u16, 0u16, 0u16, 117u16, 0u16, 0u16, 0u16,
        118u16, 0u16, 119u16, 0u16, 0u16, 0u16, 0u16, 0u16, 120u16, 0u16, 0u16, 121u16,
        0u16, 122u16, 0u16, 0u16, 123u16, 124u16, 0u16, 0u16, 0u16, 0u16, 0u16, 125u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 126u16,
        0u16, 127u16, 0u16, 0u16, 128u16, 0u16, 0u16, 0u16, 129u16, 0u16, 130u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 131u16, 132u16, 0u16, 133u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 134u16, 0u16, 135u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        136u16, 0u16, 137u16, 0u16, 0u16, 0u16, 0u16, 138u16, 0u16, 0u16, 0u16, 139u16,
        0u16, 140u16, 0u16, 141u16, 0u16, 142u16, 0u16, 0u16, 0u16, 0u16, 0u16, 143u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 144u16, 145u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 146u16, 0u16, 147u16, 0u16, 0u16,
        0u16, 148u16, 149u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 150u16, 151u16, 0u16,
        0u16, 0u16, 0u16, 152u16, 0u16, 0u16, 0u16, 0u16, 153u16, 0u16, 154u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 155u16, 0u16, 156u16, 0u16, 157u16, 0u16,
        0u16, 0u16, 0u16, 158u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 159u16, 160u16,
        0u16, 0u16, 0u16, 161u16, 162u16, 0u16, 0u16, 0u16, 0u16, 0u16, 163u16, 164u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 165u16, 0u16, 166u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 167u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 168u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 169u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 170u16, 0u16, 0u16, 0u16, 0u16, 0u16, 171u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 172u16,
    ];
    if pc < 2156948u32 || pc > 2160048u32 {
        return None;
    }
    let word_offset = ((pc - 2156948u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x0020e994(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(31usize, 6usize, 16usize, 2156952u32);
    emu.adr_no_count(28usize, 7usize, 28usize, 2156956u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2156972u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e9ac));
    } else {
        emu.pc = 2156960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e9a0));
    }
}
#[inline(always)]
pub fn block_0x0020e9a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2157264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ead0));
    } else {
        emu.pc = 2156964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e9a4));
    }
}
#[inline(always)]
pub fn block_0x0020e9a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 14usize, 15usize, 2156968u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2157272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ead8));
    } else {
        emu.pc = 2156972u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e9ac));
    }
}
#[inline(always)]
pub fn block_0x0020e9ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(6usize, 21usize, 0u32, 2156976u32);
    emu.adi_no_count(7usize, 18usize, 0u32, 2156980u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a != b {
        emu.pc = 2157484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ebac));
    } else {
        emu.pc = 2156984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e9b8));
    }
}
#[inline(always)]
pub fn block_0x0020e9b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 6usize, 13usize, 2156988u32);
    emu.lw_no_count(29usize, 2usize, 16u32, 2156992u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2157496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ebb8));
    } else {
        emu.pc = 2156996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e9c4));
    }
}
#[inline(always)]
pub fn block_0x0020e9c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2157000u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157704u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ec88));
}
#[inline(always)]
pub fn block_0x0020e9c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 5usize, 6usize, 2157004u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2157028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e9e4));
    } else {
        emu.pc = 2157008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e9d0));
    }
}
#[inline(always)]
pub fn block_0x0020e9d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2157012u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156872u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(2156872u32));
}
#[inline(always)]
pub fn block_0x0020e9d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 29usize, 15usize, 2157016u32);
    emu.adi_no_count(26usize, 16usize, 0u32, 2157020u32);
    emu.adi_no_count(27usize, 5usize, 0u32, 2157024u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2157212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ea9c));
    } else {
        emu.pc = 2157028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e9e4));
    }
}
#[inline(always)]
pub fn block_0x0020e9e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(16usize, 26usize, 6usize, 2157032u32);
    emu.sltru_no_count(5usize, 16usize, 26usize, 2157036u32);
    emu.adr_no_count(10usize, 27usize, 15usize, 2157040u32);
    emu.adr_no_count(5usize, 10usize, 5usize, 2157044u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2157060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ea04));
    } else {
        emu.pc = 2157048u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e9f8));
    }
}
#[inline(always)]
pub fn block_0x0020e9f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 5usize, 7usize, 2157052u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2157068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ea0c));
    } else {
        emu.pc = 2157056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ea00));
    }
}
#[inline(always)]
pub fn block_0x0020ea00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2157060u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157112u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ea38));
}
#[inline(always)]
pub fn block_0x0020ea04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 16usize, 28usize, 2157064u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2157112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ea38));
    } else {
        emu.pc = 2157068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ea0c));
    }
}
#[inline(always)]
pub fn block_0x0020ea0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 28usize, 26usize, 2157072u32);
    emu.sbr_no_count(12usize, 7usize, 27usize, 2157076u32);
    emu.sltru_no_count(14usize, 16usize, 28usize, 2157080u32);
    emu.sbr_no_count(10usize, 12usize, 10usize, 2157084u32);
    emu.sbr_no_count(12usize, 5usize, 7usize, 2157088u32);
    emu.sbr_no_count(12usize, 12usize, 14usize, 2157092u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2157104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ea30));
    } else {
        emu.pc = 2157096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ea28));
    }
}
#[inline(always)]
pub fn block_0x0020ea28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 28usize, 26usize, 2157100u32);
    emu.sbr_no_count(12usize, 16usize, 28usize, 2157104u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2157104u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ea30));
}
#[inline(always)]
pub fn block_0x0020ea30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 10usize, 12usize, 2157108u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2157836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed0c));
    } else {
        emu.pc = 2157112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ea38));
    }
}
#[inline(always)]
pub fn block_0x0020ea38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(30usize, 30usize, 4294967295u32, 2157116u32);
    emu.sb_no_count(30usize, 23usize, 0u32, 2157120u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2157160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ea68));
    } else {
        emu.pc = 2157124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ea44));
    }
}
#[inline(always)]
pub fn block_0x0020ea44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 5usize, 7usize, 2157128u32);
    emu.sltru_no_count(12usize, 8usize, 16usize, 2157132u32);
    emu.sbr_no_count(14usize, 9usize, 5usize, 2157136u32);
    emu.sbr_no_count(29usize, 14usize, 12usize, 2157140u32);
    emu.sbr_no_count(31usize, 8usize, 16usize, 2157144u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(29usize);
    if a != b {
        emu.pc = 2157184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ea80));
    } else {
        emu.pc = 2157148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ea5c));
    }
}
#[inline(always)]
pub fn block_0x0020ea5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(14usize, 31usize, 6usize, 2157152u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2157192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ea88));
    } else {
        emu.pc = 2157156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ea64));
    }
}
#[inline(always)]
pub fn block_0x0020ea64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2157160u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157212u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ea9c));
}
#[inline(always)]
pub fn block_0x0020ea68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 16usize, 28usize, 2157164u32);
    emu.sltru_no_count(12usize, 8usize, 16usize, 2157168u32);
    emu.sbr_no_count(14usize, 9usize, 5usize, 2157172u32);
    emu.sbr_no_count(29usize, 14usize, 12usize, 2157176u32);
    emu.sbr_no_count(31usize, 8usize, 16usize, 2157180u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(29usize);
    if a == b {
        emu.pc = 2157148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ea5c));
    } else {
        emu.pc = 2157184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ea80));
    }
}
#[inline(always)]
pub fn block_0x0020ea80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(14usize, 29usize, 15usize, 2157188u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2157212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ea9c));
    } else {
        emu.pc = 2157192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ea88));
    }
}
#[inline(always)]
pub fn block_0x0020ea88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2157012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e9d4));
    } else {
        emu.pc = 2157196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ea8c));
    }
}
#[inline(always)]
pub fn block_0x0020ea8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 31usize, 6usize, 2157200u32);
    emu.adi_no_count(26usize, 16usize, 0u32, 2157204u32);
    emu.adi_no_count(27usize, 5usize, 0u32, 2157208u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2157028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e9e4));
    } else {
        emu.pc = 2157212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ea9c));
    }
}
#[inline(always)]
pub fn block_0x0020ea9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2156884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2156884u32));
    } else {
        emu.pc = 2157216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eaa0));
    }
}
#[inline(always)]
pub fn block_0x0020eaa0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 5usize, 17usize, 2157220u32);
    emu.lw_no_count(7usize, 2usize, 20u32, 2157224u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2157584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec10));
    } else {
        emu.pc = 2157228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eaac));
    }
}
#[inline(always)]
pub fn block_0x0020eaac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2157584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec10));
    } else {
        emu.pc = 2157232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eab0));
    }
}
#[inline(always)]
pub fn block_0x0020eab0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(6usize, 16usize, 6usize, 2157236u32);
    emu.sltru_no_count(10usize, 6usize, 16usize, 2157240u32);
    emu.adr_no_count(15usize, 5usize, 15usize, 2157244u32);
    emu.adr_no_count(10usize, 15usize, 10usize, 2157248u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2157532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ebdc));
    } else {
        emu.pc = 2157252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eac4));
    }
}
#[inline(always)]
pub fn block_0x0020eac4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 10usize, 17usize, 2157256u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2157540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ebe4));
    } else {
        emu.pc = 2157260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eacc));
    }
}
#[inline(always)]
pub fn block_0x0020eacc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2157264u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157828u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ed04));
}
#[inline(always)]
pub fn block_0x0020ead0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 6usize, 16usize, 2157268u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2156972u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e9ac));
    } else {
        emu.pc = 2157272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ead8));
    }
}
#[inline(always)]
pub fn block_0x0020ead8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(14usize, 23usize, 11usize, 2157276u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2157280u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157296u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020eaf0));
}
#[inline(always)]
pub fn block_0x0020eae0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 18usize, 15usize, 2157284u32);
    emu.adi_no_count(21usize, 6usize, 0u32, 2157288u32);
    emu.adi_no_count(18usize, 7usize, 0u32, 2157292u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2157480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eba8));
    } else {
        emu.pc = 2157296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eaf0));
    }
}
#[inline(always)]
pub fn block_0x0020eaf0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(6usize, 21usize, 16usize, 2157300u32);
    emu.sltru_no_count(7usize, 6usize, 21usize, 2157304u32);
    emu.adr_no_count(10usize, 18usize, 15usize, 2157308u32);
    emu.adr_no_count(7usize, 10usize, 7usize, 2157312u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2157328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb10));
    } else {
        emu.pc = 2157316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb04));
    }
}
#[inline(always)]
pub fn block_0x0020eb04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 7usize, 29usize, 2157320u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2157336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb18));
    } else {
        emu.pc = 2157324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb0c));
    }
}
#[inline(always)]
pub fn block_0x0020eb0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2157328u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157380u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020eb44));
}
#[inline(always)]
pub fn block_0x0020eb10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 6usize, 30usize, 2157332u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2157380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb44));
    } else {
        emu.pc = 2157336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb18));
    }
}
#[inline(always)]
pub fn block_0x0020eb18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 30usize, 21usize, 2157340u32);
    emu.sbr_no_count(12usize, 29usize, 18usize, 2157344u32);
    emu.sltru_no_count(31usize, 6usize, 30usize, 2157348u32);
    emu.sbr_no_count(10usize, 12usize, 10usize, 2157352u32);
    emu.sbr_no_count(12usize, 7usize, 29usize, 2157356u32);
    emu.sbr_no_count(12usize, 12usize, 31usize, 2157360u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2157372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb3c));
    } else {
        emu.pc = 2157364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb34));
    }
}
#[inline(always)]
pub fn block_0x0020eb34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 30usize, 21usize, 2157368u32);
    emu.sbr_no_count(12usize, 6usize, 30usize, 2157372u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2157372u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020eb3c));
}
#[inline(always)]
pub fn block_0x0020eb3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 10usize, 12usize, 2157376u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2157856u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed20));
    } else {
        emu.pc = 2157380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb44));
    }
}
#[inline(always)]
pub fn block_0x0020eb44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(22usize, 22usize, 4294967295u32, 2157384u32);
    emu.sb_no_count(22usize, 14usize, 4294967295u32, 2157388u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2157428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb74));
    } else {
        emu.pc = 2157392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb50));
    }
}
#[inline(always)]
pub fn block_0x0020eb50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 7usize, 29usize, 2157396u32);
    emu.sltru_no_count(12usize, 17usize, 6usize, 2157400u32);
    emu.sbr_no_count(31usize, 5usize, 7usize, 2157404u32);
    emu.sbr_no_count(18usize, 31usize, 12usize, 2157408u32);
    emu.sbr_no_count(19usize, 17usize, 6usize, 2157412u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2157452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb8c));
    } else {
        emu.pc = 2157416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb68));
    }
}
#[inline(always)]
pub fn block_0x0020eb68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(31usize, 19usize, 16usize, 2157420u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2157460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb94));
    } else {
        emu.pc = 2157424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb70));
    }
}
#[inline(always)]
pub fn block_0x0020eb70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2157428u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157480u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020eba8));
}
#[inline(always)]
pub fn block_0x0020eb74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 6usize, 30usize, 2157432u32);
    emu.sltru_no_count(12usize, 17usize, 6usize, 2157436u32);
    emu.sbr_no_count(31usize, 5usize, 7usize, 2157440u32);
    emu.sbr_no_count(18usize, 31usize, 12usize, 2157444u32);
    emu.sbr_no_count(19usize, 17usize, 6usize, 2157448u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2157416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb68));
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
    emu.sltru_no_count(31usize, 18usize, 15usize, 2157456u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2157480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eba8));
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
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2157280u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eae0));
    } else {
        emu.pc = 2157464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb98));
    }
}
#[inline(always)]
pub fn block_0x0020eb98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 19usize, 16usize, 2157468u32);
    emu.adi_no_count(21usize, 6usize, 0u32, 2157472u32);
    emu.adi_no_count(18usize, 7usize, 0u32, 2157476u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2157296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eaf0));
    } else {
        emu.pc = 2157480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eba8));
    }
}
#[inline(always)]
pub fn block_0x0020eba8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2156984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e9b8));
    } else {
        emu.pc = 2157484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ebac));
    }
}
#[inline(always)]
pub fn block_0x0020ebac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 7usize, 28usize, 2157488u32);
    emu.lw_no_count(29usize, 2usize, 16u32, 2157492u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2157704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec88));
    } else {
        emu.pc = 2157496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ebb8));
    }
}
#[inline(always)]
pub fn block_0x0020ebb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2157704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec88));
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
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(16usize, 6usize, 16usize, 2157504u32);
    emu.sltru_no_count(10usize, 16usize, 6usize, 2157508u32);
    emu.adr_no_count(15usize, 7usize, 15usize, 2157512u32);
    emu.adr_no_count(10usize, 15usize, 10usize, 2157516u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2157652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec54));
    } else {
        emu.pc = 2157520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ebd0));
    }
}
#[inline(always)]
pub fn block_0x0020ebd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 10usize, 28usize, 2157524u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2157660u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec5c));
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
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2157532u32;
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
pub fn block_0x0020ebdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 6usize, 13usize, 2157536u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2157828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed04));
    } else {
        emu.pc = 2157540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ebe4));
    }
}
#[inline(always)]
pub fn block_0x0020ebe4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 13usize, 16usize, 2157544u32);
    emu.sbr_no_count(14usize, 17usize, 5usize, 2157548u32);
    emu.sbr_no_count(15usize, 10usize, 17usize, 2157552u32);
    emu.sltru_no_count(17usize, 6usize, 13usize, 2157556u32);
    emu.sbr_no_count(10usize, 14usize, 12usize, 2157560u32);
    emu.sbr_no_count(12usize, 15usize, 17usize, 2157564u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2157576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec08));
    } else {
        emu.pc = 2157568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec00));
    }
}
#[inline(always)]
pub fn block_0x0020ec00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 13usize, 16usize, 2157572u32);
    emu.sbr_no_count(12usize, 6usize, 13usize, 2157576u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2157576u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ec08));
}
#[inline(always)]
pub fn block_0x0020ec08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 10usize, 12usize, 2157580u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2157828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed04));
    } else {
        emu.pc = 2157584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec10));
    }
}
#[inline(always)]
pub fn block_0x0020ec10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(10usize, 16usize, 2u32, 2157588u32);
    emu.sltiu_no_count(12usize, 5usize, 1u32, 2157592u32);
    emu.anr_no_count(10usize, 12usize, 10usize, 2157596u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2157828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed04));
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
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 4294967292u32, 2157604u32);
    emu.sltru_no_count(12usize, 10usize, 8usize, 2157608u32);
    emu.adr_no_count(12usize, 9usize, 12usize, 2157612u32);
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2157616u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2157820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ecfc));
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
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 12usize, 5usize, 2157624u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2157828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed04));
    } else {
        emu.pc = 2157628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec3c));
    }
}
#[inline(always)]
pub fn block_0x0020ec3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 0usize, 11usize, 2157632u32);
    emu.sw_no_count(7usize, 18usize, 0u32, 2157636u32)?;
    emu.sw_no_count(10usize, 18usize, 4u32, 2157640u32)?;
    emu.lw_no_count(10usize, 2usize, 0u32, 2157644u32)?;
    emu.sh_no_count(10usize, 18usize, 8u32, 2157648u32)?;
    emu.add_memory_rw_events(6usize);
    let return_addr = 2157652u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157888u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ed40));
}
#[inline(always)]
pub fn block_0x0020ec54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 16usize, 13usize, 2157656u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2157884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed3c));
    } else {
        emu.pc = 2157660u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec5c));
    }
}
#[inline(always)]
pub fn block_0x0020ec5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 13usize, 6usize, 2157664u32);
    emu.sbr_no_count(14usize, 28usize, 7usize, 2157668u32);
    emu.sbr_no_count(15usize, 10usize, 28usize, 2157672u32);
    emu.sltru_no_count(28usize, 16usize, 13usize, 2157676u32);
    emu.sbr_no_count(10usize, 14usize, 12usize, 2157680u32);
    emu.sbr_no_count(12usize, 15usize, 28usize, 2157684u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2157696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec80));
    } else {
        emu.pc = 2157688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec78));
    }
}
#[inline(always)]
pub fn block_0x0020ec78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 13usize, 6usize, 2157692u32);
    emu.sbr_no_count(12usize, 16usize, 13usize, 2157696u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2157696u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ec80));
}
#[inline(always)]
pub fn block_0x0020ec80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 10usize, 12usize, 2157700u32);
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
        emu.pc = 2157704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec88));
    }
}
#[inline(always)]
pub fn block_0x0020ec88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 20u32, 2157708u32);
    emu.mulhu_no_count(12usize, 8usize, 10usize, 2157712u32);
    emu.mul_no_count(13usize, 9usize, 10usize, 2157716u32);
    emu.adr_no_count(12usize, 12usize, 13usize, 2157720u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2157736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eca8));
    } else {
        emu.pc = 2157724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec9c));
    }
}
#[inline(always)]
pub fn block_0x0020ec9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 7usize, 12usize, 2157728u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2157748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ecb4));
    } else {
        emu.pc = 2157732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eca4));
    }
}
#[inline(always)]
pub fn block_0x0020eca4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2157736u32;
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
pub fn block_0x0020eca8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.mul_no_count(10usize, 8usize, 10usize, 2157740u32);
    emu.sltru_no_count(10usize, 6usize, 10usize, 2157744u32);
    emu.add_memory_rw_events(2usize);
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
        emu.pc = 2157748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ecb4));
    }
}
#[inline]
pub fn block_0x0020ecb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4294967256u32, 2157752u32);
    emu.mul_no_count(12usize, 9usize, 10usize, 2157756u32);
    emu.mulhu_no_count(13usize, 8usize, 10usize, 2157760u32);
    emu.mul_no_count(10usize, 8usize, 10usize, 2157764u32);
    emu.sbr_no_count(13usize, 13usize, 8usize, 2157768u32);
    emu.adr_no_count(10usize, 17usize, 10usize, 2157772u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2157776u32);
    emu.adr_no_count(12usize, 5usize, 12usize, 2157780u32);
    emu.sltru_no_count(13usize, 10usize, 17usize, 2157784u32);
    emu.adr_no_count(12usize, 12usize, 13usize, 2157788u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2157876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed34));
    } else {
        emu.pc = 2157792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ece0));
    }
}
#[inline(always)]
pub fn block_0x0020ece0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 12usize, 7usize, 2157796u32);
    emu.add_memory_rw_events(1usize);
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
        emu.pc = 2157800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ece8));
    }
}
#[inline(always)]
pub fn block_0x0020ece8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(23usize, 29usize, 0u32, 2157804u32)?;
    emu.sw_no_count(11usize, 29usize, 4u32, 2157808u32)?;
    emu.lw_no_count(10usize, 2usize, 0u32, 2157812u32)?;
    emu.sh_no_count(10usize, 29usize, 8u32, 2157816u32)?;
    emu.add_memory_rw_events(5usize);
    let return_addr = 2157820u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157888u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ed40));
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
    emu.sltru_no_count(10usize, 10usize, 16usize, 2157824u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2157628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec3c));
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
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 18usize, 0u32, 2157832u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2157836u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157888u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ed40));
}
#[inline(always)]
pub fn block_0x0020ed0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 0u32, 2157840u32);
    emu.adi_no_count(16usize, 26usize, 0u32, 2157844u32);
    emu.adi_no_count(5usize, 27usize, 0u32, 2157848u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a != b {
        emu.pc = 2157216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eaa0));
    } else {
        emu.pc = 2157852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed1c));
    }
}
#[inline(always)]
pub fn block_0x0020ed1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2157856u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156884u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(2156884u32));
}
#[inline(always)]
pub fn block_0x0020ed20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(31usize, 0usize, 0u32, 2157860u32);
    emu.adi_no_count(6usize, 21usize, 0u32, 2157864u32);
    emu.adi_no_count(7usize, 18usize, 0u32, 2157868u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a != b {
        emu.pc = 2157484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ebac));
    } else {
        emu.pc = 2157872u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed30));
    }
}
#[inline(always)]
pub fn block_0x0020ed30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2157876u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156984u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e9b8));
}
#[inline(always)]
pub fn block_0x0020ed34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 10usize, 6usize, 2157880u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2157800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ece8));
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
    emu.sw_no_count(0usize, 29usize, 0u32, 2157888u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2157888u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ed40));
}
#[inline]
pub fn block_0x0020ed40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 124u32, 2157892u32)?;
    emu.lw_no_count(8usize, 2usize, 120u32, 2157896u32)?;
    emu.lw_no_count(9usize, 2usize, 116u32, 2157900u32)?;
    emu.lw_no_count(18usize, 2usize, 112u32, 2157904u32)?;
    emu.lw_no_count(19usize, 2usize, 108u32, 2157908u32)?;
    emu.lw_no_count(20usize, 2usize, 104u32, 2157912u32)?;
    emu.lw_no_count(21usize, 2usize, 100u32, 2157916u32)?;
    emu.lw_no_count(22usize, 2usize, 96u32, 2157920u32)?;
    emu.lw_no_count(23usize, 2usize, 92u32, 2157924u32)?;
    emu.lw_no_count(24usize, 2usize, 88u32, 2157928u32)?;
    emu.lw_no_count(25usize, 2usize, 84u32, 2157932u32)?;
    emu.lw_no_count(26usize, 2usize, 80u32, 2157936u32)?;
    emu.lw_no_count(27usize, 2usize, 76u32, 2157940u32)?;
    emu.adi_no_count(2usize, 2usize, 128u32, 2157944u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2157948u32;
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
pub fn block_0x0020ed7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 2usize, 52u32, 2157952u32)?;
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2157956u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965368u32, 2157960u32);
    emu.adi_no_count(11usize, 2usize, 40u32, 2157964u32);
    emu.adi_no_count(12usize, 2usize, 24u32, 2157968u32);
    emu.adi_no_count(13usize, 2usize, 52u32, 2157972u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2157976u32);
    emu.apc_no_count(1usize, 2157976u32, 4294955008u32, 2157980u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2157984u32;
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
pub fn block_0x0020eda0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2157988u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966048u32, 2157992u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2157996u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966076u32, 2158000u32);
    emu.adi_no_count(11usize, 0usize, 28u32, 2158004u32);
    emu.apc_no_count(1usize, 2158004u32, 4294955008u32, 2158008u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158012u32;
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
pub fn block_0x0020edbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2158016u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966092u32, 2158020u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2158024u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966124u32, 2158028u32);
    emu.adi_no_count(11usize, 0usize, 29u32, 2158032u32);
    emu.apc_no_count(1usize, 2158032u32, 4294955008u32, 2158036u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158040u32;
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
pub fn block_0x0020edd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2158044u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966140u32, 2158048u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2158052u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966168u32, 2158056u32);
    emu.adi_no_count(11usize, 0usize, 28u32, 2158060u32);
    emu.apc_no_count(1usize, 2158060u32, 4294955008u32, 2158064u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158068u32;
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
pub fn block_0x0020edf4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2158072u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966432u32, 2158076u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2158080u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966488u32, 2158084u32);
    emu.adi_no_count(11usize, 0usize, 54u32, 2158088u32);
    emu.apc_no_count(1usize, 2158088u32, 4294955008u32, 2158092u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158096u32;
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
pub fn block_0x0020ee10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2158100u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966360u32, 2158104u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2158108u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966416u32, 2158112u32);
    emu.adi_no_count(11usize, 0usize, 55u32, 2158116u32);
    emu.apc_no_count(1usize, 2158116u32, 4294955008u32, 2158120u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158124u32;
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
pub fn block_0x0020ee2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2158128u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966184u32, 2158132u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2158136u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966232u32, 2158140u32);
    emu.adi_no_count(11usize, 0usize, 45u32, 2158144u32);
    emu.apc_no_count(1usize, 2158144u32, 4294955008u32, 2158148u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158152u32;
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
pub fn block_0x0020ee48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2158156u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966248u32, 2158160u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2158164u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966296u32, 2158168u32);
    emu.adi_no_count(11usize, 0usize, 45u32, 2158172u32);
    emu.apc_no_count(1usize, 2158172u32, 4294955008u32, 2158176u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158180u32;
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
pub fn block_0x0020ee64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2158184u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966328u32, 2158188u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2158192u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2158196u32);
    emu.apc_no_count(1usize, 2158196u32, 4294955008u32, 2158200u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158204u32;
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
pub fn block_0x0020ee7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2158208u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966344u32, 2158212u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2158216u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2158220u32);
    emu.apc_no_count(1usize, 2158220u32, 4294955008u32, 2158224u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158228u32;
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
pub fn block_0x0020ee94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2158232u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966032u32, 2158236u32);
    emu.adi_no_count(11usize, 0usize, 81u32, 2158240u32);
    emu.adi_no_count(10usize, 14usize, 0u32, 2158244u32);
    emu.apc_no_count(1usize, 2158244u32, 4294955008u32, 2158248u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158252u32;
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
pub fn block_0x0020eeac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2158256u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2158260u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2158264u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2158268u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2158272u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2158276u32)?;
    emu.sw_no_count(20usize, 2usize, 24u32, 2158280u32)?;
    emu.sw_no_count(21usize, 2usize, 20u32, 2158284u32)?;
    emu.sw_no_count(22usize, 2usize, 16u32, 2158288u32)?;
    emu.sw_no_count(23usize, 2usize, 12u32, 2158292u32)?;
    emu.sw_no_count(24usize, 2usize, 8u32, 2158296u32)?;
    emu.sw_no_count(25usize, 2usize, 4u32, 2158300u32)?;
    emu.sw_no_count(26usize, 2usize, 0u32, 2158304u32)?;
    emu.adi_no_count(15usize, 14usize, 0u32, 2158308u32);
    emu.lw_no_count(17usize, 11usize, 0u32, 2158312u32)?;
    emu.lw_no_count(16usize, 11usize, 4u32, 2158316u32)?;
    emu.orr_no_count(14usize, 17usize, 16usize, 2158320u32);
    emu.add_memory_rw_events(17usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2159916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f52c));
    } else {
        emu.pc = 2158324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eef4));
    }
}
#[inline(always)]
pub fn block_0x0020eef4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(14usize, 16usize, 29u32, 2158328u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a != b {
        emu.pc = 2159944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f548));
    } else {
        emu.pc = 2158332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eefc));
    }
}
#[inline(always)]
pub fn block_0x0020eefc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2159972u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f564));
    } else {
        emu.pc = 2158336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ef00));
    }
}
#[inline]
pub fn block_0x0020ef00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lh_no_count(14usize, 11usize, 24u32, 2158340u32)?;
    emu.sri_no_count(11usize, 17usize, 1u32, 2158344u32);
    let a = 0u32.wrapping_add(1431654400u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2158348u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(858992640u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2158352u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(252645376u32);
    emu.write_reg_no_count(29usize, a);
    emu.pc = 2158356u32;
    emu.update_insn_clock();
    emu.adi_no_count(28usize, 5usize, 1365u32, 2158360u32);
    emu.adi_no_count(7usize, 6usize, 819u32, 2158364u32);
    emu.adi_no_count(5usize, 29usize, 4294967055u32, 2158368u32);
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2158372u32;
    emu.update_insn_clock();
    emu.adi_no_count(6usize, 6usize, 257u32, 2158376u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a != b {
        emu.pc = 2158476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ef8c));
    } else {
        emu.pc = 2158380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ef2c));
    }
}
#[inline]
pub fn block_0x0020ef2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.orr_no_count(29usize, 17usize, 11usize, 2158384u32);
    emu.sri_no_count(30usize, 29usize, 2u32, 2158388u32);
    emu.orr_no_count(29usize, 29usize, 30usize, 2158392u32);
    emu.sri_no_count(30usize, 29usize, 4u32, 2158396u32);
    emu.orr_no_count(29usize, 29usize, 30usize, 2158400u32);
    emu.sri_no_count(30usize, 29usize, 8u32, 2158404u32);
    emu.orr_no_count(29usize, 29usize, 30usize, 2158408u32);
    emu.sri_no_count(30usize, 29usize, 16u32, 2158412u32);
    emu.orr_no_count(29usize, 29usize, 30usize, 2158416u32);
    emu.xri_no_count(29usize, 29usize, 4294967295u32, 2158420u32);
    emu.sri_no_count(30usize, 29usize, 1u32, 2158424u32);
    emu.anr_no_count(28usize, 30usize, 28usize, 2158428u32);
    emu.sbr_no_count(28usize, 29usize, 28usize, 2158432u32);
    emu.anr_no_count(29usize, 28usize, 7usize, 2158436u32);
    emu.sri_no_count(28usize, 28usize, 2u32, 2158440u32);
    emu.anr_no_count(7usize, 28usize, 7usize, 2158444u32);
    emu.adr_no_count(7usize, 29usize, 7usize, 2158448u32);
    emu.sri_no_count(28usize, 7usize, 4u32, 2158452u32);
    emu.adr_no_count(7usize, 7usize, 28usize, 2158456u32);
    emu.anr_no_count(5usize, 7usize, 5usize, 2158460u32);
    emu.mul_no_count(5usize, 5usize, 6usize, 2158464u32);
    emu.sri_no_count(5usize, 5usize, 24u32, 2158468u32);
    emu.adi_no_count(6usize, 5usize, 32u32, 2158472u32);
    emu.add_memory_rw_events(24usize);
    let return_addr = 2158476u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2158568u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020efe8));
}
#[inline]
pub fn block_0x0020ef8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 23u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(29usize, 16usize, 1u32, 2158480u32);
    emu.orr_no_count(29usize, 16usize, 29usize, 2158484u32);
    emu.sri_no_count(30usize, 29usize, 2u32, 2158488u32);
    emu.orr_no_count(29usize, 29usize, 30usize, 2158492u32);
    emu.sri_no_count(30usize, 29usize, 4u32, 2158496u32);
    emu.orr_no_count(29usize, 29usize, 30usize, 2158500u32);
    emu.sri_no_count(30usize, 29usize, 8u32, 2158504u32);
    emu.orr_no_count(29usize, 29usize, 30usize, 2158508u32);
    emu.sri_no_count(30usize, 29usize, 16u32, 2158512u32);
    emu.orr_no_count(29usize, 29usize, 30usize, 2158516u32);
    emu.xri_no_count(29usize, 29usize, 4294967295u32, 2158520u32);
    emu.sri_no_count(30usize, 29usize, 1u32, 2158524u32);
    emu.anr_no_count(28usize, 30usize, 28usize, 2158528u32);
    emu.sbr_no_count(28usize, 29usize, 28usize, 2158532u32);
    emu.anr_no_count(29usize, 28usize, 7usize, 2158536u32);
    emu.sri_no_count(28usize, 28usize, 2u32, 2158540u32);
    emu.anr_no_count(7usize, 28usize, 7usize, 2158544u32);
    emu.adr_no_count(7usize, 29usize, 7usize, 2158548u32);
    emu.sri_no_count(28usize, 7usize, 4u32, 2158552u32);
    emu.adr_no_count(7usize, 7usize, 28usize, 2158556u32);
    emu.anr_no_count(5usize, 7usize, 5usize, 2158560u32);
    emu.mul_no_count(5usize, 5usize, 6usize, 2158564u32);
    emu.sri_no_count(6usize, 5usize, 24u32, 2158568u32);
    emu.add_memory_rw_events(23usize);
    emu.pc = 2158568u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020efe8));
}
#[inline]
pub fn block_0x0020efe8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(5usize, 14usize, 6usize, 2158572u32);
    emu.adi_no_count(14usize, 0usize, 4294967200u32, 2158576u32);
    emu.adi_no_count(7usize, 0usize, 80u32, 2158580u32);
    let a = 0u32.wrapping_add(2068697088u32);
    emu.write_reg_no_count(28usize, a);
    emu.pc = 2158584u32;
    emu.update_insn_clock();
    emu.sbr_no_count(14usize, 14usize, 5usize, 2158588u32);
    emu.adi_no_count(28usize, 28usize, 4294965651u32, 2158592u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2158596u32);
    emu.sai_no_count(14usize, 14usize, 1040u32, 2158600u32);
    emu.adi_no_count(14usize, 14usize, 1087u32, 2158604u32);
    emu.mul_no_count(14usize, 14usize, 7usize, 2158608u32);
    emu.mulh_no_count(14usize, 14usize, 28usize, 2158612u32);
    emu.sri_no_count(28usize, 14usize, 31u32, 2158616u32);
    emu.sai_no_count(14usize, 14usize, 1034u32, 2158620u32);
    emu.adr_no_count(14usize, 14usize, 28usize, 2158624u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a < b {
        emu.pc = 2160048u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f5b0));
    } else {
        emu.pc = 2158628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f024));
    }
}
#[inline(always)]
pub fn block_0x0020f024(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(7usize, 6usize, 4294967264u32, 2158632u32);
    emu.slr_no_count(17usize, 17usize, 6usize, 2158636u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(7usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2158648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f038));
    } else {
        emu.pc = 2158640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f030));
    }
}
#[inline(always)]
pub fn block_0x0020f030(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 17usize, 0u32, 2158644u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2158648u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2158664u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f048));
}
#[inline(always)]
pub fn block_0x0020f038(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(28usize, 6usize, 4294967295u32, 2158652u32);
    emu.srr_no_count(11usize, 11usize, 28usize, 2158656u32);
    emu.slr_no_count(16usize, 16usize, 6usize, 2158660u32);
    emu.orr_no_count(11usize, 16usize, 11usize, 2158664u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2158664u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f048));
}
#[inline(never)]
pub fn block_0x0020f048(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 40u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sai_no_count(16usize, 7usize, 1055u32, 2158668u32);
    emu.sli_no_count(14usize, 14usize, 4u32, 2158672u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2158676u32;
    emu.update_insn_clock();
    emu.adi_no_count(6usize, 6usize, 1536u32, 2158680u32);
    emu.adr_no_count(14usize, 6usize, 14usize, 2158684u32);
    emu.lw_no_count(6usize, 14usize, 0u32, 2158688u32)?;
    emu.lw_no_count(7usize, 14usize, 4u32, 2158692u32)?;
    emu.anr_no_count(16usize, 16usize, 17usize, 2158696u32);
    emu.lh_no_count(17usize, 14usize, 8u32, 2158700u32)?;
    emu.mulhu_no_count(28usize, 6usize, 16usize, 2158704u32);
    emu.mul_no_count(29usize, 7usize, 16usize, 2158708u32);
    emu.mulhu_no_count(30usize, 7usize, 16usize, 2158712u32);
    emu.mul_no_count(31usize, 6usize, 11usize, 2158716u32);
    emu.mulhu_no_count(6usize, 6usize, 11usize, 2158720u32);
    emu.mul_no_count(8usize, 7usize, 11usize, 2158724u32);
    emu.mulhu_no_count(11usize, 7usize, 11usize, 2158728u32);
    emu.adr_no_count(17usize, 5usize, 17usize, 2158732u32);
    emu.adi_no_count(16usize, 0usize, 4294967232u32, 2158736u32);
    emu.adr_no_count(28usize, 29usize, 28usize, 2158740u32);
    emu.sbr_no_count(5usize, 16usize, 17usize, 2158744u32);
    emu.sbr_no_count(16usize, 0usize, 17usize, 2158748u32);
    emu.sltru_no_count(17usize, 28usize, 29usize, 2158752u32);
    emu.adr_no_count(28usize, 31usize, 28usize, 2158756u32);
    emu.adr_no_count(17usize, 30usize, 17usize, 2158760u32);
    emu.ani_no_count(29usize, 5usize, 63u32, 2158764u32);
    emu.sltru_no_count(7usize, 28usize, 31usize, 2158768u32);
    emu.sri_no_count(30usize, 28usize, 31u32, 2158772u32);
    emu.adr_no_count(6usize, 6usize, 7usize, 2158776u32);
    emu.adi_no_count(28usize, 29usize, 4294967264u32, 2158780u32);
    emu.adr_no_count(6usize, 17usize, 6usize, 2158784u32);
    emu.sltru_no_count(17usize, 6usize, 17usize, 2158788u32);
    emu.adr_no_count(6usize, 8usize, 6usize, 2158792u32);
    emu.sltru_no_count(7usize, 6usize, 8usize, 2158796u32);
    emu.adr_no_count(11usize, 11usize, 17usize, 2158800u32);
    emu.adr_no_count(6usize, 30usize, 6usize, 2158804u32);
    emu.adr_no_count(11usize, 11usize, 7usize, 2158808u32);
    emu.sltru_no_count(7usize, 6usize, 30usize, 2158812u32);
    emu.adr_no_count(7usize, 11usize, 7usize, 2158816u32);
    emu.xri_no_count(8usize, 29usize, 4294967295u32, 2158820u32);
    emu.add_memory_rw_events(39usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2158832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f0f0));
    } else {
        emu.pc = 2158824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f0e8));
    }
}
#[inline(always)]
pub fn block_0x0020f0e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.srr_no_count(11usize, 7usize, 29usize, 2158828u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2158832u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2158848u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f100));
}
#[inline(always)]
pub fn block_0x0020f0f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 7usize, 1u32, 2158836u32);
    emu.slr_no_count(11usize, 11usize, 8usize, 2158840u32);
    emu.srr_no_count(17usize, 6usize, 16usize, 2158844u32);
    emu.orr_no_count(11usize, 17usize, 11usize, 2158848u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2158848u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f100));
}
#[inline]
pub fn block_0x0020f100(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lhu_no_count(14usize, 14usize, 10u32, 2158852u32)?;
    emu.slti_no_count(17usize, 28usize, 0u32, 2158856u32);
    emu.adi_no_count(30usize, 0usize, 1u32, 2158860u32);
    emu.slr_no_count(31usize, 30usize, 29usize, 2158864u32);
    emu.adi_no_count(9usize, 17usize, 4294967295u32, 2158868u32);
    emu.sbr_no_count(17usize, 0usize, 17usize, 2158872u32);
    emu.slr_no_count(16usize, 30usize, 16usize, 2158876u32);
    emu.anr_no_count(31usize, 9usize, 31usize, 2158880u32);
    emu.anr_no_count(30usize, 17usize, 16usize, 2158884u32);
    emu.sltiu_no_count(16usize, 30usize, 1u32, 2158888u32);
    emu.adi_no_count(9usize, 30usize, 4294967295u32, 2158892u32);
    emu.sbr_no_count(18usize, 31usize, 16usize, 2158896u32);
    emu.anr_no_count(17usize, 18usize, 7usize, 2158900u32);
    emu.anr_no_count(16usize, 9usize, 6usize, 2158904u32);
    emu.orr_no_count(19usize, 16usize, 17usize, 2158908u32);
    emu.add_memory_rw_events(15usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2158976u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f180));
    } else {
        emu.pc = 2158912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f140));
    }
}
#[inline(always)]
pub fn block_0x0020f140(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(19usize, 11usize, 4u32, 2158916u32);
    emu.adi_no_count(20usize, 0usize, 625u32, 2158920u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a >= b {
        emu.pc = 2159016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f1a8));
    } else {
        emu.pc = 2158924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f14c));
    }
}
#[inline(always)]
pub fn block_0x0020f14c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 100u32, 2158928u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2159096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f1f8));
    } else {
        emu.pc = 2158932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f154));
    }
}
#[inline(always)]
pub fn block_0x0020f154(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 9u32, 2158936u32);
    emu.sltiu_no_count(20usize, 11usize, 10u32, 2158940u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a < b {
        emu.pc = 2159744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f480));
    } else {
        emu.pc = 2158944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f160));
    }
}
#[inline(always)]
pub fn block_0x0020f160(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 1u32, 2158948u32);
    emu.xri_no_count(20usize, 20usize, 1u32, 2158952u32);
    emu.sbr_no_count(14usize, 20usize, 14usize, 2158956u32);
    emu.adi_no_count(14usize, 14usize, 1u32, 2158960u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2158964u32);
    emu.sai_no_count(14usize, 14usize, 1040u32, 2158968u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(15usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2159136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f220));
    } else {
        emu.pc = 2158972u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f17c));
    }
}
#[inline(always)]
pub fn block_0x0020f17c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2158976u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2159076u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f1e4));
}
#[inline(always)]
pub fn block_0x0020f180(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 10u32, 2158980u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a < b {
        emu.pc = 2159008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f1a0));
    } else {
        emu.pc = 2158984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f188));
    }
}
#[inline(always)]
pub fn block_0x0020f188(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(19usize, 13usize, 2u32, 2158988u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(20usize, a);
    emu.pc = 2158992u32;
    emu.update_insn_clock();
    emu.adi_no_count(20usize, 20usize, 4294966624u32, 2158996u32);
    emu.adr_no_count(19usize, 20usize, 19usize, 2159000u32);
    emu.lw_no_count(19usize, 19usize, 4294967292u32, 2159004u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2158912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f140));
    } else {
        emu.pc = 2159008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f1a0));
    }
}
#[inline(always)]
pub fn block_0x0020f1a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 10usize, 0u32, 2159012u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2159016u32;
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
pub fn block_0x0020f1a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(999424u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2159020u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 19usize, 576u32, 2159024u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2159248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f290));
    } else {
        emu.pc = 2159028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f1b4));
    }
}
#[inline(always)]
pub fn block_0x0020f1b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(98304u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2159032u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 19usize, 1696u32, 2159036u32);
    emu.sltru_no_count(20usize, 11usize, 19usize, 2159040u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
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
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2159048u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 19usize, 1808u32, 2159052u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2159052u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f1cc));
}
#[inline(always)]
pub fn block_0x0020f1cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(20usize, 20usize, 5u32, 2159056u32);
    emu.sbr_no_count(14usize, 20usize, 14usize, 2159060u32);
    emu.adi_no_count(14usize, 14usize, 1u32, 2159064u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2159068u32);
    emu.sai_no_count(14usize, 14usize, 1040u32, 2159072u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(15usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2159136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f220));
    } else {
        emu.pc = 2159076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f1e4));
    }
}
#[inline(always)]
pub fn block_0x0020f1e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(7usize, 14usize, 15usize, 2159080u32);
    emu.sli_no_count(6usize, 5usize, 16u32, 2159084u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a < b {
        emu.pc = 2159312u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f2d0));
    } else {
        emu.pc = 2159088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f1f0));
    }
}
#[inline(always)]
pub fn block_0x0020f1f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 13usize, 0u32, 2159092u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2159096u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2159320u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f2d8));
}
#[inline(always)]
pub fn block_0x0020f1f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(21usize, 0usize, 1000u32, 2159100u32);
    emu.sltiu_no_count(20usize, 11usize, 1000u32, 2159104u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2159112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f208));
    } else {
        emu.pc = 2159108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f204));
    }
}
#[inline(always)]
pub fn block_0x0020f204(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 1000u32, 2159112u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2159112u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f208));
}
#[inline(always)]
pub fn block_0x0020f208(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(20usize, 20usize, 3u32, 2159116u32);
    emu.sbr_no_count(14usize, 20usize, 14usize, 2159120u32);
    emu.adi_no_count(14usize, 14usize, 1u32, 2159124u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2159128u32);
    emu.sai_no_count(14usize, 14usize, 1040u32, 2159132u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(15usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2159076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f1e4));
    } else {
        emu.pc = 2159136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f220));
    }
}
#[inline(never)]
pub fn block_0x0020f220(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 26u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 6usize, 1u32, 2159140u32);
    emu.sli_no_count(16usize, 7usize, 31u32, 2159144u32);
    emu.sri_no_count(17usize, 7usize, 1u32, 2159148u32);
    let a = 0u32.wrapping_add(3435974656u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2159152u32;
    emu.update_insn_clock();
    emu.orr_no_count(11usize, 11usize, 16usize, 2159156u32);
    emu.adi_no_count(6usize, 5usize, 4294966477u32, 2159160u32);
    emu.adi_no_count(16usize, 5usize, 4294966476u32, 2159164u32);
    emu.adr_no_count(5usize, 11usize, 17usize, 2159168u32);
    emu.sltru_no_count(7usize, 5usize, 11usize, 2159172u32);
    emu.adr_no_count(5usize, 5usize, 7usize, 2159176u32);
    emu.mulhu_no_count(7usize, 5usize, 6usize, 2159180u32);
    emu.sri_no_count(9usize, 7usize, 2u32, 2159184u32);
    emu.ani_no_count(7usize, 7usize, 4294967292u32, 2159188u32);
    emu.adr_no_count(7usize, 7usize, 9usize, 2159192u32);
    emu.sbr_no_count(5usize, 5usize, 7usize, 2159196u32);
    emu.sbr_no_count(7usize, 11usize, 5usize, 2159200u32);
    emu.sltru_no_count(11usize, 11usize, 5usize, 2159204u32);
    emu.mul_no_count(5usize, 7usize, 16usize, 2159208u32);
    emu.mulhu_no_count(9usize, 7usize, 6usize, 2159212u32);
    emu.sbr_no_count(11usize, 17usize, 11usize, 2159216u32);
    emu.mul_no_count(16usize, 7usize, 6usize, 2159220u32);
    emu.adr_no_count(17usize, 9usize, 5usize, 2159224u32);
    emu.mul_no_count(11usize, 11usize, 6usize, 2159228u32);
    emu.adr_no_count(17usize, 17usize, 11usize, 2159232u32);
    emu.slr_no_count(11usize, 19usize, 29usize, 2159236u32);
    emu.add_memory_rw_events(25usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2159448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f358));
    } else {
        emu.pc = 2159240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f288));
    }
}
#[inline(always)]
pub fn block_0x0020f288(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(29usize, 11usize, 0u32, 2159244u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2159248u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2159456u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f360));
}
#[inline(always)]
pub fn block_0x0020f290(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(99999744u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2159252u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 19usize, 256u32, 2159256u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2159776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f4a0));
    } else {
        emu.pc = 2159260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f29c));
    }
}
#[inline(always)]
pub fn block_0x0020f29c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(9998336u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2159264u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 19usize, 1664u32, 2159268u32);
    emu.sltru_no_count(20usize, 11usize, 19usize, 2159272u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2159284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f2b4));
    } else {
        emu.pc = 2159276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f2ac));
    }
}
#[inline(always)]
pub fn block_0x0020f2ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(999424u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2159280u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 19usize, 576u32, 2159284u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2159284u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f2b4));
}
#[inline(always)]
pub fn block_0x0020f2b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(20usize, 20usize, 7u32, 2159288u32);
    emu.sbr_no_count(14usize, 20usize, 14usize, 2159292u32);
    emu.adi_no_count(14usize, 14usize, 1u32, 2159296u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2159300u32);
    emu.sai_no_count(14usize, 14usize, 1040u32, 2159304u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(15usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2159076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f1e4));
    } else {
        emu.pc = 2159308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f2cc));
    }
}
#[inline(always)]
pub fn block_0x0020f2cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2159312u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2159136u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f220));
}
#[inline(always)]
pub fn block_0x0020f2d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(7usize, 7usize, 16u32, 2159316u32);
    emu.sai_no_count(5usize, 7usize, 1040u32, 2159320u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2159320u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f2d8));
}
#[inline(always)]
pub fn block_0x0020f2d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(7usize, 6usize, 16u32, 2159324u32);
    emu.adi_no_count(22usize, 0usize, 4294967295u32, 2159328u32);
    let a = 0u32.wrapping_add(3435974656u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2159332u32;
    emu.update_insn_clock();
    emu.adi_no_count(23usize, 0usize, 10u32, 2159336u32);
    emu.adi_no_count(24usize, 6usize, 4294966477u32, 2159340u32);
    emu.adi_no_count(21usize, 0usize, 4294967295u32, 2159344u32);
    emu.adi_no_count(25usize, 12usize, 0u32, 2159348u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2159348u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f2f4));
}
#[inline(always)]
pub fn block_0x0020f2f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(26usize, 13usize, 21usize, 2159352u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a == b {
        emu.pc = 2160000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f580));
    } else {
        emu.pc = 2159356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f2fc));
    }
}
#[inline(always)]
pub fn block_0x0020f2fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(6usize, 19usize, 0u32, 2159360u32);
    emu.divu_no_count(19usize, 11usize, 19usize, 2159364u32);
    emu.mul_no_count(26usize, 19usize, 6usize, 2159368u32);
    emu.sbr_no_count(11usize, 11usize, 26usize, 2159372u32);
    emu.adr_no_count(26usize, 5usize, 21usize, 2159376u32);
    emu.adi_no_count(19usize, 19usize, 48u32, 2159380u32);
    emu.sb_no_count(19usize, 25usize, 0u32, 2159384u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a == b {
        emu.pc = 2159432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f348));
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
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(19usize, 20usize, 21usize, 2159392u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2159480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f378));
    } else {
        emu.pc = 2159396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f324));
    }
}
#[inline(always)]
pub fn block_0x0020f324(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.mulhu_no_count(19usize, 6usize, 24usize, 2159400u32);
    emu.adi_no_count(25usize, 25usize, 1u32, 2159404u32);
    emu.sri_no_count(19usize, 19usize, 3u32, 2159408u32);
    emu.adi_no_count(21usize, 21usize, 4294967295u32, 2159412u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a >= b {
        emu.pc = 2159348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f2f4));
    } else {
        emu.pc = 2159416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f338));
    }
}
#[inline(always)]
pub fn block_0x0020f338(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2159420u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966664u32, 2159424u32);
    emu.apc_no_count(1usize, 2159424u32, 0u32, 2159428u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2159432u32;
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
pub fn block_0x0020f348(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.slr_no_count(7usize, 11usize, 29usize, 2159436u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2159668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f434));
    } else {
        emu.pc = 2159440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f350));
    }
}
#[inline(always)]
pub fn block_0x0020f350(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 7usize, 0u32, 2159444u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2159448u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2159676u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f43c));
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
    emu.sri_no_count(5usize, 19usize, 1u32, 2159452u32);
    emu.srr_no_count(29usize, 5usize, 8usize, 2159456u32);
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
    emu.sai_no_count(5usize, 28usize, 1055u32, 2159460u32);
    emu.anr_no_count(28usize, 5usize, 11usize, 2159464u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2159468u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2159472u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2159476u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2159480u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2159852u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f4ec));
}
#[inline]
pub fn block_0x0020f378(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(6usize, 0usize, 0u32, 2159484u32);
    emu.adi_no_count(19usize, 7usize, 4294967295u32, 2159488u32);
    emu.sbr_no_count(11usize, 0usize, 21usize, 2159492u32);
    emu.adi_no_count(7usize, 0usize, 1u32, 2159496u32);
    emu.ani_no_count(19usize, 19usize, 63u32, 2159500u32);
    emu.xri_no_count(20usize, 19usize, 4294967295u32, 2159504u32);
    emu.adi_no_count(21usize, 19usize, 4294967264u32, 2159508u32);
    emu.sai_no_count(22usize, 21usize, 1055u32, 2159512u32);
    emu.adi_no_count(23usize, 0usize, 10u32, 2159516u32);
    emu.add_memory_rw_events(10usize);
    let return_addr = 2159520u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2159568u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f3d0));
}
#[inline(always)]
pub fn block_0x0020f3a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.srr_no_count(24usize, 17usize, 29usize, 2159524u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2159524u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f3a4));
}
#[inline]
pub fn block_0x0020f3a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(17usize, 17usize, 18usize, 2159528u32);
    emu.anr_no_count(16usize, 16usize, 9usize, 2159532u32);
    emu.mulhu_no_count(25usize, 7usize, 23usize, 2159536u32);
    emu.mul_no_count(6usize, 6usize, 23usize, 2159540u32);
    emu.mul_no_count(7usize, 7usize, 23usize, 2159544u32);
    emu.adi_no_count(24usize, 24usize, 48u32, 2159548u32);
    emu.adr_no_count(6usize, 25usize, 6usize, 2159552u32);
    emu.adr_no_count(25usize, 12usize, 11usize, 2159556u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2159560u32);
    emu.sb_no_count(24usize, 25usize, 0u32, 2159564u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2159824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f4d0));
    } else {
        emu.pc = 2159568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f3d0));
    }
}
#[inline(always)]
pub fn block_0x0020f3d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.srr_no_count(24usize, 6usize, 19usize, 2159572u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(21usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2159596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f3ec));
    } else {
        emu.pc = 2159576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f3d8));
    }
}
#[inline(always)]
pub fn block_0x0020f3d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 24usize, 0u32, 2159580u32);
    emu.anr_no_count(24usize, 22usize, 24usize, 2159584u32);
    emu.orr_no_count(24usize, 25usize, 24usize, 2159588u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a == b {
        emu.pc = 2159624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f408));
    } else {
        emu.pc = 2159592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f3e8));
    }
}
#[inline(always)]
pub fn block_0x0020f3e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2159596u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2159008u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f1a0));
}
#[inline(always)]
pub fn block_0x0020f3ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.srr_no_count(25usize, 7usize, 19usize, 2159600u32);
    emu.sli_no_count(26usize, 6usize, 1u32, 2159604u32);
    emu.slr_no_count(26usize, 26usize, 20usize, 2159608u32);
    emu.orr_no_count(25usize, 25usize, 26usize, 2159612u32);
    emu.anr_no_count(24usize, 22usize, 24usize, 2159616u32);
    emu.orr_no_count(24usize, 25usize, 24usize, 2159620u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a != b {
        emu.pc = 2159008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f1a0));
    } else {
        emu.pc = 2159624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f408));
    }
}
#[inline(always)]
pub fn block_0x0020f408(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2160024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f598));
    } else {
        emu.pc = 2159628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f40c));
    }
}
#[inline(always)]
pub fn block_0x0020f40c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.mulhu_no_count(24usize, 16usize, 23usize, 2159632u32);
    emu.mul_no_count(17usize, 17usize, 23usize, 2159636u32);
    emu.adr_no_count(17usize, 24usize, 17usize, 2159640u32);
    emu.mul_no_count(16usize, 16usize, 23usize, 2159644u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2159520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f3a0));
    } else {
        emu.pc = 2159648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f420));
    }
}
#[inline(always)]
pub fn block_0x0020f420(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(24usize, 17usize, 1u32, 2159652u32);
    emu.slr_no_count(24usize, 24usize, 8usize, 2159656u32);
    emu.srr_no_count(25usize, 16usize, 29usize, 2159660u32);
    emu.orr_no_count(24usize, 25usize, 24usize, 2159664u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2159668u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2159524u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f3a4));
}
#[inline(always)]
pub fn block_0x0020f434(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 11usize, 1u32, 2159672u32);
    emu.srr_no_count(9usize, 11usize, 8usize, 2159676u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2159676u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f43c));
}
#[inline(always)]
pub fn block_0x0020f43c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sai_no_count(11usize, 28usize, 1055u32, 2159680u32);
    emu.adr_no_count(17usize, 9usize, 17usize, 2159684u32);
    emu.anr_no_count(7usize, 11usize, 7usize, 2159688u32);
    emu.adr_no_count(16usize, 7usize, 16usize, 2159692u32);
    emu.sltru_no_count(7usize, 16usize, 7usize, 2159696u32);
    emu.adr_no_count(17usize, 17usize, 7usize, 2159700u32);
    emu.slr_no_count(7usize, 6usize, 29usize, 2159704u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2159716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f464));
    } else {
        emu.pc = 2159708u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f45c));
    }
}
#[inline(always)]
pub fn block_0x0020f45c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(29usize, 7usize, 0u32, 2159712u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2159716u32;
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
pub fn block_0x0020f464(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(6usize, 6usize, 1u32, 2159720u32);
    emu.srr_no_count(29usize, 6usize, 8usize, 2159724u32);
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
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(28usize, 11usize, 7usize, 2159728u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2159732u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2159736u32);
    emu.adi_no_count(13usize, 5usize, 0u32, 2159740u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2159744u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2159852u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f4ec));
}
#[inline(always)]
pub fn block_0x0020f480(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 10u32, 2159748u32);
    emu.xri_no_count(20usize, 20usize, 1u32, 2159752u32);
    emu.sbr_no_count(14usize, 20usize, 14usize, 2159756u32);
    emu.adi_no_count(14usize, 14usize, 1u32, 2159760u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2159764u32);
    emu.sai_no_count(14usize, 14usize, 1040u32, 2159768u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(15usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2159136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f220));
    } else {
        emu.pc = 2159772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f49c));
    }
}
#[inline(always)]
pub fn block_0x0020f49c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2159776u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2159076u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f1e4));
}
#[inline(always)]
pub fn block_0x0020f4a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(1000001536u32);
    emu.write_reg_no_count(20usize, a);
    emu.pc = 2159780u32;
    emu.update_insn_clock();
    emu.adi_no_count(21usize, 20usize, 4294965760u32, 2159784u32);
    emu.sltru_no_count(20usize, 11usize, 21usize, 2159788u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2159796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f4b4));
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
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 21usize, 0u32, 2159796u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2159796u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f4b4));
}
#[inline(always)]
pub fn block_0x0020f4b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(20usize, 20usize, 9u32, 2159800u32);
    emu.sbr_no_count(14usize, 20usize, 14usize, 2159804u32);
    emu.adi_no_count(14usize, 14usize, 1u32, 2159808u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2159812u32);
    emu.sai_no_count(14usize, 14usize, 1040u32, 2159816u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(15usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2159076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f1e4));
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
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2159824u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2159136u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f220));
}
#[inline(always)]
pub fn block_0x0020f4d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 12usize, 0u32, 2159828u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2159832u32);
    emu.adi_no_count(13usize, 5usize, 0u32, 2159836u32);
    emu.adi_no_count(28usize, 30usize, 0u32, 2159840u32);
    emu.adi_no_count(29usize, 31usize, 0u32, 2159844u32);
    emu.adi_no_count(30usize, 7usize, 0u32, 2159848u32);
    emu.adi_no_count(31usize, 6usize, 0u32, 2159852u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2159852u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f4ec));
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
    emu.apc_no_count(1usize, 2159852u32, 0u32, 2159856u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2159860u32;
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
pub fn block_0x0020f4f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2159864u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2159868u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2159872u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2159876u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2159880u32)?;
    emu.lw_no_count(20usize, 2usize, 24u32, 2159884u32)?;
    emu.lw_no_count(21usize, 2usize, 20u32, 2159888u32)?;
    emu.lw_no_count(22usize, 2usize, 16u32, 2159892u32)?;
    emu.lw_no_count(23usize, 2usize, 12u32, 2159896u32)?;
    emu.lw_no_count(24usize, 2usize, 8u32, 2159900u32)?;
    emu.lw_no_count(25usize, 2usize, 4u32, 2159904u32)?;
    emu.lw_no_count(26usize, 2usize, 0u32, 2159908u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2159912u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2159916u32;
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
pub fn block_0x0020f52c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2159920u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966048u32, 2159924u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2159928u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966540u32, 2159932u32);
    emu.adi_no_count(11usize, 0usize, 28u32, 2159936u32);
    emu.apc_no_count(1usize, 2159936u32, 4294955008u32, 2159940u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2159944u32;
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
pub fn block_0x0020f548(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2159948u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966556u32, 2159952u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2159956u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966592u32, 2159960u32);
    emu.adi_no_count(11usize, 0usize, 36u32, 2159964u32);
    emu.apc_no_count(1usize, 2159964u32, 4294955008u32, 2159968u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2159972u32;
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
pub fn block_0x0020f564(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2159976u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966504u32, 2159980u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2159984u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966608u32, 2159988u32);
    emu.adi_no_count(11usize, 0usize, 33u32, 2159992u32);
    emu.apc_no_count(1usize, 2159992u32, 4294955008u32, 2159996u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160000u32;
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
pub fn block_0x0020f580(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2160004u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966680u32, 2160008u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2160012u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2160016u32);
    emu.apc_no_count(1usize, 2160016u32, 4294955008u32, 2160020u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160024u32;
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
pub fn block_0x0020f598(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2160028u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966696u32, 2160032u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2160036u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2160040u32);
    emu.apc_no_count(1usize, 2160040u32, 4294955008u32, 2160044u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160048u32;
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
pub fn block_0x0020f5b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2160052u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966032u32, 2160056u32);
    emu.adi_no_count(11usize, 0usize, 81u32, 2160060u32);
    emu.adi_no_count(10usize, 14usize, 0u32, 2160064u32);
    emu.apc_no_count(1usize, 2160064u32, 4294955008u32, 2160068u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160072u32;
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
